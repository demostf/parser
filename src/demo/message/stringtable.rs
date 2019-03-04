use std::collections::HashMap;

use arraydeque::{ArrayDeque, Wrapping};
use bitstream_reader::{BitBuffer, BitRead, BitReadSized, BitStream, LittleEndian};
use num_traits::{PrimInt, Unsigned};
use snap::Decoder;

use crate::demo::packet::stringtable::{
    ExtraData, FixedUserdataSize, StringTable, StringTableEntry,
};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};

#[derive(Debug)]
pub struct CreateStringTableMessage {
    pub table: StringTable,
}

#[derive(Debug)]
pub struct StringTableMeta {
    pub max_entries: u16,
    pub fixed_userdata_size: Option<FixedUserdataSize>,
}

impl From<&StringTable> for StringTableMeta {
    fn from(table: &StringTable) -> Self {
        StringTableMeta {
            max_entries: table.max_entries,
            fixed_userdata_size: table.fixed_userdata_size,
        }
    }
}

impl Parse for CreateStringTableMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let name = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = log_base2(max_entries);
        let entity_count: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let bit_count = read_var_int(stream)?;

        let fixed_userdata_size = stream.read()?;

        let compressed = stream.read()?;

        let mut table_data = stream.read_bits(bit_count as usize)?;

        if compressed {
            let decompressed_size: u32 = table_data.read()?;
            let compressed_size: u32 = table_data.read()?;

            let magic = table_data.read_string(Some(4))?;

            if magic != "SNAP" {
                return Err(ParseError::UnexpectedCompressionType(magic));
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

            let buffer = BitBuffer::new(decompressed_data, LittleEndian);
            table_data = BitStream::new(buffer);
        }

        let table_meta = StringTableMeta {
            max_entries,
            fixed_userdata_size,
        };

        let entries = parse_string_table_list(&mut table_data, &table_meta, entity_count)?;

        let table = StringTable {
            entries,
            max_entries,
            fixed_userdata_size,
            client_entries: None,
            compressed,
            name,
        };
        Ok(CreateStringTableMessage { table })
    }
}

#[derive(Debug)]
pub struct UpdateStringTableMessage {
    pub entries: HashMap<u16, StringTableEntry>,
    pub table_id: u8,
}

impl Parse for UpdateStringTableMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let table_id = stream.read_sized(5)?;

        let changed: u16 = if stream.read()? { stream.read()? } else { 1 };
        let len = stream.read_int(20)?;

        let mut data = stream.read_bits(len)?;

        let entries = match state.string_tables.get(table_id as usize) {
            Some(table) => parse_string_table_sparse_entries(
                &mut data,
                &table.get_table_meta(),
                changed,
                &table.entries,
            ),
            None => return Err(ParseError::StringTableNotFound(table_id)),
        }?;

        Ok(UpdateStringTableMessage { table_id, entries })
    }
}

fn parse_string_table_sparse_entries(
    stream: &mut Stream,
    table_meta: &StringTableMeta,
    entry_count: u16,
    existing_entries: &Vec<StringTableEntry>,
) -> ReadResult<HashMap<u16, StringTableEntry>> {
    let entry_bits = log_base2(table_meta.max_entries);
    let mut entries = HashMap::with_capacity(entry_count as usize);

    let mut last_entry: i16 = -1;
    let mut history: Vec<String> = Vec::new();

    for i in 0..entry_count {
        let index = if stream.read()? {
            (last_entry + 1) as u16
        } else {
            stream.read_sized(entry_bits as usize)?
        };

        last_entry = index as i16;

        let entry = read_table_entry(
            stream,
            table_meta,
            &history,
            existing_entries.get(index as usize),
        )?;
        // optimize: any way to get rid of the clone here?
        // `entries` always outlives `history` without reallocation
        let text = entry.text.clone();
        entries.insert(index, entry);
        // not 100% sure we should be pushing front here, and not appending
        history.push(text);

        if history.len() > 32 {
            history.remove(0);
        }
    }

    Ok(entries)
}

fn parse_string_table_list(
    stream: &mut Stream,
    table_meta: &StringTableMeta,
    entry_count: u16,
) -> ReadResult<Vec<StringTableEntry>> {
    let entry_bits = log_base2(table_meta.max_entries);
    let mut entries = Vec::with_capacity(entry_count as usize);

    let mut history: Vec<String> = Vec::new();

    for i in 0..entry_count {
        if !stream.read::<bool>()? {
            panic!("there should be no holes when reading CreateStringTable message");
        };

        let entry = read_table_entry(stream, table_meta, &history, None)?;
        // optimize: any way to get rid of the clone here?
        // `entries` always outlives `history` without reallocation
        let text = entry.text.clone();
        entries.push(entry);
        // not 100% sure we should be pushing front here, and not appending
        history.push(text);

        if history.len() > 32 {
            history.remove(0);
        }
    }

    Ok(entries)
}

fn read_table_entry(
    stream: &mut Stream,
    table_meta: &StringTableMeta,
    history: &Vec<String>,
    existing_entry: Option<&StringTableEntry>,
) -> ReadResult<StringTableEntry> {
    let value = if stream.read()? {
        // set value
        if stream.read()? {
            // reuse from history
            let index: u16 = stream.read_sized(5)?;
            let bytes_to_copy: u32 = stream.read_sized(5)?;
            let rest_of_string: String = stream.read()?;

            Some(match history.get(index as usize) {
                Some(text) => String::from_utf8({
                    text.bytes()
                        .take(bytes_to_copy as usize)
                        .chain(rest_of_string.bytes())
                        .collect()
                })?,
                None => rest_of_string, // best guess, happens in some pov demos but only for unimportant tables it seems
            })
        } else {
            Some(stream.read()?)
        }
    } else {
        None
    };

    let user_data = if stream.read()? {
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

    Ok(match existing_entry {
        Some(existing_entry) => {
            let new_user_data = user_data.or_else(|| existing_entry.extra_data.clone());
            let new_value = value.unwrap_or_else(|| existing_entry.text.clone());
            StringTableEntry {
                text: new_value,
                extra_data: new_user_data,
            }
        }
        None => StringTableEntry {
            text: value.unwrap_or_default(),
            extra_data: user_data,
        },
    })
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
    (std::mem::size_of::<T>() as u32 * 8 - 1) - num.leading_zeros()
}
