use super::gamevent::{FromGameEventValue, FromRawGameEvent, GameEventValue, RawGameEvent};
use crate::{GameEventError, MalformedDemoError, Result};
#[derive(Debug)]
pub struct ServerSpawnEvent {
    pub hostname: String,
    pub address: String,
    pub ip: u32,
    pub port: u16,
    pub game: String,
    pub map_name: String,
    pub max_players: u32,
    pub os: String,
    pub dedicated: bool,
    pub password: bool,
}
impl FromRawGameEvent for ServerSpawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerSpawnEvent {
            hostname: String::from_value(iter.next(), "hostname")?,
            address: String::from_value(iter.next(), "address")?,
            ip: u32::from_value(iter.next(), "ip")?,
            port: u16::from_value(iter.next(), "port")?,
            game: String::from_value(iter.next(), "game")?,
            map_name: String::from_value(iter.next(), "map_name")?,
            max_players: u32::from_value(iter.next(), "max_players")?,
            os: String::from_value(iter.next(), "os")?,
            dedicated: bool::from_value(iter.next(), "dedicated")?,
            password: bool::from_value(iter.next(), "password")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerChangeLevelFailedEvent {
    pub level_name: String,
}
impl FromRawGameEvent for ServerChangeLevelFailedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerChangeLevelFailedEvent {
            level_name: String::from_value(iter.next(), "level_name")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerShutdownEvent {
    pub reason: String,
}
impl FromRawGameEvent for ServerShutdownEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerShutdownEvent {
            reason: String::from_value(iter.next(), "reason")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerCvarEvent {
    pub cvar_name: String,
    pub cvar_value: String,
}
impl FromRawGameEvent for ServerCvarEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerCvarEvent {
            cvar_name: String::from_value(iter.next(), "cvar_name")?,
            cvar_value: String::from_value(iter.next(), "cvar_value")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for ServerMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerMessageEvent {
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerAddBanEvent {
    pub name: String,
    pub user_id: u16,
    pub network_id: String,
    pub ip: String,
    pub duration: String,
    pub by: String,
    pub kicked: bool,
}
impl FromRawGameEvent for ServerAddBanEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerAddBanEvent {
            name: String::from_value(iter.next(), "name")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            network_id: String::from_value(iter.next(), "network_id")?,
            ip: String::from_value(iter.next(), "ip")?,
            duration: String::from_value(iter.next(), "duration")?,
            by: String::from_value(iter.next(), "by")?,
            kicked: bool::from_value(iter.next(), "kicked")?,
        })
    }
}
#[derive(Debug)]
pub struct ServerRemoveBanEvent {
    pub network_id: String,
    pub ip: String,
    pub by: String,
}
impl FromRawGameEvent for ServerRemoveBanEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ServerRemoveBanEvent {
            network_id: String::from_value(iter.next(), "network_id")?,
            ip: String::from_value(iter.next(), "ip")?,
            by: String::from_value(iter.next(), "by")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerConnectEvent {
    pub name: String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: String,
    pub address: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerConnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerConnectEvent {
            name: String::from_value(iter.next(), "name")?,
            index: u8::from_value(iter.next(), "index")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            network_id: String::from_value(iter.next(), "network_id")?,
            address: String::from_value(iter.next(), "address")?,
            bot: u16::from_value(iter.next(), "bot")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerConnectClientEvent {
    pub name: String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerConnectClientEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerConnectClientEvent {
            name: String::from_value(iter.next(), "name")?,
            index: u8::from_value(iter.next(), "index")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            network_id: String::from_value(iter.next(), "network_id")?,
            bot: u16::from_value(iter.next(), "bot")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerInfoEvent {
    pub name: String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: String,
    pub bot: bool,
}
impl FromRawGameEvent for PlayerInfoEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerInfoEvent {
            name: String::from_value(iter.next(), "name")?,
            index: u8::from_value(iter.next(), "index")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            network_id: String::from_value(iter.next(), "network_id")?,
            bot: bool::from_value(iter.next(), "bot")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDisconnectEvent {
    pub user_id: u16,
    pub reason: String,
    pub name: String,
    pub network_id: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerDisconnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDisconnectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            reason: String::from_value(iter.next(), "reason")?,
            name: String::from_value(iter.next(), "name")?,
            network_id: String::from_value(iter.next(), "network_id")?,
            bot: u16::from_value(iter.next(), "bot")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerActivateEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerActivateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerActivateEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerSayEvent {
    pub user_id: u16,
    pub text: String,
}
impl FromRawGameEvent for PlayerSayEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerSayEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct ClientDisconnectEvent {
    pub message: String,
}
impl FromRawGameEvent for ClientDisconnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ClientDisconnectEvent {
            message: String::from_value(iter.next(), "message")?,
        })
    }
}
#[derive(Debug)]
pub struct ClientBeginConnectEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
    pub source: String,
}
impl FromRawGameEvent for ClientBeginConnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ClientBeginConnectEvent {
            address: String::from_value(iter.next(), "address")?,
            ip: u32::from_value(iter.next(), "ip")?,
            port: u16::from_value(iter.next(), "port")?,
            source: String::from_value(iter.next(), "source")?,
        })
    }
}
#[derive(Debug)]
pub struct ClientConnectedEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
}
impl FromRawGameEvent for ClientConnectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ClientConnectedEvent {
            address: String::from_value(iter.next(), "address")?,
            ip: u32::from_value(iter.next(), "ip")?,
            port: u16::from_value(iter.next(), "port")?,
        })
    }
}
#[derive(Debug)]
pub struct ClientFullConnectEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
}
impl FromRawGameEvent for ClientFullConnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ClientFullConnectEvent {
            address: String::from_value(iter.next(), "address")?,
            ip: u32::from_value(iter.next(), "ip")?,
            port: u16::from_value(iter.next(), "port")?,
        })
    }
}
#[derive(Debug)]
pub struct HostQuitEvent {}
impl FromRawGameEvent for HostQuitEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(HostQuitEvent {})
    }
}
#[derive(Debug)]
pub struct TeamInfoEvent {
    pub team_id: u8,
    pub team_name: String,
}
impl FromRawGameEvent for TeamInfoEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamInfoEvent {
            team_id: u8::from_value(iter.next(), "team_id")?,
            team_name: String::from_value(iter.next(), "team_name")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamScoreEvent {
    pub team_id: u8,
    pub score: u16,
}
impl FromRawGameEvent for TeamScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamScoreEvent {
            team_id: u8::from_value(iter.next(), "team_id")?,
            score: u16::from_value(iter.next(), "score")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayBroadcastAudioEvent {
    pub team: u8,
    pub sound: String,
    pub additional_flags: u16,
}
impl FromRawGameEvent for TeamPlayBroadcastAudioEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayBroadcastAudioEvent {
            team: u8::from_value(iter.next(), "team")?,
            sound: String::from_value(iter.next(), "sound")?,
            additional_flags: u16::from_value(iter.next(), "additional_flags")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerTeamEvent {
    pub user_id: u16,
    pub team: u8,
    pub old_team: u8,
    pub disconnect: bool,
    pub auto_team: bool,
    pub silent: bool,
    pub name: String,
}
impl FromRawGameEvent for PlayerTeamEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerTeamEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            team: u8::from_value(iter.next(), "team")?,
            old_team: u8::from_value(iter.next(), "old_team")?,
            disconnect: bool::from_value(iter.next(), "disconnect")?,
            auto_team: bool::from_value(iter.next(), "auto_team")?,
            silent: bool::from_value(iter.next(), "silent")?,
            name: String::from_value(iter.next(), "name")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerClassEvent {
    pub user_id: u16,
    pub class: String,
}
impl FromRawGameEvent for PlayerClassEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerClassEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            class: String::from_value(iter.next(), "class")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDeathEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub player_penetrate_count: u16,
    pub assister_fallback: String,
    pub kill_streak_total: u16,
    pub kill_streak_wep: u16,
    pub kill_streak_assist: u16,
    pub kill_streak_victim: u16,
    pub ducks_streaked: u16,
    pub duck_streak_total: u16,
    pub duck_streak_assist: u16,
    pub duck_streak_victim: u16,
    pub rocket_jump: bool,
    pub weapon_def_index: u32,
    pub crit_type: u16,
}
impl FromRawGameEvent for PlayerDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDeathEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
            stun_flags: u16::from_value(iter.next(), "stun_flags")?,
            death_flags: u16::from_value(iter.next(), "death_flags")?,
            silent_kill: bool::from_value(iter.next(), "silent_kill")?,
            player_penetrate_count: u16::from_value(iter.next(), "player_penetrate_count")?,
            assister_fallback: String::from_value(iter.next(), "assister_fallback")?,
            kill_streak_total: u16::from_value(iter.next(), "kill_streak_total")?,
            kill_streak_wep: u16::from_value(iter.next(), "kill_streak_wep")?,
            kill_streak_assist: u16::from_value(iter.next(), "kill_streak_assist")?,
            kill_streak_victim: u16::from_value(iter.next(), "kill_streak_victim")?,
            ducks_streaked: u16::from_value(iter.next(), "ducks_streaked")?,
            duck_streak_total: u16::from_value(iter.next(), "duck_streak_total")?,
            duck_streak_assist: u16::from_value(iter.next(), "duck_streak_assist")?,
            duck_streak_victim: u16::from_value(iter.next(), "duck_streak_victim")?,
            rocket_jump: bool::from_value(iter.next(), "rocket_jump")?,
            weapon_def_index: u32::from_value(iter.next(), "weapon_def_index")?,
            crit_type: u16::from_value(iter.next(), "crit_type")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHurtEvent {
    pub user_id: u16,
    pub health: u16,
    pub attacker: u16,
    pub damage_amount: u16,
    pub custom: u16,
    pub show_disguised_crit: bool,
    pub crit: bool,
    pub mini_crit: bool,
    pub all_see_crit: bool,
    pub weapon_id: u16,
    pub bonus_effect: u8,
}
impl FromRawGameEvent for PlayerHurtEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHurtEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            health: u16::from_value(iter.next(), "health")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            damage_amount: u16::from_value(iter.next(), "damage_amount")?,
            custom: u16::from_value(iter.next(), "custom")?,
            show_disguised_crit: bool::from_value(iter.next(), "show_disguised_crit")?,
            crit: bool::from_value(iter.next(), "crit")?,
            mini_crit: bool::from_value(iter.next(), "mini_crit")?,
            all_see_crit: bool::from_value(iter.next(), "all_see_crit")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            bonus_effect: u8::from_value(iter.next(), "bonus_effect")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerChatEvent {
    pub team_only: bool,
    pub user_id: u16,
    pub text: String,
}
impl FromRawGameEvent for PlayerChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerChatEvent {
            team_only: bool::from_value(iter.next(), "team_only")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerScoreEvent {
    pub user_id: u16,
    pub kills: u16,
    pub deaths: u16,
    pub score: u16,
}
impl FromRawGameEvent for PlayerScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerScoreEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            kills: u16::from_value(iter.next(), "kills")?,
            deaths: u16::from_value(iter.next(), "deaths")?,
            score: u16::from_value(iter.next(), "score")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerSpawnEvent {
    pub user_id: u16,
    pub team: u16,
    pub class: u16,
}
impl FromRawGameEvent for PlayerSpawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerSpawnEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            team: u16::from_value(iter.next(), "team")?,
            class: u16::from_value(iter.next(), "class")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerShootEvent {
    pub user_id: u16,
    pub weapon: u8,
    pub mode: u8,
}
impl FromRawGameEvent for PlayerShootEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerShootEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            weapon: u8::from_value(iter.next(), "weapon")?,
            mode: u8::from_value(iter.next(), "mode")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerUseEvent {
    pub user_id: u16,
    pub entity: u16,
}
impl FromRawGameEvent for PlayerUseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerUseEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            entity: u16::from_value(iter.next(), "entity")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerChangeNameEvent {
    pub user_id: u16,
    pub old_name: String,
    pub new_name: String,
}
impl FromRawGameEvent for PlayerChangeNameEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerChangeNameEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            old_name: String::from_value(iter.next(), "old_name")?,
            new_name: String::from_value(iter.next(), "new_name")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHintMessageEvent {
    pub hint_message: String,
}
impl FromRawGameEvent for PlayerHintMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHintMessageEvent {
            hint_message: String::from_value(iter.next(), "hint_message")?,
        })
    }
}
#[derive(Debug)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for BasePlayerTeleportedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BasePlayerTeleportedEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct GameInitEvent {}
impl FromRawGameEvent for GameInitEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GameInitEvent {})
    }
}
#[derive(Debug)]
pub struct GameNewMapEvent {
    pub map_name: String,
}
impl FromRawGameEvent for GameNewMapEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(GameNewMapEvent {
            map_name: String::from_value(iter.next(), "map_name")?,
        })
    }
}
#[derive(Debug)]
pub struct GameStartEvent {
    pub rounds_limit: u32,
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: String,
}
impl FromRawGameEvent for GameStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(GameStartEvent {
            rounds_limit: u32::from_value(iter.next(), "rounds_limit")?,
            time_limit: u32::from_value(iter.next(), "time_limit")?,
            frag_limit: u32::from_value(iter.next(), "frag_limit")?,
            objective: String::from_value(iter.next(), "objective")?,
        })
    }
}
#[derive(Debug)]
pub struct GameEndEvent {
    pub winner: u8,
}
impl FromRawGameEvent for GameEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(GameEndEvent {
            winner: u8::from_value(iter.next(), "winner")?,
        })
    }
}
#[derive(Debug)]
pub struct RoundStartEvent {
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: String,
}
impl FromRawGameEvent for RoundStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RoundStartEvent {
            time_limit: u32::from_value(iter.next(), "time_limit")?,
            frag_limit: u32::from_value(iter.next(), "frag_limit")?,
            objective: String::from_value(iter.next(), "objective")?,
        })
    }
}
#[derive(Debug)]
pub struct RoundEndEvent {
    pub winner: u8,
    pub reason: u8,
    pub message: String,
}
impl FromRawGameEvent for RoundEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RoundEndEvent {
            winner: u8::from_value(iter.next(), "winner")?,
            reason: u8::from_value(iter.next(), "reason")?,
            message: String::from_value(iter.next(), "message")?,
        })
    }
}
#[derive(Debug)]
pub struct GameMessageEvent {
    pub target: u8,
    pub text: String,
}
impl FromRawGameEvent for GameMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(GameMessageEvent {
            target: u8::from_value(iter.next(), "target")?,
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct BreakBreakableEvent {
    pub ent_index: u32,
    pub user_id: u16,
    pub material: u8,
}
impl FromRawGameEvent for BreakBreakableEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BreakBreakableEvent {
            ent_index: u32::from_value(iter.next(), "ent_index")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
            material: u8::from_value(iter.next(), "material")?,
        })
    }
}
#[derive(Debug)]
pub struct BreakPropEvent {
    pub ent_index: u32,
    pub user_id: u16,
}
impl FromRawGameEvent for BreakPropEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BreakPropEvent {
            ent_index: u32::from_value(iter.next(), "ent_index")?,
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct EntityKilledEvent {
    pub ent_index_killed: u32,
    pub ent_index_attacker: u32,
    pub ent_index_inflictor: u32,
    pub damage_bits: u32,
}
impl FromRawGameEvent for EntityKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EntityKilledEvent {
            ent_index_killed: u32::from_value(iter.next(), "ent_index_killed")?,
            ent_index_attacker: u32::from_value(iter.next(), "ent_index_attacker")?,
            ent_index_inflictor: u32::from_value(iter.next(), "ent_index_inflictor")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
        })
    }
}
#[derive(Debug)]
pub struct BonusUpdatedEvent {
    pub num_advanced: u16,
    pub num_bronze: u16,
    pub num_silver: u16,
    pub num_gold: u16,
}
impl FromRawGameEvent for BonusUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BonusUpdatedEvent {
            num_advanced: u16::from_value(iter.next(), "num_advanced")?,
            num_bronze: u16::from_value(iter.next(), "num_bronze")?,
            num_silver: u16::from_value(iter.next(), "num_silver")?,
            num_gold: u16::from_value(iter.next(), "num_gold")?,
        })
    }
}
#[derive(Debug)]
pub struct AchievementEventEvent {
    pub achievement_name: String,
    pub cur_val: u16,
    pub max_val: u16,
}
impl FromRawGameEvent for AchievementEventEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(AchievementEventEvent {
            achievement_name: String::from_value(iter.next(), "achievement_name")?,
            cur_val: u16::from_value(iter.next(), "cur_val")?,
            max_val: u16::from_value(iter.next(), "max_val")?,
        })
    }
}
#[derive(Debug)]
pub struct AchievementIncrementEvent {
    pub achievement_id: u32,
    pub cur_val: u16,
    pub max_val: u16,
}
impl FromRawGameEvent for AchievementIncrementEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(AchievementIncrementEvent {
            achievement_id: u32::from_value(iter.next(), "achievement_id")?,
            cur_val: u16::from_value(iter.next(), "cur_val")?,
            max_val: u16::from_value(iter.next(), "max_val")?,
        })
    }
}
#[derive(Debug)]
pub struct PhysgunPickupEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for PhysgunPickupEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PhysgunPickupEvent {
            ent_index: u32::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for FlareIgniteNpcEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(FlareIgniteNpcEvent {
            ent_index: u32::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct HelicopterGrenadePuntMissEvent {}
impl FromRawGameEvent for HelicopterGrenadePuntMissEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(HelicopterGrenadePuntMissEvent {})
    }
}
#[derive(Debug)]
pub struct UserDataDownloadedEvent {}
impl FromRawGameEvent for UserDataDownloadedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(UserDataDownloadedEvent {})
    }
}
#[derive(Debug)]
pub struct RagdollDissolvedEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for RagdollDissolvedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RagdollDissolvedEvent {
            ent_index: u32::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVChangedModeEvent {
    pub old_mode: u16,
    pub new_mode: u16,
    pub obs_target: u16,
}
impl FromRawGameEvent for HLTVChangedModeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVChangedModeEvent {
            old_mode: u16::from_value(iter.next(), "old_mode")?,
            new_mode: u16::from_value(iter.next(), "new_mode")?,
            obs_target: u16::from_value(iter.next(), "obs_target")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVChangedTargetEvent {
    pub mode: u16,
    pub old_target: u16,
    pub obs_target: u16,
}
impl FromRawGameEvent for HLTVChangedTargetEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVChangedTargetEvent {
            mode: u16::from_value(iter.next(), "mode")?,
            old_target: u16::from_value(iter.next(), "old_target")?,
            obs_target: u16::from_value(iter.next(), "obs_target")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteEndedEvent {}
impl FromRawGameEvent for VoteEndedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(VoteEndedEvent {})
    }
}
#[derive(Debug)]
pub struct VoteStartedEvent {
    pub issue: String,
    pub param_1: String,
    pub team: u8,
    pub initiator: u32,
}
impl FromRawGameEvent for VoteStartedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VoteStartedEvent {
            issue: String::from_value(iter.next(), "issue")?,
            param_1: String::from_value(iter.next(), "param_1")?,
            team: u8::from_value(iter.next(), "team")?,
            initiator: u32::from_value(iter.next(), "initiator")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteChangedEvent {
    pub vote_option_1: u8,
    pub vote_option_2: u8,
    pub vote_option_3: u8,
    pub vote_option_4: u8,
    pub vote_option_5: u8,
    pub potential_votes: u8,
}
impl FromRawGameEvent for VoteChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VoteChangedEvent {
            vote_option_1: u8::from_value(iter.next(), "vote_option_1")?,
            vote_option_2: u8::from_value(iter.next(), "vote_option_2")?,
            vote_option_3: u8::from_value(iter.next(), "vote_option_3")?,
            vote_option_4: u8::from_value(iter.next(), "vote_option_4")?,
            vote_option_5: u8::from_value(iter.next(), "vote_option_5")?,
            potential_votes: u8::from_value(iter.next(), "potential_votes")?,
        })
    }
}
#[derive(Debug)]
pub struct VotePassedEvent {
    pub details: String,
    pub param_1: String,
    pub team: u8,
}
impl FromRawGameEvent for VotePassedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VotePassedEvent {
            details: String::from_value(iter.next(), "details")?,
            param_1: String::from_value(iter.next(), "param_1")?,
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteFailedEvent {
    pub team: u8,
}
impl FromRawGameEvent for VoteFailedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VoteFailedEvent {
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteCastEvent {
    pub vote_option: u8,
    pub team: u16,
    pub entity_id: u32,
}
impl FromRawGameEvent for VoteCastEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VoteCastEvent {
            vote_option: u8::from_value(iter.next(), "vote_option")?,
            team: u16::from_value(iter.next(), "team")?,
            entity_id: u32::from_value(iter.next(), "entity_id")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteOptionsEvent {
    pub count: u8,
    pub option_1: String,
    pub option_2: String,
    pub option_3: String,
    pub option_4: String,
    pub option_5: String,
}
impl FromRawGameEvent for VoteOptionsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(VoteOptionsEvent {
            count: u8::from_value(iter.next(), "count")?,
            option_1: String::from_value(iter.next(), "option_1")?,
            option_2: String::from_value(iter.next(), "option_2")?,
            option_3: String::from_value(iter.next(), "option_3")?,
            option_4: String::from_value(iter.next(), "option_4")?,
            option_5: String::from_value(iter.next(), "option_5")?,
        })
    }
}
#[derive(Debug)]
pub struct ReplaySavedEvent {}
impl FromRawGameEvent for ReplaySavedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ReplaySavedEvent {})
    }
}
#[derive(Debug)]
pub struct EnteredPerformanceModeEvent {}
impl FromRawGameEvent for EnteredPerformanceModeEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(EnteredPerformanceModeEvent {})
    }
}
#[derive(Debug)]
pub struct BrowseReplaysEvent {}
impl FromRawGameEvent for BrowseReplaysEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(BrowseReplaysEvent {})
    }
}
#[derive(Debug)]
pub struct ReplayYoutubeStatsEvent {
    pub views: u32,
    pub likes: u32,
    pub favorited: u32,
}
impl FromRawGameEvent for ReplayYoutubeStatsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ReplayYoutubeStatsEvent {
            views: u32::from_value(iter.next(), "views")?,
            likes: u32::from_value(iter.next(), "likes")?,
            favorited: u32::from_value(iter.next(), "favorited")?,
        })
    }
}
#[derive(Debug)]
pub struct InventoryUpdatedEvent {}
impl FromRawGameEvent for InventoryUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(InventoryUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct CartUpdatedEvent {}
impl FromRawGameEvent for CartUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(CartUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct StorePriceSheetUpdatedEvent {}
impl FromRawGameEvent for StorePriceSheetUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(StorePriceSheetUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct EconInventoryConnectedEvent {}
impl FromRawGameEvent for EconInventoryConnectedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(EconInventoryConnectedEvent {})
    }
}
#[derive(Debug)]
pub struct ItemSchemaInitializedEvent {}
impl FromRawGameEvent for ItemSchemaInitializedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ItemSchemaInitializedEvent {})
    }
}
#[derive(Debug)]
pub struct GcNewSessionEvent {}
impl FromRawGameEvent for GcNewSessionEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GcNewSessionEvent {})
    }
}
#[derive(Debug)]
pub struct GcLostSessionEvent {}
impl FromRawGameEvent for GcLostSessionEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GcLostSessionEvent {})
    }
}
#[derive(Debug)]
pub struct IntroFinishEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroFinishEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(IntroFinishEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroNextCameraEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(IntroNextCameraEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerChangeClassEvent {
    pub user_id: u16,
    pub class: u16,
}
impl FromRawGameEvent for PlayerChangeClassEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerChangeClassEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            class: u16::from_value(iter.next(), "class")?,
        })
    }
}
#[derive(Debug)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl FromRawGameEvent for TfMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TfMapTimeRemainingEvent {
            seconds: u32::from_value(iter.next(), "seconds")?,
        })
    }
}
#[derive(Debug)]
pub struct TfGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TfGameOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TfGameOverEvent {
            reason: String::from_value(iter.next(), "reason")?,
        })
    }
}
#[derive(Debug)]
pub struct CtfFlagCapturedEvent {
    pub capping_team: u16,
    pub capping_team_score: u16,
}
impl FromRawGameEvent for CtfFlagCapturedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CtfFlagCapturedEvent {
            capping_team: u16::from_value(iter.next(), "capping_team")?,
            capping_team_score: u16::from_value(iter.next(), "capping_team_score")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointInitializedEvent {}
impl FromRawGameEvent for ControlPointInitializedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ControlPointInitializedEvent {})
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateImagesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointUpdateImagesEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointUpdateLayoutEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateCappingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointUpdateCappingEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateOwnerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointUpdateOwnerEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointStartTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl FromRawGameEvent for ControlPointStartTouchEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointStartTouchEvent {
            player: u16::from_value(iter.next(), "player")?,
            area: u16::from_value(iter.next(), "area")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointEndTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl FromRawGameEvent for ControlPointEndTouchEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointEndTouchEvent {
            player: u16::from_value(iter.next(), "player")?,
            area: u16::from_value(iter.next(), "area")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl FromRawGameEvent for ControlPointPulseElementEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointPulseElementEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointFakeCaptureEvent {
    pub player: u16,
    pub int_data: u16,
}
impl FromRawGameEvent for ControlPointFakeCaptureEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointFakeCaptureEvent {
            player: u16::from_value(iter.next(), "player")?,
            int_data: u16::from_value(iter.next(), "int_data")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointFakeCaptureMultiplierEvent {
    pub player: u16,
    pub int_data: u16,
}
impl FromRawGameEvent for ControlPointFakeCaptureMultiplierEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointFakeCaptureMultiplierEvent {
            player: u16::from_value(iter.next(), "player")?,
            int_data: u16::from_value(iter.next(), "int_data")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: String,
}
impl FromRawGameEvent for TeamPlayRoundSelectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayRoundSelectedEvent {
            round: String::from_value(iter.next(), "round")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl FromRawGameEvent for TeamPlayRoundStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayRoundStartEvent {
            full_reset: bool::from_value(iter.next(), "full_reset")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundActiveEvent {}
impl FromRawGameEvent for TeamPlayRoundActiveEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayRoundActiveEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayWaitingBeginsEvent {}
impl FromRawGameEvent for TeamPlayWaitingBeginsEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayWaitingBeginsEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayWaitingEndsEvent {}
impl FromRawGameEvent for TeamPlayWaitingEndsEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayWaitingEndsEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayWaitingAboutToEndEvent {}
impl FromRawGameEvent for TeamPlayWaitingAboutToEndEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayWaitingAboutToEndEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayRestartRoundEvent {}
impl FromRawGameEvent for TeamPlayRestartRoundEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayRestartRoundEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayReadyRestartEvent {}
impl FromRawGameEvent for TeamPlayReadyRestartEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayReadyRestartEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayRoundRestartSecondsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds: u16::from_value(iter.next(), "seconds")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeamReadyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayTeamReadyEvent {
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundWinEvent {
    pub team: u8,
    pub win_reason: u8,
    pub flag_cap_limit: u16,
    pub full_round: u16,
    pub round_time: f32,
    pub losing_team_num_caps: u16,
    pub was_sudden_death: u8,
}
impl FromRawGameEvent for TeamPlayRoundWinEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayRoundWinEvent {
            team: u8::from_value(iter.next(), "team")?,
            win_reason: u8::from_value(iter.next(), "win_reason")?,
            flag_cap_limit: u16::from_value(iter.next(), "flag_cap_limit")?,
            full_round: u16::from_value(iter.next(), "full_round")?,
            round_time: f32::from_value(iter.next(), "round_time")?,
            losing_team_num_caps: u16::from_value(iter.next(), "losing_team_num_caps")?,
            was_sudden_death: u8::from_value(iter.next(), "was_sudden_death")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayUpdateTimerEvent {}
impl FromRawGameEvent for TeamPlayUpdateTimerEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayUpdateTimerEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl FromRawGameEvent for TeamPlayRoundStalemateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayRoundStalemateEvent {
            reason: u8::from_value(iter.next(), "reason")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayOvertimeBeginEvent {}
impl FromRawGameEvent for TeamPlayOvertimeBeginEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayOvertimeBeginEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayOvertimeEndEvent {}
impl FromRawGameEvent for TeamPlayOvertimeEndEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlayOvertimeEndEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlaySuddenDeathBeginEvent {}
impl FromRawGameEvent for TeamPlaySuddenDeathBeginEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlaySuddenDeathBeginEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlaySuddenDeathEndEvent {}
impl FromRawGameEvent for TeamPlaySuddenDeathEndEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlaySuddenDeathEndEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TeamPlayGameOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayGameOverEvent {
            reason: String::from_value(iter.next(), "reason")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds: u16::from_value(iter.next(), "seconds")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl FromRawGameEvent for TeamPlayTimerFlashEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayTimerFlashEvent {
            time_remaining: u16::from_value(iter.next(), "time_remaining")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayTimerTimeAddedEvent {
    pub timer: u16,
    pub seconds_added: u16,
}
impl FromRawGameEvent for TeamPlayTimerTimeAddedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayTimerTimeAddedEvent {
            timer: u16::from_value(iter.next(), "timer")?,
            seconds_added: u16::from_value(iter.next(), "seconds_added")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayPointStartCaptureEvent {
    pub cp: u8,
    pub cp_name: String,
    pub team: u8,
    pub cap_team: u8,
    pub cappers: String,
    pub cap_time: f32,
}
impl FromRawGameEvent for TeamPlayPointStartCaptureEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayPointStartCaptureEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            team: u8::from_value(iter.next(), "team")?,
            cap_team: u8::from_value(iter.next(), "cap_team")?,
            cappers: String::from_value(iter.next(), "cappers")?,
            cap_time: f32::from_value(iter.next(), "cap_time")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayPointCapturedEvent {
    pub cp: u8,
    pub cp_name: String,
    pub team: u8,
    pub cappers: String,
}
impl FromRawGameEvent for TeamPlayPointCapturedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayPointCapturedEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            team: u8::from_value(iter.next(), "team")?,
            cappers: String::from_value(iter.next(), "cappers")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayPointLockedEvent {
    pub cp: u8,
    pub cp_name: String,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayPointLockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayPointLockedEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayPointUnlockedEvent {
    pub cp: u8,
    pub cp_name: String,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayPointUnlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayPointUnlockedEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayCaptureBrokenEvent {
    pub cp: u8,
    pub cp_name: String,
    pub time_remaining: f32,
}
impl FromRawGameEvent for TeamPlayCaptureBrokenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayCaptureBrokenEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            time_remaining: f32::from_value(iter.next(), "time_remaining")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cp_name: String,
    pub blocker: u8,
    pub victim: u8,
}
impl FromRawGameEvent for TeamPlayCaptureBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayCaptureBlockedEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            cp_name: String::from_value(iter.next(), "cp_name")?,
            blocker: u8::from_value(iter.next(), "blocker")?,
            victim: u8::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayFlagEventEvent {
    pub player: u16,
    pub carrier: u16,
    pub event_type: u16,
    pub home: u8,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayFlagEventEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayFlagEventEvent {
            player: u16::from_value(iter.next(), "player")?,
            carrier: u16::from_value(iter.next(), "carrier")?,
            event_type: u16::from_value(iter.next(), "event_type")?,
            home: u8::from_value(iter.next(), "home")?,
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: String,
    pub flag_cap_limit: u16,
    pub blue_score: u16,
    pub red_score: u16,
    pub blue_score_prev: u16,
    pub red_score_prev: u16,
    pub round_complete: u16,
    pub rounds_remaining: u16,
    pub player_1: u16,
    pub player_1_points: u16,
    pub player_2: u16,
    pub player_2_points: u16,
    pub player_3: u16,
    pub player_3_points: u16,
    pub kill_stream_player_1: u16,
    pub kill_stream_player_1_count: u16,
    pub game_over: u8,
}
impl FromRawGameEvent for TeamPlayWinPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayWinPanelEvent {
            panel_style: u8::from_value(iter.next(), "panel_style")?,
            winning_team: u8::from_value(iter.next(), "winning_team")?,
            win_reason: u8::from_value(iter.next(), "win_reason")?,
            cappers: String::from_value(iter.next(), "cappers")?,
            flag_cap_limit: u16::from_value(iter.next(), "flag_cap_limit")?,
            blue_score: u16::from_value(iter.next(), "blue_score")?,
            red_score: u16::from_value(iter.next(), "red_score")?,
            blue_score_prev: u16::from_value(iter.next(), "blue_score_prev")?,
            red_score_prev: u16::from_value(iter.next(), "red_score_prev")?,
            round_complete: u16::from_value(iter.next(), "round_complete")?,
            rounds_remaining: u16::from_value(iter.next(), "rounds_remaining")?,
            player_1: u16::from_value(iter.next(), "player_1")?,
            player_1_points: u16::from_value(iter.next(), "player_1_points")?,
            player_2: u16::from_value(iter.next(), "player_2")?,
            player_2_points: u16::from_value(iter.next(), "player_2_points")?,
            player_3: u16::from_value(iter.next(), "player_3")?,
            player_3_points: u16::from_value(iter.next(), "player_3_points")?,
            kill_stream_player_1: u16::from_value(iter.next(), "kill_stream_player_1")?,
            kill_stream_player_1_count: u16::from_value(iter.next(), "kill_stream_player_1_count")?,
            game_over: u8::from_value(iter.next(), "game_over")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayTeamBalancedPlayerEvent {
    pub player: u16,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeamBalancedPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayTeamBalancedPlayerEvent {
            player: u16::from_value(iter.next(), "player")?,
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlaySetupFinishedEvent {}
impl FromRawGameEvent for TeamPlaySetupFinishedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamPlaySetupFinishedEvent {})
    }
}
#[derive(Debug)]
pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl FromRawGameEvent for TeamPlayAlertEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayAlertEvent {
            alert_type: u16::from_value(iter.next(), "alert_type")?,
        })
    }
}
#[derive(Debug)]
pub struct TrainingCompleteEvent {
    pub next_map: String,
    pub map: String,
    pub text: String,
}
impl FromRawGameEvent for TrainingCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TrainingCompleteEvent {
            next_map: String::from_value(iter.next(), "next_map")?,
            map: String::from_value(iter.next(), "map")?,
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl FromRawGameEvent for ShowFreezePanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ShowFreezePanelEvent {
            killer: u16::from_value(iter.next(), "killer")?,
        })
    }
}
#[derive(Debug)]
pub struct HideFreezePanelEvent {}
impl FromRawGameEvent for HideFreezePanelEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(HideFreezePanelEvent {})
    }
}
#[derive(Debug)]
pub struct FreezeCamStartedEvent {}
impl FromRawGameEvent for FreezeCamStartedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(FreezeCamStartedEvent {})
    }
}
#[derive(Debug)]
pub struct LocalPlayerChangeTeamEvent {}
impl FromRawGameEvent for LocalPlayerChangeTeamEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerChangeTeamEvent {})
    }
}
#[derive(Debug)]
pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl FromRawGameEvent for LocalPlayerScoreChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(LocalPlayerScoreChangedEvent {
            score: u16::from_value(iter.next(), "score")?,
        })
    }
}
#[derive(Debug)]
pub struct LocalPlayerChangeClassEvent {}
impl FromRawGameEvent for LocalPlayerChangeClassEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerChangeClassEvent {})
    }
}
#[derive(Debug)]
pub struct LocalPlayerRespawnEvent {}
impl FromRawGameEvent for LocalPlayerRespawnEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerRespawnEvent {})
    }
}
#[derive(Debug)]
pub struct BuildingInfoChangedEvent {
    pub building_type: u8,
    pub object_mode: u8,
    pub remove: u8,
}
impl FromRawGameEvent for BuildingInfoChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BuildingInfoChangedEvent {
            building_type: u8::from_value(iter.next(), "building_type")?,
            object_mode: u8::from_value(iter.next(), "object_mode")?,
            remove: u8::from_value(iter.next(), "remove")?,
        })
    }
}
#[derive(Debug)]
pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl FromRawGameEvent for LocalPlayerChangeDisguiseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised: bool::from_value(iter.next(), "disguised")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerAccountChangedEvent {
    pub old_value: u16,
    pub new_value: u16,
}
impl FromRawGameEvent for PlayerAccountChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerAccountChangedEvent {
            old_value: u16::from_value(iter.next(), "old_value")?,
            new_value: u16::from_value(iter.next(), "new_value")?,
        })
    }
}
#[derive(Debug)]
pub struct SpyPdaResetEvent {}
impl FromRawGameEvent for SpyPdaResetEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(SpyPdaResetEvent {})
    }
}
#[derive(Debug)]
pub struct FlagStatusUpdateEvent {
    pub user_id: u16,
    pub ent_index: u32,
}
impl FromRawGameEvent for FlagStatusUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(FlagStatusUpdateEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            ent_index: u32::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl FromRawGameEvent for PlayerStatsUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerStatsUpdatedEvent {
            force_upload: bool::from_value(iter.next(), "force_upload")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayingCommentaryEvent {}
impl FromRawGameEvent for PlayingCommentaryEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PlayingCommentaryEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerChargeDeployedEvent {
    pub user_id: u16,
    pub target_id: u16,
}
impl FromRawGameEvent for PlayerChargeDeployedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerChargeDeployedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            target_id: u16::from_value(iter.next(), "target_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerBuiltObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerBuiltObjectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerBuiltObjectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object: u16::from_value(iter.next(), "object")?,
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerUpgradedObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
    pub is_builder: bool,
}
impl FromRawGameEvent for PlayerUpgradedObjectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerUpgradedObjectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object: u16::from_value(iter.next(), "object")?,
            index: u16::from_value(iter.next(), "index")?,
            is_builder: bool::from_value(iter.next(), "is_builder")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerCarryObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerCarryObjectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerCarryObjectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object: u16::from_value(iter.next(), "object")?,
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDropObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerDropObjectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDropObjectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object: u16::from_value(iter.next(), "object")?,
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ObjectRemovedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl FromRawGameEvent for ObjectRemovedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ObjectRemovedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object_type: u16::from_value(iter.next(), "object_type")?,
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ObjectDestroyedEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub assister: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub object_type: u16,
    pub index: u16,
    pub was_building: bool,
}
impl FromRawGameEvent for ObjectDestroyedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ObjectDestroyedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            object_type: u16::from_value(iter.next(), "object_type")?,
            index: u16::from_value(iter.next(), "index")?,
            was_building: bool::from_value(iter.next(), "was_building")?,
        })
    }
}
#[derive(Debug)]
pub struct ObjectDetonatedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl FromRawGameEvent for ObjectDetonatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ObjectDetonatedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            object_type: u16::from_value(iter.next(), "object_type")?,
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct AchievementEarnedEvent {
    pub player: u8,
    pub achievement: u16,
}
impl FromRawGameEvent for AchievementEarnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(AchievementEarnedEvent {
            player: u8::from_value(iter.next(), "player")?,
            achievement: u16::from_value(iter.next(), "achievement")?,
        })
    }
}
#[derive(Debug)]
pub struct SpecTargetUpdatedEvent {}
impl FromRawGameEvent for SpecTargetUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(SpecTargetUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct TournamentStateUpdateEvent {
    pub user_id: u16,
    pub name_change: bool,
    pub ready_state: u16,
    pub new_name: String,
}
impl FromRawGameEvent for TournamentStateUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TournamentStateUpdateEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            name_change: bool::from_value(iter.next(), "name_change")?,
            ready_state: u16::from_value(iter.next(), "ready_state")?,
            new_name: String::from_value(iter.next(), "new_name")?,
        })
    }
}
#[derive(Debug)]
pub struct TournamentEnableCountdownEvent {}
impl FromRawGameEvent for TournamentEnableCountdownEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TournamentEnableCountdownEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerCalledForMedicEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerCalledForMedicEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerCalledForMedicEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerAskedForBallEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerAskedForBallEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerAskedForBallEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct LocalPlayerBecameObserverEvent {}
impl FromRawGameEvent for LocalPlayerBecameObserverEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerBecameObserverEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerIgnitedInvEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub medic_ent_index: u8,
}
impl FromRawGameEvent for PlayerIgnitedInvEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index: u8::from_value(iter.next(), "pyro_ent_index")?,
            victim_ent_index: u8::from_value(iter.next(), "victim_ent_index")?,
            medic_ent_index: u8::from_value(iter.next(), "medic_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerIgnitedEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub weapon_id: u8,
}
impl FromRawGameEvent for PlayerIgnitedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerIgnitedEvent {
            pyro_ent_index: u8::from_value(iter.next(), "pyro_ent_index")?,
            victim_ent_index: u8::from_value(iter.next(), "victim_ent_index")?,
            weapon_id: u8::from_value(iter.next(), "weapon_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
    pub item_definition_index: u16,
}
impl FromRawGameEvent for PlayerExtinguishedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerExtinguishedEvent {
            victim: u8::from_value(iter.next(), "victim")?,
            healer: u8::from_value(iter.next(), "healer")?,
            item_definition_index: u16::from_value(iter.next(), "item_definition_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerTeleportedEvent {
    pub user_id: u16,
    pub builder_id: u16,
    pub dist: f32,
}
impl FromRawGameEvent for PlayerTeleportedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerTeleportedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            builder_id: u16::from_value(iter.next(), "builder_id")?,
            dist: f32::from_value(iter.next(), "dist")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHealedMedicCallEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerHealedMedicCallEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHealedMedicCallEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct LocalPlayerChargeReadyEvent {}
impl FromRawGameEvent for LocalPlayerChargeReadyEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerChargeReadyEvent {})
    }
}
#[derive(Debug)]
pub struct LocalPlayerWindDownEvent {}
impl FromRawGameEvent for LocalPlayerWindDownEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerWindDownEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerInvulnedEvent {
    pub user_id: u16,
    pub medic_user_id: u16,
}
impl FromRawGameEvent for PlayerInvulnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerInvulnedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            medic_user_id: u16::from_value(iter.next(), "medic_user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct EscortSpeedEvent {
    pub team: u8,
    pub speed: u8,
    pub players: u8,
}
impl FromRawGameEvent for EscortSpeedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EscortSpeedEvent {
            team: u8::from_value(iter.next(), "team")?,
            speed: u8::from_value(iter.next(), "speed")?,
            players: u8::from_value(iter.next(), "players")?,
        })
    }
}
#[derive(Debug)]
pub struct EscortProgressEvent {
    pub team: u8,
    pub progress: f32,
    pub reset: bool,
}
impl FromRawGameEvent for EscortProgressEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EscortProgressEvent {
            team: u8::from_value(iter.next(), "team")?,
            progress: f32::from_value(iter.next(), "progress")?,
            reset: bool::from_value(iter.next(), "reset")?,
        })
    }
}
#[derive(Debug)]
pub struct EscortRecedeEvent {
    pub team: u8,
    pub recede_time: f32,
}
impl FromRawGameEvent for EscortRecedeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EscortRecedeEvent {
            team: u8::from_value(iter.next(), "team")?,
            recede_time: f32::from_value(iter.next(), "recede_time")?,
        })
    }
}
#[derive(Debug)]
pub struct GameUIActivatedEvent {}
impl FromRawGameEvent for GameUIActivatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GameUIActivatedEvent {})
    }
}
#[derive(Debug)]
pub struct GameUIHiddenEvent {}
impl FromRawGameEvent for GameUIHiddenEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GameUIHiddenEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl FromRawGameEvent for PlayerEscortScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerEscortScoreEvent {
            player: u8::from_value(iter.next(), "player")?,
            points: u8::from_value(iter.next(), "points")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub ent_index: u8,
    pub weapon_def_index: u32,
}
impl FromRawGameEvent for PlayerHealOnHitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHealOnHitEvent {
            amount: u16::from_value(iter.next(), "amount")?,
            ent_index: u8::from_value(iter.next(), "ent_index")?,
            weapon_def_index: u32::from_value(iter.next(), "weapon_def_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerStealSandvichEvent {
    pub owner: u16,
    pub target: u16,
}
impl FromRawGameEvent for PlayerStealSandvichEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerStealSandvichEvent {
            owner: u16::from_value(iter.next(), "owner")?,
            target: u16::from_value(iter.next(), "target")?,
        })
    }
}
#[derive(Debug)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowClassLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ShowClassLayoutEvent {
            show: bool::from_value(iter.next(), "show")?,
        })
    }
}
#[derive(Debug)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowVsPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ShowVsPanelEvent {
            show: bool::from_value(iter.next(), "show")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDamagedEvent {
    pub amount: u16,
    pub kind: u32,
}
impl FromRawGameEvent for PlayerDamagedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDamagedEvent {
            amount: u16::from_value(iter.next(), "amount")?,
            kind: u32::from_value(iter.next(), "kind")?,
        })
    }
}
#[derive(Debug)]
pub struct ArenaPlayerNotificationEvent {
    pub player: u8,
    pub message: u8,
}
impl FromRawGameEvent for ArenaPlayerNotificationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ArenaPlayerNotificationEvent {
            player: u8::from_value(iter.next(), "player")?,
            message: u8::from_value(iter.next(), "message")?,
        })
    }
}
#[derive(Debug)]
pub struct ArenaMatchMaxStreakEvent {
    pub team: u8,
    pub streak: u8,
}
impl FromRawGameEvent for ArenaMatchMaxStreakEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ArenaMatchMaxStreakEvent {
            team: u8::from_value(iter.next(), "team")?,
            streak: u8::from_value(iter.next(), "streak")?,
        })
    }
}
#[derive(Debug)]
pub struct ArenaRoundStartEvent {}
impl FromRawGameEvent for ArenaRoundStartEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ArenaRoundStartEvent {})
    }
}
#[derive(Debug)]
pub struct ArenaWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: String,
    pub flag_cap_limit: u16,
    pub blue_score: u16,
    pub red_score: u16,
    pub blue_score_prev: u16,
    pub red_score_prev: u16,
    pub round_complete: u16,
    pub player_1: u16,
    pub player_1_damage: u16,
    pub player_1_healing: u16,
    pub player_1_lifetime: u16,
    pub player_1_kills: u16,
    pub player_2: u16,
    pub player_2_damage: u16,
    pub player_2_healing: u16,
    pub player_2_lifetime: u16,
    pub player_2_kills: u16,
    pub player_3: u16,
    pub player_3_damage: u16,
    pub player_3_healing: u16,
    pub player_3_lifetime: u16,
    pub player_3_kills: u16,
    pub player_4: u16,
    pub player_4_damage: u16,
    pub player_4_healing: u16,
    pub player_4_lifetime: u16,
    pub player_4_kills: u16,
    pub player_5: u16,
    pub player_5_damage: u16,
    pub player_5_healing: u16,
    pub player_5_lifetime: u16,
    pub player_5_kills: u16,
    pub player_6: u16,
    pub player_6_damage: u16,
    pub player_6_healing: u16,
    pub player_6_lifetime: u16,
    pub player_6_kills: u16,
}
impl FromRawGameEvent for ArenaWinPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ArenaWinPanelEvent {
            panel_style: u8::from_value(iter.next(), "panel_style")?,
            winning_team: u8::from_value(iter.next(), "winning_team")?,
            win_reason: u8::from_value(iter.next(), "win_reason")?,
            cappers: String::from_value(iter.next(), "cappers")?,
            flag_cap_limit: u16::from_value(iter.next(), "flag_cap_limit")?,
            blue_score: u16::from_value(iter.next(), "blue_score")?,
            red_score: u16::from_value(iter.next(), "red_score")?,
            blue_score_prev: u16::from_value(iter.next(), "blue_score_prev")?,
            red_score_prev: u16::from_value(iter.next(), "red_score_prev")?,
            round_complete: u16::from_value(iter.next(), "round_complete")?,
            player_1: u16::from_value(iter.next(), "player_1")?,
            player_1_damage: u16::from_value(iter.next(), "player_1_damage")?,
            player_1_healing: u16::from_value(iter.next(), "player_1_healing")?,
            player_1_lifetime: u16::from_value(iter.next(), "player_1_lifetime")?,
            player_1_kills: u16::from_value(iter.next(), "player_1_kills")?,
            player_2: u16::from_value(iter.next(), "player_2")?,
            player_2_damage: u16::from_value(iter.next(), "player_2_damage")?,
            player_2_healing: u16::from_value(iter.next(), "player_2_healing")?,
            player_2_lifetime: u16::from_value(iter.next(), "player_2_lifetime")?,
            player_2_kills: u16::from_value(iter.next(), "player_2_kills")?,
            player_3: u16::from_value(iter.next(), "player_3")?,
            player_3_damage: u16::from_value(iter.next(), "player_3_damage")?,
            player_3_healing: u16::from_value(iter.next(), "player_3_healing")?,
            player_3_lifetime: u16::from_value(iter.next(), "player_3_lifetime")?,
            player_3_kills: u16::from_value(iter.next(), "player_3_kills")?,
            player_4: u16::from_value(iter.next(), "player_4")?,
            player_4_damage: u16::from_value(iter.next(), "player_4_damage")?,
            player_4_healing: u16::from_value(iter.next(), "player_4_healing")?,
            player_4_lifetime: u16::from_value(iter.next(), "player_4_lifetime")?,
            player_4_kills: u16::from_value(iter.next(), "player_4_kills")?,
            player_5: u16::from_value(iter.next(), "player_5")?,
            player_5_damage: u16::from_value(iter.next(), "player_5_damage")?,
            player_5_healing: u16::from_value(iter.next(), "player_5_healing")?,
            player_5_lifetime: u16::from_value(iter.next(), "player_5_lifetime")?,
            player_5_kills: u16::from_value(iter.next(), "player_5_kills")?,
            player_6: u16::from_value(iter.next(), "player_6")?,
            player_6_damage: u16::from_value(iter.next(), "player_6_damage")?,
            player_6_healing: u16::from_value(iter.next(), "player_6_healing")?,
            player_6_lifetime: u16::from_value(iter.next(), "player_6_lifetime")?,
            player_6_kills: u16::from_value(iter.next(), "player_6_kills")?,
        })
    }
}
#[derive(Debug)]
pub struct PveWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
}
impl FromRawGameEvent for PveWinPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PveWinPanelEvent {
            panel_style: u8::from_value(iter.next(), "panel_style")?,
            winning_team: u8::from_value(iter.next(), "winning_team")?,
            win_reason: u8::from_value(iter.next(), "win_reason")?,
        })
    }
}
#[derive(Debug)]
pub struct AirDashEvent {
    pub player: u8,
}
impl FromRawGameEvent for AirDashEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(AirDashEvent {
            player: u8::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct LandedEvent {
    pub player: u8,
}
impl FromRawGameEvent for LandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(LandedEvent {
            player: u8::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl FromRawGameEvent for PlayerDamageDodgedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDamageDodgedEvent {
            damage: u16::from_value(iter.next(), "damage")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerStunnedEvent {
    pub stunner: u16,
    pub victim: u16,
    pub victim_capping: bool,
    pub big_stun: bool,
}
impl FromRawGameEvent for PlayerStunnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerStunnedEvent {
            stunner: u16::from_value(iter.next(), "stunner")?,
            victim: u16::from_value(iter.next(), "victim")?,
            victim_capping: bool::from_value(iter.next(), "victim_capping")?,
            big_stun: bool::from_value(iter.next(), "big_stun")?,
        })
    }
}
#[derive(Debug)]
pub struct ScoutGrandSlamEvent {
    pub scout_id: u16,
    pub target_id: u16,
}
impl FromRawGameEvent for ScoutGrandSlamEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ScoutGrandSlamEvent {
            scout_id: u16::from_value(iter.next(), "scout_id")?,
            target_id: u16::from_value(iter.next(), "target_id")?,
        })
    }
}
#[derive(Debug)]
pub struct ScoutSlamdollLandedEvent {
    pub target_index: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl FromRawGameEvent for ScoutSlamdollLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ScoutSlamdollLandedEvent {
            target_index: u16::from_value(iter.next(), "target_index")?,
            x: f32::from_value(iter.next(), "x")?,
            y: f32::from_value(iter.next(), "y")?,
            z: f32::from_value(iter.next(), "z")?,
        })
    }
}
#[derive(Debug)]
pub struct ArrowImpactEvent {
    pub attached_entity: u16,
    pub shooter: u16,
    pub bone_index_attached: u16,
    pub bone_position_x: f32,
    pub bone_position_y: f32,
    pub bone_position_z: f32,
    pub bone_angles_x: f32,
    pub bone_angles_y: f32,
    pub bone_angles_z: f32,
    pub projectile_type: u16,
    pub is_crit: bool,
}
impl FromRawGameEvent for ArrowImpactEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ArrowImpactEvent {
            attached_entity: u16::from_value(iter.next(), "attached_entity")?,
            shooter: u16::from_value(iter.next(), "shooter")?,
            bone_index_attached: u16::from_value(iter.next(), "bone_index_attached")?,
            bone_position_x: f32::from_value(iter.next(), "bone_position_x")?,
            bone_position_y: f32::from_value(iter.next(), "bone_position_y")?,
            bone_position_z: f32::from_value(iter.next(), "bone_position_z")?,
            bone_angles_x: f32::from_value(iter.next(), "bone_angles_x")?,
            bone_angles_y: f32::from_value(iter.next(), "bone_angles_y")?,
            bone_angles_z: f32::from_value(iter.next(), "bone_angles_z")?,
            projectile_type: u16::from_value(iter.next(), "projectile_type")?,
            is_crit: bool::from_value(iter.next(), "is_crit")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerJaratedEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl FromRawGameEvent for PlayerJaratedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerJaratedEvent {
            thrower_ent_index: u8::from_value(iter.next(), "thrower_ent_index")?,
            victim_ent_index: u8::from_value(iter.next(), "victim_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerJaratedFadeEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl FromRawGameEvent for PlayerJaratedFadeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index: u8::from_value(iter.next(), "thrower_ent_index")?,
            victim_ent_index: u8::from_value(iter.next(), "victim_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerShieldBlockedEvent {
    pub attacker_ent_index: u8,
    pub blocker_ent_index: u8,
}
impl FromRawGameEvent for PlayerShieldBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index: u8::from_value(iter.next(), "attacker_ent_index")?,
            blocker_ent_index: u8::from_value(iter.next(), "blocker_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl FromRawGameEvent for PlayerPinnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerPinnedEvent {
            pinned: u8::from_value(iter.next(), "pinned")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl FromRawGameEvent for PlayerHealedByMedicEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHealedByMedicEvent {
            medic: u8::from_value(iter.next(), "medic")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerSappedObjectEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub object: u8,
    pub sapper_id: u16,
}
impl FromRawGameEvent for PlayerSappedObjectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerSappedObjectEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            owner_id: u16::from_value(iter.next(), "owner_id")?,
            object: u8::from_value(iter.next(), "object")?,
            sapper_id: u16::from_value(iter.next(), "sapper_id")?,
        })
    }
}
#[derive(Debug)]
pub struct ItemFoundEvent {
    pub player: u8,
    pub quality: u8,
    pub method: u8,
    pub item_def: u32,
    pub is_strange: u8,
    pub is_unusual: u8,
    pub wear: f32,
}
impl FromRawGameEvent for ItemFoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ItemFoundEvent {
            player: u8::from_value(iter.next(), "player")?,
            quality: u8::from_value(iter.next(), "quality")?,
            method: u8::from_value(iter.next(), "method")?,
            item_def: u32::from_value(iter.next(), "item_def")?,
            is_strange: u8::from_value(iter.next(), "is_strange")?,
            is_unusual: u8::from_value(iter.next(), "is_unusual")?,
            wear: f32::from_value(iter.next(), "wear")?,
        })
    }
}
#[derive(Debug)]
pub struct ShowAnnotationEvent {
    pub world_pos_x: f32,
    pub world_pos_y: f32,
    pub world_pos_z: f32,
    pub world_normal_x: f32,
    pub world_normal_y: f32,
    pub world_normal_z: f32,
    pub id: u32,
    pub text: String,
    pub lifetime: f32,
    pub visibility_bit_field: u32,
    pub follow_ent_index: u32,
    pub show_distance: bool,
    pub play_sound: String,
    pub show_effect: bool,
}
impl FromRawGameEvent for ShowAnnotationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ShowAnnotationEvent {
            world_pos_x: f32::from_value(iter.next(), "world_pos_x")?,
            world_pos_y: f32::from_value(iter.next(), "world_pos_y")?,
            world_pos_z: f32::from_value(iter.next(), "world_pos_z")?,
            world_normal_x: f32::from_value(iter.next(), "world_normal_x")?,
            world_normal_y: f32::from_value(iter.next(), "world_normal_y")?,
            world_normal_z: f32::from_value(iter.next(), "world_normal_z")?,
            id: u32::from_value(iter.next(), "id")?,
            text: String::from_value(iter.next(), "text")?,
            lifetime: f32::from_value(iter.next(), "lifetime")?,
            visibility_bit_field: u32::from_value(iter.next(), "visibility_bit_field")?,
            follow_ent_index: u32::from_value(iter.next(), "follow_ent_index")?,
            show_distance: bool::from_value(iter.next(), "show_distance")?,
            play_sound: String::from_value(iter.next(), "play_sound")?,
            show_effect: bool::from_value(iter.next(), "show_effect")?,
        })
    }
}
#[derive(Debug)]
pub struct HideAnnotationEvent {
    pub id: u32,
}
impl FromRawGameEvent for HideAnnotationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HideAnnotationEvent {
            id: u32::from_value(iter.next(), "id")?,
        })
    }
}
#[derive(Debug)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PostInventoryApplicationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PostInventoryApplicationEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointUnlockUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl FromRawGameEvent for ControlPointUnlockUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointUnlockUpdatedEvent {
            index: u16::from_value(iter.next(), "index")?,
            time: f32::from_value(iter.next(), "time")?,
        })
    }
}
#[derive(Debug)]
pub struct DeployBuffBannerEvent {
    pub buff_type: u8,
    pub buff_owner: u16,
}
impl FromRawGameEvent for DeployBuffBannerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DeployBuffBannerEvent {
            buff_type: u8::from_value(iter.next(), "buff_type")?,
            buff_owner: u16::from_value(iter.next(), "buff_owner")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerBuffEvent {
    pub user_id: u16,
    pub buff_owner: u16,
    pub buff_type: u8,
}
impl FromRawGameEvent for PlayerBuffEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerBuffEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            buff_owner: u16::from_value(iter.next(), "buff_owner")?,
            buff_type: u8::from_value(iter.next(), "buff_type")?,
        })
    }
}
#[derive(Debug)]
pub struct MedicDeathEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub healing: u16,
    pub charged: bool,
}
impl FromRawGameEvent for MedicDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MedicDeathEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            healing: u16::from_value(iter.next(), "healing")?,
            charged: bool::from_value(iter.next(), "charged")?,
        })
    }
}
#[derive(Debug)]
pub struct OvertimeNagEvent {}
impl FromRawGameEvent for OvertimeNagEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(OvertimeNagEvent {})
    }
}
#[derive(Debug)]
pub struct TeamsChangedEvent {}
impl FromRawGameEvent for TeamsChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TeamsChangedEvent {})
    }
}
#[derive(Debug)]
pub struct HalloweenPumpkinGrabEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for HalloweenPumpkinGrabEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HalloweenPumpkinGrabEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct RocketJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl FromRawGameEvent for RocketJumpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RocketJumpEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            play_sound: bool::from_value(iter.next(), "play_sound")?,
        })
    }
}
#[derive(Debug)]
pub struct RocketJumpLandedEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for RocketJumpLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RocketJumpLandedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct StickyJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl FromRawGameEvent for StickyJumpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(StickyJumpEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            play_sound: bool::from_value(iter.next(), "play_sound")?,
        })
    }
}
#[derive(Debug)]
pub struct StickyJumpLandedEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for StickyJumpLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(StickyJumpLandedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct RocketPackLaunchEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl FromRawGameEvent for RocketPackLaunchEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RocketPackLaunchEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            play_sound: bool::from_value(iter.next(), "play_sound")?,
        })
    }
}
#[derive(Debug)]
pub struct RocketPackLandedEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for RocketPackLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RocketPackLandedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct MedicDefendedEvent {
    pub user_id: u16,
    pub medic: u16,
}
impl FromRawGameEvent for MedicDefendedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MedicDefendedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            medic: u16::from_value(iter.next(), "medic")?,
        })
    }
}
#[derive(Debug)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl FromRawGameEvent for LocalPlayerHealedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(LocalPlayerHealedEvent {
            amount: u16::from_value(iter.next(), "amount")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerDestroyedPipeBombEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDestroyedPipeBombEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct ObjectDeflectedEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub weapon_id: u16,
    pub object_ent_index: u16,
}
impl FromRawGameEvent for ObjectDeflectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ObjectDeflectedEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            owner_id: u16::from_value(iter.next(), "owner_id")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            object_ent_index: u16::from_value(iter.next(), "object_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerMvpEvent {
    pub player: u16,
}
impl FromRawGameEvent for PlayerMvpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerMvpEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct RaidSpawnMobEvent {}
impl FromRawGameEvent for RaidSpawnMobEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RaidSpawnMobEvent {})
    }
}
#[derive(Debug)]
pub struct RaidSpawnSquadEvent {}
impl FromRawGameEvent for RaidSpawnSquadEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RaidSpawnSquadEvent {})
    }
}
#[derive(Debug)]
pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl FromRawGameEvent for NavBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(NavBlockedEvent {
            area: u32::from_value(iter.next(), "area")?,
            blocked: bool::from_value(iter.next(), "blocked")?,
        })
    }
}
#[derive(Debug)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl FromRawGameEvent for PathTrackPassedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PathTrackPassedEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct NumCappersChangedEvent {
    pub index: u16,
    pub count: u8,
}
impl FromRawGameEvent for NumCappersChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(NumCappersChangedEvent {
            index: u16::from_value(iter.next(), "index")?,
            count: u8::from_value(iter.next(), "count")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerRegenerateEvent {}
impl FromRawGameEvent for PlayerRegenerateEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PlayerRegenerateEvent {})
    }
}
#[derive(Debug)]
pub struct UpdateStatusItemEvent {
    pub index: u8,
    pub object: u8,
}
impl FromRawGameEvent for UpdateStatusItemEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(UpdateStatusItemEvent {
            index: u8::from_value(iter.next(), "index")?,
            object: u8::from_value(iter.next(), "object")?,
        })
    }
}
#[derive(Debug)]
pub struct StatsResetRoundEvent {}
impl FromRawGameEvent for StatsResetRoundEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(StatsResetRoundEvent {})
    }
}
#[derive(Debug)]
pub struct ScoreStatsAccumulatedUpdateEvent {}
impl FromRawGameEvent for ScoreStatsAccumulatedUpdateEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ScoreStatsAccumulatedUpdateEvent {})
    }
}
#[derive(Debug)]
pub struct ScoreStatsAccumulatedResetEvent {}
impl FromRawGameEvent for ScoreStatsAccumulatedResetEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ScoreStatsAccumulatedResetEvent {})
    }
}
#[derive(Debug)]
pub struct AchievementEarnedLocalEvent {
    pub achievement: u16,
}
impl FromRawGameEvent for AchievementEarnedLocalEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(AchievementEarnedLocalEvent {
            achievement: u16::from_value(iter.next(), "achievement")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHealedEvent {
    pub patient: u16,
    pub healer: u16,
    pub amount: u16,
}
impl FromRawGameEvent for PlayerHealedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHealedEvent {
            patient: u16::from_value(iter.next(), "patient")?,
            healer: u16::from_value(iter.next(), "healer")?,
            amount: u16::from_value(iter.next(), "amount")?,
        })
    }
}
#[derive(Debug)]
pub struct BuildingHealedEvent {
    pub building: u16,
    pub healer: u16,
    pub amount: u16,
}
impl FromRawGameEvent for BuildingHealedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(BuildingHealedEvent {
            building: u16::from_value(iter.next(), "building")?,
            healer: u16::from_value(iter.next(), "healer")?,
            amount: u16::from_value(iter.next(), "amount")?,
        })
    }
}
#[derive(Debug)]
pub struct ItemPickupEvent {
    pub user_id: u16,
    pub item: String,
}
impl FromRawGameEvent for ItemPickupEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ItemPickupEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            item: String::from_value(iter.next(), "item")?,
        })
    }
}
#[derive(Debug)]
pub struct DuelStatusEvent {
    pub killer: u16,
    pub score_type: u16,
    pub initiator: u16,
    pub target: u16,
    pub initiator_score: u16,
    pub target_score: u16,
}
impl FromRawGameEvent for DuelStatusEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DuelStatusEvent {
            killer: u16::from_value(iter.next(), "killer")?,
            score_type: u16::from_value(iter.next(), "score_type")?,
            initiator: u16::from_value(iter.next(), "initiator")?,
            target: u16::from_value(iter.next(), "target")?,
            initiator_score: u16::from_value(iter.next(), "initiator_score")?,
            target_score: u16::from_value(iter.next(), "target_score")?,
        })
    }
}
#[derive(Debug)]
pub struct FishNoticeEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
}
impl FromRawGameEvent for FishNoticeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(FishNoticeEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
            stun_flags: u16::from_value(iter.next(), "stun_flags")?,
            death_flags: u16::from_value(iter.next(), "death_flags")?,
            silent_kill: bool::from_value(iter.next(), "silent_kill")?,
            assister_fallback: String::from_value(iter.next(), "assister_fallback")?,
        })
    }
}
#[derive(Debug)]
pub struct FishNoticeArmEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
}
impl FromRawGameEvent for FishNoticeArmEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(FishNoticeArmEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
            stun_flags: u16::from_value(iter.next(), "stun_flags")?,
            death_flags: u16::from_value(iter.next(), "death_flags")?,
            silent_kill: bool::from_value(iter.next(), "silent_kill")?,
            assister_fallback: String::from_value(iter.next(), "assister_fallback")?,
        })
    }
}
#[derive(Debug)]
pub struct SlapNoticeEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
}
impl FromRawGameEvent for SlapNoticeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(SlapNoticeEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
            stun_flags: u16::from_value(iter.next(), "stun_flags")?,
            death_flags: u16::from_value(iter.next(), "death_flags")?,
            silent_kill: bool::from_value(iter.next(), "silent_kill")?,
            assister_fallback: String::from_value(iter.next(), "assister_fallback")?,
        })
    }
}
#[derive(Debug)]
pub struct ThrowableHitEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
    pub total_hits: u16,
}
impl FromRawGameEvent for ThrowableHitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ThrowableHitEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            assister: u16::from_value(iter.next(), "assister")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
            stun_flags: u16::from_value(iter.next(), "stun_flags")?,
            death_flags: u16::from_value(iter.next(), "death_flags")?,
            silent_kill: bool::from_value(iter.next(), "silent_kill")?,
            assister_fallback: String::from_value(iter.next(), "assister_fallback")?,
            total_hits: u16::from_value(iter.next(), "total_hits")?,
        })
    }
}
#[derive(Debug)]
pub struct PumpkinLordSummonedEvent {}
impl FromRawGameEvent for PumpkinLordSummonedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PumpkinLordSummonedEvent {})
    }
}
#[derive(Debug)]
pub struct PumpkinLordKilledEvent {}
impl FromRawGameEvent for PumpkinLordKilledEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PumpkinLordKilledEvent {})
    }
}
#[derive(Debug)]
pub struct MerasmusSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusSummonedEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusKilledEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct MerasmusEscapeWarningEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl FromRawGameEvent for MerasmusEscapeWarningEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusEscapeWarningEvent {
            level: u16::from_value(iter.next(), "level")?,
            time_remaining: u8::from_value(iter.next(), "time_remaining")?,
        })
    }
}
#[derive(Debug)]
pub struct MerasmusEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusEscapedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusEscapedEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossSummonedEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossStunnedEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl FromRawGameEvent for EyeballBossStunnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossStunnedEvent {
            level: u16::from_value(iter.next(), "level")?,
            player_ent_index: u8::from_value(iter.next(), "player_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossKilledEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossKillerEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl FromRawGameEvent for EyeballBossKillerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossKillerEvent {
            level: u16::from_value(iter.next(), "level")?,
            player_ent_index: u8::from_value(iter.next(), "player_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossEscapeImminentEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl FromRawGameEvent for EyeballBossEscapeImminentEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossEscapeImminentEvent {
            level: u16::from_value(iter.next(), "level")?,
            time_remaining: u8::from_value(iter.next(), "time_remaining")?,
        })
    }
}
#[derive(Debug)]
pub struct EyeballBossEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossEscapedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EyeballBossEscapedEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct NpcHurtEvent {
    pub ent_index: u16,
    pub health: u16,
    pub attacker_player: u16,
    pub weapon_id: u16,
    pub damage_amount: u16,
    pub crit: bool,
    pub boss: u16,
}
impl FromRawGameEvent for NpcHurtEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(NpcHurtEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
            health: u16::from_value(iter.next(), "health")?,
            attacker_player: u16::from_value(iter.next(), "attacker_player")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_amount: u16::from_value(iter.next(), "damage_amount")?,
            crit: bool::from_value(iter.next(), "crit")?,
            boss: u16::from_value(iter.next(), "boss")?,
        })
    }
}
#[derive(Debug)]
pub struct ControlPointTimerUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl FromRawGameEvent for ControlPointTimerUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ControlPointTimerUpdatedEvent {
            index: u16::from_value(iter.next(), "index")?,
            time: f32::from_value(iter.next(), "time")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHighFiveStartEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHighFiveStartEvent {
            ent_index: u8::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHighFiveCancelEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveCancelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHighFiveCancelEvent {
            ent_index: u8::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerHighFiveSuccessEvent {
    pub initiator_ent_index: u8,
    pub partner_ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveSuccessEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerHighFiveSuccessEvent {
            initiator_ent_index: u8::from_value(iter.next(), "initiator_ent_index")?,
            partner_ent_index: u8::from_value(iter.next(), "partner_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerBonusPointsEvent {
    pub points: u16,
    pub player_ent_index: u16,
    pub source_ent_index: u16,
}
impl FromRawGameEvent for PlayerBonusPointsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerBonusPointsEvent {
            points: u16::from_value(iter.next(), "points")?,
            player_ent_index: u16::from_value(iter.next(), "player_ent_index")?,
            source_ent_index: u16::from_value(iter.next(), "source_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerUpgradedEvent {}
impl FromRawGameEvent for PlayerUpgradedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PlayerUpgradedEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerBuybackEvent {
    pub player: u16,
    pub cost: u16,
}
impl FromRawGameEvent for PlayerBuybackEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerBuybackEvent {
            player: u16::from_value(iter.next(), "player")?,
            cost: u16::from_value(iter.next(), "cost")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerUsedPowerUpBottleEvent {
    pub player: u16,
    pub kind: u16,
    pub time: f32,
}
impl FromRawGameEvent for PlayerUsedPowerUpBottleEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerUsedPowerUpBottleEvent {
            player: u16::from_value(iter.next(), "player")?,
            kind: u16::from_value(iter.next(), "kind")?,
            time: f32::from_value(iter.next(), "time")?,
        })
    }
}
#[derive(Debug)]
pub struct ChristmasGiftGrabEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for ChristmasGiftGrabEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ChristmasGiftGrabEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerKilledAchievementZoneEvent {
    pub attacker: u16,
    pub victim: u16,
    pub zone_id: u16,
}
impl FromRawGameEvent for PlayerKilledAchievementZoneEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerKilledAchievementZoneEvent {
            attacker: u16::from_value(iter.next(), "attacker")?,
            victim: u16::from_value(iter.next(), "victim")?,
            zone_id: u16::from_value(iter.next(), "zone_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PartyUpdatedEvent {}
impl FromRawGameEvent for PartyUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PartyUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct PartyPrefChangedEvent {}
impl FromRawGameEvent for PartyPrefChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PartyPrefChangedEvent {})
    }
}
#[derive(Debug)]
pub struct PartyCriteriaChangedEvent {}
impl FromRawGameEvent for PartyCriteriaChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PartyCriteriaChangedEvent {})
    }
}
#[derive(Debug)]
pub struct PartyInvitesChangedEvent {}
impl FromRawGameEvent for PartyInvitesChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PartyInvitesChangedEvent {})
    }
}
#[derive(Debug)]
pub struct PartyQueueStateChangedEvent {
    pub match_group: u16,
}
impl FromRawGameEvent for PartyQueueStateChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PartyQueueStateChangedEvent {
            match_group: u16::from_value(iter.next(), "match_group")?,
        })
    }
}
#[derive(Debug)]
pub struct PartyChatEvent {
    pub steam_id: String,
    pub text: String,
    pub kind: u16,
}
impl FromRawGameEvent for PartyChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PartyChatEvent {
            steam_id: String::from_value(iter.next(), "steam_id")?,
            text: String::from_value(iter.next(), "text")?,
            kind: u16::from_value(iter.next(), "kind")?,
        })
    }
}
#[derive(Debug)]
pub struct PartyMemberJoinEvent {
    pub steam_id: String,
}
impl FromRawGameEvent for PartyMemberJoinEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PartyMemberJoinEvent {
            steam_id: String::from_value(iter.next(), "steam_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PartyMemberLeaveEvent {
    pub steam_id: String,
}
impl FromRawGameEvent for PartyMemberLeaveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PartyMemberLeaveEvent {
            steam_id: String::from_value(iter.next(), "steam_id")?,
        })
    }
}
#[derive(Debug)]
pub struct MatchInvitesUpdatedEvent {}
impl FromRawGameEvent for MatchInvitesUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MatchInvitesUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct LobbyUpdatedEvent {}
impl FromRawGameEvent for LobbyUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LobbyUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct MvmMissionUpdateEvent {
    pub class: u16,
    pub count: u16,
}
impl FromRawGameEvent for MvmMissionUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmMissionUpdateEvent {
            class: u16::from_value(iter.next(), "class")?,
            count: u16::from_value(iter.next(), "count")?,
        })
    }
}
#[derive(Debug)]
pub struct RecalculateHolidaysEvent {}
impl FromRawGameEvent for RecalculateHolidaysEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RecalculateHolidaysEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerCurrencyChangedEvent {
    pub currency: u16,
}
impl FromRawGameEvent for PlayerCurrencyChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerCurrencyChangedEvent {
            currency: u16::from_value(iter.next(), "currency")?,
        })
    }
}
#[derive(Debug)]
pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl FromRawGameEvent for DoomsdayRocketOpenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DoomsdayRocketOpenEvent {
            team: u8::from_value(iter.next(), "team")?,
        })
    }
}
#[derive(Debug)]
pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl FromRawGameEvent for RemoveNemesisRelationshipsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RemoveNemesisRelationshipsEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmCreditBonusWaveEvent {}
impl FromRawGameEvent for MvmCreditBonusWaveEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmCreditBonusWaveEvent {})
    }
}
#[derive(Debug)]
pub struct MvmCreditBonusAllEvent {}
impl FromRawGameEvent for MvmCreditBonusAllEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmCreditBonusAllEvent {})
    }
}
#[derive(Debug)]
pub struct MvmCreditBonusAllAdvancedEvent {}
impl FromRawGameEvent for MvmCreditBonusAllAdvancedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmCreditBonusAllAdvancedEvent {})
    }
}
#[derive(Debug)]
pub struct MvmQuickSentryUpgradeEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmQuickSentryUpgradeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmQuickSentryUpgradeEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmTankDestroyedByPlayersEvent {}
impl FromRawGameEvent for MvmTankDestroyedByPlayersEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmTankDestroyedByPlayersEvent {})
    }
}
#[derive(Debug)]
pub struct MvmKillRobotDeliveringBombEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmKillRobotDeliveringBombEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmKillRobotDeliveringBombEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmPickupCurrencyEvent {
    pub player: u16,
    pub currency: u16,
}
impl FromRawGameEvent for MvmPickupCurrencyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmPickupCurrencyEvent {
            player: u16::from_value(iter.next(), "player")?,
            currency: u16::from_value(iter.next(), "currency")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MvmBombCarrierKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmBombCarrierKilledEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmSentryBusterDetonateEvent {
    pub player: u16,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}
impl FromRawGameEvent for MvmSentryBusterDetonateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmSentryBusterDetonateEvent {
            player: u16::from_value(iter.next(), "player")?,
            det_x: f32::from_value(iter.next(), "det_x")?,
            det_y: f32::from_value(iter.next(), "det_y")?,
            det_z: f32::from_value(iter.next(), "det_z")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmScoutMarkedForDeathEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmScoutMarkedForDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmScoutMarkedForDeathEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmMedicPowerUpSharedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmMedicPowerUpSharedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmMedicPowerUpSharedEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmBeginWaveEvent {
    pub wave_index: u16,
    pub max_waves: u16,
    pub advanced: u16,
}
impl FromRawGameEvent for MvmBeginWaveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmBeginWaveEvent {
            wave_index: u16::from_value(iter.next(), "wave_index")?,
            max_waves: u16::from_value(iter.next(), "max_waves")?,
            advanced: u16::from_value(iter.next(), "advanced")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmWaveCompleteEvent {
    pub advanced: bool,
}
impl FromRawGameEvent for MvmWaveCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmWaveCompleteEvent {
            advanced: bool::from_value(iter.next(), "advanced")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmMissionCompleteEvent {
    pub mission: String,
}
impl FromRawGameEvent for MvmMissionCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmMissionCompleteEvent {
            mission: String::from_value(iter.next(), "mission")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombResetByPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmBombResetByPlayerEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmBombAlarmTriggeredEvent {}
impl FromRawGameEvent for MvmBombAlarmTriggeredEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmBombAlarmTriggeredEvent {})
    }
}
#[derive(Debug)]
pub struct MvmBombDeployResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombDeployResetByPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmBombDeployResetByPlayerEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmWaveFailedEvent {}
impl FromRawGameEvent for MvmWaveFailedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmWaveFailedEvent {})
    }
}
#[derive(Debug)]
pub struct MvmResetStatsEvent {}
impl FromRawGameEvent for MvmResetStatsEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmResetStatsEvent {})
    }
}
#[derive(Debug)]
pub struct DamageResistedEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for DamageResistedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DamageResistedEvent {
            ent_index: u8::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct RevivePlayerNotifyEvent {
    pub ent_index: u16,
    pub marker_ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerNotifyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RevivePlayerNotifyEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
            marker_ent_index: u16::from_value(iter.next(), "marker_ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct RevivePlayerStoppedEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerStoppedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RevivePlayerStoppedEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct RevivePlayerCompleteEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RevivePlayerCompleteEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerTurnedToGhostEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerTurnedToGhostEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerTurnedToGhostEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct MedigunShieldBlockedDamageEvent {
    pub user_id: u16,
    pub damage: f32,
}
impl FromRawGameEvent for MedigunShieldBlockedDamageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MedigunShieldBlockedDamageEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            damage: f32::from_value(iter.next(), "damage")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl FromRawGameEvent for MvmAdvWaveCompleteNoGatesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmAdvWaveCompleteNoGatesEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmSniperHeadshotCurrencyEvent {
    pub user_id: u16,
    pub currency: u16,
}
impl FromRawGameEvent for MvmSniperHeadshotCurrencyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmSniperHeadshotCurrencyEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            currency: u16::from_value(iter.next(), "currency")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmMannhattanPitEvent {}
impl FromRawGameEvent for MvmMannhattanPitEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmMannhattanPitEvent {})
    }
}
#[derive(Debug)]
pub struct FlagCarriedInDetectionZoneEvent {}
impl FromRawGameEvent for FlagCarriedInDetectionZoneEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(FlagCarriedInDetectionZoneEvent {})
    }
}
#[derive(Debug)]
pub struct MvmAdvWaveKilledStunRadioEvent {}
impl FromRawGameEvent for MvmAdvWaveKilledStunRadioEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MvmAdvWaveKilledStunRadioEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerDirectHitStunEvent {
    pub attacker: u16,
    pub victim: u16,
}
impl FromRawGameEvent for PlayerDirectHitStunEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDirectHitStunEvent {
            attacker: u16::from_value(iter.next(), "attacker")?,
            victim: u16::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl FromRawGameEvent for MvmSentryBusterKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MvmSentryBusterKilledEvent {
            sentry_buster: u16::from_value(iter.next(), "sentry_buster")?,
        })
    }
}
#[derive(Debug)]
pub struct UpgradesFileChangedEvent {
    pub path: String,
}
impl FromRawGameEvent for UpgradesFileChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(UpgradesFileChangedEvent {
            path: String::from_value(iter.next(), "path")?,
        })
    }
}
#[derive(Debug)]
pub struct RdTeamPointsChangedEvent {
    pub points: u16,
    pub team: u8,
    pub method: u8,
}
impl FromRawGameEvent for RdTeamPointsChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RdTeamPointsChangedEvent {
            points: u16::from_value(iter.next(), "points")?,
            team: u8::from_value(iter.next(), "team")?,
            method: u8::from_value(iter.next(), "method")?,
        })
    }
}
#[derive(Debug)]
pub struct RdRulesStateChangedEvent {}
impl FromRawGameEvent for RdRulesStateChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RdRulesStateChangedEvent {})
    }
}
#[derive(Debug)]
pub struct RdRobotKilledEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub weapon_log_class_name: String,
}
impl FromRawGameEvent for RdRobotKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RdRobotKilledEvent {
            user_id: u16::from_value(iter.next(), "user_id")?,
            victim_ent_index: u32::from_value(iter.next(), "victim_ent_index")?,
            inflictor_ent_index: u32::from_value(iter.next(), "inflictor_ent_index")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
            weapon: String::from_value(iter.next(), "weapon")?,
            weapon_id: u16::from_value(iter.next(), "weapon_id")?,
            damage_bits: u32::from_value(iter.next(), "damage_bits")?,
            custom_kill: u16::from_value(iter.next(), "custom_kill")?,
            weapon_log_class_name: String::from_value(iter.next(), "weapon_log_class_name")?,
        })
    }
}
#[derive(Debug)]
pub struct RdRobotImpactEvent {
    pub ent_index: u16,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}
impl FromRawGameEvent for RdRobotImpactEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RdRobotImpactEvent {
            ent_index: u16::from_value(iter.next(), "ent_index")?,
            impulse_x: f32::from_value(iter.next(), "impulse_x")?,
            impulse_y: f32::from_value(iter.next(), "impulse_y")?,
            impulse_z: f32::from_value(iter.next(), "impulse_z")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamPlayPreRoundTimeLeftEvent {
    pub time: u16,
}
impl FromRawGameEvent for TeamPlayPreRoundTimeLeftEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamPlayPreRoundTimeLeftEvent {
            time: u16::from_value(iter.next(), "time")?,
        })
    }
}
#[derive(Debug)]
pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteDeployEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ParachuteDeployEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteHolsterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ParachuteHolsterEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl FromRawGameEvent for KillRefillsMeterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(KillRefillsMeterEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct RpsTauntEventEvent {
    pub winner: u16,
    pub winner_rps: u8,
    pub loser: u16,
    pub loser_rps: u8,
}
impl FromRawGameEvent for RpsTauntEventEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RpsTauntEventEvent {
            winner: u16::from_value(iter.next(), "winner")?,
            winner_rps: u8::from_value(iter.next(), "winner_rps")?,
            loser: u16::from_value(iter.next(), "loser")?,
            loser_rps: u8::from_value(iter.next(), "loser_rps")?,
        })
    }
}
#[derive(Debug)]
pub struct CongaKillEvent {
    pub index: u16,
}
impl FromRawGameEvent for CongaKillEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CongaKillEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl FromRawGameEvent for PlayerInitialSpawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerInitialSpawnEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct CompetitiveVictoryEvent {}
impl FromRawGameEvent for CompetitiveVictoryEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(CompetitiveVictoryEvent {})
    }
}
#[derive(Debug)]
pub struct CompetitiveStatsUpdateEvent {
    pub index: u16,
    pub kills_rank: u8,
    pub score_rank: u8,
    pub damage_rank: u8,
    pub healing_rank: u8,
    pub support_rank: u8,
}
impl FromRawGameEvent for CompetitiveStatsUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CompetitiveStatsUpdateEvent {
            index: u16::from_value(iter.next(), "index")?,
            kills_rank: u8::from_value(iter.next(), "kills_rank")?,
            score_rank: u8::from_value(iter.next(), "score_rank")?,
            damage_rank: u8::from_value(iter.next(), "damage_rank")?,
            healing_rank: u8::from_value(iter.next(), "healing_rank")?,
            support_rank: u8::from_value(iter.next(), "support_rank")?,
        })
    }
}
#[derive(Debug)]
pub struct MiniGameWinEvent {
    pub team: u8,
    pub kind: u8,
}
impl FromRawGameEvent for MiniGameWinEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MiniGameWinEvent {
            team: u8::from_value(iter.next(), "team")?,
            kind: u8::from_value(iter.next(), "kind")?,
        })
    }
}
#[derive(Debug)]
pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl FromRawGameEvent for SentryOnGoActiveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(SentryOnGoActiveEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl FromRawGameEvent for DuckXpLevelUpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DuckXpLevelUpEvent {
            level: u16::from_value(iter.next(), "level")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestLogOpenedEvent {}
impl FromRawGameEvent for QuestLogOpenedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(QuestLogOpenedEvent {})
    }
}
#[derive(Debug)]
pub struct SchemaUpdatedEvent {}
impl FromRawGameEvent for SchemaUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(SchemaUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct LocalPlayerPickupWeaponEvent {}
impl FromRawGameEvent for LocalPlayerPickupWeaponEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(LocalPlayerPickupWeaponEvent {})
    }
}
#[derive(Debug)]
pub struct RdPlayerScorePointsEvent {
    pub player: u16,
    pub method: u16,
    pub amount: u16,
}
impl FromRawGameEvent for RdPlayerScorePointsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RdPlayerScorePointsEvent {
            player: u16::from_value(iter.next(), "player")?,
            method: u16::from_value(iter.next(), "method")?,
            amount: u16::from_value(iter.next(), "amount")?,
        })
    }
}
#[derive(Debug)]
pub struct DemomanDetStickiesEvent {
    pub player: u16,
}
impl FromRawGameEvent for DemomanDetStickiesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DemomanDetStickiesEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestObjectiveCompletedEvent {
    pub quest_item_id_low: u32,
    pub quest_item_id_hi: u32,
    pub quest_objective_id: u32,
    pub scorer_user_id: u16,
}
impl FromRawGameEvent for QuestObjectiveCompletedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(QuestObjectiveCompletedEvent {
            quest_item_id_low: u32::from_value(iter.next(), "quest_item_id_low")?,
            quest_item_id_hi: u32::from_value(iter.next(), "quest_item_id_hi")?,
            quest_objective_id: u32::from_value(iter.next(), "quest_objective_id")?,
            scorer_user_id: u16::from_value(iter.next(), "scorer_user_id")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerScoreChangedEvent {
    pub player: u8,
    pub delta: u16,
}
impl FromRawGameEvent for PlayerScoreChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerScoreChangedEvent {
            player: u8::from_value(iter.next(), "player")?,
            delta: u16::from_value(iter.next(), "delta")?,
        })
    }
}
#[derive(Debug)]
pub struct KilledCappingPlayerEvent {
    pub cp: u8,
    pub killer: u8,
    pub victim: u8,
    pub assister: u8,
}
impl FromRawGameEvent for KilledCappingPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(KilledCappingPlayerEvent {
            cp: u8::from_value(iter.next(), "cp")?,
            killer: u8::from_value(iter.next(), "killer")?,
            victim: u8::from_value(iter.next(), "victim")?,
            assister: u8::from_value(iter.next(), "assister")?,
        })
    }
}
#[derive(Debug)]
pub struct EnvironmentalDeathEvent {
    pub killer: u8,
    pub victim: u8,
}
impl FromRawGameEvent for EnvironmentalDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EnvironmentalDeathEvent {
            killer: u8::from_value(iter.next(), "killer")?,
            victim: u8::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct ProjectileDirectHitEvent {
    pub attacker: u8,
    pub victim: u8,
    pub weapon_def_index: u32,
}
impl FromRawGameEvent for ProjectileDirectHitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ProjectileDirectHitEvent {
            attacker: u8::from_value(iter.next(), "attacker")?,
            victim: u8::from_value(iter.next(), "victim")?,
            weapon_def_index: u32::from_value(iter.next(), "weapon_def_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PassGetEvent {
    pub owner: u16,
}
impl FromRawGameEvent for PassGetEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassGetEvent {
            owner: u16::from_value(iter.next(), "owner")?,
        })
    }
}
#[derive(Debug)]
pub struct PassScoreEvent {
    pub scorer: u16,
    pub assister: u16,
    pub points: u8,
}
impl FromRawGameEvent for PassScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassScoreEvent {
            scorer: u16::from_value(iter.next(), "scorer")?,
            assister: u16::from_value(iter.next(), "assister")?,
            points: u8::from_value(iter.next(), "points")?,
        })
    }
}
#[derive(Debug)]
pub struct PassFreeEvent {
    pub owner: u16,
    pub attacker: u16,
}
impl FromRawGameEvent for PassFreeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassFreeEvent {
            owner: u16::from_value(iter.next(), "owner")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
        })
    }
}
#[derive(Debug)]
pub struct PassPassCaughtEvent {
    pub passer: u16,
    pub catcher: u16,
    pub dist: f32,
    pub duration: f32,
}
impl FromRawGameEvent for PassPassCaughtEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassPassCaughtEvent {
            passer: u16::from_value(iter.next(), "passer")?,
            catcher: u16::from_value(iter.next(), "catcher")?,
            dist: f32::from_value(iter.next(), "dist")?,
            duration: f32::from_value(iter.next(), "duration")?,
        })
    }
}
#[derive(Debug)]
pub struct PassBallStolenEvent {
    pub victim: u16,
    pub attacker: u16,
}
impl FromRawGameEvent for PassBallStolenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassBallStolenEvent {
            victim: u16::from_value(iter.next(), "victim")?,
            attacker: u16::from_value(iter.next(), "attacker")?,
        })
    }
}
#[derive(Debug)]
pub struct PassBallBlockedEvent {
    pub owner: u16,
    pub blocker: u16,
}
impl FromRawGameEvent for PassBallBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PassBallBlockedEvent {
            owner: u16::from_value(iter.next(), "owner")?,
            blocker: u16::from_value(iter.next(), "blocker")?,
        })
    }
}
#[derive(Debug)]
pub struct DamagePreventedEvent {
    pub preventor: u16,
    pub victim: u16,
    pub amount: u16,
    pub condition: u16,
}
impl FromRawGameEvent for DamagePreventedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DamagePreventedEvent {
            preventor: u16::from_value(iter.next(), "preventor")?,
            victim: u16::from_value(iter.next(), "victim")?,
            amount: u16::from_value(iter.next(), "amount")?,
            condition: u16::from_value(iter.next(), "condition")?,
        })
    }
}
#[derive(Debug)]
pub struct HalloweenBossKilledEvent {
    pub boss: u16,
    pub killer: u16,
}
impl FromRawGameEvent for HalloweenBossKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HalloweenBossKilledEvent {
            boss: u16::from_value(iter.next(), "boss")?,
            killer: u16::from_value(iter.next(), "killer")?,
        })
    }
}
#[derive(Debug)]
pub struct EscapedLootIslandEvent {
    pub player: u16,
}
impl FromRawGameEvent for EscapedLootIslandEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EscapedLootIslandEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct TaggedPlayerAsItEvent {
    pub player: u16,
}
impl FromRawGameEvent for TaggedPlayerAsItEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TaggedPlayerAsItEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MerasmusStunnedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MerasmusStunnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusStunnedEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MerasmusPropFoundEvent {
    pub player: u16,
}
impl FromRawGameEvent for MerasmusPropFoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MerasmusPropFoundEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct HalloweenSkeletonKilledEvent {
    pub player: u16,
}
impl FromRawGameEvent for HalloweenSkeletonKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HalloweenSkeletonKilledEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct EscapeHellEvent {
    pub player: u16,
}
impl FromRawGameEvent for EscapeHellEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(EscapeHellEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct CrossSpectralBridgeEvent {
    pub player: u16,
}
impl FromRawGameEvent for CrossSpectralBridgeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CrossSpectralBridgeEvent {
            player: u16::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct MiniGameWonEvent {
    pub player: u16,
    pub game: u16,
}
impl FromRawGameEvent for MiniGameWonEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(MiniGameWonEvent {
            player: u16::from_value(iter.next(), "player")?,
            game: u16::from_value(iter.next(), "game")?,
        })
    }
}
#[derive(Debug)]
pub struct RespawnGhostEvent {
    pub reviver: u16,
    pub ghost: u16,
}
impl FromRawGameEvent for RespawnGhostEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RespawnGhostEvent {
            reviver: u16::from_value(iter.next(), "reviver")?,
            ghost: u16::from_value(iter.next(), "ghost")?,
        })
    }
}
#[derive(Debug)]
pub struct KillInHellEvent {
    pub killer: u16,
    pub victim: u16,
}
impl FromRawGameEvent for KillInHellEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(KillInHellEvent {
            killer: u16::from_value(iter.next(), "killer")?,
            victim: u16::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct HalloweenDuckCollectedEvent {
    pub collector: u16,
}
impl FromRawGameEvent for HalloweenDuckCollectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HalloweenDuckCollectedEvent {
            collector: u16::from_value(iter.next(), "collector")?,
        })
    }
}
#[derive(Debug)]
pub struct SpecialScoreEvent {
    pub player: u8,
}
impl FromRawGameEvent for SpecialScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(SpecialScoreEvent {
            player: u8::from_value(iter.next(), "player")?,
        })
    }
}
#[derive(Debug)]
pub struct TeamLeaderKilledEvent {
    pub killer: u8,
    pub victim: u8,
}
impl FromRawGameEvent for TeamLeaderKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(TeamLeaderKilledEvent {
            killer: u8::from_value(iter.next(), "killer")?,
            victim: u8::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct HalloweenSoulCollectedEvent {
    pub intended_target: u8,
    pub collecting_player: u8,
    pub soul_count: u8,
}
impl FromRawGameEvent for HalloweenSoulCollectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HalloweenSoulCollectedEvent {
            intended_target: u8::from_value(iter.next(), "intended_target")?,
            collecting_player: u8::from_value(iter.next(), "collecting_player")?,
            soul_count: u8::from_value(iter.next(), "soul_count")?,
        })
    }
}
#[derive(Debug)]
pub struct RecalculateTruceEvent {}
impl FromRawGameEvent for RecalculateTruceEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RecalculateTruceEvent {})
    }
}
#[derive(Debug)]
pub struct DeadRingerCheatDeathEvent {
    pub spy: u8,
    pub attacker: u8,
}
impl FromRawGameEvent for DeadRingerCheatDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DeadRingerCheatDeathEvent {
            spy: u8::from_value(iter.next(), "spy")?,
            attacker: u8::from_value(iter.next(), "attacker")?,
        })
    }
}
#[derive(Debug)]
pub struct CrossbowHealEvent {
    pub healer: u8,
    pub target: u8,
    pub amount: u16,
}
impl FromRawGameEvent for CrossbowHealEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CrossbowHealEvent {
            healer: u8::from_value(iter.next(), "healer")?,
            target: u8::from_value(iter.next(), "target")?,
            amount: u16::from_value(iter.next(), "amount")?,
        })
    }
}
#[derive(Debug)]
pub struct DamageMitigatedEvent {
    pub mitigator: u8,
    pub damaged: u8,
    pub amount: u16,
    pub item_definition_index: u16,
}
impl FromRawGameEvent for DamageMitigatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DamageMitigatedEvent {
            mitigator: u8::from_value(iter.next(), "mitigator")?,
            damaged: u8::from_value(iter.next(), "damaged")?,
            amount: u16::from_value(iter.next(), "amount")?,
            item_definition_index: u16::from_value(iter.next(), "item_definition_index")?,
        })
    }
}
#[derive(Debug)]
pub struct PayloadPushedEvent {
    pub pusher: u8,
    pub distance: u16,
}
impl FromRawGameEvent for PayloadPushedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PayloadPushedEvent {
            pusher: u8::from_value(iter.next(), "pusher")?,
            distance: u16::from_value(iter.next(), "distance")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerAbandonedMatchEvent {
    pub game_over: bool,
}
impl FromRawGameEvent for PlayerAbandonedMatchEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerAbandonedMatchEvent {
            game_over: bool::from_value(iter.next(), "game_over")?,
        })
    }
}
#[derive(Debug)]
pub struct ClDrawlineEvent {
    pub player: u8,
    pub panel: u8,
    pub line: u8,
    pub x: f32,
    pub y: f32,
}
impl FromRawGameEvent for ClDrawlineEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ClDrawlineEvent {
            player: u8::from_value(iter.next(), "player")?,
            panel: u8::from_value(iter.next(), "panel")?,
            line: u8::from_value(iter.next(), "line")?,
            x: f32::from_value(iter.next(), "x")?,
            y: f32::from_value(iter.next(), "y")?,
        })
    }
}
#[derive(Debug)]
pub struct RestartTimerTimeEvent {
    pub time: u8,
}
impl FromRawGameEvent for RestartTimerTimeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RestartTimerTimeEvent {
            time: u8::from_value(iter.next(), "time")?,
        })
    }
}
#[derive(Debug)]
pub struct WinLimitChangedEvent {}
impl FromRawGameEvent for WinLimitChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(WinLimitChangedEvent {})
    }
}
#[derive(Debug)]
pub struct WinPanelShowScoresEvent {}
impl FromRawGameEvent for WinPanelShowScoresEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(WinPanelShowScoresEvent {})
    }
}
#[derive(Debug)]
pub struct TopStreamsRequestFinishedEvent {}
impl FromRawGameEvent for TopStreamsRequestFinishedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(TopStreamsRequestFinishedEvent {})
    }
}
#[derive(Debug)]
pub struct CompetitiveStateChangedEvent {}
impl FromRawGameEvent for CompetitiveStateChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(CompetitiveStateChangedEvent {})
    }
}
#[derive(Debug)]
pub struct GlobalWarDataUpdatedEvent {}
impl FromRawGameEvent for GlobalWarDataUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(GlobalWarDataUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct StopWatchChangedEvent {}
impl FromRawGameEvent for StopWatchChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(StopWatchChangedEvent {})
    }
}
#[derive(Debug)]
pub struct DsStopEvent {}
impl FromRawGameEvent for DsStopEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(DsStopEvent {})
    }
}
#[derive(Debug)]
pub struct DsScreenshotEvent {
    pub delay: f32,
}
impl FromRawGameEvent for DsScreenshotEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(DsScreenshotEvent {
            delay: f32::from_value(iter.next(), "delay")?,
        })
    }
}
#[derive(Debug)]
pub struct ShowMatchSummaryEvent {}
impl FromRawGameEvent for ShowMatchSummaryEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ShowMatchSummaryEvent {})
    }
}
#[derive(Debug)]
pub struct ExperienceChangedEvent {}
impl FromRawGameEvent for ExperienceChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ExperienceChangedEvent {})
    }
}
#[derive(Debug)]
pub struct BeginXpLerpEvent {}
impl FromRawGameEvent for BeginXpLerpEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(BeginXpLerpEvent {})
    }
}
#[derive(Debug)]
pub struct MatchmakerStatsUpdatedEvent {}
impl FromRawGameEvent for MatchmakerStatsUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MatchmakerStatsUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct RematchVotePeriodOverEvent {
    pub success: bool,
}
impl FromRawGameEvent for RematchVotePeriodOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(RematchVotePeriodOverEvent {
            success: bool::from_value(iter.next(), "success")?,
        })
    }
}
#[derive(Debug)]
pub struct RematchFailedToCreateEvent {}
impl FromRawGameEvent for RematchFailedToCreateEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(RematchFailedToCreateEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerRematchChangeEvent {}
impl FromRawGameEvent for PlayerRematchChangeEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PlayerRematchChangeEvent {})
    }
}
#[derive(Debug)]
pub struct PingUpdatedEvent {}
impl FromRawGameEvent for PingUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(PingUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct MMStatsUpdatedEvent {}
impl FromRawGameEvent for MMStatsUpdatedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MMStatsUpdatedEvent {})
    }
}
#[derive(Debug)]
pub struct PlayerNextMapVoteChangeEvent {
    pub map_index: u8,
    pub vote: u8,
}
impl FromRawGameEvent for PlayerNextMapVoteChangeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerNextMapVoteChangeEvent {
            map_index: u8::from_value(iter.next(), "map_index")?,
            vote: u8::from_value(iter.next(), "vote")?,
        })
    }
}
#[derive(Debug)]
pub struct VoteMapsChangedEvent {}
impl FromRawGameEvent for VoteMapsChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(VoteMapsChangedEvent {})
    }
}
#[derive(Debug)]
pub struct ProtoDefChangedEvent {
    pub kind: u8,
    pub definition_index: u32,
    pub created: bool,
    pub deleted: bool,
    pub erase_history: bool,
}
impl FromRawGameEvent for ProtoDefChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ProtoDefChangedEvent {
            kind: u8::from_value(iter.next(), "kind")?,
            definition_index: u32::from_value(iter.next(), "definition_index")?,
            created: bool::from_value(iter.next(), "created")?,
            deleted: bool::from_value(iter.next(), "deleted")?,
            erase_history: bool::from_value(iter.next(), "erase_history")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerDominationEvent {
    pub dominator: u16,
    pub dominated: u16,
    pub dominations: u16,
}
impl FromRawGameEvent for PlayerDominationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerDominationEvent {
            dominator: u16::from_value(iter.next(), "dominator")?,
            dominated: u16::from_value(iter.next(), "dominated")?,
            dominations: u16::from_value(iter.next(), "dominations")?,
        })
    }
}
#[derive(Debug)]
pub struct PlayerRocketPackPushedEvent {
    pub pusher: u16,
    pub pushed: u16,
}
impl FromRawGameEvent for PlayerRocketPackPushedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(PlayerRocketPackPushedEvent {
            pusher: u16::from_value(iter.next(), "pusher")?,
            pushed: u16::from_value(iter.next(), "pushed")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestRequestEvent {
    pub request: u32,
    pub msg: String,
}
impl FromRawGameEvent for QuestRequestEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(QuestRequestEvent {
            request: u32::from_value(iter.next(), "request")?,
            msg: String::from_value(iter.next(), "msg")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestResponseEvent {
    pub request: u32,
    pub success: bool,
    pub msg: String,
}
impl FromRawGameEvent for QuestResponseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(QuestResponseEvent {
            request: u32::from_value(iter.next(), "request")?,
            success: bool::from_value(iter.next(), "success")?,
            msg: String::from_value(iter.next(), "msg")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestProgressEvent {
    pub owner: u16,
    pub scorer: u16,
    pub kind: u8,
    pub completed: bool,
    pub quest_definition_index: u32,
}
impl FromRawGameEvent for QuestProgressEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(QuestProgressEvent {
            owner: u16::from_value(iter.next(), "owner")?,
            scorer: u16::from_value(iter.next(), "scorer")?,
            kind: u8::from_value(iter.next(), "kind")?,
            completed: bool::from_value(iter.next(), "completed")?,
            quest_definition_index: u32::from_value(iter.next(), "quest_definition_index")?,
        })
    }
}
#[derive(Debug)]
pub struct ProjectileRemovedEvent {
    pub attacker: u8,
    pub weapon_def_index: u32,
    pub num_hit: u8,
    pub num_direct_hit: u8,
}
impl FromRawGameEvent for ProjectileRemovedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ProjectileRemovedEvent {
            attacker: u8::from_value(iter.next(), "attacker")?,
            weapon_def_index: u32::from_value(iter.next(), "weapon_def_index")?,
            num_hit: u8::from_value(iter.next(), "num_hit")?,
            num_direct_hit: u8::from_value(iter.next(), "num_direct_hit")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestMapDataChangedEvent {}
impl FromRawGameEvent for QuestMapDataChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(QuestMapDataChangedEvent {})
    }
}
#[derive(Debug)]
pub struct GasDousedPlayerIgnitedEvent {
    pub igniter: u16,
    pub douser: u16,
    pub victim: u16,
}
impl FromRawGameEvent for GasDousedPlayerIgnitedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(GasDousedPlayerIgnitedEvent {
            igniter: u16::from_value(iter.next(), "igniter")?,
            douser: u16::from_value(iter.next(), "douser")?,
            victim: u16::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct QuestTurnInStateEvent {
    pub state: u16,
}
impl FromRawGameEvent for QuestTurnInStateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(QuestTurnInStateEvent {
            state: u16::from_value(iter.next(), "state")?,
        })
    }
}
#[derive(Debug)]
pub struct ItemsAcknowledgedEvent {}
impl FromRawGameEvent for ItemsAcknowledgedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ItemsAcknowledgedEvent {})
    }
}
#[derive(Debug)]
pub struct CapperKilledEvent {
    pub blocker: u16,
    pub victim: u16,
}
impl FromRawGameEvent for CapperKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(CapperKilledEvent {
            blocker: u16::from_value(iter.next(), "blocker")?,
            victim: u16::from_value(iter.next(), "victim")?,
        })
    }
}
#[derive(Debug)]
pub struct MainMenuStabilizedEvent {}
impl FromRawGameEvent for MainMenuStabilizedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(MainMenuStabilizedEvent {})
    }
}
#[derive(Debug)]
pub struct WorldStatusChangedEvent {}
impl FromRawGameEvent for WorldStatusChangedEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(WorldStatusChangedEvent {})
    }
}
#[derive(Debug)]
pub struct HLTVStatusEvent {
    pub clients: u32,
    pub slots: u32,
    pub proxies: u16,
    pub master: String,
}
impl FromRawGameEvent for HLTVStatusEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVStatusEvent {
            clients: u32::from_value(iter.next(), "clients")?,
            slots: u32::from_value(iter.next(), "slots")?,
            proxies: u16::from_value(iter.next(), "proxies")?,
            master: String::from_value(iter.next(), "master")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVCameramanEvent {
    pub index: u16,
}
impl FromRawGameEvent for HLTVCameramanEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVCameramanEvent {
            index: u16::from_value(iter.next(), "index")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVRankCameraEvent {
    pub index: u8,
    pub rank: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVRankCameraEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVRankCameraEvent {
            index: u8::from_value(iter.next(), "index")?,
            rank: f32::from_value(iter.next(), "rank")?,
            target: u16::from_value(iter.next(), "target")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVRankEntityEvent {
    pub index: u16,
    pub rank: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVRankEntityEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVRankEntityEvent {
            index: u16::from_value(iter.next(), "index")?,
            rank: f32::from_value(iter.next(), "rank")?,
            target: u16::from_value(iter.next(), "target")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVFixedEvent {
    pub pos_x: u32,
    pub pos_y: u32,
    pub pos_z: u32,
    pub theta: u16,
    pub phi: u16,
    pub offset: u16,
    pub fov: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVFixedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVFixedEvent {
            pos_x: u32::from_value(iter.next(), "pos_x")?,
            pos_y: u32::from_value(iter.next(), "pos_y")?,
            pos_z: u32::from_value(iter.next(), "pos_z")?,
            theta: u16::from_value(iter.next(), "theta")?,
            phi: u16::from_value(iter.next(), "phi")?,
            offset: u16::from_value(iter.next(), "offset")?,
            fov: f32::from_value(iter.next(), "fov")?,
            target: u16::from_value(iter.next(), "target")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVChaseEvent {
    pub target_1: u16,
    pub target_2: u16,
    pub distance: u16,
    pub theta: u16,
    pub phi: u16,
    pub inertia: u8,
    pub in_eye: u8,
}
impl FromRawGameEvent for HLTVChaseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVChaseEvent {
            target_1: u16::from_value(iter.next(), "target_1")?,
            target_2: u16::from_value(iter.next(), "target_2")?,
            distance: u16::from_value(iter.next(), "distance")?,
            theta: u16::from_value(iter.next(), "theta")?,
            phi: u16::from_value(iter.next(), "phi")?,
            inertia: u8::from_value(iter.next(), "inertia")?,
            in_eye: u8::from_value(iter.next(), "in_eye")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVMessageEvent {
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVTitleEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVTitleEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVTitleEvent {
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct HLTVChatEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(HLTVChatEvent {
            text: String::from_value(iter.next(), "text")?,
        })
    }
}
#[derive(Debug)]
pub struct ReplayStartRecordEvent {}
impl FromRawGameEvent for ReplayStartRecordEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ReplayStartRecordEvent {})
    }
}
#[derive(Debug)]
pub struct ReplaySessionInfoEvent {
    pub sn: String,
    pub di: u8,
    pub cb: u32,
    pub st: u32,
}
impl FromRawGameEvent for ReplaySessionInfoEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ReplaySessionInfoEvent {
            sn: String::from_value(iter.next(), "sn")?,
            di: u8::from_value(iter.next(), "di")?,
            cb: u32::from_value(iter.next(), "cb")?,
            st: u32::from_value(iter.next(), "st")?,
        })
    }
}
#[derive(Debug)]
pub struct ReplayEndRecordEvent {}
impl FromRawGameEvent for ReplayEndRecordEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ReplayEndRecordEvent {})
    }
}
#[derive(Debug)]
pub struct ReplayReplaysAvailableEvent {}
impl FromRawGameEvent for ReplayReplaysAvailableEvent {
    fn from_raw_event(_values: Vec<GameEventValue>) -> Result<Self> {
        Ok(ReplayReplaysAvailableEvent {})
    }
}
#[derive(Debug)]
pub struct ReplayServerErrorEvent {
    pub error: String,
}
impl FromRawGameEvent for ReplayServerErrorEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        Ok(ReplayServerErrorEvent {
            error: String::from_value(iter.next(), "error")?,
        })
    }
}
#[derive(Debug)]
pub enum GameEvent {
    ServerSpawn(ServerSpawnEvent),
    ServerChangeLevelFailed(ServerChangeLevelFailedEvent),
    ServerShutdown(ServerShutdownEvent),
    ServerCvar(ServerCvarEvent),
    ServerMessage(ServerMessageEvent),
    ServerAddBan(ServerAddBanEvent),
    ServerRemoveBan(ServerRemoveBanEvent),
    PlayerConnect(PlayerConnectEvent),
    PlayerConnectClient(PlayerConnectClientEvent),
    PlayerInfo(PlayerInfoEvent),
    PlayerDisconnect(PlayerDisconnectEvent),
    PlayerActivate(PlayerActivateEvent),
    PlayerSay(PlayerSayEvent),
    ClientDisconnect(ClientDisconnectEvent),
    ClientBeginConnect(ClientBeginConnectEvent),
    ClientConnected(ClientConnectedEvent),
    ClientFullConnect(ClientFullConnectEvent),
    HostQuit(HostQuitEvent),
    TeamInfo(TeamInfoEvent),
    TeamScore(TeamScoreEvent),
    TeamPlayBroadcastAudio(TeamPlayBroadcastAudioEvent),
    PlayerTeam(PlayerTeamEvent),
    PlayerClass(PlayerClassEvent),
    PlayerDeath(PlayerDeathEvent),
    PlayerHurt(PlayerHurtEvent),
    PlayerChat(PlayerChatEvent),
    PlayerScore(PlayerScoreEvent),
    PlayerSpawn(PlayerSpawnEvent),
    PlayerShoot(PlayerShootEvent),
    PlayerUse(PlayerUseEvent),
    PlayerChangeName(PlayerChangeNameEvent),
    PlayerHintMessage(PlayerHintMessageEvent),
    BasePlayerTeleported(BasePlayerTeleportedEvent),
    GameInit(GameInitEvent),
    GameNewMap(GameNewMapEvent),
    GameStart(GameStartEvent),
    GameEnd(GameEndEvent),
    RoundStart(RoundStartEvent),
    RoundEnd(RoundEndEvent),
    GameMessage(GameMessageEvent),
    BreakBreakable(BreakBreakableEvent),
    BreakProp(BreakPropEvent),
    EntityKilled(EntityKilledEvent),
    BonusUpdated(BonusUpdatedEvent),
    AchievementEvent(AchievementEventEvent),
    AchievementIncrement(AchievementIncrementEvent),
    PhysgunPickup(PhysgunPickupEvent),
    FlareIgniteNpc(FlareIgniteNpcEvent),
    HelicopterGrenadePuntMiss(HelicopterGrenadePuntMissEvent),
    UserDataDownloaded(UserDataDownloadedEvent),
    RagdollDissolved(RagdollDissolvedEvent),
    HLTVChangedMode(HLTVChangedModeEvent),
    HLTVChangedTarget(HLTVChangedTargetEvent),
    VoteEnded(VoteEndedEvent),
    VoteStarted(VoteStartedEvent),
    VoteChanged(VoteChangedEvent),
    VotePassed(VotePassedEvent),
    VoteFailed(VoteFailedEvent),
    VoteCast(VoteCastEvent),
    VoteOptions(VoteOptionsEvent),
    ReplaySaved(ReplaySavedEvent),
    EnteredPerformanceMode(EnteredPerformanceModeEvent),
    BrowseReplays(BrowseReplaysEvent),
    ReplayYoutubeStats(ReplayYoutubeStatsEvent),
    InventoryUpdated(InventoryUpdatedEvent),
    CartUpdated(CartUpdatedEvent),
    StorePriceSheetUpdated(StorePriceSheetUpdatedEvent),
    EconInventoryConnected(EconInventoryConnectedEvent),
    ItemSchemaInitialized(ItemSchemaInitializedEvent),
    GcNewSession(GcNewSessionEvent),
    GcLostSession(GcLostSessionEvent),
    IntroFinish(IntroFinishEvent),
    IntroNextCamera(IntroNextCameraEvent),
    PlayerChangeClass(PlayerChangeClassEvent),
    TfMapTimeRemaining(TfMapTimeRemainingEvent),
    TfGameOver(TfGameOverEvent),
    CtfFlagCaptured(CtfFlagCapturedEvent),
    ControlPointInitialized(ControlPointInitializedEvent),
    ControlPointUpdateImages(ControlPointUpdateImagesEvent),
    ControlPointUpdateLayout(ControlPointUpdateLayoutEvent),
    ControlPointUpdateCapping(ControlPointUpdateCappingEvent),
    ControlPointUpdateOwner(ControlPointUpdateOwnerEvent),
    ControlPointStartTouch(ControlPointStartTouchEvent),
    ControlPointEndTouch(ControlPointEndTouchEvent),
    ControlPointPulseElement(ControlPointPulseElementEvent),
    ControlPointFakeCapture(ControlPointFakeCaptureEvent),
    ControlPointFakeCaptureMultiplier(ControlPointFakeCaptureMultiplierEvent),
    TeamPlayRoundSelected(TeamPlayRoundSelectedEvent),
    TeamPlayRoundStart(TeamPlayRoundStartEvent),
    TeamPlayRoundActive(TeamPlayRoundActiveEvent),
    TeamPlayWaitingBegins(TeamPlayWaitingBeginsEvent),
    TeamPlayWaitingEnds(TeamPlayWaitingEndsEvent),
    TeamPlayWaitingAboutToEnd(TeamPlayWaitingAboutToEndEvent),
    TeamPlayRestartRound(TeamPlayRestartRoundEvent),
    TeamPlayReadyRestart(TeamPlayReadyRestartEvent),
    TeamPlayRoundRestartSeconds(TeamPlayRoundRestartSecondsEvent),
    TeamPlayTeamReady(TeamPlayTeamReadyEvent),
    TeamPlayRoundWin(TeamPlayRoundWinEvent),
    TeamPlayUpdateTimer(TeamPlayUpdateTimerEvent),
    TeamPlayRoundStalemate(TeamPlayRoundStalemateEvent),
    TeamPlayOvertimeBegin(TeamPlayOvertimeBeginEvent),
    TeamPlayOvertimeEnd(TeamPlayOvertimeEndEvent),
    TeamPlaySuddenDeathBegin(TeamPlaySuddenDeathBeginEvent),
    TeamPlaySuddenDeathEnd(TeamPlaySuddenDeathEndEvent),
    TeamPlayGameOver(TeamPlayGameOverEvent),
    TeamPlayMapTimeRemaining(TeamPlayMapTimeRemainingEvent),
    TeamPlayTimerFlash(TeamPlayTimerFlashEvent),
    TeamPlayTimerTimeAdded(TeamPlayTimerTimeAddedEvent),
    TeamPlayPointStartCapture(TeamPlayPointStartCaptureEvent),
    TeamPlayPointCaptured(TeamPlayPointCapturedEvent),
    TeamPlayPointLocked(TeamPlayPointLockedEvent),
    TeamPlayPointUnlocked(TeamPlayPointUnlockedEvent),
    TeamPlayCaptureBroken(TeamPlayCaptureBrokenEvent),
    TeamPlayCaptureBlocked(TeamPlayCaptureBlockedEvent),
    TeamPlayFlagEvent(TeamPlayFlagEventEvent),
    TeamPlayWinPanel(TeamPlayWinPanelEvent),
    TeamPlayTeamBalancedPlayer(TeamPlayTeamBalancedPlayerEvent),
    TeamPlaySetupFinished(TeamPlaySetupFinishedEvent),
    TeamPlayAlert(TeamPlayAlertEvent),
    TrainingComplete(TrainingCompleteEvent),
    ShowFreezePanel(ShowFreezePanelEvent),
    HideFreezePanel(HideFreezePanelEvent),
    FreezeCamStarted(FreezeCamStartedEvent),
    LocalPlayerChangeTeam(LocalPlayerChangeTeamEvent),
    LocalPlayerScoreChanged(LocalPlayerScoreChangedEvent),
    LocalPlayerChangeClass(LocalPlayerChangeClassEvent),
    LocalPlayerRespawn(LocalPlayerRespawnEvent),
    BuildingInfoChanged(BuildingInfoChangedEvent),
    LocalPlayerChangeDisguise(LocalPlayerChangeDisguiseEvent),
    PlayerAccountChanged(PlayerAccountChangedEvent),
    SpyPdaReset(SpyPdaResetEvent),
    FlagStatusUpdate(FlagStatusUpdateEvent),
    PlayerStatsUpdated(PlayerStatsUpdatedEvent),
    PlayingCommentary(PlayingCommentaryEvent),
    PlayerChargeDeployed(PlayerChargeDeployedEvent),
    PlayerBuiltObject(PlayerBuiltObjectEvent),
    PlayerUpgradedObject(PlayerUpgradedObjectEvent),
    PlayerCarryObject(PlayerCarryObjectEvent),
    PlayerDropObject(PlayerDropObjectEvent),
    ObjectRemoved(ObjectRemovedEvent),
    ObjectDestroyed(ObjectDestroyedEvent),
    ObjectDetonated(ObjectDetonatedEvent),
    AchievementEarned(AchievementEarnedEvent),
    SpecTargetUpdated(SpecTargetUpdatedEvent),
    TournamentStateUpdate(TournamentStateUpdateEvent),
    TournamentEnableCountdown(TournamentEnableCountdownEvent),
    PlayerCalledForMedic(PlayerCalledForMedicEvent),
    PlayerAskedForBall(PlayerAskedForBallEvent),
    LocalPlayerBecameObserver(LocalPlayerBecameObserverEvent),
    PlayerIgnitedInv(PlayerIgnitedInvEvent),
    PlayerIgnited(PlayerIgnitedEvent),
    PlayerExtinguished(PlayerExtinguishedEvent),
    PlayerTeleported(PlayerTeleportedEvent),
    PlayerHealedMedicCall(PlayerHealedMedicCallEvent),
    LocalPlayerChargeReady(LocalPlayerChargeReadyEvent),
    LocalPlayerWindDown(LocalPlayerWindDownEvent),
    PlayerInvulned(PlayerInvulnedEvent),
    EscortSpeed(EscortSpeedEvent),
    EscortProgress(EscortProgressEvent),
    EscortRecede(EscortRecedeEvent),
    GameUIActivated(GameUIActivatedEvent),
    GameUIHidden(GameUIHiddenEvent),
    PlayerEscortScore(PlayerEscortScoreEvent),
    PlayerHealOnHit(PlayerHealOnHitEvent),
    PlayerStealSandvich(PlayerStealSandvichEvent),
    ShowClassLayout(ShowClassLayoutEvent),
    ShowVsPanel(ShowVsPanelEvent),
    PlayerDamaged(PlayerDamagedEvent),
    ArenaPlayerNotification(ArenaPlayerNotificationEvent),
    ArenaMatchMaxStreak(ArenaMatchMaxStreakEvent),
    ArenaRoundStart(ArenaRoundStartEvent),
    ArenaWinPanel(ArenaWinPanelEvent),
    PveWinPanel(PveWinPanelEvent),
    AirDash(AirDashEvent),
    Landed(LandedEvent),
    PlayerDamageDodged(PlayerDamageDodgedEvent),
    PlayerStunned(PlayerStunnedEvent),
    ScoutGrandSlam(ScoutGrandSlamEvent),
    ScoutSlamdollLanded(ScoutSlamdollLandedEvent),
    ArrowImpact(ArrowImpactEvent),
    PlayerJarated(PlayerJaratedEvent),
    PlayerJaratedFade(PlayerJaratedFadeEvent),
    PlayerShieldBlocked(PlayerShieldBlockedEvent),
    PlayerPinned(PlayerPinnedEvent),
    PlayerHealedByMedic(PlayerHealedByMedicEvent),
    PlayerSappedObject(PlayerSappedObjectEvent),
    ItemFound(ItemFoundEvent),
    ShowAnnotation(ShowAnnotationEvent),
    HideAnnotation(HideAnnotationEvent),
    PostInventoryApplication(PostInventoryApplicationEvent),
    ControlPointUnlockUpdated(ControlPointUnlockUpdatedEvent),
    DeployBuffBanner(DeployBuffBannerEvent),
    PlayerBuff(PlayerBuffEvent),
    MedicDeath(MedicDeathEvent),
    OvertimeNag(OvertimeNagEvent),
    TeamsChanged(TeamsChangedEvent),
    HalloweenPumpkinGrab(HalloweenPumpkinGrabEvent),
    RocketJump(RocketJumpEvent),
    RocketJumpLanded(RocketJumpLandedEvent),
    StickyJump(StickyJumpEvent),
    StickyJumpLanded(StickyJumpLandedEvent),
    RocketPackLaunch(RocketPackLaunchEvent),
    RocketPackLanded(RocketPackLandedEvent),
    MedicDefended(MedicDefendedEvent),
    LocalPlayerHealed(LocalPlayerHealedEvent),
    PlayerDestroyedPipeBomb(PlayerDestroyedPipeBombEvent),
    ObjectDeflected(ObjectDeflectedEvent),
    PlayerMvp(PlayerMvpEvent),
    RaidSpawnMob(RaidSpawnMobEvent),
    RaidSpawnSquad(RaidSpawnSquadEvent),
    NavBlocked(NavBlockedEvent),
    PathTrackPassed(PathTrackPassedEvent),
    NumCappersChanged(NumCappersChangedEvent),
    PlayerRegenerate(PlayerRegenerateEvent),
    UpdateStatusItem(UpdateStatusItemEvent),
    StatsResetRound(StatsResetRoundEvent),
    ScoreStatsAccumulatedUpdate(ScoreStatsAccumulatedUpdateEvent),
    ScoreStatsAccumulatedReset(ScoreStatsAccumulatedResetEvent),
    AchievementEarnedLocal(AchievementEarnedLocalEvent),
    PlayerHealed(PlayerHealedEvent),
    BuildingHealed(BuildingHealedEvent),
    ItemPickup(ItemPickupEvent),
    DuelStatus(DuelStatusEvent),
    FishNotice(FishNoticeEvent),
    FishNoticeArm(FishNoticeArmEvent),
    SlapNotice(SlapNoticeEvent),
    ThrowableHit(ThrowableHitEvent),
    PumpkinLordSummoned(PumpkinLordSummonedEvent),
    PumpkinLordKilled(PumpkinLordKilledEvent),
    MerasmusSummoned(MerasmusSummonedEvent),
    MerasmusKilled(MerasmusKilledEvent),
    MerasmusEscapeWarning(MerasmusEscapeWarningEvent),
    MerasmusEscaped(MerasmusEscapedEvent),
    EyeballBossSummoned(EyeballBossSummonedEvent),
    EyeballBossStunned(EyeballBossStunnedEvent),
    EyeballBossKilled(EyeballBossKilledEvent),
    EyeballBossKiller(EyeballBossKillerEvent),
    EyeballBossEscapeImminent(EyeballBossEscapeImminentEvent),
    EyeballBossEscaped(EyeballBossEscapedEvent),
    NpcHurt(NpcHurtEvent),
    ControlPointTimerUpdated(ControlPointTimerUpdatedEvent),
    PlayerHighFiveStart(PlayerHighFiveStartEvent),
    PlayerHighFiveCancel(PlayerHighFiveCancelEvent),
    PlayerHighFiveSuccess(PlayerHighFiveSuccessEvent),
    PlayerBonusPoints(PlayerBonusPointsEvent),
    PlayerUpgraded(PlayerUpgradedEvent),
    PlayerBuyback(PlayerBuybackEvent),
    PlayerUsedPowerUpBottle(PlayerUsedPowerUpBottleEvent),
    ChristmasGiftGrab(ChristmasGiftGrabEvent),
    PlayerKilledAchievementZone(PlayerKilledAchievementZoneEvent),
    PartyUpdated(PartyUpdatedEvent),
    PartyPrefChanged(PartyPrefChangedEvent),
    PartyCriteriaChanged(PartyCriteriaChangedEvent),
    PartyInvitesChanged(PartyInvitesChangedEvent),
    PartyQueueStateChanged(PartyQueueStateChangedEvent),
    PartyChat(PartyChatEvent),
    PartyMemberJoin(PartyMemberJoinEvent),
    PartyMemberLeave(PartyMemberLeaveEvent),
    MatchInvitesUpdated(MatchInvitesUpdatedEvent),
    LobbyUpdated(LobbyUpdatedEvent),
    MvmMissionUpdate(MvmMissionUpdateEvent),
    RecalculateHolidays(RecalculateHolidaysEvent),
    PlayerCurrencyChanged(PlayerCurrencyChangedEvent),
    DoomsdayRocketOpen(DoomsdayRocketOpenEvent),
    RemoveNemesisRelationships(RemoveNemesisRelationshipsEvent),
    MvmCreditBonusWave(MvmCreditBonusWaveEvent),
    MvmCreditBonusAll(MvmCreditBonusAllEvent),
    MvmCreditBonusAllAdvanced(MvmCreditBonusAllAdvancedEvent),
    MvmQuickSentryUpgrade(MvmQuickSentryUpgradeEvent),
    MvmTankDestroyedByPlayers(MvmTankDestroyedByPlayersEvent),
    MvmKillRobotDeliveringBomb(MvmKillRobotDeliveringBombEvent),
    MvmPickupCurrency(MvmPickupCurrencyEvent),
    MvmBombCarrierKilled(MvmBombCarrierKilledEvent),
    MvmSentryBusterDetonate(MvmSentryBusterDetonateEvent),
    MvmScoutMarkedForDeath(MvmScoutMarkedForDeathEvent),
    MvmMedicPowerUpShared(MvmMedicPowerUpSharedEvent),
    MvmBeginWave(MvmBeginWaveEvent),
    MvmWaveComplete(MvmWaveCompleteEvent),
    MvmMissionComplete(MvmMissionCompleteEvent),
    MvmBombResetByPlayer(MvmBombResetByPlayerEvent),
    MvmBombAlarmTriggered(MvmBombAlarmTriggeredEvent),
    MvmBombDeployResetByPlayer(MvmBombDeployResetByPlayerEvent),
    MvmWaveFailed(MvmWaveFailedEvent),
    MvmResetStats(MvmResetStatsEvent),
    DamageResisted(DamageResistedEvent),
    RevivePlayerNotify(RevivePlayerNotifyEvent),
    RevivePlayerStopped(RevivePlayerStoppedEvent),
    RevivePlayerComplete(RevivePlayerCompleteEvent),
    PlayerTurnedToGhost(PlayerTurnedToGhostEvent),
    MedigunShieldBlockedDamage(MedigunShieldBlockedDamageEvent),
    MvmAdvWaveCompleteNoGates(MvmAdvWaveCompleteNoGatesEvent),
    MvmSniperHeadshotCurrency(MvmSniperHeadshotCurrencyEvent),
    MvmMannhattanPit(MvmMannhattanPitEvent),
    FlagCarriedInDetectionZone(FlagCarriedInDetectionZoneEvent),
    MvmAdvWaveKilledStunRadio(MvmAdvWaveKilledStunRadioEvent),
    PlayerDirectHitStun(PlayerDirectHitStunEvent),
    MvmSentryBusterKilled(MvmSentryBusterKilledEvent),
    UpgradesFileChanged(UpgradesFileChangedEvent),
    RdTeamPointsChanged(RdTeamPointsChangedEvent),
    RdRulesStateChanged(RdRulesStateChangedEvent),
    RdRobotKilled(RdRobotKilledEvent),
    RdRobotImpact(RdRobotImpactEvent),
    TeamPlayPreRoundTimeLeft(TeamPlayPreRoundTimeLeftEvent),
    ParachuteDeploy(ParachuteDeployEvent),
    ParachuteHolster(ParachuteHolsterEvent),
    KillRefillsMeter(KillRefillsMeterEvent),
    RpsTauntEvent(RpsTauntEventEvent),
    CongaKill(CongaKillEvent),
    PlayerInitialSpawn(PlayerInitialSpawnEvent),
    CompetitiveVictory(CompetitiveVictoryEvent),
    CompetitiveStatsUpdate(CompetitiveStatsUpdateEvent),
    MiniGameWin(MiniGameWinEvent),
    SentryOnGoActive(SentryOnGoActiveEvent),
    DuckXpLevelUp(DuckXpLevelUpEvent),
    QuestLogOpened(QuestLogOpenedEvent),
    SchemaUpdated(SchemaUpdatedEvent),
    LocalPlayerPickupWeapon(LocalPlayerPickupWeaponEvent),
    RdPlayerScorePoints(RdPlayerScorePointsEvent),
    DemomanDetStickies(DemomanDetStickiesEvent),
    QuestObjectiveCompleted(QuestObjectiveCompletedEvent),
    PlayerScoreChanged(PlayerScoreChangedEvent),
    KilledCappingPlayer(KilledCappingPlayerEvent),
    EnvironmentalDeath(EnvironmentalDeathEvent),
    ProjectileDirectHit(ProjectileDirectHitEvent),
    PassGet(PassGetEvent),
    PassScore(PassScoreEvent),
    PassFree(PassFreeEvent),
    PassPassCaught(PassPassCaughtEvent),
    PassBallStolen(PassBallStolenEvent),
    PassBallBlocked(PassBallBlockedEvent),
    DamagePrevented(DamagePreventedEvent),
    HalloweenBossKilled(HalloweenBossKilledEvent),
    EscapedLootIsland(EscapedLootIslandEvent),
    TaggedPlayerAsIt(TaggedPlayerAsItEvent),
    MerasmusStunned(MerasmusStunnedEvent),
    MerasmusPropFound(MerasmusPropFoundEvent),
    HalloweenSkeletonKilled(HalloweenSkeletonKilledEvent),
    EscapeHell(EscapeHellEvent),
    CrossSpectralBridge(CrossSpectralBridgeEvent),
    MiniGameWon(MiniGameWonEvent),
    RespawnGhost(RespawnGhostEvent),
    KillInHell(KillInHellEvent),
    HalloweenDuckCollected(HalloweenDuckCollectedEvent),
    SpecialScore(SpecialScoreEvent),
    TeamLeaderKilled(TeamLeaderKilledEvent),
    HalloweenSoulCollected(HalloweenSoulCollectedEvent),
    RecalculateTruce(RecalculateTruceEvent),
    DeadRingerCheatDeath(DeadRingerCheatDeathEvent),
    CrossbowHeal(CrossbowHealEvent),
    DamageMitigated(DamageMitigatedEvent),
    PayloadPushed(PayloadPushedEvent),
    PlayerAbandonedMatch(PlayerAbandonedMatchEvent),
    ClDrawline(ClDrawlineEvent),
    RestartTimerTime(RestartTimerTimeEvent),
    WinLimitChanged(WinLimitChangedEvent),
    WinPanelShowScores(WinPanelShowScoresEvent),
    TopStreamsRequestFinished(TopStreamsRequestFinishedEvent),
    CompetitiveStateChanged(CompetitiveStateChangedEvent),
    GlobalWarDataUpdated(GlobalWarDataUpdatedEvent),
    StopWatchChanged(StopWatchChangedEvent),
    DsStop(DsStopEvent),
    DsScreenshot(DsScreenshotEvent),
    ShowMatchSummary(ShowMatchSummaryEvent),
    ExperienceChanged(ExperienceChangedEvent),
    BeginXpLerp(BeginXpLerpEvent),
    MatchmakerStatsUpdated(MatchmakerStatsUpdatedEvent),
    RematchVotePeriodOver(RematchVotePeriodOverEvent),
    RematchFailedToCreate(RematchFailedToCreateEvent),
    PlayerRematchChange(PlayerRematchChangeEvent),
    PingUpdated(PingUpdatedEvent),
    MMStatsUpdated(MMStatsUpdatedEvent),
    PlayerNextMapVoteChange(PlayerNextMapVoteChangeEvent),
    VoteMapsChanged(VoteMapsChangedEvent),
    ProtoDefChanged(ProtoDefChangedEvent),
    PlayerDomination(PlayerDominationEvent),
    PlayerRocketPackPushed(PlayerRocketPackPushedEvent),
    QuestRequest(QuestRequestEvent),
    QuestResponse(QuestResponseEvent),
    QuestProgress(QuestProgressEvent),
    ProjectileRemoved(ProjectileRemovedEvent),
    QuestMapDataChanged(QuestMapDataChangedEvent),
    GasDousedPlayerIgnited(GasDousedPlayerIgnitedEvent),
    QuestTurnInState(QuestTurnInStateEvent),
    ItemsAcknowledged(ItemsAcknowledgedEvent),
    CapperKilled(CapperKilledEvent),
    MainMenuStabilized(MainMenuStabilizedEvent),
    WorldStatusChanged(WorldStatusChangedEvent),
    HLTVStatus(HLTVStatusEvent),
    HLTVCameraman(HLTVCameramanEvent),
    HLTVRankCamera(HLTVRankCameraEvent),
    HLTVRankEntity(HLTVRankEntityEvent),
    HLTVFixed(HLTVFixedEvent),
    HLTVChase(HLTVChaseEvent),
    HLTVMessage(HLTVMessageEvent),
    HLTVTitle(HLTVTitleEvent),
    HLTVChat(HLTVChatEvent),
    ReplayStartRecord(ReplayStartRecordEvent),
    ReplaySessionInfo(ReplaySessionInfoEvent),
    ReplayEndRecord(ReplayEndRecordEvent),
    ReplayReplaysAvailable(ReplayReplaysAvailableEvent),
    ReplayServerError(ReplayServerErrorEvent),
    Unknown(RawGameEvent),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum GameEventType {
    ServerSpawn = 0,
    ServerChangeLevelFailed = 1,
    ServerShutdown = 2,
    ServerCvar = 3,
    ServerMessage = 4,
    ServerAddBan = 5,
    ServerRemoveBan = 6,
    PlayerConnect = 7,
    PlayerConnectClient = 8,
    PlayerInfo = 9,
    PlayerDisconnect = 10,
    PlayerActivate = 11,
    PlayerSay = 12,
    ClientDisconnect = 13,
    ClientBeginConnect = 14,
    ClientConnected = 15,
    ClientFullConnect = 16,
    HostQuit = 17,
    TeamInfo = 18,
    TeamScore = 19,
    TeamPlayBroadcastAudio = 20,
    PlayerTeam = 21,
    PlayerClass = 22,
    PlayerDeath = 23,
    PlayerHurt = 24,
    PlayerChat = 25,
    PlayerScore = 26,
    PlayerSpawn = 27,
    PlayerShoot = 28,
    PlayerUse = 29,
    PlayerChangeName = 30,
    PlayerHintMessage = 31,
    BasePlayerTeleported = 32,
    GameInit = 33,
    GameNewMap = 34,
    GameStart = 35,
    GameEnd = 36,
    RoundStart = 37,
    RoundEnd = 38,
    GameMessage = 39,
    BreakBreakable = 40,
    BreakProp = 41,
    EntityKilled = 42,
    BonusUpdated = 43,
    AchievementEvent = 44,
    AchievementIncrement = 45,
    PhysgunPickup = 46,
    FlareIgniteNpc = 47,
    HelicopterGrenadePuntMiss = 48,
    UserDataDownloaded = 49,
    RagdollDissolved = 50,
    HLTVChangedMode = 51,
    HLTVChangedTarget = 52,
    VoteEnded = 53,
    VoteStarted = 54,
    VoteChanged = 55,
    VotePassed = 56,
    VoteFailed = 57,
    VoteCast = 58,
    VoteOptions = 59,
    ReplaySaved = 60,
    EnteredPerformanceMode = 61,
    BrowseReplays = 62,
    ReplayYoutubeStats = 63,
    InventoryUpdated = 64,
    CartUpdated = 65,
    StorePriceSheetUpdated = 66,
    EconInventoryConnected = 67,
    ItemSchemaInitialized = 68,
    GcNewSession = 69,
    GcLostSession = 70,
    IntroFinish = 71,
    IntroNextCamera = 72,
    PlayerChangeClass = 73,
    TfMapTimeRemaining = 74,
    TfGameOver = 75,
    CtfFlagCaptured = 76,
    ControlPointInitialized = 77,
    ControlPointUpdateImages = 78,
    ControlPointUpdateLayout = 79,
    ControlPointUpdateCapping = 80,
    ControlPointUpdateOwner = 81,
    ControlPointStartTouch = 82,
    ControlPointEndTouch = 83,
    ControlPointPulseElement = 84,
    ControlPointFakeCapture = 85,
    ControlPointFakeCaptureMultiplier = 86,
    TeamPlayRoundSelected = 87,
    TeamPlayRoundStart = 88,
    TeamPlayRoundActive = 89,
    TeamPlayWaitingBegins = 90,
    TeamPlayWaitingEnds = 91,
    TeamPlayWaitingAboutToEnd = 92,
    TeamPlayRestartRound = 93,
    TeamPlayReadyRestart = 94,
    TeamPlayRoundRestartSeconds = 95,
    TeamPlayTeamReady = 96,
    TeamPlayRoundWin = 97,
    TeamPlayUpdateTimer = 98,
    TeamPlayRoundStalemate = 99,
    TeamPlayOvertimeBegin = 100,
    TeamPlayOvertimeEnd = 101,
    TeamPlaySuddenDeathBegin = 102,
    TeamPlaySuddenDeathEnd = 103,
    TeamPlayGameOver = 104,
    TeamPlayMapTimeRemaining = 105,
    TeamPlayTimerFlash = 106,
    TeamPlayTimerTimeAdded = 107,
    TeamPlayPointStartCapture = 108,
    TeamPlayPointCaptured = 109,
    TeamPlayPointLocked = 110,
    TeamPlayPointUnlocked = 111,
    TeamPlayCaptureBroken = 112,
    TeamPlayCaptureBlocked = 113,
    TeamPlayFlagEvent = 114,
    TeamPlayWinPanel = 115,
    TeamPlayTeamBalancedPlayer = 116,
    TeamPlaySetupFinished = 117,
    TeamPlayAlert = 118,
    TrainingComplete = 119,
    ShowFreezePanel = 120,
    HideFreezePanel = 121,
    FreezeCamStarted = 122,
    LocalPlayerChangeTeam = 123,
    LocalPlayerScoreChanged = 124,
    LocalPlayerChangeClass = 125,
    LocalPlayerRespawn = 126,
    BuildingInfoChanged = 127,
    LocalPlayerChangeDisguise = 128,
    PlayerAccountChanged = 129,
    SpyPdaReset = 130,
    FlagStatusUpdate = 131,
    PlayerStatsUpdated = 132,
    PlayingCommentary = 133,
    PlayerChargeDeployed = 134,
    PlayerBuiltObject = 135,
    PlayerUpgradedObject = 136,
    PlayerCarryObject = 137,
    PlayerDropObject = 138,
    ObjectRemoved = 139,
    ObjectDestroyed = 140,
    ObjectDetonated = 141,
    AchievementEarned = 142,
    SpecTargetUpdated = 143,
    TournamentStateUpdate = 144,
    TournamentEnableCountdown = 145,
    PlayerCalledForMedic = 146,
    PlayerAskedForBall = 147,
    LocalPlayerBecameObserver = 148,
    PlayerIgnitedInv = 149,
    PlayerIgnited = 150,
    PlayerExtinguished = 151,
    PlayerTeleported = 152,
    PlayerHealedMedicCall = 153,
    LocalPlayerChargeReady = 154,
    LocalPlayerWindDown = 155,
    PlayerInvulned = 156,
    EscortSpeed = 157,
    EscortProgress = 158,
    EscortRecede = 159,
    GameUIActivated = 160,
    GameUIHidden = 161,
    PlayerEscortScore = 162,
    PlayerHealOnHit = 163,
    PlayerStealSandvich = 164,
    ShowClassLayout = 165,
    ShowVsPanel = 166,
    PlayerDamaged = 167,
    ArenaPlayerNotification = 168,
    ArenaMatchMaxStreak = 169,
    ArenaRoundStart = 170,
    ArenaWinPanel = 171,
    PveWinPanel = 172,
    AirDash = 173,
    Landed = 174,
    PlayerDamageDodged = 175,
    PlayerStunned = 176,
    ScoutGrandSlam = 177,
    ScoutSlamdollLanded = 178,
    ArrowImpact = 179,
    PlayerJarated = 180,
    PlayerJaratedFade = 181,
    PlayerShieldBlocked = 182,
    PlayerPinned = 183,
    PlayerHealedByMedic = 184,
    PlayerSappedObject = 185,
    ItemFound = 186,
    ShowAnnotation = 187,
    HideAnnotation = 188,
    PostInventoryApplication = 189,
    ControlPointUnlockUpdated = 190,
    DeployBuffBanner = 191,
    PlayerBuff = 192,
    MedicDeath = 193,
    OvertimeNag = 194,
    TeamsChanged = 195,
    HalloweenPumpkinGrab = 196,
    RocketJump = 197,
    RocketJumpLanded = 198,
    StickyJump = 199,
    StickyJumpLanded = 200,
    RocketPackLaunch = 201,
    RocketPackLanded = 202,
    MedicDefended = 203,
    LocalPlayerHealed = 204,
    PlayerDestroyedPipeBomb = 205,
    ObjectDeflected = 206,
    PlayerMvp = 207,
    RaidSpawnMob = 208,
    RaidSpawnSquad = 209,
    NavBlocked = 210,
    PathTrackPassed = 211,
    NumCappersChanged = 212,
    PlayerRegenerate = 213,
    UpdateStatusItem = 214,
    StatsResetRound = 215,
    ScoreStatsAccumulatedUpdate = 216,
    ScoreStatsAccumulatedReset = 217,
    AchievementEarnedLocal = 218,
    PlayerHealed = 219,
    BuildingHealed = 220,
    ItemPickup = 221,
    DuelStatus = 222,
    FishNotice = 223,
    FishNoticeArm = 224,
    SlapNotice = 225,
    ThrowableHit = 226,
    PumpkinLordSummoned = 227,
    PumpkinLordKilled = 228,
    MerasmusSummoned = 229,
    MerasmusKilled = 230,
    MerasmusEscapeWarning = 231,
    MerasmusEscaped = 232,
    EyeballBossSummoned = 233,
    EyeballBossStunned = 234,
    EyeballBossKilled = 235,
    EyeballBossKiller = 236,
    EyeballBossEscapeImminent = 237,
    EyeballBossEscaped = 238,
    NpcHurt = 239,
    ControlPointTimerUpdated = 240,
    PlayerHighFiveStart = 241,
    PlayerHighFiveCancel = 242,
    PlayerHighFiveSuccess = 243,
    PlayerBonusPoints = 244,
    PlayerUpgraded = 245,
    PlayerBuyback = 246,
    PlayerUsedPowerUpBottle = 247,
    ChristmasGiftGrab = 248,
    PlayerKilledAchievementZone = 249,
    PartyUpdated = 250,
    PartyPrefChanged = 251,
    PartyCriteriaChanged = 252,
    PartyInvitesChanged = 253,
    PartyQueueStateChanged = 254,
    PartyChat = 255,
    PartyMemberJoin = 256,
    PartyMemberLeave = 257,
    MatchInvitesUpdated = 258,
    LobbyUpdated = 259,
    MvmMissionUpdate = 260,
    RecalculateHolidays = 261,
    PlayerCurrencyChanged = 262,
    DoomsdayRocketOpen = 263,
    RemoveNemesisRelationships = 264,
    MvmCreditBonusWave = 265,
    MvmCreditBonusAll = 266,
    MvmCreditBonusAllAdvanced = 267,
    MvmQuickSentryUpgrade = 268,
    MvmTankDestroyedByPlayers = 269,
    MvmKillRobotDeliveringBomb = 270,
    MvmPickupCurrency = 271,
    MvmBombCarrierKilled = 272,
    MvmSentryBusterDetonate = 273,
    MvmScoutMarkedForDeath = 274,
    MvmMedicPowerUpShared = 275,
    MvmBeginWave = 276,
    MvmWaveComplete = 277,
    MvmMissionComplete = 278,
    MvmBombResetByPlayer = 279,
    MvmBombAlarmTriggered = 280,
    MvmBombDeployResetByPlayer = 281,
    MvmWaveFailed = 282,
    MvmResetStats = 283,
    DamageResisted = 284,
    RevivePlayerNotify = 285,
    RevivePlayerStopped = 286,
    RevivePlayerComplete = 287,
    PlayerTurnedToGhost = 288,
    MedigunShieldBlockedDamage = 289,
    MvmAdvWaveCompleteNoGates = 290,
    MvmSniperHeadshotCurrency = 291,
    MvmMannhattanPit = 292,
    FlagCarriedInDetectionZone = 293,
    MvmAdvWaveKilledStunRadio = 294,
    PlayerDirectHitStun = 295,
    MvmSentryBusterKilled = 296,
    UpgradesFileChanged = 297,
    RdTeamPointsChanged = 298,
    RdRulesStateChanged = 299,
    RdRobotKilled = 300,
    RdRobotImpact = 301,
    TeamPlayPreRoundTimeLeft = 302,
    ParachuteDeploy = 303,
    ParachuteHolster = 304,
    KillRefillsMeter = 305,
    RpsTauntEvent = 306,
    CongaKill = 307,
    PlayerInitialSpawn = 308,
    CompetitiveVictory = 309,
    CompetitiveStatsUpdate = 310,
    MiniGameWin = 311,
    SentryOnGoActive = 312,
    DuckXpLevelUp = 313,
    QuestLogOpened = 314,
    SchemaUpdated = 315,
    LocalPlayerPickupWeapon = 316,
    RdPlayerScorePoints = 317,
    DemomanDetStickies = 318,
    QuestObjectiveCompleted = 319,
    PlayerScoreChanged = 320,
    KilledCappingPlayer = 321,
    EnvironmentalDeath = 322,
    ProjectileDirectHit = 323,
    PassGet = 324,
    PassScore = 325,
    PassFree = 326,
    PassPassCaught = 327,
    PassBallStolen = 328,
    PassBallBlocked = 329,
    DamagePrevented = 330,
    HalloweenBossKilled = 331,
    EscapedLootIsland = 332,
    TaggedPlayerAsIt = 333,
    MerasmusStunned = 334,
    MerasmusPropFound = 335,
    HalloweenSkeletonKilled = 336,
    EscapeHell = 337,
    CrossSpectralBridge = 338,
    MiniGameWon = 339,
    RespawnGhost = 340,
    KillInHell = 341,
    HalloweenDuckCollected = 342,
    SpecialScore = 343,
    TeamLeaderKilled = 344,
    HalloweenSoulCollected = 345,
    RecalculateTruce = 346,
    DeadRingerCheatDeath = 347,
    CrossbowHeal = 348,
    DamageMitigated = 349,
    PayloadPushed = 350,
    PlayerAbandonedMatch = 351,
    ClDrawline = 352,
    RestartTimerTime = 353,
    WinLimitChanged = 354,
    WinPanelShowScores = 355,
    TopStreamsRequestFinished = 356,
    CompetitiveStateChanged = 357,
    GlobalWarDataUpdated = 358,
    StopWatchChanged = 359,
    DsStop = 360,
    DsScreenshot = 361,
    ShowMatchSummary = 362,
    ExperienceChanged = 363,
    BeginXpLerp = 364,
    MatchmakerStatsUpdated = 365,
    RematchVotePeriodOver = 366,
    RematchFailedToCreate = 367,
    PlayerRematchChange = 368,
    PingUpdated = 369,
    MMStatsUpdated = 370,
    PlayerNextMapVoteChange = 371,
    VoteMapsChanged = 372,
    ProtoDefChanged = 373,
    PlayerDomination = 374,
    PlayerRocketPackPushed = 375,
    QuestRequest = 376,
    QuestResponse = 377,
    QuestProgress = 378,
    ProjectileRemoved = 379,
    QuestMapDataChanged = 380,
    GasDousedPlayerIgnited = 381,
    QuestTurnInState = 382,
    ItemsAcknowledged = 383,
    CapperKilled = 384,
    MainMenuStabilized = 385,
    WorldStatusChanged = 386,
    HLTVStatus = 387,
    HLTVCameraman = 388,
    HLTVRankCamera = 389,
    HLTVRankEntity = 390,
    HLTVFixed = 391,
    HLTVChase = 392,
    HLTVMessage = 393,
    HLTVTitle = 394,
    HLTVChat = 395,
    ReplayStartRecord = 396,
    ReplaySessionInfo = 397,
    ReplayEndRecord = 398,
    ReplayReplaysAvailable = 399,
    ReplayServerError = 400,
    Unknown,
}
impl GameEventType {
    pub fn from_type_name(name: &str) -> Self {
        match name {
            "server_spawn" => GameEventType::ServerSpawn,
            "server_changelevel_failed" => GameEventType::ServerChangeLevelFailed,
            "server_shutdown" => GameEventType::ServerShutdown,
            "server_cvar" => GameEventType::ServerCvar,
            "server_message" => GameEventType::ServerMessage,
            "server_addban" => GameEventType::ServerAddBan,
            "server_removeban" => GameEventType::ServerRemoveBan,
            "player_connect" => GameEventType::PlayerConnect,
            "player_connect_client" => GameEventType::PlayerConnectClient,
            "player_info" => GameEventType::PlayerInfo,
            "player_disconnect" => GameEventType::PlayerDisconnect,
            "player_activate" => GameEventType::PlayerActivate,
            "player_say" => GameEventType::PlayerSay,
            "client_disconnect" => GameEventType::ClientDisconnect,
            "client_beginconnect" => GameEventType::ClientBeginConnect,
            "client_connected" => GameEventType::ClientConnected,
            "client_fullconnect" => GameEventType::ClientFullConnect,
            "host_quit" => GameEventType::HostQuit,
            "team_info" => GameEventType::TeamInfo,
            "team_score" => GameEventType::TeamScore,
            "teamplay_broadcast_audio" => GameEventType::TeamPlayBroadcastAudio,
            "player_team" => GameEventType::PlayerTeam,
            "player_class" => GameEventType::PlayerClass,
            "player_death" => GameEventType::PlayerDeath,
            "player_hurt" => GameEventType::PlayerHurt,
            "player_chat" => GameEventType::PlayerChat,
            "player_score" => GameEventType::PlayerScore,
            "player_spawn" => GameEventType::PlayerSpawn,
            "player_shoot" => GameEventType::PlayerShoot,
            "player_use" => GameEventType::PlayerUse,
            "player_changename" => GameEventType::PlayerChangeName,
            "player_hintmessage" => GameEventType::PlayerHintMessage,
            "base_player_teleported" => GameEventType::BasePlayerTeleported,
            "game_init" => GameEventType::GameInit,
            "game_newmap" => GameEventType::GameNewMap,
            "game_start" => GameEventType::GameStart,
            "game_end" => GameEventType::GameEnd,
            "round_start" => GameEventType::RoundStart,
            "round_end" => GameEventType::RoundEnd,
            "game_message" => GameEventType::GameMessage,
            "break_breakable" => GameEventType::BreakBreakable,
            "break_prop" => GameEventType::BreakProp,
            "entity_killed" => GameEventType::EntityKilled,
            "bonus_updated" => GameEventType::BonusUpdated,
            "achievement_event" => GameEventType::AchievementEvent,
            "achievement_increment" => GameEventType::AchievementIncrement,
            "physgun_pickup" => GameEventType::PhysgunPickup,
            "flare_ignite_npc" => GameEventType::FlareIgniteNpc,
            "helicopter_grenade_punt_miss" => GameEventType::HelicopterGrenadePuntMiss,
            "user_data_downloaded" => GameEventType::UserDataDownloaded,
            "ragdoll_dissolved" => GameEventType::RagdollDissolved,
            "hltv_changed_mode" => GameEventType::HLTVChangedMode,
            "hltv_changed_target" => GameEventType::HLTVChangedTarget,
            "vote_ended" => GameEventType::VoteEnded,
            "vote_started" => GameEventType::VoteStarted,
            "vote_changed" => GameEventType::VoteChanged,
            "vote_passed" => GameEventType::VotePassed,
            "vote_failed" => GameEventType::VoteFailed,
            "vote_cast" => GameEventType::VoteCast,
            "vote_options" => GameEventType::VoteOptions,
            "replay_saved" => GameEventType::ReplaySaved,
            "entered_performance_mode" => GameEventType::EnteredPerformanceMode,
            "browse_replays" => GameEventType::BrowseReplays,
            "replay_youtube_stats" => GameEventType::ReplayYoutubeStats,
            "inventory_updated" => GameEventType::InventoryUpdated,
            "cart_updated" => GameEventType::CartUpdated,
            "store_pricesheet_updated" => GameEventType::StorePriceSheetUpdated,
            "econ_inventory_connected" => GameEventType::EconInventoryConnected,
            "item_schema_initialized" => GameEventType::ItemSchemaInitialized,
            "gc_new_session" => GameEventType::GcNewSession,
            "gc_lost_session" => GameEventType::GcLostSession,
            "intro_finish" => GameEventType::IntroFinish,
            "intro_nextcamera" => GameEventType::IntroNextCamera,
            "player_changeclass" => GameEventType::PlayerChangeClass,
            "tf_map_time_remaining" => GameEventType::TfMapTimeRemaining,
            "tf_game_over" => GameEventType::TfGameOver,
            "ctf_flag_captured" => GameEventType::CtfFlagCaptured,
            "controlpoint_initialized" => GameEventType::ControlPointInitialized,
            "controlpoint_updateimages" => GameEventType::ControlPointUpdateImages,
            "controlpoint_updatelayout" => GameEventType::ControlPointUpdateLayout,
            "controlpoint_updatecapping" => GameEventType::ControlPointUpdateCapping,
            "controlpoint_updateowner" => GameEventType::ControlPointUpdateOwner,
            "controlpoint_starttouch" => GameEventType::ControlPointStartTouch,
            "controlpoint_endtouch" => GameEventType::ControlPointEndTouch,
            "controlpoint_pulse_element" => GameEventType::ControlPointPulseElement,
            "controlpoint_fake_capture" => GameEventType::ControlPointFakeCapture,
            "controlpoint_fake_capture_mult" => GameEventType::ControlPointFakeCaptureMultiplier,
            "teamplay_round_selected" => GameEventType::TeamPlayRoundSelected,
            "teamplay_round_start" => GameEventType::TeamPlayRoundStart,
            "teamplay_round_active" => GameEventType::TeamPlayRoundActive,
            "teamplay_waiting_begins" => GameEventType::TeamPlayWaitingBegins,
            "teamplay_waiting_ends" => GameEventType::TeamPlayWaitingEnds,
            "teamplay_waiting_abouttoend" => GameEventType::TeamPlayWaitingAboutToEnd,
            "teamplay_restart_round" => GameEventType::TeamPlayRestartRound,
            "teamplay_ready_restart" => GameEventType::TeamPlayReadyRestart,
            "teamplay_round_restart_seconds" => GameEventType::TeamPlayRoundRestartSeconds,
            "teamplay_team_ready" => GameEventType::TeamPlayTeamReady,
            "teamplay_round_win" => GameEventType::TeamPlayRoundWin,
            "teamplay_update_timer" => GameEventType::TeamPlayUpdateTimer,
            "teamplay_round_stalemate" => GameEventType::TeamPlayRoundStalemate,
            "teamplay_overtime_begin" => GameEventType::TeamPlayOvertimeBegin,
            "teamplay_overtime_end" => GameEventType::TeamPlayOvertimeEnd,
            "teamplay_suddendeath_begin" => GameEventType::TeamPlaySuddenDeathBegin,
            "teamplay_suddendeath_end" => GameEventType::TeamPlaySuddenDeathEnd,
            "teamplay_game_over" => GameEventType::TeamPlayGameOver,
            "teamplay_map_time_remaining" => GameEventType::TeamPlayMapTimeRemaining,
            "teamplay_timer_flash" => GameEventType::TeamPlayTimerFlash,
            "teamplay_timer_time_added" => GameEventType::TeamPlayTimerTimeAdded,
            "teamplay_point_startcapture" => GameEventType::TeamPlayPointStartCapture,
            "teamplay_point_captured" => GameEventType::TeamPlayPointCaptured,
            "teamplay_point_locked" => GameEventType::TeamPlayPointLocked,
            "teamplay_point_unlocked" => GameEventType::TeamPlayPointUnlocked,
            "teamplay_capture_broken" => GameEventType::TeamPlayCaptureBroken,
            "teamplay_capture_blocked" => GameEventType::TeamPlayCaptureBlocked,
            "teamplay_flag_event" => GameEventType::TeamPlayFlagEvent,
            "teamplay_win_panel" => GameEventType::TeamPlayWinPanel,
            "teamplay_teambalanced_player" => GameEventType::TeamPlayTeamBalancedPlayer,
            "teamplay_setup_finished" => GameEventType::TeamPlaySetupFinished,
            "teamplay_alert" => GameEventType::TeamPlayAlert,
            "training_complete" => GameEventType::TrainingComplete,
            "show_freezepanel" => GameEventType::ShowFreezePanel,
            "hide_freezepanel" => GameEventType::HideFreezePanel,
            "freezecam_started" => GameEventType::FreezeCamStarted,
            "localplayer_changeteam" => GameEventType::LocalPlayerChangeTeam,
            "localplayer_score_changed" => GameEventType::LocalPlayerScoreChanged,
            "localplayer_changeclass" => GameEventType::LocalPlayerChangeClass,
            "localplayer_respawn" => GameEventType::LocalPlayerRespawn,
            "building_info_changed" => GameEventType::BuildingInfoChanged,
            "localplayer_changedisguise" => GameEventType::LocalPlayerChangeDisguise,
            "player_account_changed" => GameEventType::PlayerAccountChanged,
            "spy_pda_reset" => GameEventType::SpyPdaReset,
            "flagstatus_update" => GameEventType::FlagStatusUpdate,
            "player_stats_updated" => GameEventType::PlayerStatsUpdated,
            "playing_commentary" => GameEventType::PlayingCommentary,
            "player_chargedeployed" => GameEventType::PlayerChargeDeployed,
            "player_builtobject" => GameEventType::PlayerBuiltObject,
            "player_upgradedobject" => GameEventType::PlayerUpgradedObject,
            "player_carryobject" => GameEventType::PlayerCarryObject,
            "player_dropobject" => GameEventType::PlayerDropObject,
            "object_removed" => GameEventType::ObjectRemoved,
            "object_destroyed" => GameEventType::ObjectDestroyed,
            "object_detonated" => GameEventType::ObjectDetonated,
            "achievement_earned" => GameEventType::AchievementEarned,
            "spec_target_updated" => GameEventType::SpecTargetUpdated,
            "tournament_stateupdate" => GameEventType::TournamentStateUpdate,
            "tournament_enablecountdown" => GameEventType::TournamentEnableCountdown,
            "player_calledformedic" => GameEventType::PlayerCalledForMedic,
            "player_askedforball" => GameEventType::PlayerAskedForBall,
            "localplayer_becameobserver" => GameEventType::LocalPlayerBecameObserver,
            "player_ignited_inv" => GameEventType::PlayerIgnitedInv,
            "player_ignited" => GameEventType::PlayerIgnited,
            "player_extinguished" => GameEventType::PlayerExtinguished,
            "player_teleported" => GameEventType::PlayerTeleported,
            "player_healedmediccall" => GameEventType::PlayerHealedMedicCall,
            "localplayer_chargeready" => GameEventType::LocalPlayerChargeReady,
            "localplayer_winddown" => GameEventType::LocalPlayerWindDown,
            "player_invulned" => GameEventType::PlayerInvulned,
            "escort_speed" => GameEventType::EscortSpeed,
            "escort_progress" => GameEventType::EscortProgress,
            "escort_recede" => GameEventType::EscortRecede,
            "gameui_activated" => GameEventType::GameUIActivated,
            "gameui_hidden" => GameEventType::GameUIHidden,
            "player_escort_score" => GameEventType::PlayerEscortScore,
            "player_healonhit" => GameEventType::PlayerHealOnHit,
            "player_stealsandvich" => GameEventType::PlayerStealSandvich,
            "show_class_layout" => GameEventType::ShowClassLayout,
            "show_vs_panel" => GameEventType::ShowVsPanel,
            "player_damaged" => GameEventType::PlayerDamaged,
            "arena_player_notification" => GameEventType::ArenaPlayerNotification,
            "arena_match_maxstreak" => GameEventType::ArenaMatchMaxStreak,
            "arena_round_start" => GameEventType::ArenaRoundStart,
            "arena_win_panel" => GameEventType::ArenaWinPanel,
            "pve_win_panel" => GameEventType::PveWinPanel,
            "air_dash" => GameEventType::AirDash,
            "landed" => GameEventType::Landed,
            "player_damage_dodged" => GameEventType::PlayerDamageDodged,
            "player_stunned" => GameEventType::PlayerStunned,
            "scout_grand_slam" => GameEventType::ScoutGrandSlam,
            "scout_slamdoll_landed" => GameEventType::ScoutSlamdollLanded,
            "arrow_impact" => GameEventType::ArrowImpact,
            "player_jarated" => GameEventType::PlayerJarated,
            "player_jarated_fade" => GameEventType::PlayerJaratedFade,
            "player_shield_blocked" => GameEventType::PlayerShieldBlocked,
            "player_pinned" => GameEventType::PlayerPinned,
            "player_healedbymedic" => GameEventType::PlayerHealedByMedic,
            "player_sapped_object" => GameEventType::PlayerSappedObject,
            "item_found" => GameEventType::ItemFound,
            "show_annotation" => GameEventType::ShowAnnotation,
            "hide_annotation" => GameEventType::HideAnnotation,
            "post_inventory_application" => GameEventType::PostInventoryApplication,
            "controlpoint_unlock_updated" => GameEventType::ControlPointUnlockUpdated,
            "deploy_buff_banner" => GameEventType::DeployBuffBanner,
            "player_buff" => GameEventType::PlayerBuff,
            "medic_death" => GameEventType::MedicDeath,
            "overtime_nag" => GameEventType::OvertimeNag,
            "teams_changed" => GameEventType::TeamsChanged,
            "halloween_pumpkin_grab" => GameEventType::HalloweenPumpkinGrab,
            "rocket_jump" => GameEventType::RocketJump,
            "rocket_jump_landed" => GameEventType::RocketJumpLanded,
            "sticky_jump" => GameEventType::StickyJump,
            "sticky_jump_landed" => GameEventType::StickyJumpLanded,
            "rocketpack_launch" => GameEventType::RocketPackLaunch,
            "rocketpack_landed" => GameEventType::RocketPackLanded,
            "medic_defended" => GameEventType::MedicDefended,
            "localplayer_healed" => GameEventType::LocalPlayerHealed,
            "player_destroyed_pipebomb" => GameEventType::PlayerDestroyedPipeBomb,
            "object_deflected" => GameEventType::ObjectDeflected,
            "player_mvp" => GameEventType::PlayerMvp,
            "raid_spawn_mob" => GameEventType::RaidSpawnMob,
            "raid_spawn_squad" => GameEventType::RaidSpawnSquad,
            "nav_blocked" => GameEventType::NavBlocked,
            "path_track_passed" => GameEventType::PathTrackPassed,
            "num_cappers_changed" => GameEventType::NumCappersChanged,
            "player_regenerate" => GameEventType::PlayerRegenerate,
            "update_status_item" => GameEventType::UpdateStatusItem,
            "stats_resetround" => GameEventType::StatsResetRound,
            "scorestats_accumulated_update" => GameEventType::ScoreStatsAccumulatedUpdate,
            "scorestats_accumulated_reset" => GameEventType::ScoreStatsAccumulatedReset,
            "achievement_earned_local" => GameEventType::AchievementEarnedLocal,
            "player_healed" => GameEventType::PlayerHealed,
            "building_healed" => GameEventType::BuildingHealed,
            "item_pickup" => GameEventType::ItemPickup,
            "duel_status" => GameEventType::DuelStatus,
            "fish_notice" => GameEventType::FishNotice,
            "fish_notice__arm" => GameEventType::FishNoticeArm,
            "slap_notice" => GameEventType::SlapNotice,
            "throwable_hit" => GameEventType::ThrowableHit,
            "pumpkin_lord_summoned" => GameEventType::PumpkinLordSummoned,
            "pumpkin_lord_killed" => GameEventType::PumpkinLordKilled,
            "merasmus_summoned" => GameEventType::MerasmusSummoned,
            "merasmus_killed" => GameEventType::MerasmusKilled,
            "merasmus_escape_warning" => GameEventType::MerasmusEscapeWarning,
            "merasmus_escaped" => GameEventType::MerasmusEscaped,
            "eyeball_boss_summoned" => GameEventType::EyeballBossSummoned,
            "eyeball_boss_stunned" => GameEventType::EyeballBossStunned,
            "eyeball_boss_killed" => GameEventType::EyeballBossKilled,
            "eyeball_boss_killer" => GameEventType::EyeballBossKiller,
            "eyeball_boss_escape_imminent" => GameEventType::EyeballBossEscapeImminent,
            "eyeball_boss_escaped" => GameEventType::EyeballBossEscaped,
            "npc_hurt" => GameEventType::NpcHurt,
            "controlpoint_timer_updated" => GameEventType::ControlPointTimerUpdated,
            "player_highfive_start" => GameEventType::PlayerHighFiveStart,
            "player_highfive_cancel" => GameEventType::PlayerHighFiveCancel,
            "player_highfive_success" => GameEventType::PlayerHighFiveSuccess,
            "player_bonuspoints" => GameEventType::PlayerBonusPoints,
            "player_upgraded" => GameEventType::PlayerUpgraded,
            "player_buyback" => GameEventType::PlayerBuyback,
            "player_used_powerup_bottle" => GameEventType::PlayerUsedPowerUpBottle,
            "christmas_gift_grab" => GameEventType::ChristmasGiftGrab,
            "player_killed_achievement_zone" => GameEventType::PlayerKilledAchievementZone,
            "party_updated" => GameEventType::PartyUpdated,
            "party_pref_changed" => GameEventType::PartyPrefChanged,
            "party_criteria_changed" => GameEventType::PartyCriteriaChanged,
            "party_invites_changed" => GameEventType::PartyInvitesChanged,
            "party_queue_state_changed" => GameEventType::PartyQueueStateChanged,
            "party_chat" => GameEventType::PartyChat,
            "party_member_join" => GameEventType::PartyMemberJoin,
            "party_member_leave" => GameEventType::PartyMemberLeave,
            "match_invites_updated" => GameEventType::MatchInvitesUpdated,
            "lobby_updated" => GameEventType::LobbyUpdated,
            "mvm_mission_update" => GameEventType::MvmMissionUpdate,
            "recalculate_holidays" => GameEventType::RecalculateHolidays,
            "player_currency_changed" => GameEventType::PlayerCurrencyChanged,
            "doomsday_rocket_open" => GameEventType::DoomsdayRocketOpen,
            "remove_nemesis_relationships" => GameEventType::RemoveNemesisRelationships,
            "mvm_creditbonus_wave" => GameEventType::MvmCreditBonusWave,
            "mvm_creditbonus_all" => GameEventType::MvmCreditBonusAll,
            "mvm_creditbonus_all_advanced" => GameEventType::MvmCreditBonusAllAdvanced,
            "mvm_quick_sentry_upgrade" => GameEventType::MvmQuickSentryUpgrade,
            "mvm_tank_destroyed_by_players" => GameEventType::MvmTankDestroyedByPlayers,
            "mvm_kill_robot_delivering_bomb" => GameEventType::MvmKillRobotDeliveringBomb,
            "mvm_pickup_currency" => GameEventType::MvmPickupCurrency,
            "mvm_bomb_carrier_killed" => GameEventType::MvmBombCarrierKilled,
            "mvm_sentrybuster_detonate" => GameEventType::MvmSentryBusterDetonate,
            "mvm_scout_marked_for_death" => GameEventType::MvmScoutMarkedForDeath,
            "mvm_medic_powerup_shared" => GameEventType::MvmMedicPowerUpShared,
            "mvm_begin_wave" => GameEventType::MvmBeginWave,
            "mvm_wave_complete" => GameEventType::MvmWaveComplete,
            "mvm_mission_complete" => GameEventType::MvmMissionComplete,
            "mvm_bomb_reset_by_player" => GameEventType::MvmBombResetByPlayer,
            "mvm_bomb_alarm_triggered" => GameEventType::MvmBombAlarmTriggered,
            "mvm_bomb_deploy_reset_by_player" => GameEventType::MvmBombDeployResetByPlayer,
            "mvm_wave_failed" => GameEventType::MvmWaveFailed,
            "mvm_reset_stats" => GameEventType::MvmResetStats,
            "damage_resisted" => GameEventType::DamageResisted,
            "revive_player_notify" => GameEventType::RevivePlayerNotify,
            "revive_player_stopped" => GameEventType::RevivePlayerStopped,
            "revive_player_complete" => GameEventType::RevivePlayerComplete,
            "player_turned_to_ghost" => GameEventType::PlayerTurnedToGhost,
            "medigun_shield_blocked_damage" => GameEventType::MedigunShieldBlockedDamage,
            "mvm_adv_wave_complete_no_gates" => GameEventType::MvmAdvWaveCompleteNoGates,
            "mvm_sniper_headshot_currency" => GameEventType::MvmSniperHeadshotCurrency,
            "mvm_mannhattan_pit" => GameEventType::MvmMannhattanPit,
            "flag_carried_in_detection_zone" => GameEventType::FlagCarriedInDetectionZone,
            "mvm_adv_wave_killed_stun_radio" => GameEventType::MvmAdvWaveKilledStunRadio,
            "player_directhit_stun" => GameEventType::PlayerDirectHitStun,
            "mvm_sentrybuster_killed" => GameEventType::MvmSentryBusterKilled,
            "upgrades_file_changed" => GameEventType::UpgradesFileChanged,
            "rd_team_points_changed" => GameEventType::RdTeamPointsChanged,
            "rd_rules_state_changed" => GameEventType::RdRulesStateChanged,
            "rd_robot_killed" => GameEventType::RdRobotKilled,
            "rd_robot_impact" => GameEventType::RdRobotImpact,
            "teamplay_pre_round_time_left" => GameEventType::TeamPlayPreRoundTimeLeft,
            "parachute_deploy" => GameEventType::ParachuteDeploy,
            "parachute_holster" => GameEventType::ParachuteHolster,
            "kill_refills_meter" => GameEventType::KillRefillsMeter,
            "rps_taunt_event" => GameEventType::RpsTauntEvent,
            "conga_kill" => GameEventType::CongaKill,
            "player_initial_spawn" => GameEventType::PlayerInitialSpawn,
            "competitive_victory" => GameEventType::CompetitiveVictory,
            "competitive_stats_update" => GameEventType::CompetitiveStatsUpdate,
            "minigame_win" => GameEventType::MiniGameWin,
            "sentry_on_go_active" => GameEventType::SentryOnGoActive,
            "duck_xp_level_up" => GameEventType::DuckXpLevelUp,
            "questlog_opened" => GameEventType::QuestLogOpened,
            "schema_updated" => GameEventType::SchemaUpdated,
            "localplayer_pickup_weapon" => GameEventType::LocalPlayerPickupWeapon,
            "rd_player_score_points" => GameEventType::RdPlayerScorePoints,
            "demoman_det_stickies" => GameEventType::DemomanDetStickies,
            "quest_objective_completed" => GameEventType::QuestObjectiveCompleted,
            "player_score_changed" => GameEventType::PlayerScoreChanged,
            "killed_capping_player" => GameEventType::KilledCappingPlayer,
            "environmental_death" => GameEventType::EnvironmentalDeath,
            "projectile_direct_hit" => GameEventType::ProjectileDirectHit,
            "pass_get" => GameEventType::PassGet,
            "pass_score" => GameEventType::PassScore,
            "pass_free" => GameEventType::PassFree,
            "pass_pass_caught" => GameEventType::PassPassCaught,
            "pass_ball_stolen" => GameEventType::PassBallStolen,
            "pass_ball_blocked" => GameEventType::PassBallBlocked,
            "damage_prevented" => GameEventType::DamagePrevented,
            "halloween_boss_killed" => GameEventType::HalloweenBossKilled,
            "escaped_loot_island" => GameEventType::EscapedLootIsland,
            "tagged_player_as_it" => GameEventType::TaggedPlayerAsIt,
            "merasmus_stunned" => GameEventType::MerasmusStunned,
            "merasmus_prop_found" => GameEventType::MerasmusPropFound,
            "halloween_skeleton_killed" => GameEventType::HalloweenSkeletonKilled,
            "escape_hell" => GameEventType::EscapeHell,
            "cross_spectral_bridge" => GameEventType::CrossSpectralBridge,
            "minigame_won" => GameEventType::MiniGameWon,
            "respawn_ghost" => GameEventType::RespawnGhost,
            "kill_in_hell" => GameEventType::KillInHell,
            "halloween_duck_collected" => GameEventType::HalloweenDuckCollected,
            "special_score" => GameEventType::SpecialScore,
            "team_leader_killed" => GameEventType::TeamLeaderKilled,
            "halloween_soul_collected" => GameEventType::HalloweenSoulCollected,
            "recalculate_truce" => GameEventType::RecalculateTruce,
            "deadringer_cheat_death" => GameEventType::DeadRingerCheatDeath,
            "crossbow_heal" => GameEventType::CrossbowHeal,
            "damage_mitigated" => GameEventType::DamageMitigated,
            "payload_pushed" => GameEventType::PayloadPushed,
            "player_abandoned_match" => GameEventType::PlayerAbandonedMatch,
            "cl_drawline" => GameEventType::ClDrawline,
            "restart_timer_time" => GameEventType::RestartTimerTime,
            "winlimit_changed" => GameEventType::WinLimitChanged,
            "winpanel_show_scores" => GameEventType::WinPanelShowScores,
            "top_streams_request_finished" => GameEventType::TopStreamsRequestFinished,
            "competitive_state_changed" => GameEventType::CompetitiveStateChanged,
            "global_war_data_updated" => GameEventType::GlobalWarDataUpdated,
            "stop_watch_changed" => GameEventType::StopWatchChanged,
            "ds_stop" => GameEventType::DsStop,
            "ds_screenshot" => GameEventType::DsScreenshot,
            "show_match_summary" => GameEventType::ShowMatchSummary,
            "experience_changed" => GameEventType::ExperienceChanged,
            "begin_xp_lerp" => GameEventType::BeginXpLerp,
            "matchmaker_stats_updated" => GameEventType::MatchmakerStatsUpdated,
            "rematch_vote_period_over" => GameEventType::RematchVotePeriodOver,
            "rematch_failed_to_create" => GameEventType::RematchFailedToCreate,
            "player_rematch_change" => GameEventType::PlayerRematchChange,
            "ping_updated" => GameEventType::PingUpdated,
            "mmstats_updated" => GameEventType::MMStatsUpdated,
            "player_next_map_vote_change" => GameEventType::PlayerNextMapVoteChange,
            "vote_maps_changed" => GameEventType::VoteMapsChanged,
            "proto_def_changed" => GameEventType::ProtoDefChanged,
            "player_domination" => GameEventType::PlayerDomination,
            "player_rocketpack_pushed" => GameEventType::PlayerRocketPackPushed,
            "quest_request" => GameEventType::QuestRequest,
            "quest_response" => GameEventType::QuestResponse,
            "quest_progress" => GameEventType::QuestProgress,
            "projectile_removed" => GameEventType::ProjectileRemoved,
            "quest_map_data_changed" => GameEventType::QuestMapDataChanged,
            "gas_doused_player_ignited" => GameEventType::GasDousedPlayerIgnited,
            "quest_turn_in_state" => GameEventType::QuestTurnInState,
            "items_acknowledged" => GameEventType::ItemsAcknowledged,
            "capper_killed" => GameEventType::CapperKilled,
            "mainmenu_stabilized" => GameEventType::MainMenuStabilized,
            "world_status_changed" => GameEventType::WorldStatusChanged,
            "hltv_status" => GameEventType::HLTVStatus,
            "hltv_cameraman" => GameEventType::HLTVCameraman,
            "hltv_rank_camera" => GameEventType::HLTVRankCamera,
            "hltv_rank_entity" => GameEventType::HLTVRankEntity,
            "hltv_fixed" => GameEventType::HLTVFixed,
            "hltv_chase" => GameEventType::HLTVChase,
            "hltv_message" => GameEventType::HLTVMessage,
            "hltv_title" => GameEventType::HLTVTitle,
            "hltv_chat" => GameEventType::HLTVChat,
            "replay_startrecord" => GameEventType::ReplayStartRecord,
            "replay_sessioninfo" => GameEventType::ReplaySessionInfo,
            "replay_endrecord" => GameEventType::ReplayEndRecord,
            "replay_replaysavailable" => GameEventType::ReplayReplaysAvailable,
            "replay_servererror" => GameEventType::ReplayServerError,
            _ => GameEventType::Unknown,
        }
    }
}
impl GameEvent {
    pub fn from_raw_event(event: RawGameEvent) -> Result<Self> {
        Ok(match event.event_type {
            GameEventType::ServerSpawn => {
                GameEvent::ServerSpawn(ServerSpawnEvent::from_raw_event(event.values)?)
            }
            GameEventType::ServerChangeLevelFailed => GameEvent::ServerChangeLevelFailed(
                ServerChangeLevelFailedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ServerShutdown => {
                GameEvent::ServerShutdown(ServerShutdownEvent::from_raw_event(event.values)?)
            }
            GameEventType::ServerCvar => {
                GameEvent::ServerCvar(ServerCvarEvent::from_raw_event(event.values)?)
            }
            GameEventType::ServerMessage => {
                GameEvent::ServerMessage(ServerMessageEvent::from_raw_event(event.values)?)
            }
            GameEventType::ServerAddBan => {
                GameEvent::ServerAddBan(ServerAddBanEvent::from_raw_event(event.values)?)
            }
            GameEventType::ServerRemoveBan => {
                GameEvent::ServerRemoveBan(ServerRemoveBanEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerConnect => {
                GameEvent::PlayerConnect(PlayerConnectEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerConnectClient => GameEvent::PlayerConnectClient(
                PlayerConnectClientEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerInfo => {
                GameEvent::PlayerInfo(PlayerInfoEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDisconnect => {
                GameEvent::PlayerDisconnect(PlayerDisconnectEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerActivate => {
                GameEvent::PlayerActivate(PlayerActivateEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerSay => {
                GameEvent::PlayerSay(PlayerSayEvent::from_raw_event(event.values)?)
            }
            GameEventType::ClientDisconnect => {
                GameEvent::ClientDisconnect(ClientDisconnectEvent::from_raw_event(event.values)?)
            }
            GameEventType::ClientBeginConnect => GameEvent::ClientBeginConnect(
                ClientBeginConnectEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ClientConnected => {
                GameEvent::ClientConnected(ClientConnectedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ClientFullConnect => {
                GameEvent::ClientFullConnect(ClientFullConnectEvent::from_raw_event(event.values)?)
            }
            GameEventType::HostQuit => {
                GameEvent::HostQuit(HostQuitEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamInfo => {
                GameEvent::TeamInfo(TeamInfoEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamScore => {
                GameEvent::TeamScore(TeamScoreEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayBroadcastAudio => GameEvent::TeamPlayBroadcastAudio(
                TeamPlayBroadcastAudioEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerTeam => {
                GameEvent::PlayerTeam(PlayerTeamEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerClass => {
                GameEvent::PlayerClass(PlayerClassEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDeath => {
                GameEvent::PlayerDeath(PlayerDeathEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerHurt => {
                GameEvent::PlayerHurt(PlayerHurtEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerChat => {
                GameEvent::PlayerChat(PlayerChatEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerScore => {
                GameEvent::PlayerScore(PlayerScoreEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerSpawn => {
                GameEvent::PlayerSpawn(PlayerSpawnEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerShoot => {
                GameEvent::PlayerShoot(PlayerShootEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerUse => {
                GameEvent::PlayerUse(PlayerUseEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerChangeName => {
                GameEvent::PlayerChangeName(PlayerChangeNameEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerHintMessage => {
                GameEvent::PlayerHintMessage(PlayerHintMessageEvent::from_raw_event(event.values)?)
            }
            GameEventType::BasePlayerTeleported => GameEvent::BasePlayerTeleported(
                BasePlayerTeleportedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::GameInit => {
                GameEvent::GameInit(GameInitEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameNewMap => {
                GameEvent::GameNewMap(GameNewMapEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameStart => {
                GameEvent::GameStart(GameStartEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameEnd => {
                GameEvent::GameEnd(GameEndEvent::from_raw_event(event.values)?)
            }
            GameEventType::RoundStart => {
                GameEvent::RoundStart(RoundStartEvent::from_raw_event(event.values)?)
            }
            GameEventType::RoundEnd => {
                GameEvent::RoundEnd(RoundEndEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameMessage => {
                GameEvent::GameMessage(GameMessageEvent::from_raw_event(event.values)?)
            }
            GameEventType::BreakBreakable => {
                GameEvent::BreakBreakable(BreakBreakableEvent::from_raw_event(event.values)?)
            }
            GameEventType::BreakProp => {
                GameEvent::BreakProp(BreakPropEvent::from_raw_event(event.values)?)
            }
            GameEventType::EntityKilled => {
                GameEvent::EntityKilled(EntityKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::BonusUpdated => {
                GameEvent::BonusUpdated(BonusUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::AchievementEvent => {
                GameEvent::AchievementEvent(AchievementEventEvent::from_raw_event(event.values)?)
            }
            GameEventType::AchievementIncrement => GameEvent::AchievementIncrement(
                AchievementIncrementEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PhysgunPickup => {
                GameEvent::PhysgunPickup(PhysgunPickupEvent::from_raw_event(event.values)?)
            }
            GameEventType::FlareIgniteNpc => {
                GameEvent::FlareIgniteNpc(FlareIgniteNpcEvent::from_raw_event(event.values)?)
            }
            GameEventType::HelicopterGrenadePuntMiss => GameEvent::HelicopterGrenadePuntMiss(
                HelicopterGrenadePuntMissEvent::from_raw_event(event.values)?,
            ),
            GameEventType::UserDataDownloaded => GameEvent::UserDataDownloaded(
                UserDataDownloadedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RagdollDissolved => {
                GameEvent::RagdollDissolved(RagdollDissolvedEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVChangedMode => {
                GameEvent::HLTVChangedMode(HLTVChangedModeEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVChangedTarget => {
                GameEvent::HLTVChangedTarget(HLTVChangedTargetEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteEnded => {
                GameEvent::VoteEnded(VoteEndedEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteStarted => {
                GameEvent::VoteStarted(VoteStartedEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteChanged => {
                GameEvent::VoteChanged(VoteChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::VotePassed => {
                GameEvent::VotePassed(VotePassedEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteFailed => {
                GameEvent::VoteFailed(VoteFailedEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteCast => {
                GameEvent::VoteCast(VoteCastEvent::from_raw_event(event.values)?)
            }
            GameEventType::VoteOptions => {
                GameEvent::VoteOptions(VoteOptionsEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplaySaved => {
                GameEvent::ReplaySaved(ReplaySavedEvent::from_raw_event(event.values)?)
            }
            GameEventType::EnteredPerformanceMode => GameEvent::EnteredPerformanceMode(
                EnteredPerformanceModeEvent::from_raw_event(event.values)?,
            ),
            GameEventType::BrowseReplays => {
                GameEvent::BrowseReplays(BrowseReplaysEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplayYoutubeStats => GameEvent::ReplayYoutubeStats(
                ReplayYoutubeStatsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::InventoryUpdated => {
                GameEvent::InventoryUpdated(InventoryUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::CartUpdated => {
                GameEvent::CartUpdated(CartUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::StorePriceSheetUpdated => GameEvent::StorePriceSheetUpdated(
                StorePriceSheetUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EconInventoryConnected => GameEvent::EconInventoryConnected(
                EconInventoryConnectedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ItemSchemaInitialized => GameEvent::ItemSchemaInitialized(
                ItemSchemaInitializedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::GcNewSession => {
                GameEvent::GcNewSession(GcNewSessionEvent::from_raw_event(event.values)?)
            }
            GameEventType::GcLostSession => {
                GameEvent::GcLostSession(GcLostSessionEvent::from_raw_event(event.values)?)
            }
            GameEventType::IntroFinish => {
                GameEvent::IntroFinish(IntroFinishEvent::from_raw_event(event.values)?)
            }
            GameEventType::IntroNextCamera => {
                GameEvent::IntroNextCamera(IntroNextCameraEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerChangeClass => {
                GameEvent::PlayerChangeClass(PlayerChangeClassEvent::from_raw_event(event.values)?)
            }
            GameEventType::TfMapTimeRemaining => GameEvent::TfMapTimeRemaining(
                TfMapTimeRemainingEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TfGameOver => {
                GameEvent::TfGameOver(TfGameOverEvent::from_raw_event(event.values)?)
            }
            GameEventType::CtfFlagCaptured => {
                GameEvent::CtfFlagCaptured(CtfFlagCapturedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ControlPointInitialized => GameEvent::ControlPointInitialized(
                ControlPointInitializedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointUpdateImages => GameEvent::ControlPointUpdateImages(
                ControlPointUpdateImagesEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointUpdateLayout => GameEvent::ControlPointUpdateLayout(
                ControlPointUpdateLayoutEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointUpdateCapping => GameEvent::ControlPointUpdateCapping(
                ControlPointUpdateCappingEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointUpdateOwner => GameEvent::ControlPointUpdateOwner(
                ControlPointUpdateOwnerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointStartTouch => GameEvent::ControlPointStartTouch(
                ControlPointStartTouchEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointEndTouch => GameEvent::ControlPointEndTouch(
                ControlPointEndTouchEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointPulseElement => GameEvent::ControlPointPulseElement(
                ControlPointPulseElementEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointFakeCapture => GameEvent::ControlPointFakeCapture(
                ControlPointFakeCaptureEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointFakeCaptureMultiplier => {
                GameEvent::ControlPointFakeCaptureMultiplier(
                    ControlPointFakeCaptureMultiplierEvent::from_raw_event(event.values)?,
                )
            }
            GameEventType::TeamPlayRoundSelected => GameEvent::TeamPlayRoundSelected(
                TeamPlayRoundSelectedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayRoundStart => GameEvent::TeamPlayRoundStart(
                TeamPlayRoundStartEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayRoundActive => GameEvent::TeamPlayRoundActive(
                TeamPlayRoundActiveEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayWaitingBegins => GameEvent::TeamPlayWaitingBegins(
                TeamPlayWaitingBeginsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayWaitingEnds => GameEvent::TeamPlayWaitingEnds(
                TeamPlayWaitingEndsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayWaitingAboutToEnd => GameEvent::TeamPlayWaitingAboutToEnd(
                TeamPlayWaitingAboutToEndEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayRestartRound => GameEvent::TeamPlayRestartRound(
                TeamPlayRestartRoundEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayReadyRestart => GameEvent::TeamPlayReadyRestart(
                TeamPlayReadyRestartEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayRoundRestartSeconds => GameEvent::TeamPlayRoundRestartSeconds(
                TeamPlayRoundRestartSecondsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayTeamReady => {
                GameEvent::TeamPlayTeamReady(TeamPlayTeamReadyEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayRoundWin => {
                GameEvent::TeamPlayRoundWin(TeamPlayRoundWinEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayUpdateTimer => GameEvent::TeamPlayUpdateTimer(
                TeamPlayUpdateTimerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayRoundStalemate => GameEvent::TeamPlayRoundStalemate(
                TeamPlayRoundStalemateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayOvertimeBegin => GameEvent::TeamPlayOvertimeBegin(
                TeamPlayOvertimeBeginEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayOvertimeEnd => GameEvent::TeamPlayOvertimeEnd(
                TeamPlayOvertimeEndEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlaySuddenDeathBegin => GameEvent::TeamPlaySuddenDeathBegin(
                TeamPlaySuddenDeathBeginEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlaySuddenDeathEnd => GameEvent::TeamPlaySuddenDeathEnd(
                TeamPlaySuddenDeathEndEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayGameOver => {
                GameEvent::TeamPlayGameOver(TeamPlayGameOverEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayMapTimeRemaining => GameEvent::TeamPlayMapTimeRemaining(
                TeamPlayMapTimeRemainingEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayTimerFlash => GameEvent::TeamPlayTimerFlash(
                TeamPlayTimerFlashEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayTimerTimeAdded => GameEvent::TeamPlayTimerTimeAdded(
                TeamPlayTimerTimeAddedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayPointStartCapture => GameEvent::TeamPlayPointStartCapture(
                TeamPlayPointStartCaptureEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayPointCaptured => GameEvent::TeamPlayPointCaptured(
                TeamPlayPointCapturedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayPointLocked => GameEvent::TeamPlayPointLocked(
                TeamPlayPointLockedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayPointUnlocked => GameEvent::TeamPlayPointUnlocked(
                TeamPlayPointUnlockedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayCaptureBroken => GameEvent::TeamPlayCaptureBroken(
                TeamPlayCaptureBrokenEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayCaptureBlocked => GameEvent::TeamPlayCaptureBlocked(
                TeamPlayCaptureBlockedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayFlagEvent => {
                GameEvent::TeamPlayFlagEvent(TeamPlayFlagEventEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayWinPanel => {
                GameEvent::TeamPlayWinPanel(TeamPlayWinPanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayTeamBalancedPlayer => GameEvent::TeamPlayTeamBalancedPlayer(
                TeamPlayTeamBalancedPlayerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlaySetupFinished => GameEvent::TeamPlaySetupFinished(
                TeamPlaySetupFinishedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TeamPlayAlert => {
                GameEvent::TeamPlayAlert(TeamPlayAlertEvent::from_raw_event(event.values)?)
            }
            GameEventType::TrainingComplete => {
                GameEvent::TrainingComplete(TrainingCompleteEvent::from_raw_event(event.values)?)
            }
            GameEventType::ShowFreezePanel => {
                GameEvent::ShowFreezePanel(ShowFreezePanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::HideFreezePanel => {
                GameEvent::HideFreezePanel(HideFreezePanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::FreezeCamStarted => {
                GameEvent::FreezeCamStarted(FreezeCamStartedEvent::from_raw_event(event.values)?)
            }
            GameEventType::LocalPlayerChangeTeam => GameEvent::LocalPlayerChangeTeam(
                LocalPlayerChangeTeamEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerScoreChanged => GameEvent::LocalPlayerScoreChanged(
                LocalPlayerScoreChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerChangeClass => GameEvent::LocalPlayerChangeClass(
                LocalPlayerChangeClassEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerRespawn => GameEvent::LocalPlayerRespawn(
                LocalPlayerRespawnEvent::from_raw_event(event.values)?,
            ),
            GameEventType::BuildingInfoChanged => GameEvent::BuildingInfoChanged(
                BuildingInfoChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerChangeDisguise => GameEvent::LocalPlayerChangeDisguise(
                LocalPlayerChangeDisguiseEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerAccountChanged => GameEvent::PlayerAccountChanged(
                PlayerAccountChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::SpyPdaReset => {
                GameEvent::SpyPdaReset(SpyPdaResetEvent::from_raw_event(event.values)?)
            }
            GameEventType::FlagStatusUpdate => {
                GameEvent::FlagStatusUpdate(FlagStatusUpdateEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerStatsUpdated => GameEvent::PlayerStatsUpdated(
                PlayerStatsUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayingCommentary => {
                GameEvent::PlayingCommentary(PlayingCommentaryEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerChargeDeployed => GameEvent::PlayerChargeDeployed(
                PlayerChargeDeployedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerBuiltObject => {
                GameEvent::PlayerBuiltObject(PlayerBuiltObjectEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerUpgradedObject => GameEvent::PlayerUpgradedObject(
                PlayerUpgradedObjectEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerCarryObject => {
                GameEvent::PlayerCarryObject(PlayerCarryObjectEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDropObject => {
                GameEvent::PlayerDropObject(PlayerDropObjectEvent::from_raw_event(event.values)?)
            }
            GameEventType::ObjectRemoved => {
                GameEvent::ObjectRemoved(ObjectRemovedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ObjectDestroyed => {
                GameEvent::ObjectDestroyed(ObjectDestroyedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ObjectDetonated => {
                GameEvent::ObjectDetonated(ObjectDetonatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::AchievementEarned => {
                GameEvent::AchievementEarned(AchievementEarnedEvent::from_raw_event(event.values)?)
            }
            GameEventType::SpecTargetUpdated => {
                GameEvent::SpecTargetUpdated(SpecTargetUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::TournamentStateUpdate => GameEvent::TournamentStateUpdate(
                TournamentStateUpdateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TournamentEnableCountdown => GameEvent::TournamentEnableCountdown(
                TournamentEnableCountdownEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerCalledForMedic => GameEvent::PlayerCalledForMedic(
                PlayerCalledForMedicEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerAskedForBall => GameEvent::PlayerAskedForBall(
                PlayerAskedForBallEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerBecameObserver => GameEvent::LocalPlayerBecameObserver(
                LocalPlayerBecameObserverEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerIgnitedInv => {
                GameEvent::PlayerIgnitedInv(PlayerIgnitedInvEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerIgnited => {
                GameEvent::PlayerIgnited(PlayerIgnitedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerExtinguished => GameEvent::PlayerExtinguished(
                PlayerExtinguishedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerTeleported => {
                GameEvent::PlayerTeleported(PlayerTeleportedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerHealedMedicCall => GameEvent::PlayerHealedMedicCall(
                PlayerHealedMedicCallEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerChargeReady => GameEvent::LocalPlayerChargeReady(
                LocalPlayerChargeReadyEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LocalPlayerWindDown => GameEvent::LocalPlayerWindDown(
                LocalPlayerWindDownEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerInvulned => {
                GameEvent::PlayerInvulned(PlayerInvulnedEvent::from_raw_event(event.values)?)
            }
            GameEventType::EscortSpeed => {
                GameEvent::EscortSpeed(EscortSpeedEvent::from_raw_event(event.values)?)
            }
            GameEventType::EscortProgress => {
                GameEvent::EscortProgress(EscortProgressEvent::from_raw_event(event.values)?)
            }
            GameEventType::EscortRecede => {
                GameEvent::EscortRecede(EscortRecedeEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameUIActivated => {
                GameEvent::GameUIActivated(GameUIActivatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::GameUIHidden => {
                GameEvent::GameUIHidden(GameUIHiddenEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerEscortScore => {
                GameEvent::PlayerEscortScore(PlayerEscortScoreEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerHealOnHit => {
                GameEvent::PlayerHealOnHit(PlayerHealOnHitEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerStealSandvich => GameEvent::PlayerStealSandvich(
                PlayerStealSandvichEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ShowClassLayout => {
                GameEvent::ShowClassLayout(ShowClassLayoutEvent::from_raw_event(event.values)?)
            }
            GameEventType::ShowVsPanel => {
                GameEvent::ShowVsPanel(ShowVsPanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDamaged => {
                GameEvent::PlayerDamaged(PlayerDamagedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ArenaPlayerNotification => GameEvent::ArenaPlayerNotification(
                ArenaPlayerNotificationEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ArenaMatchMaxStreak => GameEvent::ArenaMatchMaxStreak(
                ArenaMatchMaxStreakEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ArenaRoundStart => {
                GameEvent::ArenaRoundStart(ArenaRoundStartEvent::from_raw_event(event.values)?)
            }
            GameEventType::ArenaWinPanel => {
                GameEvent::ArenaWinPanel(ArenaWinPanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::PveWinPanel => {
                GameEvent::PveWinPanel(PveWinPanelEvent::from_raw_event(event.values)?)
            }
            GameEventType::AirDash => {
                GameEvent::AirDash(AirDashEvent::from_raw_event(event.values)?)
            }
            GameEventType::Landed => GameEvent::Landed(LandedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDamageDodged => GameEvent::PlayerDamageDodged(
                PlayerDamageDodgedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerStunned => {
                GameEvent::PlayerStunned(PlayerStunnedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ScoutGrandSlam => {
                GameEvent::ScoutGrandSlam(ScoutGrandSlamEvent::from_raw_event(event.values)?)
            }
            GameEventType::ScoutSlamdollLanded => GameEvent::ScoutSlamdollLanded(
                ScoutSlamdollLandedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ArrowImpact => {
                GameEvent::ArrowImpact(ArrowImpactEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerJarated => {
                GameEvent::PlayerJarated(PlayerJaratedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerJaratedFade => {
                GameEvent::PlayerJaratedFade(PlayerJaratedFadeEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerShieldBlocked => GameEvent::PlayerShieldBlocked(
                PlayerShieldBlockedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerPinned => {
                GameEvent::PlayerPinned(PlayerPinnedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerHealedByMedic => GameEvent::PlayerHealedByMedic(
                PlayerHealedByMedicEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerSappedObject => GameEvent::PlayerSappedObject(
                PlayerSappedObjectEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ItemFound => {
                GameEvent::ItemFound(ItemFoundEvent::from_raw_event(event.values)?)
            }
            GameEventType::ShowAnnotation => {
                GameEvent::ShowAnnotation(ShowAnnotationEvent::from_raw_event(event.values)?)
            }
            GameEventType::HideAnnotation => {
                GameEvent::HideAnnotation(HideAnnotationEvent::from_raw_event(event.values)?)
            }
            GameEventType::PostInventoryApplication => GameEvent::PostInventoryApplication(
                PostInventoryApplicationEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ControlPointUnlockUpdated => GameEvent::ControlPointUnlockUpdated(
                ControlPointUnlockUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::DeployBuffBanner => {
                GameEvent::DeployBuffBanner(DeployBuffBannerEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerBuff => {
                GameEvent::PlayerBuff(PlayerBuffEvent::from_raw_event(event.values)?)
            }
            GameEventType::MedicDeath => {
                GameEvent::MedicDeath(MedicDeathEvent::from_raw_event(event.values)?)
            }
            GameEventType::OvertimeNag => {
                GameEvent::OvertimeNag(OvertimeNagEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamsChanged => {
                GameEvent::TeamsChanged(TeamsChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::HalloweenPumpkinGrab => GameEvent::HalloweenPumpkinGrab(
                HalloweenPumpkinGrabEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RocketJump => {
                GameEvent::RocketJump(RocketJumpEvent::from_raw_event(event.values)?)
            }
            GameEventType::RocketJumpLanded => {
                GameEvent::RocketJumpLanded(RocketJumpLandedEvent::from_raw_event(event.values)?)
            }
            GameEventType::StickyJump => {
                GameEvent::StickyJump(StickyJumpEvent::from_raw_event(event.values)?)
            }
            GameEventType::StickyJumpLanded => {
                GameEvent::StickyJumpLanded(StickyJumpLandedEvent::from_raw_event(event.values)?)
            }
            GameEventType::RocketPackLaunch => {
                GameEvent::RocketPackLaunch(RocketPackLaunchEvent::from_raw_event(event.values)?)
            }
            GameEventType::RocketPackLanded => {
                GameEvent::RocketPackLanded(RocketPackLandedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MedicDefended => {
                GameEvent::MedicDefended(MedicDefendedEvent::from_raw_event(event.values)?)
            }
            GameEventType::LocalPlayerHealed => {
                GameEvent::LocalPlayerHealed(LocalPlayerHealedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDestroyedPipeBomb => GameEvent::PlayerDestroyedPipeBomb(
                PlayerDestroyedPipeBombEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ObjectDeflected => {
                GameEvent::ObjectDeflected(ObjectDeflectedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerMvp => {
                GameEvent::PlayerMvp(PlayerMvpEvent::from_raw_event(event.values)?)
            }
            GameEventType::RaidSpawnMob => {
                GameEvent::RaidSpawnMob(RaidSpawnMobEvent::from_raw_event(event.values)?)
            }
            GameEventType::RaidSpawnSquad => {
                GameEvent::RaidSpawnSquad(RaidSpawnSquadEvent::from_raw_event(event.values)?)
            }
            GameEventType::NavBlocked => {
                GameEvent::NavBlocked(NavBlockedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PathTrackPassed => {
                GameEvent::PathTrackPassed(PathTrackPassedEvent::from_raw_event(event.values)?)
            }
            GameEventType::NumCappersChanged => {
                GameEvent::NumCappersChanged(NumCappersChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerRegenerate => {
                GameEvent::PlayerRegenerate(PlayerRegenerateEvent::from_raw_event(event.values)?)
            }
            GameEventType::UpdateStatusItem => {
                GameEvent::UpdateStatusItem(UpdateStatusItemEvent::from_raw_event(event.values)?)
            }
            GameEventType::StatsResetRound => {
                GameEvent::StatsResetRound(StatsResetRoundEvent::from_raw_event(event.values)?)
            }
            GameEventType::ScoreStatsAccumulatedUpdate => GameEvent::ScoreStatsAccumulatedUpdate(
                ScoreStatsAccumulatedUpdateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ScoreStatsAccumulatedReset => GameEvent::ScoreStatsAccumulatedReset(
                ScoreStatsAccumulatedResetEvent::from_raw_event(event.values)?,
            ),
            GameEventType::AchievementEarnedLocal => GameEvent::AchievementEarnedLocal(
                AchievementEarnedLocalEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerHealed => {
                GameEvent::PlayerHealed(PlayerHealedEvent::from_raw_event(event.values)?)
            }
            GameEventType::BuildingHealed => {
                GameEvent::BuildingHealed(BuildingHealedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ItemPickup => {
                GameEvent::ItemPickup(ItemPickupEvent::from_raw_event(event.values)?)
            }
            GameEventType::DuelStatus => {
                GameEvent::DuelStatus(DuelStatusEvent::from_raw_event(event.values)?)
            }
            GameEventType::FishNotice => {
                GameEvent::FishNotice(FishNoticeEvent::from_raw_event(event.values)?)
            }
            GameEventType::FishNoticeArm => {
                GameEvent::FishNoticeArm(FishNoticeArmEvent::from_raw_event(event.values)?)
            }
            GameEventType::SlapNotice => {
                GameEvent::SlapNotice(SlapNoticeEvent::from_raw_event(event.values)?)
            }
            GameEventType::ThrowableHit => {
                GameEvent::ThrowableHit(ThrowableHitEvent::from_raw_event(event.values)?)
            }
            GameEventType::PumpkinLordSummoned => GameEvent::PumpkinLordSummoned(
                PumpkinLordSummonedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PumpkinLordKilled => {
                GameEvent::PumpkinLordKilled(PumpkinLordKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::MerasmusSummoned => {
                GameEvent::MerasmusSummoned(MerasmusSummonedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MerasmusKilled => {
                GameEvent::MerasmusKilled(MerasmusKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::MerasmusEscapeWarning => GameEvent::MerasmusEscapeWarning(
                MerasmusEscapeWarningEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MerasmusEscaped => {
                GameEvent::MerasmusEscaped(MerasmusEscapedEvent::from_raw_event(event.values)?)
            }
            GameEventType::EyeballBossSummoned => GameEvent::EyeballBossSummoned(
                EyeballBossSummonedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EyeballBossStunned => GameEvent::EyeballBossStunned(
                EyeballBossStunnedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EyeballBossKilled => {
                GameEvent::EyeballBossKilled(EyeballBossKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::EyeballBossKiller => {
                GameEvent::EyeballBossKiller(EyeballBossKillerEvent::from_raw_event(event.values)?)
            }
            GameEventType::EyeballBossEscapeImminent => GameEvent::EyeballBossEscapeImminent(
                EyeballBossEscapeImminentEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EyeballBossEscaped => GameEvent::EyeballBossEscaped(
                EyeballBossEscapedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::NpcHurt => {
                GameEvent::NpcHurt(NpcHurtEvent::from_raw_event(event.values)?)
            }
            GameEventType::ControlPointTimerUpdated => GameEvent::ControlPointTimerUpdated(
                ControlPointTimerUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerHighFiveStart => GameEvent::PlayerHighFiveStart(
                PlayerHighFiveStartEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerHighFiveCancel => GameEvent::PlayerHighFiveCancel(
                PlayerHighFiveCancelEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerHighFiveSuccess => GameEvent::PlayerHighFiveSuccess(
                PlayerHighFiveSuccessEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerBonusPoints => {
                GameEvent::PlayerBonusPoints(PlayerBonusPointsEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerUpgraded => {
                GameEvent::PlayerUpgraded(PlayerUpgradedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerBuyback => {
                GameEvent::PlayerBuyback(PlayerBuybackEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerUsedPowerUpBottle => GameEvent::PlayerUsedPowerUpBottle(
                PlayerUsedPowerUpBottleEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ChristmasGiftGrab => {
                GameEvent::ChristmasGiftGrab(ChristmasGiftGrabEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerKilledAchievementZone => GameEvent::PlayerKilledAchievementZone(
                PlayerKilledAchievementZoneEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PartyUpdated => {
                GameEvent::PartyUpdated(PartyUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PartyPrefChanged => {
                GameEvent::PartyPrefChanged(PartyPrefChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PartyCriteriaChanged => GameEvent::PartyCriteriaChanged(
                PartyCriteriaChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PartyInvitesChanged => GameEvent::PartyInvitesChanged(
                PartyInvitesChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PartyQueueStateChanged => GameEvent::PartyQueueStateChanged(
                PartyQueueStateChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PartyChat => {
                GameEvent::PartyChat(PartyChatEvent::from_raw_event(event.values)?)
            }
            GameEventType::PartyMemberJoin => {
                GameEvent::PartyMemberJoin(PartyMemberJoinEvent::from_raw_event(event.values)?)
            }
            GameEventType::PartyMemberLeave => {
                GameEvent::PartyMemberLeave(PartyMemberLeaveEvent::from_raw_event(event.values)?)
            }
            GameEventType::MatchInvitesUpdated => GameEvent::MatchInvitesUpdated(
                MatchInvitesUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::LobbyUpdated => {
                GameEvent::LobbyUpdated(LobbyUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmMissionUpdate => {
                GameEvent::MvmMissionUpdate(MvmMissionUpdateEvent::from_raw_event(event.values)?)
            }
            GameEventType::RecalculateHolidays => GameEvent::RecalculateHolidays(
                RecalculateHolidaysEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerCurrencyChanged => GameEvent::PlayerCurrencyChanged(
                PlayerCurrencyChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::DoomsdayRocketOpen => GameEvent::DoomsdayRocketOpen(
                DoomsdayRocketOpenEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RemoveNemesisRelationships => GameEvent::RemoveNemesisRelationships(
                RemoveNemesisRelationshipsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmCreditBonusWave => GameEvent::MvmCreditBonusWave(
                MvmCreditBonusWaveEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmCreditBonusAll => {
                GameEvent::MvmCreditBonusAll(MvmCreditBonusAllEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmCreditBonusAllAdvanced => GameEvent::MvmCreditBonusAllAdvanced(
                MvmCreditBonusAllAdvancedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmQuickSentryUpgrade => GameEvent::MvmQuickSentryUpgrade(
                MvmQuickSentryUpgradeEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmTankDestroyedByPlayers => GameEvent::MvmTankDestroyedByPlayers(
                MvmTankDestroyedByPlayersEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmKillRobotDeliveringBomb => GameEvent::MvmKillRobotDeliveringBomb(
                MvmKillRobotDeliveringBombEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmPickupCurrency => {
                GameEvent::MvmPickupCurrency(MvmPickupCurrencyEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmBombCarrierKilled => GameEvent::MvmBombCarrierKilled(
                MvmBombCarrierKilledEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmSentryBusterDetonate => GameEvent::MvmSentryBusterDetonate(
                MvmSentryBusterDetonateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmScoutMarkedForDeath => GameEvent::MvmScoutMarkedForDeath(
                MvmScoutMarkedForDeathEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmMedicPowerUpShared => GameEvent::MvmMedicPowerUpShared(
                MvmMedicPowerUpSharedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmBeginWave => {
                GameEvent::MvmBeginWave(MvmBeginWaveEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmWaveComplete => {
                GameEvent::MvmWaveComplete(MvmWaveCompleteEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmMissionComplete => GameEvent::MvmMissionComplete(
                MvmMissionCompleteEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmBombResetByPlayer => GameEvent::MvmBombResetByPlayer(
                MvmBombResetByPlayerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmBombAlarmTriggered => GameEvent::MvmBombAlarmTriggered(
                MvmBombAlarmTriggeredEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmBombDeployResetByPlayer => GameEvent::MvmBombDeployResetByPlayer(
                MvmBombDeployResetByPlayerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmWaveFailed => {
                GameEvent::MvmWaveFailed(MvmWaveFailedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MvmResetStats => {
                GameEvent::MvmResetStats(MvmResetStatsEvent::from_raw_event(event.values)?)
            }
            GameEventType::DamageResisted => {
                GameEvent::DamageResisted(DamageResistedEvent::from_raw_event(event.values)?)
            }
            GameEventType::RevivePlayerNotify => GameEvent::RevivePlayerNotify(
                RevivePlayerNotifyEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RevivePlayerStopped => GameEvent::RevivePlayerStopped(
                RevivePlayerStoppedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RevivePlayerComplete => GameEvent::RevivePlayerComplete(
                RevivePlayerCompleteEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerTurnedToGhost => GameEvent::PlayerTurnedToGhost(
                PlayerTurnedToGhostEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MedigunShieldBlockedDamage => GameEvent::MedigunShieldBlockedDamage(
                MedigunShieldBlockedDamageEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmAdvWaveCompleteNoGates => GameEvent::MvmAdvWaveCompleteNoGates(
                MvmAdvWaveCompleteNoGatesEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmSniperHeadshotCurrency => GameEvent::MvmSniperHeadshotCurrency(
                MvmSniperHeadshotCurrencyEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmMannhattanPit => {
                GameEvent::MvmMannhattanPit(MvmMannhattanPitEvent::from_raw_event(event.values)?)
            }
            GameEventType::FlagCarriedInDetectionZone => GameEvent::FlagCarriedInDetectionZone(
                FlagCarriedInDetectionZoneEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmAdvWaveKilledStunRadio => GameEvent::MvmAdvWaveKilledStunRadio(
                MvmAdvWaveKilledStunRadioEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerDirectHitStun => GameEvent::PlayerDirectHitStun(
                PlayerDirectHitStunEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MvmSentryBusterKilled => GameEvent::MvmSentryBusterKilled(
                MvmSentryBusterKilledEvent::from_raw_event(event.values)?,
            ),
            GameEventType::UpgradesFileChanged => GameEvent::UpgradesFileChanged(
                UpgradesFileChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RdTeamPointsChanged => GameEvent::RdTeamPointsChanged(
                RdTeamPointsChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RdRulesStateChanged => GameEvent::RdRulesStateChanged(
                RdRulesStateChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RdRobotKilled => {
                GameEvent::RdRobotKilled(RdRobotKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::RdRobotImpact => {
                GameEvent::RdRobotImpact(RdRobotImpactEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamPlayPreRoundTimeLeft => GameEvent::TeamPlayPreRoundTimeLeft(
                TeamPlayPreRoundTimeLeftEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ParachuteDeploy => {
                GameEvent::ParachuteDeploy(ParachuteDeployEvent::from_raw_event(event.values)?)
            }
            GameEventType::ParachuteHolster => {
                GameEvent::ParachuteHolster(ParachuteHolsterEvent::from_raw_event(event.values)?)
            }
            GameEventType::KillRefillsMeter => {
                GameEvent::KillRefillsMeter(KillRefillsMeterEvent::from_raw_event(event.values)?)
            }
            GameEventType::RpsTauntEvent => {
                GameEvent::RpsTauntEvent(RpsTauntEventEvent::from_raw_event(event.values)?)
            }
            GameEventType::CongaKill => {
                GameEvent::CongaKill(CongaKillEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerInitialSpawn => GameEvent::PlayerInitialSpawn(
                PlayerInitialSpawnEvent::from_raw_event(event.values)?,
            ),
            GameEventType::CompetitiveVictory => GameEvent::CompetitiveVictory(
                CompetitiveVictoryEvent::from_raw_event(event.values)?,
            ),
            GameEventType::CompetitiveStatsUpdate => GameEvent::CompetitiveStatsUpdate(
                CompetitiveStatsUpdateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MiniGameWin => {
                GameEvent::MiniGameWin(MiniGameWinEvent::from_raw_event(event.values)?)
            }
            GameEventType::SentryOnGoActive => {
                GameEvent::SentryOnGoActive(SentryOnGoActiveEvent::from_raw_event(event.values)?)
            }
            GameEventType::DuckXpLevelUp => {
                GameEvent::DuckXpLevelUp(DuckXpLevelUpEvent::from_raw_event(event.values)?)
            }
            GameEventType::QuestLogOpened => {
                GameEvent::QuestLogOpened(QuestLogOpenedEvent::from_raw_event(event.values)?)
            }
            GameEventType::SchemaUpdated => {
                GameEvent::SchemaUpdated(SchemaUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::LocalPlayerPickupWeapon => GameEvent::LocalPlayerPickupWeapon(
                LocalPlayerPickupWeaponEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RdPlayerScorePoints => GameEvent::RdPlayerScorePoints(
                RdPlayerScorePointsEvent::from_raw_event(event.values)?,
            ),
            GameEventType::DemomanDetStickies => GameEvent::DemomanDetStickies(
                DemomanDetStickiesEvent::from_raw_event(event.values)?,
            ),
            GameEventType::QuestObjectiveCompleted => GameEvent::QuestObjectiveCompleted(
                QuestObjectiveCompletedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerScoreChanged => GameEvent::PlayerScoreChanged(
                PlayerScoreChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::KilledCappingPlayer => GameEvent::KilledCappingPlayer(
                KilledCappingPlayerEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EnvironmentalDeath => GameEvent::EnvironmentalDeath(
                EnvironmentalDeathEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ProjectileDirectHit => GameEvent::ProjectileDirectHit(
                ProjectileDirectHitEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PassGet => {
                GameEvent::PassGet(PassGetEvent::from_raw_event(event.values)?)
            }
            GameEventType::PassScore => {
                GameEvent::PassScore(PassScoreEvent::from_raw_event(event.values)?)
            }
            GameEventType::PassFree => {
                GameEvent::PassFree(PassFreeEvent::from_raw_event(event.values)?)
            }
            GameEventType::PassPassCaught => {
                GameEvent::PassPassCaught(PassPassCaughtEvent::from_raw_event(event.values)?)
            }
            GameEventType::PassBallStolen => {
                GameEvent::PassBallStolen(PassBallStolenEvent::from_raw_event(event.values)?)
            }
            GameEventType::PassBallBlocked => {
                GameEvent::PassBallBlocked(PassBallBlockedEvent::from_raw_event(event.values)?)
            }
            GameEventType::DamagePrevented => {
                GameEvent::DamagePrevented(DamagePreventedEvent::from_raw_event(event.values)?)
            }
            GameEventType::HalloweenBossKilled => GameEvent::HalloweenBossKilled(
                HalloweenBossKilledEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EscapedLootIsland => {
                GameEvent::EscapedLootIsland(EscapedLootIslandEvent::from_raw_event(event.values)?)
            }
            GameEventType::TaggedPlayerAsIt => {
                GameEvent::TaggedPlayerAsIt(TaggedPlayerAsItEvent::from_raw_event(event.values)?)
            }
            GameEventType::MerasmusStunned => {
                GameEvent::MerasmusStunned(MerasmusStunnedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MerasmusPropFound => {
                GameEvent::MerasmusPropFound(MerasmusPropFoundEvent::from_raw_event(event.values)?)
            }
            GameEventType::HalloweenSkeletonKilled => GameEvent::HalloweenSkeletonKilled(
                HalloweenSkeletonKilledEvent::from_raw_event(event.values)?,
            ),
            GameEventType::EscapeHell => {
                GameEvent::EscapeHell(EscapeHellEvent::from_raw_event(event.values)?)
            }
            GameEventType::CrossSpectralBridge => GameEvent::CrossSpectralBridge(
                CrossSpectralBridgeEvent::from_raw_event(event.values)?,
            ),
            GameEventType::MiniGameWon => {
                GameEvent::MiniGameWon(MiniGameWonEvent::from_raw_event(event.values)?)
            }
            GameEventType::RespawnGhost => {
                GameEvent::RespawnGhost(RespawnGhostEvent::from_raw_event(event.values)?)
            }
            GameEventType::KillInHell => {
                GameEvent::KillInHell(KillInHellEvent::from_raw_event(event.values)?)
            }
            GameEventType::HalloweenDuckCollected => GameEvent::HalloweenDuckCollected(
                HalloweenDuckCollectedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::SpecialScore => {
                GameEvent::SpecialScore(SpecialScoreEvent::from_raw_event(event.values)?)
            }
            GameEventType::TeamLeaderKilled => {
                GameEvent::TeamLeaderKilled(TeamLeaderKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::HalloweenSoulCollected => GameEvent::HalloweenSoulCollected(
                HalloweenSoulCollectedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RecalculateTruce => {
                GameEvent::RecalculateTruce(RecalculateTruceEvent::from_raw_event(event.values)?)
            }
            GameEventType::DeadRingerCheatDeath => GameEvent::DeadRingerCheatDeath(
                DeadRingerCheatDeathEvent::from_raw_event(event.values)?,
            ),
            GameEventType::CrossbowHeal => {
                GameEvent::CrossbowHeal(CrossbowHealEvent::from_raw_event(event.values)?)
            }
            GameEventType::DamageMitigated => {
                GameEvent::DamageMitigated(DamageMitigatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PayloadPushed => {
                GameEvent::PayloadPushed(PayloadPushedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerAbandonedMatch => GameEvent::PlayerAbandonedMatch(
                PlayerAbandonedMatchEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ClDrawline => {
                GameEvent::ClDrawline(ClDrawlineEvent::from_raw_event(event.values)?)
            }
            GameEventType::RestartTimerTime => {
                GameEvent::RestartTimerTime(RestartTimerTimeEvent::from_raw_event(event.values)?)
            }
            GameEventType::WinLimitChanged => {
                GameEvent::WinLimitChanged(WinLimitChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::WinPanelShowScores => GameEvent::WinPanelShowScores(
                WinPanelShowScoresEvent::from_raw_event(event.values)?,
            ),
            GameEventType::TopStreamsRequestFinished => GameEvent::TopStreamsRequestFinished(
                TopStreamsRequestFinishedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::CompetitiveStateChanged => GameEvent::CompetitiveStateChanged(
                CompetitiveStateChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::GlobalWarDataUpdated => GameEvent::GlobalWarDataUpdated(
                GlobalWarDataUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::StopWatchChanged => {
                GameEvent::StopWatchChanged(StopWatchChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::DsStop => GameEvent::DsStop(DsStopEvent::from_raw_event(event.values)?),
            GameEventType::DsScreenshot => {
                GameEvent::DsScreenshot(DsScreenshotEvent::from_raw_event(event.values)?)
            }
            GameEventType::ShowMatchSummary => {
                GameEvent::ShowMatchSummary(ShowMatchSummaryEvent::from_raw_event(event.values)?)
            }
            GameEventType::ExperienceChanged => {
                GameEvent::ExperienceChanged(ExperienceChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::BeginXpLerp => {
                GameEvent::BeginXpLerp(BeginXpLerpEvent::from_raw_event(event.values)?)
            }
            GameEventType::MatchmakerStatsUpdated => GameEvent::MatchmakerStatsUpdated(
                MatchmakerStatsUpdatedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RematchVotePeriodOver => GameEvent::RematchVotePeriodOver(
                RematchVotePeriodOverEvent::from_raw_event(event.values)?,
            ),
            GameEventType::RematchFailedToCreate => GameEvent::RematchFailedToCreate(
                RematchFailedToCreateEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PlayerRematchChange => GameEvent::PlayerRematchChange(
                PlayerRematchChangeEvent::from_raw_event(event.values)?,
            ),
            GameEventType::PingUpdated => {
                GameEvent::PingUpdated(PingUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::MMStatsUpdated => {
                GameEvent::MMStatsUpdated(MMStatsUpdatedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerNextMapVoteChange => GameEvent::PlayerNextMapVoteChange(
                PlayerNextMapVoteChangeEvent::from_raw_event(event.values)?,
            ),
            GameEventType::VoteMapsChanged => {
                GameEvent::VoteMapsChanged(VoteMapsChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::ProtoDefChanged => {
                GameEvent::ProtoDefChanged(ProtoDefChangedEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerDomination => {
                GameEvent::PlayerDomination(PlayerDominationEvent::from_raw_event(event.values)?)
            }
            GameEventType::PlayerRocketPackPushed => GameEvent::PlayerRocketPackPushed(
                PlayerRocketPackPushedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::QuestRequest => {
                GameEvent::QuestRequest(QuestRequestEvent::from_raw_event(event.values)?)
            }
            GameEventType::QuestResponse => {
                GameEvent::QuestResponse(QuestResponseEvent::from_raw_event(event.values)?)
            }
            GameEventType::QuestProgress => {
                GameEvent::QuestProgress(QuestProgressEvent::from_raw_event(event.values)?)
            }
            GameEventType::ProjectileRemoved => {
                GameEvent::ProjectileRemoved(ProjectileRemovedEvent::from_raw_event(event.values)?)
            }
            GameEventType::QuestMapDataChanged => GameEvent::QuestMapDataChanged(
                QuestMapDataChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::GasDousedPlayerIgnited => GameEvent::GasDousedPlayerIgnited(
                GasDousedPlayerIgnitedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::QuestTurnInState => {
                GameEvent::QuestTurnInState(QuestTurnInStateEvent::from_raw_event(event.values)?)
            }
            GameEventType::ItemsAcknowledged => {
                GameEvent::ItemsAcknowledged(ItemsAcknowledgedEvent::from_raw_event(event.values)?)
            }
            GameEventType::CapperKilled => {
                GameEvent::CapperKilled(CapperKilledEvent::from_raw_event(event.values)?)
            }
            GameEventType::MainMenuStabilized => GameEvent::MainMenuStabilized(
                MainMenuStabilizedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::WorldStatusChanged => GameEvent::WorldStatusChanged(
                WorldStatusChangedEvent::from_raw_event(event.values)?,
            ),
            GameEventType::HLTVStatus => {
                GameEvent::HLTVStatus(HLTVStatusEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVCameraman => {
                GameEvent::HLTVCameraman(HLTVCameramanEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVRankCamera => {
                GameEvent::HLTVRankCamera(HLTVRankCameraEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVRankEntity => {
                GameEvent::HLTVRankEntity(HLTVRankEntityEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVFixed => {
                GameEvent::HLTVFixed(HLTVFixedEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVChase => {
                GameEvent::HLTVChase(HLTVChaseEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVMessage => {
                GameEvent::HLTVMessage(HLTVMessageEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVTitle => {
                GameEvent::HLTVTitle(HLTVTitleEvent::from_raw_event(event.values)?)
            }
            GameEventType::HLTVChat => {
                GameEvent::HLTVChat(HLTVChatEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplayStartRecord => {
                GameEvent::ReplayStartRecord(ReplayStartRecordEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplaySessionInfo => {
                GameEvent::ReplaySessionInfo(ReplaySessionInfoEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplayEndRecord => {
                GameEvent::ReplayEndRecord(ReplayEndRecordEvent::from_raw_event(event.values)?)
            }
            GameEventType::ReplayReplaysAvailable => GameEvent::ReplayReplaysAvailable(
                ReplayReplaysAvailableEvent::from_raw_event(event.values)?,
            ),
            GameEventType::ReplayServerError => {
                GameEvent::ReplayServerError(ReplayServerErrorEvent::from_raw_event(event.values)?)
            }
            GameEventType::Unknown => GameEvent::Unknown(event),
        })
    }
}

