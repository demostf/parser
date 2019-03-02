use std::collections::HashMap;

use bitstream_reader::{BitRead, LittleEndian, ReadError};

use crate::demo::gamevent::GameEventValue;
use crate::demo::header::Header;
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod state;
mod analyser;

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
    InvalidGameEvent { name: String, value: GameEventValue },
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
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        Self::read(stream).map_err(ParseError::from)
    }
}

pub struct DemoParser {
    stream: Stream,
    state: ParserState,
    analyser: Analyser,
}

impl DemoParser {
    pub fn new(stream: Stream) -> Self {
        DemoParser {
            state: ParserState::new(),
            stream,
            analyser: Analyser::new(),
        }
    }

    pub fn read<T: Parse>(&mut self) -> Result<T> {
        T::parse(&mut self.stream, &self.state)
    }

    pub fn stream_pos(&self) -> usize {
        self.stream.pos()
    }

    pub fn set_stream_pos(&mut self, pos: usize) -> Result<()> {
        self.stream.set_pos(pos)?;
        Ok(())
    }

    pub fn parse_demo(mut self) -> Result<(Header, Analyser)> {
        let header = self.read::<Header>()?;
        loop {
            let packet = self.read::<Packet>()?;
            let messages = match packet {
                Packet::Stop(_) => break,
                packet => self.state.handle_packet(packet),
            };
            for message in messages {
                self.analyser.handle_message(message, self.state.tick);
            }
        }
        Ok((header, self.analyser))
    }
}
