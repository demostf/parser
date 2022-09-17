#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pretty_assertions::assert_eq;
use std::fs;

use std::collections::{HashMap, HashSet};
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::RawSendPropDefinition;
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser, ParserState};

struct AllMessages;

impl MessageHandler for AllMessages {
    type Output = bool;

    fn does_handle(message_type: MessageType) -> bool {
        true
    }

    fn handle_message(&mut self, message: &Message, tick: DemoTick, _parser_state: &ParserState) {
        black_box(message);
    }

    fn into_output(self, state: &ParserState) -> Self::Output {
        black_box(true)
    }
}

fn bench_all(input_file: &str, b: &mut Criterion) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let stream = demo.get_stream();
    b.bench_function("bench_all", |b| {
        b.iter(|| {
            let _ = black_box(
                DemoParser::new_with_analyser(stream.clone(), AllMessages)
                    .parse()
                    .unwrap(),
            );
        })
    });
}

fn all_test_gully(b: &mut Criterion) {
    bench_all("test_data/gully.dem", b);
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = all_test_gully
}
criterion_main!(benches);
