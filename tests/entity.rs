use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fs;
use test_case::test_case;

use fnv::FnvHashMap;
use std::collections::HashMap;
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::message::packetentities::{EntityId, PacketEntity, UpdateType};
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{
    ParseSendTable, SendTableName, ServerClass, ServerClassName,
};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::{SendPropIdentifier, SendPropName, SendPropValue};
use tf_demo_parser::{Demo, DemoParser, MessageType, ParserState};

/// Compatible serialization with the js parser entity dumps
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PVSCompat {
    Preserve = 0,
    Leave = 2,
    Enter = 1,
    Delete = 6,
}

impl From<UpdateType> for PVSCompat {
    fn from(pvs: UpdateType) -> Self {
        match pvs {
            UpdateType::Preserve => PVSCompat::Preserve,
            UpdateType::Leave => PVSCompat::Leave,
            UpdateType::Enter => PVSCompat::Enter,
            UpdateType::Delete => PVSCompat::Delete,
        }
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EntityDump {
    tick: DemoTick,
    server_class: ServerClassName,
    id: EntityId,
    props: HashMap<String, SendPropValue>,
    pvs: PVSCompat,
}

impl EntityDump {
    pub fn from_entity(
        entity: PacketEntity,
        tick: DemoTick,
        classes: &[ServerClass],
        prop_names: &FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
        state: &ParserState,
    ) -> Self {
        EntityDump {
            tick,
            server_class: classes[usize::from(entity.server_class)].name.clone(),
            id: entity.entity_index,
            pvs: entity.update_type.into(),
            props: entity
                .props(state)
                .map(|prop| {
                    let (table_name, prop_name) = &prop_names[&prop.identifier];
                    (format!("{}.{}", table_name, prop_name), prop.value)
                })
                .collect(),
        }
    }
}

struct EntityDumper {
    entities: Vec<(DemoTick, PacketEntity)>,
    prop_names: FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
}

impl EntityDumper {
    pub fn new() -> Self {
        EntityDumper {
            entities: Vec::with_capacity(128),
            prop_names: FnvHashMap::default(),
        }
    }
}

impl MessageHandler for EntityDumper {
    type Output = Vec<EntityDump>;

    fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::PacketEntities => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: &Message, tick: DemoTick, _parser_state: &ParserState) {
        match message {
            Message::PacketEntities(entity_message) => self.entities.extend(
                entity_message
                    .entities
                    .iter()
                    .map(|entity| (tick, entity.clone())),
            ),
            _ => {}
        }
    }

    fn handle_data_tables(
        &mut self,
        tables: &[ParseSendTable],
        _server_classes: &[ServerClass],
        _parser_state: &ParserState,
    ) {
        for table in tables {
            for prop_def in &table.props {
                self.prop_names.insert(
                    prop_def.identifier(),
                    (table.name.clone(), prop_def.name.clone()),
                );
            }
        }
    }

    fn into_output(self, state: &ParserState) -> Self::Output {
        let prop_names = self.prop_names;
        self.entities
            .into_iter()
            .map(|(tick, entity)| {
                EntityDump::from_entity(entity, tick, &state.server_classes, &prop_names, state)
            })
            .collect()
    }
}

#[test_case("test_data/small.dem")]
fn entity_test(input_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, entities) = DemoParser::new_with_analyser(demo.get_stream(), EntityDumper::new())
        .parse()
        .unwrap();

    insta::with_settings!({sort_maps =>true}, {
        insta::assert_json_snapshot!(entities);
    });
}
