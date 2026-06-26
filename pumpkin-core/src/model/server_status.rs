#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

impl ServerStatus {
    pub fn label(self) -> &'static str {
        match self {
            ServerStatus::Stopped => "Stopped",
            ServerStatus::Starting => "Starting",
            ServerStatus::Running => "Running",
            ServerStatus::Stopping => "Stopping",
            ServerStatus::Failed => "Failed",
        }
    }
}
