#[macro_use]
extern crate rental;

pub use demo::{
    parser::{DemoParser, Parse, ParseError, ParserState, Result},
    Demo, Stream,
};

mod demo;
mod state;
mod test;

fn main() {
    println!("Hello, world!");
}
