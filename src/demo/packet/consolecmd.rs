use crate::{Parse, ParseError, ParserState, Result, Stream};

pub struct ConsoleCmdPacket {
    tick: u32,
    command: String,
}

impl<'a> Parse<'a> for ConsoleCmdPacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read(32)?;
        let len = stream.read(32)?;
        Ok(ConsoleCmdPacket {
            tick,
            command: stream.read_string(Some(len))?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32)?;
        let len = stream.read(32)?;
        stream.skip(len).map_err(ParseError::from)
    }
}