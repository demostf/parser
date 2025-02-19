use crate::demo::data::DemoTick;
use crate::demo::gameevent_gen::PlayerDeathEvent;
use crate::demo::gamevent::GameEvent;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{ClassId, ServerClass, ServerClassName};
use crate::demo::parser::analyser::{Class, Team, UserId, UserInfo};
use crate::demo::vector::Vector;
use parse_display::Display;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};

#[derive(Default, Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash, Display)]
pub struct Handle(pub i64);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum PlayerState {
    #[default]
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

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Box {
    pub min: Vector,
    pub max: Vector,
}

impl Box {
    pub fn new(min: Vector, max: Vector) -> Box {
        Box { min, max }
    }

    pub fn contains(&self, point: Vector) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Player {
    pub entity: EntityId,
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
    pub simtime: u16,
    pub ping: u16,
    pub in_pvs: bool,
    pub bounds: Box,
    pub weapons: [Handle; 3],
}

pub const PLAYER_BOX_DEFAULT: Box = Box {
    min: Vector {
        x: -24.0,
        y: -24.0,
        z: 0.0,
    },
    max: Vector {
        x: 24.0,
        y: 24.0,
        z: 82.0,
    },
};

impl Player {
    pub fn new(entity: EntityId) -> Player {
        Player {
            entity,
            bounds: PLAYER_BOX_DEFAULT,
            ..Player::default()
        }
    }

    pub fn collides(&self, projectile: &Projectile, time_per_tick: f32) -> bool {
        let current_position = projectile.position;
        let next_position = projectile.position + (projectile.initial_speed * time_per_tick);
        match projectile.bounds {
            Some(_) => todo!(),
            None => {
                self.bounds.contains(current_position - self.position)
                    || self.bounds.contains(next_position - self.position)
            }
        }
    }
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Projectile {
    pub id: EntityId,
    pub team: Team,
    pub class: ClassId,
    pub position: Vector,
    pub rotation: Vector,
    pub initial_speed: Vector,
    pub bounds: Option<Box>,
    pub launcher: Handle,
    pub ty: ProjectileType,
}

impl Projectile {
    pub fn new(id: EntityId, class: ClassId, class_name: &ServerClassName) -> Self {
        Projectile {
            id,
            team: Team::default(),
            class,
            position: Vector::default(),
            rotation: Vector::default(),
            initial_speed: Vector::default(),
            bounds: None,
            launcher: Handle::default(),
            ty: ProjectileType::new(class_name, None),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PipeType {
    Regular = 0,
    Sticky = 1,
    StickyJumper = 2,
    LooseCannon = 3,
}

impl PipeType {
    pub fn new(number: i64) -> Self {
        match number {
            1 => PipeType::Sticky,
            2 => PipeType::StickyJumper,
            3 => PipeType::LooseCannon,
            _ => PipeType::Regular,
        }
    }

    pub fn is_sticky(&self) -> bool {
        match self {
            PipeType::Regular | PipeType::LooseCannon => false,
            PipeType::Sticky | PipeType::StickyJumper => true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum ProjectileType {
    Rocket = 0,
    HealingArrow = 1,
    Sticky = 2,
    Pipe = 3,
    Flare = 4,
    LooseCannon = 5,
    #[default]
    Unknown = 7,
}

impl ProjectileType {
    pub fn new(class: &ServerClassName, pipe_type: Option<PipeType>) -> Self {
        match (class.as_str(), pipe_type) {
            ("CTFGrenadePipebombProjectile", Some(PipeType::Sticky | PipeType::StickyJumper)) => {
                ProjectileType::Sticky
            }
            ("CTFGrenadePipebombProjectile", Some(PipeType::LooseCannon)) => {
                ProjectileType::LooseCannon
            }
            ("CTFGrenadePipebombProjectile", _) => ProjectileType::Pipe,
            ("CTFProjectile_SentryRocket" | "CTFProjectile_Rocket", _) => ProjectileType::Rocket,
            ("CTFProjectile_Flare", _) => ProjectileType::Flare,
            ("CTFProjectile_HealingBolt", _) => ProjectileType::HealingArrow,
            _ => ProjectileType::Unknown,
        }
    }
}

impl From<u8> for ProjectileType {
    fn from(value: u8) -> Self {
        match value {
            0 => ProjectileType::Rocket,
            1 => ProjectileType::HealingArrow,
            2 => ProjectileType::Sticky,
            3 => ProjectileType::Pipe,
            4 => ProjectileType::Flare,
            5 => ProjectileType::LooseCannon,
            _ => ProjectileType::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Collision {
    pub tick: DemoTick,
    pub target: EntityId,
    pub projectile: Projectile,
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
    pub fn new(tick: DemoTick, death: &PlayerDeathEvent) -> Self {
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
    pub projectiles: BTreeMap<EntityId, Projectile>,
    pub collisions: Vec<Collision>,
    pub world: Option<World>,
    pub kills: Vec<Kill>,
    pub tick: DemoTick,
    pub server_classes: Vec<ServerClass>,
    pub interval_per_tick: f32,
    pub outer_map: HashMap<Handle, EntityId>,
    pub events: Vec<(DemoTick, GameEvent)>,
}

impl GameState {
    pub fn get_player(&self, id: EntityId) -> Option<&Player> {
        self.players.iter().find(|player| player.entity == id)
    }

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
                self.players.push(Player::new(entity_id));
                index
            }
        };

        #[allow(clippy::indexing_slicing)]
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

    pub fn check_collision(&self, projectile: &Projectile) -> Option<&Player> {
        self.players
            .iter()
            .filter(|player| player.state == PlayerState::Alive)
            .filter(|player| player.team != projectile.team)
            .find(|player| player.collides(projectile, self.interval_per_tick))
    }

    pub fn projectile_destroy(&mut self, id: EntityId) {
        if let Some(projectile) = self.projectiles.remove(&id) {
            if let Some(target) = self.check_collision(&projectile) {
                self.collisions.push(Collision {
                    tick: self.tick,
                    target: target.entity,
                    projectile,
                })
            }
        }
    }

    pub fn remove_building(&mut self, entity_id: EntityId) {
        self.buildings.remove(&entity_id);
    }
}
