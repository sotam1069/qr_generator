use std::collections::HashMap;
use crate::error::QRError;

#[derive(Debug)]
pub enum InputMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji
}

pub struct QRInput {
    content: String,
    mode: InputMode,
    alphanumeric_chars: HashMap<char, bool>
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
            alphanumeric_chars: alpha_chars
        }
    }

    pub fn set_content(&mut self, text: &str) -> Result<InputMode, QRError> {
        if text.is_empty() {
            return Err(QRError::InvalidInput("Input text cannot be empty".to_string()));
        }
        self.content = text.to_string();
        self.determine_mode()
    }

    pub fn determine_mode(&self) -> Result<InputMode, QRError> {
        let content = self.content.as_str();

        if content.chars().all(|c| c.is_ascii_digit()) {
            return Ok(InputMode::Numeric);
        }

        if content.chars().all(|c| self.alphanumeric_chars.contains_key(&c)) {
            return Ok(InputMode::Alphanumeric);
        }

        if content.chars().all(|c| c as u32 <= 255) {
            return Ok(InputMode::Byte);
        }

        Ok(InputMode::Byte)
    }

    pub fn validate_length(&self) -> Result<(), QRError> {
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
                len, max_length, self.determine_mode()?
            )));
        }

        Ok(())
    }

    fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_mode(&self) -> Result<InputMode, QRError> {
        self.determine_mode()
    }

}


fn main() {
    let mut input = QRInput::new();
    
    // Example inputs
    let test_cases = vec![
        "12345",                    // Numeric
        "ABC123",                   // Alphanumeric
        "Hello, World!",            // Byte (ASCII)
        "Hello, 世界!",             // Byte (UTF-8)
    ];

    for test in test_cases {
        match input.set_content(test) {
            Ok(mode) => {
                println!("Input: {}", test);
                println!("Mode: {:?}", mode);
                match input.validate_length() {
                    Ok(_) => println!("Length validation passed"),
                    Err(e) => println!("Length validation failed: {}", e),
                }
                println!();
            },
            Err(e) => println!("Error setting content: {}", e),
        }
    }
}