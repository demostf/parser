use crate::gameevent::generate_game_events;
use prettyplease::unparse;
use std::env;
use std::fs;
use syn::{parse2, File};
use tf_demo_parser::Demo;

mod gameevent;

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let file = fs::read(path).expect("Unable to read file");
    let demo = Demo::new(&file);
    let tokens = generate_game_events(demo);
    let file = parse2::<File>(tokens)?;
    let code = unparse(&file);
    println!("{}", code);
    Ok(())
}
