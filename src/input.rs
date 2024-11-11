use crate::error::QRError;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

#[derive(Debug, Clone)]
pub struct QRInput {
    content: String,
    mode: InputMode,
    alphanumeric_chars: HashMap<char, bool>,
}

impl QRInput {
    pub fn new() -> Self {
        let mut alpha_chars = HashMap::new();

        for c in '0'..='9' {
            alpha_chars.insert(c, true);
        }

        for c in 'A'..='Z' {
            alpha_chars.insert(c, true);
        }

        for c in [' ', '$', '%', '*', '+', '-', '.', '/', ':'] {
            alpha_chars.insert(c, true);
        }

        QRInput {
            content: String::new(),
            mode: InputMode::Numeric,
            alphanumeric_chars: alpha_chars,
        }
    }
    
    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn set_content(&mut self, text: &str) -> Result<InputMode, QRError> {
        if text.is_empty() {
            return Err(QRError::InvalidInput(
                "Input text cannot be empty".to_string(),
            ));
        }
        self.content = text.to_string();
        self.determine_mode()
    }

    pub fn get_mode(&mut self) -> Result<InputMode, QRError> {
        self.determine_mode()
    }

    pub fn determine_mode(&mut self) -> Result<InputMode, QRError> {
        let content = self.content.as_str();

        if content.chars().all(|c| c.is_ascii_digit()) {
            self.mode = InputMode::Numeric;
            return Ok(InputMode::Numeric);
        }

        if content
            .chars()
            .all(|c| self.alphanumeric_chars.contains_key(&c))
        {
            self.mode = InputMode::Alphanumeric;
            return Ok(InputMode::Alphanumeric);
        }

        if content.chars().all(|c| c as u32 <= 255) {
            self.mode = InputMode::Byte;
            return Ok(InputMode::Byte);
        }

        self.mode = InputMode::Byte;
        Ok(InputMode::Byte)
    }

    pub fn get_mode_indicator(&self) -> Result<u8, QRError> {
        match self.mode {
            InputMode::Numeric => Ok(0b0001),
            InputMode::Alphanumeric => Ok(0b0010),
            InputMode::Byte => Ok(0b0100),
            InputMode::Kanji => Ok(0b1000),
        }
    }

    pub fn validate_length(&mut self) -> Result<(), QRError> {
        let len = self.content.len();
        let max_length = match self.determine_mode()? {
            InputMode::Numeric => 7089,
            InputMode::Alphanumeric => 4296,
            InputMode::Byte => 2953,
            InputMode::Kanji => 1817,
        };

        if len > max_length {
            return Err(QRError::InvalidLength(format!(
                "Input length {} exceeds maximum {} for mode {:?}",
                len,
                max_length,
                self.determine_mode()?
            )));
        }

        Ok(())
    }
}