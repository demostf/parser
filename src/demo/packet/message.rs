use bitbuffer::{bit_size_of, BitRead, Endianness, LazyBitRead, LittleEndian};

use crate::demo::message::{Message, MessageType};
use crate::demo::vector::Vector;
use crate::{Parse, ParserState, ReadResult, Result, Stream};
use std::fmt;

#[derive(Debug, BitRead)]
pub struct MessagePacketMeta {
    pub flags: u32, // TODO
    pub view_angles: ViewAngles,
    pub sequence_in: u32,
    pub sequence_out: u32,
}

#[derive(Debug)]
pub struct MessagePacket<'a> {
    pub tick: u32,
    pub messages: MessageIterator<'a>,
    pub meta: LazyBitRead<'a, MessagePacketMeta, LittleEndian>,
}

#[derive(Clone, Debug)]
pub struct ViewAngles {
    pub origin: (Vector, Vector),
    pub angles: (Vector, Vector),
    pub local_angles: (Vector, Vector),
}

impl<E: Endianness> BitRead<'_, E> for ViewAngles {
    fn read(stream: &mut bitbuffer::BitReadStream<E>) -> ReadResult<Self> {
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

    fn bit_size() -> Option<usize> {
        Some(bit_size_of::<Vector>().unwrap() * 6)
    }
}

impl<'a> Parse<'a> for MessagePacket<'a> {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;

        let meta = stream.read()?;

        let length: u32 = stream.read()?;
        let packet_data = stream.read_bits(length as usize * 8)?;

        let messages = MessageIterator::new(packet_data);

        let packet = MessagePacket {
            tick,
            messages,
            meta,
        };
        Ok(packet)
    }
}

pub struct MessageIterator<'a> {
    packet_data: Stream<'a>,
}

impl fmt::Debug for MessageIterator<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MessageIterator {{}}")
    }
}

impl<'a> MessageIterator<'a> {
    fn new(packet_data: Stream<'a>) -> Self {
        MessageIterator { packet_data }
    }

    pub fn next(&mut self, state: &ParserState) -> Option<Result<Message<'a>>> {
        while self.packet_data.bits_left() > 6 {
            let message_type = match MessageType::parse(&mut self.packet_data, state) {
                Ok(message_type) => message_type,
                Err(e) => return Some(Err(e)),
            };

            if state.should_parse_message(message_type) {
                return Some(Message::from_type(
                    message_type,
                    &mut self.packet_data,
                    state,
                ));
            } else {
                match Message::skip_type(message_type, &mut self.packet_data) {
                    Ok(_) => (),
                    Err(e) => return Some(Err(e)),
                }
            }
        }
        None
    }
}
