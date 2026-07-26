// SPDX-License-Identifier: GPL-3.0-only

//! Résolution du binaire moteur Sory IA depuis les sources du monorepo.

use std::path::{Path, PathBuf};

/// Type de binaire runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeBinaryKind {
    /// `sory-app-server` compilé depuis `sory-ia/sory-rs`.
    AppServer,
    /// CLI `sory` compilé depuis `sory-ia/sory-rs` (sous-commande `app-server`).
    SoryCli,
    /// Installation standalone gérée sous `~/.sory-ia/packages/standalone/`.
    ManagedInstall,
}

#[derive(Debug, Clone)]
pub struct ResolvedRuntimeBinary {
    pub path: PathBuf,
    pub kind: RuntimeBinaryKind,
}

impl ResolvedRuntimeBinary {
    pub fn uses_managed_daemon(&self) -> bool {
        self.kind == RuntimeBinaryKind::ManagedInstall
    }

    /// Arguments pour lancer app-server en mode développement (socket Unix).
    pub fn direct_spawn_args(&self) -> &'static [&'static str] {
        match self.kind {
            RuntimeBinaryKind::AppServer => &["--listen", "unix://"],
            RuntimeBinaryKind::SoryCli | RuntimeBinaryKind::ManagedInstall => {
                &["app-server", "--listen", "unix://"]
            }
        }
    }
}

/// Racine du workspace Rust du moteur (`sory-ia/sory-rs`).
pub fn sory_ia_rs_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../sory-ia/sory-rs")
}

/// Racine du workspace Cargo unifié (cible du target/ partagé).
fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..")
}

fn classify_binary(path: &Path) -> ResolvedRuntimeBinary {
    let path_str = path.to_string_lossy();
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();

    let kind = if file_name == "sory-app-server" {
        RuntimeBinaryKind::AppServer
    } else if path_str.contains("/.sory-ia/packages/standalone/") {
        RuntimeBinaryKind::ManagedInstall
    } else {
        RuntimeBinaryKind::SoryCli
    };

    ResolvedRuntimeBinary {
        path: path.to_path_buf(),
        kind,
    }
}

fn local_project_candidates() -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    let ws_root = workspace_root();
    let old_root = sory_ia_rs_root();
    for profile in ["release", "debug"] {
        for name in ["sory-app-server", "sory"] {
            candidates.push(ws_root.join(format!("target/{profile}/{name}")));
            candidates.push(old_root.join(format!("target/{profile}/{name}")));
        }
    }
    candidates
}

/// Résout le binaire moteur à utiliser (sources locales en priorité).
pub fn resolve_runtime_binary() -> Result<ResolvedRuntimeBinary, String> {
    if let Ok(command) = std::env::var("SORY_IA_RUNTIME_COMMAND") {
        let path = PathBuf::from(&command);
        if path.is_file() {
            return Ok(classify_binary(&path));
        }
    }

    for candidate in local_project_candidates() {
        if candidate.is_file() {
            log::info!(
                "Moteur Sory IA local : {}",
                candidate.display()
            );
            return Ok(classify_binary(&candidate));
        }
    }

    if let Ok(home) = std::env::var("HOME") {
        let managed = PathBuf::from(home).join(".sory-ia/packages/standalone/current/sory");
        if managed.is_file() {
            return Ok(classify_binary(&managed));
        }
    }

    Err(build_instructions())
}

/// Résout à partir d'un chemin persisté ; retombe sur la découverte automatique si invalide.
pub fn resolve_runtime_binary_or_default(stored: &str) -> ResolvedRuntimeBinary {
    let stored_path = PathBuf::from(stored);
    if stored_path.is_file() {
        return classify_binary(&stored_path);
    }

    if stored != "sory" {
        log::warn!(
            "Commande runtime enregistrée introuvable ({stored}), nouvelle résolution…"
        );
    }

    resolve_runtime_binary().unwrap_or_else(|error| {
        log::error!("{error}");
        let fallback = workspace_root().join("target/debug/sory-app-server");
        ResolvedRuntimeBinary {
            path: fallback,
            kind: RuntimeBinaryKind::AppServer,
        }
    })
}

pub fn build_instructions() -> String {
    let ws_root = workspace_root();
    format!(
        "Moteur Sory IA introuvable.\n\
Compilez le cœur depuis les sources du projet (pas besoin d'installer le CLI Sory IA) :\n\
  cd {}\n\
  cargo build -p sory-cli -p sory-app-server",
        ws_root.display()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sory_ia_rs_root_points_into_monorepo() {
        let root = sory_ia_rs_root();
        assert!(root.ends_with("sory-ia/sory-rs"));
    }

    #[test]
    fn app_server_spawn_args() {
        let binary = ResolvedRuntimeBinary {
            path: PathBuf::from("/tmp/sory-app-server"),
            kind: RuntimeBinaryKind::AppServer,
        };
        assert_eq!(binary.direct_spawn_args(), &["--listen", "unix://"]);
    }
}
