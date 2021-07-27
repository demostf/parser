use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};

use crate::{Parse, ParserState, Result, Stream};

use self::consolecmd::ConsoleCmdPacket;
use self::datatable::DataTablePacket;
use self::message::MessagePacket;
use self::stop::StopPacket;
use self::stringtable::StringTablePacket;
use self::synctick::SyncTickPacket;
use self::usercmd::UserCmdPacket;
use crate::demo::parser::Encode;
use serde::{Deserialize, Serialize};

pub mod consolecmd;
pub mod datatable;
pub mod message;
pub mod stop;
pub mod stringtable;
pub mod synctick;
pub mod usercmd;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
#[serde(tag = "type")]
pub enum Packet<'a> {
    Sigon(MessagePacket<'a>),
    Message(MessagePacket<'a>),
    SyncTick(SyncTickPacket),
    ConsoleCmd(ConsoleCmdPacket),
    UserCmd(UserCmdPacket),
    DataTables(DataTablePacket),
    Stop(StopPacket),
    StringTables(StringTablePacket<'a>),
}

impl Packet<'_> {
    pub fn tick(&self) -> u32 {
        match self {
            Packet::Sigon(msg) => msg.tick,
            Packet::Message(msg) => msg.tick,
            Packet::SyncTick(msg) => msg.tick,
            Packet::ConsoleCmd(msg) => msg.tick,
            Packet::UserCmd(msg) => msg.tick,
            Packet::DataTables(msg) => msg.tick,
            Packet::Stop(msg) => msg.tick,
            Packet::StringTables(msg) => msg.tick,
        }
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, Copy, Eq, PartialEq)]
#[discriminant_bits = 8]
#[repr(u8)]
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

impl Packet<'_> {
    pub fn packet_type(&self) -> PacketType {
        match self {
            Packet::Sigon(_) => PacketType::Sigon,
            Packet::Message(_) => PacketType::Message,
            Packet::SyncTick(_) => PacketType::SyncTick,
            Packet::ConsoleCmd(_) => PacketType::ConsoleCmd,
            Packet::UserCmd(_) => PacketType::UserCmd,
            Packet::DataTables(_) => PacketType::DataTables,
            Packet::Stop(_) => PacketType::Stop,
            Packet::StringTables(_) => PacketType::StringTables,
        }
    }
}

impl<'a> Parse<'a> for Packet<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let packet_type = PacketType::read(stream)?;
        Ok(match packet_type {
            PacketType::Sigon => Packet::Sigon(MessagePacket::parse(stream, state)?),
            PacketType::Message => Packet::Message(MessagePacket::parse(stream, state)?),
            PacketType::SyncTick => Packet::SyncTick(SyncTickPacket::parse(stream, state)?),
            PacketType::ConsoleCmd => Packet::ConsoleCmd(ConsoleCmdPacket::parse(stream, state)?),
            PacketType::UserCmd => Packet::UserCmd(UserCmdPacket::parse(stream, state)?),
            PacketType::DataTables => Packet::DataTables(DataTablePacket::parse(stream, state)?),
            PacketType::Stop => Packet::Stop(StopPacket::parse(stream, state)?),
            PacketType::StringTables => {
                Packet::StringTables(StringTablePacket::parse(stream, state)?)
            }
        })
    }
}

impl Encode for Packet<'_> {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        self.packet_type().write(stream)?;
        match self {
            Packet::Sigon(inner) => inner.encode(stream, state),
            Packet::Message(inner) => inner.encode(stream, state),
            Packet::SyncTick(inner) => inner.encode(stream, state),
            Packet::ConsoleCmd(inner) => inner.encode(stream, state),
            Packet::UserCmd(inner) => inner.encode(stream, state),
            Packet::DataTables(inner) => inner.encode(stream, state),
            Packet::Stop(inner) => inner.encode(stream, state),
            Packet::StringTables(inner) => inner.encode(stream, state),
        }
    }
}
