use bitstream_reader::{BitRead, LittleEndian};
use enumflags2::BitFlags;
use enumflags2_derive::EnumFlags;

use crate::{ReadResult, Result, Stream};

use super::packet::datatable::SendTable;
use super::vector::{Vector, VectorXY};

#[derive(Debug)]
pub struct SendPropDefinition {
    pub prop_type: SendPropType,
    pub name: String,
    pub flags: SendPropFlags,
    pub exclude_dt_name: Option<String>,
    pub low_value: Option<f32>,
    pub high_value: Option<f32>,
    pub bit_count: Option<u32>,
    pub original_bit_count: Option<u32>,
    pub table: Option<SendTable>,
    pub element_count: Option<u16>,
    pub array_property: Option<Box<SendPropDefinition>>,
    pub owner_table_name: String,
}

impl SendPropDefinition {
    pub fn read(stream: &mut Stream, owner_table_name: String) -> Result<Self> {
        let prop_type = SendPropType::read(stream)?;
        let name = stream.read_string(None)?;
        let flags = SendPropFlags::read(stream)?;
        let mut exclude_dt_name = None;
        let mut element_count = None;
        let mut low_value = None;
        let mut high_value = None;
        let mut bit_count = None;
        if prop_type == SendPropType::DataTable {
            exclude_dt_name = Some(stream.read_string(None)?);
        } else {
            if flags.contains(SendPropFlag::Exclude) {
                exclude_dt_name = Some(stream.read_string(None)?);
            } else if prop_type == SendPropType::Array {
                element_count = Some(stream.read_int(10)?);
            } else {
                low_value = Some(stream.read_float()?);
                high_value = Some(stream.read_float()?);
                bit_count = Some(stream.read_int(7)?);
            }
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

        Ok(SendPropDefinition {
            prop_type,
            name,
            flags,
            exclude_dt_name,
            low_value,
            high_value,
            bit_count,
            original_bit_count,
            table: None,
            element_count,
            array_property: None,
            owner_table_name,
        })
    }

    pub fn with_array_property(self, array_property: Self) -> Self {
        SendPropDefinition {
            prop_type: self.prop_type,
            name: self.name,
            flags: self.flags,
            exclude_dt_name: self.exclude_dt_name,
            low_value: self.low_value,
            high_value: self.high_value,
            bit_count: self.bit_count,
            original_bit_count: self.original_bit_count,
            table: None,
            element_count: self.element_count,
            array_property: Some(Box::new(array_property)),
            owner_table_name: self.owner_table_name,
        }
    }
}

#[derive(BitRead, Copy, Clone, PartialEq, Debug)]
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

#[derive(EnumFlags, Copy, Clone, PartialEq, Debug)]
#[repr(u16)]
pub enum SendPropFlag {
    Unsigned = 1,
    //(1 << 0)
    // Unsigned integer data.
    Coord = 2,
    //(1 << 1)
    // If this is set, the float/vector is treated like a world coordinate.
    // Note that the bit count is ignored in this case.
    NoScale = 4,
    //(1 << 2)
    // For floating point, don't scale into range, just take value as is.
    RoundDown = 8,
    //(1 << 3)
    // For floating point, limit high value to range minus one bit unit
    RoundUp = 16,
    //(1 << 4)
    // For floating point, limit low value to range minus one bit unit
    // Normal = 32, // seems to be depricated
    //(1 << 5)
    // If this is set, the vector is treated like a normal (only valid for vectors)
    Exclude = 64,
    //(1 << 6)
    // This is an exclude prop (not excludED, but it points at another prop to be excluded).
    XYZE = 128,
    //(1 << 7)
    // Use XYZ/Exponent encoding for vectors.
    InsideArray = 256,
    //(1 << 8)
    // This tells us that the property is inside an array, so it shouldn't be put into the
    // flattened property list. Its array will point at it when it needs to.
    PropxyAlwaysYes = 512,
    //(1 << 9)
    // Set for datatable props using one of the default datatable proxies like
    // SendProxy_DataTableToDataTable that always send the data to all clients.
    ChangesOften = 1024,
    //(1 << 10)
    // this is an often changed field, moved to head of sendtable so it gets a small index
    IsVectorElement = 2048,
    //(1 << 11)
    // Set automatically if SPROP_VECTORELEM is used.
    Collapsible = 4096,
    //(1 << 12)
    // Set automatically if it's a datatable with an offset of 0 that doesn't change the pointer
    // (ie: for all automatically-chained base classes).
    // In this case, it can get rid of this SendPropDataTable altogether and spare the
    // trouble of walking the hierarchy more than necessary.
    CoordMP = 8192,
    //(1 << 13)
    // Like SPROP_COORD, but special handling for multiplayer games
    CoordMPLowPercision = 16384,
    //(1 << 14)
    // Like SPROP_COORD, but special handling for multiplayer games
    // where the fractional component only gets a 3 bits instead of 5
    CoordMPIntegral = 32768,
    //(1 << 15)
    // SPROP_COORD_MP, but coordinates are rounded to integral boundaries
    // overloaded as both "Normal" and "VarInt"
    NormalVarInt = 32, //(1 << 5)
}

#[derive(Debug, Copy, Clone)]
pub struct SendPropFlags(BitFlags<SendPropFlag>);

impl SendPropFlags {
    pub fn contains(self, other: SendPropFlag) -> bool {
        self.0.contains(other)
    }
}

impl BitRead<LittleEndian> for SendPropFlags {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let raw = stream.read_int(16)?;
        // since all 16 bits worth of flags are used there are no invalid flags
        Ok(SendPropFlags(BitFlags::from_bits_truncate(raw)))
    }
}

#[derive(Debug, Clone)]
pub enum SendPropValue {
    Vector(Vector),
    VectorXY(VectorXY),
    Integer(i32),
    Float(f32),
    String(String),
    Array(Vec<SendPropValue>),
}

#[derive(Debug)]
pub struct SendProp {
    definition: SendPropDefinition,
    value: SendPropValue,
}

pub fn read_bit_coord(stream: &mut Stream) -> ReadResult<f32> {
    let has_int = stream.read()?;
    let has_frac = stream.read()?;

    Ok(if has_int || has_frac {
        let sign = if stream.read()? { -1f32 } else { 1f32 };
        let int_val: u16 = if has_int { stream.read_sized::<u16>(14)? + 1 } else { 0 };
        let frac_val: u8 = if has_frac { stream.read_sized(5)? } else { 0 };
        let value = int_val as f32 + (frac_val as f32 * (1f32 / 32f32));
        value * sign
    } else {
        0f32
    })
}