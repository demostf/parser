use std::fs;
use pretty_assertions::assert_eq;

use tf_demo_parser::{Demo, DemoParser, Stream, MatchState};

fn snapshot_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let stream: Stream = demo.get_stream();
    let parser = DemoParser::new(stream);
    let (_, state) = parser.parse_demo().unwrap();

    let expected: MatchState = serde_json::from_slice(fs::read(snapshot_file).expect("Unable to read file").as_slice()).unwrap();
    assert_eq!(expected, state);
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