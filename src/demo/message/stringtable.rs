use bitbuffer::{
    BitReadBuffer, BitReadStream, BitWrite, BitWriteSized, BitWriteStream, LittleEndian,
};
use num_traits::{PrimInt, Unsigned};
use serde::{Deserialize, Serialize};
use snap::raw::{decompress_len, Decoder};

use crate::demo::lzss::decompress;
use crate::demo::packet::stringtable::{
    ExtraData, FixedUserDataSize, StringTable, StringTableEntry,
};
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use std::borrow::Cow;
use std::cmp::min;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct CreateStringTableMessage<'a> {
    pub table: StringTable<'a>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let name = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = log_base2(max_entries);
        let entity_count: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(20)?
        };

        let fixed_userdata_size = stream.read()?;

        let compressed = stream.read()?;

        let mut table_data = stream.read_bits(length as usize)?;

        if compressed {
            let decompressed_size: u32 = table_data.read()?;
            let compressed_size: u32 = table_data.read()?;

            if !(4..=10 * 1024 * 1024).contains(&compressed_size) {
                return Err(ParseError::InvalidDemo(
                    "Invalid compressed string table size",
                ));
            }

            if decompressed_size > 100 * 1024 * 1024 {
                return Err(ParseError::InvalidDemo(
                    "Invalid decompressed string table size",
                ));
            }

            let magic = table_data.read_string(Some(4))?;

            match magic.as_ref() {
                "SNAP" => {
                    let compressed_data = table_data.read_bytes(compressed_size as usize - 4)?;

                    let mut decoder = Decoder::new();

                    let decompressed_size_from_header = decompress_len(&compressed_data)?;

                    if decompressed_size_from_header != decompressed_size as usize {
                        return Err(ParseError::UnexpectedDecompressedSize {
                            expected: decompressed_size,
                            size: decompressed_size_from_header as u32,
                        });
                    }

                    let mut decompressed_data = vec![0; decompressed_size_from_header];
                    decoder
                        .decompress(&compressed_data, &mut decompressed_data)
                        .map_err(ParseError::from)?;

                    let buffer = BitReadBuffer::new_owned(decompressed_data, LittleEndian);
                    table_data = BitReadStream::new(buffer);
                }
                "LZSS" => {
                    let compressed_data = table_data.read_bytes(compressed_size as usize - 4)?;
                    let mut decompressed_data = Vec::with_capacity(decompressed_size as usize);
                    decompress(&compressed_data, &mut decompressed_data);

                    if decompressed_data.len() != decompressed_size as usize {
                        return Err(ParseError::UnexpectedDecompressedSize {
                            expected: decompressed_size,
                            size: decompressed_data.len() as u32,
                        });
                    }

                    let buffer = BitReadBuffer::new_owned(decompressed_data, LittleEndian);
                    table_data = BitReadStream::new(buffer);
                }
                _ => {
                    return Err(ParseError::UnexpectedCompressionType(magic.into_owned()));
                }
            }
        }

        let table_meta = StringTableMeta {
            max_entries,
            fixed_userdata_size,
        };

        let entries = parse_string_table_update(&mut table_data, &table_meta, entity_count)?;

        let table = StringTable {
            entries,
            max_entries,
            fixed_user_data_size: fixed_userdata_size,
            client_entries: None,
            compressed,
            name,
        };
        Ok(CreateStringTableMessage { table })
    }
}

impl<'a> ParseBitSkip<'a> for CreateStringTableMessage<'a> {
    fn parse_skip(stream: &mut Stream<'a>, state: &ParserState) -> Result<()> {
        let _: String = stream.read()?;
        let max_entries: u16 = stream.read()?;
        let encode_bits = log_base2(max_entries);
        let _: u16 = stream.read_sized(encode_bits as usize + 1)?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(20)?
        };

        let _: Option<FixedUserDataSize> = stream.read()?;

        let _: bool = stream.read()?;

        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

