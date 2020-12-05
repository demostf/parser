use crate::{Parse, ParseError, ParserState, Result, Stream};

use super::packetentities::PacketEntity;
use super::stringtable::read_var_int;
use crate::demo::parser::ParseBitSkip;

#[derive(Debug)]
pub struct TempEntitiesMessage {
    pub entities: Vec<PacketEntity>,
}

impl Parse<'_> for TempEntitiesMessage {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let _count: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        let _data = stream.read_bits(length as usize)?;

        Ok(TempEntitiesMessage {
            entities: Vec::new(),
        })
    }
}

impl ParseBitSkip<'_> for TempEntitiesMessage {
    fn parse_skip(stream: &mut Stream) -> Result<()> {
        let _: u8 = stream.read()?;
        let length = read_var_int(stream)?;
        stream.skip_bits(length as usize).map_err(ParseError::from)
    }
}
