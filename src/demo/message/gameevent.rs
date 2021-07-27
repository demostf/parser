use bitbuffer::{BitRead, BitWrite, BitWriteSized, BitWriteStream, LittleEndian};
use parse_display::Display;
use serde::{Deserialize, Serialize};

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::{
    GameEvent, GameEventDefinition, GameEventEntry, GameEventValueType, RawGameEvent,
};
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::{GameEventError, Parse, ParseError, ParserState, ReadResult, Result, Stream};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameEventMessage {
    pub event_type_id: GameEventTypeId,
    pub event: GameEvent,
}

impl Parse<'_> for GameEventMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let length: u16 = stream.read_sized(11)?;
        let mut data = stream.read_bits(length as usize)?;
        let event_type_id: GameEventTypeId = data.read()?;

        // game event definitions haven't been sent yet, ignore
        if state.event_definitions.is_empty() {
            return Ok(GameEventMessage {
                event_type_id,
                event: GameEvent::Unknown(RawGameEvent {
                    event_type: GameEventType::Unknown(String::new()),
                    values: Vec::new(),
                }),
            });
        }

        let event = match state.event_definitions.get(usize::from(event_type_id)) {
            Some(definition) => GameEvent::read(&mut data, definition)?,
            None => {
                return Err(ParseError::MalformedGameEvent(GameEventError::UnknownType(
                    event_type_id,
                )));
            }
        };
        Ok(GameEventMessage {
            event_type_id,
            event,
        })
    }
}

impl Encode for GameEventMessage {
    fn encode(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        _state: &ParserState,
    ) -> Result<()> {
        Ok(stream.reserve_length(11, |stream| {
            self.event_type_id.write(stream)?;
            self.event.write(stream)
        })?)
    }
}

#[test]
fn test_game_event_roundtrip() {
    use crate::demo::gameevent_gen::{GameInitEvent, ServerShutdownEvent};

    let definitions = vec![
        GameEventDefinition {
            id: GameEventTypeId(0),
            event_type: GameEventType::ServerShutdown,
            entries: vec![GameEventEntry {
                name: "reason".to_string(),
                kind: GameEventValueType::String,
            }],
        },
        GameEventDefinition {
            id: GameEventTypeId(1),
            event_type: GameEventType::ServerChangeLevelFailed,
            entries: vec![GameEventEntry {
                name: "level_name".to_string(),
                kind: GameEventValueType::String,
            }],
        },
        GameEventDefinition {
            id: GameEventTypeId(2),
            event_type: GameEventType::GameInit,
            entries: vec![],
        },
    ];
    let mut state = ParserState::new(24, |_| false, false);
    state.event_definitions = definitions;

    crate::test_roundtrip_encode(
        GameEventMessage {
            event_type_id: GameEventTypeId(0),
            event: GameEvent::ServerShutdown(ServerShutdownEvent {
                reason: "asd".into(),
            }),
        },
        &state,
    );
    crate::test_roundtrip_encode(
        GameEventMessage {
            event_type_id: GameEventTypeId(2),
            event: GameEvent::GameInit(GameInitEvent {}),
        },
        &state,
    );
}

impl ParseBitSkip<'_> for GameEventMessage {
    fn parse_skip(stream: &mut Stream, _state: &ParserState) -> Result<()> {
        let length: u16 = stream.read_sized(11)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitRead,
    BitWrite,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    Serialize,
    Deserialize,
)]
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
            entries,
        })
    }
}

impl BitWrite<LittleEndian> for GameEventDefinition {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.id.write(stream)?;
        // if self.event_type == GameEventType::Unknown {
        //     panic!("unknown");
        // }
        self.event_type.as_str().write(stream)?;

        for entry in self.entries.iter() {
            entry.kind.write(stream)?;
            entry.name.write(stream)?;
        }
        GameEventValueType::None.write(stream)?;

        Ok(())
    }
}

#[test]
fn test_event_definition_roundtrip() {
    crate::test_roundtrip_write(GameEventDefinition {
        id: GameEventTypeId(0),
        event_type: GameEventType::ServerChangeLevelFailed,
        entries: vec![GameEventEntry {
            name: "level_name".to_string(),
            kind: GameEventValueType::String,
        }],
    });
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
        (self.event_list.len() as u16).write_sized(stream, 9)?;
        stream.reserve_length(20, |stream| {
            for event in self.event_list.iter() {
                event.write(stream)?;
            }
            Ok(())
        })
    }
}

#[test]
fn test_event_list_roundtrip() {
    crate::test_roundtrip_write(GameEventListMessage { event_list: vec![] });
    crate::test_roundtrip_write(GameEventListMessage {
        event_list: vec![GameEventDefinition {
            id: GameEventTypeId(0),
            event_type: GameEventType::ServerChangeLevelFailed,
            entries: vec![GameEventEntry {
                name: "level_name".to_string(),
                kind: GameEventValueType::String,
            }],
        }],
    });
    crate::test_roundtrip_write(GameEventListMessage {
        event_list: vec![
            GameEventDefinition {
                id: GameEventTypeId(0),
                event_type: GameEventType::ServerSpawn,
                entries: vec![
                    GameEventEntry {
                        name: "hostname".to_string(),
                        kind: GameEventValueType::String,
                    },
                    GameEventEntry {
                        name: "address".to_string(),
                        kind: GameEventValueType::String,
                    },
                    GameEventEntry {
                        name: "ip".to_string(),
                        kind: GameEventValueType::Long,
                    },
                    GameEventEntry {
                        name: "port".to_string(),
                        kind: GameEventValueType::Short,
                    },
                    GameEventEntry {
                        name: "game".to_string(),
                        kind: GameEventValueType::String,
                    },
                    GameEventEntry {
                        name: "map_name".to_string(),
                        kind: GameEventValueType::String,
                    },
                    GameEventEntry {
                        name: "max_players".to_string(),
                        kind: GameEventValueType::Long,
                    },
                    GameEventEntry {
                        name: "os".to_string(),
                        kind: GameEventValueType::String,
                    },
                    GameEventEntry {
                        name: "dedicated".to_string(),
                        kind: GameEventValueType::Boolean,
                    },
                    GameEventEntry {
                        name: "password".to_string(),
                        kind: GameEventValueType::Boolean,
                    },
                ],
            },
            GameEventDefinition {
                id: GameEventTypeId(1),
                event_type: GameEventType::ServerChangeLevelFailed,
                entries: vec![GameEventEntry {
                    name: "level_name".to_string(),
                    kind: GameEventValueType::String,
                }],
            },
            GameEventDefinition {
                id: GameEventTypeId(2),
                event_type: GameEventType::GameInit,
                entries: vec![],
            },
        ],
    });
}
