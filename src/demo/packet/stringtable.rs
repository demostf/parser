use std::fmt;

use bitbuffer::{BitRead, LittleEndian};

use crate::demo::message::stringtable::StringTableMeta;
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use std::borrow::{Borrow, Cow};
use std::cmp::min;

#[derive(BitRead, Clone, Copy, Debug)]
pub struct FixedUserDataSize {
    #[size = 12]
    pub size: u16,
    #[size = 4]
    pub bits: u8,
}

#[derive(Debug)]
pub struct StringTable<'a> {
    pub name: Cow<'a, str>,
    pub entries: Vec<(u16, StringTableEntry<'a>)>,
    pub max_entries: u16,
    pub fixed_user_data_size: Option<FixedUserDataSize>,
    pub client_entries: Option<Vec<StringTableEntry<'a>>>,
    pub compressed: bool,
}

impl StringTable<'_> {
    pub fn get_table_meta(&self) -> StringTableMeta {
        StringTableMeta {
            fixed_userdata_size: self.fixed_user_data_size,
            max_entries: self.max_entries,
        }
    }
}

impl<'a> BitRead<'a, LittleEndian> for StringTable<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
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
pub struct ExtraData<'a> {
    pub byte_len: u16,
    #[size = "byte_len.saturating_mul(8)"]
    pub data: Stream<'a>,
}

impl<'a> ExtraData<'a> {
    pub fn new(data: Stream<'a>) -> Self {
        let byte_len = (data.bit_len() / 8) as u16;
        ExtraData { byte_len, data }
    }
}

#[derive(Clone, Default)]
pub struct StringTableEntry<'a> {
    pub text: Option<Cow<'a, str>>,
    pub extra_data: Option<ExtraData<'a>>,
}

impl StringTableEntry<'_> {
    pub fn text(&self) -> &str {
        self.text
            .as_ref()
            .map(|text| text.borrow())
            .unwrap_or_default()
    }
}

impl<'a> BitRead<'a, LittleEndian> for StringTableEntry<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        Ok(StringTableEntry {
            text: Some(stream.read()?),
            extra_data: stream.read()?,
        })
    }
}

impl fmt::Debug for StringTableEntry<'_> {
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
pub struct StringTablePacket<'a> {
    pub tick: u32,
    pub tables: Vec<StringTable<'a>>,
}

impl<'a> Parse<'a> for StringTablePacket<'a> {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState) -> Result<Self> {
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
