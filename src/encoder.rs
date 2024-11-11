use std::fmt::Error;
use crate::{error::QRError, InputMode, QRInput};
use crate::versions::{VersionInfo,VERSION_CAPACITIES};

#[derive(Debug, Clone, Copy)]
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

    pub fn get_ec_level(&self) -> Result<ErrorCorrectionLevel, QRError> {
        Ok(self.ec_level)
    }

    pub fn set_ec_level(&mut self, level: ErrorCorrectionLevel) -> Result<(), QRError> {
        let content_length = self.content.get_content().len();
        
        let max_length = match level {
            ErrorCorrectionLevel::L => 7089,
            ErrorCorrectionLevel::M => 5596,
            ErrorCorrectionLevel::Q => 3993,
            ErrorCorrectionLevel::H => 3057,
        };
    
        if content_length > max_length {
            return Err(QRError::InvalidLength(
                format!("Content length {} too large for error correction level {:?}", 
                        content_length, level)
            ));
        }
    
        self.ec_level = level;
        Ok(())
    }

    pub fn determine_version(&self) -> Result<Option<u8>, QRError> {



        Ok(self.version)
    }

    fn can_fit_in_version(version: Option<u8>, mode: InputMode, length: usize) -> bool {

        false
    }




}

