use std::fs;
use test_case::test_case;

use tf_demo_parser::demo::parser::gamestateanalyser::GameStateAnalyser;
use tf_demo_parser::{Demo, DemoParser};

#[test_case("small.dem")]
#[test_case("gully.dem")]
#[test_case("comp.dem")]
#[test_case("malformed_cvar.dem")]
#[test_case("unicode-saytext.dem")]
#[test_case("nousers.dem")]
#[test_case("decal.dem")]
#[test_case("saytext2.dem")]
#[test_case("emptysaytext.dem")]
#[test_case("protocol23.dem")]
fn snapshot_test(input_file: &str) {
    let file = fs::read(format!("test_data/{}", input_file)).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, state) = DemoParser::new(demo.get_stream()).parse().unwrap();

    insta::assert_json_snapshot!(format!("{input_file}_minimal"), state);

    let (_, state) = DemoParser::new_all(demo.get_stream()).parse().unwrap();
    insta::assert_json_snapshot!(format!("{input_file}_minimal"), state);
}

#[test_case("small.dem")]
#[test_case("gully.dem")]
fn game_state_test(input_file: &str) {
    let file = fs::read(format!("test_data/{}", input_file)).expect("Unable to read file");
    let demo = Demo::new(&file);
    let (_, mut state) = DemoParser::new_with_analyser(demo.get_stream(), GameStateAnalyser::new())
        .parse()
        .unwrap();
    state.events.clear();

    insta::with_settings!({sort_maps => true}, {
        insta::assert_json_snapshot!(input_file, state);
    });
}
