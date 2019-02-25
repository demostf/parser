use crate::{Parse, ParseError, ParserState, Result, Stream};
use crate::demo::sendprop::{SendPropDefinition, SendPropFlag, SendPropType};

pub struct ServerClass {
    id: u32,
    name: String,
    data_table: String,
}

pub struct SendTable {
    name: String,
    props: Vec<SendPropDefinition>,
    needs_decoder: bool,
    flattened_props: Option<Vec<SendPropDefinition>>,
}

pub struct DataTablePacket {
    tick: u32,
    tables: Vec<SendTable>,
    server_classes: Vec<ServerClass>,
}

impl<'a> Parse<'a> for DataTablePacket {
    fn parse(stream: &mut Stream<'a>, state: &ParserState<'a>) -> Result<Self> {
        let tick = stream.read(32)?;
        let _len = stream.read::<u32>(32)?;

        let mut tables = vec![];
        while stream.read_bool()? {
            let needs_decoder = stream.read_bool()?;
            let name = stream.read_string(None)?;
            let prop_count = stream.read(10)?;

            let mut array_element_prop = None;
            let mut props = Vec::with_capacity(prop_count);

            for i in 0..prop_count {
                let prop: SendPropDefinition = SendPropDefinition::parse(stream, state, name.clone())?;
                if prop.flags.contains(SendPropFlag::InsideArray) {
                    if array_element_prop.is_some() || prop.flags.contains(SendPropFlag::ChangesOften) {
                        return Err(ParseError::InvalidSendPropArray);
                    }
                    array_element_prop = Some(prop);
                } else if let Some(array_element) = array_element_prop {
                    if prop.prop_type == SendPropType::Array {
                        return Err(ParseError::InvalidSendPropArray);
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

        let server_class_count = stream.read(16)?;
        let mut server_classes = Vec::with_capacity(server_class_count);
        for i in 0..server_class_count {
            let id = stream.read(16)?;
            let name = stream.read_string(None)?;
            let data_table = stream.read_string(None)?;
            server_classes.push(ServerClass {
                id,
                name,
                data_table,
            });
        }

        Ok(DataTablePacket {
            tick,
            tables,
            server_classes,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32)?;
        let len = stream.read(32)?;
        stream.skip(len).map_err(ParseError::from)
    }
}