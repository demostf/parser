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
mod handler;
mod messagetypeanalyser;
mod state;

/// Errors that can occur during parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error(display = "Error while reading bits from stream: {}", _0)]
    ReadError(#[error(cause)] ReadError),
    #[error(display = "Malformed utf8 while reading string")]
    MalformedUTF8(#[error(cause)] FromUtf8Error),
    #[error(display = "Malformed demo file: {}", _0)]
    MalformedDemo(#[error(cause)] MalformedDemoError),
    #[error(display = "Unexpected type of compressed data: {}", _0)]
    UnexpectedCompressionType(String),
    #[error(
        display = "Error while decompressing SNAP compressed string table: {}",
        _0
    )]
    SnapError(#[error(cause)] snap::Error),
    #[error(
        display = "Unexpected size after decompressing SNAP data, got {} bytes, expected {} bytes",
        size,
        expected
    )]
    UnexpectedDecompressedSize {
        /// Expected decompressed size
        expected: u32,
        /// Actual decompressed size
        size: u32,
    },
    #[error(display = "Malformed demo file: {}", _0)]
    InvalidDemo(&'static str),
}

/// Malformed demo file
#[derive(Debug, Error)]
pub enum MalformedDemoError {
    #[error(display = "Packet identifier is invalid: {}", _0)]
    InvalidPacketType(u8),
    #[error(display = "Message identifier is invalid: {}", _0)]
    InvalidMessageType(u8),
    #[error(display = "Invalid SendProp type: {}", _0)]
    InvalidSendPropType(u8),
    #[error(display = "Invalid SendProp: {}", _0)]
    InvalidSendProp(MalformedSendPropDefinitionError),
    #[error(
        display = "Unexpected amount of data left after parsing an object, {} bits remaining",
        _0
    )]
    DataRemaining(usize),
    #[error(display = "String table with index {} not found", _0)]
    StringTableNotFound(u8),
    #[error(display = "A malformed game event was read")]
    MalformedGameEvent(#[error(cause)] GameEventError),
    #[error(
        display = "A read game event doesn't contain the expected values, expected type {} for {} event, got type {}",
        expected_type,
        name,
        found_type
    )]
    InvalidGameEvent {
        expected_type: GameEventValueType,
        name: &'static str,
        found_type: GameEventValueType,
    },
    #[error(display = "An entity with an unknown server class({}) was read", _0)]
    UnknownServerClass(ClassId),
    #[error(display = "Unknown send table: {}", _0)]
    UnknownSendTable(SendTableName),
    #[error(
        display = "Property index out of bounds, got {} but only {} props exist",
        _0,
        _1
    )]
    PropIndexOutOfBounds { index: i32, prop_count: usize },
    #[error(display = "An attempt was made to update an unknown entity: {}", _0)]
    UnknownEntity(EntityId),
}

#[derive(Debug, Error)]
pub enum MalformedSendPropDefinitionError {
    #[error(display = "Float property without defined size")]
    UnsizedFloat,
    #[error(display = "Array property without defined size")]
    UnsizedArray,
    #[error(display = "Array property without defined inner type")]
    UntypedArray,
    #[error(display = "Property used that can't be read")]
    InvalidPropType,
    #[error(display = "Array contents can't have the 'ChangesOften' flag")]
    ArrayChangesOften,
    #[error(display = "SendProp value out of range")]
    OutOfRange,
}

#[derive(Debug, Error)]
pub enum GameEventError {
    #[error(display = "Incorrect number of values")]
    IncorrectValueCount,
    #[error(display = "Event with 'none' value")]
    NoneValue,
    #[error(display = "Unknown type")]
    UnknownType,
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> ParseError {
        match err {
            ReadError::Utf8Error(utf8_error) => ParseError::MalformedUTF8(utf8_error),
            _ => ParseError::ReadError(err),
        }
    }
}

impl From<snap::Error> for ParseError {
    fn from(err: snap::Error) -> ParseError {
        ParseError::SnapError(err)
    }
}

impl From<MalformedDemoError> for ParseError {
    fn from(err: MalformedDemoError) -> ParseError {
        ParseError::MalformedDemo(err)
    }
}

impl From<MalformedSendPropDefinitionError> for ParseError {
    fn from(err: MalformedSendPropDefinitionError) -> ParseError {
        ParseError::MalformedDemo(MalformedDemoError::InvalidSendProp(err))
    }
}

impl From<GameEventError> for ParseError {
    fn from(err: GameEventError) -> ParseError {
        ParseError::MalformedDemo(MalformedDemoError::MalformedGameEvent(err))
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
