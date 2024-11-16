use qrcodegenerator::{InputMode, QRData, QRInput};

fn main() {
    let mut qr_data = QRData::new();

    let test_cases = vec![
        "12345",         // Numeric
        "ABC123",        // Alphanumeric
        "Hello, World!", // Byte (ASCII)
        "Hello, 世界!",  // Byte (UTF-8)
        // Numeric cases
        "12345",                    // Short numeric
        "123456789012345",         // Medium numeric
        "9999999999999999999",     // Long numeric
        
        // Alphanumeric cases
        "HELLO WORLD",
        "ABC123",                  // Short alphanumeric
        "HELLO WORLD:123",         // With allowed special chars
        "TEST-TEST/123:ABC",       // Mix of allowed chars
        
        // Byte cases (ASCII)
        "Hello, World!",           // Simple ASCII
        "Hello... World!!!",       // With punctuation
        "Test@email.com",          // Email format
        "https://test.com",        // URL format
        
        // Byte cases (UTF-8)
        "Hello, 世界!",            // Mixed ASCII and Unicode
        "こんにちは",              // Pure Japanese
        "Café München",            // European chars
        "Hello 👋 World",          // With emoji
        
        // Edge cases
        "",                        // Empty string (should error)
        "A",                       // Single character
        " ",                       // Single space
        "12A",                     // Numeric + Alpha
        "123456",                  // 10 bits all three groups
        "1234567",                 // Single digit remainder

    ];

    for test in test_cases {
        println!("\nTesting: {}", test);
        println!("{}","-".repeat(40));
        
        match qr_data.set_content(test) {
            Ok(mode) => {

                let mode_indicator = qr_data.get_input().get_mode_indicator();
                let char_count = qr_data.get_input().calculate_character_count_indicator(qr_data.get_version());

                println!("Mode: {:?}", qr_data.get_input().get_mode());
                println!("Mode Indicator: {:04b}", qr_data.get_input().get_mode_indicator());
                println!("EC Level: {:?}", qr_data.get_ec_level());
                println!("Version: {:?}", qr_data.get_version().unwrap());


                match qr_data.validate_length() {
                    Ok(_) => println!("Length validated"),
                    Err(e) => println!("Length validation failed: ({})", e),
                }

                let (mode_ind, char_count, encoded_data) = qr_data.get_data();

                println!("Bit String: ({:04b}, {}, {:?})",
                         mode_ind,
                         match qr_data.get_input().get_mode() {
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
                         },
                         encoded_data
                );
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}