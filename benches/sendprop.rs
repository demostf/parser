#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use pretty_assertions::assert_eq;
use std::fs;

use std::collections::{HashMap, HashSet};
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::RawSendPropDefinition;
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser, ParserState};

pub struct SendPropAnalyser;

impl MessageHandler for SendPropAnalyser {
    type Output = Vec<ParseSendTable>;

    fn does_handle(message_type: MessageType) -> bool {
        false
    }

    fn into_output(self, state: &ParserState) -> Self::Output {
        state
            .send_tables
            .iter()
            .map(|v| ParseSendTable {
                name: v.name.clone(),
                props: v.raw_props.clone(),
                needs_decoder: v.needs_decoder,
            })
            .collect()
    }
}

fn flatten_bench(input_file: &str, b: &mut Criterion) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let stream = demo.get_stream();
    let (_, send_tables) = DemoParser::new_with_analyser(stream.clone(), SendPropAnalyser)
        .parse()
        .unwrap();
    b.bench_function(&format!("flatten {}", input_file), |b| {
        b.iter(|| {
            let flat: Vec<_> = send_tables
                .iter()
                .map(|table| table.flatten_props(&send_tables))
                .collect();
            black_box(flat);
        })
    });
}

fn sendprop_test_gully(b: &mut Criterion) {
    flatten_bench("test_data/gully.dem", b);
}

criterion_group!(benches, sendprop_test_gully);
criterion_main!(benches);
