use bitstream_reader::{BitRead, BitSize, Endianness, LazyBitRead, LittleEndian};

use crate::demo::message::{Message, MessageType};
use crate::demo::vector::Vector;
use crate::{Parse, ParserState, ReadResult, Result, Stream};

#[derive(Debug, BitRead, BitSize)]
pub struct MessagePacketMeta {
    pub flags: u32, // TODO
    pub view_angles: ViewAngles,
    pub sequence_in: u32,
    pub sequence_out: u32,
}

#[derive(Debug)]
pub struct MessagePacket {
    pub tick: u32,
    pub messages: Vec<Message>,
    pub meta: LazyBitRead<MessagePacketMeta, LittleEndian>,
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

impl<E: Endianness> BitRead<E> for ViewAngles {
    fn read(stream: &mut bitstream_reader::BitStream<E>) -> ReadResult<Self> {
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

        let meta = stream.read()?;

        let length: u32 = stream.read()?;
        let mut packet_data = stream.read_bits(length as usize * 8)?;

        let mut messages: Vec<Message> = Vec::with_capacity(10);
        while packet_data.bits_left() > 6 {
            let message_type = MessageType::parse(&mut packet_data, state)?;

            if state.should_parse_message(message_type) {
                let message = Message::from_type(message_type, &mut packet_data, state)?;
                messages.push(message);
            } else {
                let _ = Message::skip_type(message_type, &mut packet_data)?;
            }
        }

        let packet = MessagePacket {
            tick,
            messages,
            meta,
        };
        Ok(packet)
    }
}
