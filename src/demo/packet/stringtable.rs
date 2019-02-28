use std::fmt;

use bitstream_reader::{BitRead, LittleEndian};

use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use crate::demo::sendprop::SendPropFlag::Exclude;

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
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let name = stream.read()?;
        let entry_count = stream.read_int(16)?;
        let entries = stream.read_sized(entry_count as usize)?;

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
            fixed_userdata_size: None,
            fixed_userdata_size_bits: None,
            client_entries,
            compressed: false,
        })
    }
}

#[derive(BitRead)]
#[endianness = "LittleEndian"]
pub struct ExtraData {
    len: u16,
    #[size = "len * 8"]
    data: Stream,
}

#[derive(BitRead)]
#[endianness = "LittleEndian"]
pub struct StringTableEntry {
    text: String,
    extra_data: Option<ExtraData>,
}

impl fmt::Debug for StringTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.extra_data {
            None => write!(f, "Table Entry: '{}'", self.text),
            Some(extra_data) => write!(
                f,
                "StringTableEntry{{ '{}' with {} bits of extra data }}",
                self.text,
                extra_data.len
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
        let count: usize = packet_data.read_int(8)?;
        let tables = packet_data.read_sized(count)?;

        if packet_data.bits_left() > 7 {
            Err(ParseError::DataRemaining(packet_data.bits_left()))
        } else {
            Ok(StringTablePacket { tick, tables })
        }
    }
}
