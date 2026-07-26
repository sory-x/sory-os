// SPDX-License-Identifier: GPL-3.0-only

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sory_app_server_protocol::ConfigEdit;

/// Configuration runtime transmise avec chaque envoi de message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeMessageConfig {
    /// Identifiant provider sory (`model_provider`).
    pub provider_id: String,
    pub model: String,
    /// Éditions `config/batchWrite` (modèle actif + registre `model_providers`).
    pub config_edits: Vec<ConfigEdit>,
}

/// Commandes applicatives envoyées au runtime Sory IA.
///
/// Elles restent indépendantes du protocole app-server afin que l'UI ne dépende
/// pas des types internes du moteur.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackendCommand {
    Connect,
    Disconnect,
    SendMessage {
        conversation_id: Uuid,
        content: String,
        runtime_config: RuntimeMessageConfig,
    },
    /// Synchronise modèle actif + registre `model_providers` vers sory.
    SyncRuntimeConfig {
        runtime_config: RuntimeMessageConfig,
    },
    /// Redémarre le runtime pour recharger les variables d'environnement (ex: clé API).
    RestartRuntime,
    OpenWorkspace {
        path: Option<String>,
    },
    StopGeneration {
        conversation_id: Uuid,
    },
    ResolveServerRequest {
        request_id: sory_app_server_protocol::RequestId,
        result: sory_app_server_protocol::Result,
    },
    RejectServerRequest {
        request_id: sory_app_server_protocol::RequestId,
        error: sory_app_server_protocol::JSONRPCErrorError,
    },
    Shutdown,
}

/// Événements normalisés reçus du runtime et consommés par l'état centralisé.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackendEvent {
    Connected,
    Disconnected,
    Reconnecting,
    /// Le daemon runtime est lancé mais le health check a échoué.
    /// Le backend tentera une reconnexion (équivalent OpenCode `/global/health`).
    HealthCheckFailed {
        message: String,
    },
    /// Le health check a réussi (émit après Connected pour confirmer).
    HealthCheckPassed,
    Progress {
        message: String,
    },
    ConversationLinked {
        conversation_id: Uuid,
        thread_id: String,
    },
    Token {
        conversation_id: Option<Uuid>,
        token: String,
    },
    ToolStarted {
        conversation_id: Option<Uuid>,
        name: String,
    },
    ToolFinished {
        conversation_id: Option<Uuid>,
        name: String,
    },
    PermissionRequested {
        conversation_id: Option<Uuid>,
        title: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    QuestionAsked {
        conversation_id: Option<Uuid>,
        prompt: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    ToolApprovalRequested {
        conversation_id: Option<Uuid>,
        tool: String,
        details: String,
        request_id: sory_app_server_protocol::RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    },
    AgentStep {
        conversation_id: Option<Uuid>,
        label: String,
    },
    AgentFinished {
        conversation_id: Option<Uuid>,
        turn_id: Option<String>,
    },
    /// Avertissement non bloquant du runtime (ne doit pas interrompre le chat).
    Warning {
        message: String,
    },
    Error {
        message: String,
    },
}
