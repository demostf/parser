use bitstream_reader::{BitBuffer, BitStream, LittleEndian};

pub mod gamevent;
pub mod header;
pub mod message;
pub mod packet;
pub mod parser;
pub mod sendprop;
pub mod vector;

pub type Buffer = BitBuffer<LittleEndian>;
pub type Stream = BitStream<LittleEndian>;

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
