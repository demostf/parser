use bitstream_reader::{BitRead, BitSkip, LittleEndian, ReadError};

pub use self::messagetypeanalyser::MessageTypeAnalyser;
use crate::demo::gamevent::{GameEventValue, GameEventValueType};
use crate::demo::header::Header;
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
pub use crate::demo::parser::analyser::MatchState;
pub use crate::demo::parser::handler::{DemoHandler, MessageHandler};
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod analyser;
mod handler;
mod messagetypeanalyser;
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
    /// Invalid SendProp
    InvalidSendProp(String),
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
    /// Misc malformed demo error
    InvalidDemo(String),
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

    pub fn parse_with_analyser<T: MessageHandler>(
        mut stream: Stream,
        analyser: T,
    ) -> Result<(Header, T::Output)> {
        let mut handler = DemoHandler::with_analyser(analyser);
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
