pub type Result<T> = std::result::Result<T, CaptureError>;

#[derive(Debug)]
pub enum CaptureError {
    UnknownProtocol(String),
    Errno(nix::errno::Errno),
    Io(std::io::ErrorKind),
}

impl std::error::Error for CaptureError {}

impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptureError::UnknownProtocol(proto) => write!(f, "Unknown protocol `{proto}`."),
            CaptureError::Errno(errno) => write!(f, "Error {errno}: {}", errno.desc()),
            CaptureError::Io(err) => write!(f, "IO error: {err}"),
        }
    }
}
