


use bitbuffer::{BitRead, LittleEndian};
use parse_display::Display;

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::{
    GameEvent, GameEventDefinition, GameEventEntry, GameEventValue, GameEventValueType,
    RawGameEvent,
};
use crate::demo::handle_utf8_error;
use crate::demo::parser::ParseBitSkip;
use crate::{GameEventError, Parse, ParseError, ParserState, ReadResult, Result, Stream};

fn read_event_value(stream: &mut Stream, definition: &GameEventEntry) -> Result<GameEventValue> {
    Ok(match definition.kind {
        GameEventValueType::String => {
            GameEventValue::String(stream.read().or_else(handle_utf8_error)?)
        }
        GameEventValueType::Float => GameEventValue::Float(stream.read()?),
        GameEventValueType::Long => GameEventValue::Long(stream.read()?),
        GameEventValueType::Short => GameEventValue::Short(stream.read()?),
        GameEventValueType::Byte => GameEventValue::Byte(stream.read()?),
        GameEventValueType::Boolean => GameEventValue::Boolean(stream.read()?),
        GameEventValueType::Local => GameEventValue::Local,
        GameEventValueType::None => return Err(GameEventError::NoneValue.into()),
    })
}

#[derive(Debug)]
pub struct GameEventMessage {
    pub event: Box<GameEvent>,
}

impl Parse for GameEventMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let length: u16 = stream.read_sized(11)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_type: GameEventTypeId = data.read()?;

        // game event definitions haven't been sent yet, ignore
        if state.event_definitions.len() == 0 {
            return Ok(GameEventMessage {
                event: Box::new(GameEvent::Unknown(RawGameEvent {
                    event_type: GameEventType::Unknown,
                    values: Vec::new(),
                })),
            });
        }

        let raw_event = match state.event_definitions.get(usize::from(event_type)) {
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
            None => {
                return Err(ParseError::MalformedGameEvent(GameEventError::UnknownType(
                    event_type,
                )));
            }
        };
        let event = GameEvent::from_raw_event(raw_event)?;
        Ok(GameEventMessage {
            event: Box::new(event),
        })
    }
}

impl ParseBitSkip for GameEventMessage {
    fn parse_skip(stream: &mut Stream) -> Result<()> {
        let length: u16 = stream.read_sized(11)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

#[derive(BitRead, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
pub struct GameEventTypeId(#[size = 9] u16);

impl From<GameEventTypeId> for usize {
    fn from(id: GameEventTypeId) -> Self {
        id.0 as usize
    }
}

impl From<GameEventTypeId> for u16 {
    fn from(id: GameEventTypeId) -> Self {
        id.0
    }
}

#[derive(Debug)]
pub struct GameEventListMessage {
    pub event_list: Vec<GameEventDefinition>,
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
        let event_list: Vec<GameEventDefinition> = data.read_sized(count as usize)?;

        Ok(GameEventListMessage { event_list })
    }
}
