use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::fs::{self, File};
use test_case::test_case;

use fnv::FnvHashMap;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
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

#[test_case("test_data/small.dem", "test_data/small_entities.json"; "small.dem")]
fn entity_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, entities) = DemoParser::new_with_analyser(demo.get_stream(), EntityDumper::new())
        .parse()
        .unwrap();

    let json_file = File::open(snapshot_file).expect("Unable to read file");
    let mut reader = BufReader::new(json_file);
    let mut buffer = String::new();

    let mut expected = Vec::with_capacity(128);

    while reader.read_line(&mut buffer).expect("failed to read line") > 0 {
        let entity: EntityDump =
            serde_json::from_str(buffer.trim_end()).expect("failed to parse json");
        expected.push(entity);
        buffer.clear();
    }

    pretty_assertions::assert_eq!(expected.len(), entities.len());

    let entity_ids: Vec<_> = entities.iter().map(|entity| entity.id).collect();
    let expected_ids: Vec<_> = expected.iter().map(|entity| entity.id).collect();

    pretty_assertions::assert_eq!(expected_ids, entity_ids);

    for (expected_entity, entity) in expected.into_iter().zip(entities.into_iter()) {
        pretty_assertions::assert_eq!(
            expected_entity.tick,
            entity.tick,
            "Failed comparing entity {}",
            entity.id
        );
        pretty_assertions::assert_eq!(
            expected_entity.id,
            entity.id,
            "Failed comparing entity {}",
            entity.id
        );
        pretty_assertions::assert_eq!(
            expected_entity.server_class,
            entity.server_class,
            "Failed comparing entity {}",
            entity.id
        );
        pretty_assertions::assert_eq!(
            expected_entity.pvs,
            entity.pvs,
            "Failed comparing entity {}",
            entity.id
        );
        let mut prop_names: Vec<_> = entity.props.keys().collect();
        let mut expected_prop_names: Vec<_> = expected_entity.props.keys().collect();
        prop_names.sort();
        expected_prop_names.sort();

        pretty_assertions::assert_eq!(
            expected_prop_names,
            prop_names,
            "Failed comparing entity {}",
            entity.id
        );

        for prop_name in expected_prop_names {
            pretty_assertions::assert_eq!(
                expected_entity.props.get(prop_name),
                entity.props.get(prop_name),
                "Failed comparing entity {} prop {}",
                entity.id,
                prop_name
            );
        }

        pretty_assertions::assert_eq!(
            expected_entity,
            entity,
            "Failed comparing entity {}",
            entity.id
        );
    }
}
