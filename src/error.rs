use std::fmt;
#[derive(Debug)]
pub enum QRError {
    InvalidInput(String),
    InvalidLength(String),
    EncodingError(String),
    InvalidVersion(String),
}

impl fmt::Display for QRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            QRError::InvalidLength(msg) => write!(f, "Invalid length: {}", msg),
            QRError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            QRError::InvalidVersion(msg) => write!(f, "Invalid version: {}", msg),
        }
    }
}

impl std::error::Error for QRError {}
