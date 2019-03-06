use bitstream_reader::BitRead;

use crate::{ParseError, Result};

pub use super::gameevent_gen::{GameEvent, GameEventType};
use crate::demo::message::gameevent::GameEventTypeId;

#[derive(Debug)]
pub struct GameEventDefinition {
    pub id: GameEventTypeId,
    pub event_type: GameEventType,
    pub name: String,
    pub entries: Vec<GameEventEntry>,
}

#[derive(Debug)]
pub struct GameEventEntry {
    pub name: String,
    pub kind: GameEventValueType,
}

#[derive(BitRead, Debug, Clone, Copy, PartialEq)]
#[discriminant_bits = 3]
pub enum GameEventValueType {
    None = 0,
    String = 1,
    Float = 2,
    Long = 3,
    Short = 4,
    Byte = 5,
    Boolean = 6,
    Local = 7,
}

#[derive(Debug, Clone)]
pub enum GameEventValue {
    String(String),
    Float(f32),
    Long(u32),
    Short(u16),
    Byte(u8),
    Boolean(bool),
    Local,
}

pub trait FromGameEventValue: Sized {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self>;
}

impl FromGameEventValue for String {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::String(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::String,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for f32 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Float(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Float,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for u32 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Long(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Long,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for u16 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Short(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Short,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for u8 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Byte(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Byte,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for bool {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Boolean(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Boolean,
                name: name.to_string(),
                value,
            }),
        }
    }
}

impl FromGameEventValue for () {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Local => Ok(()),
            _ => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Local,
                name: name.to_string(),
                value,
            }),
        }
    }
}

#[derive(Debug)]
pub struct RawGameEvent {
    pub event_type: GameEventType,
    pub values: Vec<GameEventValue>,
}

pub trait FromRawGameEvent: Sized {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self>;
}
