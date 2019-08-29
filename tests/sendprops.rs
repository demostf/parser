#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use pretty_assertions::assert_eq;
use std::fs;

use std::collections::{HashMap, HashSet};
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTable, SendTableName};
use tf_demo_parser::demo::packet::stringtable::StringTableEntry;
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::SendPropDefinition;
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser, ParserState};

pub struct SendPropAnalyser {
    tables: Vec<ParseSendTable>,
}

impl SendPropAnalyser {
    pub fn new() -> Self {
        SendPropAnalyser { tables: Vec::new() }
    }
}

impl MessageHandler for SendPropAnalyser {
    type Output = Vec<ParseSendTable>;

    fn does_handle(message_type: MessageType) -> bool {
        false
    }

    fn handle_data_tables(&mut self, tables: &[SendTable]) {
        self.tables = tables
            .iter()
            .map(|v| ParseSendTable {
                name: v.name.clone(),
                props: v.props.clone(),
                needs_decoder: v.needs_decoder,
            })
            .collect()
    }

    fn get_output(self, state: ParserState) -> Self::Output {
        self.tables
    }
}

fn flatten_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, send_tables) =
        DemoParser::parse_with_analyser(demo.get_stream(), SendPropAnalyser::new()).unwrap();
    let flat_props: HashMap<SendTableName, Vec<String>> = send_tables
        .iter()
        .map(|table| {
            (
                table.name.clone(),
                table
                    .flatten_props(&send_tables)
                    .into_iter()
                    .map(|prop| format!("{}.{}", prop.owner_table, prop.name))
                    .collect(),
            )
        })
        .collect();

    let expected: HashMap<SendTableName, Vec<String>> = serde_json::from_slice(
        fs::read(snapshot_file)
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();

    let expected_tables: HashSet<_> = expected.keys().collect();
    let actual_tables: HashSet<_> = flat_props.keys().collect();

    assert_eq!(expected_tables, actual_tables);
    for table in expected_tables {
        assert_eq!(expected[table], flat_props[table]);
    }
}

#[test]
fn sendprop_test_gully() {
    flatten_test("data/gully.dem", "data/gully_props.json");
}
