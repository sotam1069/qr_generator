use crate::error::QRError;
use std::{collections::HashMap, fmt::format};

#[derive(Debug, Clone, Copy, PartialEq)]
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

    pub fn calculate_character_count_indicator(&self, version: Option<u8>) -> u16 {
        let version = version.unwrap();
        let bit_length = match version {
            1..=9 => match self.mode {
                InputMode::Numeric => 10,
                InputMode::Alphanumeric => 9,
                InputMode::Byte => 8,
                InputMode::Kanji => 8,
            },
            10..=26 => match self.mode {
                InputMode::Numeric => 12,
                InputMode::Alphanumeric => 11,
                InputMode::Byte => 16,
                InputMode::Kanji => 10,
            },
            27..=40 => match self.mode {
                InputMode::Numeric => 14,
                InputMode::Alphanumeric => 13,
                InputMode::Byte => 16,
                InputMode::Kanji => 12,
            },
            0 | 41..=u8::MAX => {
                0
            },
        };

        let count = self.content.chars().count();
        let binary_padded =  format!("{:0width$b}", count, width=bit_length);
        
        return u16::from_str_radix(&binary_padded, 2).unwrap()
    }

    pub fn get_indicator(&self, version: Option<u8>) -> (u8, u16) {
        let mode_indicator = self.get_mode_indicator();
        let char_count_indicator = self.calculate_character_count_indicator(version);

        (mode_indicator, char_count_indicator)
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