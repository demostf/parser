use std::env;
use std::fs;

use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use main_error::MainError;
use tf_demo_parser::demo::header::Header;
use tf_demo_parser::demo::parser::{DemoHandler, Encode, NullHandler, RawPacketStream};
use tf_demo_parser::Demo;

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

        let mut packets = RawPacketStream::new(stream);
        let mut handler = DemoHandler::parse_all_with_analyser(NullHandler);

        while let Some(packet) = packets.next(&handler.state_handler)? {
            packet.encode(&mut out_stream, &handler.state_handler)?;
            handler.handle_packet(packet)?;
        }
    }

    fs::write(out_path, out_buffer)?;

    Ok(())
}
