// SPDX-License-Identifier: GPL-3.0-only

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("failed to start Sory IA runtime process: {0}")]
    Start(std::io::Error),
    #[error("failed to start Sory IA runtime: {0}")]
    RuntimeStartFailed(String),
    #[error("failed to stop Sory IA runtime: {0}")]
    RuntimeStopFailed(String),
    #[error("Sory IA runtime connection error: {0}")]
    Connection(String),
    #[error("Sory IA runtime transport error: {0}")]
    Transport(String),
    #[error("Sory IA runtime protocol error: {0}")]
    Protocol(#[from] serde_json::Error),
    #[error("Sory IA runtime health check failed: {0}")]
    HealthCheck(String),
}

pub type BackendResult<T> = Result<T, BackendError>;
