use std::collections::HashMap;

use crate::demo::gameevent_gen::GameEventType;
use crate::demo::gamevent::GameEventDefinition;
use crate::demo::message::Message;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{SendTable, ServerClass};
use crate::demo::packet::Packet;
use crate::demo::packet::stringtable::StringTable;
use crate::demo::sendprop::SendProp;
use crate::Stream;

#[derive(Default, Debug)]
pub struct ParserState {
    pub version: u32,
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<GameEventType, GameEventDefinition>,
    pub string_tables: Vec<StringTable>,
    pub entity_classes: HashMap<EntityId, ServerClass>,
    pub send_tables: HashMap<String, SendTable>,
    pub server_classes: Vec<ServerClass>,
    pub instance_baselines: [HashMap<EntityId, Vec<SendProp>>; 2],
    pub tick: u32,
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

    pub fn handle_packet(&mut self, packet: Packet) {
        match packet {
            Packet::Message(packet) | Packet::Sigon(packet) => {
                for message in packet.messages {
                    self.handle_message(message);
                }
            }
            _ => {}
        }
    }

    fn handle_message(&mut self, message: Message) {
        match message {
            Message::GameEventList(message) => {
                self.event_definitions = message.event_list;
            }
            _ => {}
        }
    }
}
