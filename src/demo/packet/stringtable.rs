use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::demo::data::DemoTick;
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::parser::Encode;
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use std::borrow::{Borrow, Cow};
use std::cmp::min;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FixedUserDataSize {
    #[size = 12]
    pub size: u16,
    #[size = 4]
    pub bits: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct StringTable<'a> {
    pub name: Cow<'a, str>,
    pub entries: Vec<(u16, StringTableEntry<'a>)>,
    pub max_entries: u16,
    pub fixed_user_data_size: Option<FixedUserDataSize>,
    pub client_entries: Option<Vec<StringTableEntry<'a>>>,
    pub compressed: bool,
}

impl PartialEq for StringTable<'_> {
    fn eq(&self, other: &Self) -> bool {
        // ignore `compresses` until we encode compressed
        self.name.eq(&other.name)
            && (self.entries.eq(&other.entries))
            && (self.max_entries.eq(&other.max_entries))
            && (self.fixed_user_data_size.eq(&other.fixed_user_data_size))
            && (self.client_entries.eq(&other.client_entries))
    }
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

impl BitWrite<LittleEndian> for StringTable<'_> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.name.as_ref().write(stream)?;
        (self.entries.len() as u16).write(stream)?;
        for (_, entry) in self.entries.iter() {
            entry.write(stream)?;
        }

        self.client_entries.is_some().write(stream)?;
        if let Some(client_entries) = self.client_entries.as_ref() {
            (client_entries.len() as u16).write(stream)?;
            client_entries.write(stream)?;
        }

        Ok(())
    }
}

#[test]
fn test_string_table_roundtrip() {
    crate::test_roundtrip_write(StringTable {
        name: "foo".into(),
        entries: vec![],
        max_entries: 0,
        fixed_user_data_size: None,
        client_entries: None,
        compressed: false,
    });
    crate::test_roundtrip_write(StringTable {
        name: "foo".into(),
        entries: vec![(
            0,
            StringTableEntry {
                text: Some("bar".into()),
                extra_data: None,
            },
        )],
        max_entries: 1,
        fixed_user_data_size: None,
        client_entries: None,
        compressed: false,
    });
    crate::test_roundtrip_write(StringTable {
        name: "foo".into(),
        entries: vec![
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
                    text: Some("asd".into()),
                    extra_data: None,
                },
            ),
        ],
        max_entries: 2,
        fixed_user_data_size: None,
        client_entries: Some(vec![StringTableEntry {
            text: Some("client".into()),
            extra_data: None,
        }]),
        compressed: false,
    });
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[endianness = "LittleEndian"]
#[serde(bound(deserialize = "'a: 'static"))]
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

    pub fn to_owned(&self) -> ExtraData<'static> {
        ExtraData {
            byte_len: self.byte_len,
            data: self.data.to_owned(),
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'a: 'static"))]
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

    pub fn to_owned(&self) -> StringTableEntry<'static> {
        StringTableEntry {
            text: self.text.as_deref().map(|text| Cow::Owned(text.into())),
            extra_data: self.extra_data.as_ref().map(|data| data.to_owned()),
        }
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

impl BitWrite<LittleEndian> for StringTableEntry<'_> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.text.as_deref().unwrap_or_default().write(stream)?;
        self.extra_data.is_some().write(stream)?;
        if let Some(extra_data) = self.extra_data.as_ref() {
            extra_data.write(stream)?;
        }
        Ok(())
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct StringTablePacket<'a> {
    pub tick: DemoTick,
    pub tables: Vec<StringTable<'a>>,
}

impl<'a> Parse<'a> for StringTablePacket<'a> {
    fn parse(stream: &mut Stream<'a>, _state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
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

impl Encode for StringTablePacket<'_> {
    fn encode(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        _state: &ParserState,
    ) -> Result<()> {
        self.tick.write(stream)?;
        stream.reserve_byte_length(32, |stream| {
            (self.tables.len() as u8).write(stream)?;
            self.tables.write(stream)?;

            Ok(())
        })
    }
}

#[test]
fn test_string_table_packet_roundtrip() {
    let state = ParserState::new(24, |_| false, false);
    crate::test_roundtrip_encode(
        StringTablePacket {
            tick: 1.into(),
            tables: vec![],
        },
        &state,
    );
    crate::test_roundtrip_encode(
        StringTablePacket {
            tick: 1.into(),
            tables: vec![StringTable {
                name: "table1".into(),
                entries: vec![],
                max_entries: 0,
                fixed_user_data_size: None,
                client_entries: None,
                compressed: false,
            }],
        },
        &state,
    );
    crate::test_roundtrip_encode(
        StringTablePacket {
            tick: 1.into(),
            tables: vec![
                StringTable {
                    name: "table1".into(),
                    entries: vec![(
                        0,
                        StringTableEntry {
                            text: Some("bar".into()),
                            extra_data: None,
                        },
                    )],
                    max_entries: 1,
                    fixed_user_data_size: None,
                    client_entries: None,
                    compressed: false,
                },
                StringTable {
                    name: "table2".into(),
                    entries: vec![
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
                                text: Some("asd".into()),
                                extra_data: None,
                            },
                        ),
                    ],
                    max_entries: 2,
                    fixed_user_data_size: None,
                    client_entries: Some(vec![StringTableEntry {
                        text: Some("client".into()),
                        extra_data: None,
                    }]),
                    compressed: false,
                },
            ],
        },
        &state,
    );
}
