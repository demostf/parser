use crate::{Parse, ParserState, Result, Stream};

use super::packetentities::PacketEntity;
use super::stringtable::read_var_int;

#[derive(Debug)]
pub struct TempEntitiesMessage {
    pub entities: Vec<PacketEntity>,
}

impl Parse for TempEntitiesMessage {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let _count: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        let _data = stream.read_bits(length as usize)?;

        Ok(TempEntitiesMessage {
            entities: Vec::new(),
        })
    }
}
