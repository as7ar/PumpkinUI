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
            executable: PathBuf::from("java"),
            arguments: vec![
                "-jar".to_string(),
                "server.jar".to_string(),
                "nogui".to_string(),
            ],
        }
    }
}
