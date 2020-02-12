use crate::ReadResult;
use bitbuffer::{BitReadBuffer, BitReadStream, LittleEndian, ReadError};

pub mod gameevent_gen;
pub mod gamevent;
pub mod header;
pub mod message;
pub mod packet;
pub mod parser;
pub mod sendprop;
pub mod vector;

pub type Buffer = BitReadBuffer<LittleEndian>;
pub type Stream = BitReadStream<LittleEndian>;

pub struct Demo {
    stream: Stream,
}

impl Demo {
    pub fn new(vec: Vec<u8>) -> Self {
        let data = Buffer::new(vec, LittleEndian);
        let stream = Stream::new(data);
        Demo { stream }
    }

    /// Get a new stream with the data of the demo
    pub fn get_stream(&self) -> Stream {
        self.stream.clone()
    }
}

pub(crate) fn handle_utf8_error(error: ReadError) -> ReadResult<String> {
    match error {
        ReadError::Utf8Error(_) => Ok("-- Malformed utf8 --".into()),
        _ => Err(error),
    }
}
