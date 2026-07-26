// SPDX-License-Identifier: GPL-3.0-only

use std::{path::PathBuf, process::Stdio, time::Duration};

use sory_app_server_client::app_server_control_socket_path;
use sory_utils_home_dir::find_sory_home;
use serde::Deserialize;
use tokio::{
    net::UnixStream,
    process::Command,
    time::{sleep, timeout},
};

use super::{BackendError, BackendResult};
use crate::platform::{provider_auth, runtime_paths::ResolvedRuntimeBinary};

const STARTUP_POLL_INTERVAL: Duration = Duration::from_millis(100);
const STARTUP_ATTEMPTS: usize = 100;
const PROBE_TIMEOUT: Duration = Duration::from_secs(2);

/// Gère le cycle de vie du runtime Sory IA.
///
/// Utilise le binaire compilé depuis `sory-ia/sory-rs` (même cœur que le CLI Sory IA).
/// Le mode `daemon` n'est utilisé que pour l'installation standalone `~/.sory-ia/…`.
#[derive(Debug, Clone)]
pub struct RuntimeManager {
    binary: ResolvedRuntimeBinary,
}

impl RuntimeManager {
    pub fn new(binary: ResolvedRuntimeBinary) -> Self {
        Self { binary }
    }

    pub fn binary(&self) -> &ResolvedRuntimeBinary {
        &self.binary
    }

    pub async fn ensure_running(&self) -> BackendResult<RuntimeEndpoint> {
        // Toujours réutiliser un runtime existant pour que desktop et CLI partagent
        // le même app-server et donc les mêmes sessions.
        if let Some(socket_path) = self.default_socket_path() {
            if probe_socket(&socket_path).await {
                log::info!("Runtime déjà actif ({}) — réutilisation", socket_path.display());
                return Ok(RuntimeEndpoint { socket_path });
            }
        }

        if self.binary.uses_managed_daemon() {
            match self.try_daemon_start().await {
                Ok(endpoint) => return Ok(endpoint),
                Err(error) => {
                    log::warn!(
                        "daemon standalone indisponible ({error}), bascule vers démarrage direct…"
                    );
                }
            }
        }

        self.try_spawn_direct().await?;
        self.wait_for_socket().await
    }

    pub async fn restart_with_provider_env(&self) -> BackendResult<RuntimeEndpoint> {
        log::info!(
            "Reconnexion au runtime ({}) après mise à jour des clés API",
            self.binary.path.display()
        );

        // Ne pas tuer le runtime existant — les clés API sont transmises via
        // ConfigBatchWrite avec experimental_bearer_token. On attend juste que
        // le socket soit toujours accessible.
        self.wait_for_socket().await
    }

    pub async fn ensure_running_with_env(&self) -> BackendResult<RuntimeEndpoint> {
        self.ensure_running().await
    }

    async fn stop_local_process(&self) -> BackendResult<()> {
        if let Some(socket_path) = self.default_socket_path() {
            if probe_socket(&socket_path).await {
                let _ = UnixStream::connect(&socket_path).await;
            }
            let _ = std::fs::remove_file(&socket_path);
        }

        let binary_name = self
            .binary
            .path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("sory-app-server");

        let _ = Command::new("pkill")
            .args(["-f", binary_name])
            .output()
            .await;

        sleep(Duration::from_millis(200)).await;
        Ok(())
    }

    async fn try_spawn_direct(&self) -> BackendResult<()> {
        let mut command = Command::new(&self.binary.path);
        command
            .args(self.binary.direct_spawn_args())
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .kill_on_drop(false);
        apply_provider_env(&mut command);

        log::info!(
            "Démarrage direct app-server : {} {:?}",
            self.binary.path.display(),
            self.binary.direct_spawn_args()
        );

        match command.spawn() {
            Ok(_) => Ok(()),
            Err(error) => Err(BackendError::RuntimeStartFailed(format!(
                "impossible de lancer {} : {error}\n\n{}",
                self.binary.path.display(),
                runtime_paths::build_instructions()
            ))),
        }
    }

