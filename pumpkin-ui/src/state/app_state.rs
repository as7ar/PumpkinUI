use std::path::PathBuf;
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

use haven::PaneState;
use haven::{ButtonState, TextState};
use pumpkin_core::{PumpkinConfig, ServerController, ServerLogLine, ServerSnapshot, ServerStatus};

#[derive(Debug)]
pub struct AppState {
    pub controller: ServerController,
    pub server_directory: TextState,
    pub executable: TextState,
    pub arguments: TextState,
    pub logs: Vec<ServerLogLine>,
    pub status_message: String,
    pub error_message: Option<String>,
    pub start_button: ButtonState,
    pub stop_button: ButtonState,
    pub save_button: ButtonState,
    pub refresh_button: ButtonState,
    config_error_message: Option<String>,
    event_tx: Sender<UiEvent>,
    event_rx: Receiver<UiEvent>,
    refresh_worker_started: bool,
    start_pending: bool,
}

#[derive(Debug)]
enum UiEvent {
    RefreshTick,
    StartFinished(Result<(), String>),
}

impl AppState {
    pub fn new() -> Self {
        let base_directory = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let config_file = base_directory.join("pumpkin-ui.toml");
        let (event_tx, event_rx) = mpsc::channel();
        let (controller, error_message) = match ServerController::from_config_path(&config_file) {
            Ok(controller) => (controller, None),
            Err(error) => (
                ServerController::new(config_file.clone(), PumpkinConfig::default()),
                Some(error.to_string()),
            ),
        };
        let config = controller.config();

        Self {
            controller,
            server_directory: TextState::new(config.server_directory.to_string_lossy().to_string()),
            executable: TextState::new(config.executable.to_string_lossy().to_string()),
            arguments: TextState::new(config.arguments.join(" ")),
            logs: Vec::new(),
            status_message: error_message
                .as_ref()
                .map(|_| "Config load failed".to_string())
                .unwrap_or_else(|| ServerStatus::Stopped.label().to_string()),
            error_message: error_message.clone(),
            start_button: ButtonState::default(),
            stop_button: ButtonState::default(),
            save_button: ButtonState::default(),
            refresh_button: ButtonState::default(),
            config_error_message: error_message.clone(),
            event_tx,
            event_rx,
            refresh_worker_started: false,
            start_pending: false,
        }
    }

    pub fn on_start(&mut self, app: &mut PaneState) {
        self.start_refresh_worker(app);
        app.redraw();
    }

    pub fn on_wake(&mut self, app: &mut PaneState) {
        let mut updated = false;

        loop {
            match self.event_rx.try_recv() {
                Ok(UiEvent::RefreshTick) => {
                    self.refresh_runtime();
                    updated = true;
                }
                Ok(UiEvent::StartFinished(result)) => {
                    self.start_pending = false;
                    if let Err(error) = result {
                        self.error_message = Some(error);
                    }
                    self.refresh_runtime();
                    updated = true;
                }
                Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => {
                    break;
                }
            }
        }

        if updated {
            app.redraw();
        }
    }

    pub fn refresh_runtime(&mut self) {
        self.controller.refresh();
        let snapshot = self.controller.status();
        self.apply_snapshot(snapshot);
        if self.error_message.is_none() {
            self.error_message = self.config_error_message.clone();
        }
        self.logs = self.controller.logs();
    }

    pub fn load_config_from_fields(&self) -> PumpkinConfig {
        PumpkinConfig {
            server_directory: PathBuf::from(self.server_directory.text.trim()),
            executable: PathBuf::from(self.executable.text.trim()),
            arguments: parse_arguments(&self.arguments.text),
        }
    }

    pub fn save_config(&mut self, app: &mut PaneState) {
        let config = self.load_config_from_fields();
        self.controller.set_config(config);
        match self.controller.save_config() {
            Ok(()) => {
                self.error_message = None;
                self.config_error_message = None;
                self.status_message = "Config saved".to_string();
            }
            Err(error) => {
                self.error_message = Some(error.to_string());
                self.status_message = "Save failed".to_string();
            }
        }

        app.redraw();
    }

    pub fn start_server(&mut self, app: &mut PaneState) {
        if self.start_pending {
            return;
        }

        let config = self.load_config_from_fields();
        self.controller.set_config(config);
        self.start_pending = true;
        self.error_message = None;
        self.status_message = ServerStatus::Starting.label().to_string();

        let controller = self.controller.clone();
        let event_tx = self.event_tx.clone();
        let wake = app.waker();

        thread::spawn(move || {
            let result = controller.start().map_err(|error| error.to_string());
            let _ = event_tx.send(UiEvent::StartFinished(result));
            wake.wake();
        });

        app.redraw();
    }

    pub fn stop_server(&mut self, app: &mut PaneState) {
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

        app.redraw();
    }

    pub fn reload_logs(&mut self, app: &mut PaneState) {
        self.refresh_runtime();
        app.redraw();
    }

    fn apply_snapshot(&mut self, snapshot: ServerSnapshot) {
        self.status_message = snapshot.status.label().to_string();
        self.error_message = snapshot.last_error;
        if let Some(process_id) = snapshot.process_id {
            self.status_message = format!("{} · pid {}", snapshot.status.label(), process_id);
        }
    }

    fn start_refresh_worker(&mut self, app: &mut PaneState) {
        if self.refresh_worker_started {
            return;
        }

        self.refresh_worker_started = true;
        let event_tx = self.event_tx.clone();
        let wake = app.waker();

        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(750));
                if event_tx.send(UiEvent::RefreshTick).is_err() {
                    break;
                }
                wake.wake();
            }
        });
    }
}

fn parse_arguments(input: &str) -> Vec<String> {
    input
        .split_whitespace()
        .filter(|part| !part.is_empty())
        .map(str::to_string)
        .collect()
}
