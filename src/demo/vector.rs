use bitstream_reader::{BitRead, BitSize, LittleEndian};
use serde::Serialize;

use crate::{ReadResult, Stream};

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize)]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}
