use bitstream_reader::{BitRead, LittleEndian};

use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};
use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};
use std::fmt;
use serde::{Serialize, Deserialize};
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
pub struct SendTable {
    pub name: SendTableName,
    pub props: Vec<SendPropDefinition>,
    pub needs_decoder: bool,
    pub flattened_props: Option<Vec<SendPropDefinition>>,
}

impl Parse for SendTable {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let needs_decoder = stream.read()?;
        let raw_name: String = stream.read()?;
        let name: SendTableName = raw_name.into();
        let prop_count = stream.read_int(10)?;

        let mut array_element_prop = None;
        let mut props = Vec::with_capacity(prop_count);

        for _ in 0..prop_count {
            let prop: SendPropDefinition =
                SendPropDefinition::read(stream, name.clone())?;
            if prop.flags.contains(SendPropFlag::InsideArray) {
                if array_element_prop.is_some()
                    || prop.flags.contains(SendPropFlag::ChangesOften)
                {
                    return Err(ParseError::InvalidSendPropArray(
                        "Array contents can't have the 'ChangesOften' flag".to_owned(),
                    ));
                }
                array_element_prop = Some(prop);
            } else if let Some(array_element) = array_element_prop {
                if prop.prop_type != SendPropType::Array {
                    return Err(ParseError::InvalidSendPropArray(
                        "Array contents can without array".to_owned(),
                    ));
                }
                array_element_prop = None;
                props.push(prop.with_array_property(array_element));
            } else {
                props.push(prop);
            }
        }

        Ok(SendTable {
            name,
            flattened_props: None,
            needs_decoder,
            props,
        })
    }
}

impl SendTable {
    pub fn flatten_props(&self, tables: &[SendTable]) -> Vec<SendPropDefinition> {
        let mut flat = Vec::new();
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

        flat
    }

    fn get_excludes<'a>(&'a self, tables: &'a [SendTable]) -> Vec<Exclude<'a>> {
        let mut excludes = Vec::new();

        for prop in self.props.iter() {
            if prop.is_exclude() && prop.table_name.is_some() {
                excludes.push(Exclude {
                    table: prop.table_name.as_ref().unwrap(),
                    prop: &prop.name,
                })
            } else if prop.prop_type == SendPropType::DataTable {
                if let Some(table) = prop.get_data_table(tables) {
                    excludes.extend_from_slice(&table.get_excludes(tables));
                }
            }
        }

        excludes
    }

    // TODO: below is a direct port from the js which is a direct port from C++ and not very optimal
    fn get_all_props(&self, tables: &[SendTable], excludes: &[Exclude], props: &mut Vec<SendPropDefinition>) {
        let mut local_props = Vec::new();

        self.get_all_props_iterator_props(tables, excludes, &mut local_props, props);
        props.extend_from_slice(&local_props);
    }

    fn get_all_props_iterator_props(&self, tables: &[SendTable], excludes: &[Exclude], props: &mut Vec<SendPropDefinition>, child_props: &mut Vec<SendPropDefinition>) {
        for prop in self.props.iter() {
            if prop.is_exclude() {
                continue;
            }

            if excludes.iter().any(|exclude| exclude.matches(prop)) {
                continue;
            }

            if prop.prop_type == SendPropType::DataTable {
                if let Some(table) = prop.get_data_table(tables) {
                    if prop.flags.contains(SendPropFlag::Collapsible) {
                        table.get_all_props_iterator_props(tables, excludes, props, child_props);
                    } else {
                        table.get_all_props(tables, excludes, child_props);
                    }
                }
            } else {
                props.push(prop.clone());
            }
        }
    }
}

#[derive(Clone)]
struct Exclude<'a> {
    table: &'a SendTableName,
    prop: &'a str,
}

impl<'a> Exclude<'a> {
    pub fn matches(&self, prop: &SendPropDefinition) -> bool {
        self.prop == prop.name && self.table == &prop.owner_table
    }
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

        let mut tables = Vec::new();
        while packet_data.read_bool()? {
            let table = SendTable::parse(&mut packet_data, state)?;
            tables.push(table);
        }

        // TODO linked tables?

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
