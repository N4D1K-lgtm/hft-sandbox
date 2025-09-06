use thiserror::Error;

#[derive(Error, Debug)]
pub enum ItchError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid message type: {0}")]
    InvalidMessageType(u8),
    #[error("Invalid message length: expected {expected}, got {actual}")]
    InvalidLength { expected: usize, actual: usize },
    #[error("Invalid message field {field}: expected one of {expected:?}, got {actual}")]
    InvalidCharField {
        field: &'static str,
        expected: &'static [char],
        actual: char,
    },
}

pub type Result<T> = std::result::Result<T, ItchError>;
