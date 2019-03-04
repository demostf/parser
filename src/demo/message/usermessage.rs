use bitstream_reader::{BitRead, BitReadSized, LittleEndian};
use enum_primitive_derive::Primitive;
use num_traits::{FromPrimitive, ToPrimitive};
use serde::Serialize;

use crate::demo::message::usermessage::UserMessage::SayText2;
use crate::{ReadResult, Stream};

#[derive(Primitive, Clone, Copy, Debug)]
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

#[derive(Debug)]
pub enum UserMessage {
    SayText2(SayText2Message),
    Text(TextMessage),
    ResetHUD(ResetHudMessage),
    Train(TrainMessage),
    VoiceSubtitle(VoiceSubtitleMessage),
    Shake(ShakeMessage),
    Unknown(UnknownUserMessage),
}

impl BitRead<LittleEndian> for UserMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let message_type =
            UserMessageType::from_u8(stream.read()?).unwrap_or(UserMessageType::Unknown);
        let length = stream.read_int(11)?;
        let mut data = stream.read_bits(length)?;
        let message = match message_type {
            UserMessageType::SayText2 => UserMessage::SayText2(data.read()?),
            //UserMessageType::TextMsg => UserMessage::Text(data.read()?),
            UserMessageType::ResetHUD => UserMessage::ResetHUD(data.read()?),
            UserMessageType::Train => UserMessage::Train(data.read()?),
            UserMessageType::VoiceSubtitle => UserMessage::VoiceSubtitle(data.read()?),
            UserMessageType::Shake => UserMessage::Shake(data.read()?),
            _ => UserMessage::Unknown(data.read()?),
        };
        Ok(message)
    }
}

#[derive(Debug, Clone, Serialize)]
pub enum ChatMessageKind {
    ChatAll,
    ChatTeam,
    ChatAllDead,
    NameChange,
}

impl BitRead<LittleEndian> for ChatMessageKind {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let raw: String = stream.read()?;
        Ok(match raw.as_str() {
            "TF_Chat_Team" => ChatMessageKind::ChatTeam,
            "TF_Chat_AllDead" => ChatMessageKind::ChatAllDead,
            "#TF_Name_Change" => ChatMessageKind::NameChange,
            _ => ChatMessageKind::ChatAll,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SayText2Message {
    pub client: u8,
    pub raw: u8,
    pub kind: ChatMessageKind,
    pub from: String,
    pub text: String,
}

impl BitRead<LittleEndian> for SayText2Message {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let client = stream.read()?;
        let raw = stream.read()?;
        let (kind, from, text): (ChatMessageKind, String, String) = if stream.read::<u8>()? == 1 {
            let first = stream.read::<u8>()?;
            if first == 7 {
                let _color = stream.read_string(Some(6))?;
            } else {
                let _ = stream.skip(8)?;
            }

            let text: String = stream.read()?;
            if text.starts_with("*DEAD*") {
                // grave talk is in the format '*DEAD* \u0003$from\u0001:    $text'b
                let start = text.find(char::from(3)).unwrap_or(0);
                let end = text.find(char::from(1)).unwrap_or(0);
                let from: String = String::from_utf8(
                    text.bytes().skip(start + 1).take(end - start - 1).collect(),
                )?;
                let text: String = String::from_utf8(text.bytes().skip(end + 5).collect())?;
                let kind = ChatMessageKind::ChatAllDead;
                (kind, from, text)
            } else {
                (ChatMessageKind::ChatAll, "".to_owned(), text)
            }
        } else {
            let _ = stream.set_pos(stream.pos() - 8)?;

            let kind = stream.read()?;
            let from = stream.read()?;
            let text = stream.read()?;
            let _ = stream.skip(16)?;
            (kind, from, text)
        };

        // cleanup color codes
        let mut text = text.replace(char::from(1), "").replace(char::from(3), "");
        while let Some(pos) = text.find(char::from(7)) {
            text = String::from_utf8(
                text.bytes()
                    .take(pos)
                    .chain(text.bytes().skip(pos + 7))
                    .collect(),
            )?;
        }

        Ok(SayText2Message {
            client,
            raw,
            kind,
            from,
            text,
        })
    }
}

#[derive(BitRead, Debug, Clone)]
#[discriminant_bits = 8]
pub enum HudTextLocation {
    PrintNotify = 1,
    PrintConsole,
    PrintTalk,
    PrintCenter,
}

#[derive(BitRead, Debug, Clone)]
pub struct TextMessage {
    pub location: HudTextLocation,
    pub text: String,
    #[size = 4]
    pub substitute: Vec<String>,
}

#[derive(BitRead, Debug, Clone)]
pub struct ResetHudMessage {
    pub data: u8,
}

#[derive(BitRead, Debug, Clone)]
pub struct TrainMessage {
    pub data: u8,
}

#[derive(BitRead, Debug, Clone)]
pub struct VoiceSubtitleMessage {
    client: u8,
    menu: u8,
    item: u8,
}

#[derive(BitRead, Debug, Clone)]
pub struct ShakeMessage {
    command: u8,
    amplitude: f32,
    frequency: f32,
    duration: f32,
}

#[derive(Debug, Clone)]
pub struct UnknownUserMessage {
    data: Stream,
}

impl BitRead<LittleEndian> for UnknownUserMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        Ok(UnknownUserMessage {
            data: stream.read_bits(stream.bits_left())?,
        })
    }
}
