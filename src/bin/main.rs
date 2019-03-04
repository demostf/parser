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
    //dbg!(header);
    //dbg!(state.deaths);
    //std::thread::sleep(std::time::Duration::from_secs(5));
    let json = serde_json::to_string(&state).unwrap_or("err".to_string());
    println!("{}", json);
    Ok(())
}
