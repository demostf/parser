use crate::demo::data::DemoTick;
use crate::demo::parser::MalformedSendPropDefinitionError;
use crate::demo::sendprop::{
    RawSendPropDefinition, SendPropDefinition, SendPropFlag, SendPropIdentifier, SendPropType,
};
use crate::{Parse, ParseError, ParserState, Result, Stream};
use bitbuffer::{
    BitRead, BitReadStream, BitWrite, BitWriteSized, BitWriteStream, Endianness, LittleEndian,
};
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::cmp::min;
use std::convert::TryFrom;
use std::iter::once;
use std::ops::Deref;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitRead,
    BitWrite,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Ord,
    PartialOrd,
    Display,
    FromStr,
    Serialize,
    Deserialize,
)]
pub struct ClassId(u16);

impl From<u16> for ClassId {
    fn from(int: u16) -> Self {
        ClassId(int)
    }
}

impl From<ClassId> for usize {
    fn from(class: ClassId) -> Self {
        class.0 as usize
    }
}

impl From<ClassId> for u16 {
    fn from(class: ClassId) -> Self {
        class.0
    }
}

impl PartialEq<u16> for ClassId {
    fn eq(&self, other: &u16) -> bool {
        self.0 == *other
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Display)]
pub struct ServerClassName(String);

impl ServerClassName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for ServerClassName {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for ServerClassName {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl PartialEq<&str> for ServerClassName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl AsRef<str> for ServerClassName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for ServerClassName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ServerClass {
    pub id: ClassId,
    pub name: ServerClassName,
    pub data_table: SendTableName,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitWrite,
    PartialEq,
    Eq,
    Hash,
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Display,
    PartialOrd,
    Ord,
    Default,
)]
pub struct SendTableName(Cow<'static, str>);

impl SendTableName {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<E: Endianness> BitRead<'_, E> for SendTableName {
    fn read(stream: &mut BitReadStream<'_, E>) -> bitbuffer::Result<Self> {
        String::read(stream).map(SendTableName::from)
    }
}

impl From<String> for SendTableName {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<&'static str> for SendTableName {
    fn from(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }
}

impl PartialEq<&str> for SendTableName {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl AsRef<str> for SendTableName {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Deref for SendTableName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParseSendTable {
    pub name: SendTableName,
    pub props: Vec<RawSendPropDefinition>,
    pub needs_decoder: bool,
}

impl Parse<'_> for ParseSendTable {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let needs_decoder = stream.read()?;
        let name: SendTableName = stream.read()?;
        let prop_count = stream.read_int(10)?;

        let mut array_element_prop = None;
        let mut props = Vec::with_capacity(min(prop_count, 128));

        for _ in 0..prop_count {
            let prop: RawSendPropDefinition = RawSendPropDefinition::read(stream, &name)?;
            if prop.flags.contains(SendPropFlag::InsideArray) {
                if array_element_prop.is_some() || prop.flags.contains(SendPropFlag::ChangesOften) {
                    return Err(MalformedSendPropDefinitionError::ArrayChangesOften.into());
                }
                array_element_prop = Some(prop);
            } else if let Some(array_element) = array_element_prop {
                if prop.prop_type != SendPropType::Array {
                    return Err(MalformedSendPropDefinitionError::UntypedArray.into());
                }
                array_element_prop = None;
                props.push(prop.with_array_property(array_element));
            } else {
                props.push(prop);
            }
        }

        Ok(ParseSendTable {
            name,
            props,
            needs_decoder,
        })
    }
}

impl BitWrite<LittleEndian> for ParseSendTable {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> bitbuffer::Result<()> {
        self.needs_decoder.write(stream)?;
        self.name.write(stream)?;

        let prop_count: u16 = self
            .props
            .iter()
            .map(|prop| match prop.array_property {
                Some(_) => 2,
                None => 1,
            })
            .sum();
        prop_count.write_sized(stream, 10)?;

        for prop in self
            .props
            .iter()
            .flat_map(|prop| prop.array_property.as_deref().into_iter().chain(once(prop)))
        {
            prop.write(stream)?;
        }

        Ok(())
    }
}

#[test]
fn test_parse_send_table_roundtrip() {
    use crate::demo::sendprop::SendPropFlags;

    let state = ParserState::new(24, |_| false, false);
    crate::test_roundtrip_encode(
        ParseSendTable {
            name: "foo".into(),
            props: vec![],
            needs_decoder: true,
        },
        &state,
    );
    crate::test_roundtrip_encode(
        ParseSendTable {
            name: "table1".into(),
            props: vec![
                RawSendPropDefinition {
                    prop_type: SendPropType::Float,
                    name: "prop1".into(),
                    identifier: SendPropIdentifier::new("table1", "prop1"),
                    flags: SendPropFlags::default() | SendPropFlag::ChangesOften,
                    table_name: None,
                    low_value: Some(0.0),
                    high_value: Some(128.0),
                    bit_count: Some(10),
                    element_count: None,
                    array_property: None,
                    original_bit_count: Some(10),
                },
                RawSendPropDefinition {
                    prop_type: SendPropType::Array,
                    name: "prop2".into(),
                    identifier: SendPropIdentifier::new("table1", "prop2"),
                    flags: SendPropFlags::default(),
                    table_name: None,
                    low_value: None,
                    high_value: None,
                    bit_count: None,
                    element_count: Some(10),
                    array_property: Some(Box::new(RawSendPropDefinition {
                        prop_type: SendPropType::Int,
                        name: "prop3".into(),
                        identifier: SendPropIdentifier::new("table1", "prop3"),
                        flags: SendPropFlags::default()
                            | SendPropFlag::InsideArray
                            | SendPropFlag::NoScale,
                        table_name: None,
                        low_value: Some(i32::MIN as f32),
                        high_value: Some(i32::MAX as f32),
                        bit_count: Some(32),
                        element_count: None,
                        array_property: None,
                        original_bit_count: Some(32),
                    })),
                    original_bit_count: None,
                },
                RawSendPropDefinition {
                    prop_type: SendPropType::DataTable,
                    name: "prop1".into(),
                    identifier: SendPropIdentifier::new("table1", "prop1"),
                    flags: SendPropFlags::default() | SendPropFlag::Exclude,
                    table_name: Some("table2".into()),
                    low_value: None,
                    high_value: None,
                    bit_count: None,
                    element_count: None,
                    array_property: None,
                    original_bit_count: None,
                },
            ],
            needs_decoder: true,
        },
        &state,
    );
}

impl ParseSendTable {
    pub fn flatten_props(&self, tables: &[ParseSendTable]) -> Result<Vec<SendPropDefinition>> {
        let mut flat = Vec::with_capacity(32);
        self.push_props_end(
            tables,
            &self.get_excludes(tables),
            &mut flat,
            &mut Vec::with_capacity(16),
        )?;

        // sort often changed props before the others
        let mut start = 0;
        for i in 0..flat.len() {
            if flat[i].parse_definition.changes_often() {
                if i != start {
                    flat.swap(i, start);
                }
                start += 1;
            }
        }

        Ok(flat)
    }

    fn get_excludes<'a>(&'a self, tables: &'a [ParseSendTable]) -> Vec<SendPropIdentifier> {
        let mut excludes = Vec::with_capacity(8);

        self.build_excludes(tables, &mut Vec::with_capacity(8), &mut excludes);

        excludes
    }

    fn build_excludes<'a>(
        &'a self,
        tables: &'a [ParseSendTable],
        processed_tables: &mut Vec<&'a SendTableName>,
        excludes: &mut Vec<SendPropIdentifier>,
    ) {
        processed_tables.push(&self.name);

        for prop in self.props.iter() {
            if let Some(exclude_table) = prop.get_exclude_table() {
                excludes.push(SendPropIdentifier::new(
                    exclude_table.as_str(),
                    prop.name.as_str(),
                ))
            } else if let Some(table) = prop.get_data_table(tables) {
                if !processed_tables.contains(&&table.name) {
                    table.build_excludes(tables, processed_tables, excludes);
                }
            }
        }
    }

