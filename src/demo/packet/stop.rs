use crate::{Parse, ParserState, Result, Stream};

pub struct StopPacket;

impl<'a> Parse<'a> for StopPacket {
    fn parse(_stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Ok(StopPacket)
    }

    fn skip(_stream: &mut Stream) -> Result<()> {
        Ok(())
    }
}
