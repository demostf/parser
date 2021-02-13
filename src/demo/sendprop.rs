use bitbuffer::{BitRead, LittleEndian};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};

use crate::{ParseError, ReadResult, Result, Stream};

use super::packet::datatable::ParseSendTable;
use super::vector::{Vector, VectorXY};
use crate::demo::message::stringtable::log_base2;
use crate::demo::packet::datatable::SendTableName;
use crate::demo::parser::MalformedSendPropDefinitionError;
use parse_display::Display;
use std::cmp::min;
use std::convert::TryFrom;

use std::fmt;
use std::rc::Rc;

#[derive(
    BitRead, PartialEq, Eq, Hash, Debug, Display, Clone, Serialize, Deserialize, Ord, PartialOrd,
)]
pub struct SendPropName(Rc<String>);

impl SendPropName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl PartialEq<&str> for SendPropName {
    fn eq(&self, other: &&str) -> bool {
        self.0.as_str() == *other
    }
}

impl From<String> for SendPropName {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

#[derive(Debug, Clone)]
pub struct RawSendPropDefinition {
    pub prop_type: SendPropType,
    pub identifier: Rc<SendPropIdentifier>,
    pub flags: SendPropFlags,
    pub table_name: Option<SendTableName>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub bit_count: Option<u32>,
    pub element_count: Option<u16>,
    pub array_property: Option<Box<RawSendPropDefinition>>,
}

impl PartialEq for RawSendPropDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.identifier.index == other.identifier.index
    }
}

impl fmt::Display for RawSendPropDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.prop_type {
            SendPropType::Vector | SendPropType::VectorXY => write!(
                f,
                "{}::{}({})(flags: {}, low: {}, high: {}, bits: {})",
                self.identifier.owner_table,
                self.identifier.name,
                self.prop_type,
                self.flags,
                self.low_value.unwrap_or_default(),
                self.high_value.unwrap_or_default(),
                self.bit_count.unwrap_or(96) / 3
            ),
            SendPropType::Float => write!(
                f,
                "{}::{}({})(flags: {}, low: {}, high: {}, bits: {})",
                self.identifier.owner_table,
                self.identifier.name,
                self.prop_type,
                self.flags,
                self.low_value.unwrap_or_default(),
                self.high_value.unwrap_or_default(),
                self.bit_count.unwrap_or(32)
            ),
            SendPropType::Int => write!(
                f,
                "{}::{}({})(flags: {}, bits: {})",
                self.identifier.owner_table,
                self.identifier.name,
                self.prop_type,
                self.flags,
                self.bit_count.unwrap_or(32)
            ),
            SendPropType::String => {
                write!(
                    f,
                    "{}::{}({})",
                    self.identifier.owner_table, self.identifier.name, self.prop_type
                )
            }
            SendPropType::Array => match &self.array_property {
                Some(array_prop) => write!(
                    f,
                    "{}::{}([{}({})] * {})",
                    self.identifier.owner_table,
                    self.identifier.name,
                    array_prop.prop_type,
                    array_prop.flags,
                    self.element_count.unwrap_or_default(),
                ),
                None => write!(f, "{}(Malformed array)", self.identifier.name),
            },
            SendPropType::DataTable => match &self.table_name {
                Some(sub_table) => write!(
                    f,
                    "{}::{}(DataTable = {})",
                    self.identifier.owner_table, self.identifier.name, sub_table
                ),
                None => write!(f, "{}(Malformed DataTable)", self.identifier.name),
            },
            SendPropType::NumSendPropTypes => {
                write!(
                    f,
                    "{}::{}(NumSendPropTypes)",
                    self.identifier.owner_table, self.identifier.name
                )
            }
        }
    }
}

impl RawSendPropDefinition {
    pub fn with_array_property(self, array_property: Self) -> Self {
        RawSendPropDefinition {
            prop_type: self.prop_type,
            identifier: self.identifier,
            flags: self.flags,
            table_name: self.table_name,
            low_value: self.low_value,
            high_value: self.high_value,
            bit_count: self.bit_count,
            element_count: self.element_count,
            array_property: Some(Box::new(array_property)),
        }
    }

