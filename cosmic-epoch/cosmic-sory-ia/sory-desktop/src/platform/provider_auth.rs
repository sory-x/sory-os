// SPDX-License-Identifier: GPL-3.0-only

//! Synchronisation des clés API vers l'environnement du runtime.

use std::{fs, path::PathBuf};

/// Variable d'environnement associée à un provider.
pub fn env_var_for_provider(provider_id: &str) -> Option<&'static str> {
    match provider_id {
        "openai" => Some("OPENAI_API_KEY"),
        "mistral" => Some("MISTRAL_API_KEY"),
        "anthropic" => Some("ANTHROPIC_API_KEY"),
        "google" => Some("GOOGLE_API_KEY"),
        "groq" => Some("GROQ_API_KEY"),
        "deepseek" => Some("DEEPSEEK_API_KEY"),
        "xai" => Some("XAI_API_KEY"),
        "cohere" => Some("COHERE_API_KEY"),
        "perplexity" => Some("PERPLEXITY_API_KEY"),
        "openrouter" => Some("OPENROUTER_API_KEY"),
        "togetherai" => Some("TOGETHER_API_KEY"),
        "fireworks" => Some("FIREWORKS_API_KEY"),
        "deepinfra" => Some("DEEPINFRA_API_KEY"),
        "nvidia" => Some("NVIDIA_API_KEY"),
        "cerebras" => Some("CEREBRAS_API_KEY"),
        "github-copilot" => Some("GITHUB_TOKEN"),
        "huggingface" => Some("HUGGINGFACE_API_KEY"),
        "replicate" => Some("REPLICATE_API_TOKEN"),
        "venice" => Some("VENICE_API_KEY"),
        "alibaba" => Some("DASHSCOPE_API_KEY"),
        "cloudflare" => Some("CLOUDFLARE_API_TOKEN"),
        "azure" => Some("AZURE_OPENAI_API_KEY"),
        "soryos-zen" => Some("OPENCODE_API_KEY"),
        "opencode-go" => Some("OPENCODE_API_KEY"),
        _ => None,
    }
}

pub fn env_file_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config/sory-ia/provider.env")
}

/// Écrit la clé API dans un fichier env et dans les variables d'environnement du processus.
pub fn sync_api_key(provider_id: &str, api_key: &str) -> Result<(), String> {
    if api_key.trim().is_empty() {
        return Ok(());
    }

    if let Some(var) = env_var_for_provider(provider_id) {
        // Disponible immédiatement pour les sous-processus lancés ensuite.
        unsafe {
            std::env::set_var(var, api_key);
        }
    }

    let path = env_file_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Impossible de créer {} : {e}", parent.display()))?;
    }

    let mut lines = Vec::new();
    if path.exists() {
        if let Ok(content) = fs::read_to_string(&path) {
            for line in content.lines() {
                let key = line.split('=').next().unwrap_or("").trim();
                if let Some(var) = env_var_for_provider(provider_id) {
                    if key == var {
                        continue;
                    }
                }
                if !line.trim().is_empty() {
                    lines.push(line.to_string());
                }
            }
        }
    }

    if let Some(var) = env_var_for_provider(provider_id) {
        lines.push(format!("{var}={api_key}"));
    } else {
        let upper = provider_id.to_uppercase().replace('-', "_");
        lines.push(format!("{upper}_API_KEY={api_key}"));
    }

    fs::write(&path, lines.join("\n") + "\n")
        .map_err(|e| format!("Impossible d'écrire {} : {e}", path.display()))?;

    log::info!("Clé API synchronisée pour le provider {provider_id}");
    Ok(())
}

/// Synchronise toutes les clés API des providers configurés.
pub fn sync_all_api_keys(settings: &crate::models::Settings) -> Result<(), String> {
    for (provider_id, cfg) in &settings.provider_configs {
        sync_api_key(provider_id, &cfg.api_key)?;
    }
    Ok(())
}

/// Indique si une clé API non vide est configurée pour ce provider.
pub fn has_api_key(settings: &crate::models::Settings, provider_id: &str) -> bool {
    settings
        .provider_configs
        .get(provider_id)
        .is_some_and(|cfg| !cfg.api_key.trim().is_empty())
}

/// Providers locaux qui ne nécessitent pas de clé API.
pub fn is_local_provider(provider_id: &str) -> bool {
    matches!(provider_id, "ollama" | "lmstudio")
}

/// Providers avec une clé publique intégrée (pas besoin de clé utilisateur).
pub fn has_public_key(provider_id: &str) -> bool {
    matches!(provider_id, "soryos-zen")
}

/// Vérifie que le provider actif peut être utilisé ; retourne un message d'erreur sinon.
pub fn validate_active_provider(settings: &crate::models::Settings) -> Result<(), String> {
    let provider_id = &settings.provider_id;
    if is_local_provider(provider_id) || has_public_key(provider_id) {
        return Ok(());
    }

    if has_api_key(settings, provider_id) {
        return Ok(());
    }

    let name = crate::models::known_providers()
        .into_iter()
        .find(|p| p.id == *provider_id)
        .map(|p| p.name)
        .unwrap_or_else(|| provider_id.clone());

    Err(format!(
        "Clé API absente pour {name}. Ajoutez votre clé dans Paramètres → Providers."
    ))
}

/// Formate une erreur transport/runtime en message utilisateur.
pub fn humanize_runtime_error(raw: &str) -> String {
    let lower = raw.to_lowercase();
    if lower.contains("401") || lower.contains("unauthorized") || lower.contains("invalid api key") {
        return "Clé API invalide ou refusée par le provider.".into();
    }
    if lower.contains("403") || lower.contains("forbidden") {
        return "Accès refusé par le provider (permissions ou quota).".into();
    }
    if lower.contains("429") || lower.contains("rate limit") {
        return "Limite de requêtes atteinte (rate limit). Réessayez dans quelques instants.".into();
    }
    if lower.contains("timeout") || lower.contains("timed out") {
        return "Délai d'attente dépassé — le provider ou le runtime ne répond pas.".into();
    }
    if lower.contains("connection refused") || lower.contains("dns") {
        return "Erreur réseau — impossible de joindre le provider.".into();
    }
    if lower.contains("model") && lower.contains("not found") {
        return format!("Modèle introuvable côté provider : {raw}");
    }
    raw.to_string()
}

/// Charge les paires clé/valeur du fichier provider.env pour les sous-processus.
pub fn load_env_pairs() -> Vec<(String, String)> {
    let path = env_file_path();
    let Ok(content) = fs::read_to_string(path) else {
        return Vec::new();
    };

    content
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (key, value) = line.split_once('=')?;
            Some((key.trim().to_string(), value.trim().to_string()))
        })
        .collect()
}