    // TODO: below is a direct port from the js which is a direct port from C++ and not very optimal
    fn push_props_end<'a>(
        &'a self,
        tables: &'a [ParseSendTable],
        excludes: &[SendPropIdentifier],
        props: &mut Vec<SendPropDefinition>,
        table_stack: &mut Vec<&'a SendTableName>,
    ) -> Result<()> {
        let mut local_props = Vec::new();

        self.push_props_collapse(tables, excludes, &mut local_props, props, table_stack)?;
        props.extend_from_slice(&local_props);
        Ok(())
    }

    fn push_props_collapse<'a>(
        &'a self,
        tables: &'a [ParseSendTable],
        excludes: &[SendPropIdentifier],
        local_props: &mut Vec<SendPropDefinition>,
        props: &mut Vec<SendPropDefinition>,
        table_stack: &mut Vec<&'a SendTableName>,
    ) -> Result<()> {
        table_stack.push(&self.name);

        let result = self
            .props
            .iter()
            .filter(|prop| !prop.is_exclude())
            .filter(|prop| !excludes.iter().any(|exclude| *exclude == prop.identifier()))
            .try_for_each(|prop| {
                if let Some(table) = prop.get_data_table(tables) {
                    if !table_stack.contains(&&table.name) {
                        if prop.flags.contains(SendPropFlag::Collapsible) {
                            table.push_props_collapse(
                                tables,
                                excludes,
                                local_props,
                                props,
                                table_stack,
                            )?;
                        } else {
                            table.push_props_end(tables, excludes, props, table_stack)?;
                        }
                    }
                } else {
                    local_props.push(SendPropDefinition::try_from(prop)?);
                }
                Ok(())
            });

        table_stack.pop();

        result
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTable {
    pub name: SendTableName,
    pub needs_decoder: bool,
    pub raw_props: Vec<RawSendPropDefinition>,
    pub flattened_props: Vec<SendPropDefinition>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DataTablePacket {
    pub tick: DemoTick,
    pub tables: Vec<ParseSendTable>,
    pub server_classes: Vec<ServerClass>,
}

impl Parse<'_> for DataTablePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;

        let mut tables = Vec::new();
        while packet_data.read_bool()? {
            let table = ParseSendTable::parse(&mut packet_data, state)?;
            tables.push(table);
        }

        let server_class_count = packet_data.read_int(16)?;
        let server_classes = packet_data.read_sized(server_class_count)?;

        if packet_data.bits_left() > 7 {
            Err(ParseError::DataRemaining(packet_data.bits_left()))
        } else {
            Ok(DataTablePacket {
                tick,
                tables,
                server_classes,
            })
        }
    }
}

