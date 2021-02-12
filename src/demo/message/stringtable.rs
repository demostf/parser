use bitbuffer::{BitReadBuffer, BitReadStream, LittleEndian};
use num_traits::{PrimInt, Unsigned};
use snap::raw::Decoder;

use crate::demo::packet::stringtable::{
    ExtraData, FixedUserDataSize, StringTable, StringTableEntry,
};
use crate::demo::parser::ParseBitSkip;
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use std::borrow::Cow;
use std::cmp::min;

#[derive(Debug)]
pub struct CreateStringTableMessage<'a> {
    pub table: Box<StringTable<'a>>,
}

#[derive(Debug, Clone)]
pub struct StringTableMeta {
    pub max_entries: u16,
    pub fixed_userdata_size: Option<FixedUserDataSize>,
}

impl From<&StringTable<'_>> for StringTableMeta {
    fn from(table: &StringTable) -> Self {
        StringTableMeta {
            max_entries: table.max_entries,
            fixed_userdata_size: table.fixed_user_data_size,
        }
    }
}

impl<'a> Parse<'a> for CreateStringTableMessage<'a> {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState) -> Result<Self> {
        let name = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = log_base2(max_entries);
        let entity_count: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let length = read_var_int(stream)?;

        let fixed_userdata_size = stream.read()?;

        let compressed = stream.read()?;

        let mut table_data = stream.read_bits(length as usize)?;

        if compressed {
            let decompressed_size: u32 = table_data.read()?;
            let compressed_size: u32 = table_data.read()?;

            let magic = table_data.read_string(Some(4))?;

            if magic != "SNAP" {
                return Err(ParseError::UnexpectedCompressionType(magic.into_owned()));
            }

            let compressed_data = table_data.read_bytes(compressed_size as usize - 4)?;

            let mut decoder = Decoder::new();
            let decompressed_data = decoder
                .decompress_vec(&compressed_data)
                .map_err(ParseError::from)?;

            if decompressed_data.len() != decompressed_size as usize {
                return Err(ParseError::UnexpectedDecompressedSize {
                    expected: decompressed_size,
                    size: decompressed_data.len() as u32,
                });
            }

            let buffer = BitReadBuffer::new_owned(decompressed_data, LittleEndian);
            table_data = BitReadStream::new(buffer);
        }

        let table_meta = StringTableMeta {
            max_entries,
            fixed_userdata_size,
        };

        let entries = parse_string_table_list(&mut table_data, &table_meta, entity_count)?;

        let table = StringTable {
            entries,
            max_entries,
            fixed_user_data_size: fixed_userdata_size,
            client_entries: None,
            compressed,
            name,
        };
        Ok(CreateStringTableMessage {
            table: Box::new(table),
        })
    }
}

impl<'a> ParseBitSkip<'a> for CreateStringTableMessage<'a> {
    fn parse_skip(stream: &mut Stream<'a>) -> Result<()> {
        let _: String = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = log_base2(max_entries);
        let _: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let length = read_var_int(stream)?;

        let _: Option<FixedUserDataSize> = stream.read()?;

        let _: bool = stream.read()?;

        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

#[derive(Debug)]
pub struct UpdateStringTableMessage<'a> {
    pub entries: Vec<(u16, StringTableEntry<'a>)>,
    pub table_id: u8,
}

impl<'a> Parse<'a> for UpdateStringTableMessage<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let table_id = stream.read_sized(5)?;

        let changed: u16 = if stream.read()? { stream.read()? } else { 1 };
        let length: u32 = stream.read_int(20)?;

        let mut data = stream.read_bits(length as usize)?;

        let entries = match state.string_tables.get(table_id as usize) {
            Some(table) => parse_string_table_update(&mut data, table, changed),
            None => return Err(ParseError::StringTableNotFound(table_id)),
        }?;

        Ok(UpdateStringTableMessage { table_id, entries })
    }
}

