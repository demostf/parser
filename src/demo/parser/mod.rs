pub use crate::demo::parser::state::ParserState;
use crate::Stream;
use bitstream_reader::ReadError;

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
    InvalidSendPropArray,
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
}
