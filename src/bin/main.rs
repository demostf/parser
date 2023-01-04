use std::env;
use std::fs;

use main_error::MainError;
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};
use tf_demo_parser::demo::parser::player_summary_analyzer::PlayerSummaryAnalyzer;


#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), MainError> {
    #[cfg(feature = "better_panic")]
    better_panic::install();

    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let all = args.contains(&std::string::String::from("all"));
    let detailed_summaries = args.contains(&std::string::String::from("detailed_summaries"));
    let file = fs::read(path)?;
    let demo = Demo::new(&file);

    if !detailed_summaries {
        // Use the default (simple) analyzer to track kills, assists, and deaths
        let parser = if all {
            DemoParser::new_all(demo.get_stream())
        } else {
            DemoParser::new(demo.get_stream())
        };
        let (_, state) = parser.parse()?;
        println!("{}", serde_json::to_string(&state)?);
    } else {
        let parser = DemoParser::new_with_analyser(demo.get_stream(), PlayerSummaryAnalyzer::new());
        let (header, state) = parser.parse()?;

        println!("{:?}", header);

        let table_header = "Player                           | Points     | Kills      | Deaths     | Assists    | Destruction | Captures   | Defenses   | Domination | Revenge    | Ubers      | Headshots  | Teleports  | Healing    | Backstabs  | Bonus      | Support    | Damage Dealt";
        let divider      = "---------------------------------|------------|------------|------------|------------|-------------|------------|------------|------------|------------|------------|------------|------------|------------|------------|------------|------------|-------------";
        println!("{}", table_header);
        println!("{}", divider);

        for (user_id, user_data) in state.users {
            let player_name = user_data.name;
            let summary = state.player_summaries.get(&user_id);
            match summary {
                Some(s) => {
                    let (color_code_start, color_code_end) = if player_name == header.nick {
                        // Give the line for the player a green background with white text
                        // ANSI color codes are in hex, since rust doesn't support octal literals in strings
                        // See: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
                        ("\x1b[1;42;37m", "\x1b[0m")
                    } else {
                        ("", "")
                    };

                    println!(
                        "{}{:32} | {:10} | {:10} | {:10} | {:10} | {:11} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:10} | {:12}{}",
                        color_code_start,

                        player_name,
                        s.points,
                        s.kills,
                        s.deaths,
                        s.assists,
                        s.buildings_destroyed,
                        s.captures,
                        s.defenses,
                        s.dominations,
                        s.revenges,
                        s.ubercharges,
                        s.headshots,
                        s.teleports,
                        s.healing,
                        s.backstabs,
                        s.bonus_points,
                        s.support,
                        s.damage_dealt,

                        color_code_end,
                    );
                },
                None => {
                    // No summary for this player - they likely joined at the end of the match, or left before they did anything noteworthy
                }
            }
        }
    }

    Ok(())
}
