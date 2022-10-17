use bitbuffer::{BitError, BitRead, BitWrite, BitWriteStream, Endianness, LittleEndian};
use serde::{Deserialize, Serialize};

use crate::demo::data::MaybeUtf8String;
use crate::demo::message::packetentities::EntityId;
use crate::{ReadResult, Stream};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
#[discriminant_bits = 8]
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
    VGuiMenu = 12,
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

impl PartialEq<u8> for UserMessageType {
    fn eq(&self, other: &u8) -> bool {
        *self as u8 == *other
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(bound(deserialize = "'a: 'static"))]
#[serde(tag = "type")]
pub enum UserMessage<'a> {
    SayText2(Box<SayText2Message>),
    Text(Box<TextMessage>),
    ResetHUD(ResetHudMessage),
    Train(TrainMessage),
    VoiceSubtitle(VoiceSubtitleMessage),
    Shake(ShakeMessage),
    VGuiMenu(VGuiMenuMessage),
    Rumble(RumbleMessage),
    Fade(FadeMessage),
    HapMeleeContact(HapMeleeContactMessage),
    Unknown(UnknownUserMessage<'a>),
}

impl UserMessage<'_> {
    pub fn message_type(&self) -> u8 {
        match self {
            UserMessage::SayText2(_) => UserMessageType::SayText2 as u8,
            UserMessage::Text(_) => UserMessageType::TextMsg as u8,
            UserMessage::ResetHUD(_) => UserMessageType::ResetHUD as u8,
            UserMessage::Train(_) => UserMessageType::Train as u8,
            UserMessage::VoiceSubtitle(_) => UserMessageType::VoiceSubtitle as u8,
            UserMessage::Shake(_) => UserMessageType::Shake as u8,
            UserMessage::VGuiMenu(_) => UserMessageType::VGuiMenu as u8,
            UserMessage::Rumble(_) => UserMessageType::Rumble as u8,
            UserMessage::Fade(_) => UserMessageType::Fade as u8,
            UserMessage::HapMeleeContact(_) => UserMessageType::HapMeleeContact as u8,
            UserMessage::Unknown(msg) => msg.raw_type,
        }
    }
}

impl<'a> BitRead<'a, LittleEndian> for UserMessage<'a> {
    fn read(stream: &mut Stream<'a>) -> ReadResult<Self> {
        let message = match stream.read() {
            Ok(message_type) => {
                let length = stream.read_int(11)?;
                let mut data = stream.read_bits(length)?;
                match message_type {
                    UserMessageType::SayText2 => UserMessage::SayText2(data.read()?),
                    UserMessageType::TextMsg => UserMessage::Text(data.read()?),
                    UserMessageType::ResetHUD => UserMessage::ResetHUD(data.read()?),
                    UserMessageType::Train => UserMessage::Train(data.read()?),
                    UserMessageType::VoiceSubtitle => UserMessage::VoiceSubtitle(data.read()?),
                    UserMessageType::Shake => UserMessage::Shake(data.read()?),
                    UserMessageType::VGuiMenu => UserMessage::VGuiMenu(data.read()?),
                    UserMessageType::Rumble => UserMessage::Rumble(data.read()?),
                    UserMessageType::Fade => UserMessage::Fade(data.read()?),
                    UserMessageType::HapMeleeContact => UserMessage::HapMeleeContact(data.read()?),
                    _ => UserMessage::Unknown(UnknownUserMessage {
                        raw_type: message_type as u8,
                        data,
                    }),
                }
            }
            Err(BitError::UnmatchedDiscriminant { discriminant, .. }) => {
                let length = stream.read_int(11)?;
                let data = stream.read_bits(length)?;
                UserMessage::Unknown(UnknownUserMessage {
                    raw_type: discriminant as u8,
                    data,
                })
            }
            Err(e) => return Err(e),
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
        self.message_type().write(stream)?;
        stream.reserve_length(11, |stream| match self {
            UserMessage::SayText2(body) => stream.write(body),
            UserMessage::Text(body) => stream.write(body),
            UserMessage::ResetHUD(body) => stream.write(body),
            UserMessage::Train(body) => stream.write(body),
            UserMessage::VoiceSubtitle(body) => stream.write(body),
            UserMessage::Shake(body) => stream.write(body),
            UserMessage::VGuiMenu(body) => stream.write(body),
            UserMessage::Rumble(body) => stream.write(body),
            UserMessage::Fade(body) => stream.write(body),
            UserMessage::HapMeleeContact(body) => stream.write(body),
            UserMessage::Unknown(body) => stream.write(&body.data),
        })?;

        Ok(())
    }
}

#[test]
fn test_user_message_roundtrip() {
    crate::test_roundtrip_write(UserMessage::Train(TrainMessage { data: 12 }));
    crate::test_roundtrip_write(UserMessage::SayText2(Box::new(SayText2Message {
        client: 3u32.into(),
        raw: 1,
        kind: ChatMessageKind::ChatTeamDead,
        from: Some("Old Billy Riley".into()),
        text: "[P-REC] Stop record.".into(),
    })));
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
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

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SayText2Message {
    pub client: EntityId,
    pub raw: u8,
    pub kind: ChatMessageKind,
    pub from: Option<MaybeUtf8String>,
    pub text: MaybeUtf8String,
}

impl SayText2Message {
    pub fn plain_text(&self) -> String {
        // 1: normal, 2: old colors, 3: team, 4: location, 5 achievement, 6 custom
        let mut text = self.text.to_string().replace(|c| c <= char::from(6), "");
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
        let client = EntityId::from(stream.read::<u8>()? as u32);
        let raw = stream.read()?;
        let (kind, from, text): (ChatMessageKind, Option<MaybeUtf8String>, MaybeUtf8String) =
            if stream.read::<u8>()? == 1 {
                stream.set_pos(stream.pos() - 8)?;

                let text: MaybeUtf8String = stream.read()?;
                (ChatMessageKind::ChatAll, None, text)
            } else {
                stream.set_pos(stream.pos() - 8)?;

                let kind = stream.read()?;
                let from = stream.read()?;
                let text = stream.read()?;

                // ends with 2 0 bytes?
                if stream.bits_left() >= 16 {
                    let _: u16 = stream.read()?;
                }
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
        (u32::from(self.client) as u8).write(stream)?;
        self.raw.write(stream)?;

        if let Some(from) = self.from.as_ref().map(|s| s.as_ref()) {
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
        client: 3u32.into(),
        raw: 1,
        kind: ChatMessageKind::ChatTeamDead,
        from: Some("Old Billy Riley".into()),
        text: "[P-REC] Stop record.".into(),
    });
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[discriminant_bits = 8]
pub enum HudTextLocation {
    PrintNotify = 1,
    PrintConsole,
    PrintTalk,
    PrintCenter,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextMessage {
    pub location: HudTextLocation,
    pub text: MaybeUtf8String,
    pub substitute: [MaybeUtf8String; 4],
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResetHudMessage {
    pub data: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainMessage {
    pub data: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VoiceSubtitleMessage {
    pub client: u8,
    pub menu: u8,
    pub item: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShakeMessage {
    pub command: u8,
    pub amplitude: f32,
    pub frequency: f32,
    pub duration: f32,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VGuiMenuMessage {
    pub name: MaybeUtf8String,
    pub show: u8,
    #[size_bits = 8]
    pub data: Vec<VGuiMenuMessageData>,
}

impl<E: Endianness> BitWrite<E> for VGuiMenuMessage {
    fn write(&self, stream: &mut BitWriteStream<E>) -> ReadResult<()> {
        self.name.write(stream)?;
        self.show.write(stream)?;
        (self.data.len() as u8).write(stream)?;
        for item in &self.data {
            item.write(stream)?;
        }
        Ok(())
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VGuiMenuMessageData {
    pub key: MaybeUtf8String,
    pub data: MaybeUtf8String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RumbleMessage {
    pub waveform_index: u8,
    pub rumble_data: u8,
    pub rumble_flags: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FadeMessage {
    pub duration: u16,
    pub hold: u16,
    pub flags: u16,
    pub color: [u8; 4],
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitRead, BitWrite, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HapMeleeContactMessage {
    pub data: u8,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(bound(deserialize = "'a: 'static"))]
pub struct UnknownUserMessage<'a> {
    pub raw_type: u8,
    pub data: Stream<'a>,
}
