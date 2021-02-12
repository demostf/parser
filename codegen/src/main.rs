use crate::gameevent::generate_game_events;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use tf_demo_parser::Demo;

mod gameevent;

fn which_rustfmt() -> Option<PathBuf> {
    match env::var_os("RUSTFMT") {
        Some(which) => {
            if which.is_empty() {
                None
            } else {
                Some(PathBuf::from(which))
            }
        }
        None => toolchain_find::find_installed_component("rustfmt"),
    }
}

fn format(code: &str) -> io::Result<String> {
    let mut builder = tempfile::Builder::new();
    builder.prefix("rustfmtr");
    let outdir = builder.tempdir()?;
    let outfile_path = outdir.path().join("code");

    fs::write(&outfile_path, code)?;

    let rustfmt = which_rustfmt().ok_or(io::Error::from(io::ErrorKind::NotFound))?;

    let _status = Command::new(rustfmt)
        .arg("--edition=2018")
        .arg(&outfile_path)
        .status();

    fs::read_to_string(&outfile_path)
}

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
    let code = tokens.to_string();
    let formatted = format(&code)?;
    println!("{}", formatted);
    Ok(())
}
