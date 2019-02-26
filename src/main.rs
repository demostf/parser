#[macro_use]
extern crate rental;

use std::error::Error;
use std::fs;

pub use demo::{
    Demo,
    parser::{DemoParser, Parse, ParseError, ParserState, Result}, Stream,
};

mod demo;
mod state;
mod test;

fn main() -> std::result::Result<(), Box<ParseError>> {
    let file = fs::read("data/small.dem").expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();
    let mut parser = DemoParser::new(stream);
    let (header, packets) = parser.parse_demo()?;
    dbg!(header);
    Ok(())
}
