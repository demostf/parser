use pretty_assertions::assert_eq;
use std::fs;

use tf_demo_parser::demo::parser::gamestateanalyser::{GameState, GameStateAnalyser};
use tf_demo_parser::{Demo, DemoParser, MatchState, MessageType, MessageTypeAnalyser};

fn snapshot_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, state) = DemoParser::new(demo.get_stream()).parse().unwrap();

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
        DemoParser::new_with_analyser(demo.get_stream(), MessageTypeAnalyser::default())
            .parse()
            .unwrap();

    let expected: Vec<MessageType> = serde_json::from_slice(
        fs::read(snapshot_file)
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
    assert_eq!(expected, message_types);
}

fn game_state_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(file);
    let (_, state) = DemoParser::new_with_analyser(demo.get_stream(), GameStateAnalyser::new())
        .parse()
        .unwrap();

    let expected: GameState = serde_json::from_slice(
        fs::read(snapshot_file)
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
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

#[test]
fn snapshot_test_malformed_cvar() {
    snapshot_test("data/malformed_cvar.dem", "data/malformed_cvar.json");
}

#[test]
fn snapshot_test_unicode_chat() {
    snapshot_test("data/unicode-saytext.dem", "data/unicode-saytext.json");
}

#[test]
fn snapshot_test_player_in_update() {
    snapshot_test("data/nousers.dem", "data/nousers.json");
}

#[test]
fn snapshot_test_decal() {
    snapshot_test("data/decal.dem", "data/decal.json");
}

#[test]
fn game_state_test_small() {
    game_state_test("data/small.dem", "data/small_game_state.json");
}

#[test]
fn game_state_test_gully() {
    game_state_test("data/gully.dem", "data/gully_game_state.json");
}
