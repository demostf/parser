use pretty_assertions::assert_eq;
use std::fs;

use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser};

fn snapshot_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, state) = DemoParser::parse_demo(demo.get_stream()).unwrap();

    let expected: MatchState = serde_json::from_slice(
        fs::read(snapshot_file)
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
    assert_eq!(expected, state);
}

fn test_message_types(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, message_types) =
        DemoParser::parse_with_analyser(demo.get_stream(), MessageTypeAnalyser::new()).unwrap();

    let expected: Vec<MessageType> = serde_json::from_slice(
        fs::read(snapshot_file)
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
    assert_eq!(expected, message_types);
}

fn dump_message_types(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, message_types) =
        DemoParser::parse_with_analyser(demo.get_stream(), MessageTypeAnalyser::new()).unwrap();

    fs::write(snapshot_file, serde_json::to_vec(&message_types).unwrap()).unwrap();
}

#[test]
fn snapshot_test_small() {
    snapshot_test("data/small.dem", "data/small.json");
}

#[test]
fn snapshot_test_gully() {
    snapshot_test("data/gully.dem", "data/gully.json");
}

#[test]
fn snapshot_test_comp() {
    snapshot_test("data/comp.dem", "data/comp.json");
}

#[test]
fn snapshot_test_malformed_cvar() {
    snapshot_test("data/malformed_cvar.dem", "data/malformed_cvar.json");
}

#[test]
fn message_type_test_comp() {
    dump_message_types("data/comp.dem", "data/comp_message_types.json");
}