    /// Get the referred data table
    ///
    /// Note that this is not the owner table
    pub fn get_data_table<'a>(&self, tables: &'a [ParseSendTable]) -> Option<&'a ParseSendTable> {
        if self.prop_type == SendPropType::DataTable {
            self.table_name
                .as_ref()
                .and_then(|name| tables.iter().find(|table| table.name == *name))
        } else {
            None
        }
    }

    pub fn read(
        stream: &mut Stream,
        owner_table: SendTableName,
        index: SendPropDefinitionIndex,
    ) -> ReadResult<Self> {
        let prop_type = SendPropType::read(stream)?;
        let name = stream.read_string(None)?.to_string().into();
        let flags = SendPropFlags::read(stream)?;
        let mut table_name = None;
        let mut element_count = None;
        let mut low_value = None;
        let mut high_value = None;
        let mut bit_count = None;
        if flags.contains(SendPropFlag::Exclude) || prop_type == SendPropType::DataTable {
            table_name = Some(stream.read()?);
        } else if prop_type == SendPropType::Array {
            element_count = Some(stream.read_int(10)?);
        } else {
            low_value = Some(stream.read()?);
            high_value = Some(stream.read()?);
            bit_count = Some(stream.read_int(7)?);
        }

        if flags.contains(SendPropFlag::NoScale) {
            if prop_type == SendPropType::Float {
                bit_count = Some(32);
            } else if prop_type == SendPropType::Vector
                && !flags.contains(SendPropFlag::NormalVarInt)
            {
                bit_count = Some(32 * 3);
            }
        }

        Ok(RawSendPropDefinition {
            prop_type,
            identifier: Rc::new(SendPropIdentifier {
                index,
                name,
                owner_table,
            }),
            flags,
            table_name,
            low_value,
            high_value,
            bit_count,
            element_count,
            array_property: None,
        })
    }

    pub fn is_exclude(&self) -> bool {
        self.flags.contains(SendPropFlag::Exclude)
    }

    pub fn get_exclude_table(&self) -> Option<&SendTableName> {
        if self.is_exclude() {
            self.table_name.as_ref()
        } else {
            None
        }
    }
}

#[derive(BitRead, Copy, Clone, PartialEq, Debug, Display)]
#[discriminant_bits = 5]
pub enum SendPropType {
    Int = 0,
    Float = 1,
    Vector = 2,
    VectorXY = 3,
    String = 4,
    Array = 5,
    DataTable = 6,
    NumSendPropTypes = 7,
}

#[derive(BitFlags, Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum SendPropFlag {
    // Unsigned integer data.
    Unsigned = 1,
    // If this is set, the float/vector is treated like a world coordinate.
    // Note that the bit count is ignored in this case.
    Coord = 2,
    // For floating point, don't scale into range, just take value as is.
    NoScale = 4,
    // For floating point, limit high value to range minus one bit unit
    RoundDown = 8,
    // For floating point, limit low value to range minus one bit unit
    RoundUp = 16,
    // This is an exclude prop (not excluded, but it points at another prop to be excluded).
    Exclude = 64,
    // Use XYZ/Exponent encoding for vectors.
    XYZE = 128,
    // This tells us that the property is inside an array, so it shouldn't be put into the
    // flattened property list. Its array will point at it when it needs to.
    InsideArray = 256,
    // Set for datatable props using one of the default datatable proxies like
    // SendProxy_DataTableToDataTable that always send the data to all clients.
    ProxyAlwaysYes = 512,
    // this is an often changed field, moved to head of sendtable so it gets a small index
    ChangesOften = 1024,
    // Set automatically if SPROP_VECTORELEM is used.
    IsVectorElement = 2048,
    // Set automatically if it's a datatable with an offset of 0 that doesn't change the pointer
    // (ie: for all automatically-chained base classes).
    // In this case, it can get rid of this SendPropDataTable altogether and spare the
    // trouble of walking the hierarchy more than necessary.
    Collapsible = 4096,
    // Like SPROP_COORD, but special handling for multiplayer games
    CoordMP = 8192,
    // Like SPROP_COORD, but special handling for multiplayer games
    // where the fractional component only gets a 3 bits instead of 5
    CoordMPLowPrecision = 16384,
    // SPROP_COORD_MP, but coordinates are rounded to integral boundaries
    // overloaded as both "Normal" and "VarInt"
    CoordMPIntegral = 32768,
    NormalVarInt = 32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct SendPropFlags(BitFlags<SendPropFlag>);

impl fmt::Display for SendPropFlags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug = format!("{:?}", self.0);
        let flags: String = debug
            .chars()
            .skip_while(|c| *c != '[')
            .take_while(|c| *c != ')')
            .collect();
        write!(f, "{}", flags)
    }
}

impl SendPropFlags {
    pub fn contains(self, other: SendPropFlag) -> bool {
        self.0.contains(other)
    }
}

impl BitRead<'_, LittleEndian> for SendPropFlags {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        // since all 16 bits worth of flags are used there are no invalid flags
        Ok(SendPropFlags(BitFlags::from_bits_truncate(stream.read()?)))
    }

    fn bit_size() -> Option<usize> {
        Some(16)
    }
}

#[derive(Debug, Clone)]
pub enum FloatDefinition {
    Coord,
    CoordMP,
    CoordMPLowPrecision,
    CoordMPIntegral,
    FloatNoScale,
    NormalVarFloat,
    Scaled { bit_count: u8, high: f32, low: f32 },
}

impl FloatDefinition {
    pub fn new(
        flags: SendPropFlags,
        bit_count: Option<u32>,
        high: Option<f32>,
        low: Option<f32>,
    ) -> std::result::Result<Self, MalformedSendPropDefinitionError> {
        if flags.contains(SendPropFlag::Coord) {
            Ok(FloatDefinition::Coord)
        } else if flags.contains(SendPropFlag::CoordMP) {
            Ok(FloatDefinition::CoordMP)
        } else if flags.contains(SendPropFlag::CoordMPLowPrecision) {
            Ok(FloatDefinition::CoordMPLowPrecision)
        } else if flags.contains(SendPropFlag::CoordMPIntegral) {
            Ok(FloatDefinition::CoordMPIntegral)
        } else if flags.contains(SendPropFlag::NoScale) {
            Ok(FloatDefinition::FloatNoScale)
        } else if flags.contains(SendPropFlag::NormalVarInt) {
            Ok(FloatDefinition::NormalVarFloat)
        } else if let (Some(bit_count), Some(high), Some(low)) = (bit_count, high, low) {
            Ok(FloatDefinition::Scaled {
                bit_count: bit_count as u8,
                high,
                low,
            })
        } else {
            Err(MalformedSendPropDefinitionError::UnsizedFloat)
        }
    }
}

#[derive(Debug, Clone)]
pub enum SendPropDefinition {
    NormalVarInt {
        identifier: Rc<SendPropIdentifier>,
        unsigned: bool,
    },
    UnsignedInt {
        identifier: Rc<SendPropIdentifier>,
        bit_count: u8,
    },
    Int {
        identifier: Rc<SendPropIdentifier>,
        bit_count: u8,
    },
    Float {
        identifier: Rc<SendPropIdentifier>,
        definition: FloatDefinition,
    },
    String {
        identifier: Rc<SendPropIdentifier>,
    },
    Vector {
        identifier: Rc<SendPropIdentifier>,
        definition: FloatDefinition,
    },
    VectorXY {
        identifier: Rc<SendPropIdentifier>,
        definition: FloatDefinition,
    },
    Array {
        identifier: Rc<SendPropIdentifier>,
        inner_definition: Box<SendPropDefinition>,
        count_bit_count: u16,
    },
}

impl SendPropDefinition {
    pub fn identifier(&self) -> Rc<SendPropIdentifier> {
        match self {
            SendPropDefinition::NormalVarInt { identifier, .. } => identifier.clone(),
            SendPropDefinition::UnsignedInt { identifier, .. } => identifier.clone(),
            SendPropDefinition::Int { identifier, .. } => identifier.clone(),
            SendPropDefinition::Float { identifier, .. } => identifier.clone(),
            SendPropDefinition::String { identifier, .. } => identifier.clone(),
            SendPropDefinition::Vector { identifier, .. } => identifier.clone(),
            SendPropDefinition::VectorXY { identifier, .. } => identifier.clone(),
            SendPropDefinition::Array { identifier, .. } => identifier.clone(),
        }
    }
}

impl TryFrom<&RawSendPropDefinition> for SendPropDefinition {
    type Error = MalformedSendPropDefinitionError;

    fn try_from(definition: &RawSendPropDefinition) -> std::result::Result<Self, Self::Error> {
        let identifier = definition.identifier.clone();
        match definition.prop_type {
            SendPropType::Int => {
                if definition.flags.contains(SendPropFlag::NormalVarInt) {
                    Ok(SendPropDefinition::NormalVarInt {
                        identifier,
                        unsigned: definition.flags.contains(SendPropFlag::Unsigned),
                    })
                } else if definition.flags.contains(SendPropFlag::Unsigned) {
                    Ok(SendPropDefinition::UnsignedInt {
                        identifier,
                        bit_count: definition.bit_count.unwrap_or(32) as u8,
                    })
                } else {
                    Ok(SendPropDefinition::Int {
                        identifier,
                        bit_count: definition.bit_count.unwrap_or(32) as u8,
                    })
                }
            }
            SendPropType::Float => Ok(SendPropDefinition::Float {
                identifier,
                definition: FloatDefinition::new(
                    definition.flags,
                    definition.bit_count,
                    definition.high_value,
                    definition.low_value,
                )?,
            }),
            SendPropType::String => Ok(SendPropDefinition::String { identifier }),
            SendPropType::Vector => Ok(SendPropDefinition::Vector {
                identifier,
                definition: FloatDefinition::new(
                    definition.flags,
                    definition.bit_count,
                    definition.high_value,
                    definition.low_value,
                )?,
            }),
            SendPropType::VectorXY => Ok(SendPropDefinition::VectorXY {
                identifier,
                definition: FloatDefinition::new(
                    definition.flags,
                    definition.bit_count,
                    definition.high_value,
                    definition.low_value,
                )?,
            }),
            SendPropType::Array => {
                let count_bit_count = log_base2(
                    definition
                        .element_count
                        .ok_or(MalformedSendPropDefinitionError::UnsizedArray)?,
                ) as u16
                    + 1;
                let child_definition = definition
                    .array_property
                    .as_deref()
                    .ok_or(MalformedSendPropDefinitionError::UntypedArray)?;
                Ok(SendPropDefinition::Array {
                    identifier,
                    inner_definition: Box::new(SendPropDefinition::try_from(child_definition)?),
                    count_bit_count,
                })
            }
            _ => Err(MalformedSendPropDefinitionError::InvalidPropType),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SendPropValue {
    Vector(Vector),
    VectorXY(VectorXY),
    Integer(i64),
    Float(f32),
    String(String),
    Array(Vec<SendPropValue>),
}

impl PartialEq for SendPropValue {
    fn eq(&self, other: &Self) -> bool {
        // allow comparing some "compatible" types
        match (self, other) {
            (SendPropValue::Vector(value1), SendPropValue::Vector(value2)) => value1 == value2,
            (SendPropValue::VectorXY(value1), SendPropValue::VectorXY(value2)) => value1 == value2,
            (SendPropValue::Integer(value1), SendPropValue::Integer(value2)) => value1 == value2,
            (SendPropValue::Float(value1), SendPropValue::Float(value2)) => value1 - value2 < 0.001,
            (SendPropValue::String(value1), SendPropValue::String(value2)) => value1 == value2,
            (SendPropValue::Array(value1), SendPropValue::Array(value2)) => value1 == value2,
            (SendPropValue::Integer(value1), SendPropValue::Float(value2)) => {
                *value1 as f64 == *value2 as f64
            }
            (SendPropValue::Float(value1), SendPropValue::Integer(value2)) => {
                *value1 as f64 == *value2 as f64
            }
            (SendPropValue::Vector(value1), SendPropValue::VectorXY(value2)) => {
                value1.x == value2.x && value1.y == value2.y && value1.z == 0.0
            }
            (SendPropValue::VectorXY(value1), SendPropValue::Vector(value2)) => {
                value1.x == value2.x && value1.y == value2.y && value2.z == 0.0
            }
            (SendPropValue::Vector(value1), SendPropValue::Array(value2)) if value2.len() == 3 => {
                SendPropValue::Float(value1.x) == value2[0]
                    && SendPropValue::Float(value1.y) == value2[1]
                    && SendPropValue::Float(value1.z) == value2[2]
            }
            (SendPropValue::Array(value1), SendPropValue::Vector(value2)) if value1.len() == 3 => {
                SendPropValue::Float(value2.x) == value1[0]
                    && SendPropValue::Float(value2.y) == value1[1]
                    && SendPropValue::Float(value2.z) == value1[2]
            }
            (SendPropValue::VectorXY(value1), SendPropValue::Array(value2))
                if value2.len() == 2 =>
            {
                SendPropValue::Float(value1.x) == value2[0]
                    && SendPropValue::Float(value1.y) == value2[1]
            }
            (SendPropValue::Array(value1), SendPropValue::VectorXY(value2))
                if value1.len() == 2 =>
            {
                SendPropValue::Float(value2.x) == value1[0]
                    && SendPropValue::Float(value2.y) == value1[1]
            }
            _ => false,
        }
    }
}

impl fmt::Display for SendPropValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SendPropValue::Vector(vector) => vector.fmt(f),
            SendPropValue::VectorXY(vector) => vector.fmt(f),
            SendPropValue::Integer(int) => int.fmt(f),
            SendPropValue::Float(float) => float.fmt(f),
            SendPropValue::String(string) => string.fmt(f),
            SendPropValue::Array(array) => {
                write!(f, "[")?;
                for child in array {
                    write!(f, "{}", child)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl SendPropValue {
    pub fn parse(stream: &mut Stream, definition: &SendPropDefinition) -> Result<Self> {
        match definition {
            SendPropDefinition::NormalVarInt { unsigned, .. } => read_var_int(stream, !*unsigned)
                .map_err(ParseError::from)
                .map(|int| int as i64)
                .map(SendPropValue::from),
            SendPropDefinition::UnsignedInt { bit_count, .. } => {
                Ok((stream.read_sized::<u32>(*bit_count as usize)? as i64).into())
            }
            SendPropDefinition::Int { bit_count, .. } => stream
                .read_int::<i32>((*bit_count) as usize)
                .map_err(ParseError::from)
                .map(SendPropValue::from),
            SendPropDefinition::Float {
                definition: float_definition,
                ..
            } => Self::read_float(stream, float_definition).map(SendPropValue::from),
            SendPropDefinition::String { .. } => {
                let length = stream.read_int(9)?;
                stream
                    .read_sized::<String>(length)
                    .map_err(ParseError::from)
                    .map(SendPropValue::from)
            }
            SendPropDefinition::Vector {
                definition: float_definition,
                ..
            } => Ok(Vector {
                x: Self::read_float(stream, float_definition)?,
                y: Self::read_float(stream, float_definition)?,
                z: Self::read_float(stream, float_definition)?,
            }
            .into()),
            SendPropDefinition::VectorXY {
                definition: float_definition,
                ..
            } => Ok(VectorXY {
                x: Self::read_float(stream, float_definition)?,
                y: Self::read_float(stream, float_definition)?,
            }
            .into()),
            SendPropDefinition::Array {
                count_bit_count,
                inner_definition,
                ..
            } => {
                let count = stream.read_int(*count_bit_count as usize)?;
                let mut values = Vec::with_capacity(min(count, 128));

                for _ in 0..count {
                    values.push(Self::parse(stream, inner_definition)?);
                }

                Ok(values.into())
            }
        }
    }

    fn read_float(stream: &mut Stream, definition: &FloatDefinition) -> Result<f32> {
        match definition {
            FloatDefinition::Coord => read_bit_coord(stream).map_err(ParseError::from),
            FloatDefinition::CoordMP => {
                read_bit_coord_mp(stream, false, false).map_err(ParseError::from)
            }
            FloatDefinition::CoordMPLowPrecision => {
                read_bit_coord_mp(stream, false, true).map_err(ParseError::from)
            }
            FloatDefinition::CoordMPIntegral => {
                read_bit_coord_mp(stream, true, false).map_err(ParseError::from)
            }
            FloatDefinition::FloatNoScale => stream.read().map_err(ParseError::from),
            FloatDefinition::NormalVarFloat => read_bit_normal(stream).map_err(ParseError::from),
            FloatDefinition::Scaled {
                bit_count,
                low,
                high,
            } => {
                let raw: u32 = stream.read_int(*bit_count as usize)?;
                // is this -1 correct?, it is consistent with the js version but seems weird
                let percentage =
                    (raw as f32) / ((1i32.wrapping_shl(*bit_count as u32)) as f32 - 1.0);
                Ok(low + ((high - low) * percentage))
            }
        }
    }
}

impl From<i32> for SendPropValue {
    fn from(value: i32) -> Self {
        SendPropValue::Integer(value as i64)
    }
}

impl From<i64> for SendPropValue {
    fn from(value: i64) -> Self {
        SendPropValue::Integer(value)
    }
}

impl From<Vector> for SendPropValue {
    fn from(value: Vector) -> Self {
        SendPropValue::Vector(value)
    }
}

impl From<VectorXY> for SendPropValue {
    fn from(value: VectorXY) -> Self {
        SendPropValue::VectorXY(value)
    }
}

impl From<f32> for SendPropValue {
    fn from(value: f32) -> Self {
        SendPropValue::Float(value)
    }
}

impl From<String> for SendPropValue {
    fn from(value: String) -> Self {
        SendPropValue::String(value)
    }
}

impl From<Vec<SendPropValue>> for SendPropValue {
    fn from(value: Vec<SendPropValue>) -> Self {
        SendPropValue::Array(value)
    }
}

impl TryFrom<&SendPropValue> for i64 {
    type Error = ();
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Integer(val) => Ok(*val),
            _ => Err(()),
        }
    }
}

impl TryFrom<&SendPropValue> for Vector {
    type Error = ();
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Vector(val) => Ok(*val),
            _ => Err(()),
        }
    }
}

impl TryFrom<&SendPropValue> for VectorXY {
    type Error = ();
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::VectorXY(val) => Ok(*val),
            _ => Err(()),
        }
    }
}

impl TryFrom<&SendPropValue> for f32 {
    type Error = ();
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Float(val) => Ok(*val),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a SendPropValue> for &'a str {
    type Error = ();
    fn try_from(value: &'a SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::String(val) => Ok(val.as_str()),
            _ => Err(()),
        }
    }
}

impl<'a> TryFrom<&'a SendPropValue> for &'a [SendPropValue] {
    type Error = ();
    fn try_from(value: &'a SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Array(val) => Ok(val.as_slice()),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct SendPropDefinitionIndex(u32);

impl SendPropDefinitionIndex {
    pub fn new(table: usize, prop: usize) -> Self {
        // there can be at most 1024 (10 bits) props per table
        SendPropDefinitionIndex((table * 1024 + prop) as u32)
    }
}

#[derive(Debug, Display)]
#[display("{owner_table}::{name}")]
pub struct SendPropIdentifier {
    pub index: SendPropDefinitionIndex,
    pub name: SendPropName,
    pub owner_table: SendTableName,
}

#[derive(Debug, Clone, Display)]
#[display("{identifier} = {value}")]
pub struct SendProp {
    pub identifier: Rc<SendPropIdentifier>,
    pub value: SendPropValue,
}

pub fn read_var_int(stream: &mut Stream, signed: bool) -> ReadResult<i32> {
    let mut result: i32 = 0;

    for i in (0..35).step_by(7) {
        let byte: u8 = stream.read()?;
        result |= ((byte & 0x7F) as i32) << i;

        if (byte >> 7) == 0 {
            break;
        }
    }

    if signed {
        Ok((result >> 1) ^ -(result & 1))
    } else {
        Ok(result)
    }
}

pub fn read_bit_coord(stream: &mut Stream) -> ReadResult<f32> {
    let has_int = stream.read()?;
    let has_frac = stream.read()?;

    Ok(if has_int || has_frac {
        let sign = if stream.read()? { -1f32 } else { 1f32 };
        let int_val: u16 = if has_int {
            stream.read_sized::<u16>(14)? + 1
        } else {
            0
        };
        let frac_val: u8 = if has_frac { stream.read_sized(5)? } else { 0 };
        let value = int_val as f32 + (frac_val as f32 * get_frac_factor(5));
        value * sign
    } else {
        0f32
    })
}

fn get_frac_factor(bits: usize) -> f32 {
    1.0 / ((1 << bits) as f32)
}

pub fn read_bit_coord_mp(
    stream: &mut Stream,
    is_integral: bool,
    low_precision: bool,
) -> ReadResult<f32> {
    let mut value = 0.0;
    let mut is_negative = false;

    let in_bounds = stream.read()?;
    let has_int_val = stream.read()?;

    if is_integral {
        if has_int_val {
            is_negative = stream.read()?;

            let int_val = stream.read_sized::<u32>(if in_bounds { 11 } else { 14 })? + 1;
            value = int_val as f32;
        }
    } else {
        is_negative = stream.read()?;
        if has_int_val {
            let int_val = stream.read_sized::<u32>(if in_bounds { 11 } else { 14 })? + 1;
            value = int_val as f32;
        }
        let frac_bits = if low_precision { 3 } else { 5 };
        let frac_val: u32 = stream.read_sized(frac_bits)?;
        value += (frac_val as f32) * get_frac_factor(frac_bits);
    }

    if is_negative {
        value = -value;
    }

    Ok(value)
}

pub fn read_bit_normal(stream: &mut Stream) -> ReadResult<f32> {
    let is_negative = stream.read()?;
    let frac_val: u16 = stream.read_sized(11)?;
    let value = (frac_val as f32) * get_frac_factor(11);
    if is_negative {
        Ok(-value)
    } else {
        Ok(value)
    }
}
