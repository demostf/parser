use crate::{Parse, ParseError, ParserState, Result, Stream};
use crate::demo::vector::Vector;

pub struct Message;

pub struct MessagePacket {
    tick: u32,
    messages: Vec<Message>,
    view_origin: (Vector, Vector),
    view_angles: (Vector, Vector),
    local_view_angles: (Vector, Vector),
    sequence_in: u32,
    sequence_out: u32,
    flags: u32, // TODO
}

impl<'a> Parse<'a> for MessagePacket {
    fn parse(stream: &mut Stream<'a>, state: &ParserState<'a>) -> Result<Self> {
        let tick = stream.read(32)?;
        let flags = stream.read(32)?;

        let view_origin_1 = Vector::parse(stream, state)?;
        let view_angle_1 = Vector::parse(stream, state)?;
        let local_view_angle_1 = Vector::parse(stream, state)?;
        let view_origin = (Vector::parse(stream, state)?, view_origin_1);
        let view_angles = (Vector::parse(stream, state)?, view_angle_1);
        let local_view_angles = (Vector::parse(stream, state)?, local_view_angle_1);

        let sequence_in = stream.read(32)?;
        let sequence_out = stream.read(32)?;
        let length: usize = stream.read(32)?;
        let _ = stream.skip(length).map_err(ParseError::from);

        let mut messages = vec![];

        Ok(MessagePacket {
            tick,
            messages,
            view_origin,
            view_angles,
            local_view_angles,
            sequence_in,
            sequence_out,
            flags,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32 * 2)?;

        for i in 0..6 {
            Vector::skip(stream)?;
        }

        let _ = stream.skip(32 * 2)?;
        let length: usize = stream.read(32)?;
        stream.skip(length).map_err(ParseError::from)
    }
}