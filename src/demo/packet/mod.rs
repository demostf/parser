use crate::{Parse, ParseError, ParserState, Result, Stream};
use num_traits::FromPrimitive;
use self::stop::Stop;

mod stop;
mod synctick;
mod consolecmd;
mod usercmd;
mod stringtable;

pub enum Packet {
    Stop(Stop)
}

#[derive(Primitive)]
pub enum PacketType {
    Sigon = 1,
    Packet = 2,
    SyncTick = 3,
    ConsoleCmd = 4,
    UserCmd = 5,
    DataTables = 6,
    Stop = 7,
    StringTables = 8,
}

impl Packet {
    fn read_type(stream: &mut Stream) -> Result<PacketType> {
        let raw = stream.read(8)?;
        let packet_type: Option<PacketType> = PacketType::from_u8(raw);
        packet_type.ok_or(ParseError::InvalidPacketType(raw))
    }
}

//impl Parse for Packet {
//    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
//        let packet_type = Packet::read_type(stream);
//        match packet_type {
//            Sigon => {}
//        }
//    }
//
//    fn skip(stream: &mut Stream) -> Result<()> {
//        Ok(())
//    }
//}