use std::collections::HashMap;

use arraydeque::{ArrayDeque, Wrapping};
use bitstream_reader::{BitRead, BitReadSized, BitStream, LittleEndian};

use crate::{Parse, ParseError, ParserState, ReadResult, Stream, Result};
use crate::demo::packet::stringtable::{ExtraData, FixedUserdataSize, StringTable, StringTableEntry};

pub struct CreateStringTableMessage {
    pub table: StringTable,
}

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

impl BitRead<LittleEndian> for CreateStringTableMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let name = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = 16 - max_entries.leading_zeros();
        let entity_count: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let bit_count = read_var_int(stream)?;

        let fixed_userdata_size = stream.read()?;

        let compressed = stream.read()?;

        let table_data = stream.read_bits(bit_count as usize)?;

        if compressed {
            unimplemented!("TODO: SNAP decoding");
        }

        let table_meta = StringTableMeta {
            max_entries,
            fixed_userdata_size,
        };

        let entries = parse_string_table_entries(stream, &table_meta, entity_count, &Vec::new())?;
        let mut entries: Vec<(u16, StringTableEntry)> = entries.into_iter().collect();

        // verify that there are no holes in our indexes
        entries.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        let last_index = entries.last().map(|(index, _)| *index).unwrap_or(0u16) as usize;
        if last_index != entries.len() {
            panic!("there should be no holes when reading CreateStringTable message");
        }
        let table_entries = entries.into_iter().map(|(_, entry)| entry).collect();
        let table = StringTable {
            entries: table_entries,
            max_entries,
            fixed_userdata_size,
            client_entries: None,
            compressed,
            name,
        };
        Ok(CreateStringTableMessage {
            table
        })
    }
}

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
            Some(table) => parse_string_table_entries(&mut data, &table.get_table_meta(), changed, &table.entries),
            None => return Err(ParseError::StringTableNotFound(table_id))
        }?;

        Ok(UpdateStringTableMessage {
            table_id,
            entries,
        })
    }
}

fn parse_string_table_entries(
    stream: &mut Stream,
    table_meta: &StringTableMeta,
    entry_count: u16,
    existing_entries: &Vec<StringTableEntry>,
) -> ReadResult<HashMap<u16, StringTableEntry>> {
    let entry_bits = 16 - table_meta.max_entries.leading_zeros();
    let mut entries = HashMap::with_capacity(entry_count as usize);

    let mut last_entry: i16 = -1;
    let mut history: ArrayDeque<[String; 32], Wrapping> = ArrayDeque::new();

    for i in 0..entry_count {
        let index = if stream.read()? {
            (last_entry + 1) as u16
        } else {
            stream.read_sized(entry_bits as usize)?
        };

        let value = if stream.read()? { // set value
            if stream.read()? { // reuse from history
                let index: u16 = stream.read_sized(5)?;
                let bytes_to_copy: u32 = stream.read_sized(5)?;
                let rest_of_string: String = stream.read()?;

                Some(match history.get(index as usize) {
                    Some(text) => String::from_utf8({
                        text.bytes().take(bytes_to_copy as usize).chain(rest_of_string.bytes()).collect()
                    })?,
                    None => rest_of_string // best guess, happens in some pov demos but only for unimportant tables it seems
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
                    stream.read_bits(bytes as usize)?
                }
            })
        } else {
            None
        }.map(|stream| ExtraData {
            len: stream.bit_len() as u16 / 8,
            data: stream,
        });

        let entry = match existing_entries.get(index as usize) {
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
            }
        };
        // optimize: any way to get rid of the clone here?
        // `entries` always outlives `history` without reallocation
        let text = entry.text.clone();
        entries.insert(index, entry);
        unsafe {
            // not 100% sure we should be pushing front here, and not appending
            history.push_front(text);
        }
    }

    Ok(entries)
}

fn read_var_int(stream: &mut Stream) -> ReadResult<u32> {
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
