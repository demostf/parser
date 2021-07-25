use bitbuffer::{bit_size_of, BitRead, BitWrite, BitWriteStream, Endianness, LittleEndian};
use serde::{Deserialize, Serialize};

use crate::demo::message::{Message, MessageType};
use crate::demo::parser::Encode;
use crate::demo::vector::Vector;
use crate::{Parse, ParserState, ReadResult, Result, Stream};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitRead, BitWrite, PartialEq, Serialize, Deserialize)]
pub struct MessagePacketMeta {
    pub flags: u32, // TODO
    pub view_angles: ViewAngles,
    pub sequence_in: u32,
    pub sequence_out: u32,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct MessagePacket<'a> {
    pub tick: u32,
    pub messages: Vec<Message<'a>>,
    pub meta: MessagePacketMeta,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ViewAngles {
    pub origin: (Vector, Vector),
    pub angles: (Vector, Vector),
    pub local_angles: (Vector, Vector),
}

impl<E: Endianness> BitRead<'_, E> for ViewAngles {
    fn read(stream: &mut bitbuffer::BitReadStream<E>) -> ReadResult<Self> {
        let vectors = <[Vector; 6]>::read(stream)?;
        let origin = (vectors[3], vectors[0]);
        let angles = (vectors[4], vectors[1]);
        let local_angles = (vectors[5], vectors[2]);
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

impl<E: Endianness> BitWrite<E> for ViewAngles {
    fn write(&self, stream: &mut bitbuffer::BitWriteStream<E>) -> ReadResult<()> {
        [
            self.origin.1,
            self.angles.1,
            self.local_angles.1,
            self.origin.0,
            self.angles.0,
            self.local_angles.0,
        ]
        .write(stream)
    }
}

#[test]
fn test_view_angles_roundtrip() {
    crate::test_roundtrip_write(ViewAngles {
        origin: (Vector::default(), Vector::default()),
        angles: (Vector::default(), Vector::default()),
        local_angles: (Vector::default(), Vector::default()),
    });
    crate::test_roundtrip_write(ViewAngles {
        origin: (
            Vector {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            Vector {
                x: 2.0,
                y: 2.0,
                z: 2.0,
            },
        ),
        angles: (
            Vector {
                x: 3.0,
                y: 3.0,
                z: 3.0,
            },
            Vector {
                x: 4.0,
                y: 4.0,
                z: 4.0,
            },
        ),
        local_angles: (
            Vector {
                x: 5.0,
                y: 5.0,
                z: 5.0,
            },
            Vector {
                x: 6.0,
                y: 6.0,
                z: 6.0,
            },
        ),
    });
}

impl<'a> Parse<'a> for MessagePacket<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;

        let meta = stream.read()?;

        let length: u32 = stream.read()?;
        let mut packet_data = stream.read_bits(length as usize * 8)?;

        let mut messages = Vec::with_capacity(8);
        while packet_data.bits_left() > 6 {
            let message_type = MessageType::read(&mut packet_data)?;

            if state.should_parse_message(message_type) && message_type != MessageType::Empty {
                messages.push(Message::from_type(message_type, &mut packet_data, state)?);
            } else {
                Message::skip_type(message_type, &mut packet_data, state)?;
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

impl Encode for MessagePacket<'_> {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        self.tick.write(stream)?;
        self.meta.write(stream)?;
        stream.reserve_byte_length(32, |stream| {
            for message in self.messages.iter() {
                message.get_message_type().write(stream)?;
                message.encode(stream, state)?;
            }

            Ok(())
        })
    }
}
