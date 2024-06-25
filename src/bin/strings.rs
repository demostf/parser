use std::collections::HashMap;
use std::env;
use std::fs;

use main_error::MainError;
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParserState};
use tf_demo_parser::demo::packet::stringtable::{StringTableEntry};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::MessageType;

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
    let file = fs::read(path)?;
    let demo = Demo::new(&file);
    let parser = DemoParser::new_with_analyser(demo.get_stream(), StringTableHandler::default());
    let (_, tables) = parser.parse()?;
    if let Some(name) = args.get(2) {
        if name == "--tables" {
            for (name, _) in tables {
                println!("{name}");
            }
        } else {
            let table = tables.get(name).expect("table not found");
            for item in table {
                println!("{item}");
            }
        }
    } else {
        for (name, table) in tables {
            println!("{name}:");
            for item in table {
                println!("\t{item}");
            }
        }
    }
    Ok(())
}

#[derive(Default)]
struct StringTableHandler {
    pub tables: HashMap<String, Vec<String>>,
}

impl MessageHandler for StringTableHandler {
    type Output = HashMap<String, Vec<String>>;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::CreateStringTable | MessageType::UpdateStringTable)
    }

    fn handle_string_entry(&mut self, table: &str, index: usize, entries: &StringTableEntry, _parser_state: &ParserState) {
        let table = self.tables.entry(table.into()).or_default();
        if index < table.len() {
            table[index] = entries.text().into();
        } else {
            table.insert(index, entries.text().into());
        }
    }


    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.tables
    }
}