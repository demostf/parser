use crate::gameevent::generate_game_events;
use crate::propnames::generate_prop_names;
use main_error::MainError;
use prettyplease::unparse;
use std::env;
use std::fs;
use syn::{parse2, File};
use tf_demo_parser::Demo;

mod gameevent;
mod propnames;

fn main() -> std::result::Result<(), MainError> {
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let file = fs::read(path).expect("Unable to read file");
    let demo = Demo::new(&file);
    let tokens = match args.get(2).map(|s| s.as_str()) {
        None | Some("events") => generate_game_events(demo),
        Some("props") => generate_prop_names(demo),
        _ => panic!("unsupported"),
    };
    let file = parse2::<File>(tokens)?;
    let code = unparse(&file);
    println!("{}", code);
    Ok(())
}
