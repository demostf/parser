use crate::demo::sendprop::SendPropValue;
use bitbuffer::{BitRead, BitWrite};
use parse_display::Display;
use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, Copy, Default, Serialize, Deserialize, Display)]
#[display("({x}, {y}, {z})")]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<Vector> for [f32; 3] {
    fn from(vec: Vector) -> Self {
        [vec.x, vec.y, vec.z]
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x < 0.001) && (self.y - other.y < 0.001) && (self.z - other.z < 0.001)
    }
}

impl PartialEq<[SendPropValue]> for Vector {
    fn eq(&self, other: &[SendPropValue]) -> bool {
        match other {
            &[SendPropValue::Float(x), SendPropValue::Float(y), SendPropValue::Float(z)] => {
                self.x == x && self.y == y && self.z == z
            }
            _ => false,
        }
    }
}

#[test]
fn test_vec_array_eq() {
    assert!(!Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }
    .eq([SendPropValue::Float(1.0), SendPropValue::Float(2.0)].as_slice()));

    assert!(Vector {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    }
    .eq([
        SendPropValue::Float(1.0),
        SendPropValue::Float(2.0),
        SendPropValue::Float(3.0),
    ]
    .as_slice()));
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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

impl PartialEq<[SendPropValue]> for VectorXY {
    fn eq(&self, other: &[SendPropValue]) -> bool {
        match other {
            &[SendPropValue::Float(x), SendPropValue::Float(y)] => self.x == x && self.y == y,
            _ => false,
        }
    }
}

#[test]
fn test_vec_xy_array_eq() {
    assert!(VectorXY { x: 1.0, y: 2.0 }
        .eq([SendPropValue::Float(1.0), SendPropValue::Float(2.0)].as_slice()));

    assert!(!VectorXY { x: 1.0, y: 2.0 }.eq([
        SendPropValue::Float(1.0),
        SendPropValue::Float(2.0),
        SendPropValue::Float(3.0)
    ]
    .as_slice()));
}

impl From<Vector> for VectorXY {
    fn from(vec: Vector) -> Self {
        VectorXY { x: vec.x, y: vec.y }
    }
}

impl Add for VectorXY {
    type Output = VectorXY;

    fn add(self, rhs: Self) -> Self::Output {
        VectorXY {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for VectorXY {
    type Output = VectorXY;

    fn sub(self, rhs: Self) -> Self::Output {
        VectorXY {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
