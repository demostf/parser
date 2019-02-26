use std::error::Error;

use bitstream_reader::ReadError;

use crate::demo::header::Header;
use crate::demo::packet::Packet;
pub use crate::demo::parser::state::ParserState;
use crate::Stream;

mod state;

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    /// Error while reading bits from stream
    ReadError(ReadError),
    /// Packet identifier is invalid
    InvalidPacketType(u8),
    /// SendProp type is invalid
    InvalidSendPropType(u8),
    /// Invalid structure found while creating array SendProp
    InvalidSendPropArray(String),
    /// Expected amount of data left after parsing an object
    DataRemaining(usize),
}

impl From<ReadError> for ParseError {
    fn from(err: ReadError) -> ParseError {
        ParseError::ReadError(err)
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;

pub trait Parse<'a>: Sized {
    fn parse(stream: &mut Stream<'a>, state: &ParserState<'a>) -> Result<Self>;
    fn skip(stream: &mut Stream) -> Result<()>;
}

pub struct DemoParser<'a> {
    stream: Stream<'a>,
    state: ParserState<'a>,
}

impl<'a> DemoParser<'a> {
    pub fn new(stream: Stream<'a>) -> Self {
        DemoParser {
            state: ParserState::new(&stream),
            stream,
        }
    }

    pub fn read<T: Parse<'a>>(&mut self) -> Result<T> {
        T::parse(&mut self.stream, &self.state)
    }

    pub fn skip<T: Parse<'a>>(&mut self) -> Result<()> {
        T::skip(&mut self.stream)
    }

    pub fn stream_pos(&self) -> usize {
        self.stream.pos()
    }

    pub fn set_stream_pos(&mut self, pos: usize) -> Result<()> {
        self.stream.set_pos(pos)?;
        Ok(())
    }

    pub fn split_packets(mut self) -> Result<Vec<Stream<'a>>> {
        let _ = self.skip::<Header>()?;
        let mut streams = vec![];
        while self.stream.bits_left() > 7 {
            let start = self.stream.pos();
            let _ = self.skip::<Packet>()?;
            let end = self.stream.pos();
            let _ = self.stream.set_pos(start);
            streams.push(self.stream.read_bits(end - start)?);
        }
        Ok(streams)
    }

    pub fn parse_demo(mut self) -> Result<(Header, Vec<Packet<'a>>)> {
        let header = self.read::<Header>()?;
        let mut packets = vec![];
        loop {
            let packet = self.read::<Packet>()?;
            match packet {
                Packet::Stop(_) => break,
                packet => packets.push(packet)
            }
        }
        Ok((header, packets))
    }
}
