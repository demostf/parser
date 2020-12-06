use crate::ReadResult;
use bitbuffer::{BitError, BitReadBuffer, BitReadStream, LittleEndian};

pub mod gameevent_gen;
pub mod gamevent;
pub mod header;
pub mod message;
pub mod packet;
pub mod parser;
pub mod sendprop;
pub mod vector;

pub type Buffer<'a> = BitReadBuffer<'a, LittleEndian>;
pub type Stream<'a> = BitReadStream<'a, LittleEndian>;

pub struct Demo<'a> {
    stream: Stream<'a>,
}

impl<'a> Demo<'a> {
    pub fn new(byes: &'a [u8]) -> Self {
        let data = Buffer::new(byes, LittleEndian);
        let stream = Stream::new(data);
        Demo { stream }
    }

    /// Get a new stream with the data of the demo
    pub fn get_stream(&self) -> Stream<'a> {
        self.stream.clone()
    }
}

pub(crate) fn handle_utf8_error(error: BitError) -> ReadResult<String> {
    match error {
        BitError::Utf8Error(_, _) => Ok("-- Malformed utf8 --".into()),
        _ => Err(error),
    }
}
