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
use tf_demo_parser::demo::sendprop::SendPropDefinition;
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser, ParserState};

struct AllMessages;

impl MessageHandler for AllMessages {
    type Output = bool;

    fn does_handle(message_type: MessageType) -> bool {
        true
    }

    fn handle_message(&mut self, message: &Message, tick: u32) {
        test::black_box(message);
    }

    fn get_output(self, state: &ParserState) -> Self::Output {
        test::black_box(true)
    }
}

fn bench_all(input_file: &str, b: &mut Bencher) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let stream = demo.get_stream();
    b.iter(|| {
        let _ =
            test::black_box(DemoParser::parse_with_analyser(stream.clone(), AllMessages).unwrap());
    });
}

#[bench]
fn all_test_gully(b: &mut Bencher) {
    bench_all("data/gully.dem", b);
}
