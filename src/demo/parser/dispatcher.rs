use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{SendTable, ServerClass};
use crate::demo::packet::stringtable::{StringTable, StringTableEntry};
use crate::demo::packet::Packet;
use crate::ParserState;

pub trait MessageHandler {
    fn does_handle(&self, message_type: MessageType) -> bool;

    fn handle_message(&mut self, message: Message, tick: u32);
}

pub trait StringTableEntryHandler {
    fn handle_string_entry(&mut self, table: &String, index: usize, entries: &StringTableEntry);
}

#[derive(Default)]
pub struct Dispatcher {
    tick: u32,
    string_table_names: Vec<String>,
    on_message: Vec<Rc<RefCell<MessageHandler>>>,
    on_string_table_entry: Vec<Rc<RefCell<StringTableEntryHandler>>>,
    state_handler: Option<Rc<RefCell<ParserState>>>,
}

impl Dispatcher {
    pub fn register_message_handler(&mut self, handler: Rc<RefCell<MessageHandler>>) {
        self.on_message.push(handler);
    }

    pub fn set_state_handler(&mut self, handler: Rc<RefCell<ParserState>>) {
        self.state_handler = Some(handler);
    }

    pub fn register_string_table_entry_handler(
        &mut self,
        handler: Rc<RefCell<StringTableEntryHandler>>,
    ) {
        self.on_string_table_entry.push(handler);
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

        for (entry_index, entry) in table.entries.iter().enumerate() {
            self.handle_string_entry(&table.name, entry_index, entry);
        }

        if let Some(handler) = &self.state_handler {
            handler.borrow_mut().handle_string_table(table);
        }
    }

    fn handle_table_update(&mut self, table_id: u8, entries: HashMap<u16, StringTableEntry>) {
        let table_name = self.string_table_names.get(table_id as usize);
        match table_name {
            Some(table_name) => {
                let table_name = table_name.clone();
                for (index, entry) in &entries {
                    self.handle_string_entry(&table_name, *index as usize, entry);
                }
            }
            _ => unreachable!("trying to update non existing table"),
        }
    }

    fn handle_data_table(&mut self, send_tables: Vec<SendTable>, server_classes: Vec<ServerClass>) {
        if let Some(handler) = &self.state_handler {
            handler
                .borrow_mut()
                .handle_data_table(send_tables, server_classes);
        }
    }

    fn handle_string_entry(&mut self, table: &String, index: usize, entries: &StringTableEntry) {
        for handler in self.on_string_table_entry.iter() {
            handler
                .borrow_mut()
                .handle_string_entry(table, index, entries);
        }
    }

    fn handle_message(&mut self, message: Message) {
        let message_type = message.get_message_type();
        for handler in self.on_message.iter() {
            let mut handler = handler.borrow_mut();
            if handler.does_handle(message_type) {
                handler.handle_message(message, self.tick);
                return;
            }
        }
    }
}
