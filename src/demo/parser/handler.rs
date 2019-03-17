use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{SendTable, ServerClass};
use crate::demo::packet::stringtable::{StringTable, StringTableEntry};
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::{Analyser, MatchState};
use crate::ParserState;

pub trait MessageHandler {
    fn does_handle(message_type: MessageType) -> bool;

    fn handle_message(&mut self, message: Message, tick: u32);
}

pub trait StringTableEntryHandler {
    fn handle_string_entry(&mut self, table: &String, index: usize, entries: &StringTableEntry);
}

#[derive(Default)]
pub struct DemoHandler {
    tick: u32,
    string_table_names: Vec<String>,
    analyser: Analyser,
    state_handler: ParserState,
}

impl DemoHandler {
    pub fn new() -> Self {
        let analyser = Analyser::new();
        let state_handler = ParserState::new();

        DemoHandler {
            tick: 0,
            string_table_names: Vec::new(),
            analyser,
            state_handler,
        }
    }

    pub fn handle_packet(&mut self, packet: Packet) {
        match packet {
            Packet::DataTables(packet) => {
                self.handle_data_table(packet.tables, packet.server_classes);
            }
            Packet::StringTables(packet) => {
                for table in packet.tables.into_iter() {
                    self.handle_string_table(table)
                }
            }
            Packet::Message(packet) | Packet::Sigon(packet) => {
                for message in packet.messages {
                    match message {
                        Message::NetTick(message) => self.tick = message.tick,
                        Message::CreateStringTable(message) => {
                            self.handle_string_table(message.table)
                        }
                        Message::UpdateStringTable(message) => {
                            self.handle_table_update(message.table_id, message.entries)
                        }
                        _ => self.handle_message(message),
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_string_table(&mut self, table: StringTable) {
        self.string_table_names.push(table.name.clone());

        self.state_handler
            .handle_string_table_meta(table.get_table_meta());
        for (entry_index, entry) in table.entries.into_iter().enumerate() {
            self.handle_string_entry(&table.name, entry_index, entry);
        }
    }

    fn handle_table_update(&mut self, table_id: u8, entries: Vec<(u16, StringTableEntry)>) {
        let table_name = self.string_table_names.get(table_id as usize);
        match table_name {
            Some(table_name) => {
                let table_name = table_name.clone();
                for (index, entry) in entries {
                    self.handle_string_entry(&table_name, index as usize, entry);
                }
            }
            _ => unreachable!("trying to update non existing table"),
        }
    }

    fn handle_data_table(&mut self, send_tables: Vec<SendTable>, server_classes: Vec<ServerClass>) {
        self.state_handler
            .handle_data_table(send_tables, server_classes);
    }

    fn handle_string_entry(&mut self, table: &String, index: usize, entries: StringTableEntry) {
        self.state_handler
            .handle_string_entry(table, index, &entries);
        self.analyser.handle_string_entry(table, index, &entries);
    }

    fn handle_message(&mut self, message: Message) {
        let message_type = message.get_message_type();
        if ParserState::does_handle(message_type) {
            self.state_handler.handle_message(message, self.tick);
            return;
        }
        if Analyser::does_handle(message_type) {
            self.analyser.handle_message(message, self.tick);
            return;
        }
    }

    pub fn get_match_state(self) -> MatchState {
        self.analyser.get_match_state(self.state_handler)
    }

    pub fn get_parser_state(&self) -> &ParserState {
        &self.state_handler
    }
}
