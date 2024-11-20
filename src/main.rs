use std::fmt::format;
use qrcodegenerator::encoder::ErrorCorrectionLevel;
use qrcodegenerator::{InputMode, QRData};

fn format_bit_string(bit_string: &str, chunk_size: usize) -> String {
    bit_string
        .chars()
        .collect::<Vec<char>>()
        .chunks(chunk_size)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    let mut qr_data = QRData::new();

    let test_cases = vec![
        "12345",
        "ABC123",
        "Hello, World!",
        "Hello, 世界!",
        "12345",
        "123456789012345",
        "9999999999999999999",
        "HELLO WORLD",
        "ABC123",
        "HELLO WORLD:123",
        "TEST-TEST/123:ABC",
        "Hello, World!",
        "Hello... World!!!",
        "Test@email.com",
        "https://test.com",
        "Hello, 世界!",
        "こんにちは",
        "Café München",
        "Hello 👋 World",
        "",
        "A",
        " ",
        "12A",
        "123456",
        "1234567",
        "HELLO WORLD|L",
        "HELLO WORLD|Q",
        "HELLO WORLD|H",
        "ABC123|Q",
        "TEST DATA|L",
        "SECURE DATA|H",
        "99999|Q",
    ];

    for test in test_cases {
        let (content, ec_level) = if test.contains('|') {
            let parts: Vec<&str> = test.split('|').collect();
            (
                parts[0],
                match parts[1] {
                    "L" => ErrorCorrectionLevel::L,
                    "Q" => ErrorCorrectionLevel::Q,
                    "H" => ErrorCorrectionLevel::H,
                    _ => ErrorCorrectionLevel::M,
                },
            )
        } else {
            (test, ErrorCorrectionLevel::M)
        };

        println!("\nTesting: {}", content);
        println!("{}", "-".repeat(40));

        match qr_data.set_content(content) {
            Ok(mode) => {
                qr_data.set_ec_level(ec_level).unwrap();
                println!("Mode: {:?}", qr_data.get_input().get_mode());
                println!("EC Level: {:?}", qr_data.get_ec_level());
                println!("Version: {:?}", qr_data.get_version().unwrap());
                let required_bits = qr_data.get_required_bits();
                println!("Required Bits: {}", required_bits);

                match qr_data.validate_length() {
                    Ok(_) => println!("Length validated"),
                    Err(e) => println!("Length validation failed: ({})", e),
                }

                let (mode_ind, char_count, encoded_data) = qr_data.get_data();

                let count_str = match qr_data.get_input().get_mode() {
                    InputMode::Numeric => match qr_data.get_version() {
                        Some(v) if v <= 9 => format!("{:010b}", char_count),
                        Some(v) if v <= 26 => format!("{:012b}", char_count),
                        Some(_) => format!("{:014b}", char_count),
                        None => format!("{:b}", char_count),
                    },
                    InputMode::Alphanumeric => match qr_data.get_version() {
                        Some(v) if v <= 9 => format!("{:09b}", char_count),
                        Some(v) if v <= 26 => format!("{:011b}", char_count),
                        Some(_) => format!("{:013b}", char_count),
                        None => format!("{:b}", char_count),
                    },
                    InputMode::Byte => match qr_data.get_version() {
                        Some(v) if v <= 9 => format!("{:08b}", char_count),
                        Some(_) => format!("{:016b}", char_count),
                        None => format!("{:b}", char_count),
                    },
                };

                let mode = format!("{:04b}", mode_ind);
                let initial_data = format!("{}{}{}", mode, count_str, encoded_data);

                let final_data = {
                    let mut d = initial_data.clone();
                    let remaining_bits = required_bits as usize - d.len();
                    if remaining_bits > 0 {
                        let terminator_length = remaining_bits.min(4);
                        d.push_str(&"0".repeat(terminator_length));
                    }
                    d
                };

                println!("\nDetailed Bit String:");
                println!("Mode Indicator: {} ({} bits)", format_bit_string(&mode, 4), mode.len());
                println!("Character Count: {} ({} bits)", format_bit_string(&count_str, 4), count_str.len());

                println!(
                    "Encoded Data: {} ({} bits)",
                    format_bit_string(&encoded_data, 4),
                    encoded_data.len()
                );
                let terminator = final_data.get(initial_data.len()..).unwrap_or("");
                println!(
                    "Terminator: {} ({} bits)",
                    format_bit_string(&terminator, 4),
                    terminator.len()
                );
                println!(
                    "\nFinal String ({} bits):",
                    final_data.len()
                );
                println!("{}", format_bit_string(&final_data, 8));
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}
