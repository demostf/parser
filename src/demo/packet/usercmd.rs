use crate::{Parse, ParseError, ParserState, Result, Stream};

#[derive(Debug)]
pub struct UserCmdPacket {
    tick: u32,
    sequence_out: u32,
}

impl<'a> Parse<'a> for UserCmdPacket {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let tick = stream.read(32)?;
        let sequence_out = stream.read(32)?;
        let len = stream.read::<usize>(32)?;
        let mut _packet_data = stream.read_bits(len * 8)?;
        // TODO parse the packet data
        Ok(UserCmdPacket {
            tick,
            sequence_out
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let _ = stream.skip(64)?;
        let len = stream.read::<usize>(32)?;
        stream.skip(len * 8).map_err(ParseError::from)
    }
}