    async fn try_daemon_start(&self) -> BackendResult<RuntimeEndpoint> {
        let mut command = Command::new(&self.binary.path);
        command.args(["app-server", "daemon", "start"]);
        apply_provider_env(&mut command);

        let output = command
            .output()
            .await
            .map_err(BackendError::Start)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            return Err(BackendError::RuntimeStartFailed(format!(
                "daemon start a échoué : {stderr}"
            )));
        }

        if let Some(socket_path) = parse_socket_path(&output.stdout) {
            if wait_for_socket(&socket_path).await {
                return Ok(RuntimeEndpoint { socket_path });
            }
        }

        self.wait_for_socket().await
    }

    async fn wait_for_socket(&self) -> BackendResult<RuntimeEndpoint> {
        for _ in 0..STARTUP_ATTEMPTS {
            if let Some(socket_path) = self.default_socket_path() {
                if probe_socket(&socket_path).await {
                    log::info!("Socket runtime prêt : {}", socket_path.display());
                    return Ok(RuntimeEndpoint { socket_path });
                }
            }
            sleep(STARTUP_POLL_INTERVAL).await;
        }

        let expected = self
            .default_socket_path()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "(chemin inconnu)".into());

        Err(BackendError::RuntimeStartFailed(format!(
            "le runtime n'a pas exposé de socket app-server à {expected}.\n\n{}",
            runtime_paths::build_instructions()
        )))
    }

    pub async fn stop(&self) -> BackendResult<()> {
        if !self.binary.uses_managed_daemon() {
            return self.stop_local_process().await;
        }

        let output = Command::new(&self.binary.path)
            .args(["app-server", "daemon", "stop"])
            .output()
            .await
            .map_err(BackendError::Start)?;

        if output.status.success() {
            Ok(())
        } else {
            Err(BackendError::RuntimeStopFailed(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ))
        }
    }

    fn default_socket_path(&self) -> Option<PathBuf> {
        resolve_sory_ia_home()
            .ok()
            .and_then(|home| app_server_control_socket_path(home.as_path()).ok())
            .map(|path| path.into_path_buf())
    }
}

use crate::platform::runtime_paths;

fn apply_provider_env(command: &mut tokio::process::Command) {
    for (key, value) in provider_auth::load_env_pairs() {
        command.env(key, value);
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeEndpoint {
    pub socket_path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LifecycleOutput {
    socket_path: PathBuf,
}

fn resolve_sory_ia_home() -> std::io::Result<sory_utils_absolute_path::AbsolutePathBuf> {
    if let Ok(home) = std::env::var("SORY_IA_HOME") {
        return sory_utils_absolute_path::AbsolutePathBuf::from_absolute_path(PathBuf::from(home));
    }
    find_sory_home()
}

fn parse_socket_path(stdout: &[u8]) -> Option<PathBuf> {
    let text = String::from_utf8_lossy(stdout);
    serde_json::from_str::<LifecycleOutput>(&text)
        .ok()
        .map(|output| output.socket_path)
}

async fn wait_for_socket(socket_path: &PathBuf) -> bool {
    for _ in 0..STARTUP_ATTEMPTS {
        if probe_socket(socket_path).await {
            return true;
        }
        sleep(STARTUP_POLL_INTERVAL).await;
    }
    false
}

async fn probe_socket(socket_path: &PathBuf) -> bool {
    if !socket_path.exists() {
        return false;
    }

    matches!(
        timeout(PROBE_TIMEOUT, UnixStream::connect(socket_path)).await,
        Ok(Ok(_stream))
    )
}

#[cfg(test)]
mod tests {
    use super::{parse_socket_path, resolve_sory_ia_home};
    use sory_app_server_client::app_server_control_socket_path;

    #[test]
    fn parses_lifecycle_socket_path() {
        let path = parse_socket_path(br#"{"status":"started","socketPath":"/tmp/sory.sock"}"#);
        assert_eq!(path.unwrap().to_string_lossy(), "/tmp/sory.sock");
    }

    #[test]
    fn uses_official_control_socket_path() {
        let home = resolve_sory_ia_home().expect("sory-ia home");
        let socket = app_server_control_socket_path(home.as_path()).expect("socket path");
        assert!(
            socket
                .as_path()
                .ends_with("app-server-control/app-server-control.sock")
        );
    }
}
