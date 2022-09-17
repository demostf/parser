use std::fs;
use test_case::test_case;

use tf_demo_parser::demo::parser::gamestateanalyser::{GameState, GameStateAnalyser};
use tf_demo_parser::{Demo, DemoParser, MatchState};

#[test_case("small.dem", "small.json"; "small.dem")]
#[test_case("gully.dem", "gully.json"; "gully.dem")]
#[test_case("comp.dem", "comp.json"; "comp.dem")]
#[test_case("malformed_cvar.dem", "malformed_cvar.json"; "malformed_cvar.dem")]
#[test_case("unicode-saytext.dem", "unicode-saytext.json"; "unicode-saytext.dem")]
#[test_case("nousers.dem", "nousers.json"; "nousers.dem")]
#[test_case("decal.dem", "decal.json"; "decal.dem")]
#[test_case("saytext2.dem", "saytext2.json"; "saytext2.dem")]
#[test_case("emptysaytext.dem", "emptysaytext.json"; "emptysaytext.dem")]
#[test_case("protocol23.dem", "protocol23.json"; "protocol23.dem")]
fn snapshot_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(format!("test_data/{}", input_file)).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, state) = DemoParser::new(demo.get_stream()).parse().unwrap();
    //
    // fs::write(
    //     format!("test_data/{}", snapshot_file),
    //     serde_json::to_string_pretty(&state).unwrap(),
    // )
    // .unwrap();

    let expected: MatchState = serde_json::from_slice(
        fs::read(format!("test_data/{}", snapshot_file))
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
    pretty_assertions::assert_eq!(expected, state);

    let (_, state) = DemoParser::new_all(demo.get_stream()).parse().unwrap();
    pretty_assertions::assert_eq!(expected, state);
}

#[test_case("small.dem", "small_game_state.json"; "small.dem")]
#[test_case("gully.dem", "gully_game_state.json"; "gully.dem")]
fn game_state_test(input_file: &str, snapshot_file: &str) {
    let file = fs::read(format!("test_data/{}", input_file)).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, state) = DemoParser::new_with_analyser(demo.get_stream(), GameStateAnalyser::new())
        .parse()
        .unwrap();

    // fs::write(
    //     format!("test_data/{}", snapshot_file),
    //     serde_json::to_string_pretty(&state).unwrap(),
    // )
    // .unwrap();

    let expected: GameState = serde_json::from_slice(
        fs::read(format!("test_data/{}", snapshot_file))
            .expect("Unable to read file")
            .as_slice(),
    )
    .unwrap();
    pretty_assertions::assert_eq!(expected.players, state.players);
    pretty_assertions::assert_eq!(expected, state);
}
