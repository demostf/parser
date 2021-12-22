use std::fs;
use test_case::test_case;

use bitbuffer::{BitRead, BitReadBuffer, BitReadStream, BitWrite, BitWriteStream, LittleEndian};
use std::collections::HashMap;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::datatable::SendTableName;
use tf_demo_parser::demo::packet::Packet;
use tf_demo_parser::demo::parser::{DemoHandler, Encode, NullHandler, RawPacketStream};
use tf_demo_parser::demo::sendprop::{RawSendPropDefinition, SendPropIdentifier, SendPropName};
use tf_demo_parser::{Demo, Parse};

#[test_case("test_data/small.dem"; "small.dem")]
#[test_case("test_data/gully.dem"; "gully.dem")]
#[test_case("test_data/comp.dem"; "comp.dem")]
#[test_case("test_data/malformed_cvar.dem"; "malformed_cvar.dem")]
#[test_case("test_data/unicode-saytext.dem"; "unicode-saytext.dem")]
#[test_case("test_data/nousers.dem"; "nousers.dem")]
#[test_case("test_data/decal.dem"; "decal.dem")]
#[test_case("test_data/saytext2.dem"; "saytext2.dem")]
#[test_case("test_data/emptysaytext.dem"; "emptysaytext.dem")]
fn re_encode_test(input_file: &str) {
    let file = fs::read(input_file).expect("Unable to read file");
    let demo = Demo::new(&file);
    let mut out_buffer = Vec::with_capacity(file.len());

    let mut stream = demo.get_stream();
    let header = Header::read(&mut stream).unwrap();
    let header_size = stream.pos() / 8;
    assert_eq!(1072, header_size);

    let mut packets = RawPacketStream::new(stream);
    let mut handler = DemoHandler::parse_all_with_analyser(NullHandler);

    {
        let mut out_stream = BitWriteStream::new(&mut out_buffer, LittleEndian);
        header.write(&mut out_stream).unwrap();
    }
    assert_eq!(file[0..header_size], out_buffer);

    let mut prop_names: HashMap<
        SendPropIdentifier,
        (SendTableName, SendPropName, RawSendPropDefinition),
    > = HashMap::new();

    while let Some(packet) = packets.next(&handler.state_handler).unwrap() {
        if let Packet::DataTables(data_table) = &packet {
            for table in data_table.tables.iter() {
                for prop_def in &table.props {
                    prop_names.insert(
                        prop_def.identifier(),
                        (table.name.clone(), prop_def.name.clone(), prop_def.clone()),
                    );
                }
            }
        }

        out_buffer.clear();
        {
            let mut out_stream = BitWriteStream::new(&mut out_buffer, LittleEndian);
            packet
                .encode(&mut out_stream, &handler.state_handler)
                .unwrap();
        }
        let mut re_read = BitReadStream::new(BitReadBuffer::new(&out_buffer, LittleEndian));
        let re_decoded = Packet::parse(&mut re_read, &handler.state_handler)
            .expect(&format!("while parsing {:?}", packet.packet_type()));
        assert_eq!(packet.packet_type(), re_decoded.packet_type());
        match (&packet, &re_decoded) {
            (
                Packet::Message(msg) | Packet::Signon(msg),
                Packet::Message(re_msg) | Packet::Signon(re_msg),
            ) => {
                assert_eq!(msg.tick, re_msg.tick);
                assert_eq!(msg.meta, re_msg.meta);
                assert_eq!(
                    msg.messages
                        .iter()
                        .map(|msg| msg.get_message_type())
                        .collect::<Vec<_>>(),
                    re_msg
                        .messages
                        .iter()
                        .map(|msg| msg.get_message_type())
                        .collect::<Vec<_>>()
                );
                assert_eq!(msg.messages.len(), re_msg.messages.len());
                for (msg, re_msg) in msg.messages.iter().zip(re_msg.messages.iter()) {
                    assert_eq!(msg.get_message_type(), re_msg.get_message_type());
                    match (msg, re_msg) {
                        (Message::PacketEntities(msg), Message::PacketEntities(re_msg)) => {
                            assert_eq!(msg.updated_base_line, re_msg.updated_base_line);
                            assert_eq!(msg.base_line, re_msg.base_line);
                            assert_eq!(msg.delta, re_msg.delta);
                            assert_eq!(msg.max_entries, re_msg.max_entries);
                            assert_eq!(msg.removed_entities, re_msg.removed_entities);
                            assert_eq!(msg.entities.len(), re_msg.entities.len());
                            for (ent, re_ent) in msg.entities.iter().zip(re_msg.entities.iter()) {
                                let props = ent
                                    .props
                                    .iter()
                                    .map(|prop| {
                                        (
                                            prop_names.get(&prop.identifier).unwrap(),
                                            prop.value.clone(),
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                let re_props = re_ent
                                    .props
                                    .iter()
                                    .map(|prop| {
                                        (
                                            prop_names.get(&prop.identifier).unwrap(),
                                            prop.value.clone(),
                                        )
                                    })
                                    .collect::<Vec<_>>();
                                pretty_assertions::assert_eq!(props, re_props);
                            }
                        }
                        (msg, re_msg) => assert_eq!(msg, re_msg),
                    }
                }
            }
            (Packet::StringTables(packet), Packet::StringTables(re_packet)) => {
                assert_eq!(packet.tick, re_packet.tick);
                assert_eq!(packet.tables.len(), re_packet.tables.len());
                for (table, re_table) in packet.tables.iter().zip(re_packet.tables.iter()) {
                    assert_eq!(table, re_table);
                }
            }
            (packet, re_decoded) => {
                assert_eq!(packet, re_decoded);
            }
        }

        handler.handle_packet(packet).unwrap();
    }
}
