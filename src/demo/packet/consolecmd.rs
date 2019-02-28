use bitstream_reader::{BitRead, LittleEndian};

use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};

#[derive(Debug)]
pub struct ConsoleCmdPacket {
    tick: u32,
    command: String,
}

impl BitRead<LittleEndian> for ConsoleCmdPacket {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let tick = stream.read_int(32)?;
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;
        let command = packet_data.read_string(None)?;
        Ok(ConsoleCmdPacket { tick, command })
    }
}
