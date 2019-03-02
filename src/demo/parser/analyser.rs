use crate::demo::gameevent_gen::{GameEvent, PlayerDeathEvent};
use crate::demo::message::Message;
use crate::demo::message::usermessage::{SayText2Kind, UserMessage};
use crate::demo::vector::Vector;

#[derive(Debug, Clone)]
pub struct ChatMassage {
    pub kind: SayText2Kind,
    pub from: String,
    pub text: String,
    pub tick: u32,
}

#[derive(Debug, Clone)]
pub struct UserEntityInfo {
    pub name: String,
    pub user_id: u8,
    pub steam_id: String,
    pub entity_id: u32,
}

#[derive(Debug, Clone)]
pub struct UserInfo {
    pub team: String,
    pub entity_info: UserEntityInfo,
}

#[derive(Debug, Clone)]
pub struct Death {
    pub weapon: String,
    pub victim: u8,
    pub assister: Option<u8>,
    pub killer: u8,
    pub tick: u32,
}

#[derive(Debug, Clone)]
pub struct Round {
    winner: String,
    length: f32,
    end_tick: u32,
}

#[derive(Default, Debug)]
pub struct World {
    boundary_min: Vector,
    boundary_max: Vector,
}

#[derive(Default, Debug)]
pub struct Analyser {
    chat: Vec<ChatMassage>,
    users: Vec<UserInfo>,
    deaths: Vec<Death>,
    rounds: Vec<Round>,
    world: World,
}

impl Analyser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_message(&mut self, message: Message, tick: u32) {
        match message {
            Message::GameEvent(message) => self.handle_event(message.event, tick),
            Message::UserMessage(message) => self.handle_user_message(message, tick),
            _ => {}
        }
    }

    fn handle_user_message(&mut self, message: UserMessage, tick: u32) {
        match message {
            UserMessage::SayText2(message) => {
                match message.kind {
                    SayText2Kind::NameChange => self.change_name(message.from, message.text),
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
        for user in &mut self.users {
            if user.entity_info.name == from {
                user.entity_info.name = to;
                return;
            }
        }
    }

    fn handle_event(&mut self, event: GameEvent, tick: u32) {
        match event {
            GameEvent::PlayerDeath(event) => self.handle_death(event, tick),
            _ => {}
        }
    }

    fn handle_death(&mut self, death: PlayerDeathEvent, tick: u32) {
        let assister = if death.assister < (16 * 1024) {
            Some((death.assister & 255) as u8)
        } else {
            None
        };
        let death = Death {
            assister,
            tick,
            killer: (death.attacker & 255) as u8,
            weapon: death.weapon,
            victim: (death.user_id & 255) as u8,
        };
        self.deaths.push(death);
    }
}