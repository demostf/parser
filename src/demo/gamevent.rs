use std::collections::HashMap;

use crate::{ParseError, Result};

pub use super::gameevent_gen::{GameEvent, GameEventType};

#[derive(Debug)]
pub struct GameEventDefinition {
    pub id: GameEventType,
    pub name: String,
    pub entries: Vec<GameEventEntry>,
}

#[derive(Debug)]
pub struct GameEventEntry {
    pub name: String,
    pub kind: GameEventValueType,
}

#[derive(Debug, Clone, Copy)]
pub enum GameEventValueType {
    String,
    Float,
    Long,
    Short,
    Byte,
    Boolean,
    Local,
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
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for f32 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Float(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for u32 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Long(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for u16 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Short(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for u8 {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Byte(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for bool {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Boolean(val) => Ok(val),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

impl FromGameEventValue for () {
    fn from_value(value: GameEventValue, name: &str) -> Result<Self> {
        match value {
            GameEventValue::Local => Ok(()),
            _ => Err(ParseError::InvalidGameEvent {
                name: name.to_string(),
                value,
            })
        }
    }
}

pub struct RawGameEvent {
    pub event_type: GameEventType,
    pub values: HashMap<String, GameEventValue>,
}

pub trait FromRawGameEvent: Sized {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self>;
}