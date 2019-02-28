use crate::demo::vector::Vector;
use crate::{Parse, ParseError, ParserState, Result, Stream};

#[derive(Debug)]
pub struct Message;

#[derive(Debug)]
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

impl Parse for MessagePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read_int(32)?;
        let flags = stream.read_int(32)?;

        let view_origin_1 = Vector::parse(stream, state)?;
        let view_angle_1 = Vector::parse(stream, state)?;
        let local_view_angle_1 = Vector::parse(stream, state)?;
        let view_origin = (Vector::parse(stream, state)?, view_origin_1);
        let view_angles = (Vector::parse(stream, state)?, view_angle_1);
        let local_view_angles = (Vector::parse(stream, state)?, local_view_angle_1);

        let sequence_in = stream.read_int(32)?;
        let sequence_out = stream.read_int(32)?;
        let length: usize = stream.read_int(32)?;
        let mut packet_data = stream.read_bits(length * 8)?;

        let messages = vec![];

        let packet = MessagePacket {
            tick,
            messages,
            view_origin,
            view_angles,
            local_view_angles,
            sequence_in,
            sequence_out,
            flags,
        };
        Ok(packet)
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32 * 2)?;

        for i in 0..6 {
            Vector::skip(stream)?;
        }

        let _ = stream.skip(32 * 2)?;
        let length: usize = stream.read_int::<usize>(32)?;
        stream.skip(length * 8).map_err(ParseError::from)
    }
}
