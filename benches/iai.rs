use iai::black_box;
use std::fs;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::{Demo, DemoParser, MessageType, ParserState};

struct AllMessages;

impl MessageHandler for AllMessages {
    type Output = bool;

    fn does_handle(_message_type: MessageType) -> bool {
        true
    }

    fn handle_message(&mut self, message: &Message, _tick: u32) {
        black_box(message);
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        black_box(true)
    }
}

fn bench_all() {
    let file = fs::read("test_data/gully.dem").expect("Unable to read file");
    let demo = Demo::new(&file);
    let stream = demo.get_stream();
    black_box(
        DemoParser::new_with_analyser(stream.clone(), AllMessages)
            .parse()
            .unwrap(),
    );
}

iai::main!(bench_all);
