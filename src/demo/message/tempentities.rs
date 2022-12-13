use super::stringtable::read_var_int;
use crate::demo::message::packetentities::PacketEntitiesMessage;
use crate::demo::message::stringtable::{encode_var_int_fixed, log_base2};
use crate::demo::packet::datatable::ClassId;
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::demo::sendprop::SendProp;
use crate::Result;
use crate::{Parse, ParseError, ParserState, Stream};
use bitbuffer::{BitWrite, BitWriteSized, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TempEntitiesMessage {
    pub events: Vec<EventInfo>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EventInfo {
    pub class_id: ClassId,
    pub fire_delay: f32,
    pub reliable: bool,
    pub props: Vec<SendProp>,
}

impl Parse<'_> for TempEntitiesMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let count: u8 = stream.read()?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(17)?
        };
        let data = stream.read_bits(length as usize)?;
        let mut stream = data.clone();
        let stream = &mut stream;

        let (count, reliable) = if count == 0 {
            (1, true)
        } else {
            (count, false)
        };

        let mut events: Vec<EventInfo> = Vec::with_capacity(count as usize);

        for _ in 0..count {
            let delay = if stream.read()? {
                let raw: u8 = stream.read()?;
                raw as f32 / 100.0
            } else {
                0.0
            };

            let class_id = if stream.read()? {
                let bits = log_base2(state.server_classes.len()) + 1;
                (stream.read_sized::<u16>(bits as usize)?.saturating_sub(1)).into()
            } else {
                let last = events.last().ok_or(ParseError::InvalidDemo(
                    "temp entity update without previous",
                ))?;

                last.class_id
            };
            let send_table = state
                .send_tables
                .get(usize::from(class_id))
                .ok_or(ParseError::UnknownServerClass(class_id))?;

            let mut props = Vec::new();
            PacketEntitiesMessage::read_update(stream, send_table, &mut props, 0u32.into())?;

            events.push(EventInfo {
                class_id,
                fire_delay: delay,
                reliable,
                props,
            });
        }

        Ok(TempEntitiesMessage { events })
    }
}

impl ParseBitSkip<'_> for TempEntitiesMessage {
    fn parse_skip(stream: &mut Stream, state: &ParserState) -> Result<()> {
        let _: u8 = stream.read()?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(17)?
        };
        stream.skip_bits(length as usize)?;
        Ok(())
    }
}

impl Encode for TempEntitiesMessage {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        let count = if self.events.len() == 1 {
            if self.events[0].reliable {
                0
            } else {
                1
            }
        } else {
            self.events.len() as u8
        };
        count.write(stream)?;

        stream.reserve_int::<ParseError, _>(40, |stream| {
            let start = stream.bit_len();
            let mut last_class_id = u16::MAX.into();

            for event in self.events.iter() {
                if event.fire_delay > 0.0 {
                    true.write(stream)?;
                    ((event.fire_delay * 100.0) as u8).write(stream)?;
                } else {
                    false.write(stream)?;
                }

                if event.class_id != last_class_id {
                    true.write(stream)?;
                    let bits = log_base2(state.server_classes.len()) + 1;
                    let id: u16 = event.class_id.into();
                    (id + 1).write_sized(stream, bits as usize)?;
                } else {
                    false.write(stream)?;
                }
                last_class_id = event.class_id;

                let send_table = state
                    .send_tables
                    .get(usize::from(event.class_id))
                    .ok_or(ParseError::UnknownServerClass(event.class_id))?;
                PacketEntitiesMessage::write_update(&event.props, stream, send_table, 0u32.into())?;
            }
            let end = stream.bit_len();
            Ok(encode_var_int_fixed((end - start) as u32))
        })?;
        Ok(())
    }
}
