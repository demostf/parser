use std::collections::HashMap;

use bitstream_reader::{BitRead, BitReadSized, LittleEndian};

use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::{GameEvent, GameEventDefinition, GameEventEntry, GameEventValue, GameEventValueType, RawGameEvent};

fn read_event_value(stream: &mut Stream, definition: &GameEventEntry) -> Result<GameEventValue> {
    Ok(match definition.kind {
        GameEventValueType::String => GameEventValue::String(stream.read()?),
        GameEventValueType::Float => GameEventValue::Float(stream.read()?),
        GameEventValueType::Long => GameEventValue::Long(stream.read()?),
        GameEventValueType::Short => GameEventValue::Short(stream.read()?),
        GameEventValueType::Byte => GameEventValue::Byte(stream.read()?),
        GameEventValueType::Boolean => GameEventValue::Boolean(stream.read()?),
        GameEventValueType::Local => GameEventValue::Local,
    })
}

pub struct GameEventMessage {
    pub event: GameEvent
}

impl Parse for GameEventMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let length: u16 = stream.read_sized(11)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_type = data.read()?;
        let raw_event = match state.event_definitions.get(&event_type) {
            Some(definition) => {
                let mut values: HashMap<String, GameEventValue> = HashMap::with_capacity(definition.entries.len());
                for entry in &definition.entries {
                    values.insert(entry.name.clone(), read_event_value(stream, &entry)?);
                }

                RawGameEvent {
                    event_type,
                    values,
                }
            }
            None => unreachable!()
        };
        let event = GameEvent::from_raw_event(raw_event)?;
        Ok(GameEventMessage {
            event
        })
    }
}