use bitbuffer::{BitRead, BitWrite, LittleEndian};

use crate::{ReadResult, Stream};

#[derive(Debug, PartialEq)]
pub struct ConsoleCmdPacket {
    tick: u32,
    command: String,
}

impl BitRead<'_, LittleEndian> for ConsoleCmdPacket {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let tick = stream.read_int(32)?;
        let len = stream.read_int::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;
        let command: String = packet_data.read()?;
        Ok(ConsoleCmdPacket { tick, command })
    }
}

impl BitWrite<LittleEndian> for ConsoleCmdPacket {
    fn write(&self, stream: &mut bitbuffer::BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.tick.write(stream)?;
        let len = self.command.len() as u32 + 1;
        len.write(stream)?;
        self.command.write(stream)?;
        Ok(())
    }
}
