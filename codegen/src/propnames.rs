use fnv::{FnvHashMap, FnvHashSet};
use proc_macro2::TokenStream;
use quote::quote;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::{ParseSendTable, SendTableName, ServerClass};
use tf_demo_parser::demo::parser::MessageHandler;
use tf_demo_parser::demo::sendprop::{SendPropIdentifier, SendPropName};
use tf_demo_parser::{Demo, DemoParser, MessageType, ParserState};

struct PropInfo {
    identifier: SendPropIdentifier,
    table_name: String,
    prop_name: String,
}

#[derive(Default)]
struct PropAnalyzer {
    props: FnvHashSet<SendPropIdentifier>,
    prop_names: FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
}

impl MessageHandler for PropAnalyzer {
    type Output = Vec<PropInfo>;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::PacketEntities)
    }

    fn handle_message(&mut self, message: &Message, _tick: u32) {
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
        let mut props: Vec<_> = self
            .prop_names
            .into_iter()
            .map(|(identifier, (table_name, prop_name))| PropInfo {
                identifier,
                table_name: table_name.to_string(),
                prop_name: prop_name.to_string(),
            })
            .collect();
        props.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        props
    }
}

pub fn generate_prop_names(demo: Demo) -> TokenStream {
    let (_, props) = DemoParser::new_with_analyser(demo.get_stream(), PropAnalyzer::default())
        .parse()
        .unwrap();

    let imports = quote!(
        use crate::demo::sendprop::SendPropIdentifier;
    );

    let matches = props.into_iter().map(|prop| {
        let identifier: u64 = prop.identifier.into();
        let table_name = prop.table_name;
        let prop_name = prop.prop_name;
        quote! {
            #identifier => Some((#table_name, #prop_name))
        }
    });

    quote!(
        #imports

        pub fn get_prop_names(identifier: SendPropIdentifier) -> Option<(&'static str, &'static str)> {
            let identifier: u64 = identifier.into();
            match identifier {
                #(#matches,)*
                _ => None,
            }
        }
    )
}
