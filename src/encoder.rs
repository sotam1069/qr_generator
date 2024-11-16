use crate::versions::{VERSION_CAPACITIES};
use crate::{error::QRError, InputMode, QRInput};

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
            };

            if fits {
                self.version = Some((version_index + 1) as u8);
                return Ok(self.version);
            }
        }
        self.version = None;
        Ok(None)
    }

    pub fn get_character_count_indicator(&mut self) -> u16 {

        let version = self.get_version();

        self.input.calculate_character_count_indicator(version)
    }

    fn numeric_encoding(&self) -> Vec<String> {

        let input = self.input.get_content();
        let chars: Vec<char> = input.chars().collect();
        
        let chunks: Vec<Vec<char>> = chars.chunks(3)
        .map(|chunk| chunk.to_vec())
        .collect();
        let mut converted_chunks: Vec<String> = vec![];

        for chunk in chunks {

            match chunk.len() {
                3 => {
                    let number = chunk.iter().collect::<String>().parse::<u16>().unwrap();
                    let binary = format!("{:010b}", number);
                    converted_chunks.push(binary);

                },
                2 => {
                    let number = chunk.iter().collect::<String>().parse::<u16>().unwrap();
                    let binary = format!("{:07b}", number);
                    converted_chunks.push(binary);
                },
                1 => {
                    let number = chunk.iter().collect::<String>().parse::<u16>().unwrap();
                    let binary = format!("{:04b}", number);
                    converted_chunks.push(binary);
                },
                _ => {}
            }
        }
        converted_chunks
    }

    fn get_alphanumeric_value(c: char) -> u8 {
        match c {
            '0'..='9' => c as u8 - b'0',        // 0-9 map to 0-9
            'A'..='Z' => (c as u8 - b'A') + 10,  // A=10, B=11, etc.
            ' ' => 36,
            '$' => 37,
            '%' => 38,
            '*' => 39,
            '+' => 40,
            '-' => 41,
            '.' => 42,
            '/' => 43,
            ':' => 44,
            _ => panic!("Invalid alphanumeric character")
        }
    }

    fn alphanumeric_encoding(&self) -> Vec<String> {

        let input = self.input.get_content();
        let chars: Vec<char> = input.chars().collect();

        let chunks: Vec<Vec<char>> = chars.chunks(2)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut converted_chunks: Vec<String> = vec![];
        for chunk in chunks {
            match chunk.len() {
                2 => {
                    let first = Self::get_alphanumeric_value(chunk[0]) as u16;
                    let second = Self::get_alphanumeric_value(chunk[1]) as u16;
                    let number = (first * 45) + second;
                    let binary = format!("{:011b}", number);
                    converted_chunks.push(binary);
                },
                1 => {
                    let value = Self::get_alphanumeric_value(chunk[0]) as u16;
                    let binary = format!("{:06b}", value);
                    converted_chunks.push(binary);
                },
                _ => {}
            }
        }
        converted_chunks
    }

    fn byte_encoding(&self) -> Vec<String> {

        let input = self.input.get_content();
        let mut converted_chunks: Vec<&u8> = vec![];
        let mut result : Vec<String> = vec![];

        for byte in input.as_bytes() {

            converted_chunks.push(byte);
        }

        for n in converted_chunks {
            let binary = format!("{:08b}", n);
            result.push(binary);
        }

        result
    }

    pub fn get_data(&self) -> (u8, u16, Vec<String>) {

        let indicators = self.input.get_indicator(self.version);
        let encoded_data = self.encode();

        (indicators.0,indicators.1, encoded_data)

    }

    pub fn encode(&self) -> Vec<String> {

        match self.input.get_mode() {
            InputMode::Numeric => {
                self.numeric_encoding()
            },
            InputMode::Alphanumeric => {
                self.alphanumeric_encoding()
            },
            InputMode::Byte => {
                self.byte_encoding()
            },
        }
    }


}