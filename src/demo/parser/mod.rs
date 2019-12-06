use bitstream_reader::{BitRead, BitSkip, FromUtf8Error, LittleEndian, ReadError};

pub use self::messagetypeanalyser::MessageTypeAnalyser;
use crate::demo::gamevent::{GameEventValue, GameEventValueType};
use crate::demo::header::Header;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{ClassId, SendTableName};
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
pub use crate::demo::parser::analyser::MatchState;
pub use crate::demo::parser::handler::{DemoHandler, MessageHandler};
pub use crate::demo::parser::state::ParserState;
use crate::Stream;
use err_derive::Error;

mod analyser;
mod error;
pub mod gamestateanalyser;
mod handler;
mod messagetypeanalyser;
mod state;

pub use self::error::*;

pub trait Parse: Sized {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self>;
}

impl<T: BitRead<LittleEndian>> Parse for T {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub trait ParseBitSkip {
    fn parse_skip(stream: &mut Stream) -> Result<()>;
}

impl<T: BitSkip<LittleEndian>> ParseBitSkip for T {
    #[inline(always)]
    fn parse_skip(stream: &mut Stream) -> Result<()> {
        Self::skip(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser {}

impl DemoParser {
    pub fn parse_demo(stream: Stream) -> Result<(Header, MatchState)> {
        Self::parse_with_analyser(stream, Analyser::new())
    }

    pub fn parse_all(stream: Stream) -> Result<(Header, MatchState)> {
        Self::parse_all_with_analyser(stream, Analyser::new())
    }

    pub fn parse_with_analyser<T: MessageHandler>(
        stream: Stream,
        analyser: T,
    ) -> Result<(Header, T::Output)> {
        Self::parse(stream, analyser, false)
    }

    pub fn parse_all_with_analyser<T: MessageHandler>(
        stream: Stream,
        analyser: T,
    ) -> Result<(Header, T::Output)> {
        Self::parse(stream, analyser, true)
    }

    fn parse<T: MessageHandler>(
        mut stream: Stream,
        analyser: T,
        all: bool,
    ) -> Result<(Header, T::Output)> {
        let mut handler = if all {
            DemoHandler::parse_all_with_analyser(analyser)
        } else {
            DemoHandler::with_analyser(analyser)
        };
        let header = Header::read(&mut stream)?;
        loop {
            let packet = Packet::parse(&mut stream, handler.get_parser_state())?;
            match packet {
                Packet::Stop(_) => break,
                packet => handler.handle_packet(packet),
            };
        }
        Ok((header, handler.get_output()))
    }
}
