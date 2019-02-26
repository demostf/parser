use crate::{Parse, ParseError, ParserState, Result, Stream};

#[derive(Debug)]
pub struct ConsoleCmdPacket {
    tick: u32,
    command: String,
}

impl<'a> Parse<'a> for ConsoleCmdPacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read(32)?;
        let len = stream.read::<usize>(32)?;
        let mut packet_data = stream.read_bits(len * 8)?;
        let command = packet_data.read_string(None)?;
        Ok(ConsoleCmdPacket {
            tick,
            command,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(32)?;
        let len = stream.read::<usize>(32)?;
        stream.skip(len * 8).map_err(ParseError::from)
    }
}
