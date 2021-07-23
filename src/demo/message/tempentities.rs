use crate::{Parse, ParserState, ReadResult, Stream};

use super::packetentities::PacketEntity;
use super::stringtable::read_var_int;
use crate::demo::message::stringtable::write_var_int;
use crate::demo::parser::ParseBitSkip;
use crate::Result;
use bitbuffer::{BitWrite, BitWriteStream, LittleEndian};

#[derive(Debug, PartialEq)]
pub struct TempEntitiesMessage<'a> {
    pub count: u8,
    pub data: Stream<'a>,
    pub entities: Vec<PacketEntity>,
}

impl<'a> Parse<'a> for TempEntitiesMessage<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let count: u8 = stream.read()?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(17)?
        };
        let data = stream.read_bits(length as usize)?;

        Ok(TempEntitiesMessage {
            count,
            data,
            entities: Vec::new(),
        })
    }
}

impl<'a> ParseBitSkip<'a> for TempEntitiesMessage<'a> {
    fn parse_skip(stream: &mut Stream<'a>, state: &ParserState) -> Result<()> {
        let _: u8 = stream.read()?;
        let length = if state.protocol_version > 23 {
            read_var_int(stream)?
        } else {
            stream.read_sized(17)?
        };
        stream.skip_bits(length as usize)?;
        Ok(())
    }
}

impl BitWrite<LittleEndian> for TempEntitiesMessage<'_> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        if !self.entities.is_empty() {
            todo!();
        }
        self.count.write(stream)?;
        write_var_int(self.data.bit_len() as u32, stream)?;
        self.data.write(stream)
    }
}
