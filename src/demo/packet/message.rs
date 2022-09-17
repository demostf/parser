use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

use crate::demo::data::DemoTick;
use crate::demo::message::{Message, MessageType};
use crate::demo::parser::Encode;
use crate::demo::vector::Vector;
use crate::{Parse, ParserState, Result, Stream};
#[cfg(feature = "trace")]
use tracing::{event, span, Level};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitRead, BitWrite, PartialEq, Serialize, Deserialize, Clone, Default)]
pub struct MessagePacketMeta {
    pub flags: u32, // TODO
    pub view_angles: [ViewAngles; 2],
    pub sequence_in: u32,
    pub sequence_out: u32,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Default)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct MessagePacket<'a> {
    pub tick: DemoTick,
    pub messages: Vec<Message<'a>>,
    pub meta: MessagePacketMeta,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize, BitRead, BitWrite)]
pub struct ViewAngles {
    pub origin: Vector,
    pub angles: Vector,
    pub local_angles: Vector,
}

#[test]
fn test_view_angles_roundtrip() {
    crate::test_roundtrip_write(ViewAngles::default());
    crate::test_roundtrip_write(ViewAngles {
        origin: Vector {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        angles: Vector {
            x: 4.0,
            y: 5.0,
            z: 6.0,
        },
        local_angles: Vector {
            x: 7.0,
            y: 8.0,
            z: 9.0,
        },
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
            #[cfg(feature = "trace")]
            let _span =
                span!(Level::DEBUG, "reading message", message_type = ?message_type, tick = tick)
                    .entered();

            if state.should_parse_message(message_type) && message_type != MessageType::Empty {
                #[cfg(feature = "trace")]
                event!(Level::TRACE, "parsing message");
                messages.push(Message::from_type(message_type, &mut packet_data, state)?);
            } else {
                #[cfg(feature = "trace")]
                event!(Level::TRACE, "skipping message");
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
