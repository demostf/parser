use std::collections::HashMap;

use serde::Serialize;

use crate::{ReadResult, Stream};
use crate::demo::gameevent_gen::{GameEvent, PlayerDeathEvent, PlayerSpawnEvent};
use crate::demo::message::{Message, MessageType};
use crate::demo::message::packetentities::EntityId;
use crate::demo::message::usermessage::{ChatMessageKind, UserMessage};
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::dispatcher::{MessageHandler, StringTableEntryHandler};
use crate::demo::vector::Vector;

#[derive(Debug, Clone, Serialize)]
pub struct ChatMassage {
    pub kind: ChatMessageKind,
    pub from: String,
    pub text: String,
    pub tick: u32,
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash)]
pub enum Team {
    Other = 0,
    Spectator = 1,
    Red = 2,
    Blue = 3,
}

impl Team {
    pub fn new(number: u16) -> Self {
        match number {
            1 => Team::Spectator,
            2 => Team::Red,
            3 => Team::Blue,
            _ => Team::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash)]
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
            _ => Class::Other
        }
    }
}

#[derive(Debug, Clone, Serialize, Copy, PartialEq, Eq, Hash)]
pub struct UserId(u8);

#[derive(Debug, Clone, Serialize)]
pub struct Spawn {
    pub user: UserId,
    pub class: Class,
    pub team: Team,
    pub tick: u32,
}

impl Spawn {
    pub fn from_event(event: PlayerSpawnEvent, tick: u32) -> Self {
        Spawn {
            user: UserId((event.user_id & 255) as u8),
            class: Class::new(event.class),
            team: Team::new(event.team),
            tick,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct UserInfo {
    pub name: String,
    pub user_id: UserId,
    pub steam_id: String,
    pub entity_id: EntityId,
}

#[derive(Debug, Clone, Serialize)]
pub struct Death {
    pub weapon: String,
    pub victim: UserId,
    pub assister: Option<UserId>,
    pub killer: UserId,
    pub tick: u32,
}

impl Death {
    pub fn from_event(event: PlayerDeathEvent, tick: u32) -> Self {
        let assister = if event.assister < (16 * 1024) {
            Some(UserId((event.assister & 255) as u8))
        } else {
            None
        };
        Death {
            assister,
            tick,
            killer: UserId((event.attacker & 255) as u8),
            weapon: event.weapon,
            victim: UserId((event.user_id & 255) as u8),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Round {
    winner: String,
    length: f32,
    end_tick: u32,
}

#[derive(Default, Debug, Serialize)]
pub struct World {
    boundary_min: Vector,
    boundary_max: Vector,
}

#[derive(Default, Debug, Serialize)]
pub struct Analyser {
    pub chat: Vec<ChatMassage>,
    pub users: HashMap<UserId, UserInfo>,
    pub user_spans: Vec<Spawn>,
    pub deaths: Vec<Death>,
    pub rounds: Vec<Round>,
    pub world: World,
}

impl MessageHandler for Analyser {
    fn does_handle(&self, message_type: MessageType) -> bool {
        match message_type {
            MessageType::GameEvent |
            MessageType::UserMessage => true,
            _ => false
        }
    }

    fn handle_message(&mut self, message: Message, tick: u32) {
        match message {
            Message::GameEvent(message) => self.handle_event(message.event, tick),
            Message::UserMessage(message) => self.handle_user_message(message, tick),
            _ => {}
        }
    }
}

impl StringTableEntryHandler for Analyser {
    fn handle_string_entry(&mut self, table: &String, index: usize, entry: &StringTableEntry) {
        match table.as_str() {
            "userinfo" => {
                match &entry.extra_data {
                    Some(data) => {
                        if data.byte_len > 32 {
                            let _ = self.parse_user_info(&entry.text, data.data.clone());
                        }
                    }
                    _ => {} //unreachable!("no userdata {}, {}", table, &entry.text)
                }
            }
            _ => {}
        }
    }
}

impl Analyser {
    pub fn new() -> Self {
        Self::default()
    }

    fn handle_user_message(&mut self, message: UserMessage, tick: u32) {
        match message {
            UserMessage::SayText2(message) => {
                match message.kind {
                    ChatMessageKind::NameChange => self.change_name(message.from, message.text),
                    _ => self.chat.push(ChatMassage {
                        tick,
                        text: message.text,
                        from: message.from,
                        kind: message.kind,
                    })
                }
            }
            _ => {}
        }
    }

    fn change_name(&mut self, from: String, to: String) {
        for (id, user) in self.users.iter_mut() {
            if user.name == from {
                user.name = to;
                return;
            }
        }
    }

    fn handle_event(&mut self, event: GameEvent, tick: u32) {
        match event {
            GameEvent::PlayerDeath(event) => self.deaths.push(Death::from_event(event, tick)),
            GameEvent::PlayerSpawn(event) => self.user_spans.push(Spawn::from_event(event, tick)),
            _ => {}
        }
    }

    fn parse_user_info(&mut self, text: &String, mut data: Stream) -> ReadResult<()> {
        let name: String = data.read_sized(32)?;
        let user_id = UserId((data.read::<u32>()? & 255) as u8);
        let steam_id: String = data.read()?;
        let entity_id: u32 = text.parse().unwrap_or_default();

        if entity_id > 0 && steam_id.len() > 0 {
            let user = UserInfo {
                steam_id,
                user_id,
                name,
                entity_id: EntityId::new(entity_id),
            };

            self.users.insert(user_id, user);
        }
        Ok(())
    }
}