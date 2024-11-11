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
    input: QRInput,
    ec_level: ErrorCorrectionLevel,
    version: Option<u8>,
}

impl QRData {
    pub fn new() -> Self {
        QRData {
            input: QRInput::new(),
            ec_level: ErrorCorrectionLevel::M,
            version: None,
        }
    }

    pub fn get_input(&self) -> &QRInput {
        &self.input
    }

    pub fn set_content(&mut self, text: &str) -> Result<InputMode, QRError> {
        let mode = self.input.set_content(text)?;

        self.determine_version()?;
        Ok(mode)
    }

    pub fn get_ec_level(&self) -> ErrorCorrectionLevel {
        self.ec_level
    }

    pub fn set_ec_level(&mut self, level: ErrorCorrectionLevel) -> Result<(), QRError> {
        let content_length = self.input.get_content().len();

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
        self.determine_version()?;
        Ok(())
    }

    pub fn validate_length(&mut self) -> Result<(), QRError> {
        self.input.validate_length()
    }

    pub fn get_version(&self) -> Option<u8> {
        self.version
    }

    pub fn determine_version(&mut self) -> Result<Option<u8>, QRError> {

        let mode = self.input.get_mode();
        let length = self.input.get_content().len();

        for(version_index, version_info) in VERSION_CAPACITIES.iter().enumerate() {
            let capacity = &version_info.capacity_by_ec[self.ec_level as usize];

            let fits = match mode {
                InputMode::Numeric => length <= capacity.numeric,
                InputMode::Alphanumeric => length <= capacity.alphanumeric,
                InputMode::Byte => length <= capacity.byte,
                InputMode::Kanji => length <= capacity.kanji,
            };

            if fits {
                self.version = Some((version_index + 1) as u8);
                return Ok(self.version);
            }
        }
        self.version = None;
        Ok(None)
    }
}