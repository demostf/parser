use std::env;
use std::fs;

use main_error::MainError;
use std::thread::sleep;
use std::time::Duration;
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
    sleep(Duration::from_secs(1));
    for _ in 0..10 {
        parse(&file, all)?;
    }
    Ok(())
}

#[inline(never)]
fn parse(data: &[u8], all: bool) -> Result<(), MainError> {
    let demo = Demo::new(data);
    let parser = if all {
        DemoParser::new_all(demo.get_stream())
    } else {
        DemoParser::new(demo.get_stream())
    };
    let (_, state) = parser.parse()?;
    assert!(state.rounds.len() > 1);
    Ok(())
}
