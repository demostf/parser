use bitstream_reader::{BitRead, LittleEndian};
use serde::Serialize;

use crate::{ReadResult, Stream};

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl BitRead<LittleEndian> for Vector {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        Ok(Vector {
            x: stream.read()?,
            y: stream.read()?,
            z: stream.read()?,
        })
    }
}

#[derive(BitRead, Debug, Clone, Copy, Default, Serialize)]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}
