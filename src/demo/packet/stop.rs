use crate::demo::data::DemoTick;
use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, Endianness};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StopPacket {
    pub tick: DemoTick,
}

impl<'a, E: Endianness> BitRead<'a, E> for StopPacket {
    fn read(stream: &mut BitReadStream<'a, E>) -> bitbuffer::Result<Self> {
        Ok(StopPacket {
            tick: stream.read_int::<u32>(24)?.into(),
        })
    }
}

impl<E: Endianness> BitWrite<E> for StopPacket {
    fn write(&self, stream: &mut BitWriteStream<E>) -> bitbuffer::Result<()> {
        stream.write_int::<u32>(self.tick.into(), 24)
    }
}
