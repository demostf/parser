#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub use bitbuffer::Result as ReadResult;

pub use crate::demo::{
    message::MessageType,
    parser::{
        DemoParser, GameEventError, MatchState, MessageTypeAnalyser, Parse, ParseError,
        ParserState, Result,
    },
    Demo, Stream,
};

pub mod demo;
mod nullhasher;
