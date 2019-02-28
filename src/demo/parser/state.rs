use crate::demo::gamevent::GameEventDefinition;
use crate::demo::sendprop::SendProp;
use crate::Stream;
use std::collections::HashMap;

pub struct ParserState {
    pub version: u32,
    pub static_baselines: HashMap<u32, StaticBaseline>,
    pub event_definitions: HashMap<u32, GameEventDefinition>,
}

pub struct StaticBaseline {
    class_id: u32,
    raw: Stream,
    parsed: Option<Vec<SendProp>>,
}

impl ParserState {
    pub fn new() -> Self {
        ParserState {
            version: 0,
            static_baselines: HashMap::new(),
            event_definitions: HashMap::new(),
        }
    }
}
