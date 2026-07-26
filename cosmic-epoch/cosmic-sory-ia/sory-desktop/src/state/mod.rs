// SPDX-License-Identifier: GPL-3.0-only

mod backend_reducer;

use std::collections::HashSet;

use uuid::Uuid;

use crate::{
    events::AppEvent,
    models::{
        AppNotification, Conversation, Message, MessageStatus, NotificationKind, Provider,
        ProviderDefinition, RuntimeAction, RuntimeActionKind, Settings, ToolCall, ToolStatus,
        Workspace,
    },
};

#[derive(Debug, Clone)]
pub struct ConversationState {
    pub active_id: Uuid,
    pub conversations: Vec<Conversation>,
    pub is_generating: bool,
}

impl Default for ConversationState {
    fn default() -> Self {
        let conversation = Conversation::new("Nouvelle conversation");
        Self {
            active_id: conversation.id,
            conversations: vec![conversation],
            is_generating: false,
        }
    }
}

impl ConversationState {
    pub fn active_mut(&mut self) -> Option<&mut Conversation> {
        self.conversations
            .iter_mut()
            .find(|conversation| conversation.id == self.active_id)
    }

    pub fn active(&self) -> Option<&Conversation> {
        self.conversations
            .iter()
            .find(|conversation| conversation.id == self.active_id)
    }

    pub fn create_conversation(&mut self) -> Uuid {
        let conversation = Conversation::new("Nouvelle conversation");
        let id = conversation.id;
        self.conversations.insert(0, conversation);
        self.active_id = id;
        id
    }
}

#[derive(Debug, Clone, Default)]
pub struct SettingsState {
    pub settings: Settings,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ActivePage {
    #[default]
    Chat,
    History,
    Favorites,
    Settings,
    Workspace,
    About,
    ModelPicker,
    ProviderPicker,
}

#[derive(Debug, Clone, Default)]
pub struct SidebarState {
    /// Progression d'animation 0.0 (réduit) → 1.0 (développé).
    pub expand_progress: f32,
    pub active_page: ActivePage,
}

impl SidebarState {
    pub fn is_expanded(&self) -> bool {
        self.expand_progress > 0.5
    }

    pub fn effective_width(&self) -> f32 {
        let collapsed = f32::from(crate::theme::tokens::SIDEBAR_COLLAPSED_WIDTH);
        let expanded = f32::from(crate::theme::tokens::SIDEBAR_WIDTH);
        collapsed + (expanded - collapsed) * self.expand_progress
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceSidebarState {
    /// Progression d'animation 0.0 (fermé) → 1.0 (ouvert).
    pub expand_progress: f32,
}

impl WorkspaceSidebarState {
    pub fn is_expanded(&self) -> bool {
        self.expand_progress > 0.5
    }

    pub fn effective_width(&self) -> f32 {
        let collapsed = f32::from(crate::theme::tokens::RIGHT_SIDEBAR_COLLAPSED_WIDTH);
        let expanded = f32::from(crate::theme::tokens::RIGHT_SIDEBAR_WIDTH);
        collapsed + (expanded - collapsed) * self.expand_progress
    }
}

#[derive(Debug, Clone, Default)]
pub struct WorkspaceState {
    pub active: Option<Workspace>,
}

#[derive(Debug, Clone, Default)]
pub struct WindowState {
    pub title: String,
}

#[derive(Debug, Clone, Default)]
pub struct HistoryState {
    pub recent_conversations: Vec<Uuid>,
    pub favorites: Vec<Uuid>,
}

#[derive(Debug, Clone)]
pub struct ProviderState {
    /// Liste des définitions de providers connus.
    pub definitions: Vec<ProviderDefinition>,
    /// Provider actif (id).
    pub active_provider_id: String,
    /// Ancien champ Provider (rétrocompatibilité).
    pub providers: Vec<Provider>,
}

impl Default for ProviderState {
    fn default() -> Self {
        let definitions = crate::models::known_providers();
        let providers: Vec<Provider> = definitions
            .iter()
            .map(|d| Provider {
                id: d.id.clone(),
                name: d.name.clone(),
                enabled: d.enabled,
            })
            .collect();
        Self {
            active_provider_id: "soryos-zen".into(),
            definitions,
            providers,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RuntimeStatus {
    #[default]
    Disconnected,
    /// Le daemon est en cours de démarrage / socket pas encore accessible.
    Connecting,
    /// Socket connecté, health check en cours.
    HealthChecking,
    /// Health check réussi, runtime opérationnel.
    Ready,
    /// Connexion perdue, tentative de reconnexion.
    Reconnecting,
    /// Erreur irrécupérable.
    Failed,
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeActionState {
    pub actions: Vec<RuntimeAction>,
}

impl RuntimeActionState {
    pub fn push(&mut self, action: RuntimeAction) {
        self.actions.push(action);
    }
}

#[derive(Debug, Clone, Default)]
pub struct NotificationState {
    pub notifications: Vec<AppNotification>,
}

impl NotificationState {
    pub fn push(&mut self, notification: AppNotification) {
        self.notifications.push(notification);
    }
}

#[derive(Debug, Clone)]
pub struct ApplicationState {
    pub conversations: ConversationState,
    pub settings: SettingsState,
    pub providers: ProviderState,
    pub sidebar: SidebarState,
    pub workspace_sidebar: WorkspaceSidebarState,
    pub workspace: WorkspaceState,
    pub window: WindowState,
    pub history: HistoryState,
    pub notifications: NotificationState,
    pub runtime_actions: RuntimeActionState,
    pub collapsible_expanded: HashSet<String>,
    pub draft_message: String,
    pub backend_connected: bool,
    pub runtime_status: RuntimeStatus,
    pub status: String,
    /// Message de feedback pour la page paramètres (ex. « Clé enregistrée »).
    pub settings_feedback: Option<String>,
}

pub type AppState = ApplicationState;

impl Default for ApplicationState {
    fn default() -> Self {
        Self {
            conversations: ConversationState::default(),
            settings: SettingsState::default(),
            providers: ProviderState::default(),
            sidebar: SidebarState {
                expand_progress: 1.0,
                active_page: ActivePage::Chat,
            },
            workspace_sidebar: WorkspaceSidebarState {
                expand_progress: 1.0,
            },
            workspace: WorkspaceState::default(),
            window: WindowState {
                title: "Sory IA".into(),
            },
            history: HistoryState::default(),
            notifications: NotificationState::default(),
            runtime_actions: RuntimeActionState::default(),
            collapsible_expanded: HashSet::new(),
            draft_message: String::new(),
            backend_connected: false,
            runtime_status: RuntimeStatus::Connecting,
            status: "Démarrage de Sory IA…".into(),
            settings_feedback: None,
        }
    }
}

impl ApplicationState {
    pub fn reduce(&mut self, event: &AppEvent) {
        match event {
            AppEvent::NewConversation => {
                let id = self.conversations.create_conversation();
                self.history.recent_conversations.insert(0, id);
                self.sidebar.active_page = ActivePage::Chat;
            }
            AppEvent::InputChanged(value) => self.draft_message = value.clone(),
            AppEvent::SendMessage(content) => {
                if content.trim().is_empty() {
                    return;
                }

                if let Some(conversation) = self.conversations.active_mut() {
                    conversation.messages.push(Message::user(content));
                    conversation.messages.push(Message::assistant_streaming());
                }
                self.conversations.is_generating = true;
                self.draft_message.clear();
                self.status = "Sory IA réfléchit…".into();
            }
            AppEvent::StopGeneration => {
                self.conversations.is_generating = false;
                self.status = "Génération arrêtée".into();
            }
            AppEvent::ReceiveToken {
                conversation_id,
                token,
            } => {
                if let Some(conversation) = self
                    .conversations
                    .conversations
                    .iter_mut()
                    .find(|c| c.id == *conversation_id)
                {
                    if let Some(message) = conversation
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.status == MessageStatus::Streaming)
                    {
                        if message.content.is_empty() {
                            message.content = token.clone();
                        } else if token.len() >= message.content.len()
                            && token.starts_with(&message.content)
                        {
                            // Token cumulatif : remplace le contenu
                            message.content = token.clone();
                        } else {
                            // Token delta : on append toujours
                            message.content.push_str(token);
                        }
                    }
                }
            }
            AppEvent::ToolStarted {
                conversation_id,
                name,
            } => {
                if let Some(conversation) = self
                    .conversations
                    .conversations
                    .iter_mut()
                    .find(|c| c.id == *conversation_id)
                {
                    if let Some(message) = conversation
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.status == MessageStatus::Streaming)
                    {
                        message.tool_calls.push(ToolCall {
                            id: Uuid::new_v4(),
                            name: name.clone(),
                            status: ToolStatus::Started,
                            summary: None,
                        });
                    }
                }
                self.status = format!("Outil en cours : {name}");
            }
            AppEvent::ToolFinished {
                conversation_id,
                name,
            } => {
                if let Some(conversation) = self
                    .conversations
                    .conversations
                    .iter_mut()
                    .find(|c| c.id == *conversation_id)
                {
                    if let Some(message) = conversation
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.status == MessageStatus::Streaming)
                    {
                        if let Some(tool) =
                            message.tool_calls.iter_mut().rev().find(|tool| {
                                tool.name == *name && tool.status == ToolStatus::Started
                            })
                        {
                            tool.status = ToolStatus::Finished;
                            tool.summary = Some("Action terminée".into());
                        } else {
                            message.tool_calls.push(ToolCall {
                                id: Uuid::new_v4(),
                                name: name.clone(),
                                status: ToolStatus::Finished,
                                summary: Some("Action terminée".into()),
                            });
                        }
                    }
                }
                self.status = format!("Outil terminé : {name}");
            }
            AppEvent::PermissionRequested {
                conversation_id,
                title,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => {
                self.runtime_actions.push(RuntimeAction::new(
                    *conversation_id,
                    RuntimeActionKind::Permission,
                    title.clone(),
                    details.clone(),
                    request_id.clone(),
                    thread_id.clone(),
                    turn_id.clone(),
                    item_id.clone(),
                ));
                self.status = title.clone();
            }
            AppEvent::QuestionAsked {
                conversation_id,
                prompt,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => {
                self.runtime_actions.push(RuntimeAction::new(
                    *conversation_id,
                    RuntimeActionKind::Question,
                    prompt.clone(),
                    details.clone(),
                    request_id.clone(),
                    thread_id.clone(),
                    turn_id.clone(),
                    item_id.clone(),
                ));
                self.status = prompt.clone();
            }
            AppEvent::ToolApprovalRequested {
                conversation_id,
                tool,
                details,
                request_id,
                thread_id,
                turn_id,
                item_id,
            } => {
                self.runtime_actions.push(RuntimeAction::new(
                    *conversation_id,
                    RuntimeActionKind::ToolApproval,
                    format!("Autorisation outil : {tool}"),
                    details.clone(),
                    request_id.clone(),
                    thread_id.clone(),
                    turn_id.clone(),
                    item_id.clone(),
                ));
                self.status = format!("Autorisation requise : {tool}");
            }
            AppEvent::AgentFinished { conversation_id } => {
                if let Some(conversation) = self
                    .conversations
                    .conversations
                    .iter_mut()
                    .find(|c| c.id == *conversation_id)
                {
                    if let Some(message) = conversation
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.status == MessageStatus::Streaming)
                    {
                        if message.content.trim().is_empty() {
                            message.content = "Aucune réponse du modèle. Vérifiez la clé API, le modèle sélectionné et que le runtime est connecté.".into();
                            message.status = MessageStatus::Failed;
                        } else {
                            message.status = MessageStatus::Complete;
                        }
                    }
                }
                self.conversations.is_generating = false;
                self.runtime_status = RuntimeStatus::Ready;
                self.status = "Prêt".into();
                self.notifications.push(AppNotification::new(
                    NotificationKind::GenerationFinished,
                    "Génération terminée",
                    "Sory IA a terminé sa réponse.",
                ));
            }
            AppEvent::BackendConnected => {
                self.backend_connected = true;
                self.runtime_status = RuntimeStatus::Ready;
                self.status = "Connecté à Sory IA".into();
            }
            AppEvent::BackendDisconnected => {
                self.backend_connected = false;
                self.runtime_status = RuntimeStatus::Disconnected;
                self.status = "Sory IA déconnecté".into();
            }
            AppEvent::BackendReconnecting => {
                self.backend_connected = false;
                self.runtime_status = RuntimeStatus::Reconnecting;
                self.status = "Reconnexion à Sory IA…".into();
            }
            AppEvent::BackendError(error) => {
                self.conversations.is_generating = false;
                if let Some(conversation) = self.conversations.active_mut() {
                    if let Some(message) = conversation
                        .messages
                        .iter_mut()
                        .rev()
                        .find(|m| m.status == MessageStatus::Streaming)
                    {
                        if message.content.is_empty() {
                            message.content = error.clone();
                        }
                        message.status = MessageStatus::Failed;
                    }
                }
                self.runtime_status = RuntimeStatus::Failed;
                self.notifications.push(AppNotification::new(
                    NotificationKind::Error,
                    "Erreur Sory IA",
                    error.clone(),
                ));
                self.status = format!("Erreur : {error}");
            }
            AppEvent::BackendWarning(warning) => {
                log::warn!("Avertissement runtime : {warning}");
                self.status = warning.clone();
            }
            AppEvent::ConversationCreated(id) => {
                self.conversations.active_id = *id;
                self.sidebar.active_page = ActivePage::Chat;
            }
            AppEvent::OpenWorkspace(path) => {
                self.sidebar.active_page = ActivePage::Workspace;
                if path.is_some() {
                    // établira workspace plus tard
                }
            }
            AppEvent::SettingsChanged(settings) => {
                self.settings.settings = settings.clone();
            }
            AppEvent::ProviderChanged(provider_id) => {
                self.ensure_provider_config(provider_id);
                self.settings.settings.provider_id = provider_id.clone();
                self.providers.active_provider_id = provider_id.clone();
                if let Some(def) = crate::models::known_providers()
                    .into_iter()
                    .find(|p| p.id == *provider_id)
                {
                    if let Some(cfg) =
                        self.settings.settings.provider_configs.get_mut(provider_id)
                    {
                        if cfg.endpoint.is_empty() {
                            cfg.endpoint = def.endpoint.clone();
                        }
                    }
                }
                self.settings_feedback = Some(format!("Provider actif : {provider_id}"));
            }
            AppEvent::ProviderEndpointChanged(provider_id, endpoint) => {
                if let Some(cfg) = self.settings.settings.provider_configs.get_mut(provider_id) {
                    cfg.endpoint = endpoint.clone();
                }
            }
            AppEvent::ProviderApiKeyChanged(provider_id, api_key) => {
                if let Some(cfg) = self.settings.settings.provider_configs.get_mut(provider_id) {
                    cfg.api_key = api_key.clone();
                }
            }
            AppEvent::ProviderModelChanged(provider_id, model) => {
                if let Some(cfg) = self.settings.settings.provider_configs.get_mut(provider_id) {
                    cfg.model = model.clone();
                }
                self.settings.settings.model = model.clone();
            }
            AppEvent::SelectProviderAndReturn(provider_id) => {
                self.ensure_provider_config(provider_id);
                self.settings.settings.provider_id = provider_id.clone();
                self.providers.active_provider_id = provider_id.clone();
                if let Some(def) = crate::models::known_providers()
                    .into_iter()
                    .find(|p| p.id == *provider_id)
                {
                    if let Some(cfg) =
                        self.settings.settings.provider_configs.get_mut(provider_id)
                    {
                        if cfg.model.is_empty() {
                            cfg.model = def.default_model;
                        }
                        if cfg.endpoint.is_empty() {
                            cfg.endpoint = def.endpoint;
                        }
                    }
                }
                self.sidebar.active_page = ActivePage::Chat;
            }
            AppEvent::SelectModelAndReturn(provider_id, model) => {
                if let Some(cfg) = self.settings.settings.provider_configs.get_mut(provider_id) {
                    cfg.model = model.clone();
                }
                self.settings.settings.model = model.clone();
                self.sidebar.active_page = ActivePage::Chat;
            }
            AppEvent::SaveSettings => {
                self.settings_feedback = Some("Enregistrement…".into());
            }
            AppEvent::ClipboardPasted { provider_id, text } => {
                if let Some(text) = text {
                    let trimmed = text.trim().to_string();
                    if !trimmed.is_empty() {
                        if let Some(cfg) =
                            self.settings.settings.provider_configs.get_mut(provider_id)
                        {
                            cfg.api_key = trimmed;
                        }
                        self.settings_feedback =
                            Some("Clé API collée depuis le presse-papiers".into());
                    }
                } else {
                    self.settings_feedback =
                        Some("Presse-papiers vide ou inaccessible".into());
                }
            }
            AppEvent::TemperatureChanged(temp) => {
                self.settings.settings.temperature = *temp;
            }
            AppEvent::WorkspaceOpened(path) => {
                self.workspace.active = Some(Workspace {
                    id: Uuid::new_v4(),
                    name: path.clone().unwrap_or_else(|| "Workspace".into()),
                    path: path.clone(),
                });
                self.sidebar.active_page = ActivePage::Workspace;
            }
            AppEvent::OpenHistory => self.sidebar.active_page = ActivePage::History,
            AppEvent::OpenFavorites => self.sidebar.active_page = ActivePage::Favorites,
            AppEvent::OpenSettings => self.sidebar.active_page = ActivePage::Settings,
            AppEvent::OpenChat => self.sidebar.active_page = ActivePage::Chat,
            AppEvent::OpenModelPicker => self.sidebar.active_page = ActivePage::ModelPicker,
            AppEvent::OpenProviderPicker => self.sidebar.active_page = ActivePage::ProviderPicker,
            AppEvent::OpenAbout => self.sidebar.active_page = ActivePage::About,
            AppEvent::RuntimeActionResolve {
                action_id,
                decision,
            } => {
                if let Some(action) = self
                    .runtime_actions
                    .actions
                    .iter_mut()
                    .find(|action| action.id == *action_id)
                {
                    let result = match decision.as_str() {
                        "accept" => {
                            use sory_app_server_protocol::CommandExecutionApprovalDecision;
                            serde_json::to_value(CommandExecutionApprovalDecision::Accept)
                                .unwrap_or_else(|_| serde_json::json!({}))
                        }
                        _ => serde_json::json!({}),
                    };
                    let _cmd = action.resolve(result);
                    self.status = format!("Réponse envoyée : {decision}");
                }
            }
            AppEvent::RuntimeActionReject { action_id } => {
                if let Some(action) = self
                    .runtime_actions
                    .actions
                    .iter_mut()
                    .find(|action| action.id == *action_id)
                {
                    let error = sory_app_server_protocol::JSONRPCErrorError {
                        code: -32803,
                        message: "User declined".into(),
                        data: None,
                    };
                    let _cmd = action.reject(error);
                    self.status = "Demande refusée".into();
                }
            }
            AppEvent::ToggleCollapsible(id) => {
                if self.collapsible_expanded.contains(id) {
                    self.collapsible_expanded.remove(id);
                } else {
                    self.collapsible_expanded.insert(id.clone());
                }
            }
            AppEvent::ToggleSidebar => {
                self.settings.settings.sidebar_collapsed =
                    !self.settings.settings.sidebar_collapsed;
            }
            AppEvent::ToggleWorkspaceSidebar => {
                self.settings.settings.workspace_collapsed =
                    !self.settings.settings.workspace_collapsed;
            }
            AppEvent::LayoutAnimationTick => {
                Self::tick_layout_animation(
                    &mut self.sidebar.expand_progress,
                    self.settings.settings.sidebar_collapsed,
                );
                Self::tick_layout_animation(
                    &mut self.workspace_sidebar.expand_progress,
                    self.settings.settings.workspace_collapsed,
                );
            }
            AppEvent::CopyMessage(_) => {
                self.status = "Copi\u{00e9} dans le presse-papiers".into();
            }
            AppEvent::RegenerateMessage(_) => {
                self.status = "R\u{00e9}g\u{00e9}n\u{00e9}ration\u{2026}".into();
            }
            _ => {}
        }
    }

    fn tick_layout_animation(progress: &mut f32, collapsed: bool) {
        let target = if collapsed { 0.0 } else { 1.0 };
        if (*progress - target).abs() < 0.01 {
            *progress = target;
            return;
        }
        let speed = crate::theme::tokens::LAYOUT_ANIMATION_SPEED;
        if *progress < target {
            *progress = (*progress + speed).min(target);
        } else {
            *progress = (*progress - speed).max(target);
        }
    }

    pub fn layout_animating(&self) -> bool {
        let sidebar_target = if self.settings.settings.sidebar_collapsed {
            0.0
        } else {
            1.0
        };
        let workspace_target = if self.settings.settings.workspace_collapsed {
            0.0
        } else {
            1.0
        };
        (self.sidebar.expand_progress - sidebar_target).abs() > 0.01
            || (self.workspace_sidebar.expand_progress - workspace_target).abs() > 0.01
    }

    pub fn apply_loaded_settings(&mut self) {
        let resolved = crate::platform::runtime_paths::resolve_runtime_binary_or_default(
            &self.settings.settings.runtime_command,
        );
        self.settings.settings.runtime_command =
            resolved.path.to_string_lossy().into_owned();

        self.sidebar.expand_progress = if self.settings.settings.sidebar_collapsed {
            0.0
        } else {
            1.0
        };
        self.workspace_sidebar.expand_progress = if self.settings.settings.workspace_collapsed {
            0.0
        } else {
            1.0
        };
    }

    pub fn is_collapsible_expanded(&self, id: &str) -> bool {
        self.collapsible_expanded.contains(id)
    }

    fn ensure_provider_config(&mut self, provider_id: &str) {
        if self
            .settings
            .settings
            .provider_configs
            .contains_key(provider_id)
        {
            return;
        }

        if let Some(def) = crate::models::known_providers()
            .into_iter()
            .find(|p| p.id == provider_id)
        {
            self.settings.settings.provider_configs.insert(
                provider_id.to_string(),
                crate::models::ProviderConfig {
                    endpoint: def.endpoint,
                    api_key: String::new(),
                    model: def.default_model,
                },
            );
        }
    }
}
