use crate::{Parse, ParseError, ParserState, Result, Stream};

pub struct SyncTickPacket {
    tick: u32,
}

impl<'a> Parse<'a> for SyncTickPacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Ok(SyncTickPacket {
            tick: stream.read(32)?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream.skip(32).map_err(ParseError::from)
    }
}
