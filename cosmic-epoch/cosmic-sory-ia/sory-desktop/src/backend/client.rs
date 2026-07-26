// SPDX-License-Identifier: GPL-3.0-only

use tokio::sync::{mpsc, oneshot};

use super::{
    BackendCommand, BackendEvent,
    connection::{BackendConnection, RECONNECT_RESET_ATTEMPTS, reconnect_delay},
    event_queue::BackendEventQueue,
    runtime::RuntimeManager,
};
use crate::platform::runtime_paths::ResolvedRuntimeBinary;

/// Client unique utilisé par l'application pour piloter le runtime Sory IA.
///
/// Cette façade réutilise l'app-server officiel du CLI/runtime. Elle ne parle
/// jamais directement au moteur IA et ne réimplémente aucune logique métier.
#[derive(Debug, Clone)]
pub struct BackendClient {
    runtime: RuntimeManager,
}

impl BackendClient {
    pub fn new(binary: ResolvedRuntimeBinary) -> Self {
        Self {
            runtime: RuntimeManager::new(binary),
        }
    }

    pub fn connect(self) -> BackendHandle {
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let runtime = self.runtime;

        tokio::spawn(async move {
            run_backend(runtime, commands_rx, events_tx).await;
        });

        BackendHandle {
            commands: commands_tx,
            events: events_rx,
        }
    }
}

pub struct BackendHandle {
    commands: mpsc::UnboundedSender<BackendCommandEnvelope>,
    pub events: mpsc::UnboundedReceiver<BackendEvent>,
}

impl BackendHandle {
    pub fn send(&self, command: BackendCommand) {
        let _ = self.commands.send(BackendCommandEnvelope::Command(command));
    }

    pub async fn disconnect(self) {
        let (tx, rx) = oneshot::channel();
        let _ = self.commands.send(BackendCommandEnvelope::Shutdown(tx));
        let _ = rx.await;
    }
}

enum BackendCommandEnvelope {
    Command(BackendCommand),
    Shutdown(oneshot::Sender<()>),
}

async fn run_backend(
    runtime: RuntimeManager,
    mut commands: mpsc::UnboundedReceiver<BackendCommandEnvelope>,
    events: mpsc::UnboundedSender<BackendEvent>,
) {
    let mut queue = BackendEventQueue::default();
    let mut reconnect_attempt: u32 = 0;
    let mut events_since_connected: u32 = 0;

    // Boucle de connexion persistante : on réessaye indéfiniment plutôt que
    // de laisser le task backend s'arrêter. L'UI voit Reconnecting → Connected
    // et peut afficher un état approprié.
    let mut connection = loop {
        match connect_runtime(&runtime, &events).await {
            Some(connection) => break connection,
            None => {
                let _ = events.send(BackendEvent::Reconnecting);
                reconnect_attempt += 1;
                reconnect_delay(reconnect_attempt).await;
                continue;
            }
        }
    };
    // Réinitialiser le compteur après une connexion réussie
    reconnect_attempt = 0;

    loop {
        tokio::select! {
            command = commands.recv() => {
                match command {
                    Some(BackendCommandEnvelope::Command(BackendCommand::Shutdown | BackendCommand::Disconnect)) => {
                        let _ = connection.shutdown().await;
                        let _ = events.send(BackendEvent::Disconnected);
                        break;
                    }
                    Some(BackendCommandEnvelope::Command(command)) => {
                        if let BackendCommand::SyncRuntimeConfig { runtime_config } = &command {
                            // Appliquer la config sans redémarrer le runtime
                            if let Err(err) = connection.sync_runtime_config(runtime_config).await {
                                log::warn!("Erreur sync config runtime: {err}");
                            }
                        }
                        if matches!(&command, BackendCommand::RestartRuntime) {
                            // Redémarrer le runtime pour recharger les env vars (clé API)
                            restart_runtime_connection(&runtime, &events, &mut connection).await;
                        }
                        handle_command(command, &mut connection, &mut queue, &events).await;
                    }
                    Some(BackendCommandEnvelope::Shutdown(done)) => {
                        let _ = connection.shutdown().await;
                        let _ = events.send(BackendEvent::Disconnected);
                        let _ = done.send(());
                        break;
                    }
                    None => {
                        let _ = connection.shutdown().await;
                        break;
                    }
                }
            }
            event = connection.next_event() => {
                match event {
                    Some(event) => {
                        queue.push(event);
                        flush_events(&mut queue, &events);
                        events_since_connected = events_since_connected.saturating_add(1);
                    }
                    None => {
                        queue.push(BackendEvent::Disconnected);
                        queue.push(BackendEvent::Reconnecting);
                        flush_events(&mut queue, &events);
                        // Boucle de reconnexion persistante avec backoff exponentiel
                        loop {
                            reconnect_attempt += 1;
                            reconnect_delay(reconnect_attempt).await;
                            if let Some(new_connection) = connect_runtime(&runtime, &events).await {
                                connection = new_connection;
                                reconnect_attempt = 0;
                                events_since_connected = 0;
                                break;
                            }
                        }
                    }
                }
            }
        }
        // Après suffisamment d'événements reçus sans erreur, le compteur peut
        // redescendre rapidement (connexion stable).
        if events_since_connected >= RECONNECT_RESET_ATTEMPTS && reconnect_attempt > 0 {
            reconnect_attempt = reconnect_attempt.saturating_sub(1);
            events_since_connected = 0;
        }
    }
}

