use bitbuffer::{BitRead, BitWrite};

#[derive(Debug, BitRead, BitWrite, PartialEq)]
pub struct StopPacket {
    #[size = 24]
    pub tick: u32,
}
