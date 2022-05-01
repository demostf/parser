pub use generated::*;

use crate::demo::message::bspdecal::*;
use crate::demo::message::classinfo::*;
use crate::demo::message::gameevent::*;
use crate::demo::message::packetentities::*;
use crate::demo::message::setconvar::*;
use crate::demo::message::stringtable::*;
use crate::demo::message::tempentities::*;
use crate::demo::message::usermessage::*;
use crate::demo::message::voice::*;
use crate::demo::parser::{Encode, ParseBitSkip};
use crate::{Parse, ParserState, Result, Stream};
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    BitRead, BitWrite, Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
#[discriminant_bits = 6]
pub enum MessageType {
    Empty = 0,
    File = 2,
    NetTick = 3,
    StringCmd = 4,
    SetConVar = 5,
    SignOnState = 6,
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
#[serde(tag = "type")]
pub enum Message<'a> {
    Empty,
    File(FileMessage),
    NetTick(NetTickMessage),
    StringCmd(StringCmdMessage),
    SetConVar(SetConVarMessage),
    SignOnState(SignOnStateMessage),
    Print(PrintMessage),
    ServerInfo(Box<ServerInfoMessage>),
    ClassInfo(ClassInfoMessage),
    SetPause(SetPauseMessage),
    CreateStringTable(CreateStringTableMessage<'a>),
    UpdateStringTable(UpdateStringTableMessage<'a>),
    VoiceInit(VoiceInitMessage),
    VoiceData(VoiceDataMessage<'a>),
    ParseSounds(ParseSoundsMessage<'a>),
    SetView(SetViewMessage),
    FixAngle(FixAngleMessage),
    BspDecal(BSPDecalMessage),
    UserMessage(UserMessage<'a>),
    EntityMessage(EntityMessage<'a>),
    GameEvent(GameEventMessage),
    PacketEntities(PacketEntitiesMessage),
    TempEntities(TempEntitiesMessage),
    PreFetch(PreFetchMessage),
    Menu(MenuMessage<'a>),
    GameEventList(GameEventListMessage),
    GetCvarValue(GetCvarValueMessage),
    CmdKeyValues(CmdKeyValuesMessage<'a>),
}

impl<'a> Parse<'a> for Message<'a> {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let message_type = MessageType::parse(stream, state)?;
        Self::from_type(message_type, stream, state)
    }
}

impl<'a> Message<'a> {
    pub fn get_message_type(&self) -> MessageType {
        match self {
            Message::Empty => MessageType::Empty,
            Message::File(_) => MessageType::File,
            Message::NetTick(_) => MessageType::NetTick,
            Message::StringCmd(_) => MessageType::StringCmd,
            Message::SetConVar(_) => MessageType::SetConVar,
            Message::SignOnState(_) => MessageType::SignOnState,
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
        stream: &mut Stream<'a>,
        state: &ParserState,
    ) -> Result<Self> {
        Ok(match message_type {
            MessageType::Empty => Message::Empty,
            MessageType::File => Message::File(FileMessage::parse(stream, state)?),
            MessageType::NetTick => Message::NetTick(NetTickMessage::parse(stream, state)?),
            MessageType::StringCmd => Message::StringCmd(StringCmdMessage::parse(stream, state)?),
            MessageType::SetConVar => Message::SetConVar(SetConVarMessage::parse(stream, state)?),
            MessageType::SignOnState => {
                Message::SignOnState(SignOnStateMessage::parse(stream, state)?)
            }
            MessageType::Print => Message::Print(PrintMessage::parse(stream, state)?),
            MessageType::ServerInfo => {
                Message::ServerInfo(Box::new(ServerInfoMessage::parse(stream, state)?))
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

    pub fn skip_type(
        message_type: MessageType,
        stream: &mut Stream,
        state: &ParserState,
    ) -> Result<()> {
        match message_type {
            MessageType::Empty => Ok(()),
            MessageType::File => FileMessage::parse_skip(stream, state),
            MessageType::NetTick => NetTickMessage::parse_skip(stream, state),
            MessageType::StringCmd => StringCmdMessage::parse_skip(stream, state),
            MessageType::SetConVar => SetConVarMessage::parse_skip(stream, state),
            MessageType::SignOnState => SignOnStateMessage::parse_skip(stream, state),
            MessageType::Print => PrintMessage::parse_skip(stream, state),
            MessageType::ServerInfo => ServerInfoMessage::parse_skip(stream, state),
            MessageType::ClassInfo => ClassInfoMessage::parse_skip(stream, state),
            MessageType::SetPause => SetPauseMessage::parse_skip(stream, state),
            MessageType::CreateStringTable => CreateStringTableMessage::parse_skip(stream, state),
            MessageType::UpdateStringTable => UpdateStringTableMessage::parse_skip(stream, state),
            MessageType::VoiceInit => VoiceInitMessage::parse_skip(stream, state),
            MessageType::VoiceData => VoiceDataMessage::parse_skip(stream, state),
            MessageType::ParseSounds => ParseSoundsMessage::parse_skip(stream, state),
            MessageType::SetView => SetViewMessage::parse_skip(stream, state),
            MessageType::FixAngle => FixAngleMessage::parse_skip(stream, state),
            MessageType::BspDecal => BSPDecalMessage::parse_skip(stream, state),
            MessageType::UserMessage => UserMessage::parse_skip(stream, state),
            MessageType::EntityMessage => EntityMessage::parse_skip(stream, state),
            MessageType::GameEvent => GameEventMessage::parse_skip(stream, state),
            MessageType::PacketEntities => PacketEntitiesMessage::parse_skip(stream, state),
            MessageType::TempEntities => TempEntitiesMessage::parse_skip(stream, state),
            MessageType::PreFetch => PreFetchMessage::parse_skip(stream, state),
            MessageType::Menu => MenuMessage::parse_skip(stream, state),
            MessageType::GameEventList => GameEventListMessage::parse_skip(stream, state),
            MessageType::GetCvarValue => GetCvarValueMessage::parse_skip(stream, state),
            MessageType::CmdKeyValues => CmdKeyValuesMessage::parse_skip(stream, state),
        }
    }
}

impl Encode for Message<'_> {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        match self {
            Message::Empty => Ok(()),
            Message::File(message) => message.encode(stream, state),
            Message::NetTick(message) => message.encode(stream, state),
            Message::StringCmd(message) => message.encode(stream, state),
            Message::SetConVar(message) => message.encode(stream, state),
            Message::SignOnState(message) => message.encode(stream, state),
            Message::Print(message) => message.encode(stream, state),
            Message::ServerInfo(message) => message.encode(stream, state),
            Message::ClassInfo(message) => message.encode(stream, state),
            Message::SetPause(message) => message.encode(stream, state),
            Message::CreateStringTable(message) => message.encode(stream, state),
            Message::UpdateStringTable(message) => message.encode(stream, state),
            Message::VoiceInit(message) => message.encode(stream, state),
            Message::VoiceData(message) => message.encode(stream, state),
            Message::ParseSounds(message) => message.encode(stream, state),
            Message::SetView(message) => message.encode(stream, state),
            Message::FixAngle(message) => message.encode(stream, state),
            Message::BspDecal(message) => message.encode(stream, state),
            Message::UserMessage(message) => message.encode(stream, state),
            Message::EntityMessage(message) => message.encode(stream, state),
            Message::GameEvent(message) => message.encode(stream, state),
            Message::PacketEntities(message) => message.encode(stream, state),
            Message::TempEntities(message) => message.encode(stream, state),
            Message::PreFetch(message) => message.encode(stream, state),
            Message::Menu(message) => message.encode(stream, state),
            Message::GameEventList(message) => message.encode(stream, state),
            Message::GetCvarValue(message) => message.encode(stream, state),
            Message::CmdKeyValues(message) => message.encode(stream, state),
        }
    }
}
