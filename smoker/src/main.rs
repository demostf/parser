use main_error::MainError;
use rayon::prelude::*;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

fn main() -> Result<(), MainError> {
    better_panic::install();
    rayon::ThreadPoolBuilder::new()
        .num_threads(40)
        .build_global()
        .unwrap();

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
    let total = files.len();
    println!("found {} demo files", files.len());

    let count = Arc::new(AtomicUsize::new(0));
    let failures: Vec<_> = files
        .par_iter()
        .filter_map(|entry| {
            let file = fs::read(&entry).unwrap();
            let demo = Demo::new(file);
            let parser = if all {
                DemoParser::new_all(demo.get_stream())
            } else {
                DemoParser::new(demo.get_stream())
            };
            let done = count.fetch_add(1, Ordering::AcqRel);
            println!("{}/{}", done + 1, total);
            if let Err(e) = parser.parse() {
                eprintln!("{}: {}", entry.to_str().unwrap(), e);
                Some(entry)
            } else {
                None
            }
        })
        .collect();
    println!("Found {} failures", failures.len());
    for failed in failures {
        println!("{}", failed.to_str().unwrap());
    }
    Ok(())
}

fn gather_dir(path: impl AsRef<Path>) -> Result<Vec<PathBuf>, MainError> {
    let mut files = Vec::with_capacity(512);

    let meta = fs::metadata(path.as_ref())?;
    if meta.is_file() {
        let file = fs::File::open(path)?;
        for line in BufReader::new(file).lines() {
            files.push(line?.into())
        }
    } else {
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
    }

    Ok(files)
}
