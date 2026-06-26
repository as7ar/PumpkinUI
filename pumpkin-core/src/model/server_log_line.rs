use super::ServerStream;

#[derive(Debug, Clone)]
pub struct ServerLogLine {
    pub stream: ServerStream,
    pub line: String,
}

impl ServerLogLine {
    pub fn render(&self) -> String {
        match self.stream {
            ServerStream::Stdout => format!("[stdout] {}", self.line),
            ServerStream::Stderr => format!("[stderr] {}", self.line),
        }
    }
}
