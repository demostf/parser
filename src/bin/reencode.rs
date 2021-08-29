#![allow(unused_imports)]

use std::env;
use std::fs;

use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use main_error::MainError;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::message::Message;
use tf_demo_parser::demo::packet::{Packet, PacketType};
use tf_demo_parser::demo::parser::{DemoHandler, Encode, RawPacketStream};
use tf_demo_parser::Demo;

const COPY_TYPES: &[PacketType] = &[
    // PacketType::Sigon,
    // PacketType::Message,
    // PacketType::SyncTick,   // bit perfect
    // PacketType::ConsoleCmd, // bit perfect
    // PacketType::DataTables, // bit perfect
    // PacketType::StringTables, // close enough
    // PacketType::UserCmd, // close enough
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
        let mut handler = DemoHandler::default();

        let mut packet_start = packets.pos();

        while let Some(mut packet) = packets.next(&handler.state_handler)? {
            let packet_end = packets.pos();
            let packet_bits = stream.read_bits(packet_end - packet_start)?;
            if COPY_TYPES.contains(&packet.packet_type()) {
                packet_bits.write(&mut out_stream)?;
            } else {
                match &mut packet {
                    Packet::Sigon(message_packet) | Packet::Message(message_packet) => {
                        message_packet.meta.view_angles = Default::default();
                        message_packet.messages.iter_mut().for_each(|msg| {
                            if let Message::ServerInfo(info) = msg {
                                info.stv = true;
                            }
                        });
                    }
                    _ => {}
                }

                if packet.packet_type() != PacketType::ConsoleCmd {
                    packet
                        .encode(&mut out_stream, &handler.state_handler)
                        .unwrap();
                }
            }
            handler.handle_packet(packet).unwrap();
            packet_start = packet_end;
        }
        assert_eq!(false, packets.incomplete);
    }

    fs::write(out_path, out_buffer)?;

    Ok(())
}
