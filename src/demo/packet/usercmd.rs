use crate::Stream;
use bitbuffer::{BitRead, BitWrite, LittleEndian};

#[derive(Debug, BitRead, BitWrite)]
#[endianness = "LittleEndian"]
pub struct UserCmdPacket<'a> {
    tick: u32,
    sequence_out: u32,
    len: u32,
    #[size = "len.saturating_mul(8)"]
    data: Stream<'a>,
}
