use bitbuffer::{BitRead, BitWrite};

#[derive(Debug, BitRead, BitWrite)]
pub struct StopPacket;
