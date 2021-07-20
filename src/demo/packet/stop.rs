use bitbuffer::{BitRead, BitWrite};

#[derive(Debug, BitRead, BitWrite, PartialEq)]
pub struct StopPacket;
