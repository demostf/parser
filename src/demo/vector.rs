use bitstream_reader::{BitRead, BitSize};
use serde::{Deserialize, Serialize};

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}
