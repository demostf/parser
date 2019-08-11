use bitstream_reader::{BitRead, LittleEndian};

use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};
use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};
use serde::{Deserialize, Serialize};
use std::cell::{Cell, RefCell};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

#[derive(BitRead, Debug)]
pub struct ServerClass {
    pub id: u16,
    pub name: String,
    pub data_table: String,
}

#[derive(PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct SendTableName(Rc<String>);

impl Clone for SendTableName {
    fn clone(&self) -> Self {
        SendTableName(Rc::clone(&self.0))
    }
}

impl fmt::Display for SendTableName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for SendTableName {
    fn from(value: String) -> Self {
        Self(Rc::new(value))
    }
}

impl BitRead<LittleEndian> for SendTableName {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        String::read(stream).map(SendTableName::from)
    }
}

#[derive(Debug)]
pub struct ParseSendTable {
    pub name: SendTableName,
    pub props: Vec<SendPropDefinition>,
    pub needs_decoder: bool,
}

impl Parse for ParseSendTable {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let needs_decoder = stream.read()?;
        let raw_name: String = stream.read()?;
        let name: SendTableName = raw_name.into();
        let prop_count = stream.read_int(10)?;

        let mut array_element_prop = None;
        let mut props = Vec::with_capacity(prop_count);

        for _ in 0..prop_count {
            let prop: SendPropDefinition = SendPropDefinition::read(stream, name.clone())?;
            if prop.flags.contains(SendPropFlag::InsideArray) {
                if array_element_prop.is_some() || prop.flags.contains(SendPropFlag::ChangesOften) {
                    return Err(ParseError::InvalidSendProp(
                        "Array contents can't have the 'ChangesOften' flag".to_owned(),
                    ));
                }
                array_element_prop = Some(prop);
            } else if let Some(array_element) = array_element_prop {
                if prop.prop_type != SendPropType::Array {
                    return Err(ParseError::InvalidSendProp(
                        "Array contents can without array".to_owned(),
                    ));
                }
                array_element_prop = None;
                props.push(prop.with_array_property(array_element));
            } else {
                props.push(prop);
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
    pub fn flatten_props(&self, tables: &[ParseSendTable]) -> Vec<SendPropDefinition> {
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

        flat.into_iter().map(|prop| prop.clone()).collect()
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
    fn get_all_props<'a>(
        &'a self,
        tables: &'a [ParseSendTable],
        excludes: &[Exclude],
        props: &mut Vec<&'a SendPropDefinition>,
    ) {
        let mut local_props = Vec::new();

        self.get_all_props_iterator_props(tables, excludes, &mut local_props, props);
        props.extend_from_slice(&local_props);
    }

    fn get_all_props_iterator_props<'a>(
        &'a self,
        tables: &'a [ParseSendTable],
        excludes: &[Exclude],
        local_props: &mut Vec<&'a SendPropDefinition>,
        props: &mut Vec<&'a SendPropDefinition>,
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
                    local_props.push(prop);
                }
            })
    }
}

#[derive(Clone)]
struct Exclude<'a> {
    table: &'a SendTableName,
    prop: &'a str,
}

impl<'a> Exclude<'a> {
    pub fn matches(&self, prop: &SendPropDefinition) -> bool {
        self.table == &prop.owner_table && self.prop == prop.name
    }
}

#[derive(Debug)]
pub struct SendTable {
    pub name: SendTableName,
    pub props: Vec<SendPropDefinition>,
    pub needs_decoder: bool,
    pub flattened_props: Vec<SendPropDefinition>,
}

#[derive(Debug)]
pub struct DataTablePacket {
    pub tick: u32,
    pub tables: Vec<SendTable>,
    pub server_classes: Vec<ServerClass>,
}

impl Parse for DataTablePacket {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;

        let mut parse_tables = Vec::new();
        while packet_data.read_bool()? {
            let table = ParseSendTable::parse(&mut packet_data, state)?;
            parse_tables.push(table);
        }

        let flat_props: Vec<_> = parse_tables
            .iter()
            .map(|table| table.flatten_props(&parse_tables))
            .collect();

        let tables = parse_tables
            .into_iter()
            .zip(flat_props.into_iter())
            .map(|(parse_table, flat)| SendTable {
                name: parse_table.name,
                props: parse_table.props,
                needs_decoder: parse_table.needs_decoder,
                flattened_props: flat,
            })
            .collect();

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
