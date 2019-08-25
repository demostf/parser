use bitstream_reader::BitRead;

use crate::{MalformedDemoError, ParseError, Result};

pub use super::gameevent_gen::{GameEvent, GameEventType};
use crate::demo::message::gameevent::GameEventTypeId;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug)]
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

impl fmt::Display for GameEventValueType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GameEventValueType::None => write!(f, "None"),
            GameEventValueType::String => write!(f, "String"),
            GameEventValueType::Float => write!(f, "Float"),
            GameEventValueType::Long => write!(f, "Long"),
            GameEventValueType::Short => write!(f, "Short"),
            GameEventValueType::Byte => write!(f, "Byte"),
            GameEventValueType::Boolean => write!(f, "Boolean"),
            GameEventValueType::Local => write!(f, "Local"),
        }
    }
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
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self>;
}

impl FromGameEventValue for String {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::String(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::String,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for f32 {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Float(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Float,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for u32 {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Long(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Long,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for u16 {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Short(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Short,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for u8 {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Byte(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Byte,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for bool {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Boolean(val) => Ok(val),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Boolean,
                name,
                found_type: value.get_type(),
            }
            .into()),
        }
    }
}

impl FromGameEventValue for () {
    fn from_value(value: GameEventValue, name: &'static str) -> Result<Self> {
        match value {
            GameEventValue::Local => Ok(()),
            _ => Err(MalformedDemoError::InvalidGameEvent {
                expected_type: GameEventValueType::Local,
                name,
                found_type: value.get_type(),
            }
            .into()),
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
