/// Messages that consists only of primitives and string and can be derived
use crate::demo::data::{MaybeUtf8String, ServerTick};
use crate::Stream;
use bitbuffer::{BitRead, BitWrite, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FileMessage {
    pub transfer_id: u32,
    pub file_name: String,
    pub requested: bool,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NetTickMessage {
    pub tick: ServerTick,
    pub frame_time: u16,
    pub std_dev: u16,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StringCmdMessage {
    pub command: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[discriminant_bits = 8]
pub enum SignOnState {
    None = 0,
    Challenge,
    Connected,
    New,
    PreSpawn,
    Spawn,
    Full,
    ChangeLevel,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SignOnStateMessage {
    pub state: SignOnState,
    pub count: u32,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PrintMessage {
    pub value: MaybeUtf8String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerInfoMessage {
    pub version: u16,
    pub server_count: u32,
    pub stv: bool,
    pub dedicated: bool,
    pub max_crc: u32,
    pub max_classes: u16,
    pub map_hash: [u8; 16],
    pub player_slot: u8,
    pub max_player_count: u8,
    pub interval_per_tick: f32,
    #[size = 1]
    pub platform: String,
    pub game: String,
    pub map: String,
    pub skybox: String,
    pub server_name: String,
    pub replay: bool,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SetPauseMessage {
    pub pause: bool,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SetViewMessage {
    #[size = 11]
    pub index: u16,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FixAngleMessage {
    pub relative: bool,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct EntityMessage<'a> {
    #[size = 11]
    pub index: u16,
    #[size = 9]
    pub class_id: u16,
    #[size = 11]
    pub length: u16,
    #[size = "length"]
    pub data: Stream<'a>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PreFetchMessage {
    #[size = 14]
    pub index: u16,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct MenuMessage<'a> {
    pub kind: u16,
    pub length: u16,
    #[size = "length.saturating_mul(8)"]
    pub index: Stream<'a>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GetCvarValueMessage {
    pub cookie: u32,
    pub value: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize, Clone)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct CmdKeyValuesMessage<'a> {
    pub length: u32,
    #[size = "length.saturating_mul(8)"]
    pub data: Stream<'a>,
}
