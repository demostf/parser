use bitstream_reader::BitRead;
use parse_display::Display;
use serde::{Deserialize, Serialize};

#[derive(BitRead, Debug, Clone, Copy, Default, Serialize, Deserialize, Display)]
#[display("({x}, {y}, {z})")]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x < 0.001) && (self.y - other.y < 0.001) && (self.z - other.z < 0.001)
    }
}

#[derive(BitRead, Debug, Clone, Copy, Default, Serialize, Deserialize, Display)]
#[display("({x}, {y})")]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}

impl PartialEq for VectorXY {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x < 0.001) && (self.y - other.y < 0.001)
    }
}

impl From<Vector> for VectorXY {
    fn from(vec: Vector) -> Self {
        VectorXY { x: vec.x, y: vec.y }
    }
}
