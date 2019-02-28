use bitstream_reader::{BitRead};

use crate::{Parse, ParseError, ParserState, Result, Stream};

#[derive(BitRead, Debug)]
pub struct SyncTickPacket {
    tick: u32,
}