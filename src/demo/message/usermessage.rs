use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

use crate::demo::handle_utf8_error;

use crate::{ReadResult, Stream};

#[derive(TryFromPrimitive, Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UserMessageType {
    Geiger = 0,
    Train = 1,
    HudText = 2,
    SayText = 3,
    SayText2 = 4,
    TextMsg = 5,
    ResetHUD = 6,
    GameTitle = 7,
    ItemPickup = 8,
    ShowMenu = 9,
    Shake = 10,
    Fade = 11,
    VGUIMenu = 12,
    Rumble = 13,
    CloseCaption = 14,
    SendAudio = 15,
    VoiceMask = 16,
    RequestState = 17,
    Damage = 18,
    HintText = 19,
    KeyHintText = 20,
    HudMsg = 21,
    AmmoDenied = 22,
    AchievementEvent = 23,
    UpdateRadar = 24,
    VoiceSubtitle = 25,
    HudNotify = 26,
    HudNotifyCustom = 27,
    PlayerStatsUpdate = 28,
    PlayerIgnited = 29,
    PlayerIgnitedInv = 30,
    HudArenaNotify = 31,
    UpdateAchievement = 32,
    TrainingMsg = 33,
    TrainingObjective = 34,
    DamageDodged = 35,
    PlayerJarated = 36,
    PlayerExtinguished = 37,
    PlayerJaratedFade = 38,
    PlayerShieldBlocked = 39,
    BreakModel = 40,
    CheapBreakModel = 41,
    BreakModelPumpkin = 42,
    BreakModelRocketDud = 43,
    CallVoteFailed = 44,
    VoteStart = 45,
    VotePass = 46,
    VoteFailed = 47,
    VoteSetup = 48,
    PlayerBonusPoints = 49,
    SpawnFlyingBird = 50,
    PlayerGodRayEffect = 51,
    SPHapWeapEvent = 52,
    HapDmg = 53,
    HapPunch = 54,
    HapSetDrag = 55,
    HapSet = 56,
    HapMeleeContact = 57,
    Unknown = 255,
}

#[derive(Debug, PartialEq)]
pub enum UserMessage<'a> {
    SayText2(Box<SayText2Message>),
    Text(Box<TextMessage>),
    ResetHUD(ResetHudMessage),
    Train(TrainMessage),
    VoiceSubtitle(VoiceSubtitleMessage),
    Shake(ShakeMessage),
    Unknown(UserMessageType, UnknownUserMessage<'a>),
}

impl UserMessage<'_> {
    pub fn message_type(&self) -> UserMessageType {
        match self {
            UserMessage::SayText2(_) => UserMessageType::SayText2,
            UserMessage::Text(_) => UserMessageType::TextMsg,
            UserMessage::ResetHUD(_) => UserMessageType::ResetHUD,
            UserMessage::Train(_) => UserMessageType::Train,
            UserMessage::VoiceSubtitle(_) => UserMessageType::VoiceSubtitle,
            UserMessage::Shake(_) => UserMessageType::Shake,
            UserMessage::Unknown(ty, _) => *ty,
        }
    }
}

impl<'a> BitRead<'a, LittleEndian> for UserMessage<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        let message_type =
            UserMessageType::try_from(stream.read::<u8>()?).unwrap_or(UserMessageType::Unknown);
        let length = stream.read_int(11)?;
        let mut data = stream.read_bits(length)?;
        let message = match message_type {
            UserMessageType::SayText2 => UserMessage::SayText2(data.read()?),
            //UserMessageType::TextMsg => UserMessage::Text(data.read()?),
            UserMessageType::ResetHUD => UserMessage::ResetHUD(data.read()?),
            UserMessageType::Train => UserMessage::Train(data.read()?),
            UserMessageType::VoiceSubtitle => UserMessage::VoiceSubtitle(data.read()?),
            UserMessageType::Shake => UserMessage::Shake(data.read()?),
            _ => UserMessage::Unknown(message_type, data.read()?),
        };
        Ok(message)
    }

    fn skip(stream: &mut Stream) -> ReadResult<()> {
        stream.skip_bits(8)?;
        let length: u32 = stream.read_int(11)?;
        stream.skip_bits(length as usize)
    }
}

impl<'a> BitWrite<LittleEndian> for UserMessage<'a> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        (self.message_type() as u8).write(stream)?;
        stream.reserve_length(11, |stream| match self {
            UserMessage::SayText2(body) => stream.write(body),
            UserMessage::Text(body) => stream.write(body),
            UserMessage::ResetHUD(body) => stream.write(body),
            UserMessage::Train(body) => stream.write(body),
            UserMessage::VoiceSubtitle(body) => stream.write(body),
            UserMessage::Shake(body) => stream.write(body),
            UserMessage::Unknown(_, body) => stream.write(body),
        })?;

        Ok(())
    }
}

