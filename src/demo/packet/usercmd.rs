use bitstream_reader::{BitRead, LittleEndian};

use crate::{Parse, ParseError, ParserState, Result, Stream, ReadResult};

#[derive(Debug)]
pub struct UserCmdPacket {
    tick: u32,
    sequence_out: u32,
}

impl BitRead<LittleEndian> for UserCmdPacket {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let tick = stream.read_int(32)?;
        let sequence_out = stream.read_int(32)?;
        let len = stream.read_int::<usize>(32)?;
        let mut _packet_data = stream.read_bits(len * 8)?;
        // TODO parse the packet data
        Ok(UserCmdPacket { tick, sequence_out })
    }
}
