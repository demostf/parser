use bitstream_reader::{BitRead};

#[derive(BitRead, Debug)]
pub struct SyncTickPacket {
    tick: u32,
}