use crate::demo::gamevent::GameEventDefinition;
use crate::demo::sendprop::SendProp;
use std::collections::HashMap;
use crate::Stream;

pub struct ParserState<'a> {
    pub version: u32,
    pub static_baselines: HashMap<u32, StaticBaseline<'a>>,
    pub event_definitions: HashMap<u32, GameEventDefinition>,
}

pub struct StaticBaseline<'a> {
    class_id: u32,
    raw: Stream<'a>,
    parsed: Option<Vec<SendProp>>,
}

impl<'a> ParserState<'a> {
    pub fn new(_stream: &Stream<'a>) -> Self {
        ParserState {
            version: 0,
            static_baselines: HashMap::new(),
            event_definitions: HashMap::new(),
        }
    }
}