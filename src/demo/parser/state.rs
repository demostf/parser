use fnv::FnvHashMap;
use std::collections::HashMap;

use crate::demo::gamevent::GameEventDefinition;

use crate::demo::message::packetentities::{EntityId, PacketEntitiesMessage, PVS};
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{
    ClassId, ParseSendTable, SendTable, SendTableName, ServerClass,
};
use crate::demo::packet::stringtable::StringTableEntry;

use crate::demo::sendprop::{SendProp, SendPropDefinition};
use crate::nullhasher::NullHasherBuilder;
use crate::{Result, Stream};
use std::cell::RefCell;
use std::convert::TryFrom;

#[derive(Default, Clone)]
pub struct DemoMeta {
    pub version: u16,
    pub game: String,
    pub interval_per_tick: f32,
}

#[derive(Clone)]
pub struct ParserState {
    pub static_baselines: HashMap<ClassId, StaticBaseline, NullHasherBuilder>,
    pub parsed_static_baselines: RefCell<HashMap<ClassId, Vec<SendProp>, NullHasherBuilder>>,
    pub event_definitions: Vec<GameEventDefinition>,
    pub string_tables: Vec<StringTableMeta>,
    pub entity_classes: HashMap<EntityId, ClassId, NullHasherBuilder>,
    pub send_tables: Vec<SendTable>, // indexed by ClassId
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>, NullHasherBuilder>; 2],
    pub demo_meta: DemoMeta,
    analyser_handles: fn(message_type: MessageType) -> bool,
    handle_entities: bool,
    parse_all: bool,
}

#[derive(Clone)]
pub struct StaticBaseline {
    pub class_id: ClassId,
    pub raw: Stream<'static>,
}

impl StaticBaseline {
    fn new(class_id: ClassId, raw: Stream<'static>) -> Self {
        StaticBaseline { class_id, raw }
    }

    pub fn parse(&self, send_table: &SendTable) -> Result<Vec<SendProp>> {
        let mut props = Vec::with_capacity(8);
        PacketEntitiesMessage::read_update(&mut self.raw.clone(), send_table, &mut props)?;
        Ok(props)
    }
}

impl<'a> ParserState {
    pub fn new(analyser_handles: fn(message_type: MessageType) -> bool, parse_all: bool) -> Self {
        ParserState {
            static_baselines: HashMap::with_hasher(NullHasherBuilder),
            parsed_static_baselines: RefCell::new(HashMap::with_hasher(NullHasherBuilder)),
            event_definitions: Vec::new(),
            string_tables: Vec::new(),
            entity_classes: HashMap::with_hasher(NullHasherBuilder),
            send_tables: Vec::new(),
            server_classes: Vec::new(),
            instance_baselines: [
                HashMap::with_hasher(NullHasherBuilder),
                HashMap::with_hasher(NullHasherBuilder),
            ],
            demo_meta: DemoMeta::default(),
            analyser_handles,
            handle_entities: analyser_handles(MessageType::PacketEntities) || parse_all,
            parse_all,
        }
    }

    pub fn get_static_baseline(
        &self,
        class_id: ClassId,
        send_table: &SendTable,
    ) -> Result<Vec<SendProp>> {
        let mut cached = self.parsed_static_baselines.borrow_mut();
        Ok(match cached.get(&class_id) {
            Some(props) => props.clone(),
            None => match self.static_baselines.get(&class_id) {
                Some(static_baseline) => {
                    let props = static_baseline.parse(send_table)?;
                    cached.entry(class_id).or_insert(props).clone()
                }
                None => Vec::with_capacity(8),
            },
        })
    }

    pub fn handle_data_table(
        &mut self,
        parse_tables: Vec<ParseSendTable>,
        server_classes: Vec<ServerClass>,
    ) -> Result<()> {
        if self.handle_entities {
            let flat_props: Vec<_> = parse_tables
                .iter()
                .map(|table| table.flatten_props(&parse_tables))
                .collect();

            let mut send_tables: FnvHashMap<SendTableName, SendTable> = parse_tables
                .into_iter()
                .zip(flat_props.into_iter())
                .map(|(parse_table, flat)| {
                    Ok((
                        parse_table.name.clone(),
                        SendTable {
                            name: parse_table.name,
                            needs_decoder: parse_table.needs_decoder,
                            flattened_props: flat
                                .into_iter()
                                .map(|raw| SendPropDefinition::try_from(raw.as_ref()))
                                .collect::<std::result::Result<Vec<_>, _>>()?,
                        },
                    ))
                })
                .collect::<Result<_>>()?;

            self.server_classes = server_classes;

            self.send_tables.reserve(self.server_classes.len());

            for class in self.server_classes.iter() {
                if let Some(table) = send_tables.remove(&class.data_table) {
                    self.send_tables.push(table);
                }
            }
        }

        Ok(())
    }

    pub fn handle_string_table_meta(&mut self, table: StringTableMeta) {
        self.string_tables.push(table);
    }

    pub fn should_parse_message(&self, message_type: MessageType) -> bool {
        self.parse_all
            || if message_type == MessageType::PacketEntities {
                self.handle_entities
            } else {
                Self::does_handle(message_type) || (self.analyser_handles)(message_type)
            }
    }

    pub fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::ServerInfo
            | MessageType::NetTick
            | MessageType::GameEventList
            | MessageType::CreateStringTable
            | MessageType::PacketEntities
            | MessageType::UpdateStringTable => true,
            _ => false,
        }
    }

    pub fn handle_message(&mut self, message: Message, _tick: u32) {
        match message {
            Message::ServerInfo(message) => {
                self.demo_meta.version = message.version;
                self.demo_meta.game = message.game;
                self.demo_meta.interval_per_tick = message.interval_per_tick;
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            Message::PacketEntities(ent_message) => {
                for removed in ent_message.removed_entities.iter() {
                    self.entity_classes.remove(removed);
                }

                for entity in ent_message.entities.iter() {
                    if entity.pvs == PVS::Delete {
                        self.entity_classes.remove(&entity.entity_index);
                    }
                    self.entity_classes
                        .insert(entity.entity_index, entity.server_class);
                }

                if ent_message.updated_base_line {
                    let old_index = ent_message.base_line as usize;
                    let new_index = 1 - old_index;
                    self.instance_baselines.swap(0, 1);

                    for entity in ent_message.entities {
                        self.instance_baselines[new_index]
                            .insert(entity.entity_index, entity.props);
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_string_entry(
        &mut self,
        table: &str,
        _index: usize,
        entry: &StringTableEntry<'a>,
    ) {
        match table {
            "instancebaseline" => {
                if let (Some(extra), Ok(class_id)) = (&entry.extra_data, entry.text().parse()) {
                    let baseline = StaticBaseline::new(class_id, extra.data.to_owned());
                    self.static_baselines.insert(class_id, baseline);
                }
            }
            _ => {}
        }
    }
}
