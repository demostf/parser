use crate::demo::data::DemoTick;
use crate::demo::gameevent_gen::{ObjectDestroyedEvent, PlayerDeathEvent};
use crate::demo::gamevent::GameEvent;
use crate::demo::message::gameevent::GameEventMessage;
use crate::demo::message::packetentities::{EntityId, PacketEntity, UpdateType};
use crate::demo::message::Message;
use crate::demo::packet::datatable::{ParseSendTable, ServerClass, ServerClassName};
use crate::demo::packet::message::MessagePacketMeta;
use crate::demo::packet::stringtable::StringTableEntry;
use crate::demo::parser::analyser::UserInfo;
pub use crate::demo::parser::analyser::{Class, Team, UserId};
use crate::demo::parser::handler::BorrowMessageHandler;
use crate::demo::parser::MessageHandler;
use crate::demo::sendprop::{SendProp, SendPropIdentifier, SendPropValue};
use crate::demo::vector::{Vector, VectorXY};
use crate::{MessageType, ParserState, ReadResult, Stream};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
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

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::Alive
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
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
    pub charge: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Sentry {
    pub entity: EntityId,
    pub builder: UserId,
    pub position: Vector,
    pub level: u8,
    pub max_health: u16,
    pub health: u16,
    pub building: bool,
    pub sapped: bool,
    pub team: Team,
    pub angle: f32,
    pub player_controlled: bool,
    pub auto_aim_target: UserId,
    pub shells: u16,
    pub rockets: u16,
    pub is_mini: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Dispenser {
    pub entity: EntityId,
    pub builder: UserId,
    pub position: Vector,
    pub level: u8,
    pub max_health: u16,
    pub health: u16,
    pub building: bool,
    pub sapped: bool,
    pub team: Team,
    pub angle: f32,
    pub healing: Vec<UserId>,
    pub metal: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Teleporter {
    pub entity: EntityId,
    pub builder: UserId,
    pub position: Vector,
    pub level: u8,
    pub max_health: u16,
    pub health: u16,
    pub building: bool,
    pub sapped: bool,
    pub team: Team,
    pub angle: f32,
    pub is_entrance: bool,
    pub other_end: EntityId,
    pub recharge_time: f32,
    pub recharge_duration: f32,
    pub times_used: u16,
    pub yaw_to_exit: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Building {
    Sentry(Sentry),
    Dispenser(Dispenser),
    Teleporter(Teleporter),
}

impl Building {
    pub fn new(entity_id: EntityId, class: BuildingClass) -> Building {
        match class {
            BuildingClass::Sentry => Building::Sentry(Sentry {
                entity: entity_id,
                ..Sentry::default()
            }),
            BuildingClass::Dispenser => Building::Dispenser(Dispenser {
                entity: entity_id,
                ..Dispenser::default()
            }),
            BuildingClass::Teleporter => Building::Teleporter(Teleporter {
                entity: entity_id,
                ..Teleporter::default()
            }),
        }
    }

    pub fn entity_id(&self) -> EntityId {
        match self {
            Building::Sentry(Sentry { entity, .. })
            | Building::Dispenser(Dispenser { entity, .. })
            | Building::Teleporter(Teleporter { entity, .. }) => *entity,
        }
    }

    pub fn level(&self) -> u8 {
        match self {
            Building::Sentry(Sentry { level, .. })
            | Building::Dispenser(Dispenser { level, .. })
            | Building::Teleporter(Teleporter { level, .. }) => *level,
        }
    }

    pub fn position(&self) -> Vector {
        match self {
            Building::Sentry(Sentry { position, .. })
            | Building::Dispenser(Dispenser { position, .. })
            | Building::Teleporter(Teleporter { position, .. }) => *position,
        }
    }

    pub fn builder(&self) -> UserId {
        match self {
            Building::Sentry(Sentry { builder, .. })
            | Building::Dispenser(Dispenser { builder, .. })
            | Building::Teleporter(Teleporter { builder, .. }) => *builder,
        }
    }

    pub fn angle(&self) -> f32 {
        match self {
            Building::Sentry(Sentry { angle, .. })
            | Building::Dispenser(Dispenser { angle, .. })
            | Building::Teleporter(Teleporter { angle, .. }) => *angle,
        }
    }

    pub fn max_health(&self) -> u16 {
        match self {
            Building::Sentry(Sentry { max_health, .. })
            | Building::Dispenser(Dispenser { max_health, .. })
            | Building::Teleporter(Teleporter { max_health, .. }) => *max_health,
        }
    }

    pub fn health(&self) -> u16 {
        match self {
            Building::Sentry(Sentry { health, .. })
            | Building::Dispenser(Dispenser { health, .. })
            | Building::Teleporter(Teleporter { health, .. }) => *health,
        }
    }

    pub fn sapped(&self) -> bool {
        match self {
            Building::Sentry(Sentry { sapped, .. })
            | Building::Dispenser(Dispenser { sapped, .. })
            | Building::Teleporter(Teleporter { sapped, .. }) => *sapped,
        }
    }

    pub fn team(&self) -> Team {
        match self {
            Building::Sentry(Sentry { team, .. })
            | Building::Dispenser(Dispenser { team, .. })
            | Building::Teleporter(Teleporter { team, .. }) => *team,
        }
    }

    pub fn class(&self) -> BuildingClass {
        match self {
            Building::Sentry(_) => BuildingClass::Sentry,
            Building::Dispenser(_) => BuildingClass::Sentry,
            Building::Teleporter(_) => BuildingClass::Teleporter,
        }
    }
}

pub enum BuildingClass {
    Sentry,
    Dispenser,
    Teleporter,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct World {
    pub boundary_min: Vector,
    pub boundary_max: Vector,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Kill {
    pub attacker_id: u16,
    pub assister_id: u16,
    pub victim_id: u16,
    pub weapon: String,
    pub tick: DemoTick,
}

impl Kill {
    fn new(tick: DemoTick, death: &PlayerDeathEvent) -> Self {
        Kill {
            attacker_id: death.attacker,
            assister_id: death.assister,
            victim_id: death.user_id,
            weapon: death.weapon.to_string(),
            tick,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    pub players: Vec<Player>,
    pub buildings: BTreeMap<EntityId, Building>,
    pub world: Option<World>,
    pub kills: Vec<Kill>,
    pub tick: DemoTick,
}

impl GameState {
    pub fn get_or_create_player(&mut self, entity_id: EntityId) -> &mut Player {
        let index = match self
            .players
            .iter()
            .enumerate()
            .find(|(_index, player)| player.entity == entity_id)
            .map(|(index, _)| index)
        {
            Some(index) => index,
            None => {
                let index = self.players.len();
                self.players.push(Player {
                    entity: entity_id,
                    ..Player::default()
                });
                index
            }
        };

        &mut self.players[index]
    }
    pub fn get_or_create_building(
        &mut self,
        entity_id: EntityId,
        class: BuildingClass,
    ) -> &mut Building {
        self.buildings
            .entry(entity_id)
            .or_insert_with(|| Building::new(entity_id, class))
    }

    pub fn remove_building(&mut self, entity_id: EntityId) {
        self.buildings.remove(&entity_id);
    }
}

#[derive(Default, Debug)]
pub struct GameStateAnalyser {
    pub state: GameState,
    tick: DemoTick,
    class_names: Vec<ServerClassName>, // indexed by ClassId
}

impl MessageHandler for GameStateAnalyser {
    type Output = GameState;

    fn does_handle(message_type: MessageType) -> bool {
        matches!(
            message_type,
            MessageType::PacketEntities | MessageType::GameEvent
        )
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, parser_state: &ParserState) {
        match message {
            Message::PacketEntities(message) => {
                for entity in &message.entities {
                    self.handle_entity(entity, parser_state);
                }
            }
            Message::GameEvent(GameEventMessage { event, .. }) => match event {
                GameEvent::PlayerDeath(death) => {
                    self.state.kills.push(Kill::new(self.tick, death.as_ref()))
                }
                GameEvent::RoundStart(_) => {
                    self.state.buildings.clear();
                }
                GameEvent::TeamPlayRoundStart(_) => {
                    self.state.buildings.clear();
                }
                GameEvent::ObjectDestroyed(ObjectDestroyedEvent { index, .. }) => {
                    self.state.remove_building((*index as u32).into());
                }
                _ => {}
            },
            _ => {}
        }
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

    fn handle_data_tables(
        &mut self,
        _parse_tables: &[ParseSendTable],
        server_classes: &[ServerClass],
        _parser_state: &ParserState,
    ) {
        self.class_names = server_classes
            .iter()
            .map(|class| &class.name)
            .cloned()
            .collect();
    }

    fn handle_packet_meta(
        &mut self,
        tick: DemoTick,
        _meta: &MessagePacketMeta,
        _parser_state: &ParserState,
    ) {
        self.state.tick = tick;
        self.tick = tick;
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
            "CObjectSentrygun" => self.handle_sentry_entity(entity, parser_state),
            "CObjectDispenser" => self.handle_dispenser_entity(entity, parser_state),
            "CObjectTeleporter" => self.handle_teleporter_entity(entity, parser_state),
            _ => {}
        }
    }

    pub fn handle_player_resource(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        for prop in entity.props(parser_state) {
            if let Some((table_name, prop_name)) = prop.identifier.names() {
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
                            "m_iChargeLevel" => {
                                player.charge = i64::try_from(&prop.value).unwrap_or_default() as u8
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

    pub fn handle_sentry_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        const ANGLE: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFNonLocalPlayerExclusive", "m_angEyeAngles[1]");
        const MINI: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseObject", "m_bMiniBuilding");
        const CONTROLLED: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectSentrygun", "m_bPlayerControlled");
        const TARGET: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectSentrygun", "m_hAutoAimTarget");
        const SHELLS: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectSentrygun", "m_iAmmoShells");
        const ROCKETS: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectSentrygun", "m_iAmmoRockets");

        if entity.update_type == UpdateType::Delete {
            self.state.remove_building(entity.entity_index);
            return;
        }

        self.handle_building(entity, parser_state, BuildingClass::Sentry);

        let building = self
            .state
            .get_or_create_building(entity.entity_index, BuildingClass::Sentry);

        match building {
            Building::Sentry(sentry) => {
                for prop in entity.props(parser_state) {
                    match prop.identifier {
                        ANGLE => sentry.angle = f32::try_from(&prop.value).unwrap_or_default(),
                        MINI => sentry.is_mini = i64::try_from(&prop.value).unwrap_or_default() > 0,
                        CONTROLLED => {
                            sentry.player_controlled =
                                i64::try_from(&prop.value).unwrap_or_default() > 0
                        }
                        TARGET => {
                            sentry.auto_aim_target =
                                UserId::from(i64::try_from(&prop.value).unwrap_or_default() as u16)
                        }
                        SHELLS => {
                            sentry.shells = i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        ROCKETS => {
                            sentry.rockets = i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_teleporter_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        const RECHARGE_TIME: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectTeleporter", "m_flRechargeTime");
        const RECHARGE_DURATION: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectTeleporter", "m_flCurrentRechargeDuration");
        const TIMES_USED: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectTeleporter", "m_iTimesUsed");
        const OTHER_END: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectTeleporter", "m_bMatchBuilding");
        const YAW_TO_EXIT: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectTeleporter", "m_flYawToExit");
        const IS_ENTRANCE: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseObject", "m_iObjectMode");

        if entity.update_type == UpdateType::Delete {
            self.state.remove_building(entity.entity_index);
            return;
        }

        self.handle_building(entity, parser_state, BuildingClass::Teleporter);

        let building = self
            .state
            .get_or_create_building(entity.entity_index, BuildingClass::Teleporter);

        match building {
            Building::Teleporter(teleporter) => {
                for prop in entity.props(parser_state) {
                    match prop.identifier {
                        RECHARGE_TIME => {
                            teleporter.recharge_time =
                                f32::try_from(&prop.value).unwrap_or_default()
                        }
                        RECHARGE_DURATION => {
                            teleporter.recharge_duration =
                                f32::try_from(&prop.value).unwrap_or_default()
                        }
                        TIMES_USED => {
                            teleporter.times_used =
                                i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        OTHER_END => {
                            teleporter.other_end = EntityId::from(
                                i64::try_from(&prop.value).unwrap_or_default() as u32,
                            )
                        }
                        YAW_TO_EXIT => {
                            teleporter.yaw_to_exit = f32::try_from(&prop.value).unwrap_or_default()
                        }
                        IS_ENTRANCE => {
                            teleporter.is_entrance =
                                i64::try_from(&prop.value).unwrap_or_default() == 0
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    pub fn handle_dispenser_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        const AMMO: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectDispenser", "m_iAmmoMetal");
        const HEALING: SendPropIdentifier =
            SendPropIdentifier::new("DT_ObjectDispenser", "healing_array");

        if entity.update_type == UpdateType::Delete {
            self.state.remove_building(entity.entity_index);
            return;
        }

        self.handle_building(entity, parser_state, BuildingClass::Dispenser);

        let building = self
            .state
            .get_or_create_building(entity.entity_index, BuildingClass::Dispenser);

        match building {
            Building::Dispenser(dispenser) => {
                for prop in entity.props(parser_state) {
                    match prop.identifier {
                        AMMO => {
                            dispenser.metal = i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        HEALING => {
                            let values = match &prop.value {
                                SendPropValue::Array(vec) => vec.as_slice(),
                                _ => Default::default(),
                            };

                            dispenser.healing = values
                                .iter()
                                .map(|val| {
                                    UserId::from(i64::try_from(val).unwrap_or_default() as u16)
                                })
                                .collect()
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    fn handle_building(
        &mut self,
        entity: &PacketEntity,
        parser_state: &ParserState,
        class: BuildingClass,
    ) {
        let building = self
            .state
            .get_or_create_building(entity.entity_index, class);

        const LOCAL_ORIGIN: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseEntity", "m_vecOrigin");
        const TEAM: SendPropIdentifier = SendPropIdentifier::new("DT_BaseEntity", "m_iTeamNum");
        const ANGLE: SendPropIdentifier = SendPropIdentifier::new("DT_BaseEntity", "m_angRotation");
        const SAPPED: SendPropIdentifier = SendPropIdentifier::new("DT_BaseObject", "m_bHasSapper");
        const BUILDING: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseObject", "m_bBuilding");
        const LEVEL: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseObject", "m_iUpgradeLevel");
        const BUILDER: SendPropIdentifier = SendPropIdentifier::new("DT_BaseObject", "m_hBuilder");
        const MAX_HEALTH: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseObject", "m_iMaxHealth");
        const HEALTH: SendPropIdentifier = SendPropIdentifier::new("DT_BaseObject", "m_iHealth");

        match building {
            Building::Sentry(Sentry {
                position,
                team,
                angle,
                sapped,
                builder,
                level,
                building,
                max_health,
                health,
                ..
            })
            | Building::Dispenser(Dispenser {
                position,
                team,
                angle,
                sapped,
                builder,
                level,
                building,
                max_health,
                health,
                ..
            })
            | Building::Teleporter(Teleporter {
                position,
                team,
                angle,
                sapped,
                builder,
                level,
                building,
                max_health,
                health,
                ..
            }) => {
                for prop in entity.props(parser_state) {
                    match prop.identifier {
                        LOCAL_ORIGIN => {
                            *position = Vector::try_from(&prop.value).unwrap_or_default()
                        }
                        TEAM => *team = Team::new(i64::try_from(&prop.value).unwrap_or_default()),
                        ANGLE => *angle = f32::try_from(&prop.value).unwrap_or_default(),
                        SAPPED => *sapped = i64::try_from(&prop.value).unwrap_or_default() > 0,
                        BUILDING => *building = i64::try_from(&prop.value).unwrap_or_default() > 0,
                        LEVEL => *level = i64::try_from(&prop.value).unwrap_or_default() as u8,
                        BUILDER => {
                            *builder =
                                UserId::from(i64::try_from(&prop.value).unwrap_or_default() as u16)
                        }
                        MAX_HEALTH => {
                            *max_health = i64::try_from(&prop.value).unwrap_or_default() as u16
                        }
                        HEALTH => *health = i64::try_from(&prop.value).unwrap_or_default() as u16,
                        _ => {}
                    }
                }
            }
        }
    }

    fn parse_user_info(
        &mut self,
        index: usize,
        text: Option<&str>,
        data: Option<Stream>,
    ) -> ReadResult<()> {
        if let Some(user_info) =
            crate::demo::data::UserInfo::parse_from_string_table(index as u16, text, data)?
        {
            let id = user_info.entity_id;
            self.state.get_or_create_player(id).info = Some(user_info.into());
        }

        Ok(())
    }
}
