use std::fmt;
#[derive(Debug)]
pub enum QRError {
    InvalidInput(String),
    InvalidLength(String),
    EncodingError(String),
}

impl fmt::Display for QRError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QRError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            QRError::InvalidLength(msg) => write!(f, "Invalid length: {}", msg),
            QRError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
        }
    }
}

impl std::error::Error for QRError {}
