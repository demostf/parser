use bitstream_reader::{BitRead, BitSize};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

#[derive(BitRead, BitSize, Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq)]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}

impl fmt::Display for VectorXY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
