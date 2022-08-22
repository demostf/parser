use crate::demo::message::packetentities::{EntityId, PacketEntity};
use crate::demo::message::Message;
use crate::demo::packet::datatable::{ParseSendTable, SendTableName, ServerClass, ServerClassName};
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::analyser::UserInfo;
pub use crate::demo::parser::analyser::{Class, Team, UserId};
use crate::demo::parser::handler::BorrowMessageHandler;
use crate::demo::parser::MessageHandler;
use crate::demo::sendprop::{SendProp, SendPropIdentifier, SendPropName, SendPropValue};
use crate::demo::vector::{Vector, VectorXY};
use crate::{MessageType, ParserState, ReadResult, Stream};
use fnv::FnvHashMap;
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
    pub pitch_angle: f32,
    pub state: PlayerState,
    pub info: Option<UserInfo>,
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
                    pitch_angle: 0.0,
                    state: PlayerState::Alive,
                    info: None,
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
    prop_names: FnvHashMap<SendPropIdentifier, (SendTableName, SendPropName)>,
    class_names: Vec<ServerClassName>, // indexed by ClassId
}

impl MessageHandler for GameStateAnalyser {
    type Output = GameState;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(message_type, MessageType::PacketEntities)
    }

    fn handle_message(&mut self, message: &Message, _tick: u32, parser_state: &ParserState) {
        if let Message::PacketEntities(message) = message {
            for entity in &message.entities {
                self.handle_entity(entity, parser_state);
            }
        }
    }

    fn handle_data_tables(
        &mut self,
        parse_tables: &[ParseSendTable],
        server_classes: &[ServerClass],
        _parser_state: &ParserState,
    ) {
        for table in parse_tables {
            for prop_def in &table.props {
                self.prop_names.insert(
                    prop_def.identifier(),
                    (table.name.clone(), prop_def.name.clone()),
                );
            }
        }

        self.class_names = server_classes
            .iter()
            .map(|class| &class.name)
            .cloned()
            .collect();
    }

    fn handle_string_entry(
        &mut self,
        table: &str,
        index: usize,
        entry: &StringTableEntry,
        _parser_state: &ParserState,
    ) {
        if table == "userinfo" {
            let _ = self.parse_user_info(
                index,
                entry.text.as_ref().map(|s| s.as_ref()),
                entry.extra_data.as_ref().map(|data| data.data.clone()),
            );
        }
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

    pub fn handle_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        let class_name: &str = self
            .class_names
            .get(usize::from(entity.server_class))
            .map(|class_name| class_name.as_str())
            .unwrap_or("");
        match class_name {
            "CTFPlayer" => self.handle_player_entity(entity, parser_state),
            "CTFPlayerResource" => self.handle_player_resource(entity, parser_state),
            "CWorld" => self.handle_world_entity(entity, parser_state),
            _ => {}
        }
    }

    pub fn handle_player_resource(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        for prop in entity.props(parser_state) {
            if let Some((table_name, prop_name)) = self.prop_names.get(&prop.identifier) {
                if let Ok(player_id) = u32::from_str(prop_name.as_str()) {
                    let entity_id = EntityId::from(player_id);
                    if let Some(player) = self
                        .state
                        .players
                        .iter_mut()
                        .find(|player| player.entity == entity_id)
                    {
                        match table_name.as_str() {
                            "m_iTeam" => {
                                player.team =
                                    Team::new(i64::try_from(&prop.value).unwrap_or_default())
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
    }

    pub fn handle_player_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        let player = self.state.get_or_create_player(entity.entity_index);

        const HEALTH_PROP: SendPropIdentifier =
            SendPropIdentifier::new("DT_BasePlayer", "m_iHealth");
        const MAX_HEALTH_PROP: SendPropIdentifier =
            SendPropIdentifier::new("DT_BasePlayer", "m_iMaxHealth");
        const LIFE_STATE_PROP: SendPropIdentifier =
            SendPropIdentifier::new("DT_BasePlayer", "m_lifeState");

        const LOCAL_ORIGIN: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFLocalPlayerExclusive", "m_vecOrigin");
        const NON_LOCAL_ORIGIN: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFNonLocalPlayerExclusive", "m_vecOrigin");
        const LOCAL_ORIGIN_Z: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFLocalPlayerExclusive", "m_vecOrigin[2]");
        const NON_LOCAL_ORIGIN_Z: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFNonLocalPlayerExclusive", "m_vecOrigin[2]");
        const LOCAL_EYE_ANGLES: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFLocalPlayerExclusive", "m_angEyeAngles[1]");
        const NON_LOCAL_EYE_ANGLES: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFNonLocalPlayerExclusive", "m_angEyeAngles[1]");
        const LOCAL_PITCH_ANGLES: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFLocalPlayerExclusive", "m_angEyeAngles[0]");
        const NON_LOCAL_PITCH_ANGLES: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFNonLocalPlayerExclusive", "m_angEyeAngles[0]");

        for prop in entity.props(parser_state) {
            match prop.identifier {
                HEALTH_PROP => {
                    player.health = i64::try_from(&prop.value).unwrap_or_default() as u16
                }
                MAX_HEALTH_PROP => {
                    player.max_health = i64::try_from(&prop.value).unwrap_or_default() as u16
                }
                LIFE_STATE_PROP => {
                    player.state = PlayerState::new(i64::try_from(&prop.value).unwrap_or_default())
                }
                LOCAL_ORIGIN | NON_LOCAL_ORIGIN => {
                    let pos_xy = VectorXY::try_from(&prop.value).unwrap_or_default();
                    player.position.x = pos_xy.x;
                    player.position.y = pos_xy.y;
                }
                LOCAL_ORIGIN_Z | NON_LOCAL_ORIGIN_Z => {
                    player.position.z = f32::try_from(&prop.value).unwrap_or_default()
                }
                LOCAL_EYE_ANGLES | NON_LOCAL_EYE_ANGLES => {
                    player.view_angle = f32::try_from(&prop.value).unwrap_or_default()
                }
                LOCAL_PITCH_ANGLES | NON_LOCAL_PITCH_ANGLES => {
                    player.pitch_angle = f32::try_from(&prop.value).unwrap_or_default()
                }
                _ => {}
            }
        }
    }

    pub fn handle_world_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
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
            entity.get_prop_by_name("DT_WORLD", "m_WorldMins", parser_state),
            entity.get_prop_by_name("DT_WORLD", "m_WorldMaxs", parser_state),
        ) {
            self.state.world = Some(World {
                boundary_min,
                boundary_max,
            })
        }
    }

    fn parse_user_info(&mut self, index: usize, text: Option<&str>, data: Option<Stream>) -> ReadResult<()> {
        if let Some(user_info) = crate::demo::data::UserInfo::parse_from_string_table(index as u16, text, data)? {
            let id = user_info.entity_id;
            self.state.get_or_create_player(id).info = Some(user_info.into());
        }

        Ok(())
    }
}
