use bitstream_reader::{BitRead, LittleEndian, ReadError};

use crate::demo::gamevent::{GameEventValue, GameEventValueType};
use crate::demo::header::Header;
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::MatchState;
use crate::demo::parser::handler::DemoHandler;
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod analyser;
mod handler;
mod state;

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Error while reading bits from stream
    ReadError(ReadError),
    /// Packet identifier is invalid
    InvalidPacketType(u8),
    /// Message identifier is invalid
    InvalidMessageType(u8),
    /// SendProp type is invalid
    InvalidSendPropType(u8),
    /// Invalid structure found while creating array SendProp
    InvalidSendPropArray(String),
    /// Expected amount of data left after parsing an object
    DataRemaining(usize),
    /// String table that was send for update doesn't exist
    StringTableNotFound(u8),
    /// A unknown game event type was read
    UnknownGameEvent(String),
    /// A read game event doesn't contain the expected values
    InvalidGameEvent {
        expected_type: GameEventValueType,
        name: String,
        value: GameEventValue,
    },
    /// Unexpected type of compressed data
    UnexpectedCompressionType(String),
    /// Error while decompressing SNAP compressed string table
    SnapError(snap::Error),
    /// Unexpected size after decompressing SNAP data
    UnexpectedDecompressedSize {
        /// Expected decompressed size
        expected: u32,
        /// Actual decompressed size
        size: u32,
    },
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> ParseError {
        ParseError::ReadError(err)
    }
}

impl From<snap::Error> for ParseError {
    fn from(err: snap::Error) -> ParseError {
        ParseError::SnapError(err)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parse: Sized {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self>;
}

impl<T: BitRead<LittleEndian>> Parse for T {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser {
    stream: Stream,
    handler: DemoHandler,
}

impl DemoParser {
    pub fn new(stream: Stream) -> Self {
        DemoParser {
            stream,
            handler: DemoHandler::new(),
        }
    }

    #[inline(always)]
    pub fn read<T: Parse>(&mut self) -> Result<T> {
        T::parse(&mut self.stream, self.handler.get_parser_state())
    }

    pub fn parse_demo(mut self) -> Result<(Header, MatchState)> {
        let header = self.read::<Header>()?;
        loop {
            let packet = self.read::<Packet>()?;
            match packet {
                Packet::Stop(_) => break,
                packet => self.handler.handle_packet(packet),
            };
        }
        Ok((header, self.handler.get_match_state()))
    }
}
