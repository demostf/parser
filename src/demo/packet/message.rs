use crate::{Parse, ParseError, ParserState, Result, Stream};
use crate::demo::message::Message;
use crate::demo::vector::Vector;

#[derive(Debug)]
pub struct MessagePacket {
    pub tick: u32,
    pub messages: Vec<Message>,
    pub view_origin: (Vector, Vector),
    pub view_angles: (Vector, Vector),
    pub local_view_angles: (Vector, Vector),
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub flags: u32, // TODO
}

impl Parse for MessagePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let flags = stream.read()?;

        let view_origin_1 = Vector::parse(stream, state)?;
        let view_angle_1 = Vector::parse(stream, state)?;
        let local_view_angle_1 = Vector::parse(stream, state)?;
        let view_origin = (Vector::parse(stream, state)?, view_origin_1);
        let view_angles = (Vector::parse(stream, state)?, view_angle_1);
        let local_view_angles = (Vector::parse(stream, state)?, local_view_angle_1);

        let sequence_in = stream.read()?;
        let sequence_out = stream.read()?;
        let length: u32 = stream.read()?;
        let mut packet_data = stream.read_bits(length as usize * 8)?;

        let mut messages: Vec<Message> = Vec::new();
        while packet_data.bits_left() > 6 {
            let message = Message::parse(&mut packet_data, state)?;
            match message {
                Message::Empty => {}
                _ => messages.push(message)
            }
        }

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
}
