use crate::error::QRError;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum InputMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl InputMode {

    pub fn get_indicator(self) -> u8 {
         match self {
            InputMode::Numeric => 0b0001,
            InputMode::Alphanumeric => 0b0010,
            InputMode::Byte => 0b0100,
            InputMode::Kanji => 0b1000,
        }
    }
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
        self.determine_mode()?;
        Ok(self.mode)
    }

    pub fn get_mode(&self) -> InputMode {
        self.mode
    }

    fn determine_mode(&mut self) -> Result<(), QRError> {
        let content = self.content.as_str();

       self.mode =  if content.chars().all(|c| c.is_ascii_digit()) {
            InputMode::Numeric
        } else if content
            .chars()
            .all(|c| self.alphanumeric_chars.contains_key(&c))
        {
            InputMode::Alphanumeric
        } else {
            InputMode::Byte
        };

        Ok(())

    }

    pub fn get_mode_indicator(&self) -> u8 {
        self.mode.get_indicator()
    }

    pub fn validate_length(&self) -> Result<(), QRError> {
        let len = self.content.len();
        let max_length = match self.mode {
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
                self.mode
            )));
        }

        Ok(())
    }
}