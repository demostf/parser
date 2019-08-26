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
        let hostname = match iter.next() {
            Some(value) => String::from_value(value, "hostname")?,
            None => String::default(),
        };
        let address = match iter.next() {
            Some(value) => String::from_value(value, "address")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => u32::from_value(value, "ip")?,
            None => u32::default(),
        };
        let port = match iter.next() {
            Some(value) => u16::from_value(value, "port")?,
            None => u16::default(),
        };
        let game = match iter.next() {
            Some(value) => String::from_value(value, "game")?,
            None => String::default(),
        };
        let map_name = match iter.next() {
            Some(value) => String::from_value(value, "map_name")?,
            None => String::default(),
        };
        let max_players = match iter.next() {
            Some(value) => u32::from_value(value, "max_players")?,
            None => u32::default(),
        };
        let os = match iter.next() {
            Some(value) => String::from_value(value, "os")?,
            None => String::default(),
        };
        let dedicated = match iter.next() {
            Some(value) => bool::from_value(value, "dedicated")?,
            None => bool::default(),
        };
        let password = match iter.next() {
            Some(value) => bool::from_value(value, "password")?,
            None => bool::default(),
        };
        Ok(ServerSpawnEvent {
            hostname,
            address,
            ip,
            port,
            game,
            map_name,
            max_players,
            os,
            dedicated,
            password,
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
        let level_name = match iter.next() {
            Some(value) => String::from_value(value, "level_name")?,
            None => String::default(),
        };
        Ok(ServerChangeLevelFailedEvent { level_name })
    }
}
#[derive(Debug)]
pub struct ServerShutdownEvent {
    pub reason: String,
}
impl FromRawGameEvent for ServerShutdownEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let reason = match iter.next() {
            Some(value) => String::from_value(value, "reason")?,
            None => String::default(),
        };
        Ok(ServerShutdownEvent { reason })
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
        let cvar_name = match iter.next() {
            Some(value) => String::from_value(value, "cvar_name")?,
            None => String::default(),
        };
        let cvar_value = match iter.next() {
            Some(value) => String::from_value(value, "cvar_value")?,
            None => String::default(),
        };
        Ok(ServerCvarEvent {
            cvar_name,
            cvar_value,
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
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(ServerMessageEvent { text })
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
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => String::from_value(value, "ip")?,
            None => String::default(),
        };
        let duration = match iter.next() {
            Some(value) => String::from_value(value, "duration")?,
            None => String::default(),
        };
        let by = match iter.next() {
            Some(value) => String::from_value(value, "by")?,
            None => String::default(),
        };
        let kicked = match iter.next() {
            Some(value) => bool::from_value(value, "kicked")?,
            None => bool::default(),
        };
        Ok(ServerAddBanEvent {
            name,
            user_id,
            network_id,
            ip,
            duration,
            by,
            kicked,
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
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => String::from_value(value, "ip")?,
            None => String::default(),
        };
        let by = match iter.next() {
            Some(value) => String::from_value(value, "by")?,
            None => String::default(),
        };
        Ok(ServerRemoveBanEvent { network_id, ip, by })
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
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        let index = match iter.next() {
            Some(value) => u8::from_value(value, "index")?,
            None => u8::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let address = match iter.next() {
            Some(value) => String::from_value(value, "address")?,
            None => String::default(),
        };
        let bot = match iter.next() {
            Some(value) => u16::from_value(value, "bot")?,
            None => u16::default(),
        };
        Ok(PlayerConnectEvent {
            name,
            index,
            user_id,
            network_id,
            address,
            bot,
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
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        let index = match iter.next() {
            Some(value) => u8::from_value(value, "index")?,
            None => u8::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let bot = match iter.next() {
            Some(value) => u16::from_value(value, "bot")?,
            None => u16::default(),
        };
        Ok(PlayerConnectClientEvent {
            name,
            index,
            user_id,
            network_id,
            bot,
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
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        let index = match iter.next() {
            Some(value) => u8::from_value(value, "index")?,
            None => u8::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let bot = match iter.next() {
            Some(value) => bool::from_value(value, "bot")?,
            None => bool::default(),
        };
        Ok(PlayerInfoEvent {
            name,
            index,
            user_id,
            network_id,
            bot,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let reason = match iter.next() {
            Some(value) => String::from_value(value, "reason")?,
            None => String::default(),
        };
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        let network_id = match iter.next() {
            Some(value) => String::from_value(value, "network_id")?,
            None => String::default(),
        };
        let bot = match iter.next() {
            Some(value) => u16::from_value(value, "bot")?,
            None => u16::default(),
        };
        Ok(PlayerDisconnectEvent {
            user_id,
            reason,
            name,
            network_id,
            bot,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerActivateEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(PlayerSayEvent { user_id, text })
    }
}
#[derive(Debug)]
pub struct ClientDisconnectEvent {
    pub message: String,
}
impl FromRawGameEvent for ClientDisconnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let message = match iter.next() {
            Some(value) => String::from_value(value, "message")?,
            None => String::default(),
        };
        Ok(ClientDisconnectEvent { message })
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
        let address = match iter.next() {
            Some(value) => String::from_value(value, "address")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => u32::from_value(value, "ip")?,
            None => u32::default(),
        };
        let port = match iter.next() {
            Some(value) => u16::from_value(value, "port")?,
            None => u16::default(),
        };
        let source = match iter.next() {
            Some(value) => String::from_value(value, "source")?,
            None => String::default(),
        };
        Ok(ClientBeginConnectEvent {
            address,
            ip,
            port,
            source,
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
        let address = match iter.next() {
            Some(value) => String::from_value(value, "address")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => u32::from_value(value, "ip")?,
            None => u32::default(),
        };
        let port = match iter.next() {
            Some(value) => u16::from_value(value, "port")?,
            None => u16::default(),
        };
        Ok(ClientConnectedEvent { address, ip, port })
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
        let address = match iter.next() {
            Some(value) => String::from_value(value, "address")?,
            None => String::default(),
        };
        let ip = match iter.next() {
            Some(value) => u32::from_value(value, "ip")?,
            None => u32::default(),
        };
        let port = match iter.next() {
            Some(value) => u16::from_value(value, "port")?,
            None => u16::default(),
        };
        Ok(ClientFullConnectEvent { address, ip, port })
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
        let team_id = match iter.next() {
            Some(value) => u8::from_value(value, "team_id")?,
            None => u8::default(),
        };
        let team_name = match iter.next() {
            Some(value) => String::from_value(value, "team_name")?,
            None => String::default(),
        };
        Ok(TeamInfoEvent { team_id, team_name })
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
        let team_id = match iter.next() {
            Some(value) => u8::from_value(value, "team_id")?,
            None => u8::default(),
        };
        let score = match iter.next() {
            Some(value) => u16::from_value(value, "score")?,
            None => u16::default(),
        };
        Ok(TeamScoreEvent { team_id, score })
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let sound = match iter.next() {
            Some(value) => String::from_value(value, "sound")?,
            None => String::default(),
        };
        let additional_flags = match iter.next() {
            Some(value) => u16::from_value(value, "additional_flags")?,
            None => u16::default(),
        };
        Ok(TeamPlayBroadcastAudioEvent {
            team,
            sound,
            additional_flags,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let old_team = match iter.next() {
            Some(value) => u8::from_value(value, "old_team")?,
            None => u8::default(),
        };
        let disconnect = match iter.next() {
            Some(value) => bool::from_value(value, "disconnect")?,
            None => bool::default(),
        };
        let auto_team = match iter.next() {
            Some(value) => bool::from_value(value, "auto_team")?,
            None => bool::default(),
        };
        let silent = match iter.next() {
            Some(value) => bool::from_value(value, "silent")?,
            None => bool::default(),
        };
        let name = match iter.next() {
            Some(value) => String::from_value(value, "name")?,
            None => String::default(),
        };
        Ok(PlayerTeamEvent {
            user_id,
            team,
            old_team,
            disconnect,
            auto_team,
            silent,
            name,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let class = match iter.next() {
            Some(value) => String::from_value(value, "class")?,
            None => String::default(),
        };
        Ok(PlayerClassEvent { user_id, class })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        let stun_flags = match iter.next() {
            Some(value) => u16::from_value(value, "stun_flags")?,
            None => u16::default(),
        };
        let death_flags = match iter.next() {
            Some(value) => u16::from_value(value, "death_flags")?,
            None => u16::default(),
        };
        let silent_kill = match iter.next() {
            Some(value) => bool::from_value(value, "silent_kill")?,
            None => bool::default(),
        };
        let player_penetrate_count = match iter.next() {
            Some(value) => u16::from_value(value, "player_penetrate_count")?,
            None => u16::default(),
        };
        let assister_fallback = match iter.next() {
            Some(value) => String::from_value(value, "assister_fallback")?,
            None => String::default(),
        };
        let kill_streak_total = match iter.next() {
            Some(value) => u16::from_value(value, "kill_streak_total")?,
            None => u16::default(),
        };
        let kill_streak_wep = match iter.next() {
            Some(value) => u16::from_value(value, "kill_streak_wep")?,
            None => u16::default(),
        };
        let kill_streak_assist = match iter.next() {
            Some(value) => u16::from_value(value, "kill_streak_assist")?,
            None => u16::default(),
        };
        let kill_streak_victim = match iter.next() {
            Some(value) => u16::from_value(value, "kill_streak_victim")?,
            None => u16::default(),
        };
        let ducks_streaked = match iter.next() {
            Some(value) => u16::from_value(value, "ducks_streaked")?,
            None => u16::default(),
        };
        let duck_streak_total = match iter.next() {
            Some(value) => u16::from_value(value, "duck_streak_total")?,
            None => u16::default(),
        };
        let duck_streak_assist = match iter.next() {
            Some(value) => u16::from_value(value, "duck_streak_assist")?,
            None => u16::default(),
        };
        let duck_streak_victim = match iter.next() {
            Some(value) => u16::from_value(value, "duck_streak_victim")?,
            None => u16::default(),
        };
        let rocket_jump = match iter.next() {
            Some(value) => bool::from_value(value, "rocket_jump")?,
            None => bool::default(),
        };
        let weapon_def_index = match iter.next() {
            Some(value) => u32::from_value(value, "weapon_def_index")?,
            None => u32::default(),
        };
        let crit_type = match iter.next() {
            Some(value) => u16::from_value(value, "crit_type")?,
            None => u16::default(),
        };
        Ok(PlayerDeathEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            assister,
            weapon_log_class_name,
            stun_flags,
            death_flags,
            silent_kill,
            player_penetrate_count,
            assister_fallback,
            kill_streak_total,
            kill_streak_wep,
            kill_streak_assist,
            kill_streak_victim,
            ducks_streaked,
            duck_streak_total,
            duck_streak_assist,
            duck_streak_victim,
            rocket_jump,
            weapon_def_index,
            crit_type,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let health = match iter.next() {
            Some(value) => u16::from_value(value, "health")?,
            None => u16::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let damage_amount = match iter.next() {
            Some(value) => u16::from_value(value, "damage_amount")?,
            None => u16::default(),
        };
        let custom = match iter.next() {
            Some(value) => u16::from_value(value, "custom")?,
            None => u16::default(),
        };
        let show_disguised_crit = match iter.next() {
            Some(value) => bool::from_value(value, "show_disguised_crit")?,
            None => bool::default(),
        };
        let crit = match iter.next() {
            Some(value) => bool::from_value(value, "crit")?,
            None => bool::default(),
        };
        let mini_crit = match iter.next() {
            Some(value) => bool::from_value(value, "mini_crit")?,
            None => bool::default(),
        };
        let all_see_crit = match iter.next() {
            Some(value) => bool::from_value(value, "all_see_crit")?,
            None => bool::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let bonus_effect = match iter.next() {
            Some(value) => u8::from_value(value, "bonus_effect")?,
            None => u8::default(),
        };
        Ok(PlayerHurtEvent {
            user_id,
            health,
            attacker,
            damage_amount,
            custom,
            show_disguised_crit,
            crit,
            mini_crit,
            all_see_crit,
            weapon_id,
            bonus_effect,
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
        let team_only = match iter.next() {
            Some(value) => bool::from_value(value, "team_only")?,
            None => bool::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(PlayerChatEvent {
            team_only,
            user_id,
            text,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let kills = match iter.next() {
            Some(value) => u16::from_value(value, "kills")?,
            None => u16::default(),
        };
        let deaths = match iter.next() {
            Some(value) => u16::from_value(value, "deaths")?,
            None => u16::default(),
        };
        let score = match iter.next() {
            Some(value) => u16::from_value(value, "score")?,
            None => u16::default(),
        };
        Ok(PlayerScoreEvent {
            user_id,
            kills,
            deaths,
            score,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let team = match iter.next() {
            Some(value) => u16::from_value(value, "team")?,
            None => u16::default(),
        };
        let class = match iter.next() {
            Some(value) => u16::from_value(value, "class")?,
            None => u16::default(),
        };
        Ok(PlayerSpawnEvent {
            user_id,
            team,
            class,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => u8::from_value(value, "weapon")?,
            None => u8::default(),
        };
        let mode = match iter.next() {
            Some(value) => u8::from_value(value, "mode")?,
            None => u8::default(),
        };
        Ok(PlayerShootEvent {
            user_id,
            weapon,
            mode,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let entity = match iter.next() {
            Some(value) => u16::from_value(value, "entity")?,
            None => u16::default(),
        };
        Ok(PlayerUseEvent { user_id, entity })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let old_name = match iter.next() {
            Some(value) => String::from_value(value, "old_name")?,
            None => String::default(),
        };
        let new_name = match iter.next() {
            Some(value) => String::from_value(value, "new_name")?,
            None => String::default(),
        };
        Ok(PlayerChangeNameEvent {
            user_id,
            old_name,
            new_name,
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
        let hint_message = match iter.next() {
            Some(value) => String::from_value(value, "hint_message")?,
            None => String::default(),
        };
        Ok(PlayerHintMessageEvent { hint_message })
    }
}
#[derive(Debug)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for BasePlayerTeleportedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        Ok(BasePlayerTeleportedEvent { ent_index })
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
        let map_name = match iter.next() {
            Some(value) => String::from_value(value, "map_name")?,
            None => String::default(),
        };
        Ok(GameNewMapEvent { map_name })
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
        let rounds_limit = match iter.next() {
            Some(value) => u32::from_value(value, "rounds_limit")?,
            None => u32::default(),
        };
        let time_limit = match iter.next() {
            Some(value) => u32::from_value(value, "time_limit")?,
            None => u32::default(),
        };
        let frag_limit = match iter.next() {
            Some(value) => u32::from_value(value, "frag_limit")?,
            None => u32::default(),
        };
        let objective = match iter.next() {
            Some(value) => String::from_value(value, "objective")?,
            None => String::default(),
        };
        Ok(GameStartEvent {
            rounds_limit,
            time_limit,
            frag_limit,
            objective,
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
        let winner = match iter.next() {
            Some(value) => u8::from_value(value, "winner")?,
            None => u8::default(),
        };
        Ok(GameEndEvent { winner })
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
        let time_limit = match iter.next() {
            Some(value) => u32::from_value(value, "time_limit")?,
            None => u32::default(),
        };
        let frag_limit = match iter.next() {
            Some(value) => u32::from_value(value, "frag_limit")?,
            None => u32::default(),
        };
        let objective = match iter.next() {
            Some(value) => String::from_value(value, "objective")?,
            None => String::default(),
        };
        Ok(RoundStartEvent {
            time_limit,
            frag_limit,
            objective,
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
        let winner = match iter.next() {
            Some(value) => u8::from_value(value, "winner")?,
            None => u8::default(),
        };
        let reason = match iter.next() {
            Some(value) => u8::from_value(value, "reason")?,
            None => u8::default(),
        };
        let message = match iter.next() {
            Some(value) => String::from_value(value, "message")?,
            None => String::default(),
        };
        Ok(RoundEndEvent {
            winner,
            reason,
            message,
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
        let target = match iter.next() {
            Some(value) => u8::from_value(value, "target")?,
            None => u8::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(GameMessageEvent { target, text })
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
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let material = match iter.next() {
            Some(value) => u8::from_value(value, "material")?,
            None => u8::default(),
        };
        Ok(BreakBreakableEvent {
            ent_index,
            user_id,
            material,
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
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(BreakPropEvent { ent_index, user_id })
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
        let ent_index_killed = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index_killed")?,
            None => u32::default(),
        };
        let ent_index_attacker = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index_attacker")?,
            None => u32::default(),
        };
        let ent_index_inflictor = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index_inflictor")?,
            None => u32::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        Ok(EntityKilledEvent {
            ent_index_killed,
            ent_index_attacker,
            ent_index_inflictor,
            damage_bits,
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
        let num_advanced = match iter.next() {
            Some(value) => u16::from_value(value, "num_advanced")?,
            None => u16::default(),
        };
        let num_bronze = match iter.next() {
            Some(value) => u16::from_value(value, "num_bronze")?,
            None => u16::default(),
        };
        let num_silver = match iter.next() {
            Some(value) => u16::from_value(value, "num_silver")?,
            None => u16::default(),
        };
        let num_gold = match iter.next() {
            Some(value) => u16::from_value(value, "num_gold")?,
            None => u16::default(),
        };
        Ok(BonusUpdatedEvent {
            num_advanced,
            num_bronze,
            num_silver,
            num_gold,
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
        let achievement_name = match iter.next() {
            Some(value) => String::from_value(value, "achievement_name")?,
            None => String::default(),
        };
        let cur_val = match iter.next() {
            Some(value) => u16::from_value(value, "cur_val")?,
            None => u16::default(),
        };
        let max_val = match iter.next() {
            Some(value) => u16::from_value(value, "max_val")?,
            None => u16::default(),
        };
        Ok(AchievementEventEvent {
            achievement_name,
            cur_val,
            max_val,
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
        let achievement_id = match iter.next() {
            Some(value) => u32::from_value(value, "achievement_id")?,
            None => u32::default(),
        };
        let cur_val = match iter.next() {
            Some(value) => u16::from_value(value, "cur_val")?,
            None => u16::default(),
        };
        let max_val = match iter.next() {
            Some(value) => u16::from_value(value, "max_val")?,
            None => u16::default(),
        };
        Ok(AchievementIncrementEvent {
            achievement_id,
            cur_val,
            max_val,
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
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        Ok(PhysgunPickupEvent { ent_index })
    }
}
#[derive(Debug)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for FlareIgniteNpcEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        Ok(FlareIgniteNpcEvent { ent_index })
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
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        Ok(RagdollDissolvedEvent { ent_index })
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
        let old_mode = match iter.next() {
            Some(value) => u16::from_value(value, "old_mode")?,
            None => u16::default(),
        };
        let new_mode = match iter.next() {
            Some(value) => u16::from_value(value, "new_mode")?,
            None => u16::default(),
        };
        let obs_target = match iter.next() {
            Some(value) => u16::from_value(value, "obs_target")?,
            None => u16::default(),
        };
        Ok(HLTVChangedModeEvent {
            old_mode,
            new_mode,
            obs_target,
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
        let mode = match iter.next() {
            Some(value) => u16::from_value(value, "mode")?,
            None => u16::default(),
        };
        let old_target = match iter.next() {
            Some(value) => u16::from_value(value, "old_target")?,
            None => u16::default(),
        };
        let obs_target = match iter.next() {
            Some(value) => u16::from_value(value, "obs_target")?,
            None => u16::default(),
        };
        Ok(HLTVChangedTargetEvent {
            mode,
            old_target,
            obs_target,
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
        let issue = match iter.next() {
            Some(value) => String::from_value(value, "issue")?,
            None => String::default(),
        };
        let param_1 = match iter.next() {
            Some(value) => String::from_value(value, "param_1")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let initiator = match iter.next() {
            Some(value) => u32::from_value(value, "initiator")?,
            None => u32::default(),
        };
        Ok(VoteStartedEvent {
            issue,
            param_1,
            team,
            initiator,
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
        let vote_option_1 = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option_1")?,
            None => u8::default(),
        };
        let vote_option_2 = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option_2")?,
            None => u8::default(),
        };
        let vote_option_3 = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option_3")?,
            None => u8::default(),
        };
        let vote_option_4 = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option_4")?,
            None => u8::default(),
        };
        let vote_option_5 = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option_5")?,
            None => u8::default(),
        };
        let potential_votes = match iter.next() {
            Some(value) => u8::from_value(value, "potential_votes")?,
            None => u8::default(),
        };
        Ok(VoteChangedEvent {
            vote_option_1,
            vote_option_2,
            vote_option_3,
            vote_option_4,
            vote_option_5,
            potential_votes,
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
        let details = match iter.next() {
            Some(value) => String::from_value(value, "details")?,
            None => String::default(),
        };
        let param_1 = match iter.next() {
            Some(value) => String::from_value(value, "param_1")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(VotePassedEvent {
            details,
            param_1,
            team,
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(VoteFailedEvent { team })
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
        let vote_option = match iter.next() {
            Some(value) => u8::from_value(value, "vote_option")?,
            None => u8::default(),
        };
        let team = match iter.next() {
            Some(value) => u16::from_value(value, "team")?,
            None => u16::default(),
        };
        let entity_id = match iter.next() {
            Some(value) => u32::from_value(value, "entity_id")?,
            None => u32::default(),
        };
        Ok(VoteCastEvent {
            vote_option,
            team,
            entity_id,
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
        let count = match iter.next() {
            Some(value) => u8::from_value(value, "count")?,
            None => u8::default(),
        };
        let option_1 = match iter.next() {
            Some(value) => String::from_value(value, "option_1")?,
            None => String::default(),
        };
        let option_2 = match iter.next() {
            Some(value) => String::from_value(value, "option_2")?,
            None => String::default(),
        };
        let option_3 = match iter.next() {
            Some(value) => String::from_value(value, "option_3")?,
            None => String::default(),
        };
        let option_4 = match iter.next() {
            Some(value) => String::from_value(value, "option_4")?,
            None => String::default(),
        };
        let option_5 = match iter.next() {
            Some(value) => String::from_value(value, "option_5")?,
            None => String::default(),
        };
        Ok(VoteOptionsEvent {
            count,
            option_1,
            option_2,
            option_3,
            option_4,
            option_5,
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
        let views = match iter.next() {
            Some(value) => u32::from_value(value, "views")?,
            None => u32::default(),
        };
        let likes = match iter.next() {
            Some(value) => u32::from_value(value, "likes")?,
            None => u32::default(),
        };
        let favorited = match iter.next() {
            Some(value) => u32::from_value(value, "favorited")?,
            None => u32::default(),
        };
        Ok(ReplayYoutubeStatsEvent {
            views,
            likes,
            favorited,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(IntroFinishEvent { player })
    }
}
#[derive(Debug)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroNextCameraEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(IntroNextCameraEvent { player })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let class = match iter.next() {
            Some(value) => u16::from_value(value, "class")?,
            None => u16::default(),
        };
        Ok(PlayerChangeClassEvent { user_id, class })
    }
}
#[derive(Debug)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl FromRawGameEvent for TfMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let seconds = match iter.next() {
            Some(value) => u32::from_value(value, "seconds")?,
            None => u32::default(),
        };
        Ok(TfMapTimeRemainingEvent { seconds })
    }
}
#[derive(Debug)]
pub struct TfGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TfGameOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let reason = match iter.next() {
            Some(value) => String::from_value(value, "reason")?,
            None => String::default(),
        };
        Ok(TfGameOverEvent { reason })
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
        let capping_team = match iter.next() {
            Some(value) => u16::from_value(value, "capping_team")?,
            None => u16::default(),
        };
        let capping_team_score = match iter.next() {
            Some(value) => u16::from_value(value, "capping_team_score")?,
            None => u16::default(),
        };
        Ok(CtfFlagCapturedEvent {
            capping_team,
            capping_team_score,
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ControlPointUpdateImagesEvent { index })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ControlPointUpdateLayoutEvent { index })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateCappingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ControlPointUpdateCappingEvent { index })
    }
}
#[derive(Debug)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateOwnerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ControlPointUpdateOwnerEvent { index })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let area = match iter.next() {
            Some(value) => u16::from_value(value, "area")?,
            None => u16::default(),
        };
        Ok(ControlPointStartTouchEvent { player, area })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let area = match iter.next() {
            Some(value) => u16::from_value(value, "area")?,
            None => u16::default(),
        };
        Ok(ControlPointEndTouchEvent { player, area })
    }
}
#[derive(Debug)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl FromRawGameEvent for ControlPointPulseElementEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(ControlPointPulseElementEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let int_data = match iter.next() {
            Some(value) => u16::from_value(value, "int_data")?,
            None => u16::default(),
        };
        Ok(ControlPointFakeCaptureEvent { player, int_data })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let int_data = match iter.next() {
            Some(value) => u16::from_value(value, "int_data")?,
            None => u16::default(),
        };
        Ok(ControlPointFakeCaptureMultiplierEvent { player, int_data })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: String,
}
impl FromRawGameEvent for TeamPlayRoundSelectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let round = match iter.next() {
            Some(value) => String::from_value(value, "round")?,
            None => String::default(),
        };
        Ok(TeamPlayRoundSelectedEvent { round })
    }
}
#[derive(Debug)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl FromRawGameEvent for TeamPlayRoundStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let full_reset = match iter.next() {
            Some(value) => bool::from_value(value, "full_reset")?,
            None => bool::default(),
        };
        Ok(TeamPlayRoundStartEvent { full_reset })
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
        let seconds = match iter.next() {
            Some(value) => u16::from_value(value, "seconds")?,
            None => u16::default(),
        };
        Ok(TeamPlayRoundRestartSecondsEvent { seconds })
    }
}
#[derive(Debug)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeamReadyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(TeamPlayTeamReadyEvent { team })
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let win_reason = match iter.next() {
            Some(value) => u8::from_value(value, "win_reason")?,
            None => u8::default(),
        };
        let flag_cap_limit = match iter.next() {
            Some(value) => u16::from_value(value, "flag_cap_limit")?,
            None => u16::default(),
        };
        let full_round = match iter.next() {
            Some(value) => u16::from_value(value, "full_round")?,
            None => u16::default(),
        };
        let round_time = match iter.next() {
            Some(value) => f32::from_value(value, "round_time")?,
            None => f32::default(),
        };
        let losing_team_num_caps = match iter.next() {
            Some(value) => u16::from_value(value, "losing_team_num_caps")?,
            None => u16::default(),
        };
        let was_sudden_death = match iter.next() {
            Some(value) => u8::from_value(value, "was_sudden_death")?,
            None => u8::default(),
        };
        Ok(TeamPlayRoundWinEvent {
            team,
            win_reason,
            flag_cap_limit,
            full_round,
            round_time,
            losing_team_num_caps,
            was_sudden_death,
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
        let reason = match iter.next() {
            Some(value) => u8::from_value(value, "reason")?,
            None => u8::default(),
        };
        Ok(TeamPlayRoundStalemateEvent { reason })
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
        let reason = match iter.next() {
            Some(value) => String::from_value(value, "reason")?,
            None => String::default(),
        };
        Ok(TeamPlayGameOverEvent { reason })
    }
}
#[derive(Debug)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let seconds = match iter.next() {
            Some(value) => u16::from_value(value, "seconds")?,
            None => u16::default(),
        };
        Ok(TeamPlayMapTimeRemainingEvent { seconds })
    }
}
#[derive(Debug)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl FromRawGameEvent for TeamPlayTimerFlashEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let time_remaining = match iter.next() {
            Some(value) => u16::from_value(value, "time_remaining")?,
            None => u16::default(),
        };
        Ok(TeamPlayTimerFlashEvent { time_remaining })
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
        let timer = match iter.next() {
            Some(value) => u16::from_value(value, "timer")?,
            None => u16::default(),
        };
        let seconds_added = match iter.next() {
            Some(value) => u16::from_value(value, "seconds_added")?,
            None => u16::default(),
        };
        Ok(TeamPlayTimerTimeAddedEvent {
            timer,
            seconds_added,
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let cap_team = match iter.next() {
            Some(value) => u8::from_value(value, "cap_team")?,
            None => u8::default(),
        };
        let cappers = match iter.next() {
            Some(value) => String::from_value(value, "cappers")?,
            None => String::default(),
        };
        let cap_time = match iter.next() {
            Some(value) => f32::from_value(value, "cap_time")?,
            None => f32::default(),
        };
        Ok(TeamPlayPointStartCaptureEvent {
            cp,
            cp_name,
            team,
            cap_team,
            cappers,
            cap_time,
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let cappers = match iter.next() {
            Some(value) => String::from_value(value, "cappers")?,
            None => String::default(),
        };
        Ok(TeamPlayPointCapturedEvent {
            cp,
            cp_name,
            team,
            cappers,
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(TeamPlayPointLockedEvent { cp, cp_name, team })
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(TeamPlayPointUnlockedEvent { cp, cp_name, team })
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let time_remaining = match iter.next() {
            Some(value) => f32::from_value(value, "time_remaining")?,
            None => f32::default(),
        };
        Ok(TeamPlayCaptureBrokenEvent {
            cp,
            cp_name,
            time_remaining,
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let cp_name = match iter.next() {
            Some(value) => String::from_value(value, "cp_name")?,
            None => String::default(),
        };
        let blocker = match iter.next() {
            Some(value) => u8::from_value(value, "blocker")?,
            None => u8::default(),
        };
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        Ok(TeamPlayCaptureBlockedEvent {
            cp,
            cp_name,
            blocker,
            victim,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let carrier = match iter.next() {
            Some(value) => u16::from_value(value, "carrier")?,
            None => u16::default(),
        };
        let event_type = match iter.next() {
            Some(value) => u16::from_value(value, "event_type")?,
            None => u16::default(),
        };
        let home = match iter.next() {
            Some(value) => u8::from_value(value, "home")?,
            None => u8::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(TeamPlayFlagEventEvent {
            player,
            carrier,
            event_type,
            home,
            team,
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
        let panel_style = match iter.next() {
            Some(value) => u8::from_value(value, "panel_style")?,
            None => u8::default(),
        };
        let winning_team = match iter.next() {
            Some(value) => u8::from_value(value, "winning_team")?,
            None => u8::default(),
        };
        let win_reason = match iter.next() {
            Some(value) => u8::from_value(value, "win_reason")?,
            None => u8::default(),
        };
        let cappers = match iter.next() {
            Some(value) => String::from_value(value, "cappers")?,
            None => String::default(),
        };
        let flag_cap_limit = match iter.next() {
            Some(value) => u16::from_value(value, "flag_cap_limit")?,
            None => u16::default(),
        };
        let blue_score = match iter.next() {
            Some(value) => u16::from_value(value, "blue_score")?,
            None => u16::default(),
        };
        let red_score = match iter.next() {
            Some(value) => u16::from_value(value, "red_score")?,
            None => u16::default(),
        };
        let blue_score_prev = match iter.next() {
            Some(value) => u16::from_value(value, "blue_score_prev")?,
            None => u16::default(),
        };
        let red_score_prev = match iter.next() {
            Some(value) => u16::from_value(value, "red_score_prev")?,
            None => u16::default(),
        };
        let round_complete = match iter.next() {
            Some(value) => u16::from_value(value, "round_complete")?,
            None => u16::default(),
        };
        let rounds_remaining = match iter.next() {
            Some(value) => u16::from_value(value, "rounds_remaining")?,
            None => u16::default(),
        };
        let player_1 = match iter.next() {
            Some(value) => u16::from_value(value, "player_1")?,
            None => u16::default(),
        };
        let player_1_points = match iter.next() {
            Some(value) => u16::from_value(value, "player_1_points")?,
            None => u16::default(),
        };
        let player_2 = match iter.next() {
            Some(value) => u16::from_value(value, "player_2")?,
            None => u16::default(),
        };
        let player_2_points = match iter.next() {
            Some(value) => u16::from_value(value, "player_2_points")?,
            None => u16::default(),
        };
        let player_3 = match iter.next() {
            Some(value) => u16::from_value(value, "player_3")?,
            None => u16::default(),
        };
        let player_3_points = match iter.next() {
            Some(value) => u16::from_value(value, "player_3_points")?,
            None => u16::default(),
        };
        let kill_stream_player_1 = match iter.next() {
            Some(value) => u16::from_value(value, "kill_stream_player_1")?,
            None => u16::default(),
        };
        let kill_stream_player_1_count = match iter.next() {
            Some(value) => u16::from_value(value, "kill_stream_player_1_count")?,
            None => u16::default(),
        };
        let game_over = match iter.next() {
            Some(value) => u8::from_value(value, "game_over")?,
            None => u8::default(),
        };
        Ok(TeamPlayWinPanelEvent {
            panel_style,
            winning_team,
            win_reason,
            cappers,
            flag_cap_limit,
            blue_score,
            red_score,
            blue_score_prev,
            red_score_prev,
            round_complete,
            rounds_remaining,
            player_1,
            player_1_points,
            player_2,
            player_2_points,
            player_3,
            player_3_points,
            kill_stream_player_1,
            kill_stream_player_1_count,
            game_over,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(TeamPlayTeamBalancedPlayerEvent { player, team })
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
        let alert_type = match iter.next() {
            Some(value) => u16::from_value(value, "alert_type")?,
            None => u16::default(),
        };
        Ok(TeamPlayAlertEvent { alert_type })
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
        let next_map = match iter.next() {
            Some(value) => String::from_value(value, "next_map")?,
            None => String::default(),
        };
        let map = match iter.next() {
            Some(value) => String::from_value(value, "map")?,
            None => String::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(TrainingCompleteEvent {
            next_map,
            map,
            text,
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
        let killer = match iter.next() {
            Some(value) => u16::from_value(value, "killer")?,
            None => u16::default(),
        };
        Ok(ShowFreezePanelEvent { killer })
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
        let score = match iter.next() {
            Some(value) => u16::from_value(value, "score")?,
            None => u16::default(),
        };
        Ok(LocalPlayerScoreChangedEvent { score })
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
        let building_type = match iter.next() {
            Some(value) => u8::from_value(value, "building_type")?,
            None => u8::default(),
        };
        let object_mode = match iter.next() {
            Some(value) => u8::from_value(value, "object_mode")?,
            None => u8::default(),
        };
        let remove = match iter.next() {
            Some(value) => u8::from_value(value, "remove")?,
            None => u8::default(),
        };
        Ok(BuildingInfoChangedEvent {
            building_type,
            object_mode,
            remove,
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
        let disguised = match iter.next() {
            Some(value) => bool::from_value(value, "disguised")?,
            None => bool::default(),
        };
        Ok(LocalPlayerChangeDisguiseEvent { disguised })
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
        let old_value = match iter.next() {
            Some(value) => u16::from_value(value, "old_value")?,
            None => u16::default(),
        };
        let new_value = match iter.next() {
            Some(value) => u16::from_value(value, "new_value")?,
            None => u16::default(),
        };
        Ok(PlayerAccountChangedEvent {
            old_value,
            new_value,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "ent_index")?,
            None => u32::default(),
        };
        Ok(FlagStatusUpdateEvent { user_id, ent_index })
    }
}
#[derive(Debug)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl FromRawGameEvent for PlayerStatsUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let force_upload = match iter.next() {
            Some(value) => bool::from_value(value, "force_upload")?,
            None => bool::default(),
        };
        Ok(PlayerStatsUpdatedEvent { force_upload })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let target_id = match iter.next() {
            Some(value) => u16::from_value(value, "target_id")?,
            None => u16::default(),
        };
        Ok(PlayerChargeDeployedEvent { user_id, target_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object = match iter.next() {
            Some(value) => u16::from_value(value, "object")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(PlayerBuiltObjectEvent {
            user_id,
            object,
            index,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object = match iter.next() {
            Some(value) => u16::from_value(value, "object")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let is_builder = match iter.next() {
            Some(value) => bool::from_value(value, "is_builder")?,
            None => bool::default(),
        };
        Ok(PlayerUpgradedObjectEvent {
            user_id,
            object,
            index,
            is_builder,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object = match iter.next() {
            Some(value) => u16::from_value(value, "object")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(PlayerCarryObjectEvent {
            user_id,
            object,
            index,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object = match iter.next() {
            Some(value) => u16::from_value(value, "object")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(PlayerDropObjectEvent {
            user_id,
            object,
            index,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object_type = match iter.next() {
            Some(value) => u16::from_value(value, "object_type")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ObjectRemovedEvent {
            user_id,
            object_type,
            index,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let object_type = match iter.next() {
            Some(value) => u16::from_value(value, "object_type")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let was_building = match iter.next() {
            Some(value) => bool::from_value(value, "was_building")?,
            None => bool::default(),
        };
        Ok(ObjectDestroyedEvent {
            user_id,
            attacker,
            assister,
            weapon,
            weapon_id,
            object_type,
            index,
            was_building,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let object_type = match iter.next() {
            Some(value) => u16::from_value(value, "object_type")?,
            None => u16::default(),
        };
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ObjectDetonatedEvent {
            user_id,
            object_type,
            index,
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let achievement = match iter.next() {
            Some(value) => u16::from_value(value, "achievement")?,
            None => u16::default(),
        };
        Ok(AchievementEarnedEvent {
            player,
            achievement,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let name_change = match iter.next() {
            Some(value) => bool::from_value(value, "name_change")?,
            None => bool::default(),
        };
        let ready_state = match iter.next() {
            Some(value) => u16::from_value(value, "ready_state")?,
            None => u16::default(),
        };
        let new_name = match iter.next() {
            Some(value) => String::from_value(value, "new_name")?,
            None => String::default(),
        };
        Ok(TournamentStateUpdateEvent {
            user_id,
            name_change,
            ready_state,
            new_name,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerCalledForMedicEvent { user_id })
    }
}
#[derive(Debug)]
pub struct PlayerAskedForBallEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerAskedForBallEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerAskedForBallEvent { user_id })
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
        let pyro_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "pyro_ent_index")?,
            None => u8::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "victim_ent_index")?,
            None => u8::default(),
        };
        let medic_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "medic_ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index,
            victim_ent_index,
            medic_ent_index,
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
        let pyro_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "pyro_ent_index")?,
            None => u8::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "victim_ent_index")?,
            None => u8::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u8::from_value(value, "weapon_id")?,
            None => u8::default(),
        };
        Ok(PlayerIgnitedEvent {
            pyro_ent_index,
            victim_ent_index,
            weapon_id,
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
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        let healer = match iter.next() {
            Some(value) => u8::from_value(value, "healer")?,
            None => u8::default(),
        };
        let item_definition_index = match iter.next() {
            Some(value) => u16::from_value(value, "item_definition_index")?,
            None => u16::default(),
        };
        Ok(PlayerExtinguishedEvent {
            victim,
            healer,
            item_definition_index,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let builder_id = match iter.next() {
            Some(value) => u16::from_value(value, "builder_id")?,
            None => u16::default(),
        };
        let dist = match iter.next() {
            Some(value) => f32::from_value(value, "dist")?,
            None => f32::default(),
        };
        Ok(PlayerTeleportedEvent {
            user_id,
            builder_id,
            dist,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerHealedMedicCallEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let medic_user_id = match iter.next() {
            Some(value) => u16::from_value(value, "medic_user_id")?,
            None => u16::default(),
        };
        Ok(PlayerInvulnedEvent {
            user_id,
            medic_user_id,
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let speed = match iter.next() {
            Some(value) => u8::from_value(value, "speed")?,
            None => u8::default(),
        };
        let players = match iter.next() {
            Some(value) => u8::from_value(value, "players")?,
            None => u8::default(),
        };
        Ok(EscortSpeedEvent {
            team,
            speed,
            players,
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let progress = match iter.next() {
            Some(value) => f32::from_value(value, "progress")?,
            None => f32::default(),
        };
        let reset = match iter.next() {
            Some(value) => bool::from_value(value, "reset")?,
            None => bool::default(),
        };
        Ok(EscortProgressEvent {
            team,
            progress,
            reset,
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let recede_time = match iter.next() {
            Some(value) => f32::from_value(value, "recede_time")?,
            None => f32::default(),
        };
        Ok(EscortRecedeEvent { team, recede_time })
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let points = match iter.next() {
            Some(value) => u8::from_value(value, "points")?,
            None => u8::default(),
        };
        Ok(PlayerEscortScoreEvent { player, points })
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
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        let ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "ent_index")?,
            None => u8::default(),
        };
        let weapon_def_index = match iter.next() {
            Some(value) => u32::from_value(value, "weapon_def_index")?,
            None => u32::default(),
        };
        Ok(PlayerHealOnHitEvent {
            amount,
            ent_index,
            weapon_def_index,
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
        let owner = match iter.next() {
            Some(value) => u16::from_value(value, "owner")?,
            None => u16::default(),
        };
        let target = match iter.next() {
            Some(value) => u16::from_value(value, "target")?,
            None => u16::default(),
        };
        Ok(PlayerStealSandvichEvent { owner, target })
    }
}
#[derive(Debug)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowClassLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let show = match iter.next() {
            Some(value) => bool::from_value(value, "show")?,
            None => bool::default(),
        };
        Ok(ShowClassLayoutEvent { show })
    }
}
#[derive(Debug)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowVsPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let show = match iter.next() {
            Some(value) => bool::from_value(value, "show")?,
            None => bool::default(),
        };
        Ok(ShowVsPanelEvent { show })
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
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        let kind = match iter.next() {
            Some(value) => u32::from_value(value, "kind")?,
            None => u32::default(),
        };
        Ok(PlayerDamagedEvent { amount, kind })
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let message = match iter.next() {
            Some(value) => u8::from_value(value, "message")?,
            None => u8::default(),
        };
        Ok(ArenaPlayerNotificationEvent { player, message })
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let streak = match iter.next() {
            Some(value) => u8::from_value(value, "streak")?,
            None => u8::default(),
        };
        Ok(ArenaMatchMaxStreakEvent { team, streak })
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
        let panel_style = match iter.next() {
            Some(value) => u8::from_value(value, "panel_style")?,
            None => u8::default(),
        };
        let winning_team = match iter.next() {
            Some(value) => u8::from_value(value, "winning_team")?,
            None => u8::default(),
        };
        let win_reason = match iter.next() {
            Some(value) => u8::from_value(value, "win_reason")?,
            None => u8::default(),
        };
        let cappers = match iter.next() {
            Some(value) => String::from_value(value, "cappers")?,
            None => String::default(),
        };
        let flag_cap_limit = match iter.next() {
            Some(value) => u16::from_value(value, "flag_cap_limit")?,
            None => u16::default(),
        };
        let blue_score = match iter.next() {
            Some(value) => u16::from_value(value, "blue_score")?,
            None => u16::default(),
        };
        let red_score = match iter.next() {
            Some(value) => u16::from_value(value, "red_score")?,
            None => u16::default(),
        };
        let blue_score_prev = match iter.next() {
            Some(value) => u16::from_value(value, "blue_score_prev")?,
            None => u16::default(),
        };
        let red_score_prev = match iter.next() {
            Some(value) => u16::from_value(value, "red_score_prev")?,
            None => u16::default(),
        };
        let round_complete = match iter.next() {
            Some(value) => u16::from_value(value, "round_complete")?,
            None => u16::default(),
        };
        let player_1 = match iter.next() {
            Some(value) => u16::from_value(value, "player_1")?,
            None => u16::default(),
        };
        let player_1_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_1_damage")?,
            None => u16::default(),
        };
        let player_1_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_1_healing")?,
            None => u16::default(),
        };
        let player_1_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_1_lifetime")?,
            None => u16::default(),
        };
        let player_1_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_1_kills")?,
            None => u16::default(),
        };
        let player_2 = match iter.next() {
            Some(value) => u16::from_value(value, "player_2")?,
            None => u16::default(),
        };
        let player_2_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_2_damage")?,
            None => u16::default(),
        };
        let player_2_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_2_healing")?,
            None => u16::default(),
        };
        let player_2_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_2_lifetime")?,
            None => u16::default(),
        };
        let player_2_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_2_kills")?,
            None => u16::default(),
        };
        let player_3 = match iter.next() {
            Some(value) => u16::from_value(value, "player_3")?,
            None => u16::default(),
        };
        let player_3_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_3_damage")?,
            None => u16::default(),
        };
        let player_3_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_3_healing")?,
            None => u16::default(),
        };
        let player_3_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_3_lifetime")?,
            None => u16::default(),
        };
        let player_3_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_3_kills")?,
            None => u16::default(),
        };
        let player_4 = match iter.next() {
            Some(value) => u16::from_value(value, "player_4")?,
            None => u16::default(),
        };
        let player_4_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_4_damage")?,
            None => u16::default(),
        };
        let player_4_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_4_healing")?,
            None => u16::default(),
        };
        let player_4_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_4_lifetime")?,
            None => u16::default(),
        };
        let player_4_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_4_kills")?,
            None => u16::default(),
        };
        let player_5 = match iter.next() {
            Some(value) => u16::from_value(value, "player_5")?,
            None => u16::default(),
        };
        let player_5_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_5_damage")?,
            None => u16::default(),
        };
        let player_5_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_5_healing")?,
            None => u16::default(),
        };
        let player_5_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_5_lifetime")?,
            None => u16::default(),
        };
        let player_5_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_5_kills")?,
            None => u16::default(),
        };
        let player_6 = match iter.next() {
            Some(value) => u16::from_value(value, "player_6")?,
            None => u16::default(),
        };
        let player_6_damage = match iter.next() {
            Some(value) => u16::from_value(value, "player_6_damage")?,
            None => u16::default(),
        };
        let player_6_healing = match iter.next() {
            Some(value) => u16::from_value(value, "player_6_healing")?,
            None => u16::default(),
        };
        let player_6_lifetime = match iter.next() {
            Some(value) => u16::from_value(value, "player_6_lifetime")?,
            None => u16::default(),
        };
        let player_6_kills = match iter.next() {
            Some(value) => u16::from_value(value, "player_6_kills")?,
            None => u16::default(),
        };
        Ok(ArenaWinPanelEvent {
            panel_style,
            winning_team,
            win_reason,
            cappers,
            flag_cap_limit,
            blue_score,
            red_score,
            blue_score_prev,
            red_score_prev,
            round_complete,
            player_1,
            player_1_damage,
            player_1_healing,
            player_1_lifetime,
            player_1_kills,
            player_2,
            player_2_damage,
            player_2_healing,
            player_2_lifetime,
            player_2_kills,
            player_3,
            player_3_damage,
            player_3_healing,
            player_3_lifetime,
            player_3_kills,
            player_4,
            player_4_damage,
            player_4_healing,
            player_4_lifetime,
            player_4_kills,
            player_5,
            player_5_damage,
            player_5_healing,
            player_5_lifetime,
            player_5_kills,
            player_6,
            player_6_damage,
            player_6_healing,
            player_6_lifetime,
            player_6_kills,
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
        let panel_style = match iter.next() {
            Some(value) => u8::from_value(value, "panel_style")?,
            None => u8::default(),
        };
        let winning_team = match iter.next() {
            Some(value) => u8::from_value(value, "winning_team")?,
            None => u8::default(),
        };
        let win_reason = match iter.next() {
            Some(value) => u8::from_value(value, "win_reason")?,
            None => u8::default(),
        };
        Ok(PveWinPanelEvent {
            panel_style,
            winning_team,
            win_reason,
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        Ok(AirDashEvent { player })
    }
}
#[derive(Debug)]
pub struct LandedEvent {
    pub player: u8,
}
impl FromRawGameEvent for LandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        Ok(LandedEvent { player })
    }
}
#[derive(Debug)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl FromRawGameEvent for PlayerDamageDodgedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let damage = match iter.next() {
            Some(value) => u16::from_value(value, "damage")?,
            None => u16::default(),
        };
        Ok(PlayerDamageDodgedEvent { damage })
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
        let stunner = match iter.next() {
            Some(value) => u16::from_value(value, "stunner")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        let victim_capping = match iter.next() {
            Some(value) => bool::from_value(value, "victim_capping")?,
            None => bool::default(),
        };
        let big_stun = match iter.next() {
            Some(value) => bool::from_value(value, "big_stun")?,
            None => bool::default(),
        };
        Ok(PlayerStunnedEvent {
            stunner,
            victim,
            victim_capping,
            big_stun,
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
        let scout_id = match iter.next() {
            Some(value) => u16::from_value(value, "scout_id")?,
            None => u16::default(),
        };
        let target_id = match iter.next() {
            Some(value) => u16::from_value(value, "target_id")?,
            None => u16::default(),
        };
        Ok(ScoutGrandSlamEvent {
            scout_id,
            target_id,
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
        let target_index = match iter.next() {
            Some(value) => u16::from_value(value, "target_index")?,
            None => u16::default(),
        };
        let x = match iter.next() {
            Some(value) => f32::from_value(value, "x")?,
            None => f32::default(),
        };
        let y = match iter.next() {
            Some(value) => f32::from_value(value, "y")?,
            None => f32::default(),
        };
        let z = match iter.next() {
            Some(value) => f32::from_value(value, "z")?,
            None => f32::default(),
        };
        Ok(ScoutSlamdollLandedEvent {
            target_index,
            x,
            y,
            z,
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
        let attached_entity = match iter.next() {
            Some(value) => u16::from_value(value, "attached_entity")?,
            None => u16::default(),
        };
        let shooter = match iter.next() {
            Some(value) => u16::from_value(value, "shooter")?,
            None => u16::default(),
        };
        let bone_index_attached = match iter.next() {
            Some(value) => u16::from_value(value, "bone_index_attached")?,
            None => u16::default(),
        };
        let bone_position_x = match iter.next() {
            Some(value) => f32::from_value(value, "bone_position_x")?,
            None => f32::default(),
        };
        let bone_position_y = match iter.next() {
            Some(value) => f32::from_value(value, "bone_position_y")?,
            None => f32::default(),
        };
        let bone_position_z = match iter.next() {
            Some(value) => f32::from_value(value, "bone_position_z")?,
            None => f32::default(),
        };
        let bone_angles_x = match iter.next() {
            Some(value) => f32::from_value(value, "bone_angles_x")?,
            None => f32::default(),
        };
        let bone_angles_y = match iter.next() {
            Some(value) => f32::from_value(value, "bone_angles_y")?,
            None => f32::default(),
        };
        let bone_angles_z = match iter.next() {
            Some(value) => f32::from_value(value, "bone_angles_z")?,
            None => f32::default(),
        };
        let projectile_type = match iter.next() {
            Some(value) => u16::from_value(value, "projectile_type")?,
            None => u16::default(),
        };
        let is_crit = match iter.next() {
            Some(value) => bool::from_value(value, "is_crit")?,
            None => bool::default(),
        };
        Ok(ArrowImpactEvent {
            attached_entity,
            shooter,
            bone_index_attached,
            bone_position_x,
            bone_position_y,
            bone_position_z,
            bone_angles_x,
            bone_angles_y,
            bone_angles_z,
            projectile_type,
            is_crit,
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
        let thrower_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "thrower_ent_index")?,
            None => u8::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "victim_ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerJaratedEvent {
            thrower_ent_index,
            victim_ent_index,
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
        let thrower_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "thrower_ent_index")?,
            None => u8::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "victim_ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index,
            victim_ent_index,
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
        let attacker_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "attacker_ent_index")?,
            None => u8::default(),
        };
        let blocker_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "blocker_ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index,
            blocker_ent_index,
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
        let pinned = match iter.next() {
            Some(value) => u8::from_value(value, "pinned")?,
            None => u8::default(),
        };
        Ok(PlayerPinnedEvent { pinned })
    }
}
#[derive(Debug)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl FromRawGameEvent for PlayerHealedByMedicEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let medic = match iter.next() {
            Some(value) => u8::from_value(value, "medic")?,
            None => u8::default(),
        };
        Ok(PlayerHealedByMedicEvent { medic })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let owner_id = match iter.next() {
            Some(value) => u16::from_value(value, "owner_id")?,
            None => u16::default(),
        };
        let object = match iter.next() {
            Some(value) => u8::from_value(value, "object")?,
            None => u8::default(),
        };
        let sapper_id = match iter.next() {
            Some(value) => u16::from_value(value, "sapper_id")?,
            None => u16::default(),
        };
        Ok(PlayerSappedObjectEvent {
            user_id,
            owner_id,
            object,
            sapper_id,
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let quality = match iter.next() {
            Some(value) => u8::from_value(value, "quality")?,
            None => u8::default(),
        };
        let method = match iter.next() {
            Some(value) => u8::from_value(value, "method")?,
            None => u8::default(),
        };
        let item_def = match iter.next() {
            Some(value) => u32::from_value(value, "item_def")?,
            None => u32::default(),
        };
        let is_strange = match iter.next() {
            Some(value) => u8::from_value(value, "is_strange")?,
            None => u8::default(),
        };
        let is_unusual = match iter.next() {
            Some(value) => u8::from_value(value, "is_unusual")?,
            None => u8::default(),
        };
        let wear = match iter.next() {
            Some(value) => f32::from_value(value, "wear")?,
            None => f32::default(),
        };
        Ok(ItemFoundEvent {
            player,
            quality,
            method,
            item_def,
            is_strange,
            is_unusual,
            wear,
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
        let world_pos_x = match iter.next() {
            Some(value) => f32::from_value(value, "world_pos_x")?,
            None => f32::default(),
        };
        let world_pos_y = match iter.next() {
            Some(value) => f32::from_value(value, "world_pos_y")?,
            None => f32::default(),
        };
        let world_pos_z = match iter.next() {
            Some(value) => f32::from_value(value, "world_pos_z")?,
            None => f32::default(),
        };
        let world_normal_x = match iter.next() {
            Some(value) => f32::from_value(value, "world_normal_x")?,
            None => f32::default(),
        };
        let world_normal_y = match iter.next() {
            Some(value) => f32::from_value(value, "world_normal_y")?,
            None => f32::default(),
        };
        let world_normal_z = match iter.next() {
            Some(value) => f32::from_value(value, "world_normal_z")?,
            None => f32::default(),
        };
        let id = match iter.next() {
            Some(value) => u32::from_value(value, "id")?,
            None => u32::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        let lifetime = match iter.next() {
            Some(value) => f32::from_value(value, "lifetime")?,
            None => f32::default(),
        };
        let visibility_bit_field = match iter.next() {
            Some(value) => u32::from_value(value, "visibility_bit_field")?,
            None => u32::default(),
        };
        let follow_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "follow_ent_index")?,
            None => u32::default(),
        };
        let show_distance = match iter.next() {
            Some(value) => bool::from_value(value, "show_distance")?,
            None => bool::default(),
        };
        let play_sound = match iter.next() {
            Some(value) => String::from_value(value, "play_sound")?,
            None => String::default(),
        };
        let show_effect = match iter.next() {
            Some(value) => bool::from_value(value, "show_effect")?,
            None => bool::default(),
        };
        Ok(ShowAnnotationEvent {
            world_pos_x,
            world_pos_y,
            world_pos_z,
            world_normal_x,
            world_normal_y,
            world_normal_z,
            id,
            text,
            lifetime,
            visibility_bit_field,
            follow_ent_index,
            show_distance,
            play_sound,
            show_effect,
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
        let id = match iter.next() {
            Some(value) => u32::from_value(value, "id")?,
            None => u32::default(),
        };
        Ok(HideAnnotationEvent { id })
    }
}
#[derive(Debug)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PostInventoryApplicationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PostInventoryApplicationEvent { user_id })
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let time = match iter.next() {
            Some(value) => f32::from_value(value, "time")?,
            None => f32::default(),
        };
        Ok(ControlPointUnlockUpdatedEvent { index, time })
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
        let buff_type = match iter.next() {
            Some(value) => u8::from_value(value, "buff_type")?,
            None => u8::default(),
        };
        let buff_owner = match iter.next() {
            Some(value) => u16::from_value(value, "buff_owner")?,
            None => u16::default(),
        };
        Ok(DeployBuffBannerEvent {
            buff_type,
            buff_owner,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let buff_owner = match iter.next() {
            Some(value) => u16::from_value(value, "buff_owner")?,
            None => u16::default(),
        };
        let buff_type = match iter.next() {
            Some(value) => u8::from_value(value, "buff_type")?,
            None => u8::default(),
        };
        Ok(PlayerBuffEvent {
            user_id,
            buff_owner,
            buff_type,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let healing = match iter.next() {
            Some(value) => u16::from_value(value, "healing")?,
            None => u16::default(),
        };
        let charged = match iter.next() {
            Some(value) => bool::from_value(value, "charged")?,
            None => bool::default(),
        };
        Ok(MedicDeathEvent {
            user_id,
            attacker,
            healing,
            charged,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(HalloweenPumpkinGrabEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let play_sound = match iter.next() {
            Some(value) => bool::from_value(value, "play_sound")?,
            None => bool::default(),
        };
        Ok(RocketJumpEvent {
            user_id,
            play_sound,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(RocketJumpLandedEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let play_sound = match iter.next() {
            Some(value) => bool::from_value(value, "play_sound")?,
            None => bool::default(),
        };
        Ok(StickyJumpEvent {
            user_id,
            play_sound,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(StickyJumpLandedEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let play_sound = match iter.next() {
            Some(value) => bool::from_value(value, "play_sound")?,
            None => bool::default(),
        };
        Ok(RocketPackLaunchEvent {
            user_id,
            play_sound,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(RocketPackLandedEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let medic = match iter.next() {
            Some(value) => u16::from_value(value, "medic")?,
            None => u16::default(),
        };
        Ok(MedicDefendedEvent { user_id, medic })
    }
}
#[derive(Debug)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl FromRawGameEvent for LocalPlayerHealedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        Ok(LocalPlayerHealedEvent { amount })
    }
}
#[derive(Debug)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerDestroyedPipeBombEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerDestroyedPipeBombEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let owner_id = match iter.next() {
            Some(value) => u16::from_value(value, "owner_id")?,
            None => u16::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let object_ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "object_ent_index")?,
            None => u16::default(),
        };
        Ok(ObjectDeflectedEvent {
            user_id,
            owner_id,
            weapon_id,
            object_ent_index,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(PlayerMvpEvent { player })
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
        let area = match iter.next() {
            Some(value) => u32::from_value(value, "area")?,
            None => u32::default(),
        };
        let blocked = match iter.next() {
            Some(value) => bool::from_value(value, "blocked")?,
            None => bool::default(),
        };
        Ok(NavBlockedEvent { area, blocked })
    }
}
#[derive(Debug)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl FromRawGameEvent for PathTrackPassedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(PathTrackPassedEvent { index })
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let count = match iter.next() {
            Some(value) => u8::from_value(value, "count")?,
            None => u8::default(),
        };
        Ok(NumCappersChangedEvent { index, count })
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
        let index = match iter.next() {
            Some(value) => u8::from_value(value, "index")?,
            None => u8::default(),
        };
        let object = match iter.next() {
            Some(value) => u8::from_value(value, "object")?,
            None => u8::default(),
        };
        Ok(UpdateStatusItemEvent { index, object })
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
        let achievement = match iter.next() {
            Some(value) => u16::from_value(value, "achievement")?,
            None => u16::default(),
        };
        Ok(AchievementEarnedLocalEvent { achievement })
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
        let patient = match iter.next() {
            Some(value) => u16::from_value(value, "patient")?,
            None => u16::default(),
        };
        let healer = match iter.next() {
            Some(value) => u16::from_value(value, "healer")?,
            None => u16::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        Ok(PlayerHealedEvent {
            patient,
            healer,
            amount,
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
        let building = match iter.next() {
            Some(value) => u16::from_value(value, "building")?,
            None => u16::default(),
        };
        let healer = match iter.next() {
            Some(value) => u16::from_value(value, "healer")?,
            None => u16::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        Ok(BuildingHealedEvent {
            building,
            healer,
            amount,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let item = match iter.next() {
            Some(value) => String::from_value(value, "item")?,
            None => String::default(),
        };
        Ok(ItemPickupEvent { user_id, item })
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
        let killer = match iter.next() {
            Some(value) => u16::from_value(value, "killer")?,
            None => u16::default(),
        };
        let score_type = match iter.next() {
            Some(value) => u16::from_value(value, "score_type")?,
            None => u16::default(),
        };
        let initiator = match iter.next() {
            Some(value) => u16::from_value(value, "initiator")?,
            None => u16::default(),
        };
        let target = match iter.next() {
            Some(value) => u16::from_value(value, "target")?,
            None => u16::default(),
        };
        let initiator_score = match iter.next() {
            Some(value) => u16::from_value(value, "initiator_score")?,
            None => u16::default(),
        };
        let target_score = match iter.next() {
            Some(value) => u16::from_value(value, "target_score")?,
            None => u16::default(),
        };
        Ok(DuelStatusEvent {
            killer,
            score_type,
            initiator,
            target,
            initiator_score,
            target_score,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        let stun_flags = match iter.next() {
            Some(value) => u16::from_value(value, "stun_flags")?,
            None => u16::default(),
        };
        let death_flags = match iter.next() {
            Some(value) => u16::from_value(value, "death_flags")?,
            None => u16::default(),
        };
        let silent_kill = match iter.next() {
            Some(value) => bool::from_value(value, "silent_kill")?,
            None => bool::default(),
        };
        let assister_fallback = match iter.next() {
            Some(value) => String::from_value(value, "assister_fallback")?,
            None => String::default(),
        };
        Ok(FishNoticeEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            assister,
            weapon_log_class_name,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        let stun_flags = match iter.next() {
            Some(value) => u16::from_value(value, "stun_flags")?,
            None => u16::default(),
        };
        let death_flags = match iter.next() {
            Some(value) => u16::from_value(value, "death_flags")?,
            None => u16::default(),
        };
        let silent_kill = match iter.next() {
            Some(value) => bool::from_value(value, "silent_kill")?,
            None => bool::default(),
        };
        let assister_fallback = match iter.next() {
            Some(value) => String::from_value(value, "assister_fallback")?,
            None => String::default(),
        };
        Ok(FishNoticeArmEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            assister,
            weapon_log_class_name,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        let stun_flags = match iter.next() {
            Some(value) => u16::from_value(value, "stun_flags")?,
            None => u16::default(),
        };
        let death_flags = match iter.next() {
            Some(value) => u16::from_value(value, "death_flags")?,
            None => u16::default(),
        };
        let silent_kill = match iter.next() {
            Some(value) => bool::from_value(value, "silent_kill")?,
            None => bool::default(),
        };
        let assister_fallback = match iter.next() {
            Some(value) => String::from_value(value, "assister_fallback")?,
            None => String::default(),
        };
        Ok(SlapNoticeEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            assister,
            weapon_log_class_name,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        let stun_flags = match iter.next() {
            Some(value) => u16::from_value(value, "stun_flags")?,
            None => u16::default(),
        };
        let death_flags = match iter.next() {
            Some(value) => u16::from_value(value, "death_flags")?,
            None => u16::default(),
        };
        let silent_kill = match iter.next() {
            Some(value) => bool::from_value(value, "silent_kill")?,
            None => bool::default(),
        };
        let assister_fallback = match iter.next() {
            Some(value) => String::from_value(value, "assister_fallback")?,
            None => String::default(),
        };
        let total_hits = match iter.next() {
            Some(value) => u16::from_value(value, "total_hits")?,
            None => u16::default(),
        };
        Ok(ThrowableHitEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            assister,
            weapon_log_class_name,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback,
            total_hits,
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(MerasmusSummonedEvent { level })
    }
}
#[derive(Debug)]
pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(MerasmusKilledEvent { level })
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        let time_remaining = match iter.next() {
            Some(value) => u8::from_value(value, "time_remaining")?,
            None => u8::default(),
        };
        Ok(MerasmusEscapeWarningEvent {
            level,
            time_remaining,
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(MerasmusEscapedEvent { level })
    }
}
#[derive(Debug)]
pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(EyeballBossSummonedEvent { level })
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        let player_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "player_ent_index")?,
            None => u8::default(),
        };
        Ok(EyeballBossStunnedEvent {
            level,
            player_ent_index,
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(EyeballBossKilledEvent { level })
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        let player_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "player_ent_index")?,
            None => u8::default(),
        };
        Ok(EyeballBossKillerEvent {
            level,
            player_ent_index,
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        let time_remaining = match iter.next() {
            Some(value) => u8::from_value(value, "time_remaining")?,
            None => u8::default(),
        };
        Ok(EyeballBossEscapeImminentEvent {
            level,
            time_remaining,
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
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(EyeballBossEscapedEvent { level })
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
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        let health = match iter.next() {
            Some(value) => u16::from_value(value, "health")?,
            None => u16::default(),
        };
        let attacker_player = match iter.next() {
            Some(value) => u16::from_value(value, "attacker_player")?,
            None => u16::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_amount = match iter.next() {
            Some(value) => u16::from_value(value, "damage_amount")?,
            None => u16::default(),
        };
        let crit = match iter.next() {
            Some(value) => bool::from_value(value, "crit")?,
            None => bool::default(),
        };
        let boss = match iter.next() {
            Some(value) => u16::from_value(value, "boss")?,
            None => u16::default(),
        };
        Ok(NpcHurtEvent {
            ent_index,
            health,
            attacker_player,
            weapon_id,
            damage_amount,
            crit,
            boss,
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let time = match iter.next() {
            Some(value) => f32::from_value(value, "time")?,
            None => f32::default(),
        };
        Ok(ControlPointTimerUpdatedEvent { index, time })
    }
}
#[derive(Debug)]
pub struct PlayerHighFiveStartEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerHighFiveStartEvent { ent_index })
    }
}
#[derive(Debug)]
pub struct PlayerHighFiveCancelEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveCancelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerHighFiveCancelEvent { ent_index })
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
        let initiator_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "initiator_ent_index")?,
            None => u8::default(),
        };
        let partner_ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "partner_ent_index")?,
            None => u8::default(),
        };
        Ok(PlayerHighFiveSuccessEvent {
            initiator_ent_index,
            partner_ent_index,
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
        let points = match iter.next() {
            Some(value) => u16::from_value(value, "points")?,
            None => u16::default(),
        };
        let player_ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "player_ent_index")?,
            None => u16::default(),
        };
        let source_ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "source_ent_index")?,
            None => u16::default(),
        };
        Ok(PlayerBonusPointsEvent {
            points,
            player_ent_index,
            source_ent_index,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let cost = match iter.next() {
            Some(value) => u16::from_value(value, "cost")?,
            None => u16::default(),
        };
        Ok(PlayerBuybackEvent { player, cost })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let kind = match iter.next() {
            Some(value) => u16::from_value(value, "kind")?,
            None => u16::default(),
        };
        let time = match iter.next() {
            Some(value) => f32::from_value(value, "time")?,
            None => f32::default(),
        };
        Ok(PlayerUsedPowerUpBottleEvent { player, kind, time })
    }
}
#[derive(Debug)]
pub struct ChristmasGiftGrabEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for ChristmasGiftGrabEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(ChristmasGiftGrabEvent { user_id })
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
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        let zone_id = match iter.next() {
            Some(value) => u16::from_value(value, "zone_id")?,
            None => u16::default(),
        };
        Ok(PlayerKilledAchievementZoneEvent {
            attacker,
            victim,
            zone_id,
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
        let match_group = match iter.next() {
            Some(value) => u16::from_value(value, "match_group")?,
            None => u16::default(),
        };
        Ok(PartyQueueStateChangedEvent { match_group })
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
        let steam_id = match iter.next() {
            Some(value) => String::from_value(value, "steam_id")?,
            None => String::default(),
        };
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        let kind = match iter.next() {
            Some(value) => u16::from_value(value, "kind")?,
            None => u16::default(),
        };
        Ok(PartyChatEvent {
            steam_id,
            text,
            kind,
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
        let steam_id = match iter.next() {
            Some(value) => String::from_value(value, "steam_id")?,
            None => String::default(),
        };
        Ok(PartyMemberJoinEvent { steam_id })
    }
}
#[derive(Debug)]
pub struct PartyMemberLeaveEvent {
    pub steam_id: String,
}
impl FromRawGameEvent for PartyMemberLeaveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let steam_id = match iter.next() {
            Some(value) => String::from_value(value, "steam_id")?,
            None => String::default(),
        };
        Ok(PartyMemberLeaveEvent { steam_id })
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
        let class = match iter.next() {
            Some(value) => u16::from_value(value, "class")?,
            None => u16::default(),
        };
        let count = match iter.next() {
            Some(value) => u16::from_value(value, "count")?,
            None => u16::default(),
        };
        Ok(MvmMissionUpdateEvent { class, count })
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
        let currency = match iter.next() {
            Some(value) => u16::from_value(value, "currency")?,
            None => u16::default(),
        };
        Ok(PlayerCurrencyChangedEvent { currency })
    }
}
#[derive(Debug)]
pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl FromRawGameEvent for DoomsdayRocketOpenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        Ok(DoomsdayRocketOpenEvent { team })
    }
}
#[derive(Debug)]
pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl FromRawGameEvent for RemoveNemesisRelationshipsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(RemoveNemesisRelationshipsEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmQuickSentryUpgradeEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmKillRobotDeliveringBombEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let currency = match iter.next() {
            Some(value) => u16::from_value(value, "currency")?,
            None => u16::default(),
        };
        Ok(MvmPickupCurrencyEvent { player, currency })
    }
}
#[derive(Debug)]
pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MvmBombCarrierKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(MvmBombCarrierKilledEvent { level })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let det_x = match iter.next() {
            Some(value) => f32::from_value(value, "det_x")?,
            None => f32::default(),
        };
        let det_y = match iter.next() {
            Some(value) => f32::from_value(value, "det_y")?,
            None => f32::default(),
        };
        let det_z = match iter.next() {
            Some(value) => f32::from_value(value, "det_z")?,
            None => f32::default(),
        };
        Ok(MvmSentryBusterDetonateEvent {
            player,
            det_x,
            det_y,
            det_z,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmScoutMarkedForDeathEvent { player })
    }
}
#[derive(Debug)]
pub struct MvmMedicPowerUpSharedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmMedicPowerUpSharedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmMedicPowerUpSharedEvent { player })
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
        let wave_index = match iter.next() {
            Some(value) => u16::from_value(value, "wave_index")?,
            None => u16::default(),
        };
        let max_waves = match iter.next() {
            Some(value) => u16::from_value(value, "max_waves")?,
            None => u16::default(),
        };
        let advanced = match iter.next() {
            Some(value) => u16::from_value(value, "advanced")?,
            None => u16::default(),
        };
        Ok(MvmBeginWaveEvent {
            wave_index,
            max_waves,
            advanced,
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
        let advanced = match iter.next() {
            Some(value) => bool::from_value(value, "advanced")?,
            None => bool::default(),
        };
        Ok(MvmWaveCompleteEvent { advanced })
    }
}
#[derive(Debug)]
pub struct MvmMissionCompleteEvent {
    pub mission: String,
}
impl FromRawGameEvent for MvmMissionCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let mission = match iter.next() {
            Some(value) => String::from_value(value, "mission")?,
            None => String::default(),
        };
        Ok(MvmMissionCompleteEvent { mission })
    }
}
#[derive(Debug)]
pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombResetByPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmBombResetByPlayerEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MvmBombDeployResetByPlayerEvent { player })
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
        let ent_index = match iter.next() {
            Some(value) => u8::from_value(value, "ent_index")?,
            None => u8::default(),
        };
        Ok(DamageResistedEvent { ent_index })
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
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        let marker_ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "marker_ent_index")?,
            None => u16::default(),
        };
        Ok(RevivePlayerNotifyEvent {
            ent_index,
            marker_ent_index,
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
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        Ok(RevivePlayerStoppedEvent { ent_index })
    }
}
#[derive(Debug)]
pub struct RevivePlayerCompleteEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        Ok(RevivePlayerCompleteEvent { ent_index })
    }
}
#[derive(Debug)]
pub struct PlayerTurnedToGhostEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerTurnedToGhostEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        Ok(PlayerTurnedToGhostEvent { user_id })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let damage = match iter.next() {
            Some(value) => f32::from_value(value, "damage")?,
            None => f32::default(),
        };
        Ok(MedigunShieldBlockedDamageEvent { user_id, damage })
    }
}
#[derive(Debug)]
pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl FromRawGameEvent for MvmAdvWaveCompleteNoGatesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(MvmAdvWaveCompleteNoGatesEvent { index })
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let currency = match iter.next() {
            Some(value) => u16::from_value(value, "currency")?,
            None => u16::default(),
        };
        Ok(MvmSniperHeadshotCurrencyEvent { user_id, currency })
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
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        Ok(PlayerDirectHitStunEvent { attacker, victim })
    }
}
#[derive(Debug)]
pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl FromRawGameEvent for MvmSentryBusterKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let sentry_buster = match iter.next() {
            Some(value) => u16::from_value(value, "sentry_buster")?,
            None => u16::default(),
        };
        Ok(MvmSentryBusterKilledEvent { sentry_buster })
    }
}
#[derive(Debug)]
pub struct UpgradesFileChangedEvent {
    pub path: String,
}
impl FromRawGameEvent for UpgradesFileChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let path = match iter.next() {
            Some(value) => String::from_value(value, "path")?,
            None => String::default(),
        };
        Ok(UpgradesFileChangedEvent { path })
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
        let points = match iter.next() {
            Some(value) => u16::from_value(value, "points")?,
            None => u16::default(),
        };
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let method = match iter.next() {
            Some(value) => u8::from_value(value, "method")?,
            None => u8::default(),
        };
        Ok(RdTeamPointsChangedEvent {
            points,
            team,
            method,
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
        let user_id = match iter.next() {
            Some(value) => u16::from_value(value, "user_id")?,
            None => u16::default(),
        };
        let victim_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "victim_ent_index")?,
            None => u32::default(),
        };
        let inflictor_ent_index = match iter.next() {
            Some(value) => u32::from_value(value, "inflictor_ent_index")?,
            None => u32::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        let weapon = match iter.next() {
            Some(value) => String::from_value(value, "weapon")?,
            None => String::default(),
        };
        let weapon_id = match iter.next() {
            Some(value) => u16::from_value(value, "weapon_id")?,
            None => u16::default(),
        };
        let damage_bits = match iter.next() {
            Some(value) => u32::from_value(value, "damage_bits")?,
            None => u32::default(),
        };
        let custom_kill = match iter.next() {
            Some(value) => u16::from_value(value, "custom_kill")?,
            None => u16::default(),
        };
        let weapon_log_class_name = match iter.next() {
            Some(value) => String::from_value(value, "weapon_log_class_name")?,
            None => String::default(),
        };
        Ok(RdRobotKilledEvent {
            user_id,
            victim_ent_index,
            inflictor_ent_index,
            attacker,
            weapon,
            weapon_id,
            damage_bits,
            custom_kill,
            weapon_log_class_name,
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
        let ent_index = match iter.next() {
            Some(value) => u16::from_value(value, "ent_index")?,
            None => u16::default(),
        };
        let impulse_x = match iter.next() {
            Some(value) => f32::from_value(value, "impulse_x")?,
            None => f32::default(),
        };
        let impulse_y = match iter.next() {
            Some(value) => f32::from_value(value, "impulse_y")?,
            None => f32::default(),
        };
        let impulse_z = match iter.next() {
            Some(value) => f32::from_value(value, "impulse_z")?,
            None => f32::default(),
        };
        Ok(RdRobotImpactEvent {
            ent_index,
            impulse_x,
            impulse_y,
            impulse_z,
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
        let time = match iter.next() {
            Some(value) => u16::from_value(value, "time")?,
            None => u16::default(),
        };
        Ok(TeamPlayPreRoundTimeLeftEvent { time })
    }
}
#[derive(Debug)]
pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteDeployEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ParachuteDeployEvent { index })
    }
}
#[derive(Debug)]
pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteHolsterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(ParachuteHolsterEvent { index })
    }
}
#[derive(Debug)]
pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl FromRawGameEvent for KillRefillsMeterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(KillRefillsMeterEvent { index })
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
        let winner = match iter.next() {
            Some(value) => u16::from_value(value, "winner")?,
            None => u16::default(),
        };
        let winner_rps = match iter.next() {
            Some(value) => u8::from_value(value, "winner_rps")?,
            None => u8::default(),
        };
        let loser = match iter.next() {
            Some(value) => u16::from_value(value, "loser")?,
            None => u16::default(),
        };
        let loser_rps = match iter.next() {
            Some(value) => u8::from_value(value, "loser_rps")?,
            None => u8::default(),
        };
        Ok(RpsTauntEventEvent {
            winner,
            winner_rps,
            loser,
            loser_rps,
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(CongaKillEvent { index })
    }
}
#[derive(Debug)]
pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl FromRawGameEvent for PlayerInitialSpawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(PlayerInitialSpawnEvent { index })
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let kills_rank = match iter.next() {
            Some(value) => u8::from_value(value, "kills_rank")?,
            None => u8::default(),
        };
        let score_rank = match iter.next() {
            Some(value) => u8::from_value(value, "score_rank")?,
            None => u8::default(),
        };
        let damage_rank = match iter.next() {
            Some(value) => u8::from_value(value, "damage_rank")?,
            None => u8::default(),
        };
        let healing_rank = match iter.next() {
            Some(value) => u8::from_value(value, "healing_rank")?,
            None => u8::default(),
        };
        let support_rank = match iter.next() {
            Some(value) => u8::from_value(value, "support_rank")?,
            None => u8::default(),
        };
        Ok(CompetitiveStatsUpdateEvent {
            index,
            kills_rank,
            score_rank,
            damage_rank,
            healing_rank,
            support_rank,
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
        let team = match iter.next() {
            Some(value) => u8::from_value(value, "team")?,
            None => u8::default(),
        };
        let kind = match iter.next() {
            Some(value) => u8::from_value(value, "kind")?,
            None => u8::default(),
        };
        Ok(MiniGameWinEvent { team, kind })
    }
}
#[derive(Debug)]
pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl FromRawGameEvent for SentryOnGoActiveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(SentryOnGoActiveEvent { index })
    }
}
#[derive(Debug)]
pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl FromRawGameEvent for DuckXpLevelUpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let level = match iter.next() {
            Some(value) => u16::from_value(value, "level")?,
            None => u16::default(),
        };
        Ok(DuckXpLevelUpEvent { level })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let method = match iter.next() {
            Some(value) => u16::from_value(value, "method")?,
            None => u16::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        Ok(RdPlayerScorePointsEvent {
            player,
            method,
            amount,
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(DemomanDetStickiesEvent { player })
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
        let quest_item_id_low = match iter.next() {
            Some(value) => u32::from_value(value, "quest_item_id_low")?,
            None => u32::default(),
        };
        let quest_item_id_hi = match iter.next() {
            Some(value) => u32::from_value(value, "quest_item_id_hi")?,
            None => u32::default(),
        };
        let quest_objective_id = match iter.next() {
            Some(value) => u32::from_value(value, "quest_objective_id")?,
            None => u32::default(),
        };
        let scorer_user_id = match iter.next() {
            Some(value) => u16::from_value(value, "scorer_user_id")?,
            None => u16::default(),
        };
        Ok(QuestObjectiveCompletedEvent {
            quest_item_id_low,
            quest_item_id_hi,
            quest_objective_id,
            scorer_user_id,
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let delta = match iter.next() {
            Some(value) => u16::from_value(value, "delta")?,
            None => u16::default(),
        };
        Ok(PlayerScoreChangedEvent { player, delta })
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
        let cp = match iter.next() {
            Some(value) => u8::from_value(value, "cp")?,
            None => u8::default(),
        };
        let killer = match iter.next() {
            Some(value) => u8::from_value(value, "killer")?,
            None => u8::default(),
        };
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        let assister = match iter.next() {
            Some(value) => u8::from_value(value, "assister")?,
            None => u8::default(),
        };
        Ok(KilledCappingPlayerEvent {
            cp,
            killer,
            victim,
            assister,
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
        let killer = match iter.next() {
            Some(value) => u8::from_value(value, "killer")?,
            None => u8::default(),
        };
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        Ok(EnvironmentalDeathEvent { killer, victim })
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
        let attacker = match iter.next() {
            Some(value) => u8::from_value(value, "attacker")?,
            None => u8::default(),
        };
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        let weapon_def_index = match iter.next() {
            Some(value) => u32::from_value(value, "weapon_def_index")?,
            None => u32::default(),
        };
        Ok(ProjectileDirectHitEvent {
            attacker,
            victim,
            weapon_def_index,
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
        let owner = match iter.next() {
            Some(value) => u16::from_value(value, "owner")?,
            None => u16::default(),
        };
        Ok(PassGetEvent { owner })
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
        let scorer = match iter.next() {
            Some(value) => u16::from_value(value, "scorer")?,
            None => u16::default(),
        };
        let assister = match iter.next() {
            Some(value) => u16::from_value(value, "assister")?,
            None => u16::default(),
        };
        let points = match iter.next() {
            Some(value) => u8::from_value(value, "points")?,
            None => u8::default(),
        };
        Ok(PassScoreEvent {
            scorer,
            assister,
            points,
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
        let owner = match iter.next() {
            Some(value) => u16::from_value(value, "owner")?,
            None => u16::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        Ok(PassFreeEvent { owner, attacker })
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
        let passer = match iter.next() {
            Some(value) => u16::from_value(value, "passer")?,
            None => u16::default(),
        };
        let catcher = match iter.next() {
            Some(value) => u16::from_value(value, "catcher")?,
            None => u16::default(),
        };
        let dist = match iter.next() {
            Some(value) => f32::from_value(value, "dist")?,
            None => f32::default(),
        };
        let duration = match iter.next() {
            Some(value) => f32::from_value(value, "duration")?,
            None => f32::default(),
        };
        Ok(PassPassCaughtEvent {
            passer,
            catcher,
            dist,
            duration,
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
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u16::from_value(value, "attacker")?,
            None => u16::default(),
        };
        Ok(PassBallStolenEvent { victim, attacker })
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
        let owner = match iter.next() {
            Some(value) => u16::from_value(value, "owner")?,
            None => u16::default(),
        };
        let blocker = match iter.next() {
            Some(value) => u16::from_value(value, "blocker")?,
            None => u16::default(),
        };
        Ok(PassBallBlockedEvent { owner, blocker })
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
        let preventor = match iter.next() {
            Some(value) => u16::from_value(value, "preventor")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        let condition = match iter.next() {
            Some(value) => u16::from_value(value, "condition")?,
            None => u16::default(),
        };
        Ok(DamagePreventedEvent {
            preventor,
            victim,
            amount,
            condition,
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
        let boss = match iter.next() {
            Some(value) => u16::from_value(value, "boss")?,
            None => u16::default(),
        };
        let killer = match iter.next() {
            Some(value) => u16::from_value(value, "killer")?,
            None => u16::default(),
        };
        Ok(HalloweenBossKilledEvent { boss, killer })
    }
}
#[derive(Debug)]
pub struct EscapedLootIslandEvent {
    pub player: u16,
}
impl FromRawGameEvent for EscapedLootIslandEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(EscapedLootIslandEvent { player })
    }
}
#[derive(Debug)]
pub struct TaggedPlayerAsItEvent {
    pub player: u16,
}
impl FromRawGameEvent for TaggedPlayerAsItEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(TaggedPlayerAsItEvent { player })
    }
}
#[derive(Debug)]
pub struct MerasmusStunnedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MerasmusStunnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MerasmusStunnedEvent { player })
    }
}
#[derive(Debug)]
pub struct MerasmusPropFoundEvent {
    pub player: u16,
}
impl FromRawGameEvent for MerasmusPropFoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(MerasmusPropFoundEvent { player })
    }
}
#[derive(Debug)]
pub struct HalloweenSkeletonKilledEvent {
    pub player: u16,
}
impl FromRawGameEvent for HalloweenSkeletonKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(HalloweenSkeletonKilledEvent { player })
    }
}
#[derive(Debug)]
pub struct EscapeHellEvent {
    pub player: u16,
}
impl FromRawGameEvent for EscapeHellEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(EscapeHellEvent { player })
    }
}
#[derive(Debug)]
pub struct CrossSpectralBridgeEvent {
    pub player: u16,
}
impl FromRawGameEvent for CrossSpectralBridgeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        Ok(CrossSpectralBridgeEvent { player })
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
        let player = match iter.next() {
            Some(value) => u16::from_value(value, "player")?,
            None => u16::default(),
        };
        let game = match iter.next() {
            Some(value) => u16::from_value(value, "game")?,
            None => u16::default(),
        };
        Ok(MiniGameWonEvent { player, game })
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
        let reviver = match iter.next() {
            Some(value) => u16::from_value(value, "reviver")?,
            None => u16::default(),
        };
        let ghost = match iter.next() {
            Some(value) => u16::from_value(value, "ghost")?,
            None => u16::default(),
        };
        Ok(RespawnGhostEvent { reviver, ghost })
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
        let killer = match iter.next() {
            Some(value) => u16::from_value(value, "killer")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        Ok(KillInHellEvent { killer, victim })
    }
}
#[derive(Debug)]
pub struct HalloweenDuckCollectedEvent {
    pub collector: u16,
}
impl FromRawGameEvent for HalloweenDuckCollectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let collector = match iter.next() {
            Some(value) => u16::from_value(value, "collector")?,
            None => u16::default(),
        };
        Ok(HalloweenDuckCollectedEvent { collector })
    }
}
#[derive(Debug)]
pub struct SpecialScoreEvent {
    pub player: u8,
}
impl FromRawGameEvent for SpecialScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        Ok(SpecialScoreEvent { player })
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
        let killer = match iter.next() {
            Some(value) => u8::from_value(value, "killer")?,
            None => u8::default(),
        };
        let victim = match iter.next() {
            Some(value) => u8::from_value(value, "victim")?,
            None => u8::default(),
        };
        Ok(TeamLeaderKilledEvent { killer, victim })
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
        let intended_target = match iter.next() {
            Some(value) => u8::from_value(value, "intended_target")?,
            None => u8::default(),
        };
        let collecting_player = match iter.next() {
            Some(value) => u8::from_value(value, "collecting_player")?,
            None => u8::default(),
        };
        let soul_count = match iter.next() {
            Some(value) => u8::from_value(value, "soul_count")?,
            None => u8::default(),
        };
        Ok(HalloweenSoulCollectedEvent {
            intended_target,
            collecting_player,
            soul_count,
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
        let spy = match iter.next() {
            Some(value) => u8::from_value(value, "spy")?,
            None => u8::default(),
        };
        let attacker = match iter.next() {
            Some(value) => u8::from_value(value, "attacker")?,
            None => u8::default(),
        };
        Ok(DeadRingerCheatDeathEvent { spy, attacker })
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
        let healer = match iter.next() {
            Some(value) => u8::from_value(value, "healer")?,
            None => u8::default(),
        };
        let target = match iter.next() {
            Some(value) => u8::from_value(value, "target")?,
            None => u8::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        Ok(CrossbowHealEvent {
            healer,
            target,
            amount,
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
        let mitigator = match iter.next() {
            Some(value) => u8::from_value(value, "mitigator")?,
            None => u8::default(),
        };
        let damaged = match iter.next() {
            Some(value) => u8::from_value(value, "damaged")?,
            None => u8::default(),
        };
        let amount = match iter.next() {
            Some(value) => u16::from_value(value, "amount")?,
            None => u16::default(),
        };
        let item_definition_index = match iter.next() {
            Some(value) => u16::from_value(value, "item_definition_index")?,
            None => u16::default(),
        };
        Ok(DamageMitigatedEvent {
            mitigator,
            damaged,
            amount,
            item_definition_index,
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
        let pusher = match iter.next() {
            Some(value) => u8::from_value(value, "pusher")?,
            None => u8::default(),
        };
        let distance = match iter.next() {
            Some(value) => u16::from_value(value, "distance")?,
            None => u16::default(),
        };
        Ok(PayloadPushedEvent { pusher, distance })
    }
}
#[derive(Debug)]
pub struct PlayerAbandonedMatchEvent {
    pub game_over: bool,
}
impl FromRawGameEvent for PlayerAbandonedMatchEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let game_over = match iter.next() {
            Some(value) => bool::from_value(value, "game_over")?,
            None => bool::default(),
        };
        Ok(PlayerAbandonedMatchEvent { game_over })
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
        let player = match iter.next() {
            Some(value) => u8::from_value(value, "player")?,
            None => u8::default(),
        };
        let panel = match iter.next() {
            Some(value) => u8::from_value(value, "panel")?,
            None => u8::default(),
        };
        let line = match iter.next() {
            Some(value) => u8::from_value(value, "line")?,
            None => u8::default(),
        };
        let x = match iter.next() {
            Some(value) => f32::from_value(value, "x")?,
            None => f32::default(),
        };
        let y = match iter.next() {
            Some(value) => f32::from_value(value, "y")?,
            None => f32::default(),
        };
        Ok(ClDrawlineEvent {
            player,
            panel,
            line,
            x,
            y,
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
        let time = match iter.next() {
            Some(value) => u8::from_value(value, "time")?,
            None => u8::default(),
        };
        Ok(RestartTimerTimeEvent { time })
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
        let delay = match iter.next() {
            Some(value) => f32::from_value(value, "delay")?,
            None => f32::default(),
        };
        Ok(DsScreenshotEvent { delay })
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
        let success = match iter.next() {
            Some(value) => bool::from_value(value, "success")?,
            None => bool::default(),
        };
        Ok(RematchVotePeriodOverEvent { success })
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
        let map_index = match iter.next() {
            Some(value) => u8::from_value(value, "map_index")?,
            None => u8::default(),
        };
        let vote = match iter.next() {
            Some(value) => u8::from_value(value, "vote")?,
            None => u8::default(),
        };
        Ok(PlayerNextMapVoteChangeEvent { map_index, vote })
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
        let kind = match iter.next() {
            Some(value) => u8::from_value(value, "kind")?,
            None => u8::default(),
        };
        let definition_index = match iter.next() {
            Some(value) => u32::from_value(value, "definition_index")?,
            None => u32::default(),
        };
        let created = match iter.next() {
            Some(value) => bool::from_value(value, "created")?,
            None => bool::default(),
        };
        let deleted = match iter.next() {
            Some(value) => bool::from_value(value, "deleted")?,
            None => bool::default(),
        };
        let erase_history = match iter.next() {
            Some(value) => bool::from_value(value, "erase_history")?,
            None => bool::default(),
        };
        Ok(ProtoDefChangedEvent {
            kind,
            definition_index,
            created,
            deleted,
            erase_history,
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
        let dominator = match iter.next() {
            Some(value) => u16::from_value(value, "dominator")?,
            None => u16::default(),
        };
        let dominated = match iter.next() {
            Some(value) => u16::from_value(value, "dominated")?,
            None => u16::default(),
        };
        let dominations = match iter.next() {
            Some(value) => u16::from_value(value, "dominations")?,
            None => u16::default(),
        };
        Ok(PlayerDominationEvent {
            dominator,
            dominated,
            dominations,
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
        let pusher = match iter.next() {
            Some(value) => u16::from_value(value, "pusher")?,
            None => u16::default(),
        };
        let pushed = match iter.next() {
            Some(value) => u16::from_value(value, "pushed")?,
            None => u16::default(),
        };
        Ok(PlayerRocketPackPushedEvent { pusher, pushed })
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
        let request = match iter.next() {
            Some(value) => u32::from_value(value, "request")?,
            None => u32::default(),
        };
        let msg = match iter.next() {
            Some(value) => String::from_value(value, "msg")?,
            None => String::default(),
        };
        Ok(QuestRequestEvent { request, msg })
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
        let request = match iter.next() {
            Some(value) => u32::from_value(value, "request")?,
            None => u32::default(),
        };
        let success = match iter.next() {
            Some(value) => bool::from_value(value, "success")?,
            None => bool::default(),
        };
        let msg = match iter.next() {
            Some(value) => String::from_value(value, "msg")?,
            None => String::default(),
        };
        Ok(QuestResponseEvent {
            request,
            success,
            msg,
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
        let owner = match iter.next() {
            Some(value) => u16::from_value(value, "owner")?,
            None => u16::default(),
        };
        let scorer = match iter.next() {
            Some(value) => u16::from_value(value, "scorer")?,
            None => u16::default(),
        };
        let kind = match iter.next() {
            Some(value) => u8::from_value(value, "kind")?,
            None => u8::default(),
        };
        let completed = match iter.next() {
            Some(value) => bool::from_value(value, "completed")?,
            None => bool::default(),
        };
        let quest_definition_index = match iter.next() {
            Some(value) => u32::from_value(value, "quest_definition_index")?,
            None => u32::default(),
        };
        Ok(QuestProgressEvent {
            owner,
            scorer,
            kind,
            completed,
            quest_definition_index,
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
        let attacker = match iter.next() {
            Some(value) => u8::from_value(value, "attacker")?,
            None => u8::default(),
        };
        let weapon_def_index = match iter.next() {
            Some(value) => u32::from_value(value, "weapon_def_index")?,
            None => u32::default(),
        };
        let num_hit = match iter.next() {
            Some(value) => u8::from_value(value, "num_hit")?,
            None => u8::default(),
        };
        let num_direct_hit = match iter.next() {
            Some(value) => u8::from_value(value, "num_direct_hit")?,
            None => u8::default(),
        };
        Ok(ProjectileRemovedEvent {
            attacker,
            weapon_def_index,
            num_hit,
            num_direct_hit,
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
        let igniter = match iter.next() {
            Some(value) => u16::from_value(value, "igniter")?,
            None => u16::default(),
        };
        let douser = match iter.next() {
            Some(value) => u16::from_value(value, "douser")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        Ok(GasDousedPlayerIgnitedEvent {
            igniter,
            douser,
            victim,
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
        let state = match iter.next() {
            Some(value) => u16::from_value(value, "state")?,
            None => u16::default(),
        };
        Ok(QuestTurnInStateEvent { state })
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
        let blocker = match iter.next() {
            Some(value) => u16::from_value(value, "blocker")?,
            None => u16::default(),
        };
        let victim = match iter.next() {
            Some(value) => u16::from_value(value, "victim")?,
            None => u16::default(),
        };
        Ok(CapperKilledEvent { blocker, victim })
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
        let clients = match iter.next() {
            Some(value) => u32::from_value(value, "clients")?,
            None => u32::default(),
        };
        let slots = match iter.next() {
            Some(value) => u32::from_value(value, "slots")?,
            None => u32::default(),
        };
        let proxies = match iter.next() {
            Some(value) => u16::from_value(value, "proxies")?,
            None => u16::default(),
        };
        let master = match iter.next() {
            Some(value) => String::from_value(value, "master")?,
            None => String::default(),
        };
        Ok(HLTVStatusEvent {
            clients,
            slots,
            proxies,
            master,
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        Ok(HLTVCameramanEvent { index })
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
        let index = match iter.next() {
            Some(value) => u8::from_value(value, "index")?,
            None => u8::default(),
        };
        let rank = match iter.next() {
            Some(value) => f32::from_value(value, "rank")?,
            None => f32::default(),
        };
        let target = match iter.next() {
            Some(value) => u16::from_value(value, "target")?,
            None => u16::default(),
        };
        Ok(HLTVRankCameraEvent {
            index,
            rank,
            target,
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
        let index = match iter.next() {
            Some(value) => u16::from_value(value, "index")?,
            None => u16::default(),
        };
        let rank = match iter.next() {
            Some(value) => f32::from_value(value, "rank")?,
            None => f32::default(),
        };
        let target = match iter.next() {
            Some(value) => u16::from_value(value, "target")?,
            None => u16::default(),
        };
        Ok(HLTVRankEntityEvent {
            index,
            rank,
            target,
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
        let pos_x = match iter.next() {
            Some(value) => u32::from_value(value, "pos_x")?,
            None => u32::default(),
        };
        let pos_y = match iter.next() {
            Some(value) => u32::from_value(value, "pos_y")?,
            None => u32::default(),
        };
        let pos_z = match iter.next() {
            Some(value) => u32::from_value(value, "pos_z")?,
            None => u32::default(),
        };
        let theta = match iter.next() {
            Some(value) => u16::from_value(value, "theta")?,
            None => u16::default(),
        };
        let phi = match iter.next() {
            Some(value) => u16::from_value(value, "phi")?,
            None => u16::default(),
        };
        let offset = match iter.next() {
            Some(value) => u16::from_value(value, "offset")?,
            None => u16::default(),
        };
        let fov = match iter.next() {
            Some(value) => f32::from_value(value, "fov")?,
            None => f32::default(),
        };
        let target = match iter.next() {
            Some(value) => u16::from_value(value, "target")?,
            None => u16::default(),
        };
        Ok(HLTVFixedEvent {
            pos_x,
            pos_y,
            pos_z,
            theta,
            phi,
            offset,
            fov,
            target,
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
        let target_1 = match iter.next() {
            Some(value) => u16::from_value(value, "target_1")?,
            None => u16::default(),
        };
        let target_2 = match iter.next() {
            Some(value) => u16::from_value(value, "target_2")?,
            None => u16::default(),
        };
        let distance = match iter.next() {
            Some(value) => u16::from_value(value, "distance")?,
            None => u16::default(),
        };
        let theta = match iter.next() {
            Some(value) => u16::from_value(value, "theta")?,
            None => u16::default(),
        };
        let phi = match iter.next() {
            Some(value) => u16::from_value(value, "phi")?,
            None => u16::default(),
        };
        let inertia = match iter.next() {
            Some(value) => u8::from_value(value, "inertia")?,
            None => u8::default(),
        };
        let in_eye = match iter.next() {
            Some(value) => u8::from_value(value, "in_eye")?,
            None => u8::default(),
        };
        Ok(HLTVChaseEvent {
            target_1,
            target_2,
            distance,
            theta,
            phi,
            inertia,
            in_eye,
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
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(HLTVMessageEvent { text })
    }
}
#[derive(Debug)]
pub struct HLTVTitleEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVTitleEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(HLTVTitleEvent { text })
    }
}
#[derive(Debug)]
pub struct HLTVChatEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mut iter = values.into_iter();
        let text = match iter.next() {
            Some(value) => String::from_value(value, "text")?,
            None => String::default(),
        };
        Ok(HLTVChatEvent { text })
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
        let sn = match iter.next() {
            Some(value) => String::from_value(value, "sn")?,
            None => String::default(),
        };
        let di = match iter.next() {
            Some(value) => u8::from_value(value, "di")?,
            None => u8::default(),
        };
        let cb = match iter.next() {
            Some(value) => u32::from_value(value, "cb")?,
            None => u32::default(),
        };
        let st = match iter.next() {
            Some(value) => u32::from_value(value, "st")?,
            None => u32::default(),
        };
        Ok(ReplaySessionInfoEvent { sn, di, cb, st })
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
        let error = match iter.next() {
            Some(value) => String::from_value(value, "error")?,
            None => String::default(),
        };
        Ok(ReplayServerErrorEvent { error })
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