async fn connect_runtime(
    runtime: &RuntimeManager,
    events: &mpsc::UnboundedSender<BackendEvent>,
) -> Option<BackendConnection> {
    let endpoint = match runtime.ensure_running_with_env().await {
        Ok(endpoint) => endpoint,
        Err(error) => {
            let _ = events.send(BackendEvent::Error {
                message: error.to_string(),
            });
            return None;
        }
    };

    let mut connection = match BackendConnection::connect(endpoint).await {
        Ok(conn) => conn,
        Err(error) => {
            let _ = events.send(BackendEvent::Error {
                message: error.to_string(),
            });
            return None;
        }
    };

    // === Health check post-connexion (équivalent OpenCode `/global/health`) ===
    match connection.probe_health().await {
        Ok(()) => {
            let _ = events.send(BackendEvent::Connected);
            let _ = events.send(BackendEvent::HealthCheckPassed);
            Some(connection)
        }
        Err(error) => {
            let _ = events.send(BackendEvent::HealthCheckFailed {
                message: error.to_string(),
            });
            // Tenter une reconnexion sera fait par la boucle parente
            None
        }
    }
}

async fn restart_runtime_connection(
    runtime: &RuntimeManager,
    events: &mpsc::UnboundedSender<BackendEvent>,
    connection: &mut BackendConnection,
) {
    match runtime.restart_with_provider_env().await {
        Ok(endpoint) => match BackendConnection::connect(endpoint).await {
            Ok(new_connection) => {
                *connection = new_connection;
                let _ = events.send(BackendEvent::Connected);
                let _ = events.send(BackendEvent::HealthCheckPassed);
                log::info!("Pipeline: runtime redémarré et reconnecté");
            }
            Err(error) => {
                let _ = events.send(BackendEvent::Error {
                    message: format!("Reconnexion impossible après redémarrage : {error}"),
                });
            }
        },
        Err(error) => {
            let _ = events.send(BackendEvent::Error {
                message: format!("Impossible de redémarrer le runtime : {error}"),
            });
        }
    }
}

async fn handle_command(
    command: BackendCommand,
    connection: &mut BackendConnection,
    queue: &mut BackendEventQueue,
    events: &mpsc::UnboundedSender<BackendEvent>,
) {
    match command {
        BackendCommand::Connect => {
            let _ = events.send(BackendEvent::Progress {
                message: "Connexion déjà active".into(),
            });
        }
        BackendCommand::SendMessage {
            conversation_id,
            content,
            runtime_config,
        } => match connection
            .send_message(conversation_id, content, runtime_config)
            .await
        {
            Ok(runtime_events) => {
                for event in runtime_events {
                    queue.push(event);
                }
                flush_events(queue, events);
            }
            Err(error) => {
                let _ = events.send(BackendEvent::Error {
                    message: crate::platform::provider_auth::humanize_runtime_error(
                        &error.to_string(),
                    ),
                });
            }
        },
        BackendCommand::SyncRuntimeConfig { runtime_config } => {
            log::info!("Pipeline: synchronisation config runtime");
            if let Err(error) = connection.sync_runtime_config(&runtime_config).await {
                let _ = events.send(BackendEvent::Error {
                    message: crate::platform::provider_auth::humanize_runtime_error(
                        &error.to_string(),
                    ),
                });
            } else {
                let _ = events.send(BackendEvent::Progress {
                    message: "Configuration runtime synchronisée".into(),
                });
            }
        },
        BackendCommand::OpenWorkspace { path } => {
            let _ = events.send(BackendEvent::Progress {
                message: format!(
                    "Workspace actif : {}",
                    path.unwrap_or_else(|| "par défaut".into())
                ),
            });
        }
        BackendCommand::StopGeneration { conversation_id } => {
            match connection.stop_generation(conversation_id).await {
                Ok(runtime_events) => {
                    for event in runtime_events {
                        queue.push(event);
                    }
                    flush_events(queue, events);
                }
                Err(error) => {
                    let _ = events.send(BackendEvent::Error {
                        message: error.to_string(),
                    });
                }
            }
        }
        BackendCommand::RestartRuntime => {
            // Déjà traité dans la boucle select avant handle_command
        }
        BackendCommand::Disconnect | BackendCommand::Shutdown => {}
        BackendCommand::ResolveServerRequest { request_id, result } => {
            let _ = connection
                .resolve_server_request(request_id, result)
                .await
                .map_err(|error| {
                    let _ = events.send(BackendEvent::Error {
                        message: error.to_string(),
                    });
                });
        }
        BackendCommand::RejectServerRequest { request_id, error } => {
            let _ = connection
                .reject_server_request(request_id, error)
                .await
                .map_err(|error| {
                    let _ = events.send(BackendEvent::Error {
                        message: error.to_string(),
                    });
                });
        }
    }
}

fn flush_events(queue: &mut BackendEventQueue, events: &mpsc::UnboundedSender<BackendEvent>) {
    for event in queue.drain() {
        let _ = events.send(event);
    }
}