impl Encode for CreateStringTableMessage<'_> {
    fn encode(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        _state: &ParserState,
    ) -> Result<()> {
        let table = &self.table;
        table.name.write(stream)?;
        table.max_entries.write(stream)?;
        let encode_bits = log_base2(table.max_entries) as usize;
        (table.entries.len() as u16).write_sized(stream, encode_bits + 1)?;

        stream.reserve_int::<ParseError, _>(40, |stream| {
            table.fixed_user_data_size.is_some().write(stream)?;
            if let Some(fixed_size) = table.fixed_user_data_size {
                fixed_size.write(stream)?;
            }

            // no compression for now
            false.write(stream)?;

            let start = stream.bit_len();

            let table_meta = table.get_table_meta();

            write_string_table_update(&table.entries, stream, &table_meta)?;

            let end = stream.bit_len();
            Ok(encode_var_int_fixed((end - start) as u32))
        })?;

        Ok(())
    }
}

#[test]
fn test_create_string_table_roundtrip() {
    let state = ParserState::new(24, |_| false, false);
    crate::test_roundtrip_encode(
        CreateStringTableMessage {
            table: StringTable {
                name: "table1".into(),
                entries: vec![],
                max_entries: 16,
                fixed_user_data_size: None,
                client_entries: None,
                compressed: false,
            },
        },
        &state,
    );
    crate::test_roundtrip_encode(
        CreateStringTableMessage {
            table: StringTable {
                name: "table1".into(),
                entries: vec![
                    (
                        0,
                        StringTableEntry {
                            text: Some("foo".into()),
                            extra_data: None,
                        },
                    ),
                    (
                        1,
                        StringTableEntry {
                            text: Some("bar".into()),
                            extra_data: None,
                        },
                    ),
                ],
                max_entries: 16,
                fixed_user_data_size: Some(FixedUserDataSize { size: 12, bits: 4 }),
                client_entries: None,
                compressed: false,
            },
        },
        &state,
    );
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
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

        Ok(UpdateStringTableMessage { entries, table_id })
    }
}

impl<'a> ParseBitSkip<'a> for UpdateStringTableMessage<'a> {
    fn parse_skip(stream: &mut Stream<'a>, _state: &ParserState) -> Result<()> {
        let _: u8 = stream.read_sized(5)?;

        let _: u16 = if stream.read()? { stream.read()? } else { 1 };
        let length: u32 = stream.read_int(20)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}

impl Encode for UpdateStringTableMessage<'_> {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        self.table_id.write_sized(stream, 5)?;
        if self.entries.len() == 1 {
            false.write(stream)?;
        } else {
            true.write(stream)?;
            (self.entries.len() as u16).write(stream)?;
        }

        match state.string_tables.get(self.table_id as usize) {
            Some(table) => Ok(stream.reserve_length(20, |stream| {
                write_string_table_update(&self.entries, stream, table)
            })?),
            None => Err(ParseError::StringTableNotFound(self.table_id)),
        }
    }
}

#[test]
fn test_update_string_table_roundtrip() {
    let mut state = ParserState::new(24, |_| false, false);
    state.string_tables = vec![StringTableMeta {
        max_entries: 16,
        fixed_userdata_size: None,
    }];
    crate::test_roundtrip_encode(
        UpdateStringTableMessage {
            entries: vec![],
            table_id: 0,
        },
        &state,
    );
    crate::test_roundtrip_encode(
        UpdateStringTableMessage {
            entries: vec![(
                2,
                StringTableEntry {
                    text: Some("foo".into()),
                    extra_data: None,
                },
            )],
            table_id: 0,
        },
        &state,
    );
    crate::test_roundtrip_encode(
        UpdateStringTableMessage {
            entries: vec![
                (
                    2,
                    StringTableEntry {
                        text: Some("foo".into()),
                        extra_data: None,
                    },
                ),
                (
                    3,
                    StringTableEntry {
                        text: Some("bar".into()),
                        extra_data: None,
                    },
                ),
            ],
            table_id: 0,
        },
        &state,
    );
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

    pub fn find_best_history(&self, text: &str) -> Option<(usize, usize)> {
        let mut best_index = None;
        let mut best_count = 0;
        for (history_index, entry_index) in self.history.iter().enumerate() {
            if let Some((_, entry)) = self.entries.get(*entry_index as usize) {
                let similar = min(31, count_similar_characters(entry.text(), text));
                if similar >= 3 && similar > best_count {
                    best_index = Some(history_index);
                    best_count = similar;
                }
            }
        }

        best_index.map(|index| (index, best_count))
    }
}

fn count_similar_characters(a: &str, b: &str) -> usize {
    for (i, (a, b)) in a.bytes().zip(b.bytes()).enumerate() {
        if a != b {
            return i;
        }
    }
    min(a.len(), b.len())
}

