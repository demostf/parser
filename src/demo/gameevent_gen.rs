use std::collections::HashMap;
use crate::{Result, ParseError};
use super::gamevent::{FromGameEventValue, GameEventValue, FromRawGameEvent, RawGameEvent};
use bitstream_reader::BitRead;

// auto generated, nobody in their right mind would write this manually

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
        let hostname: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("hostname".to_string()))?;
            String::from_value(value.clone(), "hostname")?
        };
        let address: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        let game: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("game".to_string()))?;
            String::from_value(value.clone(), "game")?
        };
        let map_name: String = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("map_name".to_string()))?;
            String::from_value(value.clone(), "map_name")?
        };
        let max_players: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("max_players".to_string()))?;
            u32::from_value(value.clone(), "max_players")?
        };
        let os: String = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("os".to_string()))?;
            String::from_value(value.clone(), "os")?
        };
        let dedicated: bool = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("dedicated".to_string()))?;
            bool::from_value(value.clone(), "dedicated")?
        };
        let password: bool = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("password".to_string()))?;
            bool::from_value(value.clone(), "password")?
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
            password
        })
    }
}

#[derive(Debug)]
pub struct ServerShutdownEvent {
    pub reason: String,
}
impl FromRawGameEvent for ServerShutdownEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(ServerShutdownEvent {
            reason
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
        let cvar_name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cvar_name".to_string()))?;
            String::from_value(value.clone(), "cvar_name")?
        };
        let cvar_value: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cvar_value".to_string()))?;
            String::from_value(value.clone(), "cvar_value")?
        };
        Ok(ServerCvarEvent {
            cvar_name,
            cvar_value
        })
    }
}

#[derive(Debug)]
pub struct ServerMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for ServerMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(ServerMessageEvent {
            text
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
        let name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let user_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let network_id: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let ip: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            String::from_value(value.clone(), "ip")?
        };
        let duration: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("duration".to_string()))?;
            String::from_value(value.clone(), "duration")?
        };
        let by: String = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("by".to_string()))?;
            String::from_value(value.clone(), "by")?
        };
        let kicked: bool = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("kicked".to_string()))?;
            bool::from_value(value.clone(), "kicked")?
        };
        Ok(ServerAddBanEvent {
            name,
            user_id,
            network_id,
            ip,
            duration,
            by,
            kicked
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
        let network_id: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let ip: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            String::from_value(value.clone(), "ip")?
        };
        let by: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("by".to_string()))?;
            String::from_value(value.clone(), "by")?
        };
        Ok(ServerRemoveBanEvent {
            network_id,
            ip,
            by
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
        let name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let user_id: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let network_id: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let address: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let bot: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerConnectEvent {
            name,
            index,
            user_id,
            network_id,
            address,
            bot
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
        let name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let user_id: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let network_id: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let bot: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerConnectClientEvent {
            name,
            index,
            user_id,
            network_id,
            bot
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
        let name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let user_id: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let network_id: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let bot: bool = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("bot".to_string()))?;
            bool::from_value(value.clone(), "bot")?
        };
        Ok(PlayerInfoEvent {
            name,
            index,
            user_id,
            network_id,
            bot
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let reason: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        let name: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let network_id: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("network_id".to_string()))?;
            String::from_value(value.clone(), "network_id")?
        };
        let bot: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerDisconnectEvent {
            user_id,
            reason,
            name,
            network_id,
            bot
        })
    }
}

#[derive(Debug)]
pub struct PlayerActivateEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerActivateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PlayerActivateEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let text: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(PlayerSayEvent {
            user_id,
            text
        })
    }
}

#[derive(Debug)]
pub struct ClientDisconnectEvent {
    pub message: String,
}
impl FromRawGameEvent for ClientDisconnectEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let message: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("message".to_string()))?;
            String::from_value(value.clone(), "message")?
        };
        Ok(ClientDisconnectEvent {
            message
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
        let address: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        let source: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("source".to_string()))?;
            String::from_value(value.clone(), "source")?
        };
        Ok(ClientBeginConnectEvent {
            address,
            ip,
            port,
            source
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
        let address: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        Ok(ClientConnectedEvent {
            address,
            ip,
            port
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
        let address: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        Ok(ClientFullConnectEvent {
            address,
            ip,
            port
        })
    }
}

#[derive(Debug)]
pub struct HostQuitEvent {

}
impl FromRawGameEvent for HostQuitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(HostQuitEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamInfoEvent {
    pub team_id: u8,
    pub team_name: String,
}
impl FromRawGameEvent for TeamInfoEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let team_id: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team_id".to_string()))?;
            u8::from_value(value.clone(), "team_id")?
        };
        let team_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team_name".to_string()))?;
            String::from_value(value.clone(), "team_name")?
        };
        Ok(TeamInfoEvent {
            team_id,
            team_name
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
        let team_id: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team_id".to_string()))?;
            u8::from_value(value.clone(), "team_id")?
        };
        let score: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(TeamScoreEvent {
            team_id,
            score
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let sound: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("sound".to_string()))?;
            String::from_value(value.clone(), "sound")?
        };
        let additional_flags: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("additional_flags".to_string()))?;
            u16::from_value(value.clone(), "additional_flags")?
        };
        Ok(TeamPlayBroadcastAudioEvent {
            team,
            sound,
            additional_flags
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let old_team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("old_team".to_string()))?;
            u8::from_value(value.clone(), "old_team")?
        };
        let disconnect: bool = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("disconnect".to_string()))?;
            bool::from_value(value.clone(), "disconnect")?
        };
        let auto_team: bool = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("auto_team".to_string()))?;
            bool::from_value(value.clone(), "auto_team")?
        };
        let silent: bool = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("silent".to_string()))?;
            bool::from_value(value.clone(), "silent")?
        };
        let name: String = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        Ok(PlayerTeamEvent {
            user_id,
            team,
            old_team,
            disconnect,
            auto_team,
            silent,
            name
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let class: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("class".to_string()))?;
            String::from_value(value.clone(), "class")?
        };
        Ok(PlayerClassEvent {
            user_id,
            class
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
}
impl FromRawGameEvent for PlayerDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let victim_ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u32::from_value(value.clone(), "victim_ent_index")?
        };
        let inflictor_ent_index: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("inflictor_ent_index".to_string()))?;
            u32::from_value(value.clone(), "inflictor_ent_index")?
        };
        let attacker: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_bits: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        let custom_kill: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("custom_kill".to_string()))?;
            u16::from_value(value.clone(), "custom_kill")?
        };
        let assister: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_log_class_name: String = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("weapon_log_class_name".to_string()))?;
            String::from_value(value.clone(), "weapon_log_class_name")?
        };
        let stun_flags: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let player_penetrate_count: u16 = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("player_penetrate_count".to_string()))?;
            u16::from_value(value.clone(), "player_penetrate_count")?
        };
        let assister_fallback: String = {
            let value = values.get(14).ok_or_else(|| ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        let kill_streak_total: u16 = {
            let value = values.get(15).ok_or_else(|| ParseError::UnknownGameEvent("kill_streak_total".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_total")?
        };
        let kill_streak_wep: u16 = {
            let value = values.get(16).ok_or_else(|| ParseError::UnknownGameEvent("kill_streak_wep".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_wep")?
        };
        let kill_streak_assist: u16 = {
            let value = values.get(17).ok_or_else(|| ParseError::UnknownGameEvent("kill_streak_assist".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_assist")?
        };
        let kill_streak_victim: u16 = {
            let value = values.get(18).ok_or_else(|| ParseError::UnknownGameEvent("kill_streak_victim".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_victim")?
        };
        let ducks_streaked: u16 = {
            let value = values.get(19).ok_or_else(|| ParseError::UnknownGameEvent("ducks_streaked".to_string()))?;
            u16::from_value(value.clone(), "ducks_streaked")?
        };
        let duck_streak_total: u16 = {
            let value = values.get(20).ok_or_else(|| ParseError::UnknownGameEvent("duck_streak_total".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_total")?
        };
        let duck_streak_assist: u16 = {
            let value = values.get(21).ok_or_else(|| ParseError::UnknownGameEvent("duck_streak_assist".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_assist")?
        };
        let duck_streak_victim: u16 = {
            let value = values.get(22).ok_or_else(|| ParseError::UnknownGameEvent("duck_streak_victim".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_victim")?
        };
        let rocket_jump: bool = {
            let value = values.get(23).ok_or_else(|| ParseError::UnknownGameEvent("rocket_jump".to_string()))?;
            bool::from_value(value.clone(), "rocket_jump")?
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
            rocket_jump
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let health: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("health".to_string()))?;
            u16::from_value(value.clone(), "health")?
        };
        let attacker: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let damage_amount: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("damage_amount".to_string()))?;
            u16::from_value(value.clone(), "damage_amount")?
        };
        let custom: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("custom".to_string()))?;
            u16::from_value(value.clone(), "custom")?
        };
        let show_disguised_crit: bool = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("show_disguised_crit".to_string()))?;
            bool::from_value(value.clone(), "show_disguised_crit")?
        };
        let crit: bool = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("crit".to_string()))?;
            bool::from_value(value.clone(), "crit")?
        };
        let mini_crit: bool = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("mini_crit".to_string()))?;
            bool::from_value(value.clone(), "mini_crit")?
        };
        let all_see_crit: bool = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("all_see_crit".to_string()))?;
            bool::from_value(value.clone(), "all_see_crit")?
        };
        let weapon_id: u16 = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let bonus_effect: u8 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("bonus_effect".to_string()))?;
            u8::from_value(value.clone(), "bonus_effect")?
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
            bonus_effect
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
        let team_only: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team_only".to_string()))?;
            bool::from_value(value.clone(), "team_only")?
        };
        let user_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let text: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(PlayerChatEvent {
            team_only,
            user_id,
            text
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let kills: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("kills".to_string()))?;
            u16::from_value(value.clone(), "kills")?
        };
        let deaths: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("deaths".to_string()))?;
            u16::from_value(value.clone(), "deaths")?
        };
        let score: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(PlayerScoreEvent {
            user_id,
            kills,
            deaths,
            score
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let team: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u16::from_value(value.clone(), "team")?
        };
        let class: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        Ok(PlayerSpawnEvent {
            user_id,
            team,
            class
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let weapon: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            u8::from_value(value.clone(), "weapon")?
        };
        let mode: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("mode".to_string()))?;
            u8::from_value(value.clone(), "mode")?
        };
        Ok(PlayerShootEvent {
            user_id,
            weapon,
            mode
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let entity: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("entity".to_string()))?;
            u16::from_value(value.clone(), "entity")?
        };
        Ok(PlayerUseEvent {
            user_id,
            entity
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let old_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("old_name".to_string()))?;
            String::from_value(value.clone(), "old_name")?
        };
        let new_name: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("new_name".to_string()))?;
            String::from_value(value.clone(), "new_name")?
        };
        Ok(PlayerChangeNameEvent {
            user_id,
            old_name,
            new_name
        })
    }
}

#[derive(Debug)]
pub struct PlayerHintMessageEvent {
    pub hint_message: String,
}
impl FromRawGameEvent for PlayerHintMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let hint_message: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("hint_message".to_string()))?;
            String::from_value(value.clone(), "hint_message")?
        };
        Ok(PlayerHintMessageEvent {
            hint_message
        })
    }
}

#[derive(Debug)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for BasePlayerTeleportedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        Ok(BasePlayerTeleportedEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct GameInitEvent {

}
impl FromRawGameEvent for GameInitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(GameInitEvent {

        })
    }
}

#[derive(Debug)]
pub struct GameNewMapEvent {
    pub map_name: String,
}
impl FromRawGameEvent for GameNewMapEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let map_name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("map_name".to_string()))?;
            String::from_value(value.clone(), "map_name")?
        };
        Ok(GameNewMapEvent {
            map_name
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
        let rounds_limit: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("rounds_limit".to_string()))?;
            u32::from_value(value.clone(), "rounds_limit")?
        };
        let time_limit: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("time_limit".to_string()))?;
            u32::from_value(value.clone(), "time_limit")?
        };
        let frag_limit: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("frag_limit".to_string()))?;
            u32::from_value(value.clone(), "frag_limit")?
        };
        let objective: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("objective".to_string()))?;
            String::from_value(value.clone(), "objective")?
        };
        Ok(GameStartEvent {
            rounds_limit,
            time_limit,
            frag_limit,
            objective
        })
    }
}

#[derive(Debug)]
pub struct GameEndEvent {
    pub winner: u8,
}
impl FromRawGameEvent for GameEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let winner: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("winner".to_string()))?;
            u8::from_value(value.clone(), "winner")?
        };
        Ok(GameEndEvent {
            winner
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
        let time_limit: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("time_limit".to_string()))?;
            u32::from_value(value.clone(), "time_limit")?
        };
        let frag_limit: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("frag_limit".to_string()))?;
            u32::from_value(value.clone(), "frag_limit")?
        };
        let objective: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("objective".to_string()))?;
            String::from_value(value.clone(), "objective")?
        };
        Ok(RoundStartEvent {
            time_limit,
            frag_limit,
            objective
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
        let winner: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("winner".to_string()))?;
            u8::from_value(value.clone(), "winner")?
        };
        let reason: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            u8::from_value(value.clone(), "reason")?
        };
        let message: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("message".to_string()))?;
            String::from_value(value.clone(), "message")?
        };
        Ok(RoundEndEvent {
            winner,
            reason,
            message
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
        let target: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u8::from_value(value.clone(), "target")?
        };
        let text: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(GameMessageEvent {
            target,
            text
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
        let ent_index: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        let user_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let material: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("material".to_string()))?;
            u8::from_value(value.clone(), "material")?
        };
        Ok(BreakBreakableEvent {
            ent_index,
            user_id,
            material
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
        let ent_index: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        let user_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(BreakPropEvent {
            ent_index,
            user_id
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
        let ent_index_killed: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index_killed".to_string()))?;
            u32::from_value(value.clone(), "ent_index_killed")?
        };
        let ent_index_attacker: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ent_index_attacker".to_string()))?;
            u32::from_value(value.clone(), "ent_index_attacker")?
        };
        let ent_index_inflictor: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("ent_index_inflictor".to_string()))?;
            u32::from_value(value.clone(), "ent_index_inflictor")?
        };
        let damage_bits: u32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        Ok(EntityKilledEvent {
            ent_index_killed,
            ent_index_attacker,
            ent_index_inflictor,
            damage_bits
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
        let num_advanced: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("num_advanced".to_string()))?;
            u16::from_value(value.clone(), "num_advanced")?
        };
        let num_bronze: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("num_bronze".to_string()))?;
            u16::from_value(value.clone(), "num_bronze")?
        };
        let num_silver: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("num_silver".to_string()))?;
            u16::from_value(value.clone(), "num_silver")?
        };
        let num_gold: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("num_gold".to_string()))?;
            u16::from_value(value.clone(), "num_gold")?
        };
        Ok(BonusUpdatedEvent {
            num_advanced,
            num_bronze,
            num_silver,
            num_gold
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
        let achievement_name: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("achievement_name".to_string()))?;
            String::from_value(value.clone(), "achievement_name")?
        };
        let cur_val: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cur_val".to_string()))?;
            u16::from_value(value.clone(), "cur_val")?
        };
        let max_val: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("max_val".to_string()))?;
            u16::from_value(value.clone(), "max_val")?
        };
        Ok(AchievementEventEvent {
            achievement_name,
            cur_val,
            max_val
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
        let achievement_id: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("achievement_id".to_string()))?;
            u32::from_value(value.clone(), "achievement_id")?
        };
        let cur_val: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cur_val".to_string()))?;
            u16::from_value(value.clone(), "cur_val")?
        };
        let max_val: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("max_val".to_string()))?;
            u16::from_value(value.clone(), "max_val")?
        };
        Ok(AchievementIncrementEvent {
            achievement_id,
            cur_val,
            max_val
        })
    }
}

#[derive(Debug)]
pub struct PhysgunPickupEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for PhysgunPickupEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        Ok(PhysgunPickupEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for FlareIgniteNpcEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        Ok(FlareIgniteNpcEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct HelicopterGrenadePuntMissEvent {

}
impl FromRawGameEvent for HelicopterGrenadePuntMissEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(HelicopterGrenadePuntMissEvent {

        })
    }
}

#[derive(Debug)]
pub struct UserDataDownloadedEvent {

}
impl FromRawGameEvent for UserDataDownloadedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(UserDataDownloadedEvent {

        })
    }
}

#[derive(Debug)]
pub struct RagdollDissolvedEvent {
    pub ent_index: u32,
}
impl FromRawGameEvent for RagdollDissolvedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        Ok(RagdollDissolvedEvent {
            ent_index
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
        let old_mode: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("old_mode".to_string()))?;
            u16::from_value(value.clone(), "old_mode")?
        };
        let new_mode: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("new_mode".to_string()))?;
            u16::from_value(value.clone(), "new_mode")?
        };
        let obs_target: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("obs_target".to_string()))?;
            u16::from_value(value.clone(), "obs_target")?
        };
        Ok(HLTVChangedModeEvent {
            old_mode,
            new_mode,
            obs_target
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
        let mode: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("mode".to_string()))?;
            u16::from_value(value.clone(), "mode")?
        };
        let old_target: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("old_target".to_string()))?;
            u16::from_value(value.clone(), "old_target")?
        };
        let obs_target: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("obs_target".to_string()))?;
            u16::from_value(value.clone(), "obs_target")?
        };
        Ok(HLTVChangedTargetEvent {
            mode,
            old_target,
            obs_target
        })
    }
}

#[derive(Debug)]
pub struct VoteEndedEvent {

}
impl FromRawGameEvent for VoteEndedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(VoteEndedEvent {

        })
    }
}

#[derive(Debug)]
pub struct VoteStartedEvent {
    pub issue: String,
    pub param1: String,
    pub team: u8,
    pub initiator: u32,
}
impl FromRawGameEvent for VoteStartedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let issue: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("issue".to_string()))?;
            String::from_value(value.clone(), "issue")?
        };
        let param1: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("param1".to_string()))?;
            String::from_value(value.clone(), "param1")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let initiator: u32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("initiator".to_string()))?;
            u32::from_value(value.clone(), "initiator")?
        };
        Ok(VoteStartedEvent {
            issue,
            param1,
            team,
            initiator
        })
    }
}

#[derive(Debug)]
pub struct VoteChangedEvent {
    pub vote_option1: u8,
    pub vote_option2: u8,
    pub vote_option3: u8,
    pub vote_option4: u8,
    pub vote_option5: u8,
    pub potential_votes: u8,
}
impl FromRawGameEvent for VoteChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let vote_option1: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("vote_option1".to_string()))?;
            u8::from_value(value.clone(), "vote_option1")?
        };
        let vote_option2: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("vote_option2".to_string()))?;
            u8::from_value(value.clone(), "vote_option2")?
        };
        let vote_option3: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("vote_option3".to_string()))?;
            u8::from_value(value.clone(), "vote_option3")?
        };
        let vote_option4: u8 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("vote_option4".to_string()))?;
            u8::from_value(value.clone(), "vote_option4")?
        };
        let vote_option5: u8 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("vote_option5".to_string()))?;
            u8::from_value(value.clone(), "vote_option5")?
        };
        let potential_votes: u8 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("potential_votes".to_string()))?;
            u8::from_value(value.clone(), "potential_votes")?
        };
        Ok(VoteChangedEvent {
            vote_option1,
            vote_option2,
            vote_option3,
            vote_option4,
            vote_option5,
            potential_votes
        })
    }
}

#[derive(Debug)]
pub struct VotePassedEvent {
    pub details: String,
    pub param1: String,
    pub team: u8,
}
impl FromRawGameEvent for VotePassedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let details: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("details".to_string()))?;
            String::from_value(value.clone(), "details")?
        };
        let param1: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("param1".to_string()))?;
            String::from_value(value.clone(), "param1")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(VotePassedEvent {
            details,
            param1,
            team
        })
    }
}

#[derive(Debug)]
pub struct VoteFailedEvent {
    pub team: u8,
}
impl FromRawGameEvent for VoteFailedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(VoteFailedEvent {
            team
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
        let vote_option: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("vote_option".to_string()))?;
            u8::from_value(value.clone(), "vote_option")?
        };
        let team: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u16::from_value(value.clone(), "team")?
        };
        let entity_id: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("entity_id".to_string()))?;
            u32::from_value(value.clone(), "entity_id")?
        };
        Ok(VoteCastEvent {
            vote_option,
            team,
            entity_id
        })
    }
}

#[derive(Debug)]
pub struct VoteOptionsEvent {
    pub count: u8,
    pub option1: String,
    pub option2: String,
    pub option3: String,
    pub option4: String,
    pub option5: String,
}
impl FromRawGameEvent for VoteOptionsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let count: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("count".to_string()))?;
            u8::from_value(value.clone(), "count")?
        };
        let option1: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("option1".to_string()))?;
            String::from_value(value.clone(), "option1")?
        };
        let option2: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("option2".to_string()))?;
            String::from_value(value.clone(), "option2")?
        };
        let option3: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("option3".to_string()))?;
            String::from_value(value.clone(), "option3")?
        };
        let option4: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("option4".to_string()))?;
            String::from_value(value.clone(), "option4")?
        };
        let option5: String = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("option5".to_string()))?;
            String::from_value(value.clone(), "option5")?
        };
        Ok(VoteOptionsEvent {
            count,
            option1,
            option2,
            option3,
            option4,
            option5
        })
    }
}

#[derive(Debug)]
pub struct ReplaySavedEvent {

}
impl FromRawGameEvent for ReplaySavedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ReplaySavedEvent {

        })
    }
}

#[derive(Debug)]
pub struct EnteredPerformanceModeEvent {

}
impl FromRawGameEvent for EnteredPerformanceModeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(EnteredPerformanceModeEvent {

        })
    }
}

#[derive(Debug)]
pub struct BrowseReplaysEvent {

}
impl FromRawGameEvent for BrowseReplaysEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(BrowseReplaysEvent {

        })
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
        let views: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("views".to_string()))?;
            u32::from_value(value.clone(), "views")?
        };
        let likes: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("likes".to_string()))?;
            u32::from_value(value.clone(), "likes")?
        };
        let favorited: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("favorited".to_string()))?;
            u32::from_value(value.clone(), "favorited")?
        };
        Ok(ReplayYoutubeStatsEvent {
            views,
            likes,
            favorited
        })
    }
}

#[derive(Debug)]
pub struct InventoryUpdatedEvent {

}
impl FromRawGameEvent for InventoryUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(InventoryUpdatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct CartUpdatedEvent {

}
impl FromRawGameEvent for CartUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(CartUpdatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct StorePriceSheetUpdatedEvent {

}
impl FromRawGameEvent for StorePriceSheetUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(StorePriceSheetUpdatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct GcConnectedEvent {

}
impl FromRawGameEvent for GcConnectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(GcConnectedEvent {

        })
    }
}

#[derive(Debug)]
pub struct ItemSchemaInitializedEvent {

}
impl FromRawGameEvent for ItemSchemaInitializedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ItemSchemaInitializedEvent {

        })
    }
}

#[derive(Debug)]
pub struct IntroFinishEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroFinishEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(IntroFinishEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroNextCameraEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(IntroNextCameraEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MmLobbyChatEvent {
    pub steam_id: String,
    pub text: String,
    pub kind: u16,
}
impl FromRawGameEvent for MmLobbyChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let steam_id: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("steam_id".to_string()))?;
            String::from_value(value.clone(), "steam_id")?
        };
        let text: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        let kind: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("kind".to_string()))?;
            u16::from_value(value.clone(), "kind")?
        };
        Ok(MmLobbyChatEvent {
            steam_id,
            text,
            kind
        })
    }
}

#[derive(Debug)]
pub struct MmLobbyMemberJoinEvent {
    pub steam_id: String,
}
impl FromRawGameEvent for MmLobbyMemberJoinEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let steam_id: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("steam_id".to_string()))?;
            String::from_value(value.clone(), "steam_id")?
        };
        Ok(MmLobbyMemberJoinEvent {
            steam_id
        })
    }
}

#[derive(Debug)]
pub struct MmLobbyMemberLeaveEvent {
    pub steam_id: String,
    pub flags: u32,
}
impl FromRawGameEvent for MmLobbyMemberLeaveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let steam_id: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("steam_id".to_string()))?;
            String::from_value(value.clone(), "steam_id")?
        };
        let flags: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("flags".to_string()))?;
            u32::from_value(value.clone(), "flags")?
        };
        Ok(MmLobbyMemberLeaveEvent {
            steam_id,
            flags
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let class: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        Ok(PlayerChangeClassEvent {
            user_id,
            class
        })
    }
}

#[derive(Debug)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl FromRawGameEvent for TfMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let seconds: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("seconds".to_string()))?;
            u32::from_value(value.clone(), "seconds")?
        };
        Ok(TfMapTimeRemainingEvent {
            seconds
        })
    }
}

#[derive(Debug)]
pub struct TfGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TfGameOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(TfGameOverEvent {
            reason
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
        let capping_team: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("capping_team".to_string()))?;
            u16::from_value(value.clone(), "capping_team")?
        };
        let capping_team_score: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("capping_team_score".to_string()))?;
            u16::from_value(value.clone(), "capping_team_score")?
        };
        Ok(CtfFlagCapturedEvent {
            capping_team,
            capping_team_score
        })
    }
}

#[derive(Debug)]
pub struct ControlPointInitializedEvent {

}
impl FromRawGameEvent for ControlPointInitializedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ControlPointInitializedEvent {

        })
    }
}

#[derive(Debug)]
pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateImagesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateImagesEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateLayoutEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateCappingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateCappingEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateOwnerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateOwnerEvent {
            index
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let area: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("area".to_string()))?;
            u16::from_value(value.clone(), "area")?
        };
        Ok(ControlPointStartTouchEvent {
            player,
            area
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let area: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("area".to_string()))?;
            u16::from_value(value.clone(), "area")?
        };
        Ok(ControlPointEndTouchEvent {
            player,
            area
        })
    }
}

#[derive(Debug)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl FromRawGameEvent for ControlPointPulseElementEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(ControlPointPulseElementEvent {
            player
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let int_data: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("int_data".to_string()))?;
            u16::from_value(value.clone(), "int_data")?
        };
        Ok(ControlPointFakeCaptureEvent {
            player,
            int_data
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let int_data: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("int_data".to_string()))?;
            u16::from_value(value.clone(), "int_data")?
        };
        Ok(ControlPointFakeCaptureMultiplierEvent {
            player,
            int_data
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: String,
}
impl FromRawGameEvent for TeamPlayRoundSelectedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let round: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("round".to_string()))?;
            String::from_value(value.clone(), "round")?
        };
        Ok(TeamPlayRoundSelectedEvent {
            round
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl FromRawGameEvent for TeamPlayRoundStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let full_reset: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("full_reset".to_string()))?;
            bool::from_value(value.clone(), "full_reset")?
        };
        Ok(TeamPlayRoundStartEvent {
            full_reset
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRoundActiveEvent {

}
impl FromRawGameEvent for TeamPlayRoundActiveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayRoundActiveEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayWaitingBeginsEvent {

}
impl FromRawGameEvent for TeamPlayWaitingBeginsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingBeginsEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayWaitingEndsEvent {

}
impl FromRawGameEvent for TeamPlayWaitingEndsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingEndsEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayWaitingAboutToEndEvent {

}
impl FromRawGameEvent for TeamPlayWaitingAboutToEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingAboutToEndEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRestartRoundEvent {

}
impl FromRawGameEvent for TeamPlayRestartRoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayRestartRoundEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayReadyRestartEvent {

}
impl FromRawGameEvent for TeamPlayReadyRestartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayReadyRestartEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayRoundRestartSecondsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let seconds: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("seconds".to_string()))?;
            u16::from_value(value.clone(), "seconds")?
        };
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeamReadyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayTeamReadyEvent {
            team
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let win_reason: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("win_reason".to_string()))?;
            u8::from_value(value.clone(), "win_reason")?
        };
        let flag_cap_limit: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("flag_cap_limit".to_string()))?;
            u16::from_value(value.clone(), "flag_cap_limit")?
        };
        let full_round: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("full_round".to_string()))?;
            u16::from_value(value.clone(), "full_round")?
        };
        let round_time: f32 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("round_time".to_string()))?;
            f32::from_value(value.clone(), "round_time")?
        };
        let losing_team_num_caps: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("losing_team_num_caps".to_string()))?;
            u16::from_value(value.clone(), "losing_team_num_caps")?
        };
        let was_sudden_death: u8 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("was_sudden_death".to_string()))?;
            u8::from_value(value.clone(), "was_sudden_death")?
        };
        Ok(TeamPlayRoundWinEvent {
            team,
            win_reason,
            flag_cap_limit,
            full_round,
            round_time,
            losing_team_num_caps,
            was_sudden_death
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayUpdateTimerEvent {

}
impl FromRawGameEvent for TeamPlayUpdateTimerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayUpdateTimerEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl FromRawGameEvent for TeamPlayRoundStalemateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let reason: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            u8::from_value(value.clone(), "reason")?
        };
        Ok(TeamPlayRoundStalemateEvent {
            reason
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayOvertimeBeginEvent {

}
impl FromRawGameEvent for TeamPlayOvertimeBeginEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayOvertimeBeginEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayOvertimeEndEvent {

}
impl FromRawGameEvent for TeamPlayOvertimeEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlayOvertimeEndEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlaySuddenDeathBeginEvent {

}
impl FromRawGameEvent for TeamPlaySuddenDeathBeginEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySuddenDeathBeginEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlaySuddenDeathEndEvent {

}
impl FromRawGameEvent for TeamPlaySuddenDeathEndEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySuddenDeathEndEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TeamPlayGameOverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(TeamPlayGameOverEvent {
            reason
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayMapTimeRemainingEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let seconds: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("seconds".to_string()))?;
            u16::from_value(value.clone(), "seconds")?
        };
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl FromRawGameEvent for TeamPlayTimerFlashEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let time_remaining: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u16::from_value(value.clone(), "time_remaining")?
        };
        Ok(TeamPlayTimerFlashEvent {
            time_remaining
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
        let timer: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("timer".to_string()))?;
            u16::from_value(value.clone(), "timer")?
        };
        let seconds_added: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("seconds_added".to_string()))?;
            u16::from_value(value.clone(), "seconds_added")?
        };
        Ok(TeamPlayTimerTimeAddedEvent {
            timer,
            seconds_added
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
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let cap_team: u8 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("cap_team".to_string()))?;
            u8::from_value(value.clone(), "cap_team")?
        };
        let cappers: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let cap_time: f32 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("cap_time".to_string()))?;
            f32::from_value(value.clone(), "cap_time")?
        };
        Ok(TeamPlayPointStartCaptureEvent {
            cp,
            cp_name,
            team,
            cap_team,
            cappers,
            cap_time
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
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let cappers: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        Ok(TeamPlayPointCapturedEvent {
            cp,
            cp_name,
            team,
            cappers
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
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayPointLockedEvent {
            cp,
            cp_name,
            team
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
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let team: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayPointUnlockedEvent {
            cp,
            cp_name,
            team
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
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let time_remaining: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            f32::from_value(value.clone(), "time_remaining")?
        };
        Ok(TeamPlayCaptureBrokenEvent {
            cp,
            cp_name,
            time_remaining
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cp_name: String,
    pub blocker: u8,
}
impl FromRawGameEvent for TeamPlayCaptureBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cp_name: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cp_name".to_string()))?;
            String::from_value(value.clone(), "cp_name")?
        };
        let blocker: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("blocker".to_string()))?;
            u8::from_value(value.clone(), "blocker")?
        };
        Ok(TeamPlayCaptureBlockedEvent {
            cp,
            cp_name,
            blocker
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let carrier: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("carrier".to_string()))?;
            u16::from_value(value.clone(), "carrier")?
        };
        let event_type: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("event_type".to_string()))?;
            u16::from_value(value.clone(), "event_type")?
        };
        let home: u8 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("home".to_string()))?;
            u8::from_value(value.clone(), "home")?
        };
        let team: u8 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayFlagEventEvent {
            player,
            carrier,
            event_type,
            home,
            team
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
}
impl FromRawGameEvent for TeamPlayWinPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let panel_style: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let win_reason: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("win_reason".to_string()))?;
            u8::from_value(value.clone(), "win_reason")?
        };
        let cappers: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let flag_cap_limit: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("flag_cap_limit".to_string()))?;
            u16::from_value(value.clone(), "flag_cap_limit")?
        };
        let blue_score: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("blue_score".to_string()))?;
            u16::from_value(value.clone(), "blue_score")?
        };
        let red_score: u16 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("red_score".to_string()))?;
            u16::from_value(value.clone(), "red_score")?
        };
        let blue_score_prev: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("blue_score_prev".to_string()))?;
            u16::from_value(value.clone(), "blue_score_prev")?
        };
        let red_score_prev: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("red_score_prev".to_string()))?;
            u16::from_value(value.clone(), "red_score_prev")?
        };
        let round_complete: u16 = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("round_complete".to_string()))?;
            u16::from_value(value.clone(), "round_complete")?
        };
        let rounds_remaining: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("rounds_remaining".to_string()))?;
            u16::from_value(value.clone(), "rounds_remaining")?
        };
        let player_1: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("player_1".to_string()))?;
            u16::from_value(value.clone(), "player_1")?
        };
        let player_1_points: u16 = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("player_1_points".to_string()))?;
            u16::from_value(value.clone(), "player_1_points")?
        };
        let player_2: u16 = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("player_2".to_string()))?;
            u16::from_value(value.clone(), "player_2")?
        };
        let player_2_points: u16 = {
            let value = values.get(14).ok_or_else(|| ParseError::UnknownGameEvent("player_2_points".to_string()))?;
            u16::from_value(value.clone(), "player_2_points")?
        };
        let player_3: u16 = {
            let value = values.get(15).ok_or_else(|| ParseError::UnknownGameEvent("player_3".to_string()))?;
            u16::from_value(value.clone(), "player_3")?
        };
        let player_3_points: u16 = {
            let value = values.get(16).ok_or_else(|| ParseError::UnknownGameEvent("player_3_points".to_string()))?;
            u16::from_value(value.clone(), "player_3_points")?
        };
        let kill_stream_player_1: u16 = {
            let value = values.get(17).ok_or_else(|| ParseError::UnknownGameEvent("kill_stream_player_1".to_string()))?;
            u16::from_value(value.clone(), "kill_stream_player_1")?
        };
        let kill_stream_player_1_count: u16 = {
            let value = values.get(18).ok_or_else(|| ParseError::UnknownGameEvent("kill_stream_player_1_count".to_string()))?;
            u16::from_value(value.clone(), "kill_stream_player_1_count")?
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
            kill_stream_player_1_count
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayTeamBalancedPlayerEvent {
            player,
            team
        })
    }
}

#[derive(Debug)]
pub struct TeamPlaySetupFinishedEvent {

}
impl FromRawGameEvent for TeamPlaySetupFinishedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySetupFinishedEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl FromRawGameEvent for TeamPlayAlertEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let alert_type: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("alert_type".to_string()))?;
            u16::from_value(value.clone(), "alert_type")?
        };
        Ok(TeamPlayAlertEvent {
            alert_type
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
        let next_map: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("next_map".to_string()))?;
            String::from_value(value.clone(), "next_map")?
        };
        let map: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("map".to_string()))?;
            String::from_value(value.clone(), "map")?
        };
        let text: String = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(TrainingCompleteEvent {
            next_map,
            map,
            text
        })
    }
}

#[derive(Debug)]
pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl FromRawGameEvent for ShowFreezePanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let killer: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("killer".to_string()))?;
            u16::from_value(value.clone(), "killer")?
        };
        Ok(ShowFreezePanelEvent {
            killer
        })
    }
}

#[derive(Debug)]
pub struct HideFreezePanelEvent {

}
impl FromRawGameEvent for HideFreezePanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(HideFreezePanelEvent {

        })
    }
}

#[derive(Debug)]
pub struct FreezeCamStartedEvent {

}
impl FromRawGameEvent for FreezeCamStartedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(FreezeCamStartedEvent {

        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerChangeTeamEvent {

}
impl FromRawGameEvent for LocalPlayerChangeTeamEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChangeTeamEvent {

        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl FromRawGameEvent for LocalPlayerScoreChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let score: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(LocalPlayerScoreChangedEvent {
            score
        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerChangeClassEvent {

}
impl FromRawGameEvent for LocalPlayerChangeClassEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChangeClassEvent {

        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerRespawnEvent {

}
impl FromRawGameEvent for LocalPlayerRespawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerRespawnEvent {

        })
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
        let building_type: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("building_type".to_string()))?;
            u8::from_value(value.clone(), "building_type")?
        };
        let object_mode: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object_mode".to_string()))?;
            u8::from_value(value.clone(), "object_mode")?
        };
        let remove: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("remove".to_string()))?;
            u8::from_value(value.clone(), "remove")?
        };
        Ok(BuildingInfoChangedEvent {
            building_type,
            object_mode,
            remove
        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl FromRawGameEvent for LocalPlayerChangeDisguiseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let disguised: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("disguised".to_string()))?;
            bool::from_value(value.clone(), "disguised")?
        };
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised
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
        let old_value: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("old_value".to_string()))?;
            u16::from_value(value.clone(), "old_value")?
        };
        let new_value: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("new_value".to_string()))?;
            u16::from_value(value.clone(), "new_value")?
        };
        Ok(PlayerAccountChangedEvent {
            old_value,
            new_value
        })
    }
}

#[derive(Debug)]
pub struct SpyPdaResetEvent {

}
impl FromRawGameEvent for SpyPdaResetEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(SpyPdaResetEvent {

        })
    }
}

#[derive(Debug)]
pub struct FlagStatusUpdateEvent {
    pub user_id: u16,
    pub ent_index: u32,
}
impl FromRawGameEvent for FlagStatusUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u32::from_value(value.clone(), "ent_index")?
        };
        Ok(FlagStatusUpdateEvent {
            user_id,
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl FromRawGameEvent for PlayerStatsUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let force_upload: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("force_upload".to_string()))?;
            bool::from_value(value.clone(), "force_upload")?
        };
        Ok(PlayerStatsUpdatedEvent {
            force_upload
        })
    }
}

#[derive(Debug)]
pub struct PlayingCommentaryEvent {

}
impl FromRawGameEvent for PlayingCommentaryEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PlayingCommentaryEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerChargeDeployedEvent {
    pub user_id: u16,
    pub target_id: u16,
}
impl FromRawGameEvent for PlayerChargeDeployedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let target_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("target_id".to_string()))?;
            u16::from_value(value.clone(), "target_id")?
        };
        Ok(PlayerChargeDeployedEvent {
            user_id,
            target_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerBuiltObjectEvent {
            user_id,
            object,
            index
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let is_builder: bool = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("is_builder".to_string()))?;
            bool::from_value(value.clone(), "is_builder")?
        };
        Ok(PlayerUpgradedObjectEvent {
            user_id,
            object,
            index,
            is_builder
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerCarryObjectEvent {
            user_id,
            object,
            index
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerDropObjectEvent {
            user_id,
            object,
            index
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object_type: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object_type".to_string()))?;
            u16::from_value(value.clone(), "object_type")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ObjectRemovedEvent {
            user_id,
            object_type,
            index
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let attacker: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let assister: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let object_type: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("object_type".to_string()))?;
            u16::from_value(value.clone(), "object_type")?
        };
        let index: u16 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let was_building: bool = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("was_building".to_string()))?;
            bool::from_value(value.clone(), "was_building")?
        };
        Ok(ObjectDestroyedEvent {
            user_id,
            attacker,
            assister,
            weapon,
            weapon_id,
            object_type,
            index,
            was_building
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let object_type: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object_type".to_string()))?;
            u16::from_value(value.clone(), "object_type")?
        };
        let index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ObjectDetonatedEvent {
            user_id,
            object_type,
            index
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
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let achievement: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("achievement".to_string()))?;
            u16::from_value(value.clone(), "achievement")?
        };
        Ok(AchievementEarnedEvent {
            player,
            achievement
        })
    }
}

#[derive(Debug)]
pub struct SpecTargetUpdatedEvent {

}
impl FromRawGameEvent for SpecTargetUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(SpecTargetUpdatedEvent {

        })
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let name_change: bool = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("name_change".to_string()))?;
            bool::from_value(value.clone(), "name_change")?
        };
        let ready_state: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("ready_state".to_string()))?;
            u16::from_value(value.clone(), "ready_state")?
        };
        let new_name: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("new_name".to_string()))?;
            String::from_value(value.clone(), "new_name")?
        };
        Ok(TournamentStateUpdateEvent {
            user_id,
            name_change,
            ready_state,
            new_name
        })
    }
}

#[derive(Debug)]
pub struct TournamentEnableCountdownEvent {

}
impl FromRawGameEvent for TournamentEnableCountdownEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TournamentEnableCountdownEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerCalledForMedicEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerCalledForMedicEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PlayerCalledForMedicEvent {
            user_id
        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerBecameObserverEvent {

}
impl FromRawGameEvent for LocalPlayerBecameObserverEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerBecameObserverEvent {

        })
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
        let pyro_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("pyro_ent_index".to_string()))?;
            u8::from_value(value.clone(), "pyro_ent_index")?
        };
        let victim_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u8::from_value(value.clone(), "victim_ent_index")?
        };
        let medic_ent_index: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("medic_ent_index".to_string()))?;
            u8::from_value(value.clone(), "medic_ent_index")?
        };
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index,
            victim_ent_index,
            medic_ent_index
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
        let pyro_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("pyro_ent_index".to_string()))?;
            u8::from_value(value.clone(), "pyro_ent_index")?
        };
        let victim_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u8::from_value(value.clone(), "victim_ent_index")?
        };
        let weapon_id: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u8::from_value(value.clone(), "weapon_id")?
        };
        Ok(PlayerIgnitedEvent {
            pyro_ent_index,
            victim_ent_index,
            weapon_id
        })
    }
}

#[derive(Debug)]
pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
}
impl FromRawGameEvent for PlayerExtinguishedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let victim: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("victim".to_string()))?;
            u8::from_value(value.clone(), "victim")?
        };
        let healer: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("healer".to_string()))?;
            u8::from_value(value.clone(), "healer")?
        };
        Ok(PlayerExtinguishedEvent {
            victim,
            healer
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let builder_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("builder_id".to_string()))?;
            u16::from_value(value.clone(), "builder_id")?
        };
        let dist: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("dist".to_string()))?;
            f32::from_value(value.clone(), "dist")?
        };
        Ok(PlayerTeleportedEvent {
            user_id,
            builder_id,
            dist
        })
    }
}

#[derive(Debug)]
pub struct PlayerHealedMedicCallEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerHealedMedicCallEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PlayerHealedMedicCallEvent {
            user_id
        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerChargeReadyEvent {

}
impl FromRawGameEvent for LocalPlayerChargeReadyEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChargeReadyEvent {

        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerWindDownEvent {

}
impl FromRawGameEvent for LocalPlayerWindDownEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerWindDownEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerInvulnedEvent {
    pub user_id: u16,
    pub medic_user_id: u16,
}
impl FromRawGameEvent for PlayerInvulnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let medic_user_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("medic_user_id".to_string()))?;
            u16::from_value(value.clone(), "medic_user_id")?
        };
        Ok(PlayerInvulnedEvent {
            user_id,
            medic_user_id
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let speed: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("speed".to_string()))?;
            u8::from_value(value.clone(), "speed")?
        };
        let players: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("players".to_string()))?;
            u8::from_value(value.clone(), "players")?
        };
        Ok(EscortSpeedEvent {
            team,
            speed,
            players
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let progress: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("progress".to_string()))?;
            f32::from_value(value.clone(), "progress")?
        };
        let reset: bool = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("reset".to_string()))?;
            bool::from_value(value.clone(), "reset")?
        };
        Ok(EscortProgressEvent {
            team,
            progress,
            reset
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let recede_time: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("recede_time".to_string()))?;
            f32::from_value(value.clone(), "recede_time")?
        };
        Ok(EscortRecedeEvent {
            team,
            recede_time
        })
    }
}

#[derive(Debug)]
pub struct GameUIActivatedEvent {

}
impl FromRawGameEvent for GameUIActivatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(GameUIActivatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct GameUIHiddenEvent {

}
impl FromRawGameEvent for GameUIHiddenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(GameUIHiddenEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl FromRawGameEvent for PlayerEscortScoreEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let points: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("points".to_string()))?;
            u8::from_value(value.clone(), "points")?
        };
        Ok(PlayerEscortScoreEvent {
            player,
            points
        })
    }
}

#[derive(Debug)]
pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHealOnHitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let amount: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        let ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u8::from_value(value.clone(), "ent_index")?
        };
        Ok(PlayerHealOnHitEvent {
            amount,
            ent_index
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
        let owner: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("owner".to_string()))?;
            u16::from_value(value.clone(), "owner")?
        };
        let target: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(PlayerStealSandvichEvent {
            owner,
            target
        })
    }
}

#[derive(Debug)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowClassLayoutEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let show: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("show".to_string()))?;
            bool::from_value(value.clone(), "show")?
        };
        Ok(ShowClassLayoutEvent {
            show
        })
    }
}

#[derive(Debug)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowVsPanelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let show: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("show".to_string()))?;
            bool::from_value(value.clone(), "show")?
        };
        Ok(ShowVsPanelEvent {
            show
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
        let amount: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        let kind: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("kind".to_string()))?;
            u32::from_value(value.clone(), "kind")?
        };
        Ok(PlayerDamagedEvent {
            amount,
            kind
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
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let message: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("message".to_string()))?;
            u8::from_value(value.clone(), "message")?
        };
        Ok(ArenaPlayerNotificationEvent {
            player,
            message
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let streak: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("streak".to_string()))?;
            u8::from_value(value.clone(), "streak")?
        };
        Ok(ArenaMatchMaxStreakEvent {
            team,
            streak
        })
    }
}

#[derive(Debug)]
pub struct ArenaRoundStartEvent {

}
impl FromRawGameEvent for ArenaRoundStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ArenaRoundStartEvent {

        })
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
        let panel_style: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let win_reason: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("win_reason".to_string()))?;
            u8::from_value(value.clone(), "win_reason")?
        };
        let cappers: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let flag_cap_limit: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("flag_cap_limit".to_string()))?;
            u16::from_value(value.clone(), "flag_cap_limit")?
        };
        let blue_score: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("blue_score".to_string()))?;
            u16::from_value(value.clone(), "blue_score")?
        };
        let red_score: u16 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("red_score".to_string()))?;
            u16::from_value(value.clone(), "red_score")?
        };
        let blue_score_prev: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("blue_score_prev".to_string()))?;
            u16::from_value(value.clone(), "blue_score_prev")?
        };
        let red_score_prev: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("red_score_prev".to_string()))?;
            u16::from_value(value.clone(), "red_score_prev")?
        };
        let round_complete: u16 = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("round_complete".to_string()))?;
            u16::from_value(value.clone(), "round_complete")?
        };
        let player_1: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("player_1".to_string()))?;
            u16::from_value(value.clone(), "player_1")?
        };
        let player_1_damage: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("player_1_damage".to_string()))?;
            u16::from_value(value.clone(), "player_1_damage")?
        };
        let player_1_healing: u16 = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("player_1_healing".to_string()))?;
            u16::from_value(value.clone(), "player_1_healing")?
        };
        let player_1_lifetime: u16 = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("player_1_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_1_lifetime")?
        };
        let player_1_kills: u16 = {
            let value = values.get(14).ok_or_else(|| ParseError::UnknownGameEvent("player_1_kills".to_string()))?;
            u16::from_value(value.clone(), "player_1_kills")?
        };
        let player_2: u16 = {
            let value = values.get(15).ok_or_else(|| ParseError::UnknownGameEvent("player_2".to_string()))?;
            u16::from_value(value.clone(), "player_2")?
        };
        let player_2_damage: u16 = {
            let value = values.get(16).ok_or_else(|| ParseError::UnknownGameEvent("player_2_damage".to_string()))?;
            u16::from_value(value.clone(), "player_2_damage")?
        };
        let player_2_healing: u16 = {
            let value = values.get(17).ok_or_else(|| ParseError::UnknownGameEvent("player_2_healing".to_string()))?;
            u16::from_value(value.clone(), "player_2_healing")?
        };
        let player_2_lifetime: u16 = {
            let value = values.get(18).ok_or_else(|| ParseError::UnknownGameEvent("player_2_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_2_lifetime")?
        };
        let player_2_kills: u16 = {
            let value = values.get(19).ok_or_else(|| ParseError::UnknownGameEvent("player_2_kills".to_string()))?;
            u16::from_value(value.clone(), "player_2_kills")?
        };
        let player_3: u16 = {
            let value = values.get(20).ok_or_else(|| ParseError::UnknownGameEvent("player_3".to_string()))?;
            u16::from_value(value.clone(), "player_3")?
        };
        let player_3_damage: u16 = {
            let value = values.get(21).ok_or_else(|| ParseError::UnknownGameEvent("player_3_damage".to_string()))?;
            u16::from_value(value.clone(), "player_3_damage")?
        };
        let player_3_healing: u16 = {
            let value = values.get(22).ok_or_else(|| ParseError::UnknownGameEvent("player_3_healing".to_string()))?;
            u16::from_value(value.clone(), "player_3_healing")?
        };
        let player_3_lifetime: u16 = {
            let value = values.get(23).ok_or_else(|| ParseError::UnknownGameEvent("player_3_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_3_lifetime")?
        };
        let player_3_kills: u16 = {
            let value = values.get(24).ok_or_else(|| ParseError::UnknownGameEvent("player_3_kills".to_string()))?;
            u16::from_value(value.clone(), "player_3_kills")?
        };
        let player_4: u16 = {
            let value = values.get(25).ok_or_else(|| ParseError::UnknownGameEvent("player_4".to_string()))?;
            u16::from_value(value.clone(), "player_4")?
        };
        let player_4_damage: u16 = {
            let value = values.get(26).ok_or_else(|| ParseError::UnknownGameEvent("player_4_damage".to_string()))?;
            u16::from_value(value.clone(), "player_4_damage")?
        };
        let player_4_healing: u16 = {
            let value = values.get(27).ok_or_else(|| ParseError::UnknownGameEvent("player_4_healing".to_string()))?;
            u16::from_value(value.clone(), "player_4_healing")?
        };
        let player_4_lifetime: u16 = {
            let value = values.get(28).ok_or_else(|| ParseError::UnknownGameEvent("player_4_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_4_lifetime")?
        };
        let player_4_kills: u16 = {
            let value = values.get(29).ok_or_else(|| ParseError::UnknownGameEvent("player_4_kills".to_string()))?;
            u16::from_value(value.clone(), "player_4_kills")?
        };
        let player_5: u16 = {
            let value = values.get(30).ok_or_else(|| ParseError::UnknownGameEvent("player_5".to_string()))?;
            u16::from_value(value.clone(), "player_5")?
        };
        let player_5_damage: u16 = {
            let value = values.get(31).ok_or_else(|| ParseError::UnknownGameEvent("player_5_damage".to_string()))?;
            u16::from_value(value.clone(), "player_5_damage")?
        };
        let player_5_healing: u16 = {
            let value = values.get(32).ok_or_else(|| ParseError::UnknownGameEvent("player_5_healing".to_string()))?;
            u16::from_value(value.clone(), "player_5_healing")?
        };
        let player_5_lifetime: u16 = {
            let value = values.get(33).ok_or_else(|| ParseError::UnknownGameEvent("player_5_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_5_lifetime")?
        };
        let player_5_kills: u16 = {
            let value = values.get(34).ok_or_else(|| ParseError::UnknownGameEvent("player_5_kills".to_string()))?;
            u16::from_value(value.clone(), "player_5_kills")?
        };
        let player_6: u16 = {
            let value = values.get(35).ok_or_else(|| ParseError::UnknownGameEvent("player_6".to_string()))?;
            u16::from_value(value.clone(), "player_6")?
        };
        let player_6_damage: u16 = {
            let value = values.get(36).ok_or_else(|| ParseError::UnknownGameEvent("player_6_damage".to_string()))?;
            u16::from_value(value.clone(), "player_6_damage")?
        };
        let player_6_healing: u16 = {
            let value = values.get(37).ok_or_else(|| ParseError::UnknownGameEvent("player_6_healing".to_string()))?;
            u16::from_value(value.clone(), "player_6_healing")?
        };
        let player_6_lifetime: u16 = {
            let value = values.get(38).ok_or_else(|| ParseError::UnknownGameEvent("player_6_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_6_lifetime")?
        };
        let player_6_kills: u16 = {
            let value = values.get(39).ok_or_else(|| ParseError::UnknownGameEvent("player_6_kills".to_string()))?;
            u16::from_value(value.clone(), "player_6_kills")?
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
            player_6_kills
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
        let panel_style: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let win_reason: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("win_reason".to_string()))?;
            u8::from_value(value.clone(), "win_reason")?
        };
        Ok(PveWinPanelEvent {
            panel_style,
            winning_team,
            win_reason
        })
    }
}

#[derive(Debug)]
pub struct AirDashEvent {
    pub player: u8,
}
impl FromRawGameEvent for AirDashEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        Ok(AirDashEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct LandedEvent {
    pub player: u8,
}
impl FromRawGameEvent for LandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        Ok(LandedEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl FromRawGameEvent for PlayerDamageDodgedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let damage: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("damage".to_string()))?;
            u16::from_value(value.clone(), "damage")?
        };
        Ok(PlayerDamageDodgedEvent {
            damage
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
        let stunner: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("stunner".to_string()))?;
            u16::from_value(value.clone(), "stunner")?
        };
        let victim: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        let victim_capping: bool = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("victim_capping".to_string()))?;
            bool::from_value(value.clone(), "victim_capping")?
        };
        let big_stun: bool = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("big_stun".to_string()))?;
            bool::from_value(value.clone(), "big_stun")?
        };
        Ok(PlayerStunnedEvent {
            stunner,
            victim,
            victim_capping,
            big_stun
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
        let scout_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("scout_id".to_string()))?;
            u16::from_value(value.clone(), "scout_id")?
        };
        let target_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("target_id".to_string()))?;
            u16::from_value(value.clone(), "target_id")?
        };
        Ok(ScoutGrandSlamEvent {
            scout_id,
            target_id
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
        let target_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("target_index".to_string()))?;
            u16::from_value(value.clone(), "target_index")?
        };
        let x: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("x".to_string()))?;
            f32::from_value(value.clone(), "x")?
        };
        let y: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("y".to_string()))?;
            f32::from_value(value.clone(), "y")?
        };
        let z: f32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("z".to_string()))?;
            f32::from_value(value.clone(), "z")?
        };
        Ok(ScoutSlamdollLandedEvent {
            target_index,
            x,
            y,
            z
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
        let attached_entity: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("attached_entity".to_string()))?;
            u16::from_value(value.clone(), "attached_entity")?
        };
        let shooter: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("shooter".to_string()))?;
            u16::from_value(value.clone(), "shooter")?
        };
        let bone_index_attached: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("bone_index_attached".to_string()))?;
            u16::from_value(value.clone(), "bone_index_attached")?
        };
        let bone_position_x: f32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("bone_position_x".to_string()))?;
            f32::from_value(value.clone(), "bone_position_x")?
        };
        let bone_position_y: f32 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("bone_position_y".to_string()))?;
            f32::from_value(value.clone(), "bone_position_y")?
        };
        let bone_position_z: f32 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("bone_position_z".to_string()))?;
            f32::from_value(value.clone(), "bone_position_z")?
        };
        let bone_angles_x: f32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("bone_angles_x".to_string()))?;
            f32::from_value(value.clone(), "bone_angles_x")?
        };
        let bone_angles_y: f32 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("bone_angles_y".to_string()))?;
            f32::from_value(value.clone(), "bone_angles_y")?
        };
        let bone_angles_z: f32 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("bone_angles_z".to_string()))?;
            f32::from_value(value.clone(), "bone_angles_z")?
        };
        let projectile_type: u16 = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("projectile_type".to_string()))?;
            u16::from_value(value.clone(), "projectile_type")?
        };
        let is_crit: bool = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("is_crit".to_string()))?;
            bool::from_value(value.clone(), "is_crit")?
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
            is_crit
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
        let thrower_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("thrower_ent_index".to_string()))?;
            u8::from_value(value.clone(), "thrower_ent_index")?
        };
        let victim_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u8::from_value(value.clone(), "victim_ent_index")?
        };
        Ok(PlayerJaratedEvent {
            thrower_ent_index,
            victim_ent_index
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
        let thrower_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("thrower_ent_index".to_string()))?;
            u8::from_value(value.clone(), "thrower_ent_index")?
        };
        let victim_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u8::from_value(value.clone(), "victim_ent_index")?
        };
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index,
            victim_ent_index
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
        let attacker_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("attacker_ent_index".to_string()))?;
            u8::from_value(value.clone(), "attacker_ent_index")?
        };
        let blocker_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("blocker_ent_index".to_string()))?;
            u8::from_value(value.clone(), "blocker_ent_index")?
        };
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index,
            blocker_ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl FromRawGameEvent for PlayerPinnedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let pinned: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("pinned".to_string()))?;
            u8::from_value(value.clone(), "pinned")?
        };
        Ok(PlayerPinnedEvent {
            pinned
        })
    }
}

#[derive(Debug)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl FromRawGameEvent for PlayerHealedByMedicEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let medic: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("medic".to_string()))?;
            u8::from_value(value.clone(), "medic")?
        };
        Ok(PlayerHealedByMedicEvent {
            medic
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let owner_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("owner_id".to_string()))?;
            u16::from_value(value.clone(), "owner_id")?
        };
        let object: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u8::from_value(value.clone(), "object")?
        };
        let sapper_id: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("sapper_id".to_string()))?;
            u16::from_value(value.clone(), "sapper_id")?
        };
        Ok(PlayerSappedObjectEvent {
            user_id,
            owner_id,
            object,
            sapper_id
        })
    }
}

#[derive(Debug)]
pub struct ItemFoundEvent {
    pub player: u8,
    pub quality: u8,
    pub method: u8,
    pub item_def: u32,
}
impl FromRawGameEvent for ItemFoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let quality: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("quality".to_string()))?;
            u8::from_value(value.clone(), "quality")?
        };
        let method: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("method".to_string()))?;
            u8::from_value(value.clone(), "method")?
        };
        let item_def: u32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("item_def".to_string()))?;
            u32::from_value(value.clone(), "item_def")?
        };
        Ok(ItemFoundEvent {
            player,
            quality,
            method,
            item_def
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
        let world_pos_x: f32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("world_pos_x".to_string()))?;
            f32::from_value(value.clone(), "world_pos_x")?
        };
        let world_pos_y: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("world_pos_y".to_string()))?;
            f32::from_value(value.clone(), "world_pos_y")?
        };
        let world_pos_z: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("world_pos_z".to_string()))?;
            f32::from_value(value.clone(), "world_pos_z")?
        };
        let world_normal_x: f32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("world_normal_x".to_string()))?;
            f32::from_value(value.clone(), "world_normal_x")?
        };
        let world_normal_y: f32 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("world_normal_y".to_string()))?;
            f32::from_value(value.clone(), "world_normal_y")?
        };
        let world_normal_z: f32 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("world_normal_z".to_string()))?;
            f32::from_value(value.clone(), "world_normal_z")?
        };
        let id: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("id".to_string()))?;
            u32::from_value(value.clone(), "id")?
        };
        let text: String = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        let lifetime: f32 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("lifetime".to_string()))?;
            f32::from_value(value.clone(), "lifetime")?
        };
        let visibility_bit_field: u32 = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("visibility_bit_field".to_string()))?;
            u32::from_value(value.clone(), "visibility_bit_field")?
        };
        let follow_ent_index: u32 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("follow_ent_index".to_string()))?;
            u32::from_value(value.clone(), "follow_ent_index")?
        };
        let show_distance: bool = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("show_distance".to_string()))?;
            bool::from_value(value.clone(), "show_distance")?
        };
        let play_sound: String = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("play_sound".to_string()))?;
            String::from_value(value.clone(), "play_sound")?
        };
        let show_effect: bool = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("show_effect".to_string()))?;
            bool::from_value(value.clone(), "show_effect")?
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
            show_effect
        })
    }
}

#[derive(Debug)]
pub struct HideAnnotationEvent {
    pub id: u32,
}
impl FromRawGameEvent for HideAnnotationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let id: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("id".to_string()))?;
            u32::from_value(value.clone(), "id")?
        };
        Ok(HideAnnotationEvent {
            id
        })
    }
}

#[derive(Debug)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PostInventoryApplicationEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PostInventoryApplicationEvent {
            user_id
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
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let time: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(ControlPointUnlockUpdatedEvent {
            index,
            time
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
        let buff_type: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("buff_type".to_string()))?;
            u8::from_value(value.clone(), "buff_type")?
        };
        let buff_owner: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("buff_owner".to_string()))?;
            u16::from_value(value.clone(), "buff_owner")?
        };
        Ok(DeployBuffBannerEvent {
            buff_type,
            buff_owner
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let buff_owner: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("buff_owner".to_string()))?;
            u16::from_value(value.clone(), "buff_owner")?
        };
        let buff_type: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("buff_type".to_string()))?;
            u8::from_value(value.clone(), "buff_type")?
        };
        Ok(PlayerBuffEvent {
            user_id,
            buff_owner,
            buff_type
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let attacker: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let healing: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("healing".to_string()))?;
            u16::from_value(value.clone(), "healing")?
        };
        let charged: bool = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("charged".to_string()))?;
            bool::from_value(value.clone(), "charged")?
        };
        Ok(MedicDeathEvent {
            user_id,
            attacker,
            healing,
            charged
        })
    }
}

#[derive(Debug)]
pub struct OvertimeNagEvent {

}
impl FromRawGameEvent for OvertimeNagEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(OvertimeNagEvent {

        })
    }
}

#[derive(Debug)]
pub struct TeamsChangedEvent {

}
impl FromRawGameEvent for TeamsChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(TeamsChangedEvent {

        })
    }
}

#[derive(Debug)]
pub struct HalloweenPumpkinGrabEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for HalloweenPumpkinGrabEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(HalloweenPumpkinGrabEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let play_sound: bool = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("play_sound".to_string()))?;
            bool::from_value(value.clone(), "play_sound")?
        };
        Ok(RocketJumpEvent {
            user_id,
            play_sound
        })
    }
}

#[derive(Debug)]
pub struct RocketJumpLandedEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for RocketJumpLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(RocketJumpLandedEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let play_sound: bool = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("play_sound".to_string()))?;
            bool::from_value(value.clone(), "play_sound")?
        };
        Ok(StickyJumpEvent {
            user_id,
            play_sound
        })
    }
}

#[derive(Debug)]
pub struct StickyJumpLandedEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for StickyJumpLandedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(StickyJumpLandedEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let medic: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("medic".to_string()))?;
            u16::from_value(value.clone(), "medic")?
        };
        Ok(MedicDefendedEvent {
            user_id,
            medic
        })
    }
}

#[derive(Debug)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl FromRawGameEvent for LocalPlayerHealedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let amount: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(LocalPlayerHealedEvent {
            amount
        })
    }
}

#[derive(Debug)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerDestroyedPipeBombEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PlayerDestroyedPipeBombEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let owner_id: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("owner_id".to_string()))?;
            u16::from_value(value.clone(), "owner_id")?
        };
        let weapon_id: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let object_ent_index: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("object_ent_index".to_string()))?;
            u16::from_value(value.clone(), "object_ent_index")?
        };
        Ok(ObjectDeflectedEvent {
            user_id,
            owner_id,
            weapon_id,
            object_ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerMvpEvent {
    pub player: u16,
}
impl FromRawGameEvent for PlayerMvpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(PlayerMvpEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct RaidSpawnMobEvent {

}
impl FromRawGameEvent for RaidSpawnMobEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(RaidSpawnMobEvent {

        })
    }
}

#[derive(Debug)]
pub struct RaidSpawnSquadEvent {

}
impl FromRawGameEvent for RaidSpawnSquadEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(RaidSpawnSquadEvent {

        })
    }
}

#[derive(Debug)]
pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl FromRawGameEvent for NavBlockedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let area: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("area".to_string()))?;
            u32::from_value(value.clone(), "area")?
        };
        let blocked: bool = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("blocked".to_string()))?;
            bool::from_value(value.clone(), "blocked")?
        };
        Ok(NavBlockedEvent {
            area,
            blocked
        })
    }
}

#[derive(Debug)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl FromRawGameEvent for PathTrackPassedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PathTrackPassedEvent {
            index
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
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let count: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("count".to_string()))?;
            u8::from_value(value.clone(), "count")?
        };
        Ok(NumCappersChangedEvent {
            index,
            count
        })
    }
}

#[derive(Debug)]
pub struct PlayerRegenerateEvent {

}
impl FromRawGameEvent for PlayerRegenerateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PlayerRegenerateEvent {

        })
    }
}

#[derive(Debug)]
pub struct UpdateStatusItemEvent {
    pub index: u8,
    pub object: u8,
}
impl FromRawGameEvent for UpdateStatusItemEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let object: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("object".to_string()))?;
            u8::from_value(value.clone(), "object")?
        };
        Ok(UpdateStatusItemEvent {
            index,
            object
        })
    }
}

#[derive(Debug)]
pub struct StatsResetRoundEvent {

}
impl FromRawGameEvent for StatsResetRoundEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(StatsResetRoundEvent {

        })
    }
}

#[derive(Debug)]
pub struct ScoreStatsAccumulatedUpdateEvent {

}
impl FromRawGameEvent for ScoreStatsAccumulatedUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ScoreStatsAccumulatedUpdateEvent {

        })
    }
}

#[derive(Debug)]
pub struct ScoreStatsAccumulatedResetEvent {

}
impl FromRawGameEvent for ScoreStatsAccumulatedResetEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ScoreStatsAccumulatedResetEvent {

        })
    }
}

#[derive(Debug)]
pub struct AchievementEarnedLocalEvent {
    pub achievement: u16,
}
impl FromRawGameEvent for AchievementEarnedLocalEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let achievement: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("achievement".to_string()))?;
            u16::from_value(value.clone(), "achievement")?
        };
        Ok(AchievementEarnedLocalEvent {
            achievement
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
        let patient: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("patient".to_string()))?;
            u16::from_value(value.clone(), "patient")?
        };
        let healer: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("healer".to_string()))?;
            u16::from_value(value.clone(), "healer")?
        };
        let amount: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(PlayerHealedEvent {
            patient,
            healer,
            amount
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
        let building: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("building".to_string()))?;
            u16::from_value(value.clone(), "building")?
        };
        let healer: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("healer".to_string()))?;
            u16::from_value(value.clone(), "healer")?
        };
        let amount: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(BuildingHealedEvent {
            building,
            healer,
            amount
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let item: String = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("item".to_string()))?;
            String::from_value(value.clone(), "item")?
        };
        Ok(ItemPickupEvent {
            user_id,
            item
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
        let killer: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("killer".to_string()))?;
            u16::from_value(value.clone(), "killer")?
        };
        let score_type: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("score_type".to_string()))?;
            u16::from_value(value.clone(), "score_type")?
        };
        let initiator: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("initiator".to_string()))?;
            u16::from_value(value.clone(), "initiator")?
        };
        let target: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        let initiator_score: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("initiator_score".to_string()))?;
            u16::from_value(value.clone(), "initiator_score")?
        };
        let target_score: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("target_score".to_string()))?;
            u16::from_value(value.clone(), "target_score")?
        };
        Ok(DuelStatusEvent {
            killer,
            score_type,
            initiator,
            target,
            initiator_score,
            target_score
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let victim_ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u32::from_value(value.clone(), "victim_ent_index")?
        };
        let inflictor_ent_index: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("inflictor_ent_index".to_string()))?;
            u32::from_value(value.clone(), "inflictor_ent_index")?
        };
        let attacker: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_bits: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        let custom_kill: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("custom_kill".to_string()))?;
            u16::from_value(value.clone(), "custom_kill")?
        };
        let assister: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_log_class_name: String = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("weapon_log_class_name".to_string()))?;
            String::from_value(value.clone(), "weapon_log_class_name")?
        };
        let stun_flags: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
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
            assister_fallback
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let victim_ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u32::from_value(value.clone(), "victim_ent_index")?
        };
        let inflictor_ent_index: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("inflictor_ent_index".to_string()))?;
            u32::from_value(value.clone(), "inflictor_ent_index")?
        };
        let attacker: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_bits: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        let custom_kill: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("custom_kill".to_string()))?;
            u16::from_value(value.clone(), "custom_kill")?
        };
        let assister: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_log_class_name: String = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("weapon_log_class_name".to_string()))?;
            String::from_value(value.clone(), "weapon_log_class_name")?
        };
        let stun_flags: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
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
            assister_fallback
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let victim_ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u32::from_value(value.clone(), "victim_ent_index")?
        };
        let inflictor_ent_index: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("inflictor_ent_index".to_string()))?;
            u32::from_value(value.clone(), "inflictor_ent_index")?
        };
        let attacker: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_bits: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        let custom_kill: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("custom_kill".to_string()))?;
            u16::from_value(value.clone(), "custom_kill")?
        };
        let assister: u16 = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_log_class_name: String = {
            let value = values.get(9).ok_or_else(|| ParseError::UnknownGameEvent("weapon_log_class_name".to_string()))?;
            String::from_value(value.clone(), "weapon_log_class_name")?
        };
        let stun_flags: u16 = {
            let value = values.get(10).ok_or_else(|| ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get(11).ok_or_else(|| ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get(12).ok_or_else(|| ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get(13).ok_or_else(|| ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        let total_hits: u16 = {
            let value = values.get(14).ok_or_else(|| ParseError::UnknownGameEvent("total_hits".to_string()))?;
            u16::from_value(value.clone(), "total_hits")?
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
            total_hits
        })
    }
}

#[derive(Debug)]
pub struct PumpkinLordSummonedEvent {

}
impl FromRawGameEvent for PumpkinLordSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PumpkinLordSummonedEvent {

        })
    }
}

#[derive(Debug)]
pub struct PumpkinLordKilledEvent {

}
impl FromRawGameEvent for PumpkinLordKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PumpkinLordKilledEvent {

        })
    }
}

#[derive(Debug)]
pub struct MerasmusSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusSummonedEvent {
            level
        })
    }
}

#[derive(Debug)]
pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusKilledEvent {
            level
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
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let time_remaining: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u8::from_value(value.clone(), "time_remaining")?
        };
        Ok(MerasmusEscapeWarningEvent {
            level,
            time_remaining
        })
    }
}

#[derive(Debug)]
pub struct MerasmusEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusEscapedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusEscapedEvent {
            level
        })
    }
}

#[derive(Debug)]
pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossSummonedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossSummonedEvent {
            level
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
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let player_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("player_ent_index".to_string()))?;
            u8::from_value(value.clone(), "player_ent_index")?
        };
        Ok(EyeballBossStunnedEvent {
            level,
            player_ent_index
        })
    }
}

#[derive(Debug)]
pub struct EyeballBossKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossKilledEvent {
            level
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
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let player_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("player_ent_index".to_string()))?;
            u8::from_value(value.clone(), "player_ent_index")?
        };
        Ok(EyeballBossKillerEvent {
            level,
            player_ent_index
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
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let time_remaining: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u8::from_value(value.clone(), "time_remaining")?
        };
        Ok(EyeballBossEscapeImminentEvent {
            level,
            time_remaining
        })
    }
}

#[derive(Debug)]
pub struct EyeballBossEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossEscapedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossEscapedEvent {
            level
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
}
impl FromRawGameEvent for NpcHurtEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        let health: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("health".to_string()))?;
            u16::from_value(value.clone(), "health")?
        };
        let attacker_player: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("attacker_player".to_string()))?;
            u16::from_value(value.clone(), "attacker_player")?
        };
        let weapon_id: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_amount: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("damage_amount".to_string()))?;
            u16::from_value(value.clone(), "damage_amount")?
        };
        let crit: bool = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("crit".to_string()))?;
            bool::from_value(value.clone(), "crit")?
        };
        Ok(NpcHurtEvent {
            ent_index,
            health,
            attacker_player,
            weapon_id,
            damage_amount,
            crit
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
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let time: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(ControlPointTimerUpdatedEvent {
            index,
            time
        })
    }
}

#[derive(Debug)]
pub struct PlayerHighFiveStartEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveStartEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u8::from_value(value.clone(), "ent_index")?
        };
        Ok(PlayerHighFiveStartEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerHighFiveCancelEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for PlayerHighFiveCancelEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u8::from_value(value.clone(), "ent_index")?
        };
        Ok(PlayerHighFiveCancelEvent {
            ent_index
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
        let initiator_ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("initiator_ent_index".to_string()))?;
            u8::from_value(value.clone(), "initiator_ent_index")?
        };
        let partner_ent_index: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("partner_ent_index".to_string()))?;
            u8::from_value(value.clone(), "partner_ent_index")?
        };
        Ok(PlayerHighFiveSuccessEvent {
            initiator_ent_index,
            partner_ent_index
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
        let points: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("points".to_string()))?;
            u16::from_value(value.clone(), "points")?
        };
        let player_ent_index: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("player_ent_index".to_string()))?;
            u16::from_value(value.clone(), "player_ent_index")?
        };
        let source_ent_index: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("source_ent_index".to_string()))?;
            u16::from_value(value.clone(), "source_ent_index")?
        };
        Ok(PlayerBonusPointsEvent {
            points,
            player_ent_index,
            source_ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerUpgradedEvent {

}
impl FromRawGameEvent for PlayerUpgradedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PlayerUpgradedEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerBuybackEvent {
    pub player: u16,
    pub cost: u16,
}
impl FromRawGameEvent for PlayerBuybackEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let cost: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("cost".to_string()))?;
            u16::from_value(value.clone(), "cost")?
        };
        Ok(PlayerBuybackEvent {
            player,
            cost
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let kind: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("kind".to_string()))?;
            u16::from_value(value.clone(), "kind")?
        };
        let time: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(PlayerUsedPowerUpBottleEvent {
            player,
            kind,
            time
        })
    }
}

#[derive(Debug)]
pub struct ChristmasGiftGrabEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for ChristmasGiftGrabEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(ChristmasGiftGrabEvent {
            user_id
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
        let attacker: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let victim: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        let zone_id: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("zone_id".to_string()))?;
            u16::from_value(value.clone(), "zone_id")?
        };
        Ok(PlayerKilledAchievementZoneEvent {
            attacker,
            victim,
            zone_id
        })
    }
}

#[derive(Debug)]
pub struct PartyUpdatedEvent {

}
impl FromRawGameEvent for PartyUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(PartyUpdatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct LobbyUpdatedEvent {

}
impl FromRawGameEvent for LobbyUpdatedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(LobbyUpdatedEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmMissionUpdateEvent {
    pub class: u16,
    pub count: u16,
}
impl FromRawGameEvent for MvmMissionUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let class: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        let count: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("count".to_string()))?;
            u16::from_value(value.clone(), "count")?
        };
        Ok(MvmMissionUpdateEvent {
            class,
            count
        })
    }
}

#[derive(Debug)]
pub struct RecalculateHolidaysEvent {

}
impl FromRawGameEvent for RecalculateHolidaysEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(RecalculateHolidaysEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerCurrencyChangedEvent {
    pub currency: u16,
}
impl FromRawGameEvent for PlayerCurrencyChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let currency: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(PlayerCurrencyChangedEvent {
            currency
        })
    }
}

#[derive(Debug)]
pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl FromRawGameEvent for DoomsdayRocketOpenEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(DoomsdayRocketOpenEvent {
            team
        })
    }
}

#[derive(Debug)]
pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl FromRawGameEvent for RemoveNemesisRelationshipsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(RemoveNemesisRelationshipsEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MvmCreditBonusWaveEvent {

}
impl FromRawGameEvent for MvmCreditBonusWaveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusWaveEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmCreditBonusAllEvent {

}
impl FromRawGameEvent for MvmCreditBonusAllEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusAllEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmCreditBonusAllAdvancedEvent {

}
impl FromRawGameEvent for MvmCreditBonusAllAdvancedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusAllAdvancedEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmQuickSentryUpgradeEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmQuickSentryUpgradeEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmQuickSentryUpgradeEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MvmTankDestroyedByPlayersEvent {

}
impl FromRawGameEvent for MvmTankDestroyedByPlayersEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmTankDestroyedByPlayersEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmKillRobotDeliveringBombEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmKillRobotDeliveringBombEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmKillRobotDeliveringBombEvent {
            player
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let currency: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(MvmPickupCurrencyEvent {
            player,
            currency
        })
    }
}

#[derive(Debug)]
pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MvmBombCarrierKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MvmBombCarrierKilledEvent {
            level
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
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let det_x: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("det_x".to_string()))?;
            f32::from_value(value.clone(), "det_x")?
        };
        let det_y: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("det_y".to_string()))?;
            f32::from_value(value.clone(), "det_y")?
        };
        let det_z: f32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("det_z".to_string()))?;
            f32::from_value(value.clone(), "det_z")?
        };
        Ok(MvmSentryBusterDetonateEvent {
            player,
            det_x,
            det_y,
            det_z
        })
    }
}

#[derive(Debug)]
pub struct MvmScoutMarkedForDeathEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmScoutMarkedForDeathEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmScoutMarkedForDeathEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MvmMedicPowerUpSharedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmMedicPowerUpSharedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmMedicPowerUpSharedEvent {
            player
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
        let wave_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("wave_index".to_string()))?;
            u16::from_value(value.clone(), "wave_index")?
        };
        let max_waves: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("max_waves".to_string()))?;
            u16::from_value(value.clone(), "max_waves")?
        };
        let advanced: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("advanced".to_string()))?;
            u16::from_value(value.clone(), "advanced")?
        };
        Ok(MvmBeginWaveEvent {
            wave_index,
            max_waves,
            advanced
        })
    }
}

#[derive(Debug)]
pub struct MvmWaveCompleteEvent {
    pub advanced: bool,
}
impl FromRawGameEvent for MvmWaveCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let advanced: bool = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("advanced".to_string()))?;
            bool::from_value(value.clone(), "advanced")?
        };
        Ok(MvmWaveCompleteEvent {
            advanced
        })
    }
}

#[derive(Debug)]
pub struct MvmMissionCompleteEvent {
    pub mission: String,
}
impl FromRawGameEvent for MvmMissionCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let mission: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("mission".to_string()))?;
            String::from_value(value.clone(), "mission")?
        };
        Ok(MvmMissionCompleteEvent {
            mission
        })
    }
}

#[derive(Debug)]
pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombResetByPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmBombResetByPlayerEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MvmBombAlarmTriggeredEvent {

}
impl FromRawGameEvent for MvmBombAlarmTriggeredEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmBombAlarmTriggeredEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmBombDeployResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombDeployResetByPlayerEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmBombDeployResetByPlayerEvent {
            player
        })
    }
}

#[derive(Debug)]
pub struct MvmWaveFailedEvent {

}
impl FromRawGameEvent for MvmWaveFailedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmWaveFailedEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmResetStatsEvent {

}
impl FromRawGameEvent for MvmResetStatsEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmResetStatsEvent {

        })
    }
}

#[derive(Debug)]
pub struct DamageResistedEvent {
    pub ent_index: u8,
}
impl FromRawGameEvent for DamageResistedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u8::from_value(value.clone(), "ent_index")?
        };
        Ok(DamageResistedEvent {
            ent_index
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
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        let marker_ent_index: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("marker_ent_index".to_string()))?;
            u16::from_value(value.clone(), "marker_ent_index")?
        };
        Ok(RevivePlayerNotifyEvent {
            ent_index,
            marker_ent_index
        })
    }
}

#[derive(Debug)]
pub struct RevivePlayerStoppedEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerStoppedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        Ok(RevivePlayerStoppedEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct RevivePlayerCompleteEvent {
    pub ent_index: u16,
}
impl FromRawGameEvent for RevivePlayerCompleteEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        Ok(RevivePlayerCompleteEvent {
            ent_index
        })
    }
}

#[derive(Debug)]
pub struct PlayerTurnedToGhostEvent {
    pub user_id: u16,
}
impl FromRawGameEvent for PlayerTurnedToGhostEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        Ok(PlayerTurnedToGhostEvent {
            user_id
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let damage: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("damage".to_string()))?;
            f32::from_value(value.clone(), "damage")?
        };
        Ok(MedigunShieldBlockedDamageEvent {
            user_id,
            damage
        })
    }
}

#[derive(Debug)]
pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl FromRawGameEvent for MvmAdvWaveCompleteNoGatesEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(MvmAdvWaveCompleteNoGatesEvent {
            index
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let currency: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(MvmSniperHeadshotCurrencyEvent {
            user_id,
            currency
        })
    }
}

#[derive(Debug)]
pub struct MvmMannhattanPitEvent {

}
impl FromRawGameEvent for MvmMannhattanPitEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmMannhattanPitEvent {

        })
    }
}

#[derive(Debug)]
pub struct FlagCarriedInDetectionZoneEvent {

}
impl FromRawGameEvent for FlagCarriedInDetectionZoneEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(FlagCarriedInDetectionZoneEvent {

        })
    }
}

#[derive(Debug)]
pub struct MvmAdvWaveKilledStunRadioEvent {

}
impl FromRawGameEvent for MvmAdvWaveKilledStunRadioEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(MvmAdvWaveKilledStunRadioEvent {

        })
    }
}

#[derive(Debug)]
pub struct PlayerDirectHitStunEvent {
    pub attacker: u16,
    pub victim: u16,
}
impl FromRawGameEvent for PlayerDirectHitStunEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let attacker: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let victim: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        Ok(PlayerDirectHitStunEvent {
            attacker,
            victim
        })
    }
}

#[derive(Debug)]
pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl FromRawGameEvent for MvmSentryBusterKilledEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let sentry_buster: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("sentry_buster".to_string()))?;
            u16::from_value(value.clone(), "sentry_buster")?
        };
        Ok(MvmSentryBusterKilledEvent {
            sentry_buster
        })
    }
}

#[derive(Debug)]
pub struct UpgradesFileChangedEvent {
    pub path: String,
}
impl FromRawGameEvent for UpgradesFileChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let path: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("path".to_string()))?;
            String::from_value(value.clone(), "path")?
        };
        Ok(UpgradesFileChangedEvent {
            path
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
        let points: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("points".to_string()))?;
            u16::from_value(value.clone(), "points")?
        };
        let team: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let method: u8 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("method".to_string()))?;
            u8::from_value(value.clone(), "method")?
        };
        Ok(RdTeamPointsChangedEvent {
            points,
            team,
            method
        })
    }
}

#[derive(Debug)]
pub struct RdRulesStateChangedEvent {

}
impl FromRawGameEvent for RdRulesStateChangedEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(RdRulesStateChangedEvent {

        })
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
        let user_id: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("user_id".to_string()))?;
            u16::from_value(value.clone(), "user_id")?
        };
        let victim_ent_index: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("victim_ent_index".to_string()))?;
            u32::from_value(value.clone(), "victim_ent_index")?
        };
        let inflictor_ent_index: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("inflictor_ent_index".to_string()))?;
            u32::from_value(value.clone(), "inflictor_ent_index")?
        };
        let attacker: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weapon_id: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("weapon_id".to_string()))?;
            u16::from_value(value.clone(), "weapon_id")?
        };
        let damage_bits: u32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("damage_bits".to_string()))?;
            u32::from_value(value.clone(), "damage_bits")?
        };
        let custom_kill: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("custom_kill".to_string()))?;
            u16::from_value(value.clone(), "custom_kill")?
        };
        let weapon_log_class_name: String = {
            let value = values.get(8).ok_or_else(|| ParseError::UnknownGameEvent("weapon_log_class_name".to_string()))?;
            String::from_value(value.clone(), "weapon_log_class_name")?
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
            weapon_log_class_name
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
        let ent_index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("ent_index".to_string()))?;
            u16::from_value(value.clone(), "ent_index")?
        };
        let impulse_x: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("impulse_x".to_string()))?;
            f32::from_value(value.clone(), "impulse_x")?
        };
        let impulse_y: f32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("impulse_y".to_string()))?;
            f32::from_value(value.clone(), "impulse_y")?
        };
        let impulse_z: f32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("impulse_z".to_string()))?;
            f32::from_value(value.clone(), "impulse_z")?
        };
        Ok(RdRobotImpactEvent {
            ent_index,
            impulse_x,
            impulse_y,
            impulse_z
        })
    }
}

#[derive(Debug)]
pub struct TeamPlayPreRoundTimeLeftEvent {
    pub time: u16,
}
impl FromRawGameEvent for TeamPlayPreRoundTimeLeftEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let time: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("time".to_string()))?;
            u16::from_value(value.clone(), "time")?
        };
        Ok(TeamPlayPreRoundTimeLeftEvent {
            time
        })
    }
}

#[derive(Debug)]
pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteDeployEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ParachuteDeployEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteHolsterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ParachuteHolsterEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl FromRawGameEvent for KillRefillsMeterEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(KillRefillsMeterEvent {
            index
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
        let winner: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("winner".to_string()))?;
            u16::from_value(value.clone(), "winner")?
        };
        let winner_rps: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("winner_rps".to_string()))?;
            u8::from_value(value.clone(), "winner_rps")?
        };
        let loser: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("loser".to_string()))?;
            u16::from_value(value.clone(), "loser")?
        };
        let loser_rps: u8 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("loser_rps".to_string()))?;
            u8::from_value(value.clone(), "loser_rps")?
        };
        Ok(RpsTauntEventEvent {
            winner,
            winner_rps,
            loser,
            loser_rps
        })
    }
}

#[derive(Debug)]
pub struct CongaKillEvent {
    pub index: u16,
}
impl FromRawGameEvent for CongaKillEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(CongaKillEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl FromRawGameEvent for PlayerInitialSpawnEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerInitialSpawnEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct CompetitiveVictoryEvent {

}
impl FromRawGameEvent for CompetitiveVictoryEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(CompetitiveVictoryEvent {

        })
    }
}

#[derive(Debug)]
pub struct CompetitiveSkillRatingUpdateEvent {
    pub index: u16,
    pub rating: u16,
    pub delta: u16,
}
impl FromRawGameEvent for CompetitiveSkillRatingUpdateEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let rating: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("rating".to_string()))?;
            u16::from_value(value.clone(), "rating")?
        };
        let delta: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("delta".to_string()))?;
            u16::from_value(value.clone(), "delta")?
        };
        Ok(CompetitiveSkillRatingUpdateEvent {
            index,
            rating,
            delta
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
        let team: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let kind: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("kind".to_string()))?;
            u8::from_value(value.clone(), "kind")?
        };
        Ok(MiniGameWinEvent {
            team,
            kind
        })
    }
}

#[derive(Debug)]
pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl FromRawGameEvent for SentryOnGoActiveEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(SentryOnGoActiveEvent {
            index
        })
    }
}

#[derive(Debug)]
pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl FromRawGameEvent for DuckXpLevelUpEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(DuckXpLevelUpEvent {
            level
        })
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
        let clients: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("clients".to_string()))?;
            u32::from_value(value.clone(), "clients")?
        };
        let slots: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("slots".to_string()))?;
            u32::from_value(value.clone(), "slots")?
        };
        let proxies: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("proxies".to_string()))?;
            u16::from_value(value.clone(), "proxies")?
        };
        let master: String = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("master".to_string()))?;
            String::from_value(value.clone(), "master")?
        };
        Ok(HLTVStatusEvent {
            clients,
            slots,
            proxies,
            master
        })
    }
}

#[derive(Debug)]
pub struct HLTVCameramanEvent {
    pub index: u16,
}
impl FromRawGameEvent for HLTVCameramanEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(HLTVCameramanEvent {
            index
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
        let index: u8 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let rank: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("rank".to_string()))?;
            f32::from_value(value.clone(), "rank")?
        };
        let target: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVRankCameraEvent {
            index,
            rank,
            target
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
        let index: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let rank: f32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("rank".to_string()))?;
            f32::from_value(value.clone(), "rank")?
        };
        let target: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVRankEntityEvent {
            index,
            rank,
            target
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
        let pos_x: u32 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("pos_x".to_string()))?;
            u32::from_value(value.clone(), "pos_x")?
        };
        let pos_y: u32 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("pos_y".to_string()))?;
            u32::from_value(value.clone(), "pos_y")?
        };
        let pos_z: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("pos_z".to_string()))?;
            u32::from_value(value.clone(), "pos_z")?
        };
        let theta: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("theta".to_string()))?;
            u16::from_value(value.clone(), "theta")?
        };
        let phi: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("phi".to_string()))?;
            u16::from_value(value.clone(), "phi")?
        };
        let offset: u16 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("offset".to_string()))?;
            u16::from_value(value.clone(), "offset")?
        };
        let fov: f32 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("fov".to_string()))?;
            f32::from_value(value.clone(), "fov")?
        };
        let target: u16 = {
            let value = values.get(7).ok_or_else(|| ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVFixedEvent {
            pos_x,
            pos_y,
            pos_z,
            theta,
            phi,
            offset,
            fov,
            target
        })
    }
}

#[derive(Debug)]
pub struct HLTVChaseEvent {
    pub target1: u16,
    pub target2: u16,
    pub distance: u16,
    pub theta: u16,
    pub phi: u16,
    pub inertia: u8,
    pub in_eye: u8,
}
impl FromRawGameEvent for HLTVChaseEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let target1: u16 = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("target1".to_string()))?;
            u16::from_value(value.clone(), "target1")?
        };
        let target2: u16 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("target2".to_string()))?;
            u16::from_value(value.clone(), "target2")?
        };
        let distance: u16 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("distance".to_string()))?;
            u16::from_value(value.clone(), "distance")?
        };
        let theta: u16 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("theta".to_string()))?;
            u16::from_value(value.clone(), "theta")?
        };
        let phi: u16 = {
            let value = values.get(4).ok_or_else(|| ParseError::UnknownGameEvent("phi".to_string()))?;
            u16::from_value(value.clone(), "phi")?
        };
        let inertia: u8 = {
            let value = values.get(5).ok_or_else(|| ParseError::UnknownGameEvent("inertia".to_string()))?;
            u8::from_value(value.clone(), "inertia")?
        };
        let in_eye: u8 = {
            let value = values.get(6).ok_or_else(|| ParseError::UnknownGameEvent("in_eye".to_string()))?;
            u8::from_value(value.clone(), "in_eye")?
        };
        Ok(HLTVChaseEvent {
            target1,
            target2,
            distance,
            theta,
            phi,
            inertia,
            in_eye
        })
    }
}

#[derive(Debug)]
pub struct HLTVMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVMessageEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVMessageEvent {
            text
        })
    }
}

#[derive(Debug)]
pub struct HLTVTitleEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVTitleEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVTitleEvent {
            text
        })
    }
}

#[derive(Debug)]
pub struct HLTVChatEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVChatEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVChatEvent {
            text
        })
    }
}

#[derive(Debug)]
pub struct ReplayStartRecordEvent {

}
impl FromRawGameEvent for ReplayStartRecordEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ReplayStartRecordEvent {

        })
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
        let sn: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("sn".to_string()))?;
            String::from_value(value.clone(), "sn")?
        };
        let di: u8 = {
            let value = values.get(1).ok_or_else(|| ParseError::UnknownGameEvent("di".to_string()))?;
            u8::from_value(value.clone(), "di")?
        };
        let cb: u32 = {
            let value = values.get(2).ok_or_else(|| ParseError::UnknownGameEvent("cb".to_string()))?;
            u32::from_value(value.clone(), "cb")?
        };
        let st: u32 = {
            let value = values.get(3).ok_or_else(|| ParseError::UnknownGameEvent("st".to_string()))?;
            u32::from_value(value.clone(), "st")?
        };
        Ok(ReplaySessionInfoEvent {
            sn,
            di,
            cb,
            st
        })
    }
}

#[derive(Debug)]
pub struct ReplayEndRecordEvent {

}
impl FromRawGameEvent for ReplayEndRecordEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ReplayEndRecordEvent {

        })
    }
}

#[derive(Debug)]
pub struct ReplayReplaysAvailableEvent {

}
impl FromRawGameEvent for ReplayReplaysAvailableEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {

        Ok(ReplayReplaysAvailableEvent {

        })
    }
}

#[derive(Debug)]
pub struct ReplayServerErrorEvent {
    pub error: String,
}
impl FromRawGameEvent for ReplayServerErrorEvent {
    fn from_raw_event(values: Vec<GameEventValue>) -> Result<Self> {
        let error: String = {
            let value = values.get(0).ok_or_else(|| ParseError::UnknownGameEvent("error".to_string()))?;
            String::from_value(value.clone(), "error")?
        };
        Ok(ReplayServerErrorEvent {
            error
        })
    }
}


#[derive(Debug)]
pub enum GameEvent {
    ServerSpawn(ServerSpawnEvent),
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
    GcConnected(GcConnectedEvent),
    ItemSchemaInitialized(ItemSchemaInitializedEvent),
    IntroFinish(IntroFinishEvent),
    IntroNextCamera(IntroNextCameraEvent),
    MmLobbyChat(MmLobbyChatEvent),
    MmLobbyMemberJoin(MmLobbyMemberJoinEvent),
    MmLobbyMemberLeave(MmLobbyMemberLeaveEvent),
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
    CompetitiveSkillRatingUpdate(CompetitiveSkillRatingUpdateEvent),
    MiniGameWin(MiniGameWinEvent),
    SentryOnGoActive(SentryOnGoActiveEvent),
    DuckXpLevelUp(DuckXpLevelUpEvent),
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
    ServerShutdown = 1,
    ServerCvar = 2,
    ServerMessage = 3,
    ServerAddBan = 4,
    ServerRemoveBan = 5,
    PlayerConnect = 6,
    PlayerConnectClient = 7,
    PlayerInfo = 8,
    PlayerDisconnect = 9,
    PlayerActivate = 10,
    PlayerSay = 11,
    ClientDisconnect = 12,
    ClientBeginConnect = 13,
    ClientConnected = 14,
    ClientFullConnect = 15,
    HostQuit = 16,
    TeamInfo = 17,
    TeamScore = 18,
    TeamPlayBroadcastAudio = 19,
    PlayerTeam = 20,
    PlayerClass = 21,
    PlayerDeath = 22,
    PlayerHurt = 23,
    PlayerChat = 24,
    PlayerScore = 25,
    PlayerSpawn = 26,
    PlayerShoot = 27,
    PlayerUse = 28,
    PlayerChangeName = 29,
    PlayerHintMessage = 30,
    BasePlayerTeleported = 31,
    GameInit = 32,
    GameNewMap = 33,
    GameStart = 34,
    GameEnd = 35,
    RoundStart = 36,
    RoundEnd = 37,
    GameMessage = 38,
    BreakBreakable = 39,
    BreakProp = 40,
    EntityKilled = 41,
    BonusUpdated = 42,
    AchievementEvent = 43,
    AchievementIncrement = 44,
    PhysgunPickup = 45,
    FlareIgniteNpc = 46,
    HelicopterGrenadePuntMiss = 47,
    UserDataDownloaded = 48,
    RagdollDissolved = 49,
    HLTVChangedMode = 50,
    HLTVChangedTarget = 51,
    VoteEnded = 52,
    VoteStarted = 53,
    VoteChanged = 54,
    VotePassed = 55,
    VoteFailed = 56,
    VoteCast = 57,
    VoteOptions = 58,
    ReplaySaved = 59,
    EnteredPerformanceMode = 60,
    BrowseReplays = 61,
    ReplayYoutubeStats = 62,
    InventoryUpdated = 63,
    CartUpdated = 64,
    StorePriceSheetUpdated = 65,
    GcConnected = 66,
    ItemSchemaInitialized = 67,
    IntroFinish = 68,
    IntroNextCamera = 69,
    MmLobbyChat = 70,
    MmLobbyMemberJoin = 71,
    MmLobbyMemberLeave = 72,
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
    LocalPlayerBecameObserver = 147,
    PlayerIgnitedInv = 148,
    PlayerIgnited = 149,
    PlayerExtinguished = 150,
    PlayerTeleported = 151,
    PlayerHealedMedicCall = 152,
    LocalPlayerChargeReady = 153,
    LocalPlayerWindDown = 154,
    PlayerInvulned = 155,
    EscortSpeed = 156,
    EscortProgress = 157,
    EscortRecede = 158,
    GameUIActivated = 159,
    GameUIHidden = 160,
    PlayerEscortScore = 161,
    PlayerHealOnHit = 162,
    PlayerStealSandvich = 163,
    ShowClassLayout = 164,
    ShowVsPanel = 165,
    PlayerDamaged = 166,
    ArenaPlayerNotification = 167,
    ArenaMatchMaxStreak = 168,
    ArenaRoundStart = 169,
    ArenaWinPanel = 170,
    PveWinPanel = 171,
    AirDash = 172,
    Landed = 173,
    PlayerDamageDodged = 174,
    PlayerStunned = 175,
    ScoutGrandSlam = 176,
    ScoutSlamdollLanded = 177,
    ArrowImpact = 178,
    PlayerJarated = 179,
    PlayerJaratedFade = 180,
    PlayerShieldBlocked = 181,
    PlayerPinned = 182,
    PlayerHealedByMedic = 183,
    PlayerSappedObject = 184,
    ItemFound = 185,
    ShowAnnotation = 186,
    HideAnnotation = 187,
    PostInventoryApplication = 188,
    ControlPointUnlockUpdated = 189,
    DeployBuffBanner = 190,
    PlayerBuff = 191,
    MedicDeath = 192,
    OvertimeNag = 193,
    TeamsChanged = 194,
    HalloweenPumpkinGrab = 195,
    RocketJump = 196,
    RocketJumpLanded = 197,
    StickyJump = 198,
    StickyJumpLanded = 199,
    MedicDefended = 200,
    LocalPlayerHealed = 201,
    PlayerDestroyedPipeBomb = 202,
    ObjectDeflected = 203,
    PlayerMvp = 204,
    RaidSpawnMob = 205,
    RaidSpawnSquad = 206,
    NavBlocked = 207,
    PathTrackPassed = 208,
    NumCappersChanged = 209,
    PlayerRegenerate = 210,
    UpdateStatusItem = 211,
    StatsResetRound = 212,
    ScoreStatsAccumulatedUpdate = 213,
    ScoreStatsAccumulatedReset = 214,
    AchievementEarnedLocal = 215,
    PlayerHealed = 216,
    BuildingHealed = 217,
    ItemPickup = 218,
    DuelStatus = 219,
    FishNotice = 220,
    FishNoticeArm = 221,
    ThrowableHit = 222,
    PumpkinLordSummoned = 223,
    PumpkinLordKilled = 224,
    MerasmusSummoned = 225,
    MerasmusKilled = 226,
    MerasmusEscapeWarning = 227,
    MerasmusEscaped = 228,
    EyeballBossSummoned = 229,
    EyeballBossStunned = 230,
    EyeballBossKilled = 231,
    EyeballBossKiller = 232,
    EyeballBossEscapeImminent = 233,
    EyeballBossEscaped = 234,
    NpcHurt = 235,
    ControlPointTimerUpdated = 236,
    PlayerHighFiveStart = 237,
    PlayerHighFiveCancel = 238,
    PlayerHighFiveSuccess = 239,
    PlayerBonusPoints = 240,
    PlayerUpgraded = 241,
    PlayerBuyback = 242,
    PlayerUsedPowerUpBottle = 243,
    ChristmasGiftGrab = 244,
    PlayerKilledAchievementZone = 245,
    PartyUpdated = 246,
    LobbyUpdated = 247,
    MvmMissionUpdate = 248,
    RecalculateHolidays = 249,
    PlayerCurrencyChanged = 250,
    DoomsdayRocketOpen = 251,
    RemoveNemesisRelationships = 252,
    MvmCreditBonusWave = 253,
    MvmCreditBonusAll = 254,
    MvmCreditBonusAllAdvanced = 255,
    MvmQuickSentryUpgrade = 256,
    MvmTankDestroyedByPlayers = 257,
    MvmKillRobotDeliveringBomb = 258,
    MvmPickupCurrency = 259,
    MvmBombCarrierKilled = 260,
    MvmSentryBusterDetonate = 261,
    MvmScoutMarkedForDeath = 262,
    MvmMedicPowerUpShared = 263,
    MvmBeginWave = 264,
    MvmWaveComplete = 265,
    MvmMissionComplete = 266,
    MvmBombResetByPlayer = 267,
    MvmBombAlarmTriggered = 268,
    MvmBombDeployResetByPlayer = 269,
    MvmWaveFailed = 270,
    MvmResetStats = 271,
    DamageResisted = 272,
    RevivePlayerNotify = 273,
    RevivePlayerStopped = 274,
    RevivePlayerComplete = 275,
    PlayerTurnedToGhost = 276,
    MedigunShieldBlockedDamage = 277,
    MvmAdvWaveCompleteNoGates = 278,
    MvmSniperHeadshotCurrency = 279,
    MvmMannhattanPit = 280,
    FlagCarriedInDetectionZone = 281,
    MvmAdvWaveKilledStunRadio = 282,
    PlayerDirectHitStun = 283,
    MvmSentryBusterKilled = 284,
    UpgradesFileChanged = 285,
    RdTeamPointsChanged = 286,
    RdRulesStateChanged = 287,
    RdRobotKilled = 288,
    RdRobotImpact = 289,
    TeamPlayPreRoundTimeLeft = 290,
    ParachuteDeploy = 291,
    ParachuteHolster = 292,
    KillRefillsMeter = 293,
    RpsTauntEvent = 294,
    CongaKill = 295,
    PlayerInitialSpawn = 296,
    CompetitiveVictory = 297,
    CompetitiveSkillRatingUpdate = 298,
    MiniGameWin = 299,
    SentryOnGoActive = 300,
    DuckXpLevelUp = 301,
    HLTVStatus = 302,
    HLTVCameraman = 303,
    HLTVRankCamera = 304,
    HLTVRankEntity = 305,
    HLTVFixed = 306,
    HLTVChase = 307,
    HLTVMessage = 308,
    HLTVTitle = 309,
    HLTVChat = 310,
    ReplayStartRecord = 311,
    ReplaySessionInfo = 312,
    ReplayEndRecord = 313,
    ReplayReplaysAvailable = 314,
    ReplayServerError = 315,

    Unknown,
}

impl GameEventType {
    pub fn from_type_name(name: &str) -> Self {
        match name {
            "server_spawn" => GameEventType::ServerSpawn,
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
            "gc_connected" => GameEventType::GcConnected,
            "item_schema_initialized" => GameEventType::ItemSchemaInitialized,
            "intro_finish" => GameEventType::IntroFinish,
            "intro_nextcamera" => GameEventType::IntroNextCamera,
            "mm_lobby_chat" => GameEventType::MmLobbyChat,
            "mm_lobby_member_join" => GameEventType::MmLobbyMemberJoin,
            "mm_lobby_member_leave" => GameEventType::MmLobbyMemberLeave,
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
            "competitive_skillrating_update" => GameEventType::CompetitiveSkillRatingUpdate,
            "minigame_win" => GameEventType::MiniGameWin,
            "sentry_on_go_active" => GameEventType::SentryOnGoActive,
            "duck_xp_level_up" => GameEventType::DuckXpLevelUp,
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
            _ => GameEventType::Unknown
        }
    }
}


