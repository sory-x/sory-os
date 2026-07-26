// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{
    Application, Element,
    action,
    app::{Core, Settings, Task},
    executor,
    iced::Subscription,
};

use crate::{
    backend::{BackendClient, BackendCommand, BackendHandle},
    events::AppEvent,
    platform::{provider_auth, settings_store},
    state::AppState,
    theme::{SoryTheme, SoryThemeMode},
    ui,
};

pub fn run() -> cosmic::iced::Result {
    cosmic::app::run::<SoryIaApplication>(Settings::default(), ())
}

pub struct SoryIaApplication {
    core: Core,
    state: AppState,
    backend: BackendHandle,
    theme: SoryTheme,
}

impl Application for SoryIaApplication {
    type Executor = executor::Default;
    type Flags = ();
    type Message = AppEvent;

    const APP_ID: &'static str = "os.sory.ia";

    fn core(&self) -> &Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Self::Message>) {
        // Charger les settings persistés, ou utiliser les defaults
        let mut state = AppState::default();
        if let Some(saved) = settings_store::load_settings() {
            state.settings.settings = saved;
            state.apply_loaded_settings();
        }
        let _ = provider_auth::sync_all_api_keys(&state.settings.settings);

        let runtime_binary =
            crate::platform::runtime_paths::resolve_runtime_binary_or_default(
                &state.settings.settings.runtime_command,
            );
        let backend = BackendClient::new(runtime_binary).connect();
        let theme_mode = match state.settings.settings.theme {
            crate::models::ThemePreference::Dark => SoryThemeMode::Dark,
            _ => SoryThemeMode::Dark,
        };
        let app = Self {
            core,
            state,
            backend,
            theme: SoryTheme { mode: theme_mode },
        };
        // La connexion au runtime est automatiquement tentée par run_backend
        // dans le thread spawned par connect(). Les événements arriveront
        // via BackendTick.
        (app, Task::none())
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        let theme = self.theme.cosmic_theme();
        let cosmic_theme = theme.cosmic();
        Some(cosmic::iced::theme::Style {
            background_color: cosmic_theme.bg_color().into(),
            text_color: cosmic_theme.on_bg_color().into(),
            icon_color: cosmic_theme.on_bg_color().into(),
        })
    }

    fn update(&mut self, event: Self::Message) -> Task<Self::Message> {
        // Détermine si l'événement modifie les settings et nécessite une sauvegarde
        let needs_save = matches!(
            &event,
            AppEvent::SettingsChanged(_)
                | AppEvent::ProviderChanged(_)
                | AppEvent::ProviderEndpointChanged(_, _)
                | AppEvent::ProviderApiKeyChanged(_, _)
                | AppEvent::ProviderModelChanged(_, _)
                | AppEvent::TemperatureChanged(_)
                | AppEvent::ToggleSidebar
                | AppEvent::ToggleWorkspaceSidebar
                | AppEvent::SaveSettings
                | AppEvent::ClipboardPasted { .. }
                | AppEvent::SelectProviderAndReturn(_)
                | AppEvent::SelectModelAndReturn(_, _)
        );

        match &event {
            AppEvent::CopyMessage(message_id) => {
                if let Some(content) = self
                    .state
                    .conversations
                    .conversations
                    .iter()
                    .flat_map(|c| c.messages.iter())
                    .find(|m| m.id == *message_id)
                    .map(|m| m.content.clone())
                {
                    if content.is_empty() {
                        self.state.status = "Rien à copier".into();
                        return Task::none();
                    }
                    self.state.status = "Copi\u{00e9} dans le presse-papiers".into();
                    return Task::batch([
                        cosmic::iced::clipboard::write(content),
                        Task::none(),
                    ]);
                }
            }
            AppEvent::CopyApiKey(provider_id) => {
                let api_key = self
                    .state
                    .settings
                    .settings
                    .provider_configs
                    .get(provider_id)
                    .map(|c| c.api_key.clone())
                    .unwrap_or_default();
                if api_key.is_empty() {
                    self.state.settings_feedback =
                        Some("Aucune clé API à copier".into());
                    return Task::none();
                }
                self.state.settings_feedback =
                    Some("Clé API copiée dans le presse-papiers".into());
                return Task::batch([
                    cosmic::iced::clipboard::write(api_key),
                    Task::none(),
                ]);
            }
            AppEvent::PasteApiKey(provider_id) => {
                let pid = provider_id.clone();
                return cosmic::iced::clipboard::read().map(move |text| {
                    action::app(AppEvent::ClipboardPasted {
                        provider_id: pid.clone(),
                        text,
                    })
                });
            }
            AppEvent::SaveSettings => {
                self.state.reduce(&event);
                let settings = &self.state.settings.settings;

                if let Err(e) = settings_store::save_settings(settings) {
                    self.state.settings_feedback =
                        Some(format!("Erreur d'enregistrement : {e}"));
                    return Task::none();
                }

                for (provider_id, cfg) in &settings.provider_configs {
                    if let Err(e) = provider_auth::sync_api_key(provider_id, &cfg.api_key) {
                        self.state.settings_feedback =
                            Some(format!("Enregistré localement, sync env ({provider_id}) : {e}"));
                        return Task::none();
                    }
                }

                self.backend.send(BackendCommand::SyncRuntimeConfig {
                    runtime_config: settings.runtime_message_config(),
                });

                self.state.settings_feedback =
                    Some("Paramètres enregistrés avec succès".into());
                self.state.status = "Paramètres enregistrés".into();
                return Task::none();
            }
            AppEvent::SendMessage(content) => {
                let conversation_id = self.state.conversations.active_id;
                let runtime_config = self.state.settings.settings.runtime_message_config();

                // Réduire d'abord pour conserver le message utilisateur
                // même si la validation échoue.
                self.state.reduce(&event);

                if let Err(error) =
                    provider_auth::validate_active_provider(&self.state.settings.settings)
                {
                    self.state.reduce(&AppEvent::BackendError(error));
                    return Task::none();
                }

                log::info!(
                    "Desktop → Backend : envoi message (provider={}, model={})",
                    runtime_config.provider_id,
                    runtime_config.model
                );
                self.backend.send(BackendCommand::SendMessage {
                    conversation_id,
                    content: content.clone(),
                    runtime_config,
                });
            }
            AppEvent::BackendTick => {
                let was_connected = self.state.backend_connected;
                while let Ok(runtime_event) = self.backend.events.try_recv() {
                    self.state.apply_backend_event(&runtime_event);
                }
                if !was_connected && self.state.backend_connected {
                    self.backend.send(BackendCommand::SyncRuntimeConfig {
                        runtime_config: self.state.settings.settings.runtime_message_config(),
                    });
                }
            }
            AppEvent::Runtime(runtime_event) => self.state.apply_backend_event(runtime_event),
            AppEvent::StopGeneration => {
                let conversation_id = self.state.conversations.active_id;
                self.state.reduce(&event);
                self.backend
                    .send(BackendCommand::StopGeneration { conversation_id });
            }
            AppEvent::RuntimeActionResolve {
                action_id,
                decision,
            } => {
                self.state.reduce(&event);
                let maybe_action = self
                    .state
                    .runtime_actions
                    .actions
                    .iter()
                    .find(|a| a.id == *action_id)
                    .cloned();
                if let Some(mut action) = maybe_action {
                    let backend_cmd = action.resolve(match decision.as_str() {
                        "accept" => {
                            use sory_app_server_protocol::CommandExecutionApprovalDecision;
                            serde_json::to_value(CommandExecutionApprovalDecision::Accept)
                                .unwrap_or_else(|_| serde_json::json!({}))
                        }
                        _ => serde_json::json!({}),
                    });
                    self.backend.send(backend_cmd);
                }
            }
            AppEvent::RuntimeActionReject { action_id } => {
                self.state.reduce(&event);
                let maybe_action = self
                    .state
                    .runtime_actions
                    .actions
                    .iter()
                    .find(|a| a.id == *action_id)
                    .cloned();
                if let Some(mut action) = maybe_action {
                    let error = sory_app_server_protocol::JSONRPCErrorError {
                        code: -32803,
                        message: "User declined".into(),
                        data: None,
                    };
                    let backend_cmd = action.reject(error);
                    self.backend.send(backend_cmd);
                }
            }
            AppEvent::WorkspaceOpened(path) => {
                self.state.reduce(&event);
                self.backend
                    .send(BackendCommand::OpenWorkspace { path: path.clone() });
            }
            _ => {
                self.state.reduce(&event);

                // Sauvegarde automatique après modification des settings
                if needs_save {
                    if let Err(e) =
                        settings_store::save_settings(&self.state.settings.settings)
                    {
                        log::error!("Échec de la sauvegarde des settings : {e}");
                    }
                }
            }
        }

        Task::none()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        use cosmic::iced::time;
        use std::time::Duration;

        let backend_tick = time::every(Duration::from_millis(50)).map(|_| AppEvent::BackendTick);

        let layout_tick = if self.state.layout_animating() {
            time::every(Duration::from_millis(
                crate::theme::tokens::LAYOUT_ANIMATION_STEP_MS,
            ))
            .map(|_| AppEvent::LayoutAnimationTick)
        } else {
            Subscription::none()
        };

        Subscription::batch([backend_tick, layout_tick])
    }

    fn view(&self) -> Element<Self::Message> {
        ui::view(&self.state)
    }
}
