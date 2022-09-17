use crate::demo::data::DemoTick;
use crate::{ReadResult, Stream};
use bitbuffer::{BitRead, BitWrite, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ConsoleCmdPacket {
    pub tick: DemoTick,
    pub command: String,
}

impl BitRead<'_, LittleEndian> for ConsoleCmdPacket {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let tick = stream.read()?;
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
