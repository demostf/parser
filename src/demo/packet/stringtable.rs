use crate::{Parse, ParseError, ParserState, Result, Stream};

pub struct StringTable<'a> {
    name: String,
    entries: Vec<StringTableEntry<'a>>,
    max_entries: u32,
    fixed_userdata_size: Option<u32>,
    fixed_userdata_size_bits: Option<u32>,
    client_entries: Option<Vec<StringTableEntry<'a>>>,
    compressed: bool,
}

impl<'a> StringTable<'a> {
    fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let name = stream.read_string(None)?;
        let entry_count: u32 = stream.read(16)?;
        let mut entries = Vec::with_capacity(entry_count as usize);
        for _ in 0..entry_count {
            entries.push(StringTableEntry::parse(stream)?);
        }

        let client_entries = if stream.read_bool()? {
            let count = stream.read(16)?;
            let mut vec = Vec::with_capacity(count);
            for _ in 0..count {
                vec.push(StringTableEntry::parse(stream)?);
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

pub struct StringTableEntry<'a> {
    text: String,
    extra_data: Option<Stream<'a>>,
}

impl<'a> StringTableEntry<'a> {
    fn parse(stream: &mut Stream<'a>) -> Result<Self> {
        let text = stream.read_string(None)?;
        let extra_data = if stream.read_bool()? {
            let byte_len: usize = stream.read(16)?;
            Some(stream.read_bits(byte_len * 8)?)
        } else {
            None
        };
        Ok(StringTableEntry {
            text,
            extra_data,
        })
    }
}

pub struct StringTablePacket<'a> {
    tick: u32,
    tables: Vec<StringTable<'a>>,
}

impl<'a> Parse<'a> for StringTablePacket<'a> {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState<'a>) -> Result<Self> {
        let tick = stream.read(32)?;
        let length: usize = stream.read(32)?;
        let count: u32 = stream.read(8)?;
        let mut tables = Vec::with_capacity(count as usize);
        for _ in 0..count {
            tables.push(StringTable::parse(stream)?);
        }
        Ok(StringTablePacket {
            tick,
            tables,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32)?;
        let length = stream.read(32)?;
        stream.skip(length).map_err(ParseError::from)
    }
}