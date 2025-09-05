use itch_parser::ItchParser;

fn main() {
    let parser = ItchParser::new();
    
    let system_event_data = vec![
        b'S', // message type
        0x00, 0x01, // stock locate
        0x00, 0x02, // tracking number
        0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // timestamp (48-bit)
        b'O', // event code
    ];

    match parser.parse_message(&system_event_data) {
        Ok(message) => println!("Parsed message: {:?}", message),
        Err(e) => println!("Error parsing message: {}", e),
    }

    let mut add_order_data = vec![
        b'A', // message type
        0x00, 0x01, // stock locate
        0x00, 0x02, // tracking number
        0x00, 0x00, 0x00, 0x00, 0x00, 0x03, // timestamp (48-bit)
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, // order reference number
        b'B', // buy/sell indicator
        0x00, 0x00, 0x00, 0x64, // shares (100)
    ];
    add_order_data.extend_from_slice(b"AAPL    "); // stock (8 chars)
    add_order_data.extend_from_slice(&[0x00, 0x00, 0x27, 0x10]); // price

    match parser.parse_message(&add_order_data) {
        Ok(message) => println!("Parsed message: {:?}", message),
        Err(e) => println!("Error parsing message: {}", e),
    }
}
