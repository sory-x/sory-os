// SPDX-License-Identifier: GPL-3.0-only

use std::time::Duration;

use sory_app_server_client::{
    AppServerClient, AppServerEvent, DEFAULT_IN_PROCESS_CHANNEL_CAPACITY, RemoteAppServerClient,
    RemoteAppServerConnectArgs, RemoteAppServerEndpoint,
};
use sory_app_server_protocol::{
    ClientRequest, ConfigBatchWriteParams, ConfigRequirementsReadResponse,
    ConfigWriteResponse, JSONRPCErrorError, ModelListParams, ModelListResponse, RequestId,
    Result as JsonRpcResult, ServerNotification, ServerRequest, ThreadItem, ThreadStartParams,
    ThreadStartResponse, TurnInterruptParams, TurnInterruptResponse, TurnStartParams,
    TurnStartResponse, UserInput,
};
use sory_utils_absolute_path::AbsolutePathBuf;
use rand::Rng;
use tokio::time::{sleep, timeout};

use super::{
    BackendError, BackendEvent, BackendResult, RuntimeMessageConfig, runtime::RuntimeEndpoint,
    session::RuntimeSessionMap,
};

const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(5);
const RECONNECT_BASE_DELAY: Duration = Duration::from_millis(100);
const RECONNECT_MAX_DELAY: Duration = Duration::from_secs(8);
const RECONNECT_JITTER_FACTOR: f64 = 0.25;
pub const RECONNECT_RESET_ATTEMPTS: u32 = 10;

/// Calcule un délai de reconnexion avec backoff exponentiel + jitter.
///
/// - `attempt` : nombre de tentatives déjà effectuées (0 = première).
/// - Retourne un délai entre `base * 2^attempt * (1 - jitter)` et `base * 2^attempt * (1 + jitter)`,
///   plafonné à `max_delay`.
///
/// Modèle OpenCode / Google GRPC : évite les thundering herds et les boucles
/// de reconnexion synchronisées.
pub fn reconnect_delay_with_backoff(attempt: u32) -> Duration {
    let base_ms = RECONNECT_BASE_DELAY.as_millis() as f64;
    let max_ms = RECONNECT_MAX_DELAY.as_millis() as f64;
    let exponent = 2u32.saturating_pow(attempt.min(16));
    let delay_ms = (base_ms * exponent as f64).min(max_ms);
    let jitter_range = delay_ms * RECONNECT_JITTER_FACTOR;
    let mut rng = rand::thread_rng();
    let jitter = rng.gen_range(-jitter_range..jitter_range);
    let final_ms = (delay_ms + jitter).max(100.0) as u64; // jamais moins de 100ms
    Duration::from_millis(final_ms)
}

/// Connexion active au protocole app-server officiel du runtime.
pub struct BackendConnection {
    client: AppServerClient,
    sessions: RuntimeSessionMap,
    next_request_id: i64,
}

impl BackendConnection {
    pub async fn connect(endpoint: RuntimeEndpoint) -> BackendResult<Self> {
        let socket_path = AbsolutePathBuf::from_absolute_path(endpoint.socket_path.as_path())
            .map_err(|error| BackendError::Connection(error.to_string()))?;

        let remote = RemoteAppServerClient::connect(RemoteAppServerConnectArgs {
            endpoint: RemoteAppServerEndpoint::UnixSocket { socket_path },
            client_name: "sory-ia-desktop".into(),
            client_version: env!("CARGO_PKG_VERSION").into(),
            experimental_api: true,
            opt_out_notification_methods: Vec::new(),
            channel_capacity: DEFAULT_IN_PROCESS_CHANNEL_CAPACITY,
        })
        .await
        .map_err(|error| BackendError::Connection(error.to_string()))?;

        Ok(Self {
            client: AppServerClient::Remote(remote),
            sessions: RuntimeSessionMap::default(),
            next_request_id: 1,
        })
    }

    /// Health check post-connexion — équivalent de `/global/health` chez OpenCode.
    ///
    /// Utilise `ConfigRequirementsRead` comme sonde légère read-only.
    /// Si le runtime ne répond pas dans le délai imparti, la connexion est
    /// considérée comme non fonctionnelle et `BackendConnection` retourne
    /// une erreur de health check.
    pub async fn probe_health(&mut self) -> BackendResult<()> {
        let request_id = self.request_id();
        let health = timeout(
            HEALTH_CHECK_TIMEOUT,
            self.client.request_typed::<ConfigRequirementsReadResponse>(
                ClientRequest::ConfigRequirementsRead {
                    request_id,
                    params: None,
                },
            ),
        )
        .await
        .map_err(|_elapsed| {
            BackendError::HealthCheck(
                "le runtime n'a pas répondu au health check dans les 5s".into(),
            )
        })?;

        match health {
            Ok(_response) => Ok(()),
            Err(error) => Err(BackendError::HealthCheck(format!(
                "le runtime a répondu mais le health check a échoué : {error}"
            ))),
        }
    }

    pub async fn send_message(
        &mut self,
        conversation_id: uuid::Uuid,
        content: String,
        runtime_config: RuntimeMessageConfig,
    ) -> BackendResult<Vec<BackendEvent>> {
        log::info!(
            "Pipeline: envoi message (provider={}, model={})",
            runtime_config.provider_id,
            runtime_config.model
        );
        self.apply_runtime_config(&runtime_config).await?;
        self.refresh_model_catalog().await?;

        let (thread_id, linked) = self
            .ensure_thread(conversation_id, &runtime_config)
            .await?;
        let request_id = self.request_id();
        let response: TurnStartResponse = self
            .client
            .request_typed(ClientRequest::TurnStart {
                request_id,
                params: TurnStartParams {
                    thread_id: thread_id.clone(),
                    input: vec![UserInput::Text {
                        text: content,
                        text_elements: Vec::new(),
                    }],
                    responsesapi_client_metadata: None,
                    environments: None,
                    cwd: None,
                    runtime_workspace_roots: None,
                    approval_policy: None,
                    approvals_reviewer: None,
                    sandbox_policy: None,
                    permissions: None,
                    model: Some(runtime_config.model.clone()),
                    service_tier: None,
                    effort: None,
                    summary: None,
                    personality: None,
                    output_schema: None,
                    collaboration_mode: None,
                },
            })
            .await
            .map_err(|error| {
                BackendError::Transport(crate::platform::provider_auth::humanize_runtime_error(
                    &error.to_string(),
                ))
            })?;
        log::info!("Pipeline: tour démarré ({})", response.turn.id);
        self.sessions
            .set_active_turn(conversation_id, response.turn.id.clone());
        let mut events = Vec::new();
        if linked {
            events.push(BackendEvent::ConversationLinked {
                conversation_id,
                thread_id,
            });
        }
        Ok(events)
    }

    pub async fn sync_runtime_config(
        &mut self,
        runtime_config: &RuntimeMessageConfig,
    ) -> BackendResult<()> {
        self.apply_runtime_config(runtime_config).await
    }

    async fn apply_runtime_config(
        &mut self,
        runtime_config: &RuntimeMessageConfig,
    ) -> BackendResult<()> {
        if runtime_config.config_edits.is_empty() {
            return Ok(());
        }

        let request_id = self.request_id();
        let _: ConfigWriteResponse = self
            .client
            .request_typed(ClientRequest::ConfigBatchWrite {
                request_id,
                params: ConfigBatchWriteParams {
                    edits: runtime_config.config_edits.clone(),
                    file_path: None,
                    expected_version: None,
                    reload_user_config: true,
                },
            })
            .await
            .map_err(|error| BackendError::Transport(error.to_string()))?;

        Ok(())
    }

    /// Rafraîchit le catalogue de modèles côté runtime (équivalent `model/list`).
    async fn refresh_model_catalog(&mut self) -> BackendResult<()> {
        let request_id = self.request_id();
        let response: ModelListResponse = self
            .client
            .request_typed(ClientRequest::ModelList {
                request_id,
                params: ModelListParams {
                    include_hidden: Some(true),
                    ..ModelListParams::default()
                },
            })
            .await
            .map_err(|error| BackendError::Transport(error.to_string()))?;
        log::info!(
            "Pipeline: catalogue modèles rafraîchi ({} entrées)",
            response.data.len()
        );
        Ok(())
    }

    async fn ensure_thread(
        &mut self,
        conversation_id: uuid::Uuid,
        runtime_config: &RuntimeMessageConfig,
    ) -> BackendResult<(String, bool)> {
        if let Some(thread_id) = self.sessions.get_thread(conversation_id) {
            return Ok((thread_id.to_owned(), false));
        }

        let request_id = self.request_id();
        let response: ThreadStartResponse = self
            .client
            .request_typed(ClientRequest::ThreadStart {
                request_id,
                params: ThreadStartParams {
                    model: Some(runtime_config.model.clone()),
                    model_provider: Some(runtime_config.provider_id.clone()),
                    ephemeral: Some(false),
                    ..ThreadStartParams::default()
                },
            })
            .await
            .map_err(|error| BackendError::Transport(error.to_string()))?;

        let thread_id = response.thread.id;
        self.sessions
            .link_thread(conversation_id, thread_id.clone());
        Ok((thread_id, true))
    }

    fn request_id(&mut self) -> RequestId {
        let id = self.next_request_id;
        self.next_request_id += 1;
        RequestId::Integer(id)
    }

    pub async fn stop_generation(
        &mut self,
        conversation_id: uuid::Uuid,
    ) -> BackendResult<Vec<BackendEvent>> {
        let Some(thread_id) = self.sessions.get_thread(conversation_id).map(str::to_owned) else {
            return Ok(vec![BackendEvent::Progress {
                message: "Aucune session runtime active à interrompre".into(),
            }]);
        };

        let Some(turn_id) = self
            .sessions
            .active_turn(conversation_id)
            .map(str::to_owned)
        else {
            return Ok(vec![BackendEvent::Progress {
                message: "Aucune génération active à interrompre".into(),
            }]);
        };

        let request_id = self.request_id();
        let _response: TurnInterruptResponse = self
            .client
            .request_typed(ClientRequest::TurnInterrupt {
                request_id,
                params: TurnInterruptParams {
                    thread_id,
                    turn_id: turn_id.clone(),
                },
            })
            .await
            .map_err(|error| BackendError::Transport(error.to_string()))?;

        Ok(vec![BackendEvent::AgentFinished {
            conversation_id: Some(conversation_id),
            turn_id: Some(turn_id),
        }])
    }

    pub async fn next_event(&mut self) -> Option<BackendEvent> {
        loop {
            let event = self.client.next_event().await?;
            if let Some(mapped) = map_app_server_event(event) {
                return Some(mapped);
            }
        }
    }

    pub async fn resolve_server_request(
        &mut self,
        request_id: RequestId,
        result: JsonRpcResult,
    ) -> BackendResult<()> {
        match &mut self.client {
            AppServerClient::InProcess(client) => client
                .resolve_server_request(request_id, result)
                .await
                .map_err(|error| BackendError::Transport(error.to_string())),
            AppServerClient::Remote(client) => client
                .resolve_server_request(request_id, result)
                .await
                .map_err(|error| BackendError::Transport(error.to_string())),
        }
    }

    pub async fn reject_server_request(
        &mut self,
        request_id: RequestId,
        error: JSONRPCErrorError,
    ) -> BackendResult<()> {
        match &mut self.client {
            AppServerClient::InProcess(client) => client
                .reject_server_request(request_id, error)
                .await
                .map_err(|error| BackendError::Transport(error.to_string())),
            AppServerClient::Remote(client) => client
                .reject_server_request(request_id, error)
                .await
                .map_err(|error| BackendError::Transport(error.to_string())),
        }
    }

    pub async fn shutdown(self) -> BackendResult<()> {
        self.client
            .shutdown()
            .await
            .map_err(|error| BackendError::Transport(error.to_string()))
    }
}

/// Attente avec backoff exponentiel entre les tentatives de reconnexion.
///
/// `attempt` est incrémenté à chaque échec et réinitialisé quand une connexion
/// tient plus de `RECONNECT_RESET_ATTEMPTS` événements.
pub async fn reconnect_delay(attempt: u32) {
    let delay = reconnect_delay_with_backoff(attempt);
    sleep(delay).await;
}

/// Mappe un événement brut du protocole app-server en BackendEvent normalisé.
fn map_app_server_event(event: AppServerEvent) -> Option<BackendEvent> {
    match event {
        AppServerEvent::Lagged { skipped } => Some(BackendEvent::Progress {
            message: format!("{skipped} événements de progression ont été ignorés"),
        }),
        AppServerEvent::Disconnected { message } => Some(BackendEvent::Error { message }),
        AppServerEvent::ServerRequest(request) => Some(map_server_request(request)),
        AppServerEvent::ServerNotification(notification) => map_notification(notification),
    }
}

/// Mappe les requêtes serveur (approbations, permissions, questions, etc.).
fn map_server_request(request: ServerRequest) -> BackendEvent {
    match request {
        ServerRequest::CommandExecutionRequestApproval { request_id, params } => {
            let command = params.command.unwrap_or_else(|| "commande shell".into());
            let cwd = params
                .cwd
                .as_ref()
                .map(|path| path.to_string_lossy().to_string())
                .unwrap_or_else(|| "workspace courant".into());
            let reason = params
                .reason
                .unwrap_or_else(|| "Autorisation requise par le runtime".into());
            BackendEvent::ToolApprovalRequested {
                conversation_id: None,
                tool: command.clone(),
                details: format!("{reason}\nDossier : {cwd}\nCommande : {command}"),
                request_id,
                thread_id: params.thread_id,
                turn_id: params.turn_id,
                item_id: params.item_id,
            }
        }
        ServerRequest::FileChangeRequestApproval { request_id, params } => {
            BackendEvent::PermissionRequested {
                conversation_id: None,
                title: "Modification de fichiers".into(),
                details: format!(
                    "{}{}",
                    params
                        .reason
                        .unwrap_or_else(|| "Sory IA demande une autorisation d'écriture.".into()),
                    params
                        .grant_root
                        .map(|path| format!("\nRacine demandée : {}", path.display()))
                        .unwrap_or_default()
                ),
                request_id,
                thread_id: params.thread_id,
                turn_id: params.turn_id,
                item_id: params.item_id,
            }
        }
        ServerRequest::PermissionsRequestApproval { request_id, params } => {
            BackendEvent::PermissionRequested {
                conversation_id: None,
                title: "Permissions supplémentaires".into(),
                details: format!(
                    "{}\nDossier : {}\nPermissions : {:?}",
                    params.reason.unwrap_or_else(|| {
                        "Sory IA demande des permissions supplémentaires.".into()
                    }),
                    params.cwd.to_string_lossy(),
                    params.permissions
                ),
                request_id,
                thread_id: params.thread_id,
                turn_id: params.turn_id,
                item_id: params.item_id,
            }
        }
        ServerRequest::ToolRequestUserInput { request_id, params } => BackendEvent::QuestionAsked {
            conversation_id: None,
            prompt: "Sory IA a besoin d'une précision".into(),
            details: format!("Questions : {:?}", params.questions),
            request_id,
            thread_id: params.thread_id,
            turn_id: params.turn_id,
            item_id: params.item_id,
        },
        ServerRequest::McpServerElicitationRequest { request_id, params } => {
            BackendEvent::QuestionAsked {
                conversation_id: None,
                prompt: "Configuration MCP requise".into(),
                details: format!("{:?}", params),
                request_id,
                thread_id: params.thread_id,
                turn_id: params.turn_id.unwrap_or_default(),
                item_id: String::new(),
            }
        }
        ServerRequest::DynamicToolCall { params, .. } => BackendEvent::ToolStarted {
            conversation_id: None,
            name: format!("Outil dynamique : {}", params.tool),
        },
        // Requêtes legacy (API v1) — champs thread_id/turn_id/item_id absents
        ServerRequest::ExecCommandApproval { request_id, params } => {
            BackendEvent::ToolApprovalRequested {
                conversation_id: None,
                tool: "exécution".into(),
                details: format!(
                    "Exécution de commande (legacy)\nCommande : {}",
                    params.command.join(" ")
                ),
                request_id,
                thread_id: String::new(),
                turn_id: String::new(),
                item_id: String::new(),
            }
        }
        ServerRequest::ApplyPatchApproval { request_id, params } => {
            BackendEvent::PermissionRequested {
                conversation_id: None,
                title: "Application de patch".into(),
                details: format!(
                    "Application de patch (legacy)\nFichiers : {}",
                    params.file_changes.len()
                ),
                request_id,
                thread_id: String::new(),
                turn_id: String::new(),
                item_id: String::new(),
            }
        }
        ServerRequest::ChatgptAuthTokensRefresh { .. }
        | ServerRequest::AttestationGenerate { .. } => BackendEvent::Progress {
            message: "Opération interne du runtime ignorée par l'interface".into(),
        },
    }
}

/// Mappe toutes les notifications du protocole app-server.
///
/// Couvre les notifications streaming essentielles : texte, outils,
/// événements de thread, avertissements, etc.
fn map_notification(notification: ServerNotification) -> Option<BackendEvent> {
    match notification {
        // === Streaming de texte ===
        ServerNotification::AgentMessageDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::CommandExecutionOutputDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::CommandExecOutputDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta_base64.clone(),
        }),
        ServerNotification::ProcessOutputDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta_base64.clone(),
        }),
        ServerNotification::FileChangeOutputDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::PlanDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::ReasoningTextDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::ReasoningSummaryTextDelta(delta) => Some(BackendEvent::Token {
            conversation_id: None,
            token: delta.delta,
        }),
        ServerNotification::ReasoningSummaryPartAdded(_) => Some(BackendEvent::Progress {
            message: "Ajout d'une partie au résumé de raisonnement".into(),
        }),

        // === Événements de cycle de vie du turn ===
        ServerNotification::TurnStarted(started) => Some(BackendEvent::AgentStep {
            conversation_id: None,
            label: format!("Tour {} démarré", started.turn.id),
        }),
        ServerNotification::TurnCompleted(completed) => Some(BackendEvent::AgentFinished {
            conversation_id: None,
            turn_id: Some(completed.turn.id),
        }),
        ServerNotification::TurnPlanUpdated(updated) => Some(BackendEvent::AgentStep {
            conversation_id: None,
            label: format!(
                "Plan mis à jour : {}",
                updated.explanation.unwrap_or_default()
            ),
        }),
        ServerNotification::TurnDiffUpdated(_) => Some(BackendEvent::Progress {
            message: "Mise à jour des différences du tour".into(),
        }),

        // === Événements d'item ===
        ServerNotification::ItemStarted(item) => {
            if thread_item_is_tool(&item.item) {
                Some(BackendEvent::ToolStarted {
                    conversation_id: None,
                    name: thread_item_label(&item.item),
                })
            } else if matches!(item.item, ThreadItem::Reasoning { .. }) {
                Some(BackendEvent::AgentStep {
                    conversation_id: None,
                    label: "Raisonnement en cours…".into(),
                })
            } else {
                None
            }
        },
        ServerNotification::ItemCompleted(item) => match &item.item {
            ThreadItem::AgentMessage { text, .. } if !text.trim().is_empty() => {
                Some(BackendEvent::Token {
                    conversation_id: None,
                    token: text.clone(),
                })
            }
            item if thread_item_is_tool(item) => Some(BackendEvent::ToolFinished {
                conversation_id: None,
                name: thread_item_label(item),
            }),
            _ => None,
        },
        ServerNotification::ItemGuardianApprovalReviewStarted(_) => Some(BackendEvent::Progress {
            message: "Examen de l'approbation automatique en cours…".into(),
        }),
        ServerNotification::ItemGuardianApprovalReviewCompleted(_) => Some(BackendEvent::Progress {
            message: "Examen de l'approbation automatique terminé".into(),
        }),

        // === Événements de thread ===
        ServerNotification::ThreadStarted(thread) => Some(BackendEvent::Progress {
            message: format!("Thread démarré : {}", thread.thread.id),
        }),
        ServerNotification::ThreadStatusChanged(status) => Some(BackendEvent::AgentStep {
            conversation_id: None,
            label: format!("État du thread : {:?}", status.status),
        }),
        ServerNotification::ThreadNameUpdated(update) => Some(BackendEvent::Progress {
            message: format!(
                "Thread renommé : {}",
                update.thread_name.as_deref().unwrap_or("(sans nom)")
            ),
        }),
        ServerNotification::ThreadArchived(_) => Some(BackendEvent::Progress {
            message: "Thread archivé".into(),
        }),
        ServerNotification::ThreadUnarchived(_) => Some(BackendEvent::Progress {
            message: "Thread désarchivé".into(),
        }),
        ServerNotification::ThreadClosed(_) => Some(BackendEvent::Progress {
            message: "Thread fermé".into(),
        }),
        ServerNotification::ThreadTokenUsageUpdated(usage) => Some(BackendEvent::Progress {
            message: format!(
                "Tokens utilisés : {}",
                usage.token_usage.total.input_tokens + usage.token_usage.total.output_tokens
            ),
        }),

        // === Notifications MCP ===
        ServerNotification::McpToolCallProgress(progress) => Some(BackendEvent::ToolStarted {
            conversation_id: None,
            name: format!("MCP : {}", progress.message),
        }),
        ServerNotification::McpServerStatusUpdated(status) => Some(BackendEvent::Progress {
            message: format!("Serveur MCP {} : {:?}", status.name, status.status),
        }),
        ServerNotification::McpServerOauthLoginCompleted(login) => Some(BackendEvent::Progress {
            message: format!("Connexion OAuth MCP terminée pour {}", login.name),
        }),

        // === Événements de terminal/filesystem ===
        ServerNotification::TerminalInteraction(_) => Some(BackendEvent::Progress {
            message: "Interaction terminal".into(),
        }),
        ServerNotification::FileChangePatchUpdated(_) => Some(BackendEvent::Progress {
            message: "Patch de fichiers mis à jour".into(),
        }),
        ServerNotification::FsChanged(_) => Some(BackendEvent::Progress {
            message: "Modifications du système de fichiers détectées".into(),
        }),

        // === Avertissements et erreurs ===
        ServerNotification::Error(error) => {
            if error.will_retry {
                Some(BackendEvent::Warning {
                    message: error.error.message,
                })
            } else {
                Some(BackendEvent::Error {
                    message: error.error.message,
                })
            }
        }
        ServerNotification::Warning(warning) => {
            if is_fallback_model_metadata_warning(&warning.message) {
                log::debug!("Métadonnées modèle (fallback) : {}", warning.message);
                None
            } else {
                Some(BackendEvent::Warning {
                    message: warning.message,
                })
            }
        }
        ServerNotification::GuardianWarning(warning) => Some(BackendEvent::PermissionRequested {
            conversation_id: None,
            title: "Avertissement Guardian".into(),
            details: warning.message,
            request_id: sory_app_server_protocol::RequestId::Integer(0),
            thread_id: warning.thread_id.clone(),
            turn_id: String::new(),
            item_id: String::new(),
        }),
        ServerNotification::DeprecationNotice(notice) => Some(BackendEvent::Progress {
            message: notice.summary,
        }),
        ServerNotification::ConfigWarning(cfg) => Some(BackendEvent::Progress {
            message: format!("Avertissement config : {}", cfg.summary),
        }),
        ServerNotification::WindowsWorldWritableWarning(warning) => Some(BackendEvent::Error {
            message: format!(
                "Répertoires mondialement accessibles : {:?}",
                warning.sample_paths
            ),
        }),
        ServerNotification::WindowsSandboxSetupCompleted(_) => Some(BackendEvent::Progress {
            message: "Configuration du sandbox Windows terminée".into(),
        }),

        // === Comptes et authentification ===
        ServerNotification::AccountUpdated(_) => Some(BackendEvent::Progress {
            message: "Compte mis à jour".into(),
        }),
        ServerNotification::AccountRateLimitsUpdated(_) => Some(BackendEvent::Progress {
            message: "Limites de taux mises à jour".into(),
        }),
        ServerNotification::AccountLoginCompleted(_) => Some(BackendEvent::Progress {
            message: "Connexion au compte terminée".into(),
        }),
        ServerNotification::RemoteControlStatusChanged(_) => Some(BackendEvent::Progress {
            message: "Statut du contrôle à distance modifié".into(),
        }),
        ServerNotification::AppListUpdated(_) => Some(BackendEvent::Progress {
            message: "Liste des applications mise à jour".into(),
        }),

        // === Notifications diverses ===
        ServerNotification::SkillsChanged(_) => Some(BackendEvent::Progress {
            message: "Compétences mises à jour".into(),
        }),
        ServerNotification::ModelRerouted(rerouted) => Some(BackendEvent::Progress {
            message: format!("Modèle redirigé vers {}", rerouted.to_model),
        }),
        ServerNotification::ModelVerification(verif) => Some(BackendEvent::Progress {
            message: format!(
                "Vérification du modèle : {} vérifications",
                verif.verifications.len()
            ),
        }),
        ServerNotification::ContextCompacted(_) => Some(BackendEvent::Progress {
            message: "Contexte compacté".into(),
        }),
        ServerNotification::ServerRequestResolved(resolved) => Some(BackendEvent::Progress {
            message: format!("Requête serveur résolue : {}", resolved.request_id),
        }),
        ServerNotification::ExternalAgentConfigImportCompleted(_) => Some(BackendEvent::Progress {
            message: "Import de configuration d'agent externe terminé".into(),
        }),
        ServerNotification::FuzzyFileSearchSessionUpdated(_)
        | ServerNotification::FuzzyFileSearchSessionCompleted(_) => Some(BackendEvent::Progress {
            message: "Recherche de fichiers".into(),
        }),
        ServerNotification::HookStarted(hook) => Some(BackendEvent::ToolStarted {
            conversation_id: None,
            name: format!("Hook : {:?}", hook.run),
        }),
        ServerNotification::HookCompleted(hook) => Some(BackendEvent::ToolFinished {
            conversation_id: None,
            name: format!("Hook : {:?}", hook.run),
        }),
        ServerNotification::ThreadGoalUpdated(_) => Some(BackendEvent::Progress {
            message: "Objectif du thread mis à jour".into(),
        }),
        ServerNotification::ThreadGoalCleared(_) => Some(BackendEvent::Progress {
            message: "Objectif du thread effacé".into(),
        }),
        ServerNotification::ThreadRealtimeStarted(_)
        | ServerNotification::ThreadRealtimeItemAdded(_)
        | ServerNotification::ThreadRealtimeTranscriptDelta(_)
        | ServerNotification::ThreadRealtimeTranscriptDone(_)
        | ServerNotification::ThreadRealtimeOutputAudioDelta(_)
        | ServerNotification::ThreadRealtimeSdp(_)
        | ServerNotification::ThreadRealtimeError(_)
        | ServerNotification::ThreadRealtimeClosed(_) => Some(BackendEvent::Progress {
            message: "Événement temps réel du thread".into(),
        }),
        ServerNotification::ProcessExited(exit) => Some(BackendEvent::ToolFinished {
            conversation_id: None,
            name: format!("Processus (code {})", exit.exit_code),
        }),
        ServerNotification::RawResponseItemCompleted(_) => Some(BackendEvent::Progress {
            message: "Élément de réponse brute terminé".into(),
        }),
    }
}

fn is_fallback_model_metadata_warning(message: &str) -> bool {
    message.starts_with("Model metadata for `")
        && message.contains("not found. Defaulting to fallback metadata")
}

fn thread_item_is_tool(item: &ThreadItem) -> bool {
    !matches!(
        item,
        ThreadItem::UserMessage { .. }
            | ThreadItem::AgentMessage { .. }
            | ThreadItem::Reasoning { .. }
            | ThreadItem::Plan { .. }
            | ThreadItem::HookPrompt { .. }
    )
}

fn thread_item_label(item: &ThreadItem) -> String {
    match item {
        ThreadItem::UserMessage { .. } => "Message utilisateur".into(),
        ThreadItem::HookPrompt { fragments, .. } => {
            format!("Hook : {} fragment(s)", fragments.len())
        }
        ThreadItem::AgentMessage { text, .. } => {
            if text.trim().is_empty() {
                "Réponse Sory IA".into()
            } else {
                format!("Réponse Sory IA : {}", compact_label(text))
            }
        }
        ThreadItem::Plan { text, .. } => format!("Plan : {}", compact_label(text)),
        ThreadItem::Reasoning { .. } => "Raisonnement".into(),
        ThreadItem::CommandExecution { command, .. } => {
            format!("Commande : {}", compact_label(command))
        }
        ThreadItem::FileChange { changes, .. } => {
            format!("Modification de fichiers : {} changement(s)", changes.len())
        }
        ThreadItem::McpToolCall { server, tool, .. } => {
            format!("MCP {server} : {tool}")
        }
        ThreadItem::DynamicToolCall {
            namespace, tool, ..
        } => match namespace {
            Some(namespace) => format!("{namespace} : {tool}"),
            None => format!("Outil : {tool}"),
        },
        ThreadItem::CollabAgentToolCall { tool, .. } => {
            format!("Agent collaboratif : {tool:?}")
        }
        ThreadItem::WebSearch { query, .. } => format!("Recherche web : {}", compact_label(query)),
        ThreadItem::ImageView { path, .. } => {
            format!(
                "Image : {}",
                path.file_name().unwrap_or_default().to_string_lossy()
            )
        }
        ThreadItem::ImageGeneration { status, .. } => {
            format!("Génération image : {status}")
        }
        ThreadItem::EnteredReviewMode { .. } => "Mode revue activé".into(),
        ThreadItem::ExitedReviewMode { .. } => "Mode revue terminé".into(),
        ThreadItem::ContextCompaction { .. } => "Contexte compacté".into(),
    }
}

fn compact_label(value: &str) -> String {
    let line = value.lines().next().unwrap_or(value).trim();
    const MAX_CHARS: usize = 80;
    if line.chars().count() <= MAX_CHARS {
        return line.to_owned();
    }

    let mut compact = line.chars().take(MAX_CHARS).collect::<String>();
    compact.push_str("...");
    compact
}
