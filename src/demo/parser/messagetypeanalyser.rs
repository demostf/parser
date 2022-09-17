use crate::demo::data::DemoTick;
use crate::demo::message::{Message, MessageType};

use crate::demo::parser::handler::MessageHandler;

use crate::ParserState;

#[derive(Default)]
pub struct MessageTypeAnalyser {
    packet_types: Vec<MessageType>,
}

impl MessageHandler for MessageTypeAnalyser {
    type Output = Vec<MessageType>;

    fn does_handle(_message_type: MessageType) -> bool {
        true
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, _parser_state: &ParserState) {
        self.packet_types.push(message.get_message_type())
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.packet_types
    }
}
