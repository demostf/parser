#[allow(unused_variables)]
use std::error::Error;
use std::fs;

pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Result, Stream};

fn main() -> std::result::Result<(), Box<ParseError>> {
    let file = fs::read("data/gully.dem").expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();
    let mut parser = DemoParser::new(stream);
    let (header, state) = parser.parse_demo()?;
    dbg!(header);
    dbg!(state);
    Ok(())
}
