#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(test)]

extern crate test;

use pretty_assertions::assert_eq;
use std::fs;

use std::collections::{HashMap, HashSet};
use test::Bencher;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::RawSendPropDefinition;
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser, ParserState};

pub struct SendPropAnalyser;

// impl MessageHandler for SendPropAnalyser {
//     type Output = Vec<ParseSendTable>;
//
//     fn does_handle(message_type: MessageType) -> bool {
//         false
//     }
//
//     fn into_output(self, state: &ParserState) -> Self::Output {
//         state
//             .send_tables
//             .iter()
//             .map(|v| ParseSendTable {
//                 name: v.name.clone(),
//                 props: v.props.clone(),
//                 needs_decoder: v.needs_decoder,
//             })
//             .collect()
//     }
// }
//
// fn flatten_bench(input_file: &str, b: &mut Bencher) {
//     let file = fs::read(input_file).expect("Unable to read file");
//     let demo = Demo::new(&file);
//     let stream = demo.get_stream();
//     let (_, send_tables) = DemoParser::new_with_analyser(stream.clone(), SendPropAnalyser)
//         .parse()
//         .unwrap();
//     b.iter(|| {
//         let flat: Vec<_> = send_tables
//             .iter()
//             .map(|table| table.flatten_props(&send_tables))
//             .collect();
//         test::black_box(flat);
//     });
// }
//
// #[bench]
// fn sendprop_test_gully(b: &mut Bencher) {
//     flatten_bench("data/gully.dem", b);
// }
