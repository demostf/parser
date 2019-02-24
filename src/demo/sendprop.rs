use super::vector::{Vector, VectorXY};

pub struct SendPropDefinition {}

pub enum SendPropType {
    Vector(Vector),
    VectorXY(VectorXY),
    Integer(i32),
    Float(f32),
    String(String),
    Array(Vec<SendPropType>),
}

pub struct SendProp {
    definition: SendPropDefinition,
    value: SendPropType,
}
