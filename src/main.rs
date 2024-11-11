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
        println!("\nTesting: {}", test);
        println!("{}","-".repeat(40));
        
        match qr_data.set_content(test) {
            Ok(mode) => {
                println!("Mode: {:?}", qr_data.get_input().get_mode());
                println!("Mode Indicator: {:04b}", qr_data.get_input().get_mode_indicator());
                println!("EC Level: {:?}", qr_data.get_ec_level());
                
                match qr_data.get_version() {
                    Some(version) => println!("Version: {}", version),
                    None => println!("Content too large for any version"),
                }

                match qr_data.validate_length() {
                    Ok(_) => println!("Length validated"),
                    Err(e) => println!("Length validation failed: ({})", e),
                }
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}