use std::env;
use std::fs;

use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use main_error::MainError;
use steamid_ng::SteamID;
use tf_demo_parser::demo::data::UserInfo;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::stringtable::{StringTable, StringTableEntry};
use tf_demo_parser::demo::packet::{Packet, PacketType};
use tf_demo_parser::demo::parser::{DemoHandler, Encode, NullHandler, RawPacketStream};
use tf_demo_parser::{Demo, Parse};

const COPY_TYPES: &[PacketType] = &[
    // PacketType::Sigon,
    // PacketType::Message,
    // PacketType::SyncTick, // bit perfect
    // PacketType::ConsoleCmd, // bit perfect
    // PacketType::DataTables, // bit perfect
    // PacketType::StringTables, // bit perfect
    // PacketType::UserCmd,      // bit perfect
];

fn main() -> Result<(), MainError> {
    #[cfg(feature = "better_panic")]
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("2 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let out_path = args[2].clone();
    let file = fs::read(path)?;

    let mut out_buffer = Vec::with_capacity(file.len());
    {
        let mut out_stream = BitWriteStream::new(&mut out_buffer, LittleEndian);

        let demo = Demo::new(&file);
        let mut stream = demo.get_stream();
        let header = Header::read(&mut stream)?;
        header.write(&mut out_stream)?;

        let mut packets = RawPacketStream::new(stream.clone());
        let mut handler = DemoHandler::parse_all_with_analyser(NullHandler);

        let mut packet_start = packets.pos();
        let mut userinfo_table_id = 0;

        while let Some(mut packet) = packets.next(&handler.state_handler)? {
            let packet_end = packets.pos();
            let packet_bits = stream.read_bits(packet_end - packet_start)?;
            assert_eq!(
                Packet::parse(&mut packet_bits.clone(), &handler.state_handler)?,
                packet
            );
            if COPY_TYPES.contains(&packet.packet_type()) {
                packet_bits.write(&mut out_stream)?;
            } else {
                match &mut packet {
                    Packet::Sigon(message_packet) | Packet::Message(message_packet) => {
                        message_packet.meta.view_angles = Default::default();
                        for message in message_packet.messages.iter_mut() {
                            match message {
                                Message::CreateStringTable(table_message) => {
                                    if table_message.table.name == "userinfo" {
                                        for (_i, entry) in table_message.table.entries.iter_mut() {
                                            mut_string_user_info(entry);
                                        }
                                    }
                                }
                                Message::UpdateStringTable(table_message) => {
                                    let table_name = &handler.string_table_names
                                        [table_message.table_id as usize];
                                    if table_name == "userinfo" {
                                        for (_i, entry) in table_message.entries.iter_mut() {
                                            mut_string_user_info(entry);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    Packet::StringTables(table_packet) => {
                        for table in table_packet.tables.iter_mut() {
                            if table.name == "userinfo" {
                                for (_i, entry) in table.entries.iter_mut() {
                                    mut_string_user_info(entry);
                                }
                            }
                        }
                    }
                    _ => {}
                }
                packet.encode(&mut out_stream, &handler.state_handler)?;
            }
            handler.handle_packet(packet)?;
            packet_start = packet_end;
        }
        assert_eq!(false, packets.incomplete);
    }

    fs::write(out_path, out_buffer)?;

    Ok(())
}

fn mut_string_user_info(entry: &mut StringTableEntry) {
    if let Some(mut user_info) = UserInfo::parse_from_string_table(
        entry.text.as_deref(),
        entry.extra_data.as_ref().map(|data| data.data.clone()),
    )
    .unwrap()
    {
        // dbg!(&user_info);
        user_info.player_info.name = "[GC]Kimo".into();
        user_info.player_info.steam_id = "[U:1:32061783]".into();
        user_info.player_info.friends_id =
            SteamID::from_steam3("[U:1:32061783]").unwrap().account_id();
        // *entry = user_info.encode_to_string_table().unwrap();
    }
}
