pub use crate::demo::data::game_state::{
    Building, BuildingClass, Dispenser, GameState, Kill, PlayerState, Sentry, Teleporter, World,
};
use crate::demo::data::game_state::{Handle, PipeType, Projectile, ProjectileType};
use crate::demo::data::DemoTick;
use crate::demo::gameevent_gen::ObjectDestroyedEvent;
use crate::demo::gamevent::GameEvent;
use crate::demo::message::gameevent::GameEventMessage;
use crate::demo::message::packetentities::{EntityId, PacketEntity, UpdateType};
use crate::demo::message::Message;
use crate::demo::packet::datatable::{ParseSendTable, ServerClass, ServerClassName};
use crate::demo::packet::message::MessagePacketMeta;
use crate::demo::packet::stringtable::StringTableEntry;
pub use crate::demo::parser::analyser::{Class, Team, UserId};
use crate::demo::parser::handler::BorrowMessageHandler;
use crate::demo::parser::MessageHandler;
use crate::demo::sendprop::{SendProp, SendPropIdentifier, SendPropValue};
use crate::demo::vector::{Vector, VectorXY};
use crate::{MessageType, ParserState, ReadResult, Stream};
use std::convert::TryFrom;
use std::str::FromStr;

pub struct CachedEntities {}

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
            MessageType::PacketEntities | MessageType::GameEvent | MessageType::ServerInfo
        )
    }

    fn handle_message(&mut self, message: &Message, _tick: DemoTick, parser_state: &ParserState) {
        match message {
            Message::PacketEntities(message) => {
                for entity in &message.entities {
                    self.handle_entity(entity, parser_state);
                }
                for id in &message.removed_entities {
                    self.state.projectile_destroy(*id);
                    self.state.remove_building(*id);
                }
            }
            Message::ServerInfo(message) => {
                self.state.interval_per_tick = message.interval_per_tick
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

    fn into_output(mut self, state: &ParserState) -> Self::Output {
        self.state.server_classes = state.server_classes.clone();
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
        const OUTER: SendPropIdentifier =
            SendPropIdentifier::new("DT_AttributeContainer", "m_hOuter");

        let Some(class_name) = self.class_names.get(usize::from(entity.server_class)) else {
            return;
        };

        for prop in &entity.props {
            if prop.identifier == OUTER {
                let outer = i64::try_from(&prop.value).unwrap_or_default();
                self.state
                    .outer_map
                    .insert(Handle(outer), entity.entity_index);
            }
        }

        match class_name.as_str() {
            "CTFPlayer" => self.handle_player_entity(entity, parser_state),
            "CTFPlayerResource" => self.handle_player_resource(entity, parser_state),
            "CWorld" => self.handle_world_entity(entity, parser_state),
            "CObjectSentrygun" => self.handle_sentry_entity(entity, parser_state),
            "CObjectDispenser" => self.handle_dispenser_entity(entity, parser_state),
            "CObjectTeleporter" => self.handle_teleporter_entity(entity, parser_state),
            _ if class_name.starts_with("CTFProjectile_")
                || class_name.as_str() == "CTFGrenadePipebombProjectile" =>
            {
                self.handle_projectile_entity(entity, parser_state)
            }
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
                            "m_iPing" => {
                                player.ping = i64::try_from(&prop.value).unwrap_or_default() as u16
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

        const SIMTIME_PROP: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseEntity", "m_flSimulationTime");
        const PROP_BB_MAX: SendPropIdentifier =
            SendPropIdentifier::new("DT_CollisionProperty", "m_vecMaxsPreScaled");

        const WEAPON_0: SendPropIdentifier = SendPropIdentifier::new("m_hMyWeapons", "000");
        const WEAPON_1: SendPropIdentifier = SendPropIdentifier::new("m_hMyWeapons", "001");
        const WEAPON_2: SendPropIdentifier = SendPropIdentifier::new("m_hMyWeapons", "002");

        player.in_pvs = entity.in_pvs;

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
                SIMTIME_PROP => {
                    player.simtime = i64::try_from(&prop.value).unwrap_or_default() as u16
                }
                PROP_BB_MAX => {
                    let max = Vector::try_from(&prop.value).unwrap_or_default();
                    player.bounds.max = max;
                }
                WEAPON_0 => {
                    let handle = Handle(i64::try_from(&prop.value).unwrap_or_default());
                    player.weapons[0] = handle;
                }
                WEAPON_1 => {
                    let handle = Handle(i64::try_from(&prop.value).unwrap_or_default());
                    player.weapons[1] = handle;
                }
                WEAPON_2 => {
                    let handle = Handle(i64::try_from(&prop.value).unwrap_or_default());
                    player.weapons[2] = handle;
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

        if let Building::Sentry(sentry) = building {
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
                    SHELLS => sentry.shells = i64::try_from(&prop.value).unwrap_or_default() as u16,
                    ROCKETS => {
                        sentry.rockets = i64::try_from(&prop.value).unwrap_or_default() as u16
                    }
                    _ => {}
                }
            }
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

        if let Building::Teleporter(teleporter) = building {
            for prop in entity.props(parser_state) {
                match prop.identifier {
                    RECHARGE_TIME => {
                        teleporter.recharge_time = f32::try_from(&prop.value).unwrap_or_default()
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
                        teleporter.other_end =
                            EntityId::from(i64::try_from(&prop.value).unwrap_or_default() as u32)
                    }
                    YAW_TO_EXIT => {
                        teleporter.yaw_to_exit = f32::try_from(&prop.value).unwrap_or_default()
                    }
                    IS_ENTRANCE => {
                        teleporter.is_entrance = i64::try_from(&prop.value).unwrap_or_default() == 0
                    }
                    _ => {}
                }
            }
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

        if let Building::Dispenser(dispenser) = building {
            for prop in entity.props(parser_state) {
                match prop.identifier {
                    AMMO => dispenser.metal = i64::try_from(&prop.value).unwrap_or_default() as u16,
                    HEALING => {
                        let values = match &prop.value {
                            SendPropValue::Array(vec) => vec.as_slice(),
                            _ => Default::default(),
                        };

                        dispenser.healing = values
                            .iter()
                            .map(|val| UserId::from(i64::try_from(val).unwrap_or_default() as u16))
                            .collect()
                    }
                    _ => {}
                }
            }
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

    pub fn handle_projectile_entity(&mut self, entity: &PacketEntity, parser_state: &ParserState) {
        let Some(class_name) = self.class_names.get(usize::from(entity.server_class)) else {
            return;
        };

        const ROCKET_ORIGIN: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFBaseRocket", "m_vecOrigin"); // rockets, arrows, more?
        const GRENADE_ORIGIN: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFWeaponBaseGrenadeProj", "m_vecOrigin");
        // todo: flares?
        const TEAM: SendPropIdentifier = SendPropIdentifier::new("DT_BaseEntity", "m_iTeamNum");
        const INITIAL_SPEED: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFBaseRocket", "m_vInitialVelocity");
        const LAUNCHER: SendPropIdentifier =
            SendPropIdentifier::new("DT_BaseProjectile", "m_hOriginalLauncher");
        const PIPE_TYPE: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFProjectile_Pipebomb", "m_iType");
        const ROCKET_ROTATION: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFBaseRocket", "m_angRotation");
        const GRENADE_ROTATION: SendPropIdentifier =
            SendPropIdentifier::new("DT_TFWeaponBaseGrenadeProj", "m_angRotation");

        if entity.update_type == UpdateType::Delete {
            self.state.projectile_destroy(entity.entity_index);
            return;
        }

        let projectile = self
            .state
            .projectiles
            .entry(entity.entity_index)
            .or_insert_with(|| {
                Projectile::new(entity.entity_index, entity.server_class, class_name)
            });

        // todo: bounds for grenades

        for prop in entity.props(parser_state) {
            match prop.identifier {
                ROCKET_ORIGIN | GRENADE_ORIGIN => {
                    let pos = Vector::try_from(&prop.value).unwrap_or_default();
                    projectile.position = pos
                }
                TEAM => {
                    let team = Team::new(i64::try_from(&prop.value).unwrap_or_default());
                    projectile.team = team;
                }
                INITIAL_SPEED => {
                    let speed = Vector::try_from(&prop.value).unwrap_or_default();
                    projectile.initial_speed = speed;
                }
                LAUNCHER => {
                    let launcher = Handle(i64::try_from(&prop.value).unwrap_or_default());
                    projectile.launcher = launcher;
                }
                PIPE_TYPE => {
                    let pipe_type = PipeType::new(i64::try_from(&prop.value).unwrap_or_default());
                    if let Some(class_name) = self.class_names.get(usize::from(entity.server_class))
                    {
                        let ty = ProjectileType::new(class_name, Some(pipe_type));
                        projectile.ty = ty;
                    }
                }
                ROCKET_ROTATION | GRENADE_ROTATION => {
                    let rotation = Vector::try_from(&prop.value).unwrap_or_default();
                    projectile.rotation = rotation;
                }
                _ => {}
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
