use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

use crate::{Parse, ParseError, ParserState, Result, Stream};

use self::consolecmd::ConsoleCmdPacket;
use self::datatable::DataTablePacket;
use self::message::MessagePacket;
use self::stop::StopPacket;
use self::stringtable::StringTablePacket;
use self::synctick::SyncTickPacket;
use self::usercmd::UserCmdPacket;

pub mod consolecmd;
pub mod stop;
pub mod stringtable;
pub mod synctick;
pub mod usercmd;
pub mod datatable;
pub mod message;

pub enum Packet<'a> {
    Sigon(MessagePacket),
    Message(MessagePacket),
    SyncTick(SyncTickPacket),
    ConsoleCmd(ConsoleCmdPacket),
    UserCmd(UserCmdPacket),
    DataTables(DataTablePacket),
    Stop(StopPacket),
    StringTables(StringTablePacket<'a>),
}

#[derive(Primitive)]
pub enum PacketType {
    Sigon = 1,
    Message = 2,
    SyncTick = 3,
    ConsoleCmd = 4,
    UserCmd = 5,
    DataTables = 6,
    Stop = 7,
    StringTables = 8,
}

impl Parse<'_> for PacketType {
    fn parse(stream: &mut Stream, _state: &ParserState) -> Result<Self> {
        let raw = stream.read(8)?;
        let prop_type: Option<PacketType> = PacketType::from_u8(raw);
        prop_type.ok_or(ParseError::InvalidPacketType(raw))
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        stream.skip(8).map_err(ParseError::from)
    }
}

impl<'a> Parse<'a> for Packet<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState<'a>) -> Result<Self> {
        let packet_type = PacketType::parse(stream, state)?;
        Ok(match packet_type {
            PacketType::Sigon => Packet::Sigon(MessagePacket::parse(stream, state)?),
            PacketType::Message => Packet::Message(MessagePacket::parse(stream, state)?),
            PacketType::SyncTick => Packet::SyncTick(SyncTickPacket::parse(stream, state)?),
            PacketType::ConsoleCmd => Packet::ConsoleCmd(ConsoleCmdPacket::parse(stream, state)?),
            PacketType::UserCmd => Packet::UserCmd(UserCmdPacket::parse(stream, state)?),
            PacketType::DataTables => Packet::DataTables(DataTablePacket::parse(stream, state)?),
            PacketType::Stop => Packet::Stop(StopPacket::parse(stream, state)?),
            PacketType::StringTables => Packet::StringTables(StringTablePacket::parse(stream, state)?),
        })
    }

    fn skip(stream: &mut Stream) -> Result<()> {
        let packet_type = PacketType::parse(stream, &ParserState::new(&stream))?;
        match packet_type {
            PacketType::Sigon => MessagePacket::skip(stream),
            PacketType::Message => MessagePacket::skip(stream),
            PacketType::SyncTick => SyncTickPacket::skip(stream),
            PacketType::ConsoleCmd => ConsoleCmdPacket::skip(stream),
            PacketType::UserCmd => UserCmdPacket::skip(stream),
            PacketType::DataTables => DataTablePacket::skip(stream),
            PacketType::Stop => StopPacket::skip(stream),
            PacketType::StringTables => StringTablePacket::skip(stream),
        }
    }
}
