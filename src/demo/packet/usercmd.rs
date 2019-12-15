use bitstream_reader::{BitRead, LittleEndian};

use crate::{ReadResult, Stream};

#[derive(Debug)]
pub struct UserCmdPacket {
    tick: u32,
    sequence_out: u32,
}

impl BitRead<LittleEndian> for UserCmdPacket {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let tick = stream.read()?;
        let sequence_out = stream.read()?;
        let len: u32 = stream.read()?;
        stream.skip_bits(len as usize * 8)?;
        // TODO parse the packet data
        Ok(UserCmdPacket { tick, sequence_out })
    }
}
