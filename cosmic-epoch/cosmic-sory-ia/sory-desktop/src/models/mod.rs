// SPDX-License-Identifier: GPL-3.0-only

use crate::backend::BackendCommand;
use sory_app_server_protocol::{JSONRPCErrorError, RequestId, Result as JsonRpcResult};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod catalog;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation {
    pub id: Uuid,
    pub title: String,
    pub messages: Vec<Message>,
    pub workspace_id: Option<Uuid>,
    pub runtime_thread_id: Option<String>,
    pub favorite: bool,
}

impl Conversation {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            messages: Vec::new(),
            workspace_id: None,
            runtime_thread_id: None,
            favorite: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub status: MessageStatus,
    pub runtime_message_id: Option<String>,
    pub tool_calls: Vec<ToolCall>,
    pub attachments: Vec<Attachment>,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::User,
            content: content.into(),
            status: MessageStatus::Complete,
            runtime_message_id: None,
            tool_calls: Vec::new(),
            attachments: Vec::new(),
        }
    }

    pub fn assistant_streaming() -> Self {
        Self {
            id: Uuid::new_v4(),
            role: MessageRole::Assistant,
            content: String::new(),
            status: MessageStatus::Streaming,
            runtime_message_id: None,
            tool_calls: Vec::new(),
            attachments: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageStatus {
    Pending,
    Streaming,
    Complete,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Provider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

/// Définition complète d'un fournisseur IA.
/// Correspond aux providers enregistrés dans sorycode-dev/packages/core/src/plugin/provider/.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDefinition {
    /// Identifiant unique du provider (ex: "openai", "mistral", "google").
    pub id: String,
    /// Nom affiché dans l'interface (ex: "OpenAI", "Mistral", "Google Gemini").
    pub name: String,
    /// URL de base de l'API (ex: "https://api.openai.com/v1").
    pub endpoint: String,
    /// Identifiant de la clé API à utiliser (nom de variable d'env ou champ libre).
    pub api_key: String,
    /// Modèle par défaut suggéré (ex: "gpt-4o", "mistral-large-latest").
    pub default_model: String,
    /// Liste des modèles disponibles pour ce provider.
    pub models: Vec<String>,
    /// URL pour obtenir une clé API.
    pub api_key_url: String,
    /// Texte d'aide pour la clé API.
    pub api_key_hint: String,
    /// Catégorie : "openai-compatible" (API REST compatible OpenAI) ou "native" (protocole propre).
    pub kind: ProviderKind,
    /// Actif ou non.
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProviderKind {
    /// API REST compatible avec le format OpenAI /v1/chat/completions
    OpenAICompatible,
    /// API native (Anthropic, Google, Amazon Bedrock, …)
    Native,
}

impl ProviderDefinition {
    /// Crée un provider OpenAI-compatible
    pub fn openai_compatible(id: &str, name: &str, endpoint: &str, default_model: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            api_key: String::new(),
            default_model: default_model.to_string(),
            models: Vec::new(),
            api_key_url: String::new(),
            api_key_hint: String::new(),
            kind: ProviderKind::OpenAICompatible,
            enabled: true,
        }
    }

    /// Crée un provider natif
    pub fn native(id: &str, name: &str, endpoint: &str, default_model: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            endpoint: endpoint.to_string(),
            api_key: String::new(),
            default_model: default_model.to_string(),
            models: Vec::new(),
            api_key_url: String::new(),
            api_key_hint: String::new(),
            kind: ProviderKind::Native,
            enabled: true,
        }
    }

    /// Ajoute la liste des modèles disponibles (chaînable).
    pub fn with_models(mut self, models: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.models = models.into_iter().map(Into::into).collect();
        self
    }

    /// Ajoute l'URL et le texte d'aide pour obtenir une clé API (chaînable).
    pub fn with_api_key_info(mut self, url: &str, hint: &str) -> Self {
        self.api_key_url = url.to_string();
        self.api_key_hint = hint.to_string();
        self
    }
}

/// Fournisseurs connus.
///
/// Les modèles ne sont PLUS hardcodés ici — ils sont chargés depuis
/// le catalogue `models.dev` via `catalog::models_for(provider_id)`.
/// Voir `catalog.rs` pour le détail.
pub fn known_providers() -> Vec<ProviderDefinition> {
    vec![
        // ── Premium ────────────────────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "openai",
            "OpenAI",
            "https://api.openai.com/v1",
            "gpt-4o",
        )
        .with_api_key_info(
            "https://platform.openai.com/api-keys",
            "Créez une clé API sur platform.openai.com",
        ),
        ProviderDefinition::native(
            "anthropic",
            "Anthropic Claude",
            "https://api.anthropic.com/v1",
            "claude-sonnet-4-20250514",
        )
        .with_api_key_info(
            "https://console.anthropic.com/settings/keys",
            "Créez une clé API sur console.anthropic.com",
        ),
        ProviderDefinition::native(
            "google",
            "Google Gemini",
            "https://generativelanguage.googleapis.com/v1beta",
            "gemini-2.5-flash",
        )
        .with_api_key_info(
            "https://aistudio.google.com/app/apikey",
            "Créez une clé API sur aistudio.google.com",
        ),
        ProviderDefinition::openai_compatible(
            "mistral",
            "Mistral AI",
            "https://api.mistral.ai/v1",
            "mistral-large-latest",
        )
        .with_api_key_info(
            "https://console.mistral.ai/api-keys",
            "Créez une clé API sur console.mistral.ai",
        ),

        // ── Open Source / Inférence rapide ────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "groq",
            "Groq",
            "https://api.groq.com/openai/v1",
            "llama-3.3-70b-versatile",
        )
        .with_api_key_info(
            "https://console.groq.com/keys",
            "Créez une clé API sur console.groq.com",
        ),
        ProviderDefinition::openai_compatible(
            "deepinfra",
            "DeepInfra",
            "https://api.deepinfra.com/v1/openai",
            "meta-llama/Llama-4-Scout-17B-16E-Instruct",
        )
        .with_api_key_info(
            "https://deepinfra.com/dash/api_keys",
            "Créez une clé API sur deepinfra.com",
        ),
        ProviderDefinition::openai_compatible(
            "togetherai",
            "Together AI",
            "https://api.together.xyz/v1",
            "meta-llama/Llama-4-Scout-17B-16E-Instruct",
        )
        .with_api_key_info(
            "https://api.together.ai/settings/api-keys",
            "Créez une clé API sur together.ai",
        ),
        ProviderDefinition::openai_compatible(
            "fireworks",
            "Fireworks AI",
            "https://api.fireworks.ai/inference/v1",
            "accounts/fireworks/models/llama-v4-scout",
        )
        .with_api_key_info(
            "https://fireworks.ai/api-keys",
            "Créez une clé API sur fireworks.ai",
        ),
        ProviderDefinition::openai_compatible(
            "deepseek",
            "DeepSeek",
            "https://api.deepseek.com/v1",
            "deepseek-chat",
        )
        .with_api_key_info(
            "https://platform.deepseek.com/api_keys",
            "Créez une clé API sur platform.deepseek.com",
        ),
        ProviderDefinition::openai_compatible(
            "cerebras",
            "Cerebras",
            "https://api.cerebras.ai/v1",
            "cerebras-4.0",
        )
        .with_api_key_info(
            "https://cloud.cerebras.ai/",
            "Créez une clé API sur cloud.cerebras.ai",
        ),

        // ── Agrégateurs ────────────────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "openrouter",
            "OpenRouter",
            "https://openrouter.ai/api/v1",
            "openai/gpt-4o",
        )
        .with_api_key_info(
            "https://openrouter.ai/keys",
            "Créez une clé API sur openrouter.ai",
        ),

        // ── Cloud ──────────────────────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "alibaba",
            "Alibaba Qwen",
            "https://dashscope.aliyuncs.com/compatible-mode/v1",
            "qwen-max",
        )
        .with_api_key_info(
            "https://bailian.console.aliyun.com/",
            "Créez une clé API sur bailian.console.aliyun.com",
        ),
        ProviderDefinition::openai_compatible(
            "azure",
            "Azure OpenAI",
            "https://{resource}.openai.azure.com/v1",
            "gpt-4o",
        )
        .with_api_key_info(
            "https://portal.azure.com/#view/Microsoft_Azure_Marketplace",
            "Obtenez une clé depuis le portail Azure",
        ),
        ProviderDefinition::native(
            "bedrock",
            "Amazon Bedrock",
            "",
            "anthropic.claude-sonnet-4-v1",
        )
        .with_api_key_info(
            "https://console.aws.amazon.com/bedrock/",
            "Configurez Bedrock depuis la console AWS",
        ),
        ProviderDefinition::openai_compatible(
            "cloudflare",
            "Cloudflare Workers AI",
            "https://api.cloudflare.com/client/v4/accounts/{account_id}/ai/v1",
            "@cf/meta/llama-4-scout-17b-16e-instruct",
        )
        .with_api_key_info(
            "https://dash.cloudflare.com/profile/api-tokens",
            "Créez un token API sur dash.cloudflare.com",
        ),
        ProviderDefinition::openai_compatible(
            "nvidia",
            "NVIDIA NIM",
            "https://api.nvcf.nvidia.com/v1",
            "meta/llama-4-scout-17b-16e-instruct",
        )
        .with_api_key_info(
            "https://build.nvidia.com/",
            "Obtenez une clé API sur build.nvidia.com",
        ),

        // ── Recherche / Spécialisés ────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "xai",
            "xAI Grok",
            "https://api.x.ai/v1",
            "grok-3",
        )
        .with_api_key_info(
            "https://console.x.ai/",
            "Créez une clé API sur console.x.ai",
        ),
        ProviderDefinition::openai_compatible(
            "perplexity",
            "Perplexity AI",
            "https://api.perplexity.ai",
            "sonar-pro",
        )
        .with_api_key_info(
            "https://www.perplexity.ai/settings/api",
            "Créez une clé API sur perplexity.ai",
        ),
        ProviderDefinition::native(
            "cohere",
            "Cohere",
            "https://api.cohere.com/v2",
            "command-r-plus",
        )
        .with_api_key_info(
            "https://dashboard.cohere.com/api-keys",
            "Créez une clé API sur dashboard.cohere.com",
        ),

        // ── Plateformes Dev ────────────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "soryos-zen",
            "SoryOS Zen",
            "https://opencode.ai/zen/v1",
            "gpt-5-nano",
        )
        .with_api_key_info(
            "https://opencode.ai/docs/zen",
            "Provider gratuit - clé publique intégrée. Modèles premium nécessitent OPENCODE_API_KEY.",
        ),
        ProviderDefinition::openai_compatible(
            "opencode-go",
            "OpenCode Go",
            "https://opencode.ai/zen/go/v1",
            "gpt-5.4",
        )
        .with_api_key_info(
            "https://opencode.ai/docs/zen",
            "Nécessite une clé API OPENCODE_API_KEY. Modèles premium.",
        ),
        ProviderDefinition::openai_compatible(
            "github-copilot",
            "GitHub Copilot",
            "https://api.githubcopilot.com/v1",
            "gpt-4o",
        )
        .with_api_key_info(
            "https://github.com/settings/copilot",
            "Configurez GitHub Copilot dans vos settings GitHub",
        ),
        ProviderDefinition::openai_compatible(
            "huggingface",
            "Hugging Face",
            "https://api-inference.huggingface.co/v1",
            "meta-llama/Llama-4-Scout-17B-16E-Instruct",
        )
        .with_api_key_info(
            "https://huggingface.co/settings/tokens",
            "Créez un token sur huggingface.co/settings/tokens",
        ),
        ProviderDefinition::openai_compatible(
            "replicate",
            "Replicate",
            "https://api.replicate.com/v1",
            "meta/meta-llama-4-scout-17b-16e-instruct",
        )
        .with_api_key_info(
            "https://replicate.com/account/api-tokens",
            "Créez une clé API sur replicate.com",
        ),

        // ── Spécialisés / Niche ────────────────────────────────────────────────
        ProviderDefinition::openai_compatible(
            "venice",
            "Venice.ai",
            "https://api.venice.ai/api/v1",
            "llama-4-scout-17b-16e-instruct",
        )
        .with_api_key_info(
            "https://venice.ai/api-keys",
            "Créez une clé API sur venice.ai",
        ),
        ProviderDefinition::openai_compatible("kilo", "Kilo AI", "", "kilo-1")
            .with_api_key_info("", "Contactez Kilo AI pour un accès"),
        ProviderDefinition::native("sap-ai-core", "SAP AI Core", "", "gpt-4o")
            .with_api_key_info("", "Configurez SAP AI Core dans votre tenant SAP"),
        ProviderDefinition::openai_compatible("gateway", "LLM Gateway", "", "gpt-4o")
            .with_api_key_info("", "Utilisez votre propre gateway LLM"),
        ProviderDefinition::native("sorycode", "SoryCode", "", "sorycode-1")
            .with_api_key_info("", "Provider intégré SoryCode — pas de clé requise"),
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: Uuid,
    pub name: String,
    pub path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: Uuid,
    pub name: String,
    pub status: ToolStatus,
    pub summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeAction {
    pub id: Uuid,
    pub conversation_id: Uuid,
    pub kind: RuntimeActionKind,
    pub title: String,
    pub details: String,
    pub status: RuntimeActionStatus,
    pub request_id: RequestId,
    pub thread_id: String,
    pub turn_id: String,
    pub item_id: String,
}

impl RuntimeAction {
    pub fn new(
        conversation_id: Uuid,
        kind: RuntimeActionKind,
        title: impl Into<String>,
        details: impl Into<String>,
        request_id: RequestId,
        thread_id: String,
        turn_id: String,
        item_id: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            conversation_id,
            kind,
            title: title.into(),
            details: details.into(),
            status: RuntimeActionStatus::Pending,
            request_id,
            thread_id,
            turn_id,
            item_id,
        }
    }

    pub fn resolve(&mut self, result: JsonRpcResult) -> BackendCommand {
        self.status = RuntimeActionStatus::Resolved;
        BackendCommand::ResolveServerRequest {
            request_id: self.request_id.clone(),
            result,
        }
    }

    pub fn reject(&mut self, error: JSONRPCErrorError) -> BackendCommand {
        self.status = RuntimeActionStatus::Cancelled;
        BackendCommand::RejectServerRequest {
            request_id: self.request_id.clone(),
            error,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeActionKind {
    Permission,
    Question,
    ToolApproval,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RuntimeActionStatus {
    Pending,
    Resolved,
    Cancelled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolStatus {
    Started,
    Finished,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub name: String,
    pub path: Option<String>,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiffStatus {
    Modified,
    Created,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiffFile {
    pub file_path: String,
    pub status: DiffStatus,
    pub added: usize,
    pub removed: usize,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemePreference {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// URL de l'API pour ce provider (peut être vide pour utiliser l'endpoint par défaut).
    pub endpoint: String,
    /// Clé API pour ce provider.
    pub api_key: String,
    /// Modèle à utiliser (vide = modèle par défaut du provider).
    pub model: String,
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            endpoint: String::new(),
            api_key: String::new(),
            model: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub runtime_command: String,
    pub auto_start_runtime: bool,
    /// Provider actif (ex: "openai", "mistral", "google").
    pub provider_id: String,
    /// Modèle à utiliser (peut être "auto" pour le modèle par défaut du provider).
    pub model: String,
    pub temperature: f32,
    pub context_limit: u32,
    pub theme: ThemePreference,
    pub language: String,
    /// Sidebar gauche réduite (icônes seules).
    #[serde(default)]
    pub sidebar_collapsed: bool,
    /// Panneau workspace droit réduit.
    #[serde(default)]
    pub workspace_collapsed: bool,
    /// Configuration par provider (endpoint, api_key, model).
    pub provider_configs: std::collections::BTreeMap<String, ProviderConfig>,
}

/// Résout la commande du moteur Sory IA compilé depuis `sory-ia/sory-rs`.
pub fn default_runtime_command() -> String {
    crate::platform::runtime_paths::resolve_runtime_binary()
        .map(|binary| binary.path.to_string_lossy().into_owned())
        .unwrap_or_else(|_| {
            // Chemin attendu après compilation locale — évite le fallback `sory` global.
            crate::platform::runtime_paths::sory_ia_rs_root()
                .join("target/debug/sory")
                .to_string_lossy()
                .into_owned()
        })
}

impl Default for Settings {
    fn default() -> Self {
        let mut provider_configs = std::collections::BTreeMap::new();
        for p in known_providers() {
            provider_configs.insert(
                p.id.clone(),
                ProviderConfig {
                    endpoint: p.endpoint,
                    api_key: String::new(),
                    model: p.default_model,
                },
            );
        }
        Self {
            runtime_command: default_runtime_command(),
            auto_start_runtime: true,
            provider_id: "soryos-zen".into(),
            model: "auto".into(),
            temperature: 0.7,
            context_limit: 128_000,
            theme: ThemePreference::System,
            language: "fr".into(),
            sidebar_collapsed: false,
            workspace_collapsed: false,
            provider_configs,
        }
    }
}

impl Settings {
    /// Retourne la configuration active du provider sélectionné.
    pub fn active_provider_config(&self) -> Option<&ProviderConfig> {
        self.provider_configs.get(&self.provider_id)
    }

    /// Retourne l'endpoint à utiliser pour le provider actif.
    pub fn active_endpoint(&self) -> String {
        self.active_provider_config()
            .map(|c| {
                if c.endpoint.is_empty() {
                    // Fallback vers l'endpoint connu
                    known_providers()
                        .iter()
                        .find(|p| p.id == self.provider_id)
                        .map(|p| p.endpoint.clone())
                        .unwrap_or_default()
                } else {
                    c.endpoint.clone()
                }
            })
            .unwrap_or_default()
    }

    /// Résout le modèle actif (sélection UI, config provider, défaut).
    pub fn resolved_model(&self) -> String {
        if !self.model.is_empty() && self.model != "auto" {
            return self.model.clone();
        }

        if let Some(cfg) = self.active_provider_config() {
            if !cfg.model.is_empty() {
                return cfg.model.clone();
            }
        }

        known_providers()
            .into_iter()
            .find(|p| p.id == self.provider_id)
            .map(|p| p.default_model)
            .filter(|m| !m.is_empty())
            .unwrap_or_else(|| "gpt-4o".into())
    }

    /// Configuration runtime pour un envoi de message au moteur.
    pub fn runtime_message_config(&self) -> crate::backend::RuntimeMessageConfig {
        crate::backend::RuntimeMessageConfig {
            provider_id: crate::platform::sory_providers::desktop_to_sory_provider_id(
                &self.provider_id,
                self,
            ),
            model: self.resolved_model(),
            config_edits: crate::platform::sory_providers::runtime_config_edits(self),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NotificationKind {
    Error,
    Success,
    Download,
    GenerationFinished,
    ToolExecuted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppNotification {
    pub id: Uuid,
    pub kind: NotificationKind,
    pub title: String,
    pub body: String,
}

impl AppNotification {
    pub fn new(kind: NotificationKind, title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind,
            title: title.into(),
            body: body.into(),
        }
    }
}
