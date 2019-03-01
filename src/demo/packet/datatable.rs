use bitstream_reader::BitRead;

use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};
use crate::{Parse, ParseError, ParserState, Result, Stream};

#[derive(BitRead, Debug)]
pub struct ServerClass {
    id: u16,
    name: String,
    data_table: String,
}

#[derive(Debug)]
pub struct SendTable {
    name: String,
    props: Vec<SendPropDefinition>,
    needs_decoder: bool,
    flattened_props: Option<Vec<SendPropDefinition>>,
}

#[derive(Debug)]
pub struct DataTablePacket {
    tick: u32,
    tables: Vec<SendTable>,
    server_classes: Vec<ServerClass>,
}

impl Parse for DataTablePacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read()?;
        let start = stream.pos();
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;

        let mut tables = vec![];
        while packet_data.read_bool()? {
            let needs_decoder = packet_data.read()?;
            let name: String = packet_data.read()?;
            let prop_count = packet_data.read_int(10)?;

            let mut array_element_prop = None;
            let mut props = Vec::with_capacity(prop_count);

            for i in 0..prop_count {
                let prop: SendPropDefinition =
                    SendPropDefinition::read(&mut packet_data, name.clone())?;
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

            let table = SendTable {
                name,
                flattened_props: None,
                needs_decoder,
                props,
            };
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
