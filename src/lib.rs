use std::error::Error;
use std::fs;

pub use crate::demo::{
    parser::{DemoParser, Parse, ParseError, ParserState, Result},
    Demo, Stream,
};

pub use bitstream_reader::Result as ReadResult;

mod demo;
mod state;
mod test;