impl<'a> ParseBitSkip<'a> for UpdateStringTableMessage<'a> {
    fn parse_skip(stream: &mut Stream<'a>) -> Result<()> {
        let _: u8 = stream.read_sized(5)?;

        let _: u16 = if stream.read()? { stream.read()? } else { 1 };
        let length: u32 = stream.read_int(20)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

struct TableEntries<'a> {
    entries: Vec<(u16, StringTableEntry<'a>)>,
    history: Vec<u16>,
}

impl<'a> TableEntries<'a> {
    pub fn new(count: usize) -> Self {
        TableEntries {
            entries: Vec::with_capacity(min(count, 128)),
            history: Vec::with_capacity(32),
        }
    }

    pub fn push(&mut self, entry: (u16, StringTableEntry<'a>)) {
        if self.history.len() > 31 {
            self.history.remove(0);
        }
        let entry_index = self.entries.len();
        self.entries.push(entry);
        self.history.push(entry_index as u16);
    }

    pub fn get_history(&self, index: usize) -> Option<&StringTableEntry<'a>> {
        self.history
            .get(index)
            .and_then(|entry_index| self.entries.get(*entry_index as usize))
            .map(|entry| &entry.1)
    }

    pub fn into_entries(self) -> Vec<(u16, StringTableEntry<'a>)> {
        self.entries
    }
}

fn parse_string_table_update<'a>(
    stream: &mut Stream<'a>,
    table_meta: &StringTableMeta,
    entry_count: u16,
) -> ReadResult<Vec<(u16, StringTableEntry<'a>)>> {
    let entry_bits = log_base2(table_meta.max_entries);
    let mut entries = TableEntries::new(entry_count as usize);

    let mut last_entry: i16 = -1;

    for _ in 0..entry_count {
        let index = if stream.read()? {
            (last_entry + 1) as u16
        } else {
            stream.read_sized(entry_bits as usize)?
        };

        last_entry = index as i16;

        let entry = read_table_entry(stream, table_meta, &entries)?;
        entries.push((index, entry));
    }

    Ok(entries.into_entries())
}

fn parse_string_table_list<'a>(
    stream: &mut Stream<'a>,
    table_meta: &StringTableMeta,
    entry_count: u16,
) -> Result<Vec<(u16, StringTableEntry<'a>)>> {
    let mut entries = TableEntries::new(entry_count as usize);

    for index in 0..entry_count {
        if !stream.read::<bool>()? {
            return Err(ParseError::InvalidDemo(
                "there should be no holes when reading CreateStringTable message",
            ));
        };

        let entry = read_table_entry(stream, table_meta, &entries)?;
        entries.push((index, entry));
    }

    Ok(entries.into_entries())
}

fn read_table_entry<'a>(
    stream: &mut Stream<'a>,
    table_meta: &StringTableMeta,
    history: &TableEntries,
) -> ReadResult<StringTableEntry<'a>> {
    let text = if stream.read()? {
        // set value
        if stream.read()? {
            // reuse from history
            let index: u16 = stream.read_sized(5)?;
            let bytes_to_copy: u32 = stream.read_sized(5)?;
            let rest_of_string: Cow<str> = stream.read()?;

            Some(
                match history
                    .get_history(index as usize)
                    .and_then(|entry| entry.text.as_ref())
                {
                    Some(text) => Cow::Owned(String::from_utf8({
                        text.bytes()
                            .take(bytes_to_copy as usize)
                            .chain(rest_of_string.bytes())
                            .collect()
                    })?),
                    None => rest_of_string, // best guess, happens in some pov demos but only for unimportant tables it seems
                },
            )
        } else {
            Some(stream.read()?)
        }
    } else {
        None
    };

    let extra_data = if stream.read()? {
        Some(match table_meta.fixed_userdata_size {
            Some(size) => stream.read_bits(size.bits as usize)?,
            None => {
                let bytes: u16 = stream.read_sized(14)?;
                stream.read_bits(bytes as usize * 8)?
            }
        })
    } else {
        None
    }
    .map(ExtraData::new);

    Ok(StringTableEntry { text, extra_data })
}

pub fn read_var_int(stream: &mut Stream) -> ReadResult<u32> {
    let mut result: u32 = 0;
    for i in (0..35u32).step_by(7) {
        let byte: u8 = stream.read()?;
        result |= (byte as u32 & 0x7F) << i;

        if (byte >> 7) == 0 {
            break;
        }
    }
    Ok(result)
}

pub fn log_base2<T: PrimInt + Unsigned>(num: T) -> u32 {
    // log(0) = inf, but that's a useless result
    // since this would only happen in malformed demos, we just return 0
    (std::mem::size_of::<T>() as u32 * 8 - 1).saturating_sub(num.leading_zeros())
}
