use bitbuffer::{BitReadBuffer, BitReadStream, LittleEndian};

pub mod data;
pub mod gameevent_gen;
pub mod gamevent;
pub mod header;
pub mod lzss;
pub mod message;
pub mod packet;
pub mod parser;
pub mod sendprop;
mod sendprop_gen;
pub mod vector;

pub type Buffer<'a> = BitReadBuffer<'a, LittleEndian>;
pub type Stream<'a> = BitReadStream<'a, LittleEndian>;

pub struct Demo<'a> {
    stream: Stream<'a>,
}

impl<'a> Demo<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        let data = Buffer::new(bytes, LittleEndian);
        let stream = Stream::new(data);
        Demo { stream }
    }

    /// Get a new stream with the data of the demo
    pub fn get_stream(&self) -> Stream<'a> {
        self.stream.clone()
    }
}

impl Demo<'static> {
    pub fn owned(bytes: Vec<u8>) -> Self {
        let data = Buffer::new_owned(bytes, LittleEndian);
        let stream = Stream::new(data);
        Demo { stream }
    }
}
