use bitstream_reader::{BitRead, LittleEndian};

use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};
use std::fmt;

#[derive(Debug)]
pub struct StringTable {
    name: String,
    entries: Vec<StringTableEntry>,
    max_entries: u32,
    fixed_userdata_size: Option<u32>,
    fixed_userdata_size_bits: Option<u32>,
    client_entries: Option<Vec<StringTableEntry>>,
    compressed: bool,
}

impl BitRead<LittleEndian> for StringTable {
    fn read(stream: &mut Stream) -> ReadResult <Self> {
        let name = stream.read_string(None)?;
        let entry_count: u32 = stream.read_int(16)?;
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(StringTableEntry::read(stream)?);
        }

        let client_entries = if stream.read_bool()? {
            let count = stream.read_int(16)?;
            let mut vec = Vec::with_capacity(count);
            for _ in 0..count {
                vec.push(StringTableEntry::read(stream)?);
            }
            Some(vec)
        } else {
            None
        };

        Ok(StringTable {
            name,
            entries,
            max_entries: entry_count,
            fixed_userdata_size: None,
            fixed_userdata_size_bits: None,
            client_entries,
            compressed: false,
        })
    }
}

pub struct StringTableEntry {
    text: String,
    extra_data: Option<Stream>,
}

impl BitRead<LittleEndian> for StringTableEntry {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let text = stream.read_string(None)?;
        let extra_data = if stream.read_bool()? {
            let byte_len: usize = stream.read_int(16)?;
            Some(stream.read_bits(byte_len * 8)?)
        } else {
            None
        };
        Ok(StringTableEntry { text, extra_data })
    }
}

impl fmt::Debug for StringTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.extra_data {
            None => write!(f, "Table Entry: '{}'", self.text),
            Some(extra_data) => write!(
                f,
                "Table Entry: '{}' with {} bits of extra data",
                self.text,
                extra_data.bit_len()
            ),
        }
    }
}

#[derive(Debug)]
pub struct StringTablePacket {
    tick: u32,
    tables: Vec<StringTable>,
}

impl Parse for StringTablePacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read_int(32)?;
        let start = stream.pos();
        let length: usize = stream.read_int(32)?;
        let mut packet_data = stream.read_bits(length * 8)?;
        let count: u32 = packet_data.read_int(8)?;
        let mut tables = Vec::with_capacity(count as usize);
        for _ in 0..count {
            tables.push(StringTable::read(&mut packet_data)?);
        }
        if packet_data.bits_left() > 7 {
            Err(ParseError::DataRemaining(packet_data.bits_left()))
        } else {
            Ok(StringTablePacket { tick, tables })
        }
    }
}
