use bitbuffer::{BitRead, BitWrite};
use parse_display::Display;
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(BitRead, BitWrite, Debug, Clone, Copy, Default, Serialize, Deserialize, Display)]
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

#[cfg_attr(
    feature = "wasm",
    derive(wasm_typescript_definition::TypescriptDefinition)
)]
#[derive(BitRead, BitWrite, Debug, Clone, Copy, Default, Serialize, Deserialize, Display)]
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
