use bitstream_reader::{BitRead, LittleEndian};

use crate::demo::parser::MalformedSendPropDefinitionError;
use crate::demo::sendprop::{
    SendPropDefinition, SendPropDefinitionIndex, SendPropFlag, SendPropName, SendPropType,
};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use parse_display::{Display, FromStr};
use serde::{Deserialize, Serialize};
use std::borrow::Borrow;
use std::cell::{Cell, RefCell};
use std::cmp::min;
use std::fmt;
use std::num::ParseIntError;
use std::ops::Deref;
use std::rc::Rc;

#[derive(BitRead, Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Display, FromStr)]
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

#[derive(BitRead, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Display)]
pub struct ServerClassName(Rc<String>);

impl ServerClassName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for ServerClassName {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

#[derive(BitRead, Debug, Clone)]
pub struct ServerClass {
    pub id: ClassId,
    pub name: ServerClassName,
    pub data_table: SendTableName,
}

#[derive(
    BitRead, PartialEq, Eq, Hash, Debug, Serialize, Deserialize, Clone, Display, PartialOrd, Ord,
)]
pub struct SendTableName(Rc<String>);

impl SendTableName {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<String> for SendTableName {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

#[derive(Debug, Clone)]
pub struct ParseSendTable {
    pub name: SendTableName,
    pub props: Vec<Rc<SendPropDefinition>>,
    pub needs_decoder: bool,
}

impl ParseSendTable {
    fn parse(stream: &mut Stream, _state: &ParserState, table_index: usize) -> Result<Self> {
        let needs_decoder = stream.read()?;
        let raw_name: String = stream.read()?;
        let name: SendTableName = raw_name.into();
        let prop_count = stream.read_int(10)?;

        let mut array_element_prop = None;
        let mut props = Vec::with_capacity(min(prop_count, 128));

        for prop_index in 0..prop_count {
            let definition_index = SendPropDefinitionIndex::new(table_index, prop_index);
            let prop: SendPropDefinition =
                SendPropDefinition::read(stream, name.clone(), definition_index)?;
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
                props.push(Rc::new(prop.with_array_property(array_element)));
            } else {
                props.push(Rc::new(prop));
            }
        }

        Ok(ParseSendTable {
            name,
            needs_decoder,
            props,
        })
    }
}

impl ParseSendTable {
    pub fn flatten_props(&self, tables: &[ParseSendTable]) -> Vec<Rc<SendPropDefinition>> {
        let mut flat = Vec::with_capacity(32);
        self.get_all_props(tables, &self.get_excludes(tables), &mut flat);

        // sort often changed props before the others
        let mut start = 0;
        for i in 0..flat.len() {
            if flat[i].flags.contains(SendPropFlag::ChangesOften) {
                if i != start {
                    flat.swap(i, start);
                }
                start += 1;
            }
        }

        flat.into_iter().map(|prop| Rc::clone(&prop)).collect()
    }

    fn get_excludes<'a>(&'a self, tables: &'a [ParseSendTable]) -> Vec<Exclude<'a>> {
        let mut excludes = Vec::with_capacity(8);

        for prop in self.props.iter() {
            if let Some(exclude_table) = prop.get_exclude_table() {
                excludes.push(Exclude {
                    table: exclude_table,
                    prop: &prop.name,
                })
            } else if let Some(table) = prop.get_data_table(tables) {
                excludes.extend_from_slice(&table.get_excludes(tables));
            }
        }

        excludes
    }

    // TODO: below is a direct port from the js which is a direct port from C++ and not very optimal
    fn get_all_props(
        &self,
        tables: &[ParseSendTable],
        excludes: &[Exclude],
        props: &mut Vec<Rc<SendPropDefinition>>,
    ) {
        let mut local_props = Vec::new();

        self.get_all_props_iterator_props(tables, excludes, &mut local_props, props);
        props.extend_from_slice(&local_props);
    }

    fn get_all_props_iterator_props(
        &self,
        tables: &[ParseSendTable],
        excludes: &[Exclude],
        local_props: &mut Vec<Rc<SendPropDefinition>>,
        props: &mut Vec<Rc<SendPropDefinition>>,
    ) {
        self.props
            .iter()
            .filter(|prop| !prop.is_exclude())
            .filter(|prop| !excludes.iter().any(|exclude| exclude.matches(prop)))
            .for_each(|prop| {
                if let Some(table) = prop.get_data_table(tables) {
                    if prop.flags.contains(SendPropFlag::Collapsible) {
                        table.get_all_props_iterator_props(tables, excludes, local_props, props);
                    } else {
                        table.get_all_props(tables, excludes, props);
                    }
                } else {
                    local_props.push(Rc::clone(prop));
                }
            })
    }
}

#[derive(Clone)]
struct Exclude<'a> {
    table: &'a SendTableName,
    prop: &'a SendPropName,
}

impl<'a> Exclude<'a> {
    pub fn matches(&self, prop: &SendPropDefinition) -> bool {
        self.table == &prop.owner_table && *self.prop == prop.name
    }
}

#[derive(Debug, Clone)]
pub struct SendTable {
    pub name: SendTableName,
    pub props: Vec<Rc<SendPropDefinition>>,
    pub needs_decoder: bool,
    pub flattened_props: Vec<Rc<SendPropDefinition>>,
}

#[derive(Debug)]
pub struct DataTablePacket {
    pub tick: u32,
    pub tables: Vec<ParseSendTable>,
    pub server_classes: Vec<ServerClass>,
}

impl Parse for DataTablePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;

        let mut tables = Vec::new();
        let mut table_index = 0;
        while packet_data.read_bool()? {
            let table = ParseSendTable::parse(&mut packet_data, state, table_index)?;
            table_index += 1;
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
