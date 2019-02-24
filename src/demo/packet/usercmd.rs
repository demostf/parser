use crate::{Parse, ParseError, ParserState, Result, Stream};

pub struct UserCmdPacket {
    tick: u32,
    command: String,
    sequence_out: u32
}

impl<'a> Parse<'a> for UserCmdPacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read(32)?;
        let sequence_out = stream.read(32)?;
        let len = stream.read(32)?;
        Ok(UserCmdPacket {
            tick,
            sequence_out,
            command: stream.read_string(Some(len))?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(64)?;
        let len = stream.read(32)?;
        stream.skip(len).map_err(ParseError::from)
    }
}