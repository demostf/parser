use bitbuffer::{BitRead, BitWrite};

#[derive(BitRead, BitWrite, Debug, PartialEq)]
pub struct SyncTickPacket {
    pub tick: u32,
}
