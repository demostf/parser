use bitstream_reader::{BitRead, BitSize, LittleEndian, LazyBitRead};

use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};
use crate::demo::message::Message;
use crate::demo::vector::Vector;

#[derive(Debug)]
pub struct MessagePacket {
    pub tick: u32,
    pub messages: Vec<Message>,
    pub sequence_in: u32,
    pub sequence_out: u32,
    pub view_angles: LazyBitRead<ViewAngles, LittleEndian>,
    pub flags: u32, // TODO
}

#[derive(Clone, Debug)]
pub struct ViewAngles {
    pub origin: (Vector, Vector),
    pub angles: (Vector, Vector),
    pub local_angles: (Vector, Vector),
}

impl BitSize for ViewAngles {
    fn bit_size() -> usize {
        Vector::bit_size() * 6
    }
}

impl BitRead<LittleEndian> for ViewAngles {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let view_origin_1 = Vector::read(stream)?;
        let view_angle_1 = Vector::read(stream)?;
        let local_view_angle_1 = Vector::read(stream)?;
        let origin = (Vector::read(stream)?, view_origin_1);
        let angles = (Vector::read(stream)?, view_angle_1);
        let local_angles = (Vector::read(stream)?, local_view_angle_1);
        Ok(ViewAngles {
            origin,
            angles,
            local_angles,
        })
    }
}

impl Parse for MessagePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let flags = stream.read()?;

        let view_angles = stream.read()?;

        let sequence_in = stream.read()?;
        let sequence_out = stream.read()?;
        let length: u32 = stream.read()?;
        let mut packet_data = stream.read_bits(length as usize * 8)?;

        let mut messages: Vec<Message> = Vec::with_capacity(25);
        while packet_data.bits_left() > 6 {
            let message = Message::parse(&mut packet_data, state)?;
            match message {
                Message::Empty => {}
                _ => messages.push(message),
            }
        }

        let packet = MessagePacket {
            tick,
            messages,
            view_angles,
            sequence_in,
            sequence_out,
            flags,
        };
        Ok(packet)
    }
}
