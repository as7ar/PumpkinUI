use std::io;
use std::path::PathBuf;

use thiserror::Error;

pub type Result<T> = std::result::Result<T, PumpkinError>;

#[derive(Debug, Error)]
pub enum PumpkinError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("toml deserialize error: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
    #[error("toml serialize error: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("configuration file not found: {0}")]
    ConfigNotFound(PathBuf),
    #[error("server is already running")]
    ServerAlreadyRunning,
    #[error("server is not running")]
    ServerNotRunning,
    #[error("server executable is missing: {0}")]
    MissingExecutable(PathBuf),
    #[error("server working directory is missing: {0}")]
    MissingWorkingDirectory(PathBuf),
    #[error("failed to start server: {0}")]
    StartFailed(String),
    #[error("failed to stop server: {0}")]
    StopFailed(String),
    #[error("invalid configuration: {0}")]
    InvalidConfig(String),
}
