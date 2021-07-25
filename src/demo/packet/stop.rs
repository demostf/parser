use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitRead, BitWrite, PartialEq, Serialize, Deserialize)]
pub struct StopPacket {
    #[size = 24]
    pub tick: u32,
}
