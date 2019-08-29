use std::collections::{BTreeMap, HashMap};

use crate::demo::gamevent::GameEventDefinition;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::demo::message::packetentities::{EntityId, PacketEntitiesMessage, PVS};
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{ClassId, SendTable, SendTableName, ServerClass};
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::analyser::Analyser;
use crate::demo::parser::handler::MessageHandler;
use crate::demo::sendprop::SendProp;
use crate::{Result, Stream};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct DemoMeta {
    pub version: u16,
    pub game: String,
    pub interval_per_tick: f32,
}

pub struct ParserState {
    pub static_baselines: HashMap<ClassId, StaticBaseline>,
    pub parsed_static_baselines: RefCell<HashMap<ClassId, Vec<SendProp>>>,
    pub event_definitions: Vec<GameEventDefinition>,
    pub string_tables: Vec<StringTableMeta>,
    pub entity_classes: HashMap<EntityId, ClassId>,
    pub send_tables: HashMap<ClassId, SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>>; 2],
    pub demo_meta: DemoMeta,
    analyser_handles: fn(message_type: MessageType) -> bool,
    handle_entities: bool,
}

pub struct StaticBaseline {
    pub class_id: ClassId,
    pub raw: Stream,
}

impl StaticBaseline {
    fn new(class_id: ClassId, raw: Stream) -> Self {
        StaticBaseline { class_id, raw }
    }

    pub fn parse(&self, send_table: &SendTable) -> Result<Vec<SendProp>> {
        PacketEntitiesMessage::read_update(&mut self.raw.clone(), send_table)
    }
}

impl ParserState {
    pub fn new(analyser_handles: fn(message_type: MessageType) -> bool) -> Self {
        ParserState {
            static_baselines: HashMap::new(),
            parsed_static_baselines: RefCell::new(HashMap::new()),
            event_definitions: Vec::new(),
            string_tables: Vec::new(),
            entity_classes: HashMap::new(),
            send_tables: HashMap::new(),
            server_classes: Vec::new(),
            instance_baselines: [HashMap::new(), HashMap::new()],
            demo_meta: DemoMeta::default(),
            analyser_handles,
            handle_entities: analyser_handles(MessageType::PacketEntities),
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
                None => Vec::new(),
            },
        })
    }

    pub fn handle_data_table(
        &mut self,
        send_tables: Vec<SendTable>,
        server_classes: Vec<ServerClass>,
    ) {
        let mut send_tables: HashMap<SendTableName, SendTable> = send_tables
            .into_iter()
            .map(|table| (table.name.clone(), table))
            .collect();

        self.server_classes = server_classes;

        self.send_tables.reserve(self.server_classes.len());

        for class in self.server_classes.iter() {
            if let Some(table) = send_tables.remove(&class.data_table) {
                self.send_tables.insert(class.id, table);
            }
        }
    }

    pub fn handle_string_table_meta(&mut self, table: StringTableMeta) {
        self.string_tables.push(table);
    }

    pub fn should_parse_message(&self, message_type: MessageType) -> bool {
        if message_type == MessageType::PacketEntities {
            (self.analyser_handles)(message_type)
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

    pub fn handle_message(&mut self, message: Message, _tick: u32) -> Option<Message> {
        match message {
            Message::ServerInfo(message) => {
                self.demo_meta.version = message.version;
                self.demo_meta.game = message.game;
                self.demo_meta.interval_per_tick = message.interval_per_tick;
                None
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
                None
            }
            Message::PacketEntities(ent_message) => {
                if ent_message.updated_base_line {
                    let old_index = ent_message.base_line as usize;
                    let new_index = 1 - old_index;
                    self.instance_baselines.swap(0, 1);
                    //self.instance_baselines[new_index] = self.instance_baselines[new_index].clone();

                    for entity in ent_message.entities.iter() {
                        self.instance_baselines[new_index]
                            .insert(entity.entity_index, entity.props.clone());
                    }
                }

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
                Some(Message::PacketEntities(ent_message))
            }
            _ => Some(message),
        }
    }

    pub fn handle_string_entry(&mut self, table: &String, _index: usize, entry: &StringTableEntry) {
        match table.as_str() {
            "instancebaseline" => {
                if let (Some(extra), Ok(class_id)) = (&entry.extra_data, entry.text().parse()) {
                    let baseline = StaticBaseline::new(class_id, extra.data.clone());
                    self.static_baselines.insert(class_id, baseline);
                }
            }
            _ => {}
        }
    }
}
