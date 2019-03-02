use bitstream_reader::{BitRead, BitReadSized, LittleEndian};

use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};

use super::packetentities::PacketEntity;
use super::stringtable::read_var_int;

#[derive(Debug)]
pub struct TempEntitiesMessage {
    pub entities: Vec<PacketEntity>
}

impl Parse for TempEntitiesMessage {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let count: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        let data = stream.read_bits(length as usize)?;

        Ok(TempEntitiesMessage {
            entities: Vec::new()
        })
    }
}