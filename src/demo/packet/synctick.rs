use bitbuffer::{BitRead, BitWrite};

#[derive(BitRead, BitWrite, Debug)]
pub struct SyncTickPacket {
    pub tick: u32,
}
