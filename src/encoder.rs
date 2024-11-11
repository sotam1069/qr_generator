use crate::versions::{VersionInfo, VERSION_CAPACITIES};
use crate::{error::QRError, InputMode, QRInput};
use std::fmt::Error;

#[derive(Debug, Clone, Copy)]
pub enum ErrorCorrectionLevel {
    L,
    M,
    Q,
    H,
}

#[derive(Debug, Clone)]
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
            version: None,
        }
    }

    pub fn get_content(&self) -> Result<&QRInput, QRError> {
        Ok(&self.content)
    }

    pub fn set_content(&mut self, text: &str) -> Result<InputMode, QRError> {
        self.content.set_content(text)
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
            return Err(QRError::InvalidLength(format!(
                "Content length {} too large for error correction level {:?}",
                content_length, level
            )));
        }

        self.ec_level = level;
        Ok(())
    }

    pub fn validate_length(&mut self) -> Result<(), QRError> {
        self.content.validate_length()
    }

    pub fn determine_version(&mut self) -> Result<Option<u8>, QRError> {

        let mode = self.content.get_mode()?;
        let length = self.content.get_content().len();

        for(version_index, version_info) in VERSION_CAPACITIES.iter().enumerate() {
            let capacity = &version_info.capacity_by_ec[self.ec_level as usize];

            let fits = match mode {
                InputMode::Numeric => length <= capacity.numeric,
                InputMode::Alphanumeric => length <= capacity.alphanumeric,
                InputMode::Byte => length <= capacity.byte,
                InputMode::Kanji => length <= capacity.kanji,
            };

            if fits {
                return Ok(Some((version_index + 1) as u8));
            }
        }

        Ok(None)
    }


}