impl GameEvent {
    pub fn from_raw_event(event: RawGameEvent) -> Result<Self> {
        Ok(match event.event_type {
            GameEventType::ServerSpawn => GameEvent::ServerSpawn(ServerSpawnEvent::from_raw_event(event.values)?),
            GameEventType::ServerShutdown => GameEvent::ServerShutdown(ServerShutdownEvent::from_raw_event(event.values)?),
            GameEventType::ServerCvar => GameEvent::ServerCvar(ServerCvarEvent::from_raw_event(event.values)?),
            GameEventType::ServerMessage => GameEvent::ServerMessage(ServerMessageEvent::from_raw_event(event.values)?),
            GameEventType::ServerAddBan => GameEvent::ServerAddBan(ServerAddBanEvent::from_raw_event(event.values)?),
            GameEventType::ServerRemoveBan => GameEvent::ServerRemoveBan(ServerRemoveBanEvent::from_raw_event(event.values)?),
            GameEventType::PlayerConnect => GameEvent::PlayerConnect(PlayerConnectEvent::from_raw_event(event.values)?),
            GameEventType::PlayerConnectClient => GameEvent::PlayerConnectClient(PlayerConnectClientEvent::from_raw_event(event.values)?),
            GameEventType::PlayerInfo => GameEvent::PlayerInfo(PlayerInfoEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDisconnect => GameEvent::PlayerDisconnect(PlayerDisconnectEvent::from_raw_event(event.values)?),
            GameEventType::PlayerActivate => GameEvent::PlayerActivate(PlayerActivateEvent::from_raw_event(event.values)?),
            GameEventType::PlayerSay => GameEvent::PlayerSay(PlayerSayEvent::from_raw_event(event.values)?),
            GameEventType::ClientDisconnect => GameEvent::ClientDisconnect(ClientDisconnectEvent::from_raw_event(event.values)?),
            GameEventType::ClientBeginConnect => GameEvent::ClientBeginConnect(ClientBeginConnectEvent::from_raw_event(event.values)?),
            GameEventType::ClientConnected => GameEvent::ClientConnected(ClientConnectedEvent::from_raw_event(event.values)?),
            GameEventType::ClientFullConnect => GameEvent::ClientFullConnect(ClientFullConnectEvent::from_raw_event(event.values)?),
            GameEventType::HostQuit => GameEvent::HostQuit(HostQuitEvent::from_raw_event(event.values)?),
            GameEventType::TeamInfo => GameEvent::TeamInfo(TeamInfoEvent::from_raw_event(event.values)?),
            GameEventType::TeamScore => GameEvent::TeamScore(TeamScoreEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayBroadcastAudio => GameEvent::TeamPlayBroadcastAudio(TeamPlayBroadcastAudioEvent::from_raw_event(event.values)?),
            GameEventType::PlayerTeam => GameEvent::PlayerTeam(PlayerTeamEvent::from_raw_event(event.values)?),
            GameEventType::PlayerClass => GameEvent::PlayerClass(PlayerClassEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDeath => GameEvent::PlayerDeath(PlayerDeathEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHurt => GameEvent::PlayerHurt(PlayerHurtEvent::from_raw_event(event.values)?),
            GameEventType::PlayerChat => GameEvent::PlayerChat(PlayerChatEvent::from_raw_event(event.values)?),
            GameEventType::PlayerScore => GameEvent::PlayerScore(PlayerScoreEvent::from_raw_event(event.values)?),
            GameEventType::PlayerSpawn => GameEvent::PlayerSpawn(PlayerSpawnEvent::from_raw_event(event.values)?),
            GameEventType::PlayerShoot => GameEvent::PlayerShoot(PlayerShootEvent::from_raw_event(event.values)?),
            GameEventType::PlayerUse => GameEvent::PlayerUse(PlayerUseEvent::from_raw_event(event.values)?),
            GameEventType::PlayerChangeName => GameEvent::PlayerChangeName(PlayerChangeNameEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHintMessage => GameEvent::PlayerHintMessage(PlayerHintMessageEvent::from_raw_event(event.values)?),
            GameEventType::BasePlayerTeleported => GameEvent::BasePlayerTeleported(BasePlayerTeleportedEvent::from_raw_event(event.values)?),
            GameEventType::GameInit => GameEvent::GameInit(GameInitEvent::from_raw_event(event.values)?),
            GameEventType::GameNewMap => GameEvent::GameNewMap(GameNewMapEvent::from_raw_event(event.values)?),
            GameEventType::GameStart => GameEvent::GameStart(GameStartEvent::from_raw_event(event.values)?),
            GameEventType::GameEnd => GameEvent::GameEnd(GameEndEvent::from_raw_event(event.values)?),
            GameEventType::RoundStart => GameEvent::RoundStart(RoundStartEvent::from_raw_event(event.values)?),
            GameEventType::RoundEnd => GameEvent::RoundEnd(RoundEndEvent::from_raw_event(event.values)?),
            GameEventType::GameMessage => GameEvent::GameMessage(GameMessageEvent::from_raw_event(event.values)?),
            GameEventType::BreakBreakable => GameEvent::BreakBreakable(BreakBreakableEvent::from_raw_event(event.values)?),
            GameEventType::BreakProp => GameEvent::BreakProp(BreakPropEvent::from_raw_event(event.values)?),
            GameEventType::EntityKilled => GameEvent::EntityKilled(EntityKilledEvent::from_raw_event(event.values)?),
            GameEventType::BonusUpdated => GameEvent::BonusUpdated(BonusUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::AchievementEvent => GameEvent::AchievementEvent(AchievementEventEvent::from_raw_event(event.values)?),
            GameEventType::AchievementIncrement => GameEvent::AchievementIncrement(AchievementIncrementEvent::from_raw_event(event.values)?),
            GameEventType::PhysgunPickup => GameEvent::PhysgunPickup(PhysgunPickupEvent::from_raw_event(event.values)?),
            GameEventType::FlareIgniteNpc => GameEvent::FlareIgniteNpc(FlareIgniteNpcEvent::from_raw_event(event.values)?),
            GameEventType::HelicopterGrenadePuntMiss => GameEvent::HelicopterGrenadePuntMiss(HelicopterGrenadePuntMissEvent::from_raw_event(event.values)?),
            GameEventType::UserDataDownloaded => GameEvent::UserDataDownloaded(UserDataDownloadedEvent::from_raw_event(event.values)?),
            GameEventType::RagdollDissolved => GameEvent::RagdollDissolved(RagdollDissolvedEvent::from_raw_event(event.values)?),
            GameEventType::HLTVChangedMode => GameEvent::HLTVChangedMode(HLTVChangedModeEvent::from_raw_event(event.values)?),
            GameEventType::HLTVChangedTarget => GameEvent::HLTVChangedTarget(HLTVChangedTargetEvent::from_raw_event(event.values)?),
            GameEventType::VoteEnded => GameEvent::VoteEnded(VoteEndedEvent::from_raw_event(event.values)?),
            GameEventType::VoteStarted => GameEvent::VoteStarted(VoteStartedEvent::from_raw_event(event.values)?),
            GameEventType::VoteChanged => GameEvent::VoteChanged(VoteChangedEvent::from_raw_event(event.values)?),
            GameEventType::VotePassed => GameEvent::VotePassed(VotePassedEvent::from_raw_event(event.values)?),
            GameEventType::VoteFailed => GameEvent::VoteFailed(VoteFailedEvent::from_raw_event(event.values)?),
            GameEventType::VoteCast => GameEvent::VoteCast(VoteCastEvent::from_raw_event(event.values)?),
            GameEventType::VoteOptions => GameEvent::VoteOptions(VoteOptionsEvent::from_raw_event(event.values)?),
            GameEventType::ReplaySaved => GameEvent::ReplaySaved(ReplaySavedEvent::from_raw_event(event.values)?),
            GameEventType::EnteredPerformanceMode => GameEvent::EnteredPerformanceMode(EnteredPerformanceModeEvent::from_raw_event(event.values)?),
            GameEventType::BrowseReplays => GameEvent::BrowseReplays(BrowseReplaysEvent::from_raw_event(event.values)?),
            GameEventType::ReplayYoutubeStats => GameEvent::ReplayYoutubeStats(ReplayYoutubeStatsEvent::from_raw_event(event.values)?),
            GameEventType::InventoryUpdated => GameEvent::InventoryUpdated(InventoryUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::CartUpdated => GameEvent::CartUpdated(CartUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::StorePriceSheetUpdated => GameEvent::StorePriceSheetUpdated(StorePriceSheetUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::GcConnected => GameEvent::GcConnected(GcConnectedEvent::from_raw_event(event.values)?),
            GameEventType::ItemSchemaInitialized => GameEvent::ItemSchemaInitialized(ItemSchemaInitializedEvent::from_raw_event(event.values)?),
            GameEventType::IntroFinish => GameEvent::IntroFinish(IntroFinishEvent::from_raw_event(event.values)?),
            GameEventType::IntroNextCamera => GameEvent::IntroNextCamera(IntroNextCameraEvent::from_raw_event(event.values)?),
            GameEventType::MmLobbyChat => GameEvent::MmLobbyChat(MmLobbyChatEvent::from_raw_event(event.values)?),
            GameEventType::MmLobbyMemberJoin => GameEvent::MmLobbyMemberJoin(MmLobbyMemberJoinEvent::from_raw_event(event.values)?),
            GameEventType::MmLobbyMemberLeave => GameEvent::MmLobbyMemberLeave(MmLobbyMemberLeaveEvent::from_raw_event(event.values)?),
            GameEventType::PlayerChangeClass => GameEvent::PlayerChangeClass(PlayerChangeClassEvent::from_raw_event(event.values)?),
            GameEventType::TfMapTimeRemaining => GameEvent::TfMapTimeRemaining(TfMapTimeRemainingEvent::from_raw_event(event.values)?),
            GameEventType::TfGameOver => GameEvent::TfGameOver(TfGameOverEvent::from_raw_event(event.values)?),
            GameEventType::CtfFlagCaptured => GameEvent::CtfFlagCaptured(CtfFlagCapturedEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointInitialized => GameEvent::ControlPointInitialized(ControlPointInitializedEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointUpdateImages => GameEvent::ControlPointUpdateImages(ControlPointUpdateImagesEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointUpdateLayout => GameEvent::ControlPointUpdateLayout(ControlPointUpdateLayoutEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointUpdateCapping => GameEvent::ControlPointUpdateCapping(ControlPointUpdateCappingEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointUpdateOwner => GameEvent::ControlPointUpdateOwner(ControlPointUpdateOwnerEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointStartTouch => GameEvent::ControlPointStartTouch(ControlPointStartTouchEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointEndTouch => GameEvent::ControlPointEndTouch(ControlPointEndTouchEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointPulseElement => GameEvent::ControlPointPulseElement(ControlPointPulseElementEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointFakeCapture => GameEvent::ControlPointFakeCapture(ControlPointFakeCaptureEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointFakeCaptureMultiplier => GameEvent::ControlPointFakeCaptureMultiplier(ControlPointFakeCaptureMultiplierEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundSelected => GameEvent::TeamPlayRoundSelected(TeamPlayRoundSelectedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundStart => GameEvent::TeamPlayRoundStart(TeamPlayRoundStartEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundActive => GameEvent::TeamPlayRoundActive(TeamPlayRoundActiveEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayWaitingBegins => GameEvent::TeamPlayWaitingBegins(TeamPlayWaitingBeginsEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayWaitingEnds => GameEvent::TeamPlayWaitingEnds(TeamPlayWaitingEndsEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayWaitingAboutToEnd => GameEvent::TeamPlayWaitingAboutToEnd(TeamPlayWaitingAboutToEndEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRestartRound => GameEvent::TeamPlayRestartRound(TeamPlayRestartRoundEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayReadyRestart => GameEvent::TeamPlayReadyRestart(TeamPlayReadyRestartEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundRestartSeconds => GameEvent::TeamPlayRoundRestartSeconds(TeamPlayRoundRestartSecondsEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayTeamReady => GameEvent::TeamPlayTeamReady(TeamPlayTeamReadyEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundWin => GameEvent::TeamPlayRoundWin(TeamPlayRoundWinEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayUpdateTimer => GameEvent::TeamPlayUpdateTimer(TeamPlayUpdateTimerEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayRoundStalemate => GameEvent::TeamPlayRoundStalemate(TeamPlayRoundStalemateEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayOvertimeBegin => GameEvent::TeamPlayOvertimeBegin(TeamPlayOvertimeBeginEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayOvertimeEnd => GameEvent::TeamPlayOvertimeEnd(TeamPlayOvertimeEndEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlaySuddenDeathBegin => GameEvent::TeamPlaySuddenDeathBegin(TeamPlaySuddenDeathBeginEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlaySuddenDeathEnd => GameEvent::TeamPlaySuddenDeathEnd(TeamPlaySuddenDeathEndEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayGameOver => GameEvent::TeamPlayGameOver(TeamPlayGameOverEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayMapTimeRemaining => GameEvent::TeamPlayMapTimeRemaining(TeamPlayMapTimeRemainingEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayTimerFlash => GameEvent::TeamPlayTimerFlash(TeamPlayTimerFlashEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayTimerTimeAdded => GameEvent::TeamPlayTimerTimeAdded(TeamPlayTimerTimeAddedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayPointStartCapture => GameEvent::TeamPlayPointStartCapture(TeamPlayPointStartCaptureEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayPointCaptured => GameEvent::TeamPlayPointCaptured(TeamPlayPointCapturedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayPointLocked => GameEvent::TeamPlayPointLocked(TeamPlayPointLockedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayPointUnlocked => GameEvent::TeamPlayPointUnlocked(TeamPlayPointUnlockedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayCaptureBroken => GameEvent::TeamPlayCaptureBroken(TeamPlayCaptureBrokenEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayCaptureBlocked => GameEvent::TeamPlayCaptureBlocked(TeamPlayCaptureBlockedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayFlagEvent => GameEvent::TeamPlayFlagEvent(TeamPlayFlagEventEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayWinPanel => GameEvent::TeamPlayWinPanel(TeamPlayWinPanelEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayTeamBalancedPlayer => GameEvent::TeamPlayTeamBalancedPlayer(TeamPlayTeamBalancedPlayerEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlaySetupFinished => GameEvent::TeamPlaySetupFinished(TeamPlaySetupFinishedEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayAlert => GameEvent::TeamPlayAlert(TeamPlayAlertEvent::from_raw_event(event.values)?),
            GameEventType::TrainingComplete => GameEvent::TrainingComplete(TrainingCompleteEvent::from_raw_event(event.values)?),
            GameEventType::ShowFreezePanel => GameEvent::ShowFreezePanel(ShowFreezePanelEvent::from_raw_event(event.values)?),
            GameEventType::HideFreezePanel => GameEvent::HideFreezePanel(HideFreezePanelEvent::from_raw_event(event.values)?),
            GameEventType::FreezeCamStarted => GameEvent::FreezeCamStarted(FreezeCamStartedEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerChangeTeam => GameEvent::LocalPlayerChangeTeam(LocalPlayerChangeTeamEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerScoreChanged => GameEvent::LocalPlayerScoreChanged(LocalPlayerScoreChangedEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerChangeClass => GameEvent::LocalPlayerChangeClass(LocalPlayerChangeClassEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerRespawn => GameEvent::LocalPlayerRespawn(LocalPlayerRespawnEvent::from_raw_event(event.values)?),
            GameEventType::BuildingInfoChanged => GameEvent::BuildingInfoChanged(BuildingInfoChangedEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerChangeDisguise => GameEvent::LocalPlayerChangeDisguise(LocalPlayerChangeDisguiseEvent::from_raw_event(event.values)?),
            GameEventType::PlayerAccountChanged => GameEvent::PlayerAccountChanged(PlayerAccountChangedEvent::from_raw_event(event.values)?),
            GameEventType::SpyPdaReset => GameEvent::SpyPdaReset(SpyPdaResetEvent::from_raw_event(event.values)?),
            GameEventType::FlagStatusUpdate => GameEvent::FlagStatusUpdate(FlagStatusUpdateEvent::from_raw_event(event.values)?),
            GameEventType::PlayerStatsUpdated => GameEvent::PlayerStatsUpdated(PlayerStatsUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::PlayingCommentary => GameEvent::PlayingCommentary(PlayingCommentaryEvent::from_raw_event(event.values)?),
            GameEventType::PlayerChargeDeployed => GameEvent::PlayerChargeDeployed(PlayerChargeDeployedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerBuiltObject => GameEvent::PlayerBuiltObject(PlayerBuiltObjectEvent::from_raw_event(event.values)?),
            GameEventType::PlayerUpgradedObject => GameEvent::PlayerUpgradedObject(PlayerUpgradedObjectEvent::from_raw_event(event.values)?),
            GameEventType::PlayerCarryObject => GameEvent::PlayerCarryObject(PlayerCarryObjectEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDropObject => GameEvent::PlayerDropObject(PlayerDropObjectEvent::from_raw_event(event.values)?),
            GameEventType::ObjectRemoved => GameEvent::ObjectRemoved(ObjectRemovedEvent::from_raw_event(event.values)?),
            GameEventType::ObjectDestroyed => GameEvent::ObjectDestroyed(ObjectDestroyedEvent::from_raw_event(event.values)?),
            GameEventType::ObjectDetonated => GameEvent::ObjectDetonated(ObjectDetonatedEvent::from_raw_event(event.values)?),
            GameEventType::AchievementEarned => GameEvent::AchievementEarned(AchievementEarnedEvent::from_raw_event(event.values)?),
            GameEventType::SpecTargetUpdated => GameEvent::SpecTargetUpdated(SpecTargetUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::TournamentStateUpdate => GameEvent::TournamentStateUpdate(TournamentStateUpdateEvent::from_raw_event(event.values)?),
            GameEventType::TournamentEnableCountdown => GameEvent::TournamentEnableCountdown(TournamentEnableCountdownEvent::from_raw_event(event.values)?),
            GameEventType::PlayerCalledForMedic => GameEvent::PlayerCalledForMedic(PlayerCalledForMedicEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerBecameObserver => GameEvent::LocalPlayerBecameObserver(LocalPlayerBecameObserverEvent::from_raw_event(event.values)?),
            GameEventType::PlayerIgnitedInv => GameEvent::PlayerIgnitedInv(PlayerIgnitedInvEvent::from_raw_event(event.values)?),
            GameEventType::PlayerIgnited => GameEvent::PlayerIgnited(PlayerIgnitedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerExtinguished => GameEvent::PlayerExtinguished(PlayerExtinguishedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerTeleported => GameEvent::PlayerTeleported(PlayerTeleportedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHealedMedicCall => GameEvent::PlayerHealedMedicCall(PlayerHealedMedicCallEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerChargeReady => GameEvent::LocalPlayerChargeReady(LocalPlayerChargeReadyEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerWindDown => GameEvent::LocalPlayerWindDown(LocalPlayerWindDownEvent::from_raw_event(event.values)?),
            GameEventType::PlayerInvulned => GameEvent::PlayerInvulned(PlayerInvulnedEvent::from_raw_event(event.values)?),
            GameEventType::EscortSpeed => GameEvent::EscortSpeed(EscortSpeedEvent::from_raw_event(event.values)?),
            GameEventType::EscortProgress => GameEvent::EscortProgress(EscortProgressEvent::from_raw_event(event.values)?),
            GameEventType::EscortRecede => GameEvent::EscortRecede(EscortRecedeEvent::from_raw_event(event.values)?),
            GameEventType::GameUIActivated => GameEvent::GameUIActivated(GameUIActivatedEvent::from_raw_event(event.values)?),
            GameEventType::GameUIHidden => GameEvent::GameUIHidden(GameUIHiddenEvent::from_raw_event(event.values)?),
            GameEventType::PlayerEscortScore => GameEvent::PlayerEscortScore(PlayerEscortScoreEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHealOnHit => GameEvent::PlayerHealOnHit(PlayerHealOnHitEvent::from_raw_event(event.values)?),
            GameEventType::PlayerStealSandvich => GameEvent::PlayerStealSandvich(PlayerStealSandvichEvent::from_raw_event(event.values)?),
            GameEventType::ShowClassLayout => GameEvent::ShowClassLayout(ShowClassLayoutEvent::from_raw_event(event.values)?),
            GameEventType::ShowVsPanel => GameEvent::ShowVsPanel(ShowVsPanelEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDamaged => GameEvent::PlayerDamaged(PlayerDamagedEvent::from_raw_event(event.values)?),
            GameEventType::ArenaPlayerNotification => GameEvent::ArenaPlayerNotification(ArenaPlayerNotificationEvent::from_raw_event(event.values)?),
            GameEventType::ArenaMatchMaxStreak => GameEvent::ArenaMatchMaxStreak(ArenaMatchMaxStreakEvent::from_raw_event(event.values)?),
            GameEventType::ArenaRoundStart => GameEvent::ArenaRoundStart(ArenaRoundStartEvent::from_raw_event(event.values)?),
            GameEventType::ArenaWinPanel => GameEvent::ArenaWinPanel(ArenaWinPanelEvent::from_raw_event(event.values)?),
            GameEventType::PveWinPanel => GameEvent::PveWinPanel(PveWinPanelEvent::from_raw_event(event.values)?),
            GameEventType::AirDash => GameEvent::AirDash(AirDashEvent::from_raw_event(event.values)?),
            GameEventType::Landed => GameEvent::Landed(LandedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDamageDodged => GameEvent::PlayerDamageDodged(PlayerDamageDodgedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerStunned => GameEvent::PlayerStunned(PlayerStunnedEvent::from_raw_event(event.values)?),
            GameEventType::ScoutGrandSlam => GameEvent::ScoutGrandSlam(ScoutGrandSlamEvent::from_raw_event(event.values)?),
            GameEventType::ScoutSlamdollLanded => GameEvent::ScoutSlamdollLanded(ScoutSlamdollLandedEvent::from_raw_event(event.values)?),
            GameEventType::ArrowImpact => GameEvent::ArrowImpact(ArrowImpactEvent::from_raw_event(event.values)?),
            GameEventType::PlayerJarated => GameEvent::PlayerJarated(PlayerJaratedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerJaratedFade => GameEvent::PlayerJaratedFade(PlayerJaratedFadeEvent::from_raw_event(event.values)?),
            GameEventType::PlayerShieldBlocked => GameEvent::PlayerShieldBlocked(PlayerShieldBlockedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerPinned => GameEvent::PlayerPinned(PlayerPinnedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHealedByMedic => GameEvent::PlayerHealedByMedic(PlayerHealedByMedicEvent::from_raw_event(event.values)?),
            GameEventType::PlayerSappedObject => GameEvent::PlayerSappedObject(PlayerSappedObjectEvent::from_raw_event(event.values)?),
            GameEventType::ItemFound => GameEvent::ItemFound(ItemFoundEvent::from_raw_event(event.values)?),
            GameEventType::ShowAnnotation => GameEvent::ShowAnnotation(ShowAnnotationEvent::from_raw_event(event.values)?),
            GameEventType::HideAnnotation => GameEvent::HideAnnotation(HideAnnotationEvent::from_raw_event(event.values)?),
            GameEventType::PostInventoryApplication => GameEvent::PostInventoryApplication(PostInventoryApplicationEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointUnlockUpdated => GameEvent::ControlPointUnlockUpdated(ControlPointUnlockUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::DeployBuffBanner => GameEvent::DeployBuffBanner(DeployBuffBannerEvent::from_raw_event(event.values)?),
            GameEventType::PlayerBuff => GameEvent::PlayerBuff(PlayerBuffEvent::from_raw_event(event.values)?),
            GameEventType::MedicDeath => GameEvent::MedicDeath(MedicDeathEvent::from_raw_event(event.values)?),
            GameEventType::OvertimeNag => GameEvent::OvertimeNag(OvertimeNagEvent::from_raw_event(event.values)?),
            GameEventType::TeamsChanged => GameEvent::TeamsChanged(TeamsChangedEvent::from_raw_event(event.values)?),
            GameEventType::HalloweenPumpkinGrab => GameEvent::HalloweenPumpkinGrab(HalloweenPumpkinGrabEvent::from_raw_event(event.values)?),
            GameEventType::RocketJump => GameEvent::RocketJump(RocketJumpEvent::from_raw_event(event.values)?),
            GameEventType::RocketJumpLanded => GameEvent::RocketJumpLanded(RocketJumpLandedEvent::from_raw_event(event.values)?),
            GameEventType::StickyJump => GameEvent::StickyJump(StickyJumpEvent::from_raw_event(event.values)?),
            GameEventType::StickyJumpLanded => GameEvent::StickyJumpLanded(StickyJumpLandedEvent::from_raw_event(event.values)?),
            GameEventType::MedicDefended => GameEvent::MedicDefended(MedicDefendedEvent::from_raw_event(event.values)?),
            GameEventType::LocalPlayerHealed => GameEvent::LocalPlayerHealed(LocalPlayerHealedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDestroyedPipeBomb => GameEvent::PlayerDestroyedPipeBomb(PlayerDestroyedPipeBombEvent::from_raw_event(event.values)?),
            GameEventType::ObjectDeflected => GameEvent::ObjectDeflected(ObjectDeflectedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerMvp => GameEvent::PlayerMvp(PlayerMvpEvent::from_raw_event(event.values)?),
            GameEventType::RaidSpawnMob => GameEvent::RaidSpawnMob(RaidSpawnMobEvent::from_raw_event(event.values)?),
            GameEventType::RaidSpawnSquad => GameEvent::RaidSpawnSquad(RaidSpawnSquadEvent::from_raw_event(event.values)?),
            GameEventType::NavBlocked => GameEvent::NavBlocked(NavBlockedEvent::from_raw_event(event.values)?),
            GameEventType::PathTrackPassed => GameEvent::PathTrackPassed(PathTrackPassedEvent::from_raw_event(event.values)?),
            GameEventType::NumCappersChanged => GameEvent::NumCappersChanged(NumCappersChangedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerRegenerate => GameEvent::PlayerRegenerate(PlayerRegenerateEvent::from_raw_event(event.values)?),
            GameEventType::UpdateStatusItem => GameEvent::UpdateStatusItem(UpdateStatusItemEvent::from_raw_event(event.values)?),
            GameEventType::StatsResetRound => GameEvent::StatsResetRound(StatsResetRoundEvent::from_raw_event(event.values)?),
            GameEventType::ScoreStatsAccumulatedUpdate => GameEvent::ScoreStatsAccumulatedUpdate(ScoreStatsAccumulatedUpdateEvent::from_raw_event(event.values)?),
            GameEventType::ScoreStatsAccumulatedReset => GameEvent::ScoreStatsAccumulatedReset(ScoreStatsAccumulatedResetEvent::from_raw_event(event.values)?),
            GameEventType::AchievementEarnedLocal => GameEvent::AchievementEarnedLocal(AchievementEarnedLocalEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHealed => GameEvent::PlayerHealed(PlayerHealedEvent::from_raw_event(event.values)?),
            GameEventType::BuildingHealed => GameEvent::BuildingHealed(BuildingHealedEvent::from_raw_event(event.values)?),
            GameEventType::ItemPickup => GameEvent::ItemPickup(ItemPickupEvent::from_raw_event(event.values)?),
            GameEventType::DuelStatus => GameEvent::DuelStatus(DuelStatusEvent::from_raw_event(event.values)?),
            GameEventType::FishNotice => GameEvent::FishNotice(FishNoticeEvent::from_raw_event(event.values)?),
            GameEventType::FishNoticeArm => GameEvent::FishNoticeArm(FishNoticeArmEvent::from_raw_event(event.values)?),
            GameEventType::ThrowableHit => GameEvent::ThrowableHit(ThrowableHitEvent::from_raw_event(event.values)?),
            GameEventType::PumpkinLordSummoned => GameEvent::PumpkinLordSummoned(PumpkinLordSummonedEvent::from_raw_event(event.values)?),
            GameEventType::PumpkinLordKilled => GameEvent::PumpkinLordKilled(PumpkinLordKilledEvent::from_raw_event(event.values)?),
            GameEventType::MerasmusSummoned => GameEvent::MerasmusSummoned(MerasmusSummonedEvent::from_raw_event(event.values)?),
            GameEventType::MerasmusKilled => GameEvent::MerasmusKilled(MerasmusKilledEvent::from_raw_event(event.values)?),
            GameEventType::MerasmusEscapeWarning => GameEvent::MerasmusEscapeWarning(MerasmusEscapeWarningEvent::from_raw_event(event.values)?),
            GameEventType::MerasmusEscaped => GameEvent::MerasmusEscaped(MerasmusEscapedEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossSummoned => GameEvent::EyeballBossSummoned(EyeballBossSummonedEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossStunned => GameEvent::EyeballBossStunned(EyeballBossStunnedEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossKilled => GameEvent::EyeballBossKilled(EyeballBossKilledEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossKiller => GameEvent::EyeballBossKiller(EyeballBossKillerEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossEscapeImminent => GameEvent::EyeballBossEscapeImminent(EyeballBossEscapeImminentEvent::from_raw_event(event.values)?),
            GameEventType::EyeballBossEscaped => GameEvent::EyeballBossEscaped(EyeballBossEscapedEvent::from_raw_event(event.values)?),
            GameEventType::NpcHurt => GameEvent::NpcHurt(NpcHurtEvent::from_raw_event(event.values)?),
            GameEventType::ControlPointTimerUpdated => GameEvent::ControlPointTimerUpdated(ControlPointTimerUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHighFiveStart => GameEvent::PlayerHighFiveStart(PlayerHighFiveStartEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHighFiveCancel => GameEvent::PlayerHighFiveCancel(PlayerHighFiveCancelEvent::from_raw_event(event.values)?),
            GameEventType::PlayerHighFiveSuccess => GameEvent::PlayerHighFiveSuccess(PlayerHighFiveSuccessEvent::from_raw_event(event.values)?),
            GameEventType::PlayerBonusPoints => GameEvent::PlayerBonusPoints(PlayerBonusPointsEvent::from_raw_event(event.values)?),
            GameEventType::PlayerUpgraded => GameEvent::PlayerUpgraded(PlayerUpgradedEvent::from_raw_event(event.values)?),
            GameEventType::PlayerBuyback => GameEvent::PlayerBuyback(PlayerBuybackEvent::from_raw_event(event.values)?),
            GameEventType::PlayerUsedPowerUpBottle => GameEvent::PlayerUsedPowerUpBottle(PlayerUsedPowerUpBottleEvent::from_raw_event(event.values)?),
            GameEventType::ChristmasGiftGrab => GameEvent::ChristmasGiftGrab(ChristmasGiftGrabEvent::from_raw_event(event.values)?),
            GameEventType::PlayerKilledAchievementZone => GameEvent::PlayerKilledAchievementZone(PlayerKilledAchievementZoneEvent::from_raw_event(event.values)?),
            GameEventType::PartyUpdated => GameEvent::PartyUpdated(PartyUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::LobbyUpdated => GameEvent::LobbyUpdated(LobbyUpdatedEvent::from_raw_event(event.values)?),
            GameEventType::MvmMissionUpdate => GameEvent::MvmMissionUpdate(MvmMissionUpdateEvent::from_raw_event(event.values)?),
            GameEventType::RecalculateHolidays => GameEvent::RecalculateHolidays(RecalculateHolidaysEvent::from_raw_event(event.values)?),
            GameEventType::PlayerCurrencyChanged => GameEvent::PlayerCurrencyChanged(PlayerCurrencyChangedEvent::from_raw_event(event.values)?),
            GameEventType::DoomsdayRocketOpen => GameEvent::DoomsdayRocketOpen(DoomsdayRocketOpenEvent::from_raw_event(event.values)?),
            GameEventType::RemoveNemesisRelationships => GameEvent::RemoveNemesisRelationships(RemoveNemesisRelationshipsEvent::from_raw_event(event.values)?),
            GameEventType::MvmCreditBonusWave => GameEvent::MvmCreditBonusWave(MvmCreditBonusWaveEvent::from_raw_event(event.values)?),
            GameEventType::MvmCreditBonusAll => GameEvent::MvmCreditBonusAll(MvmCreditBonusAllEvent::from_raw_event(event.values)?),
            GameEventType::MvmCreditBonusAllAdvanced => GameEvent::MvmCreditBonusAllAdvanced(MvmCreditBonusAllAdvancedEvent::from_raw_event(event.values)?),
            GameEventType::MvmQuickSentryUpgrade => GameEvent::MvmQuickSentryUpgrade(MvmQuickSentryUpgradeEvent::from_raw_event(event.values)?),
            GameEventType::MvmTankDestroyedByPlayers => GameEvent::MvmTankDestroyedByPlayers(MvmTankDestroyedByPlayersEvent::from_raw_event(event.values)?),
            GameEventType::MvmKillRobotDeliveringBomb => GameEvent::MvmKillRobotDeliveringBomb(MvmKillRobotDeliveringBombEvent::from_raw_event(event.values)?),
            GameEventType::MvmPickupCurrency => GameEvent::MvmPickupCurrency(MvmPickupCurrencyEvent::from_raw_event(event.values)?),
            GameEventType::MvmBombCarrierKilled => GameEvent::MvmBombCarrierKilled(MvmBombCarrierKilledEvent::from_raw_event(event.values)?),
            GameEventType::MvmSentryBusterDetonate => GameEvent::MvmSentryBusterDetonate(MvmSentryBusterDetonateEvent::from_raw_event(event.values)?),
            GameEventType::MvmScoutMarkedForDeath => GameEvent::MvmScoutMarkedForDeath(MvmScoutMarkedForDeathEvent::from_raw_event(event.values)?),
            GameEventType::MvmMedicPowerUpShared => GameEvent::MvmMedicPowerUpShared(MvmMedicPowerUpSharedEvent::from_raw_event(event.values)?),
            GameEventType::MvmBeginWave => GameEvent::MvmBeginWave(MvmBeginWaveEvent::from_raw_event(event.values)?),
            GameEventType::MvmWaveComplete => GameEvent::MvmWaveComplete(MvmWaveCompleteEvent::from_raw_event(event.values)?),
            GameEventType::MvmMissionComplete => GameEvent::MvmMissionComplete(MvmMissionCompleteEvent::from_raw_event(event.values)?),
            GameEventType::MvmBombResetByPlayer => GameEvent::MvmBombResetByPlayer(MvmBombResetByPlayerEvent::from_raw_event(event.values)?),
            GameEventType::MvmBombAlarmTriggered => GameEvent::MvmBombAlarmTriggered(MvmBombAlarmTriggeredEvent::from_raw_event(event.values)?),
            GameEventType::MvmBombDeployResetByPlayer => GameEvent::MvmBombDeployResetByPlayer(MvmBombDeployResetByPlayerEvent::from_raw_event(event.values)?),
            GameEventType::MvmWaveFailed => GameEvent::MvmWaveFailed(MvmWaveFailedEvent::from_raw_event(event.values)?),
            GameEventType::MvmResetStats => GameEvent::MvmResetStats(MvmResetStatsEvent::from_raw_event(event.values)?),
            GameEventType::DamageResisted => GameEvent::DamageResisted(DamageResistedEvent::from_raw_event(event.values)?),
            GameEventType::RevivePlayerNotify => GameEvent::RevivePlayerNotify(RevivePlayerNotifyEvent::from_raw_event(event.values)?),
            GameEventType::RevivePlayerStopped => GameEvent::RevivePlayerStopped(RevivePlayerStoppedEvent::from_raw_event(event.values)?),
            GameEventType::RevivePlayerComplete => GameEvent::RevivePlayerComplete(RevivePlayerCompleteEvent::from_raw_event(event.values)?),
            GameEventType::PlayerTurnedToGhost => GameEvent::PlayerTurnedToGhost(PlayerTurnedToGhostEvent::from_raw_event(event.values)?),
            GameEventType::MedigunShieldBlockedDamage => GameEvent::MedigunShieldBlockedDamage(MedigunShieldBlockedDamageEvent::from_raw_event(event.values)?),
            GameEventType::MvmAdvWaveCompleteNoGates => GameEvent::MvmAdvWaveCompleteNoGates(MvmAdvWaveCompleteNoGatesEvent::from_raw_event(event.values)?),
            GameEventType::MvmSniperHeadshotCurrency => GameEvent::MvmSniperHeadshotCurrency(MvmSniperHeadshotCurrencyEvent::from_raw_event(event.values)?),
            GameEventType::MvmMannhattanPit => GameEvent::MvmMannhattanPit(MvmMannhattanPitEvent::from_raw_event(event.values)?),
            GameEventType::FlagCarriedInDetectionZone => GameEvent::FlagCarriedInDetectionZone(FlagCarriedInDetectionZoneEvent::from_raw_event(event.values)?),
            GameEventType::MvmAdvWaveKilledStunRadio => GameEvent::MvmAdvWaveKilledStunRadio(MvmAdvWaveKilledStunRadioEvent::from_raw_event(event.values)?),
            GameEventType::PlayerDirectHitStun => GameEvent::PlayerDirectHitStun(PlayerDirectHitStunEvent::from_raw_event(event.values)?),
            GameEventType::MvmSentryBusterKilled => GameEvent::MvmSentryBusterKilled(MvmSentryBusterKilledEvent::from_raw_event(event.values)?),
            GameEventType::UpgradesFileChanged => GameEvent::UpgradesFileChanged(UpgradesFileChangedEvent::from_raw_event(event.values)?),
            GameEventType::RdTeamPointsChanged => GameEvent::RdTeamPointsChanged(RdTeamPointsChangedEvent::from_raw_event(event.values)?),
            GameEventType::RdRulesStateChanged => GameEvent::RdRulesStateChanged(RdRulesStateChangedEvent::from_raw_event(event.values)?),
            GameEventType::RdRobotKilled => GameEvent::RdRobotKilled(RdRobotKilledEvent::from_raw_event(event.values)?),
            GameEventType::RdRobotImpact => GameEvent::RdRobotImpact(RdRobotImpactEvent::from_raw_event(event.values)?),
            GameEventType::TeamPlayPreRoundTimeLeft => GameEvent::TeamPlayPreRoundTimeLeft(TeamPlayPreRoundTimeLeftEvent::from_raw_event(event.values)?),
            GameEventType::ParachuteDeploy => GameEvent::ParachuteDeploy(ParachuteDeployEvent::from_raw_event(event.values)?),
            GameEventType::ParachuteHolster => GameEvent::ParachuteHolster(ParachuteHolsterEvent::from_raw_event(event.values)?),
            GameEventType::KillRefillsMeter => GameEvent::KillRefillsMeter(KillRefillsMeterEvent::from_raw_event(event.values)?),
            GameEventType::RpsTauntEvent => GameEvent::RpsTauntEvent(RpsTauntEventEvent::from_raw_event(event.values)?),
            GameEventType::CongaKill => GameEvent::CongaKill(CongaKillEvent::from_raw_event(event.values)?),
            GameEventType::PlayerInitialSpawn => GameEvent::PlayerInitialSpawn(PlayerInitialSpawnEvent::from_raw_event(event.values)?),
            GameEventType::CompetitiveVictory => GameEvent::CompetitiveVictory(CompetitiveVictoryEvent::from_raw_event(event.values)?),
            GameEventType::CompetitiveSkillRatingUpdate => GameEvent::CompetitiveSkillRatingUpdate(CompetitiveSkillRatingUpdateEvent::from_raw_event(event.values)?),
            GameEventType::MiniGameWin => GameEvent::MiniGameWin(MiniGameWinEvent::from_raw_event(event.values)?),
            GameEventType::SentryOnGoActive => GameEvent::SentryOnGoActive(SentryOnGoActiveEvent::from_raw_event(event.values)?),
            GameEventType::DuckXpLevelUp => GameEvent::DuckXpLevelUp(DuckXpLevelUpEvent::from_raw_event(event.values)?),
            GameEventType::HLTVStatus => GameEvent::HLTVStatus(HLTVStatusEvent::from_raw_event(event.values)?),
            GameEventType::HLTVCameraman => GameEvent::HLTVCameraman(HLTVCameramanEvent::from_raw_event(event.values)?),
            GameEventType::HLTVRankCamera => GameEvent::HLTVRankCamera(HLTVRankCameraEvent::from_raw_event(event.values)?),
            GameEventType::HLTVRankEntity => GameEvent::HLTVRankEntity(HLTVRankEntityEvent::from_raw_event(event.values)?),
            GameEventType::HLTVFixed => GameEvent::HLTVFixed(HLTVFixedEvent::from_raw_event(event.values)?),
            GameEventType::HLTVChase => GameEvent::HLTVChase(HLTVChaseEvent::from_raw_event(event.values)?),
            GameEventType::HLTVMessage => GameEvent::HLTVMessage(HLTVMessageEvent::from_raw_event(event.values)?),
            GameEventType::HLTVTitle => GameEvent::HLTVTitle(HLTVTitleEvent::from_raw_event(event.values)?),
            GameEventType::HLTVChat => GameEvent::HLTVChat(HLTVChatEvent::from_raw_event(event.values)?),
            GameEventType::ReplayStartRecord => GameEvent::ReplayStartRecord(ReplayStartRecordEvent::from_raw_event(event.values)?),
            GameEventType::ReplaySessionInfo => GameEvent::ReplaySessionInfo(ReplaySessionInfoEvent::from_raw_event(event.values)?),
            GameEventType::ReplayEndRecord => GameEvent::ReplayEndRecord(ReplayEndRecordEvent::from_raw_event(event.values)?),
            GameEventType::ReplayReplaysAvailable => GameEvent::ReplayReplaysAvailable(ReplayReplaysAvailableEvent::from_raw_event(event.values)?),
            GameEventType::ReplayServerError => GameEvent::ReplayServerError(ReplayServerErrorEvent::from_raw_event(event.values)?),
            GameEventType::Unknown => GameEvent::Unknown(event),
        })
    }
}

