use bitbuffer::{BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
use crate::{Parse, ParserState, Stream, Result};
use crate::demo::parser::{Encode, ParseBitSkip};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PreFetchMessage {
    pub index: u16,
}

impl PreFetchMessage {
    fn bit_size(protocol_version: u32) -> usize {
        if protocol_version > 22 {
            14
        } else {
            13
        }
    }
}

impl<'a> Parse<'a> for PreFetchMessage {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let size = Self::bit_size(state.protocol_version);
        Ok(PreFetchMessage {
            index: stream.read_sized(size)?,
        })
    }
}

impl Encode for PreFetchMessage {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        let size = Self::bit_size(state.protocol_version);
        stream.write_int(self.index, size)?;
        Ok(())
    }
}

impl<'a> ParseBitSkip<'a> for PreFetchMessage {
    fn parse_skip(stream: &mut Stream<'a>, state: &ParserState) -> Result<()> {
        let size = Self::bit_size(state.protocol_version);
        stream.skip_bits(size)?;
        Ok(())
    }
}