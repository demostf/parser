use super::packet::datatable::ParseSendTable;
use super::vector::{Vector, VectorXY};
use crate::consthash::ConstFnvHash;
use crate::demo::message::stringtable::log_base2;
use crate::demo::packet::datatable::SendTableName;
use crate::demo::parser::MalformedSendPropDefinitionError;
use crate::demo::sendprop_gen::get_prop_names;
use crate::{ParseError, ReadResult, Result, Stream};
use bitbuffer::{
    BitRead, BitReadStream, BitWrite, BitWriteSized, BitWriteStream, Endianness, LittleEndian,
};
use enumflags2::{bitflags, BitFlags};
use num_traits::Signed;
use parse_display::Display;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::cmp::min;
use std::convert::{TryFrom, TryInto};
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::Hash;
use std::ops::{BitOr, Deref};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitWrite, PartialEq, Eq, Hash, Debug, Display, Clone, Serialize, Deserialize, Ord, PartialOrd,
)]
pub struct SendPropName(Cow<'static, str>);

impl SendPropName {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<E: Endianness> BitRead<'_, E> for SendPropName {
    fn read(stream: &mut BitReadStream<'_, E>) -> bitbuffer::Result<Self> {
        String::read(stream).map(SendPropName::from)
    }
}

impl PartialEq<&str> for SendPropName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl From<String> for SendPropName {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<&'static str> for SendPropName {
    fn from(value: &'static str) -> Self {
        SendPropName(Cow::Borrowed(value))
    }
}

impl AsRef<str> for SendPropName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for SendPropName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawSendPropDefinition {
    pub prop_type: SendPropType,
    pub name: SendPropName,
    pub identifier: SendPropIdentifier,
    pub flags: SendPropFlags,
    pub table_name: Option<SendTableName>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub bit_count: Option<u32>,
    pub element_count: Option<u16>,
    pub array_property: Option<Box<RawSendPropDefinition>>,
    pub original_bit_count: Option<u32>,
}

impl PartialEq for RawSendPropDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.identifier() == other.identifier()
    }
}

impl fmt::Display for RawSendPropDefinition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.prop_type {
            SendPropType::Vector | SendPropType::VectorXY => write!(
                f,
                "{}({})(flags: {}, low: {}, high: {}, bits: {})",
                self.name,
                self.prop_type,
                self.flags,
                self.low_value.unwrap_or_default(),
                self.high_value.unwrap_or_default(),
                self.bit_count.unwrap_or(96) / 3
            ),
            SendPropType::Float => write!(
                f,
                "{}({})(flags: {}, low: {}, high: {}, bits: {})",
                self.name,
                self.prop_type,
                self.flags,
                self.low_value.unwrap_or_default(),
                self.high_value.unwrap_or_default(),
                self.bit_count.unwrap_or(32)
            ),
            SendPropType::Int => write!(
                f,
                "{}({})(flags: {}, bits: {})",
                self.name,
                self.prop_type,
                self.flags,
                self.bit_count.unwrap_or(32)
            ),
            SendPropType::String => {
                write!(f, "{}({})", self.name, self.prop_type)
            }
            SendPropType::Array => match &self.array_property {
                Some(array_prop) => write!(
                    f,
                    "{}([{}({})] * {})",
                    self.name,
                    array_prop.prop_type,
                    array_prop.flags,
                    self.element_count.unwrap_or_default(),
                ),
                None => write!(f, "{}(Malformed array)", self.name),
            },
            SendPropType::DataTable => match &self.table_name {
                Some(sub_table) => write!(f, "{}(DataTable = {})", self.name, sub_table),
                None => write!(f, "{}(Malformed DataTable)", self.name),
            },
            SendPropType::NumSendPropTypes => {
                write!(f, "{}(NumSendPropTypes)", self.name)
            }
        }
    }
}

impl RawSendPropDefinition {
    pub fn identifier(&self) -> SendPropIdentifier {
        self.identifier
    }

