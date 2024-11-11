use std::fmt::Error;
use crate::{error::QRError, QRInput};

#[derive(Debug)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

#[derive(Debug)]
pub struct CapacityInfo {
    pub numeric: usize,
    pub alphanumeric: usize,
    pub byte: usize,
    pub kanji: usize,
}

pub struct QRData {
    content: QRInput,
    ec_level: ErrorCorrectionLevel,
    version: Option<u8>,
}

impl QRData {

    pub fn new() -> Self {

        QRData {
            content: QRInput::new(),
            ec_level: ErrorCorrectionLevel::M,
            version: None
        }
    }
}

