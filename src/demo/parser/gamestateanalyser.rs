use crate::demo::message::packetentities::{EntityId, PacketEntity};
use crate::demo::message::Message;
use crate::demo::packet::datatable::{ParseSendTable, ServerClass, ServerClassName};
pub use crate::demo::parser::analyser::{Class, Team, UserId};
use crate::demo::parser::handler::BorrowMessageHandler;
use crate::demo::parser::MessageHandler;
use crate::demo::sendprop::{SendProp, SendPropValue};
use crate::demo::vector::{Vector, VectorXY};
use crate::{MessageType, ParserState};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct CachedEntities {}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum PlayerState {
    Alive = 0,
    Dying = 1,
    Death = 2,
    Respawnable = 3,
}

impl PlayerState {
    pub fn new(number: i64) -> Self {
        match number {
            1 => PlayerState::Dying,
            2 => PlayerState::Death,
            3 => PlayerState::Respawnable,
            _ => PlayerState::Alive,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Player {
    entity: EntityId,
    pub position: Vector,
    pub health: u16,
    pub max_health: u16,
    pub class: Class,
    pub team: Team,
    pub view_angle: f32,
    pub state: PlayerState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Building {
    Sentry {
        builder: UserId,
        position: Vector,
        level: u8,
        max_health: u16,
        health: u16,
        building: bool,
        sapped: bool,
        team: Team,
        angle: f32,
        player_controller: bool,
        auto_aim_target: UserId,
        shells: u16,
        rockets: u16,
        is_mini: bool,
    },
    Dispenser {
        builder: UserId,
        position: Vector,
        level: u8,
        max_health: u16,
        health: u16,
        building: bool,
        sapped: bool,
        team: Team,
        angle: f32,
        healing: Vec<UserId>,
        metal: u16,
    },
    Teleporter {
        builder: UserId,
        position: Vector,
        level: u8,
        max_health: u16,
        health: u16,
        building: bool,
        sapped: bool,
        team: Team,
        angle: f32,
        is_entrance: bool,
        other_end: EntityId,
        recharge_time: f32,
        recharge_duration: f32,
        times_used: u16,
        yaw_to_exit: f32,
    },
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct World {
    pub boundary_min: Vector,
    pub boundary_max: Vector,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    pub players: Vec<Player>,
    pub buildings: Vec<Building>,
    pub world: Option<World>,
}

impl GameState {
    pub fn get_or_create_player(&mut self, entity_id: EntityId) -> &mut Player {
        let index = match self
            .players
            .iter_mut()
            .enumerate()
            .find(|(_index, player)| player.entity == entity_id)
            .map(|(index, _)| index)
        {
            Some(index) => index,
            None => {
                let player = Player {
                    entity: entity_id,
                    position: Vector::default(),
                    health: 0,
                    max_health: 0,
                    class: Class::Other,
                    team: Team::Other,
                    view_angle: 0.0,
                    state: PlayerState::Alive,
                };

                let index = self.players.len();
                self.players.push(player);
                index
            }
        };

        &mut self.players[index]
    }
}

#[derive(Default, Debug)]
pub struct GameStateAnalyser {
    pub state: GameState,
    class_names: Vec<ServerClassName>, // indexed by ClassId
}

impl MessageHandler for GameStateAnalyser {
    type Output = GameState;

    fn does_handle(message_type: MessageType) -> bool {
        match message_type {
            MessageType::PacketEntities => true,
            _ => false,
        }
    }

    fn handle_message(&mut self, message: &Message, _tick: u32) {
        match message {
            Message::PacketEntities(message) => {
                for entity in &message.entities {
                    self.handle_entity(entity);
                }
            }
            _ => {}
        }
    }

    fn handle_data_tables(&mut self, _tables: &[ParseSendTable], server_classes: &[ServerClass]) {
        self.class_names = server_classes
            .iter()
            .map(|class| &class.name)
            .cloned()
            .collect();
    }

    fn into_output(self, _state: &ParserState) -> Self::Output {
        self.state
    }
}

impl BorrowMessageHandler for GameStateAnalyser {
    fn borrow_output(&self, _state: &ParserState) -> &Self::Output {
        &self.state
    }
}

impl GameStateAnalyser {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn handle_entity(&mut self, entity: &PacketEntity) {
        let class_name: &str = self
            .class_names
            .get(usize::from(entity.server_class))
            .map(|class_name| class_name.as_str())
            .unwrap_or("");
        match class_name {
            "CTFPlayer" => self.handle_player_entity(entity),
            "CTFPlayerResource" => self.handle_player_resource(entity),
            "CWorld" => self.handle_world_entity(entity),
            _ => {}
        }
    }

    pub fn handle_player_resource(&mut self, entity: &PacketEntity) {
        for prop in &entity.props {
            if let Ok(player_id) = u32::from_str(prop.definition.name.as_str()) {
                let entity_id = EntityId::from(player_id);
                if let Some(player) = self
                    .state
                    .players
                    .iter_mut()
                    .find(|player| player.entity == entity_id)
                {
                    match prop.definition.owner_table.as_str() {
                        "m_iTeam" => {
                            player.team = Team::new(i64::try_from(&prop.value).unwrap_or_default())
                        }
                        "m_iMaxHealth" => {
                            player.max_health =
                                i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        "m_iPlayerClass" => {
                            player.class =
                                Class::new(i64::try_from(&prop.value).unwrap_or_default())
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn handle_player_entity(&mut self, entity: &PacketEntity) {
        let player = self.state.get_or_create_player(entity.entity_index);

        for prop in &entity.props {
            match prop.definition.owner_table.as_str() {
                "DT_BasePlayer" => match prop.definition.name.as_str() {
                    "m_iHealth" => {
                        player.health = i64::try_from(&prop.value).unwrap_or_default() as u16
                    }
                    "m_iMaxHealth" => {
                        player.max_health = i64::try_from(&prop.value).unwrap_or_default() as u16
                    }
                    "m_lifeState" => {
                        player.state =
                            PlayerState::new(i64::try_from(&prop.value).unwrap_or_default())
                    }
                    _ => {}
                },
                "DT_TFLocalPlayerExclusive" | "DT_TFNonLocalPlayerExclusive" => {
                    match prop.definition.name.as_str() {
                        "m_vecOrigin" => {
                            let pos_xy = VectorXY::try_from(&prop.value).unwrap_or_default();
                            player.position.x = pos_xy.x;
                            player.position.y = pos_xy.y;
                        }
                        "m_vecOrigin[2]" => {
                            player.position.z = f32::try_from(&prop.value).unwrap_or_default()
                        }
                        "m_angEyeAngles[1]" => {
                            player.view_angle = f32::try_from(&prop.value).unwrap_or_default()
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }

    pub fn handle_world_entity(&mut self, entity: &PacketEntity) {
        if let (
            Some(SendProp {
                value: SendPropValue::Vector(boundary_min),
                ..
            }),
            Some(SendProp {
                value: SendPropValue::Vector(boundary_max),
                ..
            }),
        ) = (
            entity.get_prop_by_name("DT_WORLD", "m_WorldMins"),
            entity.get_prop_by_name("DT_WORLD", "m_WorldMaxs"),
        ) {
            self.state.world = Some(World {
                boundary_min: boundary_min.clone(),
                boundary_max: boundary_max.clone(),
            })
        }
    }
}
