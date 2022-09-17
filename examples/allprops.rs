use fnv::{FnvHashMap, FnvHashSet};
use main_error::MainError;
use std::env;
use std::fs;
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName, ServerClass};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::{SendPropIdentifier, SendPropName};
use tf_demo_parser::MessageType;
pub use tf_demo_parser::{Demo, DemoParser, Parse, ParseError, ParserState, Stream};

fn main() -> Result<(), MainError> {
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
    let parser = DemoParser::new_with_analyser(demo.get_stream(), PropAnalyzer::default());
    let (_, state) = parser.parse()?;
    for prop in state {
        println!("{}", prop);
    }
    Ok(())
}

#[derive(Default)]
struct PropAnalyzer {
    props: FnvHashSet<SendPropIdentifier>,
    prop_names: FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
}

impl MessageHandler for PropAnalyzer {
    type Output = Vec<String>;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::PacketEntities)
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, _parser_state: &ParserState) {
        if let Message::PacketEntities(message) = message {
            for entity in &message.entities {
                for prop in &entity.props {
                    self.props.insert(prop.identifier);
                }
            }
        }
    }

    fn handle_data_tables(
        &mut self,
        parse_tables: &[ParseSendTable],
        _server_classes: &[ServerClass],
        _parser_state: &ParserState,
    ) {
        for table in parse_tables {
            for prop_def in &table.props {
                self.prop_names.insert(
                    prop_def.identifier(),
                    (table.name.clone(), prop_def.name.clone()),
                );
            }
        }
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        let names = self.prop_names;
        let mut props = self
            .props
            .into_iter()
            .map(|prop| {
                let (table, name) = names.get(&prop).unwrap();
                format!("{}.{}", table, name)
            })
            .collect::<Vec<_>>();
        props.sort();
        props
    }
}