    pub fn with_array_property(self, array_property: Self) -> Self {
        RawSendPropDefinition {
            prop_type: self.prop_type,
            identifier: self.identifier,
            name: self.name,
            flags: self.flags,
            table_name: self.table_name,
            low_value: self.low_value,
            high_value: self.high_value,
            bit_count: self.bit_count,
            element_count: self.element_count,
            array_property: Some(Box::new(array_property)),
            original_bit_count: self.original_bit_count,
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

    pub fn read(stream: &mut Stream, owner_table: &SendTableName) -> ReadResult<Self> {
        let prop_type = SendPropType::read(stream)?;
        let name: SendPropName = stream.read()?;
        let identifier = SendPropIdentifier::new(owner_table.as_str(), name.as_str());
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
        let original_bit_count = bit_count;

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
            name,
            identifier,
            flags,
            table_name,
            low_value,
            high_value,
            bit_count,
            element_count,
            original_bit_count,
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

impl BitWrite<LittleEndian> for RawSendPropDefinition {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.prop_type.write(stream)?;
        self.name.write(stream)?;
        self.flags.write(stream)?;

        if let Some(table_name) = self.table_name.as_ref() {
            table_name.write(stream)?;
        }
        if let Some(element_count) = self.element_count {
            element_count.write_sized(stream, 10)?;
        }
        if let (Some(low_value), Some(high_value), Some(bit_count)) =
            (self.low_value, self.high_value, self.original_bit_count)
        {
            low_value.write(stream)?;
            high_value.write(stream)?;
            bit_count.write_sized(stream, 7)?;
        }

        Ok(())
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Copy, Clone, PartialEq, Debug, Display, Serialize, Deserialize)]
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

#[bitflags]
#[derive(Copy, Clone, PartialEq, Debug)]
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

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SendPropFlags(BitFlags<SendPropFlag>);

#[cfg(feature = "schemars")]
impl schemars::JsonSchema for SendPropFlags {
    fn schema_name() -> String {
        "SendPropFlags".into()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        u16::json_schema(gen)
    }
}

impl BitOr<SendPropFlag> for SendPropFlags {
    type Output = SendPropFlags;

    fn bitor(self, rhs: SendPropFlag) -> Self::Output {
        Self(self.0 | rhs)
    }
}

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

impl BitWrite<LittleEndian> for SendPropFlags {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.0.bits().write(stream)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendPropDefinition {
    pub identifier: SendPropIdentifier,
    pub parse_definition: SendPropParseDefinition,
}

impl TryFrom<&RawSendPropDefinition> for SendPropDefinition {
    type Error = MalformedSendPropDefinitionError;

    fn try_from(definition: &RawSendPropDefinition) -> std::result::Result<Self, Self::Error> {
        let parse_definition = definition.try_into()?;
        Ok(SendPropDefinition {
            parse_definition,
            identifier: definition.identifier(),
        })
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SendPropParseDefinition {
    NormalVarInt {
        changes_often: bool,
        unsigned: bool,
    },
    UnsignedInt {
        changes_often: bool,
        bit_count: u8,
    },
    Int {
        changes_often: bool,
        bit_count: u8,
    },
    Float {
        changes_often: bool,
        definition: FloatDefinition,
    },
    String {
        changes_often: bool,
    },
    Vector {
        changes_often: bool,
        definition: FloatDefinition,
    },
    VectorXY {
        changes_often: bool,
        definition: FloatDefinition,
    },
    Array {
        changes_often: bool,
        inner_definition: Box<SendPropParseDefinition>,
        count_bit_count: u16,
    },
}

impl SendPropParseDefinition {
    pub fn changes_often(&self) -> bool {
        match self {
            SendPropParseDefinition::NormalVarInt { changes_often, .. } => *changes_often,
            SendPropParseDefinition::UnsignedInt { changes_often, .. } => *changes_often,
            SendPropParseDefinition::Int { changes_often, .. } => *changes_often,
            SendPropParseDefinition::Float { changes_often, .. } => *changes_often,
            SendPropParseDefinition::String { changes_often, .. } => *changes_often,
            SendPropParseDefinition::Vector { changes_often, .. } => *changes_often,
            SendPropParseDefinition::VectorXY { changes_often, .. } => *changes_often,
            SendPropParseDefinition::Array { changes_often, .. } => *changes_often,
        }
    }
}

impl TryFrom<&RawSendPropDefinition> for SendPropParseDefinition {
    type Error = MalformedSendPropDefinitionError;

    fn try_from(definition: &RawSendPropDefinition) -> std::result::Result<Self, Self::Error> {
        let changes_often = definition.flags.contains(SendPropFlag::ChangesOften);
        match definition.prop_type {
            SendPropType::Int => {
                if definition.flags.contains(SendPropFlag::NormalVarInt) {
                    Ok(SendPropParseDefinition::NormalVarInt {
                        changes_often,
                        unsigned: definition.flags.contains(SendPropFlag::Unsigned),
                    })
                } else if definition.flags.contains(SendPropFlag::Unsigned) {
                    Ok(SendPropParseDefinition::UnsignedInt {
                        changes_often,
                        bit_count: definition.bit_count.unwrap_or(32) as u8,
                    })
                } else {
                    Ok(SendPropParseDefinition::Int {
                        changes_often,
                        bit_count: definition.bit_count.unwrap_or(32) as u8,
                    })
                }
            }
            SendPropType::Float => Ok(SendPropParseDefinition::Float {
                changes_often,
                definition: FloatDefinition::new(
                    definition.flags,
                    definition.bit_count,
                    definition.high_value,
                    definition.low_value,
                )?,
            }),
            SendPropType::String => Ok(SendPropParseDefinition::String { changes_often }),
            SendPropType::Vector => Ok(SendPropParseDefinition::Vector {
                changes_often,
                definition: FloatDefinition::new(
                    definition.flags,
                    definition.bit_count,
                    definition.high_value,
                    definition.low_value,
                )?,
            }),
            SendPropType::VectorXY => Ok(SendPropParseDefinition::VectorXY {
                changes_often,
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
                Ok(SendPropParseDefinition::Array {
                    changes_often,
                    inner_definition: Box::new(SendPropParseDefinition::try_from(
                        child_definition,
                    )?),
                    count_bit_count,
                })
            }
            _ => Err(MalformedSendPropDefinitionError::InvalidPropType),
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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
            SendPropValue::Vector(vector) => Display::fmt(vector, f),
            SendPropValue::VectorXY(vector) => Display::fmt(vector, f),
            SendPropValue::Integer(int) => Display::fmt(int, f),
            SendPropValue::Float(float) => Display::fmt(float, f),
            SendPropValue::String(string) => Display::fmt(string, f),
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

fn float_scale(bit_count: u8) -> f32 {
    // is this -1 correct?, it is consistent with the js version but seems weird
    (1i32.wrapping_shl(bit_count as u32)) as f32 - 1.0
}

impl SendPropValue {
    pub fn parse(stream: &mut Stream, definition: &SendPropParseDefinition) -> Result<Self> {
        match definition {
            SendPropParseDefinition::NormalVarInt { unsigned, .. } => {
                read_var_int(stream, !*unsigned)
                    .map_err(ParseError::from)
                    .map(|int| int as i64)
                    .map(SendPropValue::from)
            }
            SendPropParseDefinition::UnsignedInt { bit_count, .. } => {
                Ok((stream.read_sized::<u32>(*bit_count as usize)? as i64).into())
            }
            SendPropParseDefinition::Int { bit_count, .. } => stream
                .read_int::<i32>((*bit_count) as usize)
                .map_err(ParseError::from)
                .map(SendPropValue::from),
            SendPropParseDefinition::Float {
                definition: float_definition,
                ..
            } => Self::read_float(stream, float_definition).map(SendPropValue::from),
            SendPropParseDefinition::String { .. } => {
                let length = stream.read_int(9)?;
                stream
                    .read_sized::<String>(length)
                    .map_err(ParseError::from)
                    .map(SendPropValue::from)
            }
            SendPropParseDefinition::Vector {
                definition: float_definition,
                ..
            } => Ok(Vector {
                x: Self::read_float(stream, float_definition)?,
                y: Self::read_float(stream, float_definition)?,
                z: Self::read_float(stream, float_definition)?,
            }
            .into()),
            SendPropParseDefinition::VectorXY {
                definition: float_definition,
                ..
            } => Ok(VectorXY {
                x: Self::read_float(stream, float_definition)?,
                y: Self::read_float(stream, float_definition)?,
            }
            .into()),
            SendPropParseDefinition::Array {
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
    pub fn encode(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &SendPropParseDefinition,
    ) -> Result<()> {
        match definition {
            SendPropParseDefinition::NormalVarInt { unsigned, .. } => {
                let val: i64 = self.try_into()?;
                write_var_int(val as i32, stream, !*unsigned)?;
                Ok(())
            }
            SendPropParseDefinition::UnsignedInt { bit_count, .. } => {
                let val: i64 = self.try_into()?;
                (val as u32).write_sized(stream, *bit_count as usize)?;
                Ok(())
            }
            SendPropParseDefinition::Int { bit_count, .. } => {
                let val: i64 = self.try_into()?;
                (val as i32).write_sized(stream, *bit_count as usize)?;
                Ok(())
            }
            SendPropParseDefinition::Float {
                definition: float_definition,
                ..
            } => {
                let val: f32 = self.try_into()?;
                Self::write_float(val, stream, float_definition)
            }
            SendPropParseDefinition::String { .. } => {
                let val: &str = self.try_into()?;
                (val.len() as u16).write_sized(stream, 9)?;
                val.write_sized(stream, val.len())?;
                Ok(())
            }
            SendPropParseDefinition::Vector {
                definition: float_definition,
                ..
            } => {
                let val: Vector = self.try_into()?;
                Self::write_float(val.x, stream, float_definition)?;
                Self::write_float(val.y, stream, float_definition)?;
                Self::write_float(val.z, stream, float_definition)?;
                Ok(())
            }
            SendPropParseDefinition::VectorXY {
                definition: float_definition,
                ..
            } => {
                let val: VectorXY = self.try_into()?;
                Self::write_float(val.x, stream, float_definition)?;
                Self::write_float(val.y, stream, float_definition)?;
                Ok(())
            }
            SendPropParseDefinition::Array {
                count_bit_count,
                inner_definition,
                ..
            } => {
                let array: &[SendPropValue] = self.try_into()?;
                (array.len() as u16).write_sized(stream, *count_bit_count as usize)?;

                for inner in array {
                    inner.encode(stream, inner_definition)?
                }

                Ok(())
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
                let scale = float_scale(*bit_count);
                let percentage = (raw as f32) / scale;
                Ok(low + ((high - low) * percentage))
            }
        }
    }

    fn write_float(
        val: f32,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &FloatDefinition,
    ) -> Result<()> {
        match definition {
            FloatDefinition::Coord => write_bit_coord(val, stream).map_err(ParseError::from),
            FloatDefinition::CoordMP => {
                write_bit_coord_mp(val, stream, false, false).map_err(ParseError::from)
            }
            FloatDefinition::CoordMPLowPrecision => {
                write_bit_coord_mp(val, stream, false, true).map_err(ParseError::from)
            }
            FloatDefinition::CoordMPIntegral => {
                write_bit_coord_mp(val, stream, true, false).map_err(ParseError::from)
            }
            FloatDefinition::FloatNoScale => val.write(stream).map_err(ParseError::from),
            FloatDefinition::NormalVarFloat => {
                write_bit_normal(val, stream).map_err(ParseError::from)
            }
            FloatDefinition::Scaled {
                bit_count,
                low,
                high,
            } => {
                let percentage = (val - low) / (high - low);
                let scale = float_scale(*bit_count);
                let raw = (percentage * scale).round() as u32;
                raw.write_sized(stream, *bit_count as usize)?;

                Ok(())
            }
        }
    }
}

#[test]
fn test_send_prop_value_roundtrip() {
    use bitbuffer::{BitReadBuffer, BitReadStream};

    fn send_prop_value_roundtrip(val: SendPropValue, def: SendPropParseDefinition) {
        let mut data = Vec::new();
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            val.encode(&mut write, &def).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(val, SendPropValue::parse(&mut read, &def).unwrap());
        assert_eq!(pos, read.pos());
    }
    send_prop_value_roundtrip(
        SendPropValue::Integer(0),
        SendPropParseDefinition::UnsignedInt {
            changes_often: false,
            bit_count: 5,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Integer(12),
        SendPropParseDefinition::NormalVarInt {
            changes_often: false,
            unsigned: false,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Integer(12),
        SendPropParseDefinition::NormalVarInt {
            changes_often: false,
            unsigned: false,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Integer(-12),
        SendPropParseDefinition::NormalVarInt {
            changes_often: false,
            unsigned: true,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::String("foobar".into()),
        SendPropParseDefinition::String {
            changes_often: false,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Vector(Vector {
            x: 1.0,
            y: 0.0,
            z: 1.125,
        }),
        SendPropParseDefinition::Vector {
            changes_often: false,
            definition: FloatDefinition::Coord,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::VectorXY(VectorXY { x: 1.0, y: 0.0 }),
        SendPropParseDefinition::VectorXY {
            changes_often: false,
            definition: FloatDefinition::FloatNoScale,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Float(12.5),
        SendPropParseDefinition::Float {
            changes_often: false,
            definition: FloatDefinition::CoordMP,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Float(12.0),
        SendPropParseDefinition::Float {
            changes_often: false,
            definition: FloatDefinition::CoordMPIntegral,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Float(12.5),
        SendPropParseDefinition::Float {
            changes_often: false,
            definition: FloatDefinition::CoordMPLowPrecision,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Float(12.498169),
        SendPropParseDefinition::Float {
            changes_often: false,
            definition: FloatDefinition::Scaled {
                bit_count: 12,
                high: 25.0,
                low: 10.0,
            },
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Array(vec![
            SendPropValue::Integer(0),
            SendPropValue::Integer(1),
            SendPropValue::Integer(2),
        ]),
        SendPropParseDefinition::Array {
            changes_often: false,
            inner_definition: Box::new(SendPropParseDefinition::UnsignedInt {
                changes_often: false,
                bit_count: 3,
            }),
            count_bit_count: 5,
        },
    );

    send_prop_value_roundtrip(
        SendPropValue::Float(76.22549),
        SendPropParseDefinition::Float {
            changes_often: false,
            definition: FloatDefinition::Scaled {
                bit_count: 10,
                high: 102.3,
                low: 0.09990235,
            },
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Vector(Vector {
            x: 1.0,
            y: -25.96875,
            z: 0.1875,
        }),
        SendPropParseDefinition::Vector {
            changes_often: false,
            definition: FloatDefinition::CoordMP,
        },
    );
    send_prop_value_roundtrip(
        SendPropValue::Integer(-1),
        SendPropParseDefinition::NormalVarInt {
            changes_often: false,
            unsigned: false,
        },
    );
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
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Integer(val) => Ok(*val),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "integer",
                value: value.clone(),
            }),
        }
    }
}

impl TryFrom<&SendPropValue> for Vector {
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Vector(val) => Ok(*val),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "vector",
                value: value.clone(),
            }),
        }
    }
}

impl TryFrom<&SendPropValue> for VectorXY {
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::VectorXY(val) => Ok(*val),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "vectorxy",
                value: value.clone(),
            }),
        }
    }
}

impl TryFrom<&SendPropValue> for f32 {
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Float(val) => Ok(*val),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "float",
                value: value.clone(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a SendPropValue> for &'a str {
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &'a SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::String(val) => Ok(val.as_str()),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "string",
                value: value.clone(),
            }),
        }
    }
}

impl<'a> TryFrom<&'a SendPropValue> for &'a [SendPropValue] {
    type Error = MalformedSendPropDefinitionError;
    fn try_from(value: &'a SendPropValue) -> std::result::Result<Self, Self::Error> {
        match value {
            SendPropValue::Array(val) => Ok(val.as_slice()),
            _ => Err(MalformedSendPropDefinitionError::WrongPropType {
                expected: "array",
                value: value.clone(),
            }),
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SendPropIdentifier(u64);

impl SendPropIdentifier {
    pub const fn new(table: &str, prop: &str) -> Self {
        let hasher = ConstFnvHash::new().push_string(table).push_string(prop);
        SendPropIdentifier(hasher.finish())
    }

    /// This returns an option because only props known at compile time will return a name here
    ///
    /// If you need to know the name of every property you need to keep a map yourself
    pub fn table_name(&self) -> Option<SendTableName> {
        get_prop_names(*self).map(|(table, _)| table.into())
    }

    /// This returns an option because only props known at compile time will return a name here
    ///
    /// If you need to know the name of every property you need to keep a map yourself
    pub fn prop_name(&self) -> Option<SendPropName> {
        get_prop_names(*self).map(|(_, prop)| prop.into())
    }

    /// This returns an option because only props known at compile time will return a name here
    ///
    /// If you need to know the name of every property you need to keep a map yourself
    pub fn names(&self) -> Option<(SendTableName, SendPropName)> {
        get_prop_names(*self).map(|(table, prop)| (table.into(), prop.into()))
    }
}

impl From<u64> for SendPropIdentifier {
    fn from(raw: u64) -> Self {
        SendPropIdentifier(raw)
    }
}

impl From<SendPropIdentifier> for u64 {
    fn from(identifier: SendPropIdentifier) -> Self {
        identifier.0
    }
}

impl Display for SendPropIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match get_prop_names(*self) {
            Some((table, prop)) => write!(f, "{}.{}", table, prop),
            None => write!(f, "Prop name {} not known", self.0),
        }
    }
}

impl<'de> Deserialize<'de> for SendPropIdentifier {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Options<'a> {
            Num(u64),
            Str(&'a str),
        }

        let raw = Options::deserialize(deserializer)?;
        Ok(match raw {
            Options::Num(num) => SendPropIdentifier(num),
            Options::Str(s) => {
                let num: u64 = s.parse().map_err(|e| D::Error::custom(e))?;
                SendPropIdentifier(num)
            }
        })
    }
}

impl Serialize for SendPropIdentifier {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Display, PartialEq, Serialize, Deserialize)]
#[display("{index} = {value}")]
pub struct SendProp {
    pub index: u32,
    pub identifier: SendPropIdentifier,
    pub value: SendPropValue,
}

impl Debug for SendProp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.identifier, self.value)
    }
}

pub fn read_var_int(stream: &mut Stream, signed: bool) -> ReadResult<i32> {
    let abs_int = crate::demo::message::stringtable::read_var_int(stream)? as i32;

    if signed {
        Ok((abs_int >> 1) ^ -(abs_int & 1))
    } else {
        Ok(abs_int)
    }
}

pub fn write_var_int(
    int: i32,
    stream: &mut BitWriteStream<LittleEndian>,
    signed: bool,
) -> ReadResult<()> {
    let abs = if signed {
        let int = (int << 1) ^ (int >> 31);
        u32::from_le_bytes(int.to_le_bytes())
    } else {
        int as u32
    };

    crate::demo::message::stringtable::write_var_int(abs, stream)
}

#[test]
fn test_var_int_roundtrip() {
    use bitbuffer::{BitReadBuffer, BitReadStream};

    fn var_int_roundtrip(int: i32, signed: bool) {
        let mut data = Vec::new();
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_var_int(int, &mut write, signed).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(int, read_var_int(&mut read, signed).unwrap());
        assert_eq!(pos, read.pos());
    }
    var_int_roundtrip(0, false);
    var_int_roundtrip(1, false);
    var_int_roundtrip(10, false);
    var_int_roundtrip(55, false);
    var_int_roundtrip(355, false);
    var_int_roundtrip(12354, false);
    var_int_roundtrip(123125412, false);

    var_int_roundtrip(0, true);
    var_int_roundtrip(1, true);
    var_int_roundtrip(10, true);
    var_int_roundtrip(55, true);
    var_int_roundtrip(355, true);
    var_int_roundtrip(12354, true);
    var_int_roundtrip(123125412, true);
    var_int_roundtrip(-0, true);
    var_int_roundtrip(-1, true);
    var_int_roundtrip(-10, true);
    var_int_roundtrip(-55, true);
    var_int_roundtrip(-355, true);
    var_int_roundtrip(-12354, true);
    var_int_roundtrip(-123125412, true);
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

pub fn write_bit_coord(val: f32, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
    let has_int = val.abs() >= 1.0;
    has_int.write(stream)?;
    let has_frac = val.fract() != 0.0;
    has_frac.write(stream)?;

    if has_frac || has_int {
        let sign = val.is_negative();
        sign.write(stream)?;
    }
    let abs = val.abs();
    if has_int {
        (abs as u16 - 1).write_sized(stream, 14)?;
    }
    if has_frac {
        let frac_val = (abs.fract() / get_frac_factor(5)) as u8;
        frac_val.write_sized(stream, 5)?;
    }
    Ok(())
}

#[test]
fn bit_coord_roundtrip() {
    use bitbuffer::BitReadBuffer;

    let mut data = Vec::with_capacity(16);
    let (pos1, pos2, pos3, pos4) = {
        let mut write = BitWriteStream::new(&mut data, LittleEndian);
        write_bit_coord(0.0, &mut write).unwrap();
        let pos1 = write.bit_len();
        write_bit_coord(123.0, &mut write).unwrap();
        let pos2 = write.bit_len();
        write_bit_coord(123.4375, &mut write).unwrap();
        let pos3 = write.bit_len();
        write_bit_coord(-0.4375, &mut write).unwrap();
        let pos4 = write.bit_len();
        (pos1, pos2, pos3, pos4)
    };

    let mut read = Stream::from(BitReadBuffer::new(&data, LittleEndian));
    assert_eq!(0.0, read_bit_coord(&mut read).unwrap());
    assert_eq!(pos1, read.pos());
    assert_eq!(123.0, read_bit_coord(&mut read).unwrap());
    assert_eq!(pos2, read.pos());
    assert_eq!(123.4375, read_bit_coord(&mut read).unwrap());
    assert_eq!(pos3, read.pos());
    assert_eq!(-0.4375, read_bit_coord(&mut read).unwrap());
    assert_eq!(pos4, read.pos());
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

pub fn write_bit_coord_mp(
    val: f32,
    stream: &mut BitWriteStream<LittleEndian>,
    is_integral: bool,
    low_precision: bool,
) -> ReadResult<()> {
    let abs = val.abs();
    let in_bounds = (abs as u32) < (1 << 11);
    let has_int_val = abs >= 1.0;
    in_bounds.write(stream)?;
    has_int_val.write(stream)?;

    if is_integral {
        if has_int_val {
            val.is_sign_negative().write(stream)?;
            ((abs - 1.0) as u32).write_sized(stream, if in_bounds { 11 } else { 14 })?;
        }
    } else {
        val.is_sign_negative().write(stream)?;
        if has_int_val {
            ((abs - 1.0) as u32).write_sized(stream, if in_bounds { 11 } else { 14 })?;
        }
        let frac_bits = if low_precision { 3 } else { 5 };
        let frac_val = (abs.fract() / get_frac_factor(frac_bits)) as u32;
        frac_val.write_sized(stream, frac_bits)?;
    }

    Ok(())
}

#[test]
fn test_bit_coord_mp_roundtrip() {
    use bitbuffer::{BitReadBuffer, BitReadStream};

    fn bit_coord_mp_normal(val: f32, is_integral: bool, low_precision: bool) {
        let mut data = Vec::with_capacity(16);
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_bit_coord_mp(val, &mut write, is_integral, low_precision).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(
            val,
            read_bit_coord_mp(&mut read, is_integral, low_precision).unwrap()
        );
        assert_eq!(pos, read.pos());
    }

    bit_coord_mp_normal(1.0, false, false);

    bit_coord_mp_normal(0.0, false, false);
    bit_coord_mp_normal(0.5, false, false);
    bit_coord_mp_normal(-0.5, false, false);
    bit_coord_mp_normal(1234.5, false, false);
    bit_coord_mp_normal(-1234.5, false, false);
    bit_coord_mp_normal(2.0f32.powf(12.0) + 0.125, false, false);

    bit_coord_mp_normal(0.0, false, true);
    bit_coord_mp_normal(0.5, false, true);
    bit_coord_mp_normal(-0.5, false, true);
    bit_coord_mp_normal(1234.5, false, true);
    bit_coord_mp_normal(-1234.5, false, true);
    bit_coord_mp_normal(2.0f32.powf(12.0) + 0.125, false, true);

    bit_coord_mp_normal(0.0, true, false);
    bit_coord_mp_normal(1234.0, true, false);
    bit_coord_mp_normal(-1234.0, true, false);
    bit_coord_mp_normal(2.0f32.powf(12.0), true, false);
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

pub fn write_bit_normal(val: f32, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
    val.is_sign_negative().write(stream)?;
    let frac_val = (val.abs().fract() / get_frac_factor(11)) as u16;
    frac_val.write_sized(stream, 11)
}

#[test]
fn test_bit_normal_roundtrip() {
    use bitbuffer::{BitReadBuffer, BitReadStream};

    fn roundtrip_normal(val: f32) {
        let mut data = Vec::with_capacity(16);
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_bit_normal(val, &mut write).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(val, read_bit_normal(&mut read).unwrap());
        assert_eq!(pos, read.pos());
    }
    roundtrip_normal(0.0);
    roundtrip_normal(-0.0);
    roundtrip_normal(0.5);
    roundtrip_normal(-0.5);
}
