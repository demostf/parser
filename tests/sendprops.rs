use std::fs;
use test_case::test_case;

use std::collections::{HashMap, HashSet};
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName, ServerClass};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::{Demo, DemoParser, MessageType, ParserState};

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

    fn does_handle(_message_type: MessageType) -> bool {
        false
    }

    fn handle_data_tables(&mut self, tables: &[ParseSendTable], _server_classes: &[ServerClass]) {
        self.tables = tables.to_vec()
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.tables
    }
}

#[test_case("data/gully.dem", "data/gully_props.json"; "gully.dem")]
fn flatten_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, send_tables) =
        DemoParser::new_with_analyser(demo.get_stream(), SendPropAnalyser::new())
            .parse()
            .expect("Failed to parse");
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

    pretty_assertions::assert_eq!(expected_tables, actual_tables);
    for table in expected_tables {
        pretty_assertions::assert_eq!(expected[table], flat_props[table]);
    }
}
