# ITCH Parser

A Rust library for parsing NASDAQ ITCH 5.0 protocol messages.

## Features

- Complete support for all ITCH 5.0 message types
- Zero-copy parsing with efficient binary deserialization
- Comprehensive error handling
- Type-safe message representation

## Usage

```rust
use itch_parser::{ItchParser, ItchMessage};

let parser = ItchParser::new();
let message_data = vec![/* binary ITCH message */];

match parser.parse_message(&message_data) {
    Ok(message) => match message {
        ItchMessage::AddOrder { stock, price, shares, .. } => {
            println!("Add order for {} at ${} for {} shares", stock, price, shares);
        }
        _ => println!("Other message type: {:?}", message),
    },
    Err(e) => eprintln!("Parse error: {}", e),
}
```

## Supported Message Types

- System Event (S)
- Stock Directory (R)
- Stock Trading Action (H)
- Reg SHO Restriction (Y)
- Market Participant Position (L)
- MWCB Decline Level (V)
- MWCB Status (W)
- IPO Quoting Period Update (K)
- Add Order (A)
- Add Order - MPID Attribution (F)
- Order Executed (E)
- Order Executed With Price (C)
- Order Cancel (X)
- Order Delete (D)
- Order Replace (U)
- Trade (P)
- Cross Trade (Q)
- Broken Trade (B)
- Net Order Imbalance Indicator (I)
- Retail Price Improvement Indicator (N)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

