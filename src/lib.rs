pub use bitstream_reader::Result as ReadResult;

pub use crate::demo::{
    Demo,
    message::MessageType, parser::{
        DemoParser, MatchState, MessageTypeAnalyser, Parse, ParseError, ParserState, Result,
    },
    Stream,
};

mod demo;
