use crate::demo::gamevent::GameEventDefinition;
use crate::demo::sendprop::SendProp;
use crate::Stream;
use std::collections::HashMap;
use crate::demo::packet::stringtable::StringTable;
use crate::demo::gameevent_gen::GameEventType;

#[derive(Default, Debug)]
pub struct ParserState {
    pub version: u32,
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<GameEventType, GameEventDefinition>,
    pub string_tables: Vec<StringTable>
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
}
