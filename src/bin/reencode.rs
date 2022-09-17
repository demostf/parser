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

const COPY_TYPES: &[PacketType] = &[
    // PacketType::Signon,
    // PacketType::Message,
    // PacketType::SyncTick,   // bit perfect
    // PacketType::ConsoleCmd, // bit perfect
    // PacketType::DataTables, // bit perfect
    // PacketType::StringTables, // close enough
    // PacketType::UserCmd, // close enough
];

fn main() -> Result<(), MainError> {
    #[cfg(feature = "trace")]
    tracing_subscriber::fmt::init();

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

        let mut packet_start = packets.pos();
        let mut has_stop = false;
        let mut last_tick = DemoTick::default();

        while let Some(mut packet) = packets.next(&handler.state_handler)? {
            let packet_end = packets.pos();
            last_tick = packet.tick();
            let packet_bits = stream.read_bits(packet_end - packet_start)?;
            if COPY_TYPES.contains(&packet.packet_type()) {
                packet_bits.write(&mut out_stream)?;
            } else {
                match &mut packet {
                    Packet::Signon(message_packet) | Packet::Message(message_packet)
                        if strip_pov =>
                    {
                        message_packet.meta.view_angles = Default::default();
                        message_packet.messages.iter_mut().for_each(|msg| {
                            if let Message::ServerInfo(info) = msg {
                                info.stv = true;
                            }
                        });
                    }
                    Packet::Stop(_) => {
                        has_stop = true;
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

        if packets.incomplete {
            eprintln!("Warning: truncated demo");
        }

        if !has_stop {
            Packet::Stop(StopPacket { tick: last_tick })
                .encode(&mut out_stream, &handler.state_handler)
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
