use bitstream_reader::BitRead;

#[derive(BitRead, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(BitRead, Debug)]
pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}