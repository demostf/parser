use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncTickPacket {
    pub tick: u32,
}
