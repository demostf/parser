pub use crate::demo::{
    parser::{
        DemoParser, Parse, ParseError, ParserState, Result, MatchState
    },
    Demo, Stream,
};

pub use bitstream_reader::Result as ReadResult;

mod demo;
