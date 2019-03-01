use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

pub use generated::*;

use crate::{Parse, ParseError, ParserState, Result, Stream};

mod classinfo;
mod generated;
mod stringtable;

#[derive(Primitive, Debug)]
pub enum MessageType {
    File = 2,
    NetTick = 3,
    StringCmd = 4,
    SetConVar = 5,
    SigOnState = 6,
    Print = 7,
    ServerInfo = 8,
    ClassInfo = 10,
    SetPause = 11,
    CreateStringTable = 12,
    UpdateStringTable = 13,
    VoiceInit = 14,
    VoiceData = 15,
    ParseSounds = 17,
    SetView = 18,
    FixAngle = 19,
    BspDecal = 21,
    UserMessage = 23,
    EntityMessage = 24,
    GameEvent = 25,
    PacketEntities = 26,
    TempEntities = 27,
    PreFetch = 28,
    Menu = 29,
    GameEventList = 30,
    GetCvarValue = 31,
    CmdKeyValues = 32,
}

impl Parse for MessageType {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let raw = stream.read_int(6)?;
        let prop_type: Option<MessageType> = MessageType::from_u8(raw);
        prop_type.ok_or(ParseError::InvalidMessageType(raw))
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream.skip(6).map_err(ParseError::from)
    }
}
