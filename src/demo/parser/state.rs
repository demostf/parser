use std::collections::HashMap;

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::GameEventDefinition;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::demo::message::packetentities::EntityId;
use crate::demo::message::stringtable::StringTableMeta;
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{SendTable, ServerClass};
use crate::demo::packet::stringtable::{StringTable, StringTableEntry};
use crate::demo::packet::Packet;
use crate::demo::parser::handler::{MessageHandler, StringTableEntryHandler};
use crate::demo::sendprop::SendProp;
use crate::Stream;
use std::mem::replace;

pub type StringEntryHandler = Box<FnMut(&String, &StringTableEntry) -> ()>;

#[derive(Default)]
pub struct ParserState {
    pub version: u16,
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<GameEventTypeId, GameEventDefinition>,
    pub string_tables: Vec<StringTable>,
    pub entity_classes: HashMap<EntityId, ServerClass>,
    pub send_tables: HashMap<String, SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>>; 2],
    pub game: String,
    pub interval_per_tick: f32,
}

pub struct StaticBaseline {
    class_id: u32,
    raw: Stream,
    parsed: Option<Vec<SendProp>>,
}

impl StaticBaseline {
    fn new(class_id: u32, raw: Stream) -> Self {
        StaticBaseline {
            class_id,
            raw,
            parsed: None,
        }
    }
}

impl ParserState {
    pub fn new() -> Self {
        ParserState::default()
    }

    pub fn handle_data_table(
        &mut self,
        send_tables: Vec<SendTable>,
        server_classes: Vec<ServerClass>,
    ) {
        for table in send_tables {
            self.send_tables.insert(table.name.clone(), table);
        }
        self.server_classes = server_classes
    }

    pub fn handle_string_table(&mut self, table: StringTable) {
        self.string_tables.push(table);
    }

    pub fn handle_string_table_update(
        &mut self,
        table_id: u8,
        entries: HashMap<u16, StringTableEntry>,
    ) {
        match self.string_tables.get_mut(table_id as usize) {
            Some(table) => {
                for (index, entry) in entries {
                    let index = index as usize;
                    if index > table.entries.len() {
                        table.entries.resize(index, StringTableEntry::default())
                    }
                    unsafe {
                        replace(table.entries.get_unchecked_mut(index), entry);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

impl MessageHandler for ParserState {
    fn does_handle(&self, message_type: MessageType) -> bool {
        match message_type {
            MessageType::ServerInfo
            | MessageType::GameEventList
            | MessageType::CreateStringTable
            | MessageType::UpdateStringTable => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: Message, _tick: u32) {
        match message {
            Message::ServerInfo(message) => {
                self.version = message.version;
                self.game = message.game;
                self.interval_per_tick = message.interval_per_tick
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            _ => {}
        }
    }
}

impl StringTableEntryHandler for ParserState {
    fn handle_string_entry(&mut self, table: &String, index: usize, entry: &StringTableEntry) {
        match table.as_str() {
            "instancebaseline" => match &entry.extra_data {
                Some(extra) => match entry.text.parse::<u32>() {
                    Ok(class_id) => {
                        let baseline = StaticBaseline::new(class_id, extra.data.clone());
                        self.static_baselines.insert(class_id, baseline);
                    }
                    Err(_) => {}
                },
                _ => unreachable!("missing baseline"),
            },
            _ => {}
        }
    }
}
