use std::env;
use std::fs;

use main_error::MainError;
use serde::{Deserialize, Serialize};
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::parser::analyser::MatchState;
use tf_demo_parser::demo::parser::gamestateanalyser::GameStateAnalyser;
pub use tf_demo_parser::{Demo, DemoParser, Parse};

#[cfg(feature = "jemallocator")]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct JsonDemo {
    header: Header,
    #[serde(flatten)]
    state: MatchState,
}

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
    let file = fs::read(path)?;
    let demo = Demo::new(&file);

    let parser = DemoParser::new_all_with_analyser(demo.get_stream(), GameStateAnalyser::default());
    let (_header, state) = parser.parse()?;

    for collision in &state.collisions {
        if let Some(player) = state
            .get_player(collision.target)
            .and_then(|player| player.info.as_ref())
        {
            let weapon_class = state
                .server_classes
                .get(usize::from(collision.projectile.class))
                .map(|class| class.name.as_str())
                .unwrap_or("unknown weapon");
            println!(
                "{}: {} hit by {}",
                collision.tick, player.name, weapon_class
            );
        }
    }

    Ok(())
}
