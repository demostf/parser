# TF Demo Parser

![Build Status](https://github.com/demostf/parser/workflows/CI/badge.svg)

Parsing of tf2 demo files

## Building

This project is build using rust and requires `cargo` and friends, see [the rust website](https://www.rust-lang.org/)
for how to get started.

Once rust is setup building is as simple as

```bash
cargo build --release
```

which will place the binary at `target/release/parse_demo`

## Usage

Basic usage is as simple as `parse_demo demofile.dem` which will output a "summary" of the demo file in JSON format.

Passing the `detailed_summary` argument to the end of `parse_demo` will output a table with scoreboard information for all players who were ever on the server while the demo
was being recorded.  The player who created the demo will be highlighted in the output.

## Advanced usage

### Loop through every packet

```rust
use bitbuffer::BitRead;
use main_error::MainError;
use std::fs;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::parser::{DemoHandler, RawPacketStream};
use tf_demo_parser::Demo;

fn main() -> Result<(), MainError> {
    let file = fs::read("demofile.dem")?;

    let demo = Demo::new(&file);
    let mut handler = DemoHandler::default();

    let mut stream = demo.get_stream();
    let header = Header::read(&mut stream)?;
    handler.handle_header(&header);

    let mut packets = RawPacketStream::new(stream);

    while let Some(packet) = packets.next(&handler.state_handler)? {
        handler.handle_packet(packet).unwrap();
    }
    assert_eq!(false, packets.incomplete);

    Ok(())
}
```

### Handle demo data with a custom analyser

Sometimes it's easier to create a custom `Analyser` to handle the demo data as it comes along.

See `src/demo/parser/analyser.rs` for an example.  
Once you have a custom analyser you can use it with:

```rust
DemoParser::new_all_with_analyser(demo.get_stream(), CustomAnalyser::new());
let (header, state) = parser.parse()?;
```