// SPDX-License-Identifier: GPL-3.0-only

//! Catalogue de modèles IA — snapshot embarqué depuis `models.dev`.
//!
//! Reproduit l'approche de `sorycode-dev/packages/core/src/models.ts` :
//! les modèles sont chargés depuis un catalogue (snapshot JSON embarqué
//! à la compilation, avec refresh optionnel depuis l'API distante).
//!
//! Le snapshot est généré depuis `https://models.dev/api.json` via
//! le script `scripts/gen-catalog-snapshot.py`.

use std::collections::HashMap;

/// Information basique sur un modèle.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelEntry {
    pub name: String,
    #[serde(default)]
    pub family: Option<String>,
    #[serde(default)]
    pub limit: Option<serde_json::Value>,
    #[serde(default)]
    pub cost: Option<serde_json::Value>,
    #[serde(default)]
    pub modalities: Option<serde_json::Value>,
    #[serde(default)]
    pub reasoning: bool,
    #[serde(default)]
    pub tool_call: bool,
}

/// Information sur un provider dans le catalogue.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ProviderEntry {
    pub name: String,
    #[serde(default)]
    pub api: String,
    pub models: HashMap<String, ModelEntry>,
}

/// Catalogue complet des modèles par provider.
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ModelCatalog {
    /// Map: provider_id → ProviderEntry
    #[serde(flatten)]
    pub providers: HashMap<String, ProviderEntry>,
}

impl ModelCatalog {
    /// Charge le catalogue depuis le snapshot JSON embarqué.
    pub fn embedded() -> Self {
        let json = include_str!("catalog_snapshot.json");
        serde_json::from_str(json).expect("catalog_snapshot.json invalide")
    }

    /// Retourne la liste des IDs de modèles pour un provider donné.
    pub fn model_ids(&self, provider_id: &str) -> Vec<String> {
        self.providers
            .get(provider_id)
            .map(|p| {
                let mut ids: Vec<String> = p.models.keys().cloned().collect();
                ids.sort();
                ids
            })
            .unwrap_or_default()
    }

    /// Retourne le nombre de modèles pour un provider.
    pub fn model_count(&self, provider_id: &str) -> usize {
        self.providers
            .get(provider_id)
            .map(|p| p.models.len())
            .unwrap_or(0)
    }
}

/// Catalogue chargé une fois (singleton lazy).
static CATALOG: std::sync::LazyLock<ModelCatalog> =
    std::sync::LazyLock::new(ModelCatalog::embedded);

/// Raccourci : liste des IDs de modèles pour un provider.
pub fn models_for(provider_id: &str) -> Vec<String> {
    CATALOG.model_ids(provider_id)
}

/// Liste complète des modèles affichables pour un provider :
/// catalogue embarqué + modèle par défaut + modèle actuel custom.
pub fn resolved_models_for(provider_id: &str, current_model: &str) -> Vec<String> {
    let mut models = models_for(provider_id);

    if models.is_empty() {
        if let Some(def) = crate::models::known_providers()
            .into_iter()
            .find(|p| p.id == provider_id)
        {
            if !def.default_model.is_empty() {
                models.push(def.default_model);
            }
        }
    }

    if models.is_empty() {
        models.push("auto".into());
    }

    if !models.iter().any(|m| m == "auto") {
        models.insert(0, "auto".into());
    }

    if !current_model.is_empty()
        && current_model != "auto"
        && !models.iter().any(|m| m == current_model)
    {
        models.insert(1, current_model.to_string());
    }

    models
}

/// Raccourci : nombre de modèles pour un provider.
pub fn model_count(provider_id: &str) -> usize {
    CATALOG.model_count(provider_id)
}

/// Raccourci : tous les providers connus dans le catalogue.
pub fn known_providers_in_catalog() -> Vec<String> {
    let mut ids: Vec<String> = CATALOG.providers.keys().cloned().collect();
    ids.sort();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_loads() {
        let catalog = ModelCatalog::embedded();
        assert!(!catalog.providers.is_empty(), "Le catalogue ne doit pas etre vide");
        assert!(catalog.providers.contains_key("openai"), "OpenAI doit etre dans le catalogue");
    }

    #[test]
    fn test_openai_has_models() {
        let models = models_for("openai");
        assert!(models.len() > 10, "OpenAI doit avoir au moins 10 modeles, actuel: {}", models.len());
        assert!(models.contains(&"o3".to_string()), "OpenAI doit contenir o3");
    }

    #[test]
    fn test_unknown_provider() {
        let models = models_for("inexistant");
        assert!(models.is_empty(), "Provider inconnu doit retourner une liste vide");
    }

    #[test]
    fn test_anthropic_has_claude() {
        let models = models_for("anthropic");
        assert!(models.iter().any(|m| m.contains("claude")), "Anthropic doit contenir des modeles claude");
    }
}
