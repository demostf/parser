#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use bitstream_reader::Result as ReadResult;

pub use crate::demo::{
    Demo,
    message::MessageType, parser::{
        DemoParser, MatchState, MessageTypeAnalyser, Parse, ParseError, ParserState, Result,
    },
    Stream,
};

pub mod demo;
