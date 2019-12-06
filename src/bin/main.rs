use std::env;
use std::fs;

use main_error::MainError;
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), MainError> {
    #[cfg(feature = "better_panic")]
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let all = args
        .get(2)
        .map(|arg| arg.as_str() == "all")
        .unwrap_or_default();
    let file = fs::read(path)?;
    let demo = Demo::new(file);
    let (_, state) = if all {
        DemoParser::parse_all(demo.get_stream())
    } else {
        DemoParser::parse_demo(demo.get_stream())
    }?;
    println!("{}", serde_json::to_string(&state)?);
    Ok(())
}
