#[macro_use]
extern crate rental;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

pub use demo::{Demo, parser::{Parse, ParseError, ParserState, Result, DemoParser}, Stream};

mod demo;
mod state;
mod test;

fn main() {
    println!("Hello, world!");
}

