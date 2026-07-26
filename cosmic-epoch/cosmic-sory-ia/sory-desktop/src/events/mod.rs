// SPDX-License-Identifier: GPL-3.0-only

use uuid::Uuid;

use crate::{backend::BackendEvent, models::Settings};

#[derive(Debug, Clone)]
pub enum AppEvent {
    NewConversation,
    SendMessage(String),
    StopGeneration,
    ReceiveToken {
        conversation_id: Uuid,
        token: String,
    },
    ToolStarted {
        conversation_id: Uuid,
        name: String,
    },
    ToolFinished {
        conversation_id: Uuid,
        name: String,
    },
    PermissionRequested {
        conversation_id: Uuid,
        title: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    QuestionAsked {
        conversation_id: Uuid,
        prompt: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    ToolApprovalRequested {
        conversation_id: Uuid,
        tool: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    AgentStep {
        conversation_id: Uuid,
        label: String,
    },
    AgentFinished {
        conversation_id: Uuid,
    },
    WorkspaceOpened(Option<String>),
    OpenWorkspace(Option<String>),
    ConversationCreated(Uuid),
    BackendConnected,
    BackendDisconnected,
    BackendReconnecting,
    BackendError(String),
    BackendWarning(String),
    Runtime(BackendEvent),
    BackendTick,
    InputChanged(String),
    OpenHistory,
    OpenFavorites,
    OpenSettings,
    /// Retour à l'interface principale (chat).
    OpenChat,
    /// Ouvrir le sélecteur de modèle.
    OpenModelPicker,
    /// Ouvrir le sélecteur de provider.
    OpenProviderPicker,
    /// Ouvrir la page À propos.
    OpenAbout,
    SettingsChanged(Settings),
    /// Changement de fournisseur IA actif.
    ProviderChanged(String),
    /// Changement d'endpoint pour un provider (provider_id, nouvelle_valeur).
    ProviderEndpointChanged(String, String),
    /// Changement de clé API pour un provider (provider_id, nouvelle_valeur).
    ProviderApiKeyChanged(String, String),
    /// Changement de modèle pour un provider (provider_id, nouvelle_valeur).
    ProviderModelChanged(String, String),
    /// Changement de température.
    TemperatureChanged(f32),
    RuntimeActionResolve {
        action_id: uuid::Uuid,
        decision: String,
    },
    RuntimeActionReject {
        action_id: uuid::Uuid,
    },
    /// Toggle a collapsible section by its unique identifier.
    ToggleCollapsible(String),
    /// Réduire / développer la sidebar gauche.
    ToggleSidebar,
    /// Réduire / développer le panneau workspace droit.
    ToggleWorkspaceSidebar,
    /// Tick d'animation de layout (interpolation fluide des sidebars).
    LayoutAnimationTick,
    /// Copier le contenu d'un message.
    CopyMessage(uuid::Uuid),
    /// Régénérer la dernière réponse.
    RegenerateMessage(uuid::Uuid),
    /// Enregistrer explicitement les paramètres (clé API, endpoint, etc.).
    SaveSettings,
    /// Copier la clé API du provider actif dans le presse-papiers.
    CopyApiKey(String),
    /// Coller depuis le presse-papiers dans la clé API du provider.
    PasteApiKey(String),
    /// Résultat interne de la lecture du presse-papiers.
    ClipboardPasted {
        provider_id: String,
        text: Option<String>,
    },
    /// Sélectionner un provider puis revenir au chat (page picker).
    SelectProviderAndReturn(String),
    /// Sélectionner un modèle puis revenir au chat (page picker).
    SelectModelAndReturn(String, String),
    None,
}
