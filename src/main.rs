use qrcodegenerator::{InputMode, QRData, QRInput};

fn main() {
    let mut input = QRData::new();

    let test_cases = vec![
        "12345",         // Numeric
        "ABC123",        // Alphanumeric
        "Hello, World!", // Byte (ASCII)
        "Hello, 世界!",  // Byte (UTF-8)// Numeric cases
        "12345",                    // Short numeric
        "123456789012345",         // Medium numeric
        "9999999999999999999",     // Long numeric
        
        // Alphanumeric cases
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
    ];

    for test in test_cases {
        match input.set_content(test) {
            Ok(mode) => {
                println!("Input: {}", test);
                println!("Mode: {:?}", mode);
                match input.validate_length() {
                    Ok(_) => println!("Length validation passed"),
                    Err(e) => println!("Length validation error: {}", e),
                }
                println!("EC Level: {:?}", input.get_ec_level().unwrap());

                match input.determine_version() {
                    Ok(Some(version)) => println!("Version: {}", version),
                    Ok(None) => println!("Content too large for any version"),
                    Err(e) => println!("Error determining version: {}", e),
                }

                if let Ok(indicator) = input.get_content().unwrap().get_mode_indicator() {
                    println!("Mode Indicator: {:04b}", indicator);
                }
                println!();
            }
            Err(e) => println!("Error setting content: {}", e),
        }
    }
}
