use enum_primitive_derive::Primitive;
use num_traits::FromPrimitive;

pub use generated::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::demo::message::bspdecal::*;
use crate::demo::message::classinfo::*;
use crate::demo::message::gameevent::*;
use crate::demo::message::packetentities::*;
use crate::demo::message::setconvar::*;
use crate::demo::message::stringtable::*;
use crate::demo::message::tempentities::*;
use crate::demo::message::usermessage::*;
use crate::demo::message::voice::*;
use crate::demo::parser::ParseBitSkip;
use crate::{MalformedDemoError, Parse, ParseError, ParserState, Result, Stream};

pub mod bspdecal;
pub mod classinfo;
pub mod gameevent;
pub mod generated;
pub mod packetentities;
pub mod setconvar;
pub mod stringtable;
pub mod tempentities;
pub mod usermessage;
pub mod voice;

#[derive(Primitive, Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    Empty = 0,
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
        prop_type.ok_or(MalformedDemoError::InvalidMessageType(raw).into())
    }
}

#[derive(Debug)]
pub enum Message {
    Empty,
    File(FileMessage),
    NetTick(NetTickMessage),
    StringCmd(StringCmdMessage),
    SetConVar(SetConVarMessage),
    SigOnState(SigOnStateMessage),
    Print(PrintMessage),
    ServerInfo(ServerInfoMessage),
    ClassInfo(ClassInfoMessage),
    SetPause(SetPauseMessage),
    CreateStringTable(CreateStringTableMessage),
    UpdateStringTable(UpdateStringTableMessage),
    VoiceInit(VoiceInitMessage),
    VoiceData(VoiceDataMessage),
    ParseSounds(ParseSoundsMessage),
    SetView(SetViewMessage),
    FixAngle(FixAngleMessage),
    BspDecal(BSPDecalMessage),
    UserMessage(UserMessage),
    EntityMessage(EntityMessage),
    GameEvent(GameEventMessage),
    PacketEntities(PacketEntitiesMessage),
    TempEntities(TempEntitiesMessage),
    PreFetch(PreFetchMessage),
    Menu(MenuMessage),
    GameEventList(GameEventListMessage),
    GetCvarValue(GetCvarValueMessage),
    CmdKeyValues(CmdKeyValuesMessage),
}

impl Parse for Message {
    fn parse(stream: &mut Stream, state: &ParserState) -> Result<Self> {
        let message_type = MessageType::parse(stream, state)?;
        Self::from_type(message_type, stream, state)
    }
}

impl Message {
    pub fn get_message_type(&self) -> MessageType {
        match self {
            Message::Empty => MessageType::Empty,
            Message::File(_) => MessageType::File,
            Message::NetTick(_) => MessageType::NetTick,
            Message::StringCmd(_) => MessageType::StringCmd,
            Message::SetConVar(_) => MessageType::SetConVar,
            Message::SigOnState(_) => MessageType::SigOnState,
            Message::Print(_) => MessageType::Print,
            Message::ServerInfo(_) => MessageType::ServerInfo,
            Message::ClassInfo(_) => MessageType::ClassInfo,
            Message::SetPause(_) => MessageType::SetPause,
            Message::CreateStringTable(_) => MessageType::CreateStringTable,
            Message::UpdateStringTable(_) => MessageType::UpdateStringTable,
            Message::VoiceInit(_) => MessageType::VoiceInit,
            Message::VoiceData(_) => MessageType::VoiceData,
            Message::ParseSounds(_) => MessageType::ParseSounds,
            Message::SetView(_) => MessageType::SetView,
            Message::FixAngle(_) => MessageType::FixAngle,
            Message::BspDecal(_) => MessageType::BspDecal,
            Message::UserMessage(_) => MessageType::UserMessage,
            Message::EntityMessage(_) => MessageType::EntityMessage,
            Message::GameEvent(_) => MessageType::GameEvent,
            Message::PacketEntities(_) => MessageType::PacketEntities,
            Message::TempEntities(_) => MessageType::TempEntities,
            Message::PreFetch(_) => MessageType::PreFetch,
            Message::Menu(_) => MessageType::Menu,
            Message::GameEventList(_) => MessageType::GameEventList,
            Message::GetCvarValue(_) => MessageType::GetCvarValue,
            Message::CmdKeyValues(_) => MessageType::CmdKeyValues,
        }
    }