pub fn parse_string_table_update<'a>(
    stream: &mut Stream<'a>,
    table_meta: &StringTableMeta,
    entry_count: u16,
) -> ReadResult<Vec<(u16, StringTableEntry<'a>)>> {
    let entry_bits = log_base2(table_meta.max_entries);
    let mut entries = TableEntries::new(entry_count as usize);

    let mut last_entry: i16 = -1;

    for _ in 0..entry_count {
        let index = if stream.read()? {
            last_entry.saturating_add(1) as u16
        } else {
            stream.read_sized(entry_bits as usize)?
        };

        last_entry = index as i16;

        let entry = read_table_entry(stream, table_meta, &entries)?;
        entries.push((index, entry));
    }

    Ok(entries.into_entries())
}

pub fn write_string_table_update<'a>(
    entries: &[(u16, StringTableEntry<'a>)],
    stream: &mut BitWriteStream<LittleEndian>,
    table_meta: &StringTableMeta,
) -> ReadResult<()> {
    let entry_bits = log_base2(table_meta.max_entries);

    let mut last_entry: i16 = -1;
    let mut history = TableEntries::new(entries.len());

    for (index, entry) in entries.iter() {
        let index = *index as i16;
        if index == (last_entry + 1) {
            true.write(stream)?;
        } else {
            false.write(stream)?;
            index.write_sized(stream, entry_bits as usize)?;
        }
        last_entry = index;

        write_table_entry(entry, stream, table_meta, &history)?;
        history.push((index as u16, entry.clone()));
    }

    Ok(())
}

#[test]
fn test_table_update_roundtrip() {
    fn entry_roundtrip(
        entries: Vec<(u16, StringTableEntry)>,
        max_entries: u16,
        fixed_bits: Option<u8>,
    ) {
        let table_meta = StringTableMeta {
            max_entries,
            fixed_userdata_size: fixed_bits.map(|bits| FixedUserDataSize { size: 0, bits }),
        };
        let mut data = Vec::new();
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_string_table_update(&entries, &mut write, &table_meta).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(
            entries,
            parse_string_table_update(&mut read, &table_meta, entries.len() as u16).unwrap()
        );
        assert_eq!(pos, read.pos());
    }
    entry_roundtrip(
        vec![(
            3,
            StringTableEntry {
                text: None,
                extra_data: None,
            },
        )],
        8,
        None,
    );
    entry_roundtrip(
        vec![
            (
                0,
                StringTableEntry {
                    text: Some("bar".into()),
                    extra_data: None,
                },
            ),
            (
                1,
                StringTableEntry {
                    text: Some("foo".into()),
                    extra_data: None,
                },
            ),
            (
                5,
                StringTableEntry {
                    text: Some("asd".into()),
                    extra_data: None,
                },
            ),
        ],
        16,
        None,
    );
    entry_roundtrip(
        vec![
            (
                1,
                StringTableEntry {
                    text: Some("foo".into()),
                    extra_data: None,
                },
            ),
            (
                2,
                StringTableEntry {
                    text: Some("asd".into()),
                    extra_data: None,
                },
            ),
        ],
        16,
        None,
    );
    entry_roundtrip(
        vec![(
            1,
            StringTableEntry {
                text: Some("foo".into()),
                extra_data: None,
            },
        )],
        16,
        None,
    );
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

fn write_table_entry(
    entry: &StringTableEntry,
    stream: &mut BitWriteStream<LittleEndian>,
    table_meta: &StringTableMeta,
    history: &TableEntries,
) -> ReadResult<()> {
    entry.text.is_some().write(stream)?;
    if let Some(text) = entry.text.as_deref() {
        let history_item = history.find_best_history(text);
        history_item.is_some().write(stream)?;
        if let Some((history_index, history_count)) = history_item {
            history_index.write_sized(stream, 5)?;
            history_count.write_sized(stream, 5)?;
            let diff_bytes = &text.as_bytes()[history_count..];
            stream.write_bytes(diff_bytes)?;
            0u8.write(stream)?; // writing the string as bytes doesn't add the null terminator
        } else {
            text.write(stream)?;
        }
    }

    entry.extra_data.is_some().write(stream)?;
    if let Some(extra_data) = entry.extra_data.as_ref() {
        match table_meta.fixed_userdata_size {
            Some(size) => {
                extra_data.data.write_sized(stream, size.bits as usize)?;
            }
            None => {
                extra_data.byte_len.write_sized(stream, 14)?;
                extra_data
                    .data
                    .write_sized(stream, extra_data.byte_len as usize * 8)?;
            }
        }
    }

    Ok(())
}

#[test]
fn test_table_entry_roundtrip() {
    fn entry_roundtrip(entry: StringTableEntry, fixed_bits: Option<u8>) {
        let table_meta = StringTableMeta {
            max_entries: 0,
            fixed_userdata_size: fixed_bits.map(|bits| FixedUserDataSize { size: 0, bits }),
        };
        let mut data = Vec::new();
        let pos = {
            let history = TableEntries::new(1);
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_table_entry(&entry, &mut write, &table_meta, &history).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(
            entry,
            read_table_entry(&mut read, &table_meta, &TableEntries::new(0)).unwrap()
        );
        assert_eq!(pos, read.pos());
    }
    entry_roundtrip(
        StringTableEntry {
            text: None,
            extra_data: None,
        },
        None,
    );
    entry_roundtrip(
        StringTableEntry {
            text: Some("foo".into()),
            extra_data: None,
        },
        None,
    );
    entry_roundtrip(
        StringTableEntry {
            text: None,
            extra_data: Some(ExtraData::new(BitReadStream::new(
                BitReadBuffer::new_owned(vec![0x55], LittleEndian),
            ))),
        },
        None,
    );
    entry_roundtrip(
        StringTableEntry {
            text: None,
            extra_data: Some(ExtraData::new(BitReadStream::new(
                BitReadBuffer::new_owned(vec![0x55; 128], LittleEndian),
            ))),
        },
        None,
    );
    entry_roundtrip(
        StringTableEntry {
            text: None,
            extra_data: Some(ExtraData::new(BitReadStream::new(
                BitReadBuffer::new_owned(vec![0x55; 4], LittleEndian),
            ))),
        },
        Some(4 * 8),
    );
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

pub fn write_var_int(mut int: u32, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
    while int > 0x7F {
        let byte: u8 = int as u8 & 0x7F;
        (byte | 0x80).write(stream)?;
        int >>= 7;
    }
    (int as u8).write(stream)
}

// encode the int in such a way that it has a fixed size, but still decodes the same
// result is the first 40 bits of the return value
pub fn encode_var_int_fixed(mut int: u32) -> u64 {
    let mut out = 0;
    for i in 0..4 {
        let byte: u8 = int as u8 & 0x7F;
        out |= ((byte | 0x80) as u64) << (i * 8);
        int >>= 7;
    }
    out |= (int as u64) << 32;
    out
}

#[test]
fn test_var_int_roundtrip() {
    fn var_int_roundtrip(int: u32) {
        let mut data = Vec::new();
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            write_var_int(int, &mut write).unwrap();
            write.bit_len()
        };
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(int, read_var_int(&mut read).unwrap());
        assert_eq!(pos, read.pos());
    }
    var_int_roundtrip(0);
    var_int_roundtrip(1);
    var_int_roundtrip(10);
    var_int_roundtrip(55);
    var_int_roundtrip(355);
    var_int_roundtrip(12354);
    var_int_roundtrip(123125412);
}

#[test]
fn test_var_int_fixed_roundtrip() {
    fn var_int_roundtrip(int: u32) {
        let mut data = Vec::new();
        let pos = {
            let mut write = BitWriteStream::new(&mut data, LittleEndian);
            let encoded = encode_var_int_fixed(int);
            encoded.write_sized(&mut write, 40).unwrap();
            write.bit_len()
        };
        assert_eq!(40, pos);
        let mut read = BitReadStream::new(BitReadBuffer::new(&data, LittleEndian));
        assert_eq!(int, read_var_int(&mut read).unwrap());
        assert_eq!(pos, read.pos());
    }
    var_int_roundtrip(0);
    var_int_roundtrip(1);
    var_int_roundtrip(10);
    var_int_roundtrip(55);
    var_int_roundtrip(355);
    var_int_roundtrip(12354);
    var_int_roundtrip(123125412);
}

pub fn log_base2<T: PrimInt + Unsigned>(num: T) -> u32 {
    // log(0) = inf, but that's a useless result
    // since this would only happen in malformed demos, we just return 0
    (std::mem::size_of::<T>() as u32 * 8 - 1).saturating_sub(num.leading_zeros())
}
