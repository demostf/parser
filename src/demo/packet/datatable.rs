use bitstream_reader::{BitRead, LittleEndian};

use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};
use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};
use strum_macros::{EnumString, Display};

#[derive(BitRead, Debug)]
pub struct ServerClass {
    pub id: u16,
    pub name: String,
    pub data_table: String,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, EnumString, Display)]
pub enum SendTableName {
    #[strum(default = "true")]
    Other(String)
}

impl From<String> for SendTableName {
    fn from(value: String) -> Self {
        value.parse().unwrap()
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
        let name: String = stream.read()?;
        let prop_count = stream.read_int(10)?;

        let mut array_element_prop = None;
        let mut props = Vec::with_capacity(prop_count);

        for _ in 0..prop_count {
            let prop: SendPropDefinition =
                SendPropDefinition::read(stream)?;
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
            name: SendTableName::from(name),
            flattened_props: None,
            needs_decoder,
            props,
        })
    }
}

//impl SendTable {
//    fn get_excludes(&self) -> ExcludesIterator {
//        ExcludesIterator {
//            table: self,
//            index: 0,
//        }
//    }
//}
//
//struct ExcludesIterator<'a> {
//    table: &'a SendTable,
//    index: usize,
//}
//
//impl<'a> Iterator for ExcludesIterator<'a> {
//    type Item = &'a SendPropDefinition;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        while let Some(prop) = self.table.props.get(index) {
//            index += 1;
//            if prop.flags.contains(SendPropFlag::Exclude) {
//                return Some(prop);
//            } else if prop.prop_type == SendPropType::DataTable {
//                if let Some(table)
//            }
//        }
//        None
//    }
//}

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
