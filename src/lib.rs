pub use bitbuffer::Result as ReadResult;

pub use crate::demo::{
    message::MessageType,
    parser::{
        DemoParser, GameEventError, MatchState, MessageTypeAnalyser, Parse, ParseError,
        ParserState, Result,
    },
    Demo, Stream,
};

pub(crate) mod consthash;
pub mod demo;
mod nullhasher;

#[cfg(test)]
#[track_caller]
fn test_roundtrip_encode<
    'a,
    T: bitbuffer::BitRead<'a, bitbuffer::LittleEndian>
        + bitbuffer::BitWrite<bitbuffer::LittleEndian>
        + std::fmt::Debug
        + std::cmp::PartialEq,
>(
    val: T,
) {
    use bitbuffer::{BitReadBuffer, BitReadStream, BitWriteStream, LittleEndian};
    let mut stream = BitWriteStream::new(LittleEndian);
    val.write(&mut stream).unwrap();
    let pos = stream.bit_len();

    let mut read = BitReadStream::new(BitReadBuffer::new_owned(stream.finish(), LittleEndian));
    assert_eq!(val, read.read().unwrap());
    assert_eq!(pos, read.pos());
}