    pub fn from_type(
        message_type: MessageType,
        stream: &mut Stream,
        state: &ParserState,
    ) -> Result<Self> {
        Ok(match message_type {
            MessageType::Empty => Message::Empty,
            MessageType::File => Message::File(FileMessage::parse(stream, state)?),
            MessageType::NetTick => Message::NetTick(NetTickMessage::parse(stream, state)?),
            MessageType::StringCmd => Message::StringCmd(StringCmdMessage::parse(stream, state)?),
            MessageType::SetConVar => Message::SetConVar(SetConVarMessage::parse(stream, state)?),
            MessageType::SigOnState => {
                Message::SigOnState(SigOnStateMessage::parse(stream, state)?)
            }
            MessageType::Print => Message::Print(PrintMessage::parse(stream, state)?),
            MessageType::ServerInfo => {
                Message::ServerInfo(ServerInfoMessage::parse(stream, state)?)
            }
            MessageType::ClassInfo => Message::ClassInfo(ClassInfoMessage::parse(stream, state)?),
            MessageType::SetPause => Message::SetPause(SetPauseMessage::parse(stream, state)?),
            MessageType::CreateStringTable => {
                Message::CreateStringTable(CreateStringTableMessage::parse(stream, state)?)
            }
            MessageType::UpdateStringTable => {
                Message::UpdateStringTable(UpdateStringTableMessage::parse(stream, state)?)
            }
            MessageType::VoiceInit => Message::VoiceInit(VoiceInitMessage::parse(stream, state)?),
            MessageType::VoiceData => Message::VoiceData(VoiceDataMessage::parse(stream, state)?),
            MessageType::ParseSounds => {
                Message::ParseSounds(ParseSoundsMessage::parse(stream, state)?)
            }
            MessageType::SetView => Message::SetView(SetViewMessage::parse(stream, state)?),
            MessageType::FixAngle => Message::FixAngle(FixAngleMessage::parse(stream, state)?),
            MessageType::BspDecal => Message::BspDecal(BSPDecalMessage::parse(stream, state)?),
            MessageType::UserMessage => Message::UserMessage(UserMessage::parse(stream, state)?),
            MessageType::EntityMessage => {
                Message::EntityMessage(EntityMessage::parse(stream, state)?)
            }
            MessageType::GameEvent => Message::GameEvent(GameEventMessage::parse(stream, state)?),
            MessageType::PacketEntities => {
                Message::PacketEntities(PacketEntitiesMessage::parse(stream, state)?)
            }
            MessageType::TempEntities => {
                Message::TempEntities(TempEntitiesMessage::parse(stream, state)?)
            }
            MessageType::PreFetch => Message::PreFetch(PreFetchMessage::parse(stream, state)?),
            MessageType::Menu => Message::Menu(MenuMessage::parse(stream, state)?),
            MessageType::GameEventList => {
                Message::GameEventList(GameEventListMessage::parse(stream, state)?)
            }
            MessageType::GetCvarValue => {
                Message::GetCvarValue(GetCvarValueMessage::parse(stream, state)?)
            }
            MessageType::CmdKeyValues => {
                Message::CmdKeyValues(CmdKeyValuesMessage::parse(stream, state)?)
            }
        })
    }

    pub fn skip_type(message_type: MessageType, stream: &mut Stream) -> Result<()> {
        match message_type {
            MessageType::Empty => Ok(()),
            MessageType::File => FileMessage::parse_skip(stream),
            MessageType::NetTick => NetTickMessage::parse_skip(stream),
            MessageType::StringCmd => StringCmdMessage::parse_skip(stream),
            MessageType::SetConVar => SetConVarMessage::parse_skip(stream),
            MessageType::SigOnState => SigOnStateMessage::parse_skip(stream),
            MessageType::Print => PrintMessage::parse_skip(stream),
            MessageType::ServerInfo => ServerInfoMessage::parse_skip(stream),
            MessageType::ClassInfo => ClassInfoMessage::parse_skip(stream),
            MessageType::SetPause => SetPauseMessage::parse_skip(stream),
            MessageType::CreateStringTable => CreateStringTableMessage::parse_skip(stream),
            MessageType::UpdateStringTable => UpdateStringTableMessage::parse_skip(stream),
            MessageType::VoiceInit => VoiceInitMessage::parse_skip(stream),
            MessageType::VoiceData => VoiceDataMessage::parse_skip(stream),
            MessageType::ParseSounds => ParseSoundsMessage::parse_skip(stream),
            MessageType::SetView => SetViewMessage::parse_skip(stream),
            MessageType::FixAngle => FixAngleMessage::parse_skip(stream),
            MessageType::BspDecal => BSPDecalMessage::parse_skip(stream),
            MessageType::UserMessage => UserMessage::parse_skip(stream),
            MessageType::EntityMessage => EntityMessage::parse_skip(stream),
            MessageType::GameEvent => GameEventMessage::parse_skip(stream),
            MessageType::PacketEntities => PacketEntitiesMessage::parse_skip(stream),
            MessageType::TempEntities => TempEntitiesMessage::parse_skip(stream),
            MessageType::PreFetch => PreFetchMessage::parse_skip(stream),
            MessageType::Menu => MenuMessage::parse_skip(stream),
            MessageType::GameEventList => GameEventListMessage::parse_skip(stream),
            MessageType::GetCvarValue => GetCvarValueMessage::parse_skip(stream),
            MessageType::CmdKeyValues => CmdKeyValuesMessage::parse_skip(stream),
        }
    }
}
