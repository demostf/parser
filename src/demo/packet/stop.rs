use crate::{Parse, ParserState, Result, Stream};

pub struct Stop;

impl<'a> Parse<'a> for Stop {
    fn parse(_stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        Ok(Stop)
    }

    fn skip(_stream: &mut Stream) -> Result<()> {
        Ok(())
    }
}