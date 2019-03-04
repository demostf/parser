use std::collections::HashMap;
use std::iter::FromIterator;

use bitstream_reader::{BitRead, BitReadSized, LittleEndian};

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::{
    GameEvent, GameEventDefinition, GameEventEntry, GameEventValue, GameEventValueType,
    RawGameEvent,
};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};

fn read_event_value(stream: &mut Stream, definition: &GameEventEntry) -> Result<GameEventValue> {
    Ok(match definition.kind {
        GameEventValueType::String => GameEventValue::String(stream.read()?),
        GameEventValueType::Float => GameEventValue::Float(stream.read()?),
        GameEventValueType::Long => GameEventValue::Long(stream.read()?),
        GameEventValueType::Short => GameEventValue::Short(stream.read()?),
        GameEventValueType::Byte => GameEventValue::Byte(stream.read()?),
        GameEventValueType::Boolean => GameEventValue::Boolean(stream.read()?),
        GameEventValueType::Local => GameEventValue::Local,
        GameEventValueType::None => unreachable!(),
    })
}

#[derive(Debug)]
pub struct GameEventMessage {
    pub event: GameEvent,
}

impl Parse for GameEventMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let length: u16 = stream.read_sized(11)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_type = data.read()?;
        let raw_event = match state.event_definitions.get(&event_type) {
            Some(definition) => {
                let mut values: Vec<GameEventValue> = Vec::with_capacity(definition.entries.len());
                for entry in &definition.entries {
                    values.push(read_event_value(&mut data, &entry)?);
                }

                RawGameEvent {
                    event_type: definition.event_type,
                    values,
                }
            }
            None => unreachable!(),
        };
        let event = GameEvent::from_raw_event(raw_event)?;
        Ok(GameEventMessage { event })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameEventTypeId(u16);

impl BitRead<LittleEndian> for GameEventTypeId {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        Ok(GameEventTypeId(stream.read_int(9)?))
    }
}

#[derive(Debug)]
pub struct GameEventListMessage {
    pub event_list: HashMap<GameEventTypeId, GameEventDefinition>,
}

impl BitRead<LittleEndian> for GameEventDefinition {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let event_type: GameEventTypeId = stream.read()?;
        let name: String = stream.read()?;
        let mut entries = Vec::new();

        let mut entry_type = stream.read()?;
        while entry_type != GameEventValueType::None {
            let entry_name = stream.read()?;
            entries.push(GameEventEntry {
                name: entry_name,
                kind: entry_type,
            });
            entry_type = stream.read()?;
        }

        Ok(GameEventDefinition {
            id: event_type,
            event_type: GameEventType::from_type_name(name.as_str()),
            name,
            entries,
        })
    }
}

impl BitRead<LittleEndian> for GameEventListMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let count: u16 = stream.read_sized(9)?;
        let length: u32 = stream.read_sized(20)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_list_vec: Vec<GameEventDefinition> = data.read_sized(count as usize)?;
        let event_list = HashMap::from_iter(event_list_vec.into_iter().map(|def| (def.id, def)));

        Ok(GameEventListMessage { event_list })
    }
}
