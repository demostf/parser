use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use parse_display::Display;

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::{
    GameEvent, GameEventDefinition, GameEventEntry, GameEventValueType, RawGameEvent,
};
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::{GameEventError, Parse, ParseError, ParserState, ReadResult, Result, Stream};

#[derive(Debug)]
pub struct GameEventMessage {
    pub event: GameEvent,
}

impl Parse<'_> for GameEventMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let length: u16 = stream.read_sized(11)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_type: GameEventTypeId = data.read()?;

        // game event definitions haven't been sent yet, ignore
        if state.event_definitions.is_empty() {
            return Ok(GameEventMessage {
                event: GameEvent::Unknown(RawGameEvent {
                    event_type: GameEventType::Unknown,
                    values: Vec::new(),
                }),
            });
        }

        let event = match state.event_definitions.get(usize::from(event_type)) {
            Some(definition) => GameEvent::read(&mut data, definition)?,
            None => {
                return Err(ParseError::MalformedGameEvent(GameEventError::UnknownType(
                    event_type,
                )));
            }
        };
        Ok(GameEventMessage { event })
    }
}

impl Encode for GameEventMessage {
    fn encode(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        _state: &ParserState,
    ) -> Result<()> {
        Ok(stream.reserve_length(11, |_stream| Ok(()))?)
    }
}

impl ParseBitSkip<'_> for GameEventMessage {
    fn parse_skip(stream: &mut Stream) -> Result<()> {
        let length: u16 = stream.read_sized(11)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

#[derive(BitRead, BitWrite, Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display)]
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

impl BitRead<'_, LittleEndian> for GameEventDefinition {
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

impl BitWrite<LittleEndian> for GameEventDefinition {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.id.write(stream)?;
        self.event_type.as_str().write(stream)?;
        self.name.write(stream)?;

        for entry in self.entries.iter() {
            entry.kind.write(stream)?;
            entry.name.write(stream)?;
        }
        GameEventValueType::None.write(stream)?;

        Ok(())
    }
}

impl BitRead<'_, LittleEndian> for GameEventListMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let count: u16 = stream.read_sized(9)?;
        let length: u32 = stream.read_sized(20)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_list: Vec<GameEventDefinition> = data.read_sized(count as usize)?;

        Ok(GameEventListMessage { event_list })
    }
}

impl BitWrite<LittleEndian> for GameEventListMessage {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        (self.event_list.len() as u16).write(stream)?;
        stream.reserve_length(20, |stream| {
            for event in self.event_list.iter() {
                event.write(stream)?;
            }
            Ok(())
        })?;

        Ok(())
    }
}
