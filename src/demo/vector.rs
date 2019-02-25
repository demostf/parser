use crate::{Parse, ParseError, ParserState, Result, Stream};

pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Parse<'_> for Vector {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Ok(Vector {
            x: stream.read_float()?,
            y: stream.read_float()?,
            z: stream.read_float()?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream.skip(32 * 3).map_err(ParseError::from)
    }
}

pub struct VectorXY {
    pub x: f32,
    pub y: f32,
}

impl Parse<'_> for VectorXY {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Ok(VectorXY {
            x: stream.read_float()?,
            y: stream.read_float()?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream.skip(32 * 2).map_err(ParseError::from)
    }
}