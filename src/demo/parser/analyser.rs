use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::demo::gameevent_gen::{
    GameEvent, PlayerDeathEvent, PlayerSpawnEvent, TeamPlayRoundWinEvent,
};
use crate::demo::message::packetentities::EntityId;
use crate::demo::message::usermessage::{ChatMessageKind, SayText2Message, UserMessage};
use crate::demo::message::{Message, MessageType};
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::handler::MessageHandler;
use crate::demo::vector::Vector;
use crate::{ParserState, ReadResult, Stream};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChatMassage {
    pub kind: ChatMessageKind,
    pub from: String,
    pub text: String,
    pub tick: u32,
}

impl ChatMassage {
    pub fn from_message(message: &SayText2Message, tick: u32) -> Self {
        ChatMassage {
            kind: message.kind,
            from: message.from.clone().unwrap_or_default(),
            text: message.text.clone(),
            tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Other = 0,
    Spectator = 1,
    #[serde(rename = "red")]
    Red = 2,
    #[serde(rename = "blue")]
    Blue = 3,
}

impl Team {
    pub fn new(number: u16) -> Self {
        match number {
            1 => Team::Spectator,
            2 => Team::Red,
            3 => Team::Blue,
            _ => Team::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Class {
    Other = 0,
    Scout = 1,
    Sniper = 2,
    Solder = 3,
    Demoman = 4,
    Medic = 5,
    Heavy = 6,
    Pyro = 7,
    Spy = 8,
    Engineer = 9,
}

impl Class {
    pub fn new(number: u16) -> Self {
        match number {
            1 => Class::Scout,
            2 => Class::Sniper,
            3 => Class::Solder,
            4 => Class::Demoman,
            5 => Class::Medic,
            6 => Class::Heavy,
            7 => Class::Pyro,
            8 => Class::Spy,
            9 => Class::Engineer,
            _ => Class::Other,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u8);

impl From<u32> for UserId {
    fn from(int: u32) -> Self {
        UserId((int & 255) as u8)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Spawn {
    pub user: UserId,
    pub class: Class,
    pub team: Team,
    pub tick: u32,
}

impl Spawn {
    pub fn from_event(event: &PlayerSpawnEvent, tick: u32) -> Self {
        Spawn {
            user: UserId((event.user_id & 255) as u8),
            class: Class::new(event.class),
            team: Team::new(event.team),
            tick,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInfo {
    pub name: String,
    pub user_id: UserId,
    pub steam_id: String,
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Death {
    pub weapon: String,
    pub victim: UserId,
    pub assister: Option<UserId>,
    pub killer: UserId,
    pub tick: u32,
}

impl Death {
    pub fn from_event(event: &PlayerDeathEvent, tick: u32) -> Self {
        let assister = if event.assister < (16 * 1024) {
            Some(UserId((event.assister & 255) as u8))
        } else {
            None
        };
        Death {
            assister,
            tick,
            killer: UserId((event.attacker & 255) as u8),
            weapon: event.weapon.clone(),
            victim: UserId((event.user_id & 255) as u8),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Round {
    winner: Team,
    length: f32,
    end_tick: u32,
}

impl Round {
    pub fn from_event(event: &TeamPlayRoundWinEvent, tick: u32) -> Self {
        Round {
            winner: Team::new(event.team as u16),
            length: event.round_time,
            end_tick: tick,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct World {
    boundary_min: Vector,
    boundary_max: Vector,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct Analyser {
    pub chat: Vec<ChatMassage>,
    pub users: HashMap<UserId, UserInfo>,
    pub user_spawns: Vec<Spawn>,
    pub deaths: Vec<Death>,
    pub rounds: Vec<Round>,
    pub start_tick: u32,
}

impl MessageHandler for Analyser {
    type Output = MatchState;

    fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::GameEvent | MessageType::UserMessage => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: &Message, tick: u32) {
        if self.start_tick == 0 {
            self.start_tick = tick;
        }
        match message {
            Message::GameEvent(message) => self.handle_event(&message.event, tick),
            Message::UserMessage(message) => self.handle_user_message(&message, tick),
            _ => {}
        }
    }

    fn handle_string_entry(&mut self, table: &String, _index: usize, entry: &StringTableEntry) {
        match table.as_str() {
            "userinfo" => {
                if let (Some(text), Some(data)) = (&entry.text, &entry.extra_data) {
                    if data.byte_len > 32 {
                        let _ = self.parse_user_info(text, data.data.clone());
                    }
                }
            }
            _ => {}
        }
    }

    fn get_output(self, state: ParserState) -> MatchState {
        MatchState {
            start_tick: self.start_tick,
            interval_per_tick: state.demo_meta.interval_per_tick,
            chat: self.chat,
            deaths: self.deaths,
            rounds: self.rounds,
            users: UserState::from_users_and_spawn(self.users, self.user_spawns),
        }
    }
}

impl Analyser {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_user_message(&mut self, message: &UserMessage, tick: u32) {
        if let UserMessage::SayText2(text_message) = message {
            if text_message.kind == ChatMessageKind::NameChange {
                if let Some(from) = text_message.from.clone() {
                    self.change_name(from, text_message.text.clone());
                }
            } else {
                self.chat
                    .push(ChatMassage::from_message(text_message, tick));
            }
        }
    }

    fn change_name(&mut self, from: String, to: String) {
        if let Some(user) = self.users.values_mut().find(|user| user.name == from) {
            user.name = to;
        }
    }

    fn handle_event(&mut self, event: &GameEvent, tick: u32) {
        const WIN_REASON_TIME_LIMIT: u8 = 6;

        match event {
            GameEvent::PlayerDeath(event) => self.deaths.push(Death::from_event(event, tick)),
            GameEvent::PlayerSpawn(event) => self.user_spawns.push(Spawn::from_event(event, tick)),
            GameEvent::TeamPlayRoundWin(event) => {
                if event.win_reason != WIN_REASON_TIME_LIMIT {
                    self.rounds.push(Round::from_event(event, tick))
                }
            }
            _ => {}
        }
    }

    fn parse_user_info(&mut self, text: &str, mut data: Stream) -> ReadResult<()> {
        let name: String = data.read_sized(32).unwrap_or("Malformed Name".into());
        let user_id = data.read::<u32>()?.into();
        let steam_id: String = data.read()?;

        match text.parse() {
            Ok(entity_id) if (steam_id.len() > 0) => {
                self.users.insert(
                    user_id,
                    UserInfo {
                        steam_id,
                        user_id,
                        name,
                        entity_id,
                    },
                );
            }
            _ => {}
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserState {
    pub classes: HashMap<Class, u8>,
    pub name: String,
    pub user_id: UserId,
    pub steam_id: String,
    pub team: Team,
}

impl From<UserInfo> for UserState {
    fn from(user: UserInfo) -> Self {
        UserState {
            classes: HashMap::new(),
            team: Team::Other,
            name: user.name,
            user_id: user.user_id,
            steam_id: user.steam_id,
        }
    }
}

impl UserState {
    pub fn from_users_and_spawn(
        users: HashMap<UserId, UserInfo>,
        spawns: Vec<Spawn>,
    ) -> HashMap<UserId, Self> {
        let mut user_states: HashMap<_, _> = users
            .into_iter()
            .map(|(_, user)| (user.user_id, UserState::from(user)))
            .collect();

        for spawn in spawns {
            if let Some(user) = user_states.get_mut(&spawn.user) {
                user.handle_spawn(&spawn);
            }
        }

        user_states
    }

    fn handle_spawn(&mut self, spawn: &Spawn) {
        self.team = spawn.team;
        *self.classes.entry(spawn.class).or_default() += 1;
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MatchState {
    pub chat: Vec<ChatMassage>,
    pub users: HashMap<UserId, UserState>,
    pub deaths: Vec<Death>,
    pub rounds: Vec<Round>,
    pub start_tick: u32,
    pub interval_per_tick: f32,
}
