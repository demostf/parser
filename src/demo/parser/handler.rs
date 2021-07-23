use crate::demo::message::{Message, MessageType};
use crate::demo::packet::datatable::{ParseSendTable, ServerClass};
use crate::demo::packet::stringtable::{StringTable, StringTableEntry};
use crate::demo::packet::Packet;
use crate::demo::parser::analyser::Analyser;
use crate::Result;

use crate::demo::header::Header;
use crate::ParserState;
use std::borrow::Cow;

pub trait MessageHandler {
    type Output;

    fn does_handle(message_type: MessageType) -> bool;

    fn handle_message(&mut self, _message: &Message, _tick: u32) {}

    fn handle_string_entry(&mut self, _table: &str, _index: usize, _entries: &StringTableEntry) {}

    fn handle_data_tables(&mut self, _tables: &[ParseSendTable], _server_classes: &[ServerClass]) {}

    fn into_output(self, state: &ParserState) -> Self::Output;
}

pub trait BorrowMessageHandler: MessageHandler {
    fn borrow_output(&self, state: &ParserState) -> &Self::Output;
}

pub struct NullHandler;

impl MessageHandler for NullHandler {
    type Output = ();

    fn does_handle(_message_type: MessageType) -> bool {
        false
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        ()
    }
}

#[derive(Clone)]
pub struct DemoHandler<'a, T: MessageHandler> {
    pub tick: u32,
    pub string_table_names: Vec<Cow<'a, str>>,
    analyser: T,
    pub state_handler: ParserState,
}

impl<'a> DemoHandler<'a, Analyser> {
    pub fn new() -> Self {
        Self::with_analyser(Analyser::new())
    }
}

impl<'a> Default for DemoHandler<'a, Analyser> {
    fn default() -> Self {
        DemoHandler::new()
    }
}

impl<'a, T: MessageHandler> DemoHandler<'a, T> {
    pub fn with_analyser(analyser: T) -> Self {
        let state_handler = ParserState::new(24, T::does_handle, false);

        DemoHandler {
            tick: 0,
            string_table_names: Vec::new(),
            analyser,
            state_handler,
        }
    }
    pub fn parse_all_with_analyser(analyser: T) -> Self {
        let state_handler = ParserState::new(24, T::does_handle, true);

        DemoHandler {
            tick: 0,
            string_table_names: Vec::new(),
            analyser,
            state_handler,
        }
    }

    pub fn handle_header(&mut self, header: &Header) {
        self.state_handler.protocol_version = header.protocol;
    }

    pub fn handle_packet(&mut self, packet: Packet<'a>) -> Result<()> {
        match packet {
            Packet::DataTables(packet) => {
                self.handle_data_table(packet.tables, packet.server_classes)?;
            }
            Packet::StringTables(packet) => {
                for table in packet.tables.into_iter() {
                    self.handle_string_table(table)
                }
            }
            Packet::Message(packet) | Packet::Sigon(packet) => {
                //self.tick = packet.tick;
                for message in packet.messages {
                    match message {
                        Message::NetTick(message) => self.tick = message.tick,
                        Message::CreateStringTable(message) => {
                            self.handle_string_table(message.table)
                        }
                        Message::UpdateStringTable(message) => {
                            self.handle_table_update(message.table_id, message.entries)
                        }
                        message => self.handle_message(message),
                    }
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_string_table(&mut self, table: StringTable<'a>) {
        self.state_handler
            .handle_string_table_meta(table.get_table_meta());
        for (entry_index, entry) in table.entries.into_iter() {
            let entry_index = entry_index as usize;
            self.state_handler
                .handle_string_entry(&table.name, entry_index, &entry);
            self.analyser
                .handle_string_entry(&table.name, entry_index, &entry);
        }

        self.string_table_names.push(table.name);
    }

    fn handle_table_update(&mut self, table_id: u8, entries: Vec<(u16, StringTableEntry<'a>)>) {
        if let Some(table_name) = self.string_table_names.get(table_id as usize) {
            for (index, entry) in entries {
                let index = index as usize;
                self.state_handler
                    .handle_string_entry(table_name, index, &entry);
                self.analyser.handle_string_entry(table_name, index, &entry);
            }
        }
    }

    fn handle_data_table(
        &mut self,
        send_tables: Vec<ParseSendTable>,
        server_classes: Vec<ServerClass>,
    ) -> Result<()> {
        self.analyser
            .handle_data_tables(&send_tables, &server_classes);
        self.state_handler
            .handle_data_table(send_tables, server_classes)
    }

    fn handle_message(&mut self, message: Message<'a>) {
        let message_type = message.get_message_type();
        if T::does_handle(message_type) {
            self.analyser.handle_message(&message, self.tick);
        }
        self.state_handler.handle_message(message, self.tick);
    }

    pub fn into_output(self) -> T::Output {
        self.analyser.into_output(&self.state_handler)
    }

    pub fn get_parser_state(&'a self) -> &ParserState {
        &self.state_handler
    }
}

impl<'a, T: MessageHandler + BorrowMessageHandler> DemoHandler<'a, T> {
    pub fn borrow_output(&self) -> &T::Output {
        self.analyser.borrow_output(&self.state_handler)
    }
}
