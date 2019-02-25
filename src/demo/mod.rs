use bitstream_reader::{BitBuffer, BitStream, LittleEndian, NonPadded};
use rentals::OwnedBuffer;

pub mod gamevent;
pub mod header;
pub mod packet;
pub mod parser;
pub mod sendprop;
pub mod vector;

pub type Buffer<'a> = BitBuffer<'a, LittleEndian, NonPadded>;
pub type Stream<'a> = BitStream<'a, LittleEndian, NonPadded>;

rental! {
    mod rentals {
        use super::*;

        #[rental_mut(covariant)]
        pub struct OwnedBuffer {
            source_vec: Vec<u8>,
            stream: Buffer<'source_vec>,
        }
    }
}

pub struct Demo {
    data: OwnedBuffer,
}

impl Demo {
    pub fn new(vec: Vec<u8>) -> Self {
        let data = OwnedBuffer::new(vec, |bytes| BitBuffer::new(bytes, LittleEndian));
        Demo { data }
    }

    /// Get a new stream with the data of the demo
    pub fn get_stream(&self) -> Stream {
        BitStream::new(self.data.suffix().clone(), None, None)
    }
}
