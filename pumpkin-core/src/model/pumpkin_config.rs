use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PumpkinConfig {
    pub server_directory: PathBuf,
    pub executable: PathBuf,
    #[serde(default)]
    pub arguments: Vec<String>,
}

impl Default for PumpkinConfig {
    fn default() -> Self {
        Self {
            server_directory: PathBuf::from("."),
            executable: default_executable_path(),
            arguments: Vec::new(),
        }
    }
}

fn default_executable_path() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        PathBuf::from("pumpkin-X64-Windows.exe")
    }

    #[cfg(target_os = "macos")]
    {
        PathBuf::from("pumpkin-X64-macOS")
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from("pumpkin-X64-Linux")
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        PathBuf::from("pumpkin-X64-Windows.exe")
    }
}
