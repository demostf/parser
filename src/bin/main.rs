use std::env;
use std::fs;

pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Result, Stream};

fn main() -> std::result::Result<(), Box<ParseError>> {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        panic!("1 argument required");
    }
    let path = args[1].clone();
    let file = fs::read(path).expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();
    let parser = DemoParser::new(stream);
    let (_, state) = parser.parse_demo()?;
    let json = serde_json::to_string(&state).unwrap_or("err".to_string());
    println!("{}", json);
    Ok(())
}
