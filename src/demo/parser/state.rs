use std::collections::HashMap;

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::GameEventDefinition;
use crate::demo::message::Message;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{SendTable, ServerClass};
use crate::demo::packet::Packet;
use crate::demo::packet::stringtable::{StringTable, StringTableEntry};
use crate::demo::sendprop::SendProp;
use crate::Stream;

#[derive(Default, Debug)]
pub struct ParserState {
    pub version: u16,
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<GameEventType, GameEventDefinition>,
    pub string_tables: Vec<StringTable>,
    pub entity_classes: HashMap<EntityId, ServerClass>,
    pub send_tables: HashMap<String, SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>>; 2],
    pub tick: u32,
    pub game: String,
}

#[derive(Debug)]
pub struct StaticBaseline {
    class_id: u32,
    raw: Stream,
    parsed: Option<Vec<SendProp>>,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState::default()
    }

    pub fn handle_packet(&mut self, packet: Packet) -> Vec<Message> {
        match packet {
            Packet::Message(packet) | Packet::Sigon(packet) => {
                let mut unhandled_messages = Vec::with_capacity(packet.messages.len());
                for message in packet.messages {
                    match self.handle_message(message) {
                        Some(message) => unhandled_messages.push(message),
                        _ => {}
                    }
                }
                return unhandled_messages;
            }
            Packet::DataTables(packet) => {
                if self.send_tables.len() > 0 {
                    unreachable!("overwriting tables");
                }
                for table in packet.tables {
                    self.send_tables.insert(table.name.clone(), table);
                }
                self.server_classes = packet.server_classes;
            }
            Packet::StringTables(packet) => {
                for table in packet.tables {
                    self.handle_table(table);
                }
            }
            _ => {}
        }
        Vec::new()
    }

    fn handle_message(&mut self, message: Message) -> Option<Message> {
        match message {
            Message::NetTick(message) => self.tick = message.tick,
            Message::ServerInfo(message) => {
                self.version = message.version;
                self.game = message.game;
            }
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            Message::CreateStringTable(message) => {
                self.handle_table(message.table);
            }
            Message::UpdateStringTable(message) => {
                self.handle_table_update(message.table_id, message.entries);
            }
            _ => return Some(message)
        }
        None
    }

    fn handle_table(&mut self, table: StringTable) {
        self.string_tables.push(table);
    }

    fn handle_table_update(&mut self, table_id: u8, entries: HashMap<u16, StringTableEntry>) {
        let mut table = self.string_tables.get_mut(table_id as usize);
        match table {
            Some(table) => {
                for (index, entry) in entries {
                    table.entries.insert(index as usize, entry);
                }
            }
            _ => unreachable!("trying to update non existing table"),
        }
    }
}
