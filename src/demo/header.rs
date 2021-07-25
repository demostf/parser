use bitbuffer::{BitRead, BitWrite};
use serde::{Deserialize, Serialize};

#[derive(BitRead, BitWrite, Debug, PartialEq, Serialize, Deserialize)]
pub struct Header {
    #[size = 8]
    pub demo_type: String,
    pub version: u32,
    pub protocol: u32,
    #[size = 260]
    pub server: String,
    #[size = 260]
    pub nick: String,
    #[size = 260]
    pub map: String,
    #[size = 260]
    pub game: String,
    pub duration: f32,
    pub ticks: u32,
    pub frames: u32,
    pub sigon: u32,
}
