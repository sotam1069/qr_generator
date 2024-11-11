use qrcodegenerator::{InputMode, QRInput, QRData};

fn main() {
    let mut input = QRData::new();
    
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
                    Err(e) => println!("Length validation error: {}", e),
                }
                println!();
            },
            Err(e) => println!("Error setting content: {}", e),
        }
    }
}