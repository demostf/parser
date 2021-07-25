use crate::Stream;
/// Messages that consists only of primitives and string and can be derived
use bitbuffer::{BitRead, BitWrite, LittleEndian};
use serde::{Deserialize, Serialize};

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct FileMessage {
    pub transfer_id: u32,
    pub file_name: String,
    pub requested: bool,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetTickMessage {
    pub tick: u32,
    pub frame_time: u16,
    pub std_dev: u16,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct StringCmdMessage {
    pub command: String,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct SignOnStateMessage {
    pub state: u8,
    pub count: u32,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct PrintMessage {
    pub value: String,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct ServerInfoMessage {
    pub version: u16,
    pub server_count: u32,
    pub stv: bool,
    pub dedicated: bool,
    pub max_crc: u32,
    pub max_classes: u16,
    pub map_hash: [u8; 16],
    pub player_count: u8,
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

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetPauseMessage {
    pub pause: bool,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetViewMessage {
    #[size = 11]
    pub index: u16,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixAngleMessage {
    pub relative: bool,
    pub x: u16,
    pub y: u16,
    pub z: u16,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
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

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreFetchMessage {
    #[size = 14]
    pub index: u16,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct MenuMessage<'a> {
    pub kind: u16,
    pub length: u16,
    #[size = "length.saturating_mul(8)"]
    pub index: Stream<'a>,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCvarValueMessage {
    pub cookie: u32,
    pub value: String,
}

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct CmdKeyValuesMessage<'a> {
    pub length: u32,
    #[size = "length.saturating_mul(8)"]
    pub data: Stream<'a>,
}
