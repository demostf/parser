use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SyncTickPacket {
    pub tick: u32,
}
