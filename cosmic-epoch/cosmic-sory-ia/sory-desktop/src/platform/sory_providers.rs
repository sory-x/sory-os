// SPDX-License-Identifier: GPL-3.0-only

//! Pont entre les providers Sory IA Desktop et la config `sory_model_providers` de Sory IA.

use sory_app_server_protocol::{ConfigEdit, MergeStrategy};
use serde_json::json;

use crate::models::{ProviderDefinition, Settings, known_providers};
use crate::platform::provider_auth;

/// Providers intégrés Sory IA — ne pas redéfinir dans `sory_model_providers`.
pub const SORY_BUILTIN_PROVIDERS: &[&str] = &[
    "openai", "amazon-bedrock", "ollama", "lmstudio", "soryos-zen", "opencode-go",
];

/// Provider Sory IA dédié aux clés API OpenAI (le provider `openai` intégré exige l'auth Sory IA).
pub const SORY_OPENAI_PROVIDER_ID: &str = "sory-openai";

/// Convertit l'identifiant UI vers l'identifiant `sory_model_provider` Sory IA.
pub fn desktop_to_sory_provider_id(desktop_id: &str, settings: &Settings) -> String {
    match desktop_id {
        "bedrock" => "amazon-bedrock".into(),
        "togetherai" => "together".into(),
        // Le provider OpenAI intégré utilise l'auth Sory IA, pas OPENAI_API_KEY.
        "openai" if provider_auth::has_api_key(settings, "openai") => SORY_OPENAI_PROVIDER_ID.into(),
        other => other.to_string(),
    }
}

pub fn is_sory_builtin(sory_id: &str) -> bool {
    SORY_BUILTIN_PROVIDERS.contains(&sory_id)
}

fn resolved_endpoint(def: &ProviderDefinition, settings: &Settings) -> String {
    settings
        .provider_configs
        .get(&def.id)
        .map(|cfg| {
            if cfg.endpoint.is_empty() {
                def.endpoint.clone()
            } else {
                cfg.endpoint.clone()
            }
        })
        .unwrap_or_else(|| def.endpoint.clone())
}

/// Construit l'entrée `sory_model_providers.<id>` au format Sory IA.
/// Pour éviter un redémarrage du runtime, la clé API est passée directement
/// via `experimental_bearer_token` plutôt que via une variable d'environnement.
pub fn provider_config_value(def: &ProviderDefinition, endpoint: &str, api_key: &str) -> serde_json::Value {
    let mut provider = serde_json::Map::new();
    provider.insert("name".into(), json!(def.name));
    if !endpoint.is_empty() {
        provider.insert("base_url".into(), json!(endpoint));
    }
    let wire_api = match def.id.as_str() {
        "openai" => "responses",
        _ => "chat_completions",
    };
    provider.insert("wire_api".into(), json!(wire_api));
    provider.insert("requires_openai_auth".into(), json!(false));
    if let Some(env_key) = provider_auth::env_var_for_provider(&def.id) {
        provider.insert("env_key".into(), json!(env_key));
    }
    if !api_key.is_empty() {
        provider.insert("experimental_bearer_token".into(), json!(api_key));
    }
    serde_json::Value::Object(provider)
}

fn openai_api_provider_config(settings: &Settings) -> serde_json::Value {
    let endpoint = settings
        .provider_configs
        .get("openai")
        .map(|c| {
            if c.endpoint.is_empty() {
                "https://api.openai.com/v1".to_string()
            } else {
                c.endpoint.clone()
            }
        })
        .unwrap_or_else(|| "https://api.openai.com/v1".into());

    let api_key = settings
        .provider_configs
        .get("openai")
        .map(|c| c.api_key.as_str())
        .unwrap_or("");

    let mut provider = serde_json::Map::new();
    provider.insert("name".into(), json!("OpenAI (clé API)"));
    provider.insert("base_url".into(), json!(endpoint));
    provider.insert("wire_api".into(), json!("responses"));
    provider.insert("requires_openai_auth".into(), json!(false));
    provider.insert("env_key".into(), json!("OPENAI_API_KEY"));
    if !api_key.is_empty() {
        provider.insert("experimental_bearer_token".into(), json!(api_key));
    }
    serde_json::Value::Object(provider)
}

/// Synchronise tous les providers connus vers la config utilisateur Sory IA.
pub fn provider_registry_edits(settings: &Settings) -> Vec<ConfigEdit> {
    let mut edits = Vec::new();

    if provider_auth::has_api_key(settings, "openai") {
        edits.push(ConfigEdit {
            key_path: format!("sory_model_providers.{SORY_OPENAI_PROVIDER_ID}"),
            value: openai_api_provider_config(settings),
            merge_strategy: MergeStrategy::Upsert,
        });
    }

    for def in known_providers() {
        let sory_id = desktop_to_sory_provider_id(&def.id, settings);
        if is_sory_builtin(&sory_id) || sory_id == SORY_OPENAI_PROVIDER_ID {
            continue;
        }

        let endpoint = resolved_endpoint(&def, settings);
        if endpoint.is_empty() {
            continue;
        }

        edits.push(ConfigEdit {
            key_path: format!("sory_model_providers.{sory_id}"),
            value: provider_config_value(
                &def,
                &endpoint,
                settings.provider_configs.get(&def.id).map(|c| c.api_key.as_str()).unwrap_or(""),
            ),
            merge_strategy: MergeStrategy::Upsert,
        });
    }

    edits
}

/// Paramètres de session chat (modèle, provider, politiques).
pub fn active_session_edits(settings: &Settings) -> Vec<ConfigEdit> {
    let sory_provider = desktop_to_sory_provider_id(&settings.provider_id, settings);
    vec![
        ConfigEdit {
            key_path: "model".into(),
            value: json!(settings.resolved_model()),
            merge_strategy: MergeStrategy::Replace,
        },
        ConfigEdit {
            key_path: "sory_model_provider".into(),
            value: json!(sory_provider),
            merge_strategy: MergeStrategy::Replace,
        },
        ConfigEdit {
            key_path: "model_context_window".into(),
            value: json!(settings.context_limit),
            merge_strategy: MergeStrategy::Replace,
        },
        ConfigEdit {
            key_path: "approval_policy".into(),
            value: json!("never"),
            merge_strategy: MergeStrategy::Replace,
        },
        ConfigEdit {
            key_path: "sandbox_mode".into(),
            value: json!("danger-full-access"),
            merge_strategy: MergeStrategy::Replace,
        },
    ]
}

/// Toutes les éditions de config à appliquer avant un tour de chat.
pub fn runtime_config_edits(settings: &Settings) -> Vec<ConfigEdit> {
    let mut edits = active_session_edits(settings);
    edits.extend(provider_registry_edits(settings));
    edits
}
