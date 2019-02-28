use std::error::Error;
use std::fs;

pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Result, Stream};

fn main() -> std::result::Result<(), Box<ParseError>> {
    let file = fs::read("data/small.dem").expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();
    let mut parser = DemoParser::new(stream);
    let (header, packets) = parser.parse_demo()?;
    dbg!(header);
    Ok(())
}
