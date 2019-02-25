use crate::{Parse, ParseError, ParserState, Result, Stream};
//use crate::test::test_parse;

#[derive(Debug, PartialEq)]
pub struct Header {
    pub demo_type: String,
    pub version: u32,
    pub protocol: u32,
    pub server: String,
    pub nick: String,
    pub map: String,
    pub game: String,
    pub duration: f32,
    pub ticks: u32,
    pub frames: u32,
    pub sigon: u32,
}

impl<'a> Parse<'a> for Header {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Header> {
        Ok(Header {
            demo_type: stream.read_string(Some(8))?,
            version: stream.read(32)?,
            protocol: stream.read(32)?,
            server: stream.read_string(Some(260))?,
            nick: stream.read_string(Some(260))?,
            map: stream.read_string(Some(260))?,
            game: stream.read_string(Some(260))?,
            duration: stream.read_float()?,
            ticks: stream.read(32)?,
            frames: stream.read(32)?,
            sigon: stream.read(32)?,
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream
            .skip(8 * 8 + (32 * 2) + (260 * 8 * 4) + (32 * 4))
            .map_err(ParseError::from)
    }
}

//#[test]
//fn test_header() {
//    test_parse(
//        "data/small.dem",
//        Header {
//            demo_type: "HL2DEMO".to_owned(),
//            version: 3,
//            protocol: 24,
//            server: "localhost:27015".to_owned(),
//            nick: "Icewind | demos.tf".to_owned(),
//            map: "cp_gullywash".to_owned(),
//            game: "tf".to_owned(),
//            duration: 1.7249999,
//            ticks: 115,
//            frames: 111,
//            sigon: 218908,
//        },
//        0,
//    );
//}
