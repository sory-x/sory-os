// SPDX-License-Identifier: GPL-3.0-only

//! Persistance des paramètres de Sory IA.
//!
//! Les settings sont sauvegardés dans `~/.config/sory-ia/settings.json`
//! et chargés automatiquement au démarrage de l'application.
//! Le format JSON est choisi pour sa lisibilité et sa maintenabilité.

use std::path::PathBuf;

use crate::models::Settings;

/// Emplacement du fichier de configuration.
fn config_path() -> PathBuf {
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".into());
    PathBuf::from(home).join(".config/sory-ia/settings.json")
}

/// Charge les settings depuis le fichier de configuration.
///
/// Retourne `None` si le fichier n'existe pas ou est invalide
/// (dans ce cas on utilise les defaults).
pub fn load_settings() -> Option<Settings> {
    let path = config_path();
    if !path.exists() {
        log::info!("Aucun fichier de config trouvé à {}", path.display());
        return None;
    }
    let content = std::fs::read_to_string(&path).ok()?;
    match serde_json::from_str::<Settings>(&content) {
        Ok(settings) => {
            log::info!("Settings chargés depuis {}", path.display());
            Some(settings)
        }
        Err(e) => {
            log::warn!("Fichier de config invalide ({}), utilisation des defaults : {e}", path.display());
            None
        }
    }
}

/// Sauvegarde les settings dans le fichier de configuration.
pub fn save_settings(settings: &Settings) -> Result<(), String> {
    let path = config_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            format!("Impossible de créer le dossier de config : {e}")
        })?;
    }
    let content = serde_json::to_string_pretty(settings).map_err(|e| {
        format!("Erreur de sérialisation des settings : {e}")
    })?;
    std::fs::write(&path, content).map_err(|e| {
        format!("Impossible d'écrire les settings : {e}")
    })?;
    log::info!("Settings sauvegardés dans {}", path.display());
    Ok(())
}
