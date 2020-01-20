use main_error::MainError;
use rayon::prelude::*;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), MainError> {
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("1 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let all = args
        .get(2)
        .map(|arg| arg.as_str() == "all")
        .unwrap_or_default();

    let files = gather_dir(path)?;
    println!("found {} demo files", files.len());

    let failures = files.par_iter().filter_map(|entry| {
        let file = fs::read(&entry).unwrap();
        let demo = Demo::new(file);
        let parser = if all {
            DemoParser::new_all(demo.get_stream())
        } else {
            DemoParser::new(demo.get_stream())
        };
        if let Err(e) = parser.parse() {
            eprintln!("{}: {}", entry.to_str().unwrap(), e);
            Some(entry)
        } else {
            None
        }
    });
    println!("Found {} failures", failures.count());
    Ok(())
}

fn gather_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, MainError> {
    let mut files = Vec::with_capacity(512);

    for res in fs::read_dir(path)? {
        let entry = res?;

        if entry.file_type()?.is_dir() {
            files.extend_from_slice(&gather_dir(entry.path())?);
        } else {
            let entry_path = entry.path();
            if entry_path.extension() == Some(OsStr::new("dem")) {
                files.push(entry.path());
            }
        }
    }

    Ok(files)
}
