use std::collections::HashMap;

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
    pub event_definitions: HashMap<GameEventTypeId, GameEventDefinition>,
    pub string_tables: Vec<StringTableMeta>,
    pub entity_classes: HashMap<EntityId, Rc<ServerClass>>,
    pub send_tables: HashMap<SendTableName, SendTable>,
    pub server_classes: Vec<Rc<ServerClass>>,
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
            event_definitions: HashMap::new(),
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
        server_classes: Vec<Rc<ServerClass>>,
    ) {
        for table in send_tables {
            self.send_tables.insert(table.name.clone(), table);
        }
        self.server_classes = server_classes
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
}

impl MessageHandler for ParserState {
    type Output = Self;

    fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::ServerInfo
            | MessageType::GameEventList
            | MessageType::CreateStringTable
            | MessageType::PacketEntities
            | MessageType::UpdateStringTable => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: Message, _tick: u32) {
        match message {
            Message::ServerInfo(message) => {
                self.demo_meta.version = message.version;
                self.demo_meta.game = message.game;
                self.demo_meta.interval_per_tick = message.interval_per_tick
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            Message::PacketEntities(message) => {
                if message.updated_base_line {
                    let old_index = message.base_line as usize;
                    let new_index = 1 - old_index;
                    self.instance_baselines.swap(0, 1);
                    //self.instance_baselines[new_index] = self.instance_baselines[new_index].clone();

                    for entity in message.entities.iter() {
                        self.instance_baselines[new_index]
                            .insert(entity.entity_index, entity.props.clone());
                    }
                }

                for removed in message.removed_entities.iter() {
                    self.entity_classes.remove(removed);
                }

                for entity in message.entities.iter() {
                    if entity.pvs == PVS::Delete {
                        self.entity_classes.remove(&entity.entity_index);
                    }
                    self.entity_classes
                        .insert(entity.entity_index, Rc::clone(&entity.server_class));
                }
            }
            _ => {}
        }
    }

    fn handle_string_entry(&mut self, table: &String, _index: usize, entry: &StringTableEntry) {
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

    fn get_output(self, _state: ParserState) -> Self {
        self
    }
}
