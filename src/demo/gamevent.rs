use bitbuffer::BitRead;

use crate::{ParseError, Result};

pub use super::gameevent_gen::{GameEvent, GameEventType};
use crate::demo::message::gameevent::GameEventTypeId;
use parse_display::Display;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct GameEventDefinition {
    pub id: GameEventTypeId,
    pub event_type: GameEventType,
    pub name: String,
    pub entries: Vec<GameEventEntry>,
}

impl PartialEq<GameEventDefinition> for GameEventDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for GameEventDefinition {}

impl PartialOrd for GameEventDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Ord for GameEventDefinition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Debug, Clone)]
pub struct GameEventEntry {
    pub name: String,
    pub kind: GameEventValueType,
}

#[derive(BitRead, Debug, Clone, Copy, PartialEq, Display)]
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

impl GameEventValue {
    pub fn get_type(&self) -> GameEventValueType {
        match self {
            GameEventValue::String(_) => GameEventValueType::String,
            GameEventValue::Float(_) => GameEventValueType::Float,
            GameEventValue::Long(_) => GameEventValueType::Long,
            GameEventValue::Short(_) => GameEventValueType::Short,
            GameEventValue::Byte(_) => GameEventValueType::Byte,
            GameEventValue::Boolean(_) => GameEventValueType::Boolean,
            GameEventValue::Local => GameEventValueType::Local,
        }
    }
}

pub trait FromGameEventValue: Sized {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self>;
}

impl FromGameEventValue for String {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::String(val)) => Ok(val),
            None => Ok(String::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::String,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for f32 {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Float(val)) => Ok(val),
            None => Ok(f32::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Float,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for u32 {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Long(val)) => Ok(val),
            Some(GameEventValue::Short(val)) => Ok(val as u32),
            Some(GameEventValue::Byte(val)) => Ok(val as u32),
            None => Ok(u32::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Long,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for u16 {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Long(val)) => Ok(val as u16),
            Some(GameEventValue::Short(val)) => Ok(val),
            Some(GameEventValue::Byte(val)) => Ok(val as u16),
            None => Ok(u16::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Short,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for u8 {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Long(val)) => Ok(val as u8),
            Some(GameEventValue::Short(val)) => Ok(val as u8),
            Some(GameEventValue::Byte(val)) => Ok(val),
            None => Ok(u8::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Byte,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for bool {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Boolean(val)) => Ok(val),
            None => Ok(bool::default()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Boolean,
                name,
                found_type: value.get_type(),
            }),
        }
    }
}

impl FromGameEventValue for () {
    fn from_value(value: Option<GameEventValue>, name: &'static str) -> Result<Self> {
        match value {
            Some(GameEventValue::Local) | None => Ok(()),
            Some(value) => Err(ParseError::InvalidGameEvent {
                expected_type: GameEventValueType::Local,
                name,
                found_type: value.get_type(),
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

impl<T: FromRawGameEvent> FromRawGameEvent for Box<T> {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        Ok(Box::new(T::from_raw_event(values)?))
    }
}
