pub use super::gameevent_gen::{GameEvent, GameEventType};
use crate::demo::data::MaybeUtf8String;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::{GameEventError, Result, Stream};
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use parse_display::Display;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameEventDefinition {
    pub id: GameEventTypeId,
    pub event_type: GameEventType,
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameEventEntry {
    pub name: String,
    pub kind: GameEventValueType,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, Copy, PartialEq, Display, Serialize, Deserialize)]
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEventValue {
    String(MaybeUtf8String),
    Float(f32),
    Long(u32),
    Short(u16),
    Byte(u8),
    Boolean(bool),
    Local,
}

fn read_event_value(stream: &mut Stream, definition: &GameEventEntry) -> Result<GameEventValue> {
    Ok(match definition.kind {
        GameEventValueType::String => GameEventValue::String(stream.read()?),
        GameEventValueType::Float => GameEventValue::Float(stream.read()?),
        GameEventValueType::Long => GameEventValue::Long(stream.read()?),
        GameEventValueType::Short => GameEventValue::Short(stream.read()?),
        GameEventValueType::Byte => GameEventValue::Byte(stream.read()?),
        GameEventValueType::Boolean => GameEventValue::Boolean(stream.read()?),
        GameEventValueType::Local => GameEventValue::Local,
        GameEventValueType::None => return Err(GameEventError::NoneValue.into()),
    })
}

impl BitWrite<LittleEndian> for GameEventValue {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> bitbuffer::Result<()> {
        match self {
            GameEventValue::String(value) => value.write(stream),
            GameEventValue::Float(value) => value.write(stream),
            GameEventValue::Long(value) => value.write(stream),
            GameEventValue::Short(value) => value.write(stream),
            GameEventValue::Byte(value) => value.write(stream),
            GameEventValue::Boolean(value) => value.write(stream),
            GameEventValue::Local => Ok(()),
        }
    }
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

pub trait EventValue: Sized {
    fn value_type() -> GameEventValueType;
}

impl EventValue for String {
    fn value_type() -> GameEventValueType {
        GameEventValueType::String
    }
}

impl EventValue for MaybeUtf8String {
    fn value_type() -> GameEventValueType {
        GameEventValueType::String
    }
}

impl EventValue for f32 {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Float
    }
}

impl EventValue for u32 {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Long
    }
}

impl EventValue for u16 {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Short
    }
}

impl EventValue for u8 {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Byte
    }
}

impl EventValue for bool {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Boolean
    }
}

impl EventValue for () {
    fn value_type() -> GameEventValueType {
        GameEventValueType::Local
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RawGameEvent {
    pub event_type: GameEventType,
    pub values: Vec<GameEventValue>,
}

impl RawGameEvent {
    pub fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut values: Vec<GameEventValue> = Vec::with_capacity(definition.entries.len());
        for entry in &definition.entries {
            values.push(read_event_value(stream, entry)?);
        }

        Ok(RawGameEvent {
            event_type: definition.event_type.clone(),
            values,
        })
    }
}

impl BitWrite<LittleEndian> for RawGameEvent {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> bitbuffer::Result<()> {
        for value in self.values.iter() {
            value.write(stream)?;
        }
        Ok(())
    }
}

pub trait FromRawGameEvent: Sized {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self>;
}

impl<T: FromRawGameEvent> FromRawGameEvent for Box<T> {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        Ok(Box::new(T::from_raw_event(values)?))
    }
}
