use std::path::PathBuf;

use haven::{ButtonState, TextState};
use pumpkin_core::{
    PumpkinConfig, ServerController, ServerSnapshot, ServerStatus,
    config::{config_path, load_or_default},
};

#[derive(Debug, Clone)]
pub struct AppState {
    pub controller: ServerController,
    pub config_path: TextState,
    pub server_directory: TextState,
    pub executable: TextState,
    pub arguments: TextState,
    pub logs: Vec<String>,
    pub status_message: String,
    pub error_message: Option<String>,
    pub start_button: ButtonState,
    pub stop_button: ButtonState,
    pub save_button: ButtonState,
    pub refresh_button: ButtonState,
}

impl AppState {
    pub fn new() -> Self {
        let base_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let config_file = config_path(&base_directory);
        let (config, error_message) = match load_or_default(&config_file) {
            Ok(config) => (config, None),
            Err(error) => (PumpkinConfig::default(), Some(error.to_string())),
        };
        let controller = ServerController::new(config_file.clone(), config.clone());

        Self {
            controller,
            config_path: TextState::new(config_file.to_string_lossy().to_string()),
            server_directory: TextState::new(config.server_directory.to_string_lossy().to_string()),
            executable: TextState::new(config.executable.to_string_lossy().to_string()),
            arguments: TextState::new(config.arguments.join(" ")),
            logs: Vec::new(),
            status_message: error_message
                .as_ref()
                .map(|_| "Config load failed".to_string())
                .unwrap_or_else(|| ServerStatus::Stopped.label().to_string()),
            error_message,
            start_button: ButtonState::default(),
            stop_button: ButtonState::default(),
            save_button: ButtonState::default(),
            refresh_button: ButtonState::default(),
        }
    }

    pub fn refresh_runtime(&mut self) {
        self.controller.refresh();
        let snapshot = self.controller.status();
        self.apply_snapshot(snapshot);
        self.logs = self.controller.drain_logs();
    }

    pub fn load_config_from_fields(&self) -> PumpkinConfig {
        PumpkinConfig {
            server_directory: PathBuf::from(self.server_directory.text.trim()),
            executable: PathBuf::from(self.executable.text.trim()),
            arguments: parse_arguments(&self.arguments.text),
        }
    }

    pub fn save_config(&mut self) {
        let config = self.load_config_from_fields();
        self.controller.set_config(config);
        match self.controller.save_config() {
            Ok(()) => {
                self.error_message = None;
                self.status_message = "Config saved".to_string();
            }
            Err(error) => {
                self.error_message = Some(error.to_string());
                self.status_message = "Save failed".to_string();
            }
        }
    }

    pub fn start_server(&mut self) {
        let config = self.load_config_from_fields();
        self.controller.set_config(config);
        match self.controller.start() {
            Ok(()) => {
                self.error_message = None;
                self.status_message = ServerStatus::Starting.label().to_string();
            }
            Err(error) => {
                self.error_message = Some(error.to_string());
                self.status_message = ServerStatus::Failed.label().to_string();
            }
        }

        self.refresh_runtime();
    }

    pub fn stop_server(&mut self) {
        let mut stopped = false;
        match self.controller.stop() {
            Ok(()) => {
                self.error_message = None;
                self.status_message = ServerStatus::Stopping.label().to_string();
                stopped = true;
            }
            Err(error) => {
                self.error_message = Some(error.to_string());
                self.status_message = ServerStatus::Failed.label().to_string();
            }
        }

        if stopped {
            self.refresh_runtime();
        }
    }

    pub fn reload_logs(&mut self) {
        self.refresh_runtime();
    }

    fn apply_snapshot(&mut self, snapshot: ServerSnapshot) {
        self.status_message = snapshot.status.label().to_string();
        self.error_message = snapshot.last_error;
        if let Some(process_id) = snapshot.process_id {
            self.status_message = format!("{} · pid {}", snapshot.status.label(), process_id);
        }
    }
}

fn parse_arguments(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}
