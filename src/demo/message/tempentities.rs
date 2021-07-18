use crate::{ReadResult, Stream};

use super::packetentities::PacketEntity;
use super::stringtable::read_var_int;
use crate::demo::message::stringtable::write_var_int;
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};

#[derive(Debug)]
pub struct TempEntitiesMessage<'a> {
    pub count: u8,
    pub data: Stream<'a>,
    pub entities: Vec<PacketEntity>,
}

impl<'a> BitRead<'a, LittleEndian> for TempEntitiesMessage<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        let count: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        let data = stream.read_bits(length as usize)?;

        Ok(TempEntitiesMessage {
            count,
            data,
            entities: Vec::new(),
        })
    }

    fn skip(stream: &mut Stream) -> ReadResult<()> {
        let _: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        stream.skip_bits(length as usize)
    }
}

impl BitWrite<LittleEndian> for TempEntitiesMessage<'_> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.count.write(stream)?;
        write_var_int(self.data.bit_len() as u32, stream)?;
        self.data.write(stream)
    }
}