impl BitWrite<LittleEndian> for DataTablePacket {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> bitbuffer::Result<()> {
        self.tick.write(stream)?;
        stream.reserve_byte_length(32, |stream| {
            for table in self.tables.iter() {
                true.write(stream)?;
                table.write(stream)?;
            }
            false.write(stream)?;

            (self.server_classes.len() as u16).write(stream)?;
            self.server_classes.write(stream)?;

            Ok(())
        })
    }
}

#[test]
fn test_data_table_packet_roundtrip() {
    use crate::demo::sendprop::SendPropFlags;

    let state = ParserState::new(24, |_| false, false);
    crate::test_roundtrip_encode(
        DataTablePacket {
            tick: 123.into(),
            tables: vec![],
            server_classes: vec![],
        },
        &state,
    );

    let table1 = ParseSendTable {
        name: "table1".into(),
        props: vec![
            RawSendPropDefinition {
                prop_type: SendPropType::Float,
                name: "prop1".into(),
                identifier: SendPropIdentifier::new("table1", "prop1"),
                flags: SendPropFlags::default() | SendPropFlag::ChangesOften,
                table_name: None,
                low_value: Some(0.0),
                high_value: Some(128.0),
                bit_count: Some(10),
                element_count: None,
                array_property: None,
                original_bit_count: Some(10),
            },
            RawSendPropDefinition {
                prop_type: SendPropType::Array,
                name: "prop2".into(),
                identifier: SendPropIdentifier::new("table1", "prop2"),
                flags: SendPropFlags::default(),
                table_name: None,
                low_value: None,
                high_value: None,
                bit_count: None,
                element_count: Some(10),
                array_property: Some(Box::new(RawSendPropDefinition {
                    prop_type: SendPropType::Int,
                    name: "prop3".into(),
                    identifier: SendPropIdentifier::new("table1", "prop3"),
                    flags: SendPropFlags::default()
                        | SendPropFlag::InsideArray
                        | SendPropFlag::NoScale,
                    table_name: None,
                    low_value: Some(i32::MIN as f32),
                    high_value: Some(i32::MAX as f32),
                    bit_count: Some(32),
                    element_count: None,
                    array_property: None,
                    original_bit_count: Some(32),
                })),
                original_bit_count: None,
            },
            RawSendPropDefinition {
                prop_type: SendPropType::DataTable,
                name: "prop1".into(),
                identifier: SendPropIdentifier::new("table1", "prop1"),
                flags: SendPropFlags::default() | SendPropFlag::Exclude,
                table_name: Some("table2".into()),
                low_value: None,
                high_value: None,
                bit_count: None,
                element_count: None,
                array_property: None,
                original_bit_count: None,
            },
        ],
        needs_decoder: true,
    };
    let table2 = ParseSendTable {
        name: "table2".into(),
        props: vec![RawSendPropDefinition {
            prop_type: SendPropType::Float,
            name: "prop1".into(),
            identifier: SendPropIdentifier::new("table2", "prop1"),
            flags: SendPropFlags::default() | SendPropFlag::ChangesOften,
            table_name: None,
            low_value: Some(0.0),
            high_value: Some(128.0),
            bit_count: Some(10),
            element_count: None,
            array_property: None,
            original_bit_count: Some(10),
        }],
        needs_decoder: true,
    };
    crate::test_roundtrip_encode(
        DataTablePacket {
            tick: 1.into(),
            tables: vec![table1, table2],
            server_classes: vec![
                ServerClass {
                    id: ClassId(0),
                    name: "class1".into(),
                    data_table: "table1".into(),
                },
                ServerClass {
                    id: ClassId(1),
                    name: "class2".into(),
                    data_table: "table2".into(),
                },
            ],
        },
        &state,
    );
}
