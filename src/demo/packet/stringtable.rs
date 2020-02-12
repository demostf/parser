use std::fmt;

use bitbuffer::{BitRead, LittleEndian};

use crate::demo::message::stringtable::StringTableMeta;
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use std::cmp::min;

#[derive(BitRead, Clone, Copy, Debug)]
pub struct FixedUserDataSize {
    #[size = 12]
    pub size: u16,
    #[size = 4]
    pub bits: u8,
}

#[derive(Debug)]
pub struct StringTable {
    pub name: String,
    pub entries: Vec<(u16, StringTableEntry)>,
    pub max_entries: u16,
    pub fixed_user_data_size: Option<FixedUserDataSize>,
    pub client_entries: Option<Vec<StringTableEntry>>,
    pub compressed: bool,
}

impl StringTable {
    pub fn get_table_meta(&self) -> StringTableMeta {
        StringTableMeta {
            fixed_userdata_size: self.fixed_user_data_size,
            max_entries: self.max_entries,
        }
    }
}

impl BitRead<LittleEndian> for StringTable {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let name = stream.read()?;
        let entry_count = stream.read_int(16)?;
        let mut entries = Vec::with_capacity(min(entry_count, 128) as usize);

        for index in 0..entry_count {
            entries.push((index, stream.read()?))
        }

        let client_entries = if stream.read_bool()? {
            let count = stream.read_int(16)?;
            Some(stream.read_sized(count)?)
        } else {
            None
        };

        Ok(StringTable {
            name,
            entries,
            max_entries: entry_count,
            fixed_user_data_size: None,
            client_entries,
            compressed: false,
        })
    }
}

#[derive(BitRead, Clone, Debug)]
#[endianness = "LittleEndian"]
pub struct ExtraData {
    pub byte_len: u16,
    #[size = "byte_len.saturating_mul(8)"]
    pub data: Stream,
}

impl ExtraData {
    pub fn new(data: Stream) -> Self {
        let byte_len = (data.bit_len() / 8) as u16;
        ExtraData { byte_len, data }
    }
}

#[derive(Clone, Default)]
pub struct StringTableEntry {
    pub text: Option<String>,
    pub extra_data: Option<ExtraData>,
}

impl StringTableEntry {
    pub fn text(&self) -> &str {
        self.text.as_ref().map(|s| s.as_str()).unwrap_or("")
    }
}

impl BitRead<LittleEndian> for StringTableEntry {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        Ok(StringTableEntry {
            text: Some(stream.read()?),
            extra_data: stream.read()?,
        })
    }
}

impl fmt::Debug for StringTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.extra_data {
            None => write!(f, "StringTableEntry {{ text: \"{}\" }}", self.text()),
            Some(extra_data) => write!(
                f,
                "StringTableEntry{{ text: \"{}\" extra_data: {} bytes }}",
                self.text(),
                extra_data.byte_len
            ),
        }
    }
}

#[derive(Debug)]
pub struct StringTablePacket {
    pub tick: u32,
    pub tables: Vec<StringTable>,
}

impl Parse for StringTablePacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read_int(32)?;
        let length: usize = stream.read_int(32)?;
        let mut packet_data = stream.read_bits(length.saturating_mul(8))?;
        let count: usize = packet_data.read_int(8)?;
        let tables = packet_data.read_sized(count)?;

        if packet_data.bits_left() > 7 {
            Err(ParseError::DataRemaining(packet_data.bits_left()))
        } else {
            Ok(StringTablePacket { tick, tables })
        }
    }
}
