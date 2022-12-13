#![no_main]
use libfuzzer_sys::fuzz_target;

pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

fn fuzz(data: &[u8]) {
    let demo = Demo::new(data);
    let parser = DemoParser::new_all(demo.get_stream());
    let _ = parser.parse();
}

fuzz_target!(|data: &[u8]| { fuzz(data) });
