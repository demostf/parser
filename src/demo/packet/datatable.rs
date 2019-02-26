use crate::{Parse, ParseError, ParserState, Result, Stream};
use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};

#[derive(Debug)]
pub struct ServerClass {
    id: u32,
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

impl<'a> Parse<'a> for DataTablePacket {
    fn parse(stream: &mut Stream<'a>, state: &ParserState<'a>) -> Result<Self> {
        let tick = stream.read(32)?;
        let start = stream.pos();
        let len = stream.read::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;

        let mut tables = vec![];
        while packet_data.read_bool()? {
            let needs_decoder = packet_data.read_bool()?;
            let name = packet_data.read_string(None)?;
            let prop_count = packet_data.read(10)?;

            let mut array_element_prop = None;
            let mut props = Vec::with_capacity(prop_count);

            for i in 0..prop_count {
                let prop: SendPropDefinition = SendPropDefinition::parse(&mut packet_data, state, name.clone())?;
                if prop.flags.contains(SendPropFlag::InsideArray) {
                    if array_element_prop.is_some() || prop.flags.contains(SendPropFlag::ChangesOften) {
                        return Err(ParseError::InvalidSendPropArray("Array contents can't have the 'ChangesOften' flag".to_owned()));
                    }
                    array_element_prop = Some(prop);
                } else if let Some(array_element) = array_element_prop {
                    if prop.prop_type != SendPropType::Array {
                        return Err(ParseError::InvalidSendPropArray("Array contents can without array".to_owned()));
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

        let server_class_count = packet_data.read(16)?;
        let mut server_classes = Vec::with_capacity(server_class_count);
        for i in 0..server_class_count {
            let id = packet_data.read(16)?;
            let name = packet_data.read_string(None)?;
            let data_table = packet_data.read_string(None)?;
            server_classes.push(ServerClass {
                id,
                name,
                data_table,
            });
        }

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

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32)?;
        let len = stream.read::<usize>(32)?;
        stream.skip(len * 8).map_err(ParseError::from)
    }
}