#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerStream {
    Stdout,
    Stderr,
}
