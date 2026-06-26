use super::ServerStatus;

#[derive(Debug, Clone)]
pub struct ServerSnapshot {
    pub status: ServerStatus,
    pub process_id: Option<u32>,
    pub last_error: Option<String>,
}
