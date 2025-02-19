#![allow(unused_imports)]

use std::env;
use std::fs;

use bitbuffer::{BitError, BitRead, BitWrite, BitWriteStream, LittleEndian};
use main_error::MainError;
use tf_demo_parser::demo::data::DemoTick;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::message::{setconvar::SetConVarMessage, Message, NetTickMessage};
use tf_demo_parser::demo::packet::stop::StopPacket;
use tf_demo_parser::demo::packet::{Packet, PacketType};
use tf_demo_parser::demo::parser::{DemoHandler, Encode, RawPacketStream};
use tf_demo_parser::{Demo, ParseError};

fn main() -> Result<(), MainError> {
    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

    #[cfg(feature = "better-panic")]
    better_panic::install();

    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("2 argument required");
        return Ok(());
    }
    let path = args[1].clone();
    let out_path = args[2].clone();
    let file = fs::read(path)?;
    let strip_pov = true;

    let mut out_buffer = Vec::with_capacity(file.len());
    {
        let mut out_stream = BitWriteStream::new(&mut out_buffer, LittleEndian);

        let demo = Demo::new(&file);
        let mut stream = demo.get_stream();
        let mut header = Header::read(&mut stream)?;
        let mut packets = RawPacketStream::new(stream.clone());

        // demos that are closed unexpectedly have no length set
        if header.ticks == 0 {
            header_fixup(&mut header, packets.clone())?;
        }
        header.write(&mut out_stream)?;

        let mut handler = DemoHandler::default();
        let mut encode_handler = DemoHandler::default();

        let mut has_stop = false;
        let mut last_tick = DemoTick::default();

        while let Some(packet) = packets.next(&handler.state_handler)? {
            last_tick = packet.tick();
            if packet.packet_type() == PacketType::Stop {
                has_stop = true;
            }

            let mut encode_packet = packet.clone();
            match &mut encode_packet {
                Packet::DataTables(tables_packet) => {
                    for table in tables_packet.tables.iter_mut() {
                        for prop in table.props.iter_mut() {
                            match (table.name.as_str(), prop.name.as_str()) {
                                ("DT_ObjectDispenser", "\"healing_array\"") => {
                                    prop.element_count = Some(101);
                                }
                                ("DT_Team", "\"player_array\"") => {
                                    prop.element_count = Some(101);
                                }
                                ("DT_TFTeam", "\"team_object_array\"") => {
                                    prop.element_count = Some(606);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Packet::Signon(message_packet) | Packet::Message(message_packet) if strip_pov => {
                    message_packet.meta.view_angles = Default::default();
                    message_packet.messages.iter_mut().for_each(|msg| {
                        if let Message::ServerInfo(info) = msg {
                            info.stv = true;
                        }
                    });
                }
                _ => {}
            }

            if encode_packet.packet_type() != PacketType::ConsoleCmd {
                encode_packet
                    .encode(&mut out_stream, &encode_handler.state_handler)
                    .unwrap();
            }

            handler.handle_packet(packet).unwrap();
            encode_handler.handle_packet(encode_packet).unwrap();
        }

        if packets.incomplete {
            eprintln!("Warning: truncated demo");
        }

        if !has_stop {
            Packet::Stop(StopPacket { tick: last_tick })
                .encode(&mut out_stream, &encode_handler.state_handler)
                .unwrap();
        }
    }

    fs::write(out_path, out_buffer)?;

    Ok(())
}

fn header_fixup(header: &mut Header, mut packets: RawPacketStream) -> Result<(), MainError> {
    let mut ticks = 0;
    let mut handler = DemoHandler::default();
    let mut tickrate = 66;

    while let Some(packet) = packets.next(&handler.state_handler)? {
        ticks = packet.tick().into();

        if let Packet::Signon(message_packet) = &packet {
            for message in &message_packet.messages {
                if let Message::SetConVar(SetConVarMessage { vars, .. }) = message {
                    for cvar in vars {
                        if cvar.key == "sv_minupdaterate" {
                            tickrate = cvar.value.parse().expect("invalid sv_minupdaterate");
                        }
                    }
                }
            }
        }
        handler.handle_packet(packet).unwrap();
    }

    header.ticks = ticks;
    header.duration = ticks as f32 / tickrate as f32;
    Ok(())
}