#[test]
fn test_user_message_roundtrip() {
    crate::test_roundtrip_write(UserMessage::Train(TrainMessage { data: 12 }));
    crate::test_roundtrip_write(UserMessage::SayText2(Box::new(SayText2Message {
        client: 3,
        raw: 1,
        kind: ChatMessageKind::ChatTeamDead,
        from: Some("Old Billy Riley".into()),
        text: "[P-REC] Stop record.".into(),
    })));
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ChatMessageKind {
    #[serde(rename = "TF_Chat_All")]
    ChatAll,
    #[serde(rename = "TF_Chat_Team")]
    ChatTeam,
    #[serde(rename = "TF_Chat_AllDead")]
    ChatAllDead,
    #[serde(rename = "TF_Chat_Team_Dead")]
    ChatTeamDead,
    #[serde(rename = "TF_Chat_AllSpec")]
    ChatAllSpec,
    NameChange,
    Empty,
}

impl BitRead<'_, LittleEndian> for ChatMessageKind {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let raw: String = stream.read()?;
        Ok(match raw.as_str() {
            "TF_Chat_Team" => ChatMessageKind::ChatTeam,
            "TF_Chat_AllDead" => ChatMessageKind::ChatAllDead,
            "TF_Chat_Team_Dead" => ChatMessageKind::ChatTeamDead,
            "#TF_Name_Change" => ChatMessageKind::NameChange,
            "TF_Chat_All" => ChatMessageKind::ChatAll,
            "TF_Chat_AllSpec" => ChatMessageKind::ChatAllSpec,
            _ => ChatMessageKind::ChatAll,
        })
    }
}

impl BitWrite<LittleEndian> for ChatMessageKind {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        match self {
            ChatMessageKind::ChatAll => "TF_Chat_All",
            ChatMessageKind::ChatTeam => "TF_Chat_Team",
            ChatMessageKind::ChatAllDead => "TF_Chat_AllDead",
            ChatMessageKind::ChatTeamDead => "TF_Chat_Team_Dead",
            ChatMessageKind::ChatAllSpec => "TF_Chat_AllSpec",
            ChatMessageKind::NameChange => "#TF_Name_Change",
            ChatMessageKind::Empty => "",
        }
        .write(stream)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SayText2Message {
    pub client: u8,
    pub raw: u8,
    pub kind: ChatMessageKind,
    pub from: Option<String>,
    pub text: String,
}

impl SayText2Message {
    pub fn plain_text(&self) -> String {
        // 1: normal, 2: team, 3: team, 4: location
        let mut text = self.text.replace(|c| c <= char::from(4), "");
        // 7: 6-char hex
        while let Some(pos) = text.chars().enumerate().find_map(|(index, c)| {
            if c == char::from(7) {
                Some(index)
            } else {
                None
            }
        }) {
            text = text
                .chars()
                .take(pos)
                .chain(text.chars().skip(pos + 7))
                .collect();
        }
        // 9: 8-char hex
        while let Some(pos) = text.chars().enumerate().find_map(|(index, c)| {
            if c == char::from(9) {
                Some(index)
            } else {
                None
            }
        }) {
            text = text
                .chars()
                .take(pos)
                .chain(text.chars().skip(pos + 9))
                .collect();
        }
        text
    }
}

impl BitRead<'_, LittleEndian> for SayText2Message {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let client = stream.read()?;
        let raw = stream.read()?;
        let (kind, from, text): (ChatMessageKind, Option<String>, String) =
            if stream.read::<u8>()? == 1 {
                stream.set_pos(stream.pos() - 8)?;

                let text: String = stream.read().or_else(handle_utf8_error)?;
                (ChatMessageKind::ChatAll, None, text)
            } else {
                stream.set_pos(stream.pos() - 8)?;

                let kind = stream.read()?;
                let from = stream.read().or_else(handle_utf8_error)?;
                let text = stream.read().or_else(handle_utf8_error)?;

                // always ends with 2 0 bytes?
                let _: u16 = stream.read()?;
                (kind, Some(from), text)
            };

        Ok(SayText2Message {
            client,
            raw,
            kind,
            from,
            text,
        })
    }
}

impl BitWrite<LittleEndian> for SayText2Message {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.client.write(stream)?;
        self.raw.write(stream)?;

        if let Some(from) = self.from.as_deref() {
            self.kind.write(stream)?;
            from.write(stream)?;
            self.text.write(stream)?;
            0u16.write(stream)?;
        } else {
            self.text.write(stream)?;
        }

        Ok(())
    }
}

#[test]
fn test_say_text2_roundtrip() {
    crate::test_roundtrip_write(SayText2Message {
        client: 3,
        raw: 1,
        kind: ChatMessageKind::ChatTeamDead,
        from: Some("Old Billy Riley".into()),
        text: "[P-REC] Stop record.".into(),
    });
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
#[discriminant_bits = 8]
pub enum HudTextLocation {
    PrintNotify = 1,
    PrintConsole,
    PrintTalk,
    PrintCenter,
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
pub struct TextMessage {
    pub location: HudTextLocation,
    pub text: String,
    pub substitute: [String; 4],
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
pub struct ResetHudMessage {
    pub data: u8,
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
pub struct TrainMessage {
    pub data: u8,
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
pub struct VoiceSubtitleMessage {
    client: u8,
    menu: u8,
    item: u8,
}

#[derive(BitRead, BitWrite, Debug, Clone, PartialEq)]
pub struct ShakeMessage {
    command: u8,
    amplitude: f32,
    frequency: f32,
    duration: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnknownUserMessage<'a> {
    data: Stream<'a>,
}

impl<'a> BitRead<'a, LittleEndian> for UnknownUserMessage<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        Ok(UnknownUserMessage {
            data: stream.read_bits(stream.bits_left())?,
        })
    }
}

impl<'a> BitWrite<LittleEndian> for UnknownUserMessage<'a> {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.data.write(stream)
    }
}
