use std::collections::HashMap;
use crate::{Result, ParseError};
use super::gamevent::{FromGameEventValue, GameEventValue, FromRawGameEvent, RawGameEvent};

/// auto generated
pub struct ServerSpawnEvent {
    pub hostname: String,
    pub address: String,
    pub ip: u32,
    pub port: u16,
    pub game: String,
    pub mapname: String,
    pub maxplayers: u32,
    pub os: String,
    pub dedicated: bool,
    pub password: bool,
}
impl FromRawGameEvent for ServerSpawnEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let hostname: String = {
            let value = values.get("hostname").ok_or(ParseError::UnknownGameEvent("hostname".to_string()))?;
            String::from_value(value.clone(), "hostname")?
        };
        let address: String = {
            let value = values.get("address").ok_or(ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get("port").ok_or(ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        let game: String = {
            let value = values.get("game").ok_or(ParseError::UnknownGameEvent("game".to_string()))?;
            String::from_value(value.clone(), "game")?
        };
        let mapname: String = {
            let value = values.get("mapname").ok_or(ParseError::UnknownGameEvent("mapname".to_string()))?;
            String::from_value(value.clone(), "mapname")?
        };
        let maxplayers: u32 = {
            let value = values.get("maxplayers").ok_or(ParseError::UnknownGameEvent("maxplayers".to_string()))?;
            u32::from_value(value.clone(), "maxplayers")?
        };
        let os: String = {
            let value = values.get("os").ok_or(ParseError::UnknownGameEvent("os".to_string()))?;
            String::from_value(value.clone(), "os")?
        };
        let dedicated: bool = {
            let value = values.get("dedicated").ok_or(ParseError::UnknownGameEvent("dedicated".to_string()))?;
            bool::from_value(value.clone(), "dedicated")?
        };
        let password: bool = {
            let value = values.get("password").ok_or(ParseError::UnknownGameEvent("password".to_string()))?;
            bool::from_value(value.clone(), "password")?
        };
        Ok(ServerSpawnEvent {
            hostname,
            address,
            ip,
            port,
            game,
            mapname,
            maxplayers,
            os,
            dedicated,
            password
        })
    }
}

pub struct ServerShutdownEvent {
    pub reason: String,
}
impl FromRawGameEvent for ServerShutdownEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(ServerShutdownEvent {
            reason
        })
    }
}

pub struct ServerCvarEvent {
    pub cvarname: String,
    pub cvarvalue: String,
}
impl FromRawGameEvent for ServerCvarEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cvarname: String = {
            let value = values.get("cvarname").ok_or(ParseError::UnknownGameEvent("cvarname".to_string()))?;
            String::from_value(value.clone(), "cvarname")?
        };
        let cvarvalue: String = {
            let value = values.get("cvarvalue").ok_or(ParseError::UnknownGameEvent("cvarvalue".to_string()))?;
            String::from_value(value.clone(), "cvarvalue")?
        };
        Ok(ServerCvarEvent {
            cvarname,
            cvarvalue
        })
    }
}

pub struct ServerMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for ServerMessageEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(ServerMessageEvent {
            text
        })
    }
}

pub struct ServerAddBanEvent {
    pub name: String,
    pub userid: u16,
    pub networkid: String,
    pub ip: String,
    pub duration: String,
    pub by: String,
    pub kicked: bool,
}
impl FromRawGameEvent for ServerAddBanEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let ip: String = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            String::from_value(value.clone(), "ip")?
        };
        let duration: String = {
            let value = values.get("duration").ok_or(ParseError::UnknownGameEvent("duration".to_string()))?;
            String::from_value(value.clone(), "duration")?
        };
        let by: String = {
            let value = values.get("by").ok_or(ParseError::UnknownGameEvent("by".to_string()))?;
            String::from_value(value.clone(), "by")?
        };
        let kicked: bool = {
            let value = values.get("kicked").ok_or(ParseError::UnknownGameEvent("kicked".to_string()))?;
            bool::from_value(value.clone(), "kicked")?
        };
        Ok(ServerAddBanEvent {
            name,
            userid,
            networkid,
            ip,
            duration,
            by,
            kicked
        })
    }
}

pub struct ServerRemoveBanEvent {
    pub networkid: String,
    pub ip: String,
    pub by: String,
}
impl FromRawGameEvent for ServerRemoveBanEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let ip: String = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            String::from_value(value.clone(), "ip")?
        };
        let by: String = {
            let value = values.get("by").ok_or(ParseError::UnknownGameEvent("by".to_string()))?;
            String::from_value(value.clone(), "by")?
        };
        Ok(ServerRemoveBanEvent {
            networkid,
            ip,
            by
        })
    }
}

pub struct PlayerConnectEvent {
    pub name: String,
    pub index: u8,
    pub userid: u16,
    pub networkid: String,
    pub address: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerConnectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let address: String = {
            let value = values.get("address").ok_or(ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let bot: u16 = {
            let value = values.get("bot").ok_or(ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerConnectEvent {
            name,
            index,
            userid,
            networkid,
            address,
            bot
        })
    }
}

pub struct PlayerConnectClientEvent {
    pub name: String,
    pub index: u8,
    pub userid: u16,
    pub networkid: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerConnectClientEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let bot: u16 = {
            let value = values.get("bot").ok_or(ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerConnectClientEvent {
            name,
            index,
            userid,
            networkid,
            bot
        })
    }
}

pub struct PlayerInfoEvent {
    pub name: String,
    pub index: u8,
    pub userid: u16,
    pub networkid: String,
    pub bot: bool,
}
impl FromRawGameEvent for PlayerInfoEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let index: u8 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let bot: bool = {
            let value = values.get("bot").ok_or(ParseError::UnknownGameEvent("bot".to_string()))?;
            bool::from_value(value.clone(), "bot")?
        };
        Ok(PlayerInfoEvent {
            name,
            index,
            userid,
            networkid,
            bot
        })
    }
}

pub struct PlayerDisconnectEvent {
    pub userid: u16,
    pub reason: String,
    pub name: String,
    pub networkid: String,
    pub bot: u16,
}
impl FromRawGameEvent for PlayerDisconnectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let reason: String = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        let networkid: String = {
            let value = values.get("networkid").ok_or(ParseError::UnknownGameEvent("networkid".to_string()))?;
            String::from_value(value.clone(), "networkid")?
        };
        let bot: u16 = {
            let value = values.get("bot").ok_or(ParseError::UnknownGameEvent("bot".to_string()))?;
            u16::from_value(value.clone(), "bot")?
        };
        Ok(PlayerDisconnectEvent {
            userid,
            reason,
            name,
            networkid,
            bot
        })
    }
}

pub struct PlayerActivateEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PlayerActivateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PlayerActivateEvent {
            userid
        })
    }
}

pub struct PlayerSayEvent {
    pub userid: u16,
    pub text: String,
}
impl FromRawGameEvent for PlayerSayEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(PlayerSayEvent {
            userid,
            text
        })
    }
}

pub struct ClientDisconnectEvent {
    pub message: String,
}
impl FromRawGameEvent for ClientDisconnectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let message: String = {
            let value = values.get("message").ok_or(ParseError::UnknownGameEvent("message".to_string()))?;
            String::from_value(value.clone(), "message")?
        };
        Ok(ClientDisconnectEvent {
            message
        })
    }
}

pub struct ClientBeginConnectEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
    pub source: String,
}
impl FromRawGameEvent for ClientBeginConnectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let address: String = {
            let value = values.get("address").ok_or(ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get("port").ok_or(ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        let source: String = {
            let value = values.get("source").ok_or(ParseError::UnknownGameEvent("source".to_string()))?;
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

pub struct ClientConnectedEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
}
impl FromRawGameEvent for ClientConnectedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let address: String = {
            let value = values.get("address").ok_or(ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get("port").ok_or(ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        Ok(ClientConnectedEvent {
            address,
            ip,
            port
        })
    }
}

pub struct ClientFullConnectEvent {
    pub address: String,
    pub ip: u32,
    pub port: u16,
}
impl FromRawGameEvent for ClientFullConnectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let address: String = {
            let value = values.get("address").ok_or(ParseError::UnknownGameEvent("address".to_string()))?;
            String::from_value(value.clone(), "address")?
        };
        let ip: u32 = {
            let value = values.get("ip").ok_or(ParseError::UnknownGameEvent("ip".to_string()))?;
            u32::from_value(value.clone(), "ip")?
        };
        let port: u16 = {
            let value = values.get("port").ok_or(ParseError::UnknownGameEvent("port".to_string()))?;
            u16::from_value(value.clone(), "port")?
        };
        Ok(ClientFullConnectEvent {
            address,
            ip,
            port
        })
    }
}

pub struct HostQuitEvent {

}
impl FromRawGameEvent for HostQuitEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(HostQuitEvent {

        })
    }
}

pub struct TeamInfoEvent {
    pub teamid: u8,
    pub teamname: String,
}
impl FromRawGameEvent for TeamInfoEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let teamid: u8 = {
            let value = values.get("teamid").ok_or(ParseError::UnknownGameEvent("teamid".to_string()))?;
            u8::from_value(value.clone(), "teamid")?
        };
        let teamname: String = {
            let value = values.get("teamname").ok_or(ParseError::UnknownGameEvent("teamname".to_string()))?;
            String::from_value(value.clone(), "teamname")?
        };
        Ok(TeamInfoEvent {
            teamid,
            teamname
        })
    }
}

pub struct TeamScoreEvent {
    pub teamid: u8,
    pub score: u16,
}
impl FromRawGameEvent for TeamScoreEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let teamid: u8 = {
            let value = values.get("teamid").ok_or(ParseError::UnknownGameEvent("teamid".to_string()))?;
            u8::from_value(value.clone(), "teamid")?
        };
        let score: u16 = {
            let value = values.get("score").ok_or(ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(TeamScoreEvent {
            teamid,
            score
        })
    }
}

pub struct TeamPlayBroadcastAudioEvent {
    pub team: u8,
    pub sound: String,
    pub additional_flags: u16,
}
impl FromRawGameEvent for TeamPlayBroadcastAudioEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let sound: String = {
            let value = values.get("sound").ok_or(ParseError::UnknownGameEvent("sound".to_string()))?;
            String::from_value(value.clone(), "sound")?
        };
        let additional_flags: u16 = {
            let value = values.get("additional_flags").ok_or(ParseError::UnknownGameEvent("additional_flags".to_string()))?;
            u16::from_value(value.clone(), "additional_flags")?
        };
        Ok(TeamPlayBroadcastAudioEvent {
            team,
            sound,
            additional_flags
        })
    }
}

pub struct PlayerTeamEvent {
    pub userid: u16,
    pub team: u8,
    pub oldteam: u8,
    pub disconnect: bool,
    pub autoteam: bool,
    pub silent: bool,
    pub name: String,
}
impl FromRawGameEvent for PlayerTeamEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let oldteam: u8 = {
            let value = values.get("oldteam").ok_or(ParseError::UnknownGameEvent("oldteam".to_string()))?;
            u8::from_value(value.clone(), "oldteam")?
        };
        let disconnect: bool = {
            let value = values.get("disconnect").ok_or(ParseError::UnknownGameEvent("disconnect".to_string()))?;
            bool::from_value(value.clone(), "disconnect")?
        };
        let autoteam: bool = {
            let value = values.get("autoteam").ok_or(ParseError::UnknownGameEvent("autoteam".to_string()))?;
            bool::from_value(value.clone(), "autoteam")?
        };
        let silent: bool = {
            let value = values.get("silent").ok_or(ParseError::UnknownGameEvent("silent".to_string()))?;
            bool::from_value(value.clone(), "silent")?
        };
        let name: String = {
            let value = values.get("name").ok_or(ParseError::UnknownGameEvent("name".to_string()))?;
            String::from_value(value.clone(), "name")?
        };
        Ok(PlayerTeamEvent {
            userid,
            team,
            oldteam,
            disconnect,
            autoteam,
            silent,
            name
        })
    }
}

pub struct PlayerClassEvent {
    pub userid: u16,
    pub class: String,
}
impl FromRawGameEvent for PlayerClassEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let class: String = {
            let value = values.get("class").ok_or(ParseError::UnknownGameEvent("class".to_string()))?;
            String::from_value(value.clone(), "class")?
        };
        Ok(PlayerClassEvent {
            userid,
            class
        })
    }
}

pub struct PlayerDeathEvent {
    pub userid: u16,
    pub victim_entindex: u32,
    pub inflictor_entindex: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub damagebits: u32,
    pub customkill: u16,
    pub assister: u16,
    pub weapon_logclassname: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub playerpenetratecount: u16,
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
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let victim_entindex: u32 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u32::from_value(value.clone(), "victim_entindex")?
        };
        let inflictor_entindex: u32 = {
            let value = values.get("inflictor_entindex").ok_or(ParseError::UnknownGameEvent("inflictor_entindex".to_string()))?;
            u32::from_value(value.clone(), "inflictor_entindex")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        let customkill: u16 = {
            let value = values.get("customkill").ok_or(ParseError::UnknownGameEvent("customkill".to_string()))?;
            u16::from_value(value.clone(), "customkill")?
        };
        let assister: u16 = {
            let value = values.get("assister").ok_or(ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_logclassname: String = {
            let value = values.get("weapon_logclassname").ok_or(ParseError::UnknownGameEvent("weapon_logclassname".to_string()))?;
            String::from_value(value.clone(), "weapon_logclassname")?
        };
        let stun_flags: u16 = {
            let value = values.get("stun_flags").ok_or(ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get("death_flags").ok_or(ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get("silent_kill").ok_or(ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let playerpenetratecount: u16 = {
            let value = values.get("playerpenetratecount").ok_or(ParseError::UnknownGameEvent("playerpenetratecount".to_string()))?;
            u16::from_value(value.clone(), "playerpenetratecount")?
        };
        let assister_fallback: String = {
            let value = values.get("assister_fallback").ok_or(ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        let kill_streak_total: u16 = {
            let value = values.get("kill_streak_total").ok_or(ParseError::UnknownGameEvent("kill_streak_total".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_total")?
        };
        let kill_streak_wep: u16 = {
            let value = values.get("kill_streak_wep").ok_or(ParseError::UnknownGameEvent("kill_streak_wep".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_wep")?
        };
        let kill_streak_assist: u16 = {
            let value = values.get("kill_streak_assist").ok_or(ParseError::UnknownGameEvent("kill_streak_assist".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_assist")?
        };
        let kill_streak_victim: u16 = {
            let value = values.get("kill_streak_victim").ok_or(ParseError::UnknownGameEvent("kill_streak_victim".to_string()))?;
            u16::from_value(value.clone(), "kill_streak_victim")?
        };
        let ducks_streaked: u16 = {
            let value = values.get("ducks_streaked").ok_or(ParseError::UnknownGameEvent("ducks_streaked".to_string()))?;
            u16::from_value(value.clone(), "ducks_streaked")?
        };
        let duck_streak_total: u16 = {
            let value = values.get("duck_streak_total").ok_or(ParseError::UnknownGameEvent("duck_streak_total".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_total")?
        };
        let duck_streak_assist: u16 = {
            let value = values.get("duck_streak_assist").ok_or(ParseError::UnknownGameEvent("duck_streak_assist".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_assist")?
        };
        let duck_streak_victim: u16 = {
            let value = values.get("duck_streak_victim").ok_or(ParseError::UnknownGameEvent("duck_streak_victim".to_string()))?;
            u16::from_value(value.clone(), "duck_streak_victim")?
        };
        let rocket_jump: bool = {
            let value = values.get("rocket_jump").ok_or(ParseError::UnknownGameEvent("rocket_jump".to_string()))?;
            bool::from_value(value.clone(), "rocket_jump")?
        };
        Ok(PlayerDeathEvent {
            userid,
            victim_entindex,
            inflictor_entindex,
            attacker,
            weapon,
            weaponid,
            damagebits,
            customkill,
            assister,
            weapon_logclassname,
            stun_flags,
            death_flags,
            silent_kill,
            playerpenetratecount,
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

pub struct PlayerHurtEvent {
    pub userid: u16,
    pub health: u16,
    pub attacker: u16,
    pub damageamount: u16,
    pub custom: u16,
    pub showdisguisedcrit: bool,
    pub crit: bool,
    pub minicrit: bool,
    pub allseecrit: bool,
    pub weaponid: u16,
    pub bonuseffect: u8,
}
impl FromRawGameEvent for PlayerHurtEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let health: u16 = {
            let value = values.get("health").ok_or(ParseError::UnknownGameEvent("health".to_string()))?;
            u16::from_value(value.clone(), "health")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let damageamount: u16 = {
            let value = values.get("damageamount").ok_or(ParseError::UnknownGameEvent("damageamount".to_string()))?;
            u16::from_value(value.clone(), "damageamount")?
        };
        let custom: u16 = {
            let value = values.get("custom").ok_or(ParseError::UnknownGameEvent("custom".to_string()))?;
            u16::from_value(value.clone(), "custom")?
        };
        let showdisguisedcrit: bool = {
            let value = values.get("showdisguisedcrit").ok_or(ParseError::UnknownGameEvent("showdisguisedcrit".to_string()))?;
            bool::from_value(value.clone(), "showdisguisedcrit")?
        };
        let crit: bool = {
            let value = values.get("crit").ok_or(ParseError::UnknownGameEvent("crit".to_string()))?;
            bool::from_value(value.clone(), "crit")?
        };
        let minicrit: bool = {
            let value = values.get("minicrit").ok_or(ParseError::UnknownGameEvent("minicrit".to_string()))?;
            bool::from_value(value.clone(), "minicrit")?
        };
        let allseecrit: bool = {
            let value = values.get("allseecrit").ok_or(ParseError::UnknownGameEvent("allseecrit".to_string()))?;
            bool::from_value(value.clone(), "allseecrit")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let bonuseffect: u8 = {
            let value = values.get("bonuseffect").ok_or(ParseError::UnknownGameEvent("bonuseffect".to_string()))?;
            u8::from_value(value.clone(), "bonuseffect")?
        };
        Ok(PlayerHurtEvent {
            userid,
            health,
            attacker,
            damageamount,
            custom,
            showdisguisedcrit,
            crit,
            minicrit,
            allseecrit,
            weaponid,
            bonuseffect
        })
    }
}

pub struct PlayerChatEvent {
    pub teamonly: bool,
    pub userid: u16,
    pub text: String,
}
impl FromRawGameEvent for PlayerChatEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let teamonly: bool = {
            let value = values.get("teamonly").ok_or(ParseError::UnknownGameEvent("teamonly".to_string()))?;
            bool::from_value(value.clone(), "teamonly")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(PlayerChatEvent {
            teamonly,
            userid,
            text
        })
    }
}

pub struct PlayerScoreEvent {
    pub userid: u16,
    pub kills: u16,
    pub deaths: u16,
    pub score: u16,
}
impl FromRawGameEvent for PlayerScoreEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let kills: u16 = {
            let value = values.get("kills").ok_or(ParseError::UnknownGameEvent("kills".to_string()))?;
            u16::from_value(value.clone(), "kills")?
        };
        let deaths: u16 = {
            let value = values.get("deaths").ok_or(ParseError::UnknownGameEvent("deaths".to_string()))?;
            u16::from_value(value.clone(), "deaths")?
        };
        let score: u16 = {
            let value = values.get("score").ok_or(ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(PlayerScoreEvent {
            userid,
            kills,
            deaths,
            score
        })
    }
}

pub struct PlayerSpawnEvent {
    pub userid: u16,
    pub team: u16,
    pub class: u16,
}
impl FromRawGameEvent for PlayerSpawnEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let team: u16 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u16::from_value(value.clone(), "team")?
        };
        let class: u16 = {
            let value = values.get("class").ok_or(ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        Ok(PlayerSpawnEvent {
            userid,
            team,
            class
        })
    }
}

pub struct PlayerShootEvent {
    pub userid: u16,
    pub weapon: u8,
    pub mode: u8,
}
impl FromRawGameEvent for PlayerShootEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let weapon: u8 = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            u8::from_value(value.clone(), "weapon")?
        };
        let mode: u8 = {
            let value = values.get("mode").ok_or(ParseError::UnknownGameEvent("mode".to_string()))?;
            u8::from_value(value.clone(), "mode")?
        };
        Ok(PlayerShootEvent {
            userid,
            weapon,
            mode
        })
    }
}

pub struct PlayerUseEvent {
    pub userid: u16,
    pub entity: u16,
}
impl FromRawGameEvent for PlayerUseEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let entity: u16 = {
            let value = values.get("entity").ok_or(ParseError::UnknownGameEvent("entity".to_string()))?;
            u16::from_value(value.clone(), "entity")?
        };
        Ok(PlayerUseEvent {
            userid,
            entity
        })
    }
}

pub struct PlayerChangeNameEvent {
    pub userid: u16,
    pub oldname: String,
    pub newname: String,
}
impl FromRawGameEvent for PlayerChangeNameEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let oldname: String = {
            let value = values.get("oldname").ok_or(ParseError::UnknownGameEvent("oldname".to_string()))?;
            String::from_value(value.clone(), "oldname")?
        };
        let newname: String = {
            let value = values.get("newname").ok_or(ParseError::UnknownGameEvent("newname".to_string()))?;
            String::from_value(value.clone(), "newname")?
        };
        Ok(PlayerChangeNameEvent {
            userid,
            oldname,
            newname
        })
    }
}

pub struct PlayerHintMessageEvent {
    pub hintmessage: String,
}
impl FromRawGameEvent for PlayerHintMessageEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let hintmessage: String = {
            let value = values.get("hintmessage").ok_or(ParseError::UnknownGameEvent("hintmessage".to_string()))?;
            String::from_value(value.clone(), "hintmessage")?
        };
        Ok(PlayerHintMessageEvent {
            hintmessage
        })
    }
}

pub struct BasePlayerTeleportedEvent {
    pub entindex: u16,
}
impl FromRawGameEvent for BasePlayerTeleportedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        Ok(BasePlayerTeleportedEvent {
            entindex
        })
    }
}

pub struct GameInitEvent {

}
impl FromRawGameEvent for GameInitEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(GameInitEvent {

        })
    }
}

pub struct GameNewMapEvent {
    pub mapname: String,
}
impl FromRawGameEvent for GameNewMapEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let mapname: String = {
            let value = values.get("mapname").ok_or(ParseError::UnknownGameEvent("mapname".to_string()))?;
            String::from_value(value.clone(), "mapname")?
        };
        Ok(GameNewMapEvent {
            mapname
        })
    }
}

pub struct GameStartEvent {
    pub roundslimit: u32,
    pub timelimit: u32,
    pub fraglimit: u32,
    pub objective: String,
}
impl FromRawGameEvent for GameStartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let roundslimit: u32 = {
            let value = values.get("roundslimit").ok_or(ParseError::UnknownGameEvent("roundslimit".to_string()))?;
            u32::from_value(value.clone(), "roundslimit")?
        };
        let timelimit: u32 = {
            let value = values.get("timelimit").ok_or(ParseError::UnknownGameEvent("timelimit".to_string()))?;
            u32::from_value(value.clone(), "timelimit")?
        };
        let fraglimit: u32 = {
            let value = values.get("fraglimit").ok_or(ParseError::UnknownGameEvent("fraglimit".to_string()))?;
            u32::from_value(value.clone(), "fraglimit")?
        };
        let objective: String = {
            let value = values.get("objective").ok_or(ParseError::UnknownGameEvent("objective".to_string()))?;
            String::from_value(value.clone(), "objective")?
        };
        Ok(GameStartEvent {
            roundslimit,
            timelimit,
            fraglimit,
            objective
        })
    }
}

pub struct GameEndEvent {
    pub winner: u8,
}
impl FromRawGameEvent for GameEndEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let winner: u8 = {
            let value = values.get("winner").ok_or(ParseError::UnknownGameEvent("winner".to_string()))?;
            u8::from_value(value.clone(), "winner")?
        };
        Ok(GameEndEvent {
            winner
        })
    }
}

pub struct RoundStartEvent {
    pub timelimit: u32,
    pub fraglimit: u32,
    pub objective: String,
}
impl FromRawGameEvent for RoundStartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let timelimit: u32 = {
            let value = values.get("timelimit").ok_or(ParseError::UnknownGameEvent("timelimit".to_string()))?;
            u32::from_value(value.clone(), "timelimit")?
        };
        let fraglimit: u32 = {
            let value = values.get("fraglimit").ok_or(ParseError::UnknownGameEvent("fraglimit".to_string()))?;
            u32::from_value(value.clone(), "fraglimit")?
        };
        let objective: String = {
            let value = values.get("objective").ok_or(ParseError::UnknownGameEvent("objective".to_string()))?;
            String::from_value(value.clone(), "objective")?
        };
        Ok(RoundStartEvent {
            timelimit,
            fraglimit,
            objective
        })
    }
}

pub struct RoundEndEvent {
    pub winner: u8,
    pub reason: u8,
    pub message: String,
}
impl FromRawGameEvent for RoundEndEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let winner: u8 = {
            let value = values.get("winner").ok_or(ParseError::UnknownGameEvent("winner".to_string()))?;
            u8::from_value(value.clone(), "winner")?
        };
        let reason: u8 = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            u8::from_value(value.clone(), "reason")?
        };
        let message: String = {
            let value = values.get("message").ok_or(ParseError::UnknownGameEvent("message".to_string()))?;
            String::from_value(value.clone(), "message")?
        };
        Ok(RoundEndEvent {
            winner,
            reason,
            message
        })
    }
}

pub struct GameMessageEvent {
    pub target: u8,
    pub text: String,
}
impl FromRawGameEvent for GameMessageEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let target: u8 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u8::from_value(value.clone(), "target")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(GameMessageEvent {
            target,
            text
        })
    }
}

pub struct BreakBreakableEvent {
    pub entindex: u32,
    pub userid: u16,
    pub material: u8,
}
impl FromRawGameEvent for BreakBreakableEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let material: u8 = {
            let value = values.get("material").ok_or(ParseError::UnknownGameEvent("material".to_string()))?;
            u8::from_value(value.clone(), "material")?
        };
        Ok(BreakBreakableEvent {
            entindex,
            userid,
            material
        })
    }
}

pub struct BreakPropEvent {
    pub entindex: u32,
    pub userid: u16,
}
impl FromRawGameEvent for BreakPropEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(BreakPropEvent {
            entindex,
            userid
        })
    }
}

pub struct EntityKilledEvent {
    pub entindex_killed: u32,
    pub entindex_attacker: u32,
    pub entindex_inflictor: u32,
    pub damagebits: u32,
}
impl FromRawGameEvent for EntityKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex_killed: u32 = {
            let value = values.get("entindex_killed").ok_or(ParseError::UnknownGameEvent("entindex_killed".to_string()))?;
            u32::from_value(value.clone(), "entindex_killed")?
        };
        let entindex_attacker: u32 = {
            let value = values.get("entindex_attacker").ok_or(ParseError::UnknownGameEvent("entindex_attacker".to_string()))?;
            u32::from_value(value.clone(), "entindex_attacker")?
        };
        let entindex_inflictor: u32 = {
            let value = values.get("entindex_inflictor").ok_or(ParseError::UnknownGameEvent("entindex_inflictor".to_string()))?;
            u32::from_value(value.clone(), "entindex_inflictor")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        Ok(EntityKilledEvent {
            entindex_killed,
            entindex_attacker,
            entindex_inflictor,
            damagebits
        })
    }
}

pub struct BonusUpdatedEvent {
    pub numadvanced: u16,
    pub numbronze: u16,
    pub numsilver: u16,
    pub numgold: u16,
}
impl FromRawGameEvent for BonusUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let numadvanced: u16 = {
            let value = values.get("numadvanced").ok_or(ParseError::UnknownGameEvent("numadvanced".to_string()))?;
            u16::from_value(value.clone(), "numadvanced")?
        };
        let numbronze: u16 = {
            let value = values.get("numbronze").ok_or(ParseError::UnknownGameEvent("numbronze".to_string()))?;
            u16::from_value(value.clone(), "numbronze")?
        };
        let numsilver: u16 = {
            let value = values.get("numsilver").ok_or(ParseError::UnknownGameEvent("numsilver".to_string()))?;
            u16::from_value(value.clone(), "numsilver")?
        };
        let numgold: u16 = {
            let value = values.get("numgold").ok_or(ParseError::UnknownGameEvent("numgold".to_string()))?;
            u16::from_value(value.clone(), "numgold")?
        };
        Ok(BonusUpdatedEvent {
            numadvanced,
            numbronze,
            numsilver,
            numgold
        })
    }
}

pub struct AchievementEventEvent {
    pub achievement_name: String,
    pub cur_val: u16,
    pub max_val: u16,
}
impl FromRawGameEvent for AchievementEventEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let achievement_name: String = {
            let value = values.get("achievement_name").ok_or(ParseError::UnknownGameEvent("achievement_name".to_string()))?;
            String::from_value(value.clone(), "achievement_name")?
        };
        let cur_val: u16 = {
            let value = values.get("cur_val").ok_or(ParseError::UnknownGameEvent("cur_val".to_string()))?;
            u16::from_value(value.clone(), "cur_val")?
        };
        let max_val: u16 = {
            let value = values.get("max_val").ok_or(ParseError::UnknownGameEvent("max_val".to_string()))?;
            u16::from_value(value.clone(), "max_val")?
        };
        Ok(AchievementEventEvent {
            achievement_name,
            cur_val,
            max_val
        })
    }
}

pub struct AchievementIncrementEvent {
    pub achievement_id: u32,
    pub cur_val: u16,
    pub max_val: u16,
}
impl FromRawGameEvent for AchievementIncrementEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let achievement_id: u32 = {
            let value = values.get("achievement_id").ok_or(ParseError::UnknownGameEvent("achievement_id".to_string()))?;
            u32::from_value(value.clone(), "achievement_id")?
        };
        let cur_val: u16 = {
            let value = values.get("cur_val").ok_or(ParseError::UnknownGameEvent("cur_val".to_string()))?;
            u16::from_value(value.clone(), "cur_val")?
        };
        let max_val: u16 = {
            let value = values.get("max_val").ok_or(ParseError::UnknownGameEvent("max_val".to_string()))?;
            u16::from_value(value.clone(), "max_val")?
        };
        Ok(AchievementIncrementEvent {
            achievement_id,
            cur_val,
            max_val
        })
    }
}

pub struct PhysgunPickupEvent {
    pub entindex: u32,
}
impl FromRawGameEvent for PhysgunPickupEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        Ok(PhysgunPickupEvent {
            entindex
        })
    }
}

pub struct FlareIgniteNpcEvent {
    pub entindex: u32,
}
impl FromRawGameEvent for FlareIgniteNpcEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        Ok(FlareIgniteNpcEvent {
            entindex
        })
    }
}

pub struct HelicopterGrenadePuntMissEvent {

}
impl FromRawGameEvent for HelicopterGrenadePuntMissEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(HelicopterGrenadePuntMissEvent {

        })
    }
}

pub struct UserDataDownloadedEvent {

}
impl FromRawGameEvent for UserDataDownloadedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(UserDataDownloadedEvent {

        })
    }
}

pub struct RagdollDissolvedEvent {
    pub entindex: u32,
}
impl FromRawGameEvent for RagdollDissolvedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        Ok(RagdollDissolvedEvent {
            entindex
        })
    }
}

pub struct HLTVChangedModeEvent {
    pub oldmode: u16,
    pub newmode: u16,
    pub obs_target: u16,
}
impl FromRawGameEvent for HLTVChangedModeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let oldmode: u16 = {
            let value = values.get("oldmode").ok_or(ParseError::UnknownGameEvent("oldmode".to_string()))?;
            u16::from_value(value.clone(), "oldmode")?
        };
        let newmode: u16 = {
            let value = values.get("newmode").ok_or(ParseError::UnknownGameEvent("newmode".to_string()))?;
            u16::from_value(value.clone(), "newmode")?
        };
        let obs_target: u16 = {
            let value = values.get("obs_target").ok_or(ParseError::UnknownGameEvent("obs_target".to_string()))?;
            u16::from_value(value.clone(), "obs_target")?
        };
        Ok(HLTVChangedModeEvent {
            oldmode,
            newmode,
            obs_target
        })
    }
}

pub struct HLTVChangedTargetEvent {
    pub mode: u16,
    pub old_target: u16,
    pub obs_target: u16,
}
impl FromRawGameEvent for HLTVChangedTargetEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let mode: u16 = {
            let value = values.get("mode").ok_or(ParseError::UnknownGameEvent("mode".to_string()))?;
            u16::from_value(value.clone(), "mode")?
        };
        let old_target: u16 = {
            let value = values.get("old_target").ok_or(ParseError::UnknownGameEvent("old_target".to_string()))?;
            u16::from_value(value.clone(), "old_target")?
        };
        let obs_target: u16 = {
            let value = values.get("obs_target").ok_or(ParseError::UnknownGameEvent("obs_target".to_string()))?;
            u16::from_value(value.clone(), "obs_target")?
        };
        Ok(HLTVChangedTargetEvent {
            mode,
            old_target,
            obs_target
        })
    }
}

pub struct VoteEndedEvent {

}
impl FromRawGameEvent for VoteEndedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(VoteEndedEvent {

        })
    }
}

pub struct VoteStartedEvent {
    pub issue: String,
    pub param1: String,
    pub team: u8,
    pub initiator: u32,
}
impl FromRawGameEvent for VoteStartedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let issue: String = {
            let value = values.get("issue").ok_or(ParseError::UnknownGameEvent("issue".to_string()))?;
            String::from_value(value.clone(), "issue")?
        };
        let param1: String = {
            let value = values.get("param1").ok_or(ParseError::UnknownGameEvent("param1".to_string()))?;
            String::from_value(value.clone(), "param1")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let initiator: u32 = {
            let value = values.get("initiator").ok_or(ParseError::UnknownGameEvent("initiator".to_string()))?;
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

pub struct VoteChangedEvent {
    pub vote_option1: u8,
    pub vote_option2: u8,
    pub vote_option3: u8,
    pub vote_option4: u8,
    pub vote_option5: u8,
    pub potentialVotes: u8,
}
impl FromRawGameEvent for VoteChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let vote_option1: u8 = {
            let value = values.get("vote_option1").ok_or(ParseError::UnknownGameEvent("vote_option1".to_string()))?;
            u8::from_value(value.clone(), "vote_option1")?
        };
        let vote_option2: u8 = {
            let value = values.get("vote_option2").ok_or(ParseError::UnknownGameEvent("vote_option2".to_string()))?;
            u8::from_value(value.clone(), "vote_option2")?
        };
        let vote_option3: u8 = {
            let value = values.get("vote_option3").ok_or(ParseError::UnknownGameEvent("vote_option3".to_string()))?;
            u8::from_value(value.clone(), "vote_option3")?
        };
        let vote_option4: u8 = {
            let value = values.get("vote_option4").ok_or(ParseError::UnknownGameEvent("vote_option4".to_string()))?;
            u8::from_value(value.clone(), "vote_option4")?
        };
        let vote_option5: u8 = {
            let value = values.get("vote_option5").ok_or(ParseError::UnknownGameEvent("vote_option5".to_string()))?;
            u8::from_value(value.clone(), "vote_option5")?
        };
        let potentialVotes: u8 = {
            let value = values.get("potentialVotes").ok_or(ParseError::UnknownGameEvent("potentialVotes".to_string()))?;
            u8::from_value(value.clone(), "potentialVotes")?
        };
        Ok(VoteChangedEvent {
            vote_option1,
            vote_option2,
            vote_option3,
            vote_option4,
            vote_option5,
            potentialVotes
        })
    }
}

pub struct VotePassedEvent {
    pub details: String,
    pub param1: String,
    pub team: u8,
}
impl FromRawGameEvent for VotePassedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let details: String = {
            let value = values.get("details").ok_or(ParseError::UnknownGameEvent("details".to_string()))?;
            String::from_value(value.clone(), "details")?
        };
        let param1: String = {
            let value = values.get("param1").ok_or(ParseError::UnknownGameEvent("param1".to_string()))?;
            String::from_value(value.clone(), "param1")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(VotePassedEvent {
            details,
            param1,
            team
        })
    }
}

pub struct VoteFailedEvent {
    pub team: u8,
}
impl FromRawGameEvent for VoteFailedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(VoteFailedEvent {
            team
        })
    }
}

pub struct VoteCastEvent {
    pub vote_option: u8,
    pub team: u16,
    pub entityid: u32,
}
impl FromRawGameEvent for VoteCastEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let vote_option: u8 = {
            let value = values.get("vote_option").ok_or(ParseError::UnknownGameEvent("vote_option".to_string()))?;
            u8::from_value(value.clone(), "vote_option")?
        };
        let team: u16 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u16::from_value(value.clone(), "team")?
        };
        let entityid: u32 = {
            let value = values.get("entityid").ok_or(ParseError::UnknownGameEvent("entityid".to_string()))?;
            u32::from_value(value.clone(), "entityid")?
        };
        Ok(VoteCastEvent {
            vote_option,
            team,
            entityid
        })
    }
}

pub struct VoteOptionsEvent {
    pub count: u8,
    pub option1: String,
    pub option2: String,
    pub option3: String,
    pub option4: String,
    pub option5: String,
}
impl FromRawGameEvent for VoteOptionsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let count: u8 = {
            let value = values.get("count").ok_or(ParseError::UnknownGameEvent("count".to_string()))?;
            u8::from_value(value.clone(), "count")?
        };
        let option1: String = {
            let value = values.get("option1").ok_or(ParseError::UnknownGameEvent("option1".to_string()))?;
            String::from_value(value.clone(), "option1")?
        };
        let option2: String = {
            let value = values.get("option2").ok_or(ParseError::UnknownGameEvent("option2".to_string()))?;
            String::from_value(value.clone(), "option2")?
        };
        let option3: String = {
            let value = values.get("option3").ok_or(ParseError::UnknownGameEvent("option3".to_string()))?;
            String::from_value(value.clone(), "option3")?
        };
        let option4: String = {
            let value = values.get("option4").ok_or(ParseError::UnknownGameEvent("option4".to_string()))?;
            String::from_value(value.clone(), "option4")?
        };
        let option5: String = {
            let value = values.get("option5").ok_or(ParseError::UnknownGameEvent("option5".to_string()))?;
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

pub struct ReplaySavedEvent {

}
impl FromRawGameEvent for ReplaySavedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ReplaySavedEvent {

        })
    }
}

pub struct EnteredPerformanceModeEvent {

}
impl FromRawGameEvent for EnteredPerformanceModeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(EnteredPerformanceModeEvent {

        })
    }
}

pub struct BrowseReplaysEvent {

}
impl FromRawGameEvent for BrowseReplaysEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(BrowseReplaysEvent {

        })
    }
}

pub struct ReplayYoutubeStatsEvent {
    pub views: u32,
    pub likes: u32,
    pub favorited: u32,
}
impl FromRawGameEvent for ReplayYoutubeStatsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let views: u32 = {
            let value = values.get("views").ok_or(ParseError::UnknownGameEvent("views".to_string()))?;
            u32::from_value(value.clone(), "views")?
        };
        let likes: u32 = {
            let value = values.get("likes").ok_or(ParseError::UnknownGameEvent("likes".to_string()))?;
            u32::from_value(value.clone(), "likes")?
        };
        let favorited: u32 = {
            let value = values.get("favorited").ok_or(ParseError::UnknownGameEvent("favorited".to_string()))?;
            u32::from_value(value.clone(), "favorited")?
        };
        Ok(ReplayYoutubeStatsEvent {
            views,
            likes,
            favorited
        })
    }
}

pub struct InventoryUpdatedEvent {

}
impl FromRawGameEvent for InventoryUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(InventoryUpdatedEvent {

        })
    }
}

pub struct CartUpdatedEvent {

}
impl FromRawGameEvent for CartUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(CartUpdatedEvent {

        })
    }
}

pub struct StorePricesheetUpdatedEvent {

}
impl FromRawGameEvent for StorePricesheetUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(StorePricesheetUpdatedEvent {

        })
    }
}

pub struct GcConnectedEvent {

}
impl FromRawGameEvent for GcConnectedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(GcConnectedEvent {

        })
    }
}

pub struct ItemSchemaInitializedEvent {

}
impl FromRawGameEvent for ItemSchemaInitializedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ItemSchemaInitializedEvent {

        })
    }
}

pub struct IntroFinishEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroFinishEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(IntroFinishEvent {
            player
        })
    }
}

pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl FromRawGameEvent for IntroNextCameraEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(IntroNextCameraEvent {
            player
        })
    }
}

pub struct MmLobbyChatEvent {
    pub steamid: String,
    pub text: String,
    pub kind: u16,
}
impl FromRawGameEvent for MmLobbyChatEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let steamid: String = {
            let value = values.get("steamid").ok_or(ParseError::UnknownGameEvent("steamid".to_string()))?;
            String::from_value(value.clone(), "steamid")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        let kind: u16 = {
            let value = values.get("kind").ok_or(ParseError::UnknownGameEvent("kind".to_string()))?;
            u16::from_value(value.clone(), "kind")?
        };
        Ok(MmLobbyChatEvent {
            steamid,
            text,
            kind
        })
    }
}

pub struct MmLobbyMemberJoinEvent {
    pub steamid: String,
}
impl FromRawGameEvent for MmLobbyMemberJoinEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let steamid: String = {
            let value = values.get("steamid").ok_or(ParseError::UnknownGameEvent("steamid".to_string()))?;
            String::from_value(value.clone(), "steamid")?
        };
        Ok(MmLobbyMemberJoinEvent {
            steamid
        })
    }
}

pub struct MmLobbyMemberLeaveEvent {
    pub steamid: String,
    pub flags: u32,
}
impl FromRawGameEvent for MmLobbyMemberLeaveEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let steamid: String = {
            let value = values.get("steamid").ok_or(ParseError::UnknownGameEvent("steamid".to_string()))?;
            String::from_value(value.clone(), "steamid")?
        };
        let flags: u32 = {
            let value = values.get("flags").ok_or(ParseError::UnknownGameEvent("flags".to_string()))?;
            u32::from_value(value.clone(), "flags")?
        };
        Ok(MmLobbyMemberLeaveEvent {
            steamid,
            flags
        })
    }
}

pub struct PlayerChangeClassEvent {
    pub userid: u16,
    pub class: u16,
}
impl FromRawGameEvent for PlayerChangeClassEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let class: u16 = {
            let value = values.get("class").ok_or(ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        Ok(PlayerChangeClassEvent {
            userid,
            class
        })
    }
}

pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl FromRawGameEvent for TfMapTimeRemainingEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let seconds: u32 = {
            let value = values.get("seconds").ok_or(ParseError::UnknownGameEvent("seconds".to_string()))?;
            u32::from_value(value.clone(), "seconds")?
        };
        Ok(TfMapTimeRemainingEvent {
            seconds
        })
    }
}

pub struct TfGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TfGameOverEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(TfGameOverEvent {
            reason
        })
    }
}

pub struct CtfFlagCapturedEvent {
    pub capping_team: u16,
    pub capping_team_score: u16,
}
impl FromRawGameEvent for CtfFlagCapturedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let capping_team: u16 = {
            let value = values.get("capping_team").ok_or(ParseError::UnknownGameEvent("capping_team".to_string()))?;
            u16::from_value(value.clone(), "capping_team")?
        };
        let capping_team_score: u16 = {
            let value = values.get("capping_team_score").ok_or(ParseError::UnknownGameEvent("capping_team_score".to_string()))?;
            u16::from_value(value.clone(), "capping_team_score")?
        };
        Ok(CtfFlagCapturedEvent {
            capping_team,
            capping_team_score
        })
    }
}

pub struct ControlPointInitializedEvent {

}
impl FromRawGameEvent for ControlPointInitializedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ControlPointInitializedEvent {

        })
    }
}

pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateImagesEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateImagesEvent {
            index
        })
    }
}

pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateLayoutEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateLayoutEvent {
            index
        })
    }
}

pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateCappingEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateCappingEvent {
            index
        })
    }
}

pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl FromRawGameEvent for ControlPointUpdateOwnerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ControlPointUpdateOwnerEvent {
            index
        })
    }
}

pub struct ControlPointStartTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl FromRawGameEvent for ControlPointStartTouchEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let area: u16 = {
            let value = values.get("area").ok_or(ParseError::UnknownGameEvent("area".to_string()))?;
            u16::from_value(value.clone(), "area")?
        };
        Ok(ControlPointStartTouchEvent {
            player,
            area
        })
    }
}

pub struct ControlPointEndTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl FromRawGameEvent for ControlPointEndTouchEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let area: u16 = {
            let value = values.get("area").ok_or(ParseError::UnknownGameEvent("area".to_string()))?;
            u16::from_value(value.clone(), "area")?
        };
        Ok(ControlPointEndTouchEvent {
            player,
            area
        })
    }
}

pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl FromRawGameEvent for ControlPointPulseElementEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(ControlPointPulseElementEvent {
            player
        })
    }
}

pub struct ControlPointFakeCaptureEvent {
    pub player: u16,
    pub int_data: u16,
}
impl FromRawGameEvent for ControlPointFakeCaptureEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let int_data: u16 = {
            let value = values.get("int_data").ok_or(ParseError::UnknownGameEvent("int_data".to_string()))?;
            u16::from_value(value.clone(), "int_data")?
        };
        Ok(ControlPointFakeCaptureEvent {
            player,
            int_data
        })
    }
}

pub struct ControlPointFakeCaptureMultEvent {
    pub player: u16,
    pub int_data: u16,
}
impl FromRawGameEvent for ControlPointFakeCaptureMultEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let int_data: u16 = {
            let value = values.get("int_data").ok_or(ParseError::UnknownGameEvent("int_data".to_string()))?;
            u16::from_value(value.clone(), "int_data")?
        };
        Ok(ControlPointFakeCaptureMultEvent {
            player,
            int_data
        })
    }
}

pub struct TeamPlayRoundSelectedEvent {
    pub round: String,
}
impl FromRawGameEvent for TeamPlayRoundSelectedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let round: String = {
            let value = values.get("round").ok_or(ParseError::UnknownGameEvent("round".to_string()))?;
            String::from_value(value.clone(), "round")?
        };
        Ok(TeamPlayRoundSelectedEvent {
            round
        })
    }
}

pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl FromRawGameEvent for TeamPlayRoundStartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let full_reset: bool = {
            let value = values.get("full_reset").ok_or(ParseError::UnknownGameEvent("full_reset".to_string()))?;
            bool::from_value(value.clone(), "full_reset")?
        };
        Ok(TeamPlayRoundStartEvent {
            full_reset
        })
    }
}

pub struct TeamPlayRoundActiveEvent {

}
impl FromRawGameEvent for TeamPlayRoundActiveEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayRoundActiveEvent {

        })
    }
}

pub struct TeamPlayWaitingBeginsEvent {

}
impl FromRawGameEvent for TeamPlayWaitingBeginsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingBeginsEvent {

        })
    }
}

pub struct TeamPlayWaitingEndsEvent {

}
impl FromRawGameEvent for TeamPlayWaitingEndsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingEndsEvent {

        })
    }
}

pub struct TeamPlayWaitingAboutToEndEvent {

}
impl FromRawGameEvent for TeamPlayWaitingAboutToEndEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayWaitingAboutToEndEvent {

        })
    }
}

pub struct TeamPlayRestartRoundEvent {

}
impl FromRawGameEvent for TeamPlayRestartRoundEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayRestartRoundEvent {

        })
    }
}

pub struct TeamPlayReadyRestartEvent {

}
impl FromRawGameEvent for TeamPlayReadyRestartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayReadyRestartEvent {

        })
    }
}

pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayRoundRestartSecondsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let seconds: u16 = {
            let value = values.get("seconds").ok_or(ParseError::UnknownGameEvent("seconds".to_string()))?;
            u16::from_value(value.clone(), "seconds")?
        };
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds
        })
    }
}

pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeamReadyEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayTeamReadyEvent {
            team
        })
    }
}

pub struct TeamPlayRoundWinEvent {
    pub team: u8,
    pub winreason: u8,
    pub flagcaplimit: u16,
    pub full_round: u16,
    pub round_time: f32,
    pub losing_team_num_caps: u16,
    pub was_sudden_death: u8,
}
impl FromRawGameEvent for TeamPlayRoundWinEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let winreason: u8 = {
            let value = values.get("winreason").ok_or(ParseError::UnknownGameEvent("winreason".to_string()))?;
            u8::from_value(value.clone(), "winreason")?
        };
        let flagcaplimit: u16 = {
            let value = values.get("flagcaplimit").ok_or(ParseError::UnknownGameEvent("flagcaplimit".to_string()))?;
            u16::from_value(value.clone(), "flagcaplimit")?
        };
        let full_round: u16 = {
            let value = values.get("full_round").ok_or(ParseError::UnknownGameEvent("full_round".to_string()))?;
            u16::from_value(value.clone(), "full_round")?
        };
        let round_time: f32 = {
            let value = values.get("round_time").ok_or(ParseError::UnknownGameEvent("round_time".to_string()))?;
            f32::from_value(value.clone(), "round_time")?
        };
        let losing_team_num_caps: u16 = {
            let value = values.get("losing_team_num_caps").ok_or(ParseError::UnknownGameEvent("losing_team_num_caps".to_string()))?;
            u16::from_value(value.clone(), "losing_team_num_caps")?
        };
        let was_sudden_death: u8 = {
            let value = values.get("was_sudden_death").ok_or(ParseError::UnknownGameEvent("was_sudden_death".to_string()))?;
            u8::from_value(value.clone(), "was_sudden_death")?
        };
        Ok(TeamPlayRoundWinEvent {
            team,
            winreason,
            flagcaplimit,
            full_round,
            round_time,
            losing_team_num_caps,
            was_sudden_death
        })
    }
}

pub struct TeamPlayUpdateTimerEvent {

}
impl FromRawGameEvent for TeamPlayUpdateTimerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayUpdateTimerEvent {

        })
    }
}

pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl FromRawGameEvent for TeamPlayRoundStalemateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let reason: u8 = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            u8::from_value(value.clone(), "reason")?
        };
        Ok(TeamPlayRoundStalemateEvent {
            reason
        })
    }
}

pub struct TeamPlayOvertimeBeginEvent {

}
impl FromRawGameEvent for TeamPlayOvertimeBeginEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayOvertimeBeginEvent {

        })
    }
}

pub struct TeamPlayOvertimeEndEvent {

}
impl FromRawGameEvent for TeamPlayOvertimeEndEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlayOvertimeEndEvent {

        })
    }
}

pub struct TeamPlaySuddenDeathBeginEvent {

}
impl FromRawGameEvent for TeamPlaySuddenDeathBeginEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySuddenDeathBeginEvent {

        })
    }
}

pub struct TeamPlaySuddenDeathEndEvent {

}
impl FromRawGameEvent for TeamPlaySuddenDeathEndEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySuddenDeathEndEvent {

        })
    }
}

pub struct TeamPlayGameOverEvent {
    pub reason: String,
}
impl FromRawGameEvent for TeamPlayGameOverEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let reason: String = {
            let value = values.get("reason").ok_or(ParseError::UnknownGameEvent("reason".to_string()))?;
            String::from_value(value.clone(), "reason")?
        };
        Ok(TeamPlayGameOverEvent {
            reason
        })
    }
}

pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl FromRawGameEvent for TeamPlayMapTimeRemainingEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let seconds: u16 = {
            let value = values.get("seconds").ok_or(ParseError::UnknownGameEvent("seconds".to_string()))?;
            u16::from_value(value.clone(), "seconds")?
        };
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds
        })
    }
}

pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl FromRawGameEvent for TeamPlayTimerFlashEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let time_remaining: u16 = {
            let value = values.get("time_remaining").ok_or(ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u16::from_value(value.clone(), "time_remaining")?
        };
        Ok(TeamPlayTimerFlashEvent {
            time_remaining
        })
    }
}

pub struct TeamPlayTimerTimeAddedEvent {
    pub timer: u16,
    pub seconds_added: u16,
}
impl FromRawGameEvent for TeamPlayTimerTimeAddedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let timer: u16 = {
            let value = values.get("timer").ok_or(ParseError::UnknownGameEvent("timer".to_string()))?;
            u16::from_value(value.clone(), "timer")?
        };
        let seconds_added: u16 = {
            let value = values.get("seconds_added").ok_or(ParseError::UnknownGameEvent("seconds_added".to_string()))?;
            u16::from_value(value.clone(), "seconds_added")?
        };
        Ok(TeamPlayTimerTimeAddedEvent {
            timer,
            seconds_added
        })
    }
}

pub struct TeamPlayPointStartCaptureEvent {
    pub cp: u8,
    pub cpname: String,
    pub team: u8,
    pub capteam: u8,
    pub cappers: String,
    pub captime: f32,
}
impl FromRawGameEvent for TeamPlayPointStartCaptureEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let capteam: u8 = {
            let value = values.get("capteam").ok_or(ParseError::UnknownGameEvent("capteam".to_string()))?;
            u8::from_value(value.clone(), "capteam")?
        };
        let cappers: String = {
            let value = values.get("cappers").ok_or(ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let captime: f32 = {
            let value = values.get("captime").ok_or(ParseError::UnknownGameEvent("captime".to_string()))?;
            f32::from_value(value.clone(), "captime")?
        };
        Ok(TeamPlayPointStartCaptureEvent {
            cp,
            cpname,
            team,
            capteam,
            cappers,
            captime
        })
    }
}

pub struct TeamPlayPointCapturedEvent {
    pub cp: u8,
    pub cpname: String,
    pub team: u8,
    pub cappers: String,
}
impl FromRawGameEvent for TeamPlayPointCapturedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let cappers: String = {
            let value = values.get("cappers").ok_or(ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        Ok(TeamPlayPointCapturedEvent {
            cp,
            cpname,
            team,
            cappers
        })
    }
}

pub struct TeamPlayPointLockedEvent {
    pub cp: u8,
    pub cpname: String,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayPointLockedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayPointLockedEvent {
            cp,
            cpname,
            team
        })
    }
}

pub struct TeamPlayPointUnlockedEvent {
    pub cp: u8,
    pub cpname: String,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayPointUnlockedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayPointUnlockedEvent {
            cp,
            cpname,
            team
        })
    }
}

pub struct TeamPlayCaptureBrokenEvent {
    pub cp: u8,
    pub cpname: String,
    pub time_remaining: f32,
}
impl FromRawGameEvent for TeamPlayCaptureBrokenEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let time_remaining: f32 = {
            let value = values.get("time_remaining").ok_or(ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            f32::from_value(value.clone(), "time_remaining")?
        };
        Ok(TeamPlayCaptureBrokenEvent {
            cp,
            cpname,
            time_remaining
        })
    }
}

pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cpname: String,
    pub blocker: u8,
}
impl FromRawGameEvent for TeamPlayCaptureBlockedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let cp: u8 = {
            let value = values.get("cp").ok_or(ParseError::UnknownGameEvent("cp".to_string()))?;
            u8::from_value(value.clone(), "cp")?
        };
        let cpname: String = {
            let value = values.get("cpname").ok_or(ParseError::UnknownGameEvent("cpname".to_string()))?;
            String::from_value(value.clone(), "cpname")?
        };
        let blocker: u8 = {
            let value = values.get("blocker").ok_or(ParseError::UnknownGameEvent("blocker".to_string()))?;
            u8::from_value(value.clone(), "blocker")?
        };
        Ok(TeamPlayCaptureBlockedEvent {
            cp,
            cpname,
            blocker
        })
    }
}

pub struct TeamPlayFlagEventEvent {
    pub player: u16,
    pub carrier: u16,
    pub eventtype: u16,
    pub home: u8,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayFlagEventEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let carrier: u16 = {
            let value = values.get("carrier").ok_or(ParseError::UnknownGameEvent("carrier".to_string()))?;
            u16::from_value(value.clone(), "carrier")?
        };
        let eventtype: u16 = {
            let value = values.get("eventtype").ok_or(ParseError::UnknownGameEvent("eventtype".to_string()))?;
            u16::from_value(value.clone(), "eventtype")?
        };
        let home: u8 = {
            let value = values.get("home").ok_or(ParseError::UnknownGameEvent("home".to_string()))?;
            u8::from_value(value.clone(), "home")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayFlagEventEvent {
            player,
            carrier,
            eventtype,
            home,
            team
        })
    }
}

pub struct TeamPlayWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub winreason: u8,
    pub cappers: String,
    pub flagcaplimit: u16,
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
    pub killstreak_player_1: u16,
    pub killstreak_player_1_count: u16,
}
impl FromRawGameEvent for TeamPlayWinPanelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let panel_style: u8 = {
            let value = values.get("panel_style").ok_or(ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get("winning_team").ok_or(ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let winreason: u8 = {
            let value = values.get("winreason").ok_or(ParseError::UnknownGameEvent("winreason".to_string()))?;
            u8::from_value(value.clone(), "winreason")?
        };
        let cappers: String = {
            let value = values.get("cappers").ok_or(ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let flagcaplimit: u16 = {
            let value = values.get("flagcaplimit").ok_or(ParseError::UnknownGameEvent("flagcaplimit".to_string()))?;
            u16::from_value(value.clone(), "flagcaplimit")?
        };
        let blue_score: u16 = {
            let value = values.get("blue_score").ok_or(ParseError::UnknownGameEvent("blue_score".to_string()))?;
            u16::from_value(value.clone(), "blue_score")?
        };
        let red_score: u16 = {
            let value = values.get("red_score").ok_or(ParseError::UnknownGameEvent("red_score".to_string()))?;
            u16::from_value(value.clone(), "red_score")?
        };
        let blue_score_prev: u16 = {
            let value = values.get("blue_score_prev").ok_or(ParseError::UnknownGameEvent("blue_score_prev".to_string()))?;
            u16::from_value(value.clone(), "blue_score_prev")?
        };
        let red_score_prev: u16 = {
            let value = values.get("red_score_prev").ok_or(ParseError::UnknownGameEvent("red_score_prev".to_string()))?;
            u16::from_value(value.clone(), "red_score_prev")?
        };
        let round_complete: u16 = {
            let value = values.get("round_complete").ok_or(ParseError::UnknownGameEvent("round_complete".to_string()))?;
            u16::from_value(value.clone(), "round_complete")?
        };
        let rounds_remaining: u16 = {
            let value = values.get("rounds_remaining").ok_or(ParseError::UnknownGameEvent("rounds_remaining".to_string()))?;
            u16::from_value(value.clone(), "rounds_remaining")?
        };
        let player_1: u16 = {
            let value = values.get("player_1").ok_or(ParseError::UnknownGameEvent("player_1".to_string()))?;
            u16::from_value(value.clone(), "player_1")?
        };
        let player_1_points: u16 = {
            let value = values.get("player_1_points").ok_or(ParseError::UnknownGameEvent("player_1_points".to_string()))?;
            u16::from_value(value.clone(), "player_1_points")?
        };
        let player_2: u16 = {
            let value = values.get("player_2").ok_or(ParseError::UnknownGameEvent("player_2".to_string()))?;
            u16::from_value(value.clone(), "player_2")?
        };
        let player_2_points: u16 = {
            let value = values.get("player_2_points").ok_or(ParseError::UnknownGameEvent("player_2_points".to_string()))?;
            u16::from_value(value.clone(), "player_2_points")?
        };
        let player_3: u16 = {
            let value = values.get("player_3").ok_or(ParseError::UnknownGameEvent("player_3".to_string()))?;
            u16::from_value(value.clone(), "player_3")?
        };
        let player_3_points: u16 = {
            let value = values.get("player_3_points").ok_or(ParseError::UnknownGameEvent("player_3_points".to_string()))?;
            u16::from_value(value.clone(), "player_3_points")?
        };
        let killstreak_player_1: u16 = {
            let value = values.get("killstreak_player_1").ok_or(ParseError::UnknownGameEvent("killstreak_player_1".to_string()))?;
            u16::from_value(value.clone(), "killstreak_player_1")?
        };
        let killstreak_player_1_count: u16 = {
            let value = values.get("killstreak_player_1_count").ok_or(ParseError::UnknownGameEvent("killstreak_player_1_count".to_string()))?;
            u16::from_value(value.clone(), "killstreak_player_1_count")?
        };
        Ok(TeamPlayWinPanelEvent {
            panel_style,
            winning_team,
            winreason,
            cappers,
            flagcaplimit,
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
            killstreak_player_1,
            killstreak_player_1_count
        })
    }
}

pub struct TeamPlayTeambalancedPlayerEvent {
    pub player: u16,
    pub team: u8,
}
impl FromRawGameEvent for TeamPlayTeambalancedPlayerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(TeamPlayTeambalancedPlayerEvent {
            player,
            team
        })
    }
}

pub struct TeamPlaySetupFinishedEvent {

}
impl FromRawGameEvent for TeamPlaySetupFinishedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamPlaySetupFinishedEvent {

        })
    }
}

pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl FromRawGameEvent for TeamPlayAlertEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let alert_type: u16 = {
            let value = values.get("alert_type").ok_or(ParseError::UnknownGameEvent("alert_type".to_string()))?;
            u16::from_value(value.clone(), "alert_type")?
        };
        Ok(TeamPlayAlertEvent {
            alert_type
        })
    }
}

pub struct TrainingCompleteEvent {
    pub next_map: String,
    pub map: String,
    pub text: String,
}
impl FromRawGameEvent for TrainingCompleteEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let next_map: String = {
            let value = values.get("next_map").ok_or(ParseError::UnknownGameEvent("next_map".to_string()))?;
            String::from_value(value.clone(), "next_map")?
        };
        let map: String = {
            let value = values.get("map").ok_or(ParseError::UnknownGameEvent("map".to_string()))?;
            String::from_value(value.clone(), "map")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(TrainingCompleteEvent {
            next_map,
            map,
            text
        })
    }
}

pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl FromRawGameEvent for ShowFreezePanelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let killer: u16 = {
            let value = values.get("killer").ok_or(ParseError::UnknownGameEvent("killer".to_string()))?;
            u16::from_value(value.clone(), "killer")?
        };
        Ok(ShowFreezePanelEvent {
            killer
        })
    }
}

pub struct HideFreezePanelEvent {

}
impl FromRawGameEvent for HideFreezePanelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(HideFreezePanelEvent {

        })
    }
}

pub struct FreezeCamStartedEvent {

}
impl FromRawGameEvent for FreezeCamStartedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(FreezeCamStartedEvent {

        })
    }
}

pub struct LocalPlayerChangeTeamEvent {

}
impl FromRawGameEvent for LocalPlayerChangeTeamEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChangeTeamEvent {

        })
    }
}

pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl FromRawGameEvent for LocalPlayerScoreChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let score: u16 = {
            let value = values.get("score").ok_or(ParseError::UnknownGameEvent("score".to_string()))?;
            u16::from_value(value.clone(), "score")?
        };
        Ok(LocalPlayerScoreChangedEvent {
            score
        })
    }
}

pub struct LocalPlayerChangeClassEvent {

}
impl FromRawGameEvent for LocalPlayerChangeClassEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChangeClassEvent {

        })
    }
}

pub struct LocalPlayerRespawnEvent {

}
impl FromRawGameEvent for LocalPlayerRespawnEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerRespawnEvent {

        })
    }
}

pub struct BuildingInfoChangedEvent {
    pub building_type: u8,
    pub object_mode: u8,
    pub remove: u8,
}
impl FromRawGameEvent for BuildingInfoChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let building_type: u8 = {
            let value = values.get("building_type").ok_or(ParseError::UnknownGameEvent("building_type".to_string()))?;
            u8::from_value(value.clone(), "building_type")?
        };
        let object_mode: u8 = {
            let value = values.get("object_mode").ok_or(ParseError::UnknownGameEvent("object_mode".to_string()))?;
            u8::from_value(value.clone(), "object_mode")?
        };
        let remove: u8 = {
            let value = values.get("remove").ok_or(ParseError::UnknownGameEvent("remove".to_string()))?;
            u8::from_value(value.clone(), "remove")?
        };
        Ok(BuildingInfoChangedEvent {
            building_type,
            object_mode,
            remove
        })
    }
}

pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl FromRawGameEvent for LocalPlayerChangeDisguiseEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let disguised: bool = {
            let value = values.get("disguised").ok_or(ParseError::UnknownGameEvent("disguised".to_string()))?;
            bool::from_value(value.clone(), "disguised")?
        };
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised
        })
    }
}

pub struct PlayerAccountChangedEvent {
    pub old_value: u16,
    pub new_value: u16,
}
impl FromRawGameEvent for PlayerAccountChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let old_value: u16 = {
            let value = values.get("old_value").ok_or(ParseError::UnknownGameEvent("old_value".to_string()))?;
            u16::from_value(value.clone(), "old_value")?
        };
        let new_value: u16 = {
            let value = values.get("new_value").ok_or(ParseError::UnknownGameEvent("new_value".to_string()))?;
            u16::from_value(value.clone(), "new_value")?
        };
        Ok(PlayerAccountChangedEvent {
            old_value,
            new_value
        })
    }
}

pub struct SpyPdaResetEvent {

}
impl FromRawGameEvent for SpyPdaResetEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(SpyPdaResetEvent {

        })
    }
}

pub struct FlagStatusUpdateEvent {
    pub userid: u16,
    pub entindex: u32,
}
impl FromRawGameEvent for FlagStatusUpdateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let entindex: u32 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u32::from_value(value.clone(), "entindex")?
        };
        Ok(FlagStatusUpdateEvent {
            userid,
            entindex
        })
    }
}

pub struct PlayerStatsUpdatedEvent {
    pub forceupload: bool,
}
impl FromRawGameEvent for PlayerStatsUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let forceupload: bool = {
            let value = values.get("forceupload").ok_or(ParseError::UnknownGameEvent("forceupload".to_string()))?;
            bool::from_value(value.clone(), "forceupload")?
        };
        Ok(PlayerStatsUpdatedEvent {
            forceupload
        })
    }
}

pub struct PlayingCommentaryEvent {

}
impl FromRawGameEvent for PlayingCommentaryEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PlayingCommentaryEvent {

        })
    }
}

pub struct PlayerChargedeployedEvent {
    pub userid: u16,
    pub targetid: u16,
}
impl FromRawGameEvent for PlayerChargedeployedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let targetid: u16 = {
            let value = values.get("targetid").ok_or(ParseError::UnknownGameEvent("targetid".to_string()))?;
            u16::from_value(value.clone(), "targetid")?
        };
        Ok(PlayerChargedeployedEvent {
            userid,
            targetid
        })
    }
}

pub struct PlayerBuiltObjectEvent {
    pub userid: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerBuiltObjectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let object: u16 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerBuiltObjectEvent {
            userid,
            object,
            index
        })
    }
}

pub struct PlayerUpgradedObjectEvent {
    pub userid: u16,
    pub object: u16,
    pub index: u16,
    pub isbuilder: bool,
}
impl FromRawGameEvent for PlayerUpgradedObjectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let object: u16 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let isbuilder: bool = {
            let value = values.get("isbuilder").ok_or(ParseError::UnknownGameEvent("isbuilder".to_string()))?;
            bool::from_value(value.clone(), "isbuilder")?
        };
        Ok(PlayerUpgradedObjectEvent {
            userid,
            object,
            index,
            isbuilder
        })
    }
}

pub struct PlayerCarryObjectEvent {
    pub userid: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerCarryObjectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let object: u16 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerCarryObjectEvent {
            userid,
            object,
            index
        })
    }
}

pub struct PlayerDropObjectEvent {
    pub userid: u16,
    pub object: u16,
    pub index: u16,
}
impl FromRawGameEvent for PlayerDropObjectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let object: u16 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u16::from_value(value.clone(), "object")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerDropObjectEvent {
            userid,
            object,
            index
        })
    }
}

pub struct ObjectRemovedEvent {
    pub userid: u16,
    pub objecttype: u16,
    pub index: u16,
}
impl FromRawGameEvent for ObjectRemovedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let objecttype: u16 = {
            let value = values.get("objecttype").ok_or(ParseError::UnknownGameEvent("objecttype".to_string()))?;
            u16::from_value(value.clone(), "objecttype")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ObjectRemovedEvent {
            userid,
            objecttype,
            index
        })
    }
}

pub struct ObjectDestroyedEvent {
    pub userid: u16,
    pub attacker: u16,
    pub assister: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub objecttype: u16,
    pub index: u16,
    pub was_building: bool,
}
impl FromRawGameEvent for ObjectDestroyedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let assister: u16 = {
            let value = values.get("assister").ok_or(ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let objecttype: u16 = {
            let value = values.get("objecttype").ok_or(ParseError::UnknownGameEvent("objecttype".to_string()))?;
            u16::from_value(value.clone(), "objecttype")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let was_building: bool = {
            let value = values.get("was_building").ok_or(ParseError::UnknownGameEvent("was_building".to_string()))?;
            bool::from_value(value.clone(), "was_building")?
        };
        Ok(ObjectDestroyedEvent {
            userid,
            attacker,
            assister,
            weapon,
            weaponid,
            objecttype,
            index,
            was_building
        })
    }
}

pub struct ObjectDetonatedEvent {
    pub userid: u16,
    pub objecttype: u16,
    pub index: u16,
}
impl FromRawGameEvent for ObjectDetonatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let objecttype: u16 = {
            let value = values.get("objecttype").ok_or(ParseError::UnknownGameEvent("objecttype".to_string()))?;
            u16::from_value(value.clone(), "objecttype")?
        };
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ObjectDetonatedEvent {
            userid,
            objecttype,
            index
        })
    }
}

pub struct AchievementEarnedEvent {
    pub player: u8,
    pub achievement: u16,
}
impl FromRawGameEvent for AchievementEarnedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let achievement: u16 = {
            let value = values.get("achievement").ok_or(ParseError::UnknownGameEvent("achievement".to_string()))?;
            u16::from_value(value.clone(), "achievement")?
        };
        Ok(AchievementEarnedEvent {
            player,
            achievement
        })
    }
}

pub struct SpecTargetUpdatedEvent {

}
impl FromRawGameEvent for SpecTargetUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(SpecTargetUpdatedEvent {

        })
    }
}

pub struct TournamentStateUpdateEvent {
    pub userid: u16,
    pub namechange: bool,
    pub readystate: u16,
    pub newname: String,
}
impl FromRawGameEvent for TournamentStateUpdateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let namechange: bool = {
            let value = values.get("namechange").ok_or(ParseError::UnknownGameEvent("namechange".to_string()))?;
            bool::from_value(value.clone(), "namechange")?
        };
        let readystate: u16 = {
            let value = values.get("readystate").ok_or(ParseError::UnknownGameEvent("readystate".to_string()))?;
            u16::from_value(value.clone(), "readystate")?
        };
        let newname: String = {
            let value = values.get("newname").ok_or(ParseError::UnknownGameEvent("newname".to_string()))?;
            String::from_value(value.clone(), "newname")?
        };
        Ok(TournamentStateUpdateEvent {
            userid,
            namechange,
            readystate,
            newname
        })
    }
}

pub struct TournamentEnableCountdownEvent {

}
impl FromRawGameEvent for TournamentEnableCountdownEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TournamentEnableCountdownEvent {

        })
    }
}

pub struct PlayerCalledForMedicEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PlayerCalledForMedicEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PlayerCalledForMedicEvent {
            userid
        })
    }
}

pub struct LocalPlayerBecameObserverEvent {

}
impl FromRawGameEvent for LocalPlayerBecameObserverEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerBecameObserverEvent {

        })
    }
}

pub struct PlayerIgnitedInvEvent {
    pub pyro_entindex: u8,
    pub victim_entindex: u8,
    pub medic_entindex: u8,
}
impl FromRawGameEvent for PlayerIgnitedInvEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let pyro_entindex: u8 = {
            let value = values.get("pyro_entindex").ok_or(ParseError::UnknownGameEvent("pyro_entindex".to_string()))?;
            u8::from_value(value.clone(), "pyro_entindex")?
        };
        let victim_entindex: u8 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u8::from_value(value.clone(), "victim_entindex")?
        };
        let medic_entindex: u8 = {
            let value = values.get("medic_entindex").ok_or(ParseError::UnknownGameEvent("medic_entindex".to_string()))?;
            u8::from_value(value.clone(), "medic_entindex")?
        };
        Ok(PlayerIgnitedInvEvent {
            pyro_entindex,
            victim_entindex,
            medic_entindex
        })
    }
}

pub struct PlayerIgnitedEvent {
    pub pyro_entindex: u8,
    pub victim_entindex: u8,
    pub weaponid: u8,
}
impl FromRawGameEvent for PlayerIgnitedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let pyro_entindex: u8 = {
            let value = values.get("pyro_entindex").ok_or(ParseError::UnknownGameEvent("pyro_entindex".to_string()))?;
            u8::from_value(value.clone(), "pyro_entindex")?
        };
        let victim_entindex: u8 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u8::from_value(value.clone(), "victim_entindex")?
        };
        let weaponid: u8 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u8::from_value(value.clone(), "weaponid")?
        };
        Ok(PlayerIgnitedEvent {
            pyro_entindex,
            victim_entindex,
            weaponid
        })
    }
}

pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
}
impl FromRawGameEvent for PlayerExtinguishedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let victim: u8 = {
            let value = values.get("victim").ok_or(ParseError::UnknownGameEvent("victim".to_string()))?;
            u8::from_value(value.clone(), "victim")?
        };
        let healer: u8 = {
            let value = values.get("healer").ok_or(ParseError::UnknownGameEvent("healer".to_string()))?;
            u8::from_value(value.clone(), "healer")?
        };
        Ok(PlayerExtinguishedEvent {
            victim,
            healer
        })
    }
}

pub struct PlayerTeleportedEvent {
    pub userid: u16,
    pub builderid: u16,
    pub dist: f32,
}
impl FromRawGameEvent for PlayerTeleportedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let builderid: u16 = {
            let value = values.get("builderid").ok_or(ParseError::UnknownGameEvent("builderid".to_string()))?;
            u16::from_value(value.clone(), "builderid")?
        };
        let dist: f32 = {
            let value = values.get("dist").ok_or(ParseError::UnknownGameEvent("dist".to_string()))?;
            f32::from_value(value.clone(), "dist")?
        };
        Ok(PlayerTeleportedEvent {
            userid,
            builderid,
            dist
        })
    }
}

pub struct PlayerHealedMedicCallEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PlayerHealedMedicCallEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PlayerHealedMedicCallEvent {
            userid
        })
    }
}

pub struct LocalPlayerChargeReadyEvent {

}
impl FromRawGameEvent for LocalPlayerChargeReadyEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerChargeReadyEvent {

        })
    }
}

pub struct LocalPlayerWinddownEvent {

}
impl FromRawGameEvent for LocalPlayerWinddownEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LocalPlayerWinddownEvent {

        })
    }
}

pub struct PlayerInvulnedEvent {
    pub userid: u16,
    pub medic_userid: u16,
}
impl FromRawGameEvent for PlayerInvulnedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let medic_userid: u16 = {
            let value = values.get("medic_userid").ok_or(ParseError::UnknownGameEvent("medic_userid".to_string()))?;
            u16::from_value(value.clone(), "medic_userid")?
        };
        Ok(PlayerInvulnedEvent {
            userid,
            medic_userid
        })
    }
}

pub struct EscortSpeedEvent {
    pub team: u8,
    pub speed: u8,
    pub players: u8,
}
impl FromRawGameEvent for EscortSpeedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let speed: u8 = {
            let value = values.get("speed").ok_or(ParseError::UnknownGameEvent("speed".to_string()))?;
            u8::from_value(value.clone(), "speed")?
        };
        let players: u8 = {
            let value = values.get("players").ok_or(ParseError::UnknownGameEvent("players".to_string()))?;
            u8::from_value(value.clone(), "players")?
        };
        Ok(EscortSpeedEvent {
            team,
            speed,
            players
        })
    }
}

pub struct EscortProgressEvent {
    pub team: u8,
    pub progress: f32,
    pub reset: bool,
}
impl FromRawGameEvent for EscortProgressEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let progress: f32 = {
            let value = values.get("progress").ok_or(ParseError::UnknownGameEvent("progress".to_string()))?;
            f32::from_value(value.clone(), "progress")?
        };
        let reset: bool = {
            let value = values.get("reset").ok_or(ParseError::UnknownGameEvent("reset".to_string()))?;
            bool::from_value(value.clone(), "reset")?
        };
        Ok(EscortProgressEvent {
            team,
            progress,
            reset
        })
    }
}

pub struct EscortRecedeEvent {
    pub team: u8,
    pub recedetime: f32,
}
impl FromRawGameEvent for EscortRecedeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let recedetime: f32 = {
            let value = values.get("recedetime").ok_or(ParseError::UnknownGameEvent("recedetime".to_string()))?;
            f32::from_value(value.clone(), "recedetime")?
        };
        Ok(EscortRecedeEvent {
            team,
            recedetime
        })
    }
}

pub struct GameUIActivatedEvent {

}
impl FromRawGameEvent for GameUIActivatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(GameUIActivatedEvent {

        })
    }
}

pub struct GameUIHiddenEvent {

}
impl FromRawGameEvent for GameUIHiddenEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(GameUIHiddenEvent {

        })
    }
}

pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl FromRawGameEvent for PlayerEscortScoreEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let points: u8 = {
            let value = values.get("points").ok_or(ParseError::UnknownGameEvent("points".to_string()))?;
            u8::from_value(value.clone(), "points")?
        };
        Ok(PlayerEscortScoreEvent {
            player,
            points
        })
    }
}

pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub entindex: u8,
}
impl FromRawGameEvent for PlayerHealOnHitEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let amount: u16 = {
            let value = values.get("amount").ok_or(ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        let entindex: u8 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u8::from_value(value.clone(), "entindex")?
        };
        Ok(PlayerHealOnHitEvent {
            amount,
            entindex
        })
    }
}

pub struct PlayerStealsandvichEvent {
    pub owner: u16,
    pub target: u16,
}
impl FromRawGameEvent for PlayerStealsandvichEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let owner: u16 = {
            let value = values.get("owner").ok_or(ParseError::UnknownGameEvent("owner".to_string()))?;
            u16::from_value(value.clone(), "owner")?
        };
        let target: u16 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(PlayerStealsandvichEvent {
            owner,
            target
        })
    }
}

pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowClassLayoutEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let show: bool = {
            let value = values.get("show").ok_or(ParseError::UnknownGameEvent("show".to_string()))?;
            bool::from_value(value.clone(), "show")?
        };
        Ok(ShowClassLayoutEvent {
            show
        })
    }
}

pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl FromRawGameEvent for ShowVsPanelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let show: bool = {
            let value = values.get("show").ok_or(ParseError::UnknownGameEvent("show".to_string()))?;
            bool::from_value(value.clone(), "show")?
        };
        Ok(ShowVsPanelEvent {
            show
        })
    }
}

pub struct PlayerDamagedEvent {
    pub amount: u16,
    pub kind: u32,
}
impl FromRawGameEvent for PlayerDamagedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let amount: u16 = {
            let value = values.get("amount").ok_or(ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        let kind: u32 = {
            let value = values.get("kind").ok_or(ParseError::UnknownGameEvent("kind".to_string()))?;
            u32::from_value(value.clone(), "kind")?
        };
        Ok(PlayerDamagedEvent {
            amount,
            kind
        })
    }
}

pub struct ArenaPlayerNotificationEvent {
    pub player: u8,
    pub message: u8,
}
impl FromRawGameEvent for ArenaPlayerNotificationEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let message: u8 = {
            let value = values.get("message").ok_or(ParseError::UnknownGameEvent("message".to_string()))?;
            u8::from_value(value.clone(), "message")?
        };
        Ok(ArenaPlayerNotificationEvent {
            player,
            message
        })
    }
}

pub struct ArenaMatchMaxStreakEvent {
    pub team: u8,
    pub streak: u8,
}
impl FromRawGameEvent for ArenaMatchMaxStreakEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let streak: u8 = {
            let value = values.get("streak").ok_or(ParseError::UnknownGameEvent("streak".to_string()))?;
            u8::from_value(value.clone(), "streak")?
        };
        Ok(ArenaMatchMaxStreakEvent {
            team,
            streak
        })
    }
}

pub struct ArenaRoundStartEvent {

}
impl FromRawGameEvent for ArenaRoundStartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ArenaRoundStartEvent {

        })
    }
}

pub struct ArenaWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub winreason: u8,
    pub cappers: String,
    pub flagcaplimit: u16,
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
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let panel_style: u8 = {
            let value = values.get("panel_style").ok_or(ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get("winning_team").ok_or(ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let winreason: u8 = {
            let value = values.get("winreason").ok_or(ParseError::UnknownGameEvent("winreason".to_string()))?;
            u8::from_value(value.clone(), "winreason")?
        };
        let cappers: String = {
            let value = values.get("cappers").ok_or(ParseError::UnknownGameEvent("cappers".to_string()))?;
            String::from_value(value.clone(), "cappers")?
        };
        let flagcaplimit: u16 = {
            let value = values.get("flagcaplimit").ok_or(ParseError::UnknownGameEvent("flagcaplimit".to_string()))?;
            u16::from_value(value.clone(), "flagcaplimit")?
        };
        let blue_score: u16 = {
            let value = values.get("blue_score").ok_or(ParseError::UnknownGameEvent("blue_score".to_string()))?;
            u16::from_value(value.clone(), "blue_score")?
        };
        let red_score: u16 = {
            let value = values.get("red_score").ok_or(ParseError::UnknownGameEvent("red_score".to_string()))?;
            u16::from_value(value.clone(), "red_score")?
        };
        let blue_score_prev: u16 = {
            let value = values.get("blue_score_prev").ok_or(ParseError::UnknownGameEvent("blue_score_prev".to_string()))?;
            u16::from_value(value.clone(), "blue_score_prev")?
        };
        let red_score_prev: u16 = {
            let value = values.get("red_score_prev").ok_or(ParseError::UnknownGameEvent("red_score_prev".to_string()))?;
            u16::from_value(value.clone(), "red_score_prev")?
        };
        let round_complete: u16 = {
            let value = values.get("round_complete").ok_or(ParseError::UnknownGameEvent("round_complete".to_string()))?;
            u16::from_value(value.clone(), "round_complete")?
        };
        let player_1: u16 = {
            let value = values.get("player_1").ok_or(ParseError::UnknownGameEvent("player_1".to_string()))?;
            u16::from_value(value.clone(), "player_1")?
        };
        let player_1_damage: u16 = {
            let value = values.get("player_1_damage").ok_or(ParseError::UnknownGameEvent("player_1_damage".to_string()))?;
            u16::from_value(value.clone(), "player_1_damage")?
        };
        let player_1_healing: u16 = {
            let value = values.get("player_1_healing").ok_or(ParseError::UnknownGameEvent("player_1_healing".to_string()))?;
            u16::from_value(value.clone(), "player_1_healing")?
        };
        let player_1_lifetime: u16 = {
            let value = values.get("player_1_lifetime").ok_or(ParseError::UnknownGameEvent("player_1_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_1_lifetime")?
        };
        let player_1_kills: u16 = {
            let value = values.get("player_1_kills").ok_or(ParseError::UnknownGameEvent("player_1_kills".to_string()))?;
            u16::from_value(value.clone(), "player_1_kills")?
        };
        let player_2: u16 = {
            let value = values.get("player_2").ok_or(ParseError::UnknownGameEvent("player_2".to_string()))?;
            u16::from_value(value.clone(), "player_2")?
        };
        let player_2_damage: u16 = {
            let value = values.get("player_2_damage").ok_or(ParseError::UnknownGameEvent("player_2_damage".to_string()))?;
            u16::from_value(value.clone(), "player_2_damage")?
        };
        let player_2_healing: u16 = {
            let value = values.get("player_2_healing").ok_or(ParseError::UnknownGameEvent("player_2_healing".to_string()))?;
            u16::from_value(value.clone(), "player_2_healing")?
        };
        let player_2_lifetime: u16 = {
            let value = values.get("player_2_lifetime").ok_or(ParseError::UnknownGameEvent("player_2_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_2_lifetime")?
        };
        let player_2_kills: u16 = {
            let value = values.get("player_2_kills").ok_or(ParseError::UnknownGameEvent("player_2_kills".to_string()))?;
            u16::from_value(value.clone(), "player_2_kills")?
        };
        let player_3: u16 = {
            let value = values.get("player_3").ok_or(ParseError::UnknownGameEvent("player_3".to_string()))?;
            u16::from_value(value.clone(), "player_3")?
        };
        let player_3_damage: u16 = {
            let value = values.get("player_3_damage").ok_or(ParseError::UnknownGameEvent("player_3_damage".to_string()))?;
            u16::from_value(value.clone(), "player_3_damage")?
        };
        let player_3_healing: u16 = {
            let value = values.get("player_3_healing").ok_or(ParseError::UnknownGameEvent("player_3_healing".to_string()))?;
            u16::from_value(value.clone(), "player_3_healing")?
        };
        let player_3_lifetime: u16 = {
            let value = values.get("player_3_lifetime").ok_or(ParseError::UnknownGameEvent("player_3_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_3_lifetime")?
        };
        let player_3_kills: u16 = {
            let value = values.get("player_3_kills").ok_or(ParseError::UnknownGameEvent("player_3_kills".to_string()))?;
            u16::from_value(value.clone(), "player_3_kills")?
        };
        let player_4: u16 = {
            let value = values.get("player_4").ok_or(ParseError::UnknownGameEvent("player_4".to_string()))?;
            u16::from_value(value.clone(), "player_4")?
        };
        let player_4_damage: u16 = {
            let value = values.get("player_4_damage").ok_or(ParseError::UnknownGameEvent("player_4_damage".to_string()))?;
            u16::from_value(value.clone(), "player_4_damage")?
        };
        let player_4_healing: u16 = {
            let value = values.get("player_4_healing").ok_or(ParseError::UnknownGameEvent("player_4_healing".to_string()))?;
            u16::from_value(value.clone(), "player_4_healing")?
        };
        let player_4_lifetime: u16 = {
            let value = values.get("player_4_lifetime").ok_or(ParseError::UnknownGameEvent("player_4_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_4_lifetime")?
        };
        let player_4_kills: u16 = {
            let value = values.get("player_4_kills").ok_or(ParseError::UnknownGameEvent("player_4_kills".to_string()))?;
            u16::from_value(value.clone(), "player_4_kills")?
        };
        let player_5: u16 = {
            let value = values.get("player_5").ok_or(ParseError::UnknownGameEvent("player_5".to_string()))?;
            u16::from_value(value.clone(), "player_5")?
        };
        let player_5_damage: u16 = {
            let value = values.get("player_5_damage").ok_or(ParseError::UnknownGameEvent("player_5_damage".to_string()))?;
            u16::from_value(value.clone(), "player_5_damage")?
        };
        let player_5_healing: u16 = {
            let value = values.get("player_5_healing").ok_or(ParseError::UnknownGameEvent("player_5_healing".to_string()))?;
            u16::from_value(value.clone(), "player_5_healing")?
        };
        let player_5_lifetime: u16 = {
            let value = values.get("player_5_lifetime").ok_or(ParseError::UnknownGameEvent("player_5_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_5_lifetime")?
        };
        let player_5_kills: u16 = {
            let value = values.get("player_5_kills").ok_or(ParseError::UnknownGameEvent("player_5_kills".to_string()))?;
            u16::from_value(value.clone(), "player_5_kills")?
        };
        let player_6: u16 = {
            let value = values.get("player_6").ok_or(ParseError::UnknownGameEvent("player_6".to_string()))?;
            u16::from_value(value.clone(), "player_6")?
        };
        let player_6_damage: u16 = {
            let value = values.get("player_6_damage").ok_or(ParseError::UnknownGameEvent("player_6_damage".to_string()))?;
            u16::from_value(value.clone(), "player_6_damage")?
        };
        let player_6_healing: u16 = {
            let value = values.get("player_6_healing").ok_or(ParseError::UnknownGameEvent("player_6_healing".to_string()))?;
            u16::from_value(value.clone(), "player_6_healing")?
        };
        let player_6_lifetime: u16 = {
            let value = values.get("player_6_lifetime").ok_or(ParseError::UnknownGameEvent("player_6_lifetime".to_string()))?;
            u16::from_value(value.clone(), "player_6_lifetime")?
        };
        let player_6_kills: u16 = {
            let value = values.get("player_6_kills").ok_or(ParseError::UnknownGameEvent("player_6_kills".to_string()))?;
            u16::from_value(value.clone(), "player_6_kills")?
        };
        Ok(ArenaWinPanelEvent {
            panel_style,
            winning_team,
            winreason,
            cappers,
            flagcaplimit,
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

pub struct PveWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub winreason: u8,
}
impl FromRawGameEvent for PveWinPanelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let panel_style: u8 = {
            let value = values.get("panel_style").ok_or(ParseError::UnknownGameEvent("panel_style".to_string()))?;
            u8::from_value(value.clone(), "panel_style")?
        };
        let winning_team: u8 = {
            let value = values.get("winning_team").ok_or(ParseError::UnknownGameEvent("winning_team".to_string()))?;
            u8::from_value(value.clone(), "winning_team")?
        };
        let winreason: u8 = {
            let value = values.get("winreason").ok_or(ParseError::UnknownGameEvent("winreason".to_string()))?;
            u8::from_value(value.clone(), "winreason")?
        };
        Ok(PveWinPanelEvent {
            panel_style,
            winning_team,
            winreason
        })
    }
}

pub struct AirDashEvent {
    pub player: u8,
}
impl FromRawGameEvent for AirDashEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        Ok(AirDashEvent {
            player
        })
    }
}

pub struct LandedEvent {
    pub player: u8,
}
impl FromRawGameEvent for LandedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        Ok(LandedEvent {
            player
        })
    }
}

pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl FromRawGameEvent for PlayerDamageDodgedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let damage: u16 = {
            let value = values.get("damage").ok_or(ParseError::UnknownGameEvent("damage".to_string()))?;
            u16::from_value(value.clone(), "damage")?
        };
        Ok(PlayerDamageDodgedEvent {
            damage
        })
    }
}

pub struct PlayerStunnedEvent {
    pub stunner: u16,
    pub victim: u16,
    pub victim_capping: bool,
    pub big_stun: bool,
}
impl FromRawGameEvent for PlayerStunnedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let stunner: u16 = {
            let value = values.get("stunner").ok_or(ParseError::UnknownGameEvent("stunner".to_string()))?;
            u16::from_value(value.clone(), "stunner")?
        };
        let victim: u16 = {
            let value = values.get("victim").ok_or(ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        let victim_capping: bool = {
            let value = values.get("victim_capping").ok_or(ParseError::UnknownGameEvent("victim_capping".to_string()))?;
            bool::from_value(value.clone(), "victim_capping")?
        };
        let big_stun: bool = {
            let value = values.get("big_stun").ok_or(ParseError::UnknownGameEvent("big_stun".to_string()))?;
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

pub struct ScoutGrandSlamEvent {
    pub scout_id: u16,
    pub target_id: u16,
}
impl FromRawGameEvent for ScoutGrandSlamEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let scout_id: u16 = {
            let value = values.get("scout_id").ok_or(ParseError::UnknownGameEvent("scout_id".to_string()))?;
            u16::from_value(value.clone(), "scout_id")?
        };
        let target_id: u16 = {
            let value = values.get("target_id").ok_or(ParseError::UnknownGameEvent("target_id".to_string()))?;
            u16::from_value(value.clone(), "target_id")?
        };
        Ok(ScoutGrandSlamEvent {
            scout_id,
            target_id
        })
    }
}

pub struct ScoutSlamdollLandedEvent {
    pub target_index: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl FromRawGameEvent for ScoutSlamdollLandedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let target_index: u16 = {
            let value = values.get("target_index").ok_or(ParseError::UnknownGameEvent("target_index".to_string()))?;
            u16::from_value(value.clone(), "target_index")?
        };
        let x: f32 = {
            let value = values.get("x").ok_or(ParseError::UnknownGameEvent("x".to_string()))?;
            f32::from_value(value.clone(), "x")?
        };
        let y: f32 = {
            let value = values.get("y").ok_or(ParseError::UnknownGameEvent("y".to_string()))?;
            f32::from_value(value.clone(), "y")?
        };
        let z: f32 = {
            let value = values.get("z").ok_or(ParseError::UnknownGameEvent("z".to_string()))?;
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

pub struct ArrowImpactEvent {
    pub attachedEntity: u16,
    pub shooter: u16,
    pub boneIndexAttached: u16,
    pub bonePositionX: f32,
    pub bonePositionY: f32,
    pub bonePositionZ: f32,
    pub boneAnglesX: f32,
    pub boneAnglesY: f32,
    pub boneAnglesZ: f32,
    pub projectileType: u16,
    pub isCrit: bool,
}
impl FromRawGameEvent for ArrowImpactEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let attachedEntity: u16 = {
            let value = values.get("attachedEntity").ok_or(ParseError::UnknownGameEvent("attachedEntity".to_string()))?;
            u16::from_value(value.clone(), "attachedEntity")?
        };
        let shooter: u16 = {
            let value = values.get("shooter").ok_or(ParseError::UnknownGameEvent("shooter".to_string()))?;
            u16::from_value(value.clone(), "shooter")?
        };
        let boneIndexAttached: u16 = {
            let value = values.get("boneIndexAttached").ok_or(ParseError::UnknownGameEvent("boneIndexAttached".to_string()))?;
            u16::from_value(value.clone(), "boneIndexAttached")?
        };
        let bonePositionX: f32 = {
            let value = values.get("bonePositionX").ok_or(ParseError::UnknownGameEvent("bonePositionX".to_string()))?;
            f32::from_value(value.clone(), "bonePositionX")?
        };
        let bonePositionY: f32 = {
            let value = values.get("bonePositionY").ok_or(ParseError::UnknownGameEvent("bonePositionY".to_string()))?;
            f32::from_value(value.clone(), "bonePositionY")?
        };
        let bonePositionZ: f32 = {
            let value = values.get("bonePositionZ").ok_or(ParseError::UnknownGameEvent("bonePositionZ".to_string()))?;
            f32::from_value(value.clone(), "bonePositionZ")?
        };
        let boneAnglesX: f32 = {
            let value = values.get("boneAnglesX").ok_or(ParseError::UnknownGameEvent("boneAnglesX".to_string()))?;
            f32::from_value(value.clone(), "boneAnglesX")?
        };
        let boneAnglesY: f32 = {
            let value = values.get("boneAnglesY").ok_or(ParseError::UnknownGameEvent("boneAnglesY".to_string()))?;
            f32::from_value(value.clone(), "boneAnglesY")?
        };
        let boneAnglesZ: f32 = {
            let value = values.get("boneAnglesZ").ok_or(ParseError::UnknownGameEvent("boneAnglesZ".to_string()))?;
            f32::from_value(value.clone(), "boneAnglesZ")?
        };
        let projectileType: u16 = {
            let value = values.get("projectileType").ok_or(ParseError::UnknownGameEvent("projectileType".to_string()))?;
            u16::from_value(value.clone(), "projectileType")?
        };
        let isCrit: bool = {
            let value = values.get("isCrit").ok_or(ParseError::UnknownGameEvent("isCrit".to_string()))?;
            bool::from_value(value.clone(), "isCrit")?
        };
        Ok(ArrowImpactEvent {
            attachedEntity,
            shooter,
            boneIndexAttached,
            bonePositionX,
            bonePositionY,
            bonePositionZ,
            boneAnglesX,
            boneAnglesY,
            boneAnglesZ,
            projectileType,
            isCrit
        })
    }
}

pub struct PlayerJaratedEvent {
    pub thrower_entindex: u8,
    pub victim_entindex: u8,
}
impl FromRawGameEvent for PlayerJaratedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let thrower_entindex: u8 = {
            let value = values.get("thrower_entindex").ok_or(ParseError::UnknownGameEvent("thrower_entindex".to_string()))?;
            u8::from_value(value.clone(), "thrower_entindex")?
        };
        let victim_entindex: u8 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u8::from_value(value.clone(), "victim_entindex")?
        };
        Ok(PlayerJaratedEvent {
            thrower_entindex,
            victim_entindex
        })
    }
}

pub struct PlayerJaratedFadeEvent {
    pub thrower_entindex: u8,
    pub victim_entindex: u8,
}
impl FromRawGameEvent for PlayerJaratedFadeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let thrower_entindex: u8 = {
            let value = values.get("thrower_entindex").ok_or(ParseError::UnknownGameEvent("thrower_entindex".to_string()))?;
            u8::from_value(value.clone(), "thrower_entindex")?
        };
        let victim_entindex: u8 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u8::from_value(value.clone(), "victim_entindex")?
        };
        Ok(PlayerJaratedFadeEvent {
            thrower_entindex,
            victim_entindex
        })
    }
}

pub struct PlayerShieldBlockedEvent {
    pub attacker_entindex: u8,
    pub blocker_entindex: u8,
}
impl FromRawGameEvent for PlayerShieldBlockedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let attacker_entindex: u8 = {
            let value = values.get("attacker_entindex").ok_or(ParseError::UnknownGameEvent("attacker_entindex".to_string()))?;
            u8::from_value(value.clone(), "attacker_entindex")?
        };
        let blocker_entindex: u8 = {
            let value = values.get("blocker_entindex").ok_or(ParseError::UnknownGameEvent("blocker_entindex".to_string()))?;
            u8::from_value(value.clone(), "blocker_entindex")?
        };
        Ok(PlayerShieldBlockedEvent {
            attacker_entindex,
            blocker_entindex
        })
    }
}

pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl FromRawGameEvent for PlayerPinnedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let pinned: u8 = {
            let value = values.get("pinned").ok_or(ParseError::UnknownGameEvent("pinned".to_string()))?;
            u8::from_value(value.clone(), "pinned")?
        };
        Ok(PlayerPinnedEvent {
            pinned
        })
    }
}

pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl FromRawGameEvent for PlayerHealedByMedicEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let medic: u8 = {
            let value = values.get("medic").ok_or(ParseError::UnknownGameEvent("medic".to_string()))?;
            u8::from_value(value.clone(), "medic")?
        };
        Ok(PlayerHealedByMedicEvent {
            medic
        })
    }
}

pub struct PlayerSappedObjectEvent {
    pub userid: u16,
    pub ownerid: u16,
    pub object: u8,
    pub sapperid: u16,
}
impl FromRawGameEvent for PlayerSappedObjectEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let ownerid: u16 = {
            let value = values.get("ownerid").ok_or(ParseError::UnknownGameEvent("ownerid".to_string()))?;
            u16::from_value(value.clone(), "ownerid")?
        };
        let object: u8 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u8::from_value(value.clone(), "object")?
        };
        let sapperid: u16 = {
            let value = values.get("sapperid").ok_or(ParseError::UnknownGameEvent("sapperid".to_string()))?;
            u16::from_value(value.clone(), "sapperid")?
        };
        Ok(PlayerSappedObjectEvent {
            userid,
            ownerid,
            object,
            sapperid
        })
    }
}

pub struct ItemFoundEvent {
    pub player: u8,
    pub quality: u8,
    pub method: u8,
    pub itemdef: u32,
}
impl FromRawGameEvent for ItemFoundEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u8 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u8::from_value(value.clone(), "player")?
        };
        let quality: u8 = {
            let value = values.get("quality").ok_or(ParseError::UnknownGameEvent("quality".to_string()))?;
            u8::from_value(value.clone(), "quality")?
        };
        let method: u8 = {
            let value = values.get("method").ok_or(ParseError::UnknownGameEvent("method".to_string()))?;
            u8::from_value(value.clone(), "method")?
        };
        let itemdef: u32 = {
            let value = values.get("itemdef").ok_or(ParseError::UnknownGameEvent("itemdef".to_string()))?;
            u32::from_value(value.clone(), "itemdef")?
        };
        Ok(ItemFoundEvent {
            player,
            quality,
            method,
            itemdef
        })
    }
}

pub struct ShowAnnotationEvent {
    pub worldPosX: f32,
    pub worldPosY: f32,
    pub worldPosZ: f32,
    pub worldNormalX: f32,
    pub worldNormalY: f32,
    pub worldNormalZ: f32,
    pub id: u32,
    pub text: String,
    pub lifetime: f32,
    pub visibilityBitfield: u32,
    pub follow_entindex: u32,
    pub show_distance: bool,
    pub play_sound: String,
    pub show_effect: bool,
}
impl FromRawGameEvent for ShowAnnotationEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let worldPosX: f32 = {
            let value = values.get("worldPosX").ok_or(ParseError::UnknownGameEvent("worldPosX".to_string()))?;
            f32::from_value(value.clone(), "worldPosX")?
        };
        let worldPosY: f32 = {
            let value = values.get("worldPosY").ok_or(ParseError::UnknownGameEvent("worldPosY".to_string()))?;
            f32::from_value(value.clone(), "worldPosY")?
        };
        let worldPosZ: f32 = {
            let value = values.get("worldPosZ").ok_or(ParseError::UnknownGameEvent("worldPosZ".to_string()))?;
            f32::from_value(value.clone(), "worldPosZ")?
        };
        let worldNormalX: f32 = {
            let value = values.get("worldNormalX").ok_or(ParseError::UnknownGameEvent("worldNormalX".to_string()))?;
            f32::from_value(value.clone(), "worldNormalX")?
        };
        let worldNormalY: f32 = {
            let value = values.get("worldNormalY").ok_or(ParseError::UnknownGameEvent("worldNormalY".to_string()))?;
            f32::from_value(value.clone(), "worldNormalY")?
        };
        let worldNormalZ: f32 = {
            let value = values.get("worldNormalZ").ok_or(ParseError::UnknownGameEvent("worldNormalZ".to_string()))?;
            f32::from_value(value.clone(), "worldNormalZ")?
        };
        let id: u32 = {
            let value = values.get("id").ok_or(ParseError::UnknownGameEvent("id".to_string()))?;
            u32::from_value(value.clone(), "id")?
        };
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        let lifetime: f32 = {
            let value = values.get("lifetime").ok_or(ParseError::UnknownGameEvent("lifetime".to_string()))?;
            f32::from_value(value.clone(), "lifetime")?
        };
        let visibilityBitfield: u32 = {
            let value = values.get("visibilityBitfield").ok_or(ParseError::UnknownGameEvent("visibilityBitfield".to_string()))?;
            u32::from_value(value.clone(), "visibilityBitfield")?
        };
        let follow_entindex: u32 = {
            let value = values.get("follow_entindex").ok_or(ParseError::UnknownGameEvent("follow_entindex".to_string()))?;
            u32::from_value(value.clone(), "follow_entindex")?
        };
        let show_distance: bool = {
            let value = values.get("show_distance").ok_or(ParseError::UnknownGameEvent("show_distance".to_string()))?;
            bool::from_value(value.clone(), "show_distance")?
        };
        let play_sound: String = {
            let value = values.get("play_sound").ok_or(ParseError::UnknownGameEvent("play_sound".to_string()))?;
            String::from_value(value.clone(), "play_sound")?
        };
        let show_effect: bool = {
            let value = values.get("show_effect").ok_or(ParseError::UnknownGameEvent("show_effect".to_string()))?;
            bool::from_value(value.clone(), "show_effect")?
        };
        Ok(ShowAnnotationEvent {
            worldPosX,
            worldPosY,
            worldPosZ,
            worldNormalX,
            worldNormalY,
            worldNormalZ,
            id,
            text,
            lifetime,
            visibilityBitfield,
            follow_entindex,
            show_distance,
            play_sound,
            show_effect
        })
    }
}

pub struct HideAnnotationEvent {
    pub id: u32,
}
impl FromRawGameEvent for HideAnnotationEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let id: u32 = {
            let value = values.get("id").ok_or(ParseError::UnknownGameEvent("id".to_string()))?;
            u32::from_value(value.clone(), "id")?
        };
        Ok(HideAnnotationEvent {
            id
        })
    }
}

pub struct PostInventoryApplicationEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PostInventoryApplicationEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PostInventoryApplicationEvent {
            userid
        })
    }
}

pub struct ControlPointUnlockUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl FromRawGameEvent for ControlPointUnlockUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let time: f32 = {
            let value = values.get("time").ok_or(ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(ControlPointUnlockUpdatedEvent {
            index,
            time
        })
    }
}

pub struct DeployBuffBannerEvent {
    pub buff_type: u8,
    pub buff_owner: u16,
}
impl FromRawGameEvent for DeployBuffBannerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let buff_type: u8 = {
            let value = values.get("buff_type").ok_or(ParseError::UnknownGameEvent("buff_type".to_string()))?;
            u8::from_value(value.clone(), "buff_type")?
        };
        let buff_owner: u16 = {
            let value = values.get("buff_owner").ok_or(ParseError::UnknownGameEvent("buff_owner".to_string()))?;
            u16::from_value(value.clone(), "buff_owner")?
        };
        Ok(DeployBuffBannerEvent {
            buff_type,
            buff_owner
        })
    }
}

pub struct PlayerBuffEvent {
    pub userid: u16,
    pub buff_owner: u16,
    pub buff_type: u8,
}
impl FromRawGameEvent for PlayerBuffEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let buff_owner: u16 = {
            let value = values.get("buff_owner").ok_or(ParseError::UnknownGameEvent("buff_owner".to_string()))?;
            u16::from_value(value.clone(), "buff_owner")?
        };
        let buff_type: u8 = {
            let value = values.get("buff_type").ok_or(ParseError::UnknownGameEvent("buff_type".to_string()))?;
            u8::from_value(value.clone(), "buff_type")?
        };
        Ok(PlayerBuffEvent {
            userid,
            buff_owner,
            buff_type
        })
    }
}

pub struct MedicDeathEvent {
    pub userid: u16,
    pub attacker: u16,
    pub healing: u16,
    pub charged: bool,
}
impl FromRawGameEvent for MedicDeathEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let healing: u16 = {
            let value = values.get("healing").ok_or(ParseError::UnknownGameEvent("healing".to_string()))?;
            u16::from_value(value.clone(), "healing")?
        };
        let charged: bool = {
            let value = values.get("charged").ok_or(ParseError::UnknownGameEvent("charged".to_string()))?;
            bool::from_value(value.clone(), "charged")?
        };
        Ok(MedicDeathEvent {
            userid,
            attacker,
            healing,
            charged
        })
    }
}

pub struct OvertimeNagEvent {

}
impl FromRawGameEvent for OvertimeNagEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(OvertimeNagEvent {

        })
    }
}

pub struct TeamsChangedEvent {

}
impl FromRawGameEvent for TeamsChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(TeamsChangedEvent {

        })
    }
}

pub struct HalloweenPumpkinGrabEvent {
    pub userid: u16,
}
impl FromRawGameEvent for HalloweenPumpkinGrabEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(HalloweenPumpkinGrabEvent {
            userid
        })
    }
}

pub struct RocketJumpEvent {
    pub userid: u16,
    pub playsound: bool,
}
impl FromRawGameEvent for RocketJumpEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let playsound: bool = {
            let value = values.get("playsound").ok_or(ParseError::UnknownGameEvent("playsound".to_string()))?;
            bool::from_value(value.clone(), "playsound")?
        };
        Ok(RocketJumpEvent {
            userid,
            playsound
        })
    }
}

pub struct RocketJumpLandedEvent {
    pub userid: u16,
}
impl FromRawGameEvent for RocketJumpLandedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(RocketJumpLandedEvent {
            userid
        })
    }
}

pub struct StickyJumpEvent {
    pub userid: u16,
    pub playsound: bool,
}
impl FromRawGameEvent for StickyJumpEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let playsound: bool = {
            let value = values.get("playsound").ok_or(ParseError::UnknownGameEvent("playsound".to_string()))?;
            bool::from_value(value.clone(), "playsound")?
        };
        Ok(StickyJumpEvent {
            userid,
            playsound
        })
    }
}

pub struct StickyJumpLandedEvent {
    pub userid: u16,
}
impl FromRawGameEvent for StickyJumpLandedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(StickyJumpLandedEvent {
            userid
        })
    }
}

pub struct MedicDefendedEvent {
    pub userid: u16,
    pub medic: u16,
}
impl FromRawGameEvent for MedicDefendedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let medic: u16 = {
            let value = values.get("medic").ok_or(ParseError::UnknownGameEvent("medic".to_string()))?;
            u16::from_value(value.clone(), "medic")?
        };
        Ok(MedicDefendedEvent {
            userid,
            medic
        })
    }
}

pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl FromRawGameEvent for LocalPlayerHealedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let amount: u16 = {
            let value = values.get("amount").ok_or(ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(LocalPlayerHealedEvent {
            amount
        })
    }
}

pub struct PlayerDestroyedPipeBombEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PlayerDestroyedPipeBombEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PlayerDestroyedPipeBombEvent {
            userid
        })
    }
}

pub struct ObjectDeflectedEvent {
    pub userid: u16,
    pub ownerid: u16,
    pub weaponid: u16,
    pub object_entindex: u16,
}
impl FromRawGameEvent for ObjectDeflectedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let ownerid: u16 = {
            let value = values.get("ownerid").ok_or(ParseError::UnknownGameEvent("ownerid".to_string()))?;
            u16::from_value(value.clone(), "ownerid")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let object_entindex: u16 = {
            let value = values.get("object_entindex").ok_or(ParseError::UnknownGameEvent("object_entindex".to_string()))?;
            u16::from_value(value.clone(), "object_entindex")?
        };
        Ok(ObjectDeflectedEvent {
            userid,
            ownerid,
            weaponid,
            object_entindex
        })
    }
}

pub struct PlayerMvpEvent {
    pub player: u16,
}
impl FromRawGameEvent for PlayerMvpEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(PlayerMvpEvent {
            player
        })
    }
}

pub struct RaidSpawnMobEvent {

}
impl FromRawGameEvent for RaidSpawnMobEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(RaidSpawnMobEvent {

        })
    }
}

pub struct RaidSpawnSquadEvent {

}
impl FromRawGameEvent for RaidSpawnSquadEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(RaidSpawnSquadEvent {

        })
    }
}

pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl FromRawGameEvent for NavBlockedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let area: u32 = {
            let value = values.get("area").ok_or(ParseError::UnknownGameEvent("area".to_string()))?;
            u32::from_value(value.clone(), "area")?
        };
        let blocked: bool = {
            let value = values.get("blocked").ok_or(ParseError::UnknownGameEvent("blocked".to_string()))?;
            bool::from_value(value.clone(), "blocked")?
        };
        Ok(NavBlockedEvent {
            area,
            blocked
        })
    }
}

pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl FromRawGameEvent for PathTrackPassedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PathTrackPassedEvent {
            index
        })
    }
}

pub struct NumCappersChangedEvent {
    pub index: u16,
    pub count: u8,
}
impl FromRawGameEvent for NumCappersChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let count: u8 = {
            let value = values.get("count").ok_or(ParseError::UnknownGameEvent("count".to_string()))?;
            u8::from_value(value.clone(), "count")?
        };
        Ok(NumCappersChangedEvent {
            index,
            count
        })
    }
}

pub struct PlayerRegenerateEvent {

}
impl FromRawGameEvent for PlayerRegenerateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PlayerRegenerateEvent {

        })
    }
}

pub struct UpdateStatusItemEvent {
    pub index: u8,
    pub object: u8,
}
impl FromRawGameEvent for UpdateStatusItemEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u8 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let object: u8 = {
            let value = values.get("object").ok_or(ParseError::UnknownGameEvent("object".to_string()))?;
            u8::from_value(value.clone(), "object")?
        };
        Ok(UpdateStatusItemEvent {
            index,
            object
        })
    }
}

pub struct StatsResetRoundEvent {

}
impl FromRawGameEvent for StatsResetRoundEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(StatsResetRoundEvent {

        })
    }
}

pub struct ScoreStatsAccumulatedUpdateEvent {

}
impl FromRawGameEvent for ScoreStatsAccumulatedUpdateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ScoreStatsAccumulatedUpdateEvent {

        })
    }
}

pub struct ScoreStatsAccumulatedResetEvent {

}
impl FromRawGameEvent for ScoreStatsAccumulatedResetEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ScoreStatsAccumulatedResetEvent {

        })
    }
}

pub struct AchievementEarnedLocalEvent {
    pub achievement: u16,
}
impl FromRawGameEvent for AchievementEarnedLocalEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let achievement: u16 = {
            let value = values.get("achievement").ok_or(ParseError::UnknownGameEvent("achievement".to_string()))?;
            u16::from_value(value.clone(), "achievement")?
        };
        Ok(AchievementEarnedLocalEvent {
            achievement
        })
    }
}

pub struct PlayerHealedEvent {
    pub patient: u16,
    pub healer: u16,
    pub amount: u16,
}
impl FromRawGameEvent for PlayerHealedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let patient: u16 = {
            let value = values.get("patient").ok_or(ParseError::UnknownGameEvent("patient".to_string()))?;
            u16::from_value(value.clone(), "patient")?
        };
        let healer: u16 = {
            let value = values.get("healer").ok_or(ParseError::UnknownGameEvent("healer".to_string()))?;
            u16::from_value(value.clone(), "healer")?
        };
        let amount: u16 = {
            let value = values.get("amount").ok_or(ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(PlayerHealedEvent {
            patient,
            healer,
            amount
        })
    }
}

pub struct BuildingHealedEvent {
    pub building: u16,
    pub healer: u16,
    pub amount: u16,
}
impl FromRawGameEvent for BuildingHealedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let building: u16 = {
            let value = values.get("building").ok_or(ParseError::UnknownGameEvent("building".to_string()))?;
            u16::from_value(value.clone(), "building")?
        };
        let healer: u16 = {
            let value = values.get("healer").ok_or(ParseError::UnknownGameEvent("healer".to_string()))?;
            u16::from_value(value.clone(), "healer")?
        };
        let amount: u16 = {
            let value = values.get("amount").ok_or(ParseError::UnknownGameEvent("amount".to_string()))?;
            u16::from_value(value.clone(), "amount")?
        };
        Ok(BuildingHealedEvent {
            building,
            healer,
            amount
        })
    }
}

pub struct ItemPickupEvent {
    pub userid: u16,
    pub item: String,
}
impl FromRawGameEvent for ItemPickupEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let item: String = {
            let value = values.get("item").ok_or(ParseError::UnknownGameEvent("item".to_string()))?;
            String::from_value(value.clone(), "item")?
        };
        Ok(ItemPickupEvent {
            userid,
            item
        })
    }
}

pub struct DuelStatusEvent {
    pub killer: u16,
    pub score_type: u16,
    pub initiator: u16,
    pub target: u16,
    pub initiator_score: u16,
    pub target_score: u16,
}
impl FromRawGameEvent for DuelStatusEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let killer: u16 = {
            let value = values.get("killer").ok_or(ParseError::UnknownGameEvent("killer".to_string()))?;
            u16::from_value(value.clone(), "killer")?
        };
        let score_type: u16 = {
            let value = values.get("score_type").ok_or(ParseError::UnknownGameEvent("score_type".to_string()))?;
            u16::from_value(value.clone(), "score_type")?
        };
        let initiator: u16 = {
            let value = values.get("initiator").ok_or(ParseError::UnknownGameEvent("initiator".to_string()))?;
            u16::from_value(value.clone(), "initiator")?
        };
        let target: u16 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        let initiator_score: u16 = {
            let value = values.get("initiator_score").ok_or(ParseError::UnknownGameEvent("initiator_score".to_string()))?;
            u16::from_value(value.clone(), "initiator_score")?
        };
        let target_score: u16 = {
            let value = values.get("target_score").ok_or(ParseError::UnknownGameEvent("target_score".to_string()))?;
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

pub struct FishNoticeEvent {
    pub userid: u16,
    pub victim_entindex: u32,
    pub inflictor_entindex: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub damagebits: u32,
    pub customkill: u16,
    pub assister: u16,
    pub weapon_logclassname: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
}
impl FromRawGameEvent for FishNoticeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let victim_entindex: u32 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u32::from_value(value.clone(), "victim_entindex")?
        };
        let inflictor_entindex: u32 = {
            let value = values.get("inflictor_entindex").ok_or(ParseError::UnknownGameEvent("inflictor_entindex".to_string()))?;
            u32::from_value(value.clone(), "inflictor_entindex")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        let customkill: u16 = {
            let value = values.get("customkill").ok_or(ParseError::UnknownGameEvent("customkill".to_string()))?;
            u16::from_value(value.clone(), "customkill")?
        };
        let assister: u16 = {
            let value = values.get("assister").ok_or(ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_logclassname: String = {
            let value = values.get("weapon_logclassname").ok_or(ParseError::UnknownGameEvent("weapon_logclassname".to_string()))?;
            String::from_value(value.clone(), "weapon_logclassname")?
        };
        let stun_flags: u16 = {
            let value = values.get("stun_flags").ok_or(ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get("death_flags").ok_or(ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get("silent_kill").ok_or(ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get("assister_fallback").ok_or(ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        Ok(FishNoticeEvent {
            userid,
            victim_entindex,
            inflictor_entindex,
            attacker,
            weapon,
            weaponid,
            damagebits,
            customkill,
            assister,
            weapon_logclassname,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback
        })
    }
}

pub struct FishNoticeArmEvent {
    pub userid: u16,
    pub victim_entindex: u32,
    pub inflictor_entindex: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub damagebits: u32,
    pub customkill: u16,
    pub assister: u16,
    pub weapon_logclassname: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
}
impl FromRawGameEvent for FishNoticeArmEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let victim_entindex: u32 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u32::from_value(value.clone(), "victim_entindex")?
        };
        let inflictor_entindex: u32 = {
            let value = values.get("inflictor_entindex").ok_or(ParseError::UnknownGameEvent("inflictor_entindex".to_string()))?;
            u32::from_value(value.clone(), "inflictor_entindex")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        let customkill: u16 = {
            let value = values.get("customkill").ok_or(ParseError::UnknownGameEvent("customkill".to_string()))?;
            u16::from_value(value.clone(), "customkill")?
        };
        let assister: u16 = {
            let value = values.get("assister").ok_or(ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_logclassname: String = {
            let value = values.get("weapon_logclassname").ok_or(ParseError::UnknownGameEvent("weapon_logclassname".to_string()))?;
            String::from_value(value.clone(), "weapon_logclassname")?
        };
        let stun_flags: u16 = {
            let value = values.get("stun_flags").ok_or(ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get("death_flags").ok_or(ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get("silent_kill").ok_or(ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get("assister_fallback").ok_or(ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        Ok(FishNoticeArmEvent {
            userid,
            victim_entindex,
            inflictor_entindex,
            attacker,
            weapon,
            weaponid,
            damagebits,
            customkill,
            assister,
            weapon_logclassname,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback
        })
    }
}

pub struct ThrowableHitEvent {
    pub userid: u16,
    pub victim_entindex: u32,
    pub inflictor_entindex: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub damagebits: u32,
    pub customkill: u16,
    pub assister: u16,
    pub weapon_logclassname: String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: String,
    pub totalhits: u16,
}
impl FromRawGameEvent for ThrowableHitEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let victim_entindex: u32 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u32::from_value(value.clone(), "victim_entindex")?
        };
        let inflictor_entindex: u32 = {
            let value = values.get("inflictor_entindex").ok_or(ParseError::UnknownGameEvent("inflictor_entindex".to_string()))?;
            u32::from_value(value.clone(), "inflictor_entindex")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        let customkill: u16 = {
            let value = values.get("customkill").ok_or(ParseError::UnknownGameEvent("customkill".to_string()))?;
            u16::from_value(value.clone(), "customkill")?
        };
        let assister: u16 = {
            let value = values.get("assister").ok_or(ParseError::UnknownGameEvent("assister".to_string()))?;
            u16::from_value(value.clone(), "assister")?
        };
        let weapon_logclassname: String = {
            let value = values.get("weapon_logclassname").ok_or(ParseError::UnknownGameEvent("weapon_logclassname".to_string()))?;
            String::from_value(value.clone(), "weapon_logclassname")?
        };
        let stun_flags: u16 = {
            let value = values.get("stun_flags").ok_or(ParseError::UnknownGameEvent("stun_flags".to_string()))?;
            u16::from_value(value.clone(), "stun_flags")?
        };
        let death_flags: u16 = {
            let value = values.get("death_flags").ok_or(ParseError::UnknownGameEvent("death_flags".to_string()))?;
            u16::from_value(value.clone(), "death_flags")?
        };
        let silent_kill: bool = {
            let value = values.get("silent_kill").ok_or(ParseError::UnknownGameEvent("silent_kill".to_string()))?;
            bool::from_value(value.clone(), "silent_kill")?
        };
        let assister_fallback: String = {
            let value = values.get("assister_fallback").ok_or(ParseError::UnknownGameEvent("assister_fallback".to_string()))?;
            String::from_value(value.clone(), "assister_fallback")?
        };
        let totalhits: u16 = {
            let value = values.get("totalhits").ok_or(ParseError::UnknownGameEvent("totalhits".to_string()))?;
            u16::from_value(value.clone(), "totalhits")?
        };
        Ok(ThrowableHitEvent {
            userid,
            victim_entindex,
            inflictor_entindex,
            attacker,
            weapon,
            weaponid,
            damagebits,
            customkill,
            assister,
            weapon_logclassname,
            stun_flags,
            death_flags,
            silent_kill,
            assister_fallback,
            totalhits
        })
    }
}

pub struct PumpkinLordSummonedEvent {

}
impl FromRawGameEvent for PumpkinLordSummonedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PumpkinLordSummonedEvent {

        })
    }
}

pub struct PumpkinLordKilledEvent {

}
impl FromRawGameEvent for PumpkinLordKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PumpkinLordKilledEvent {

        })
    }
}

pub struct MerasmusSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusSummonedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusSummonedEvent {
            level
        })
    }
}

pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusKilledEvent {
            level
        })
    }
}

pub struct MerasmusEscapeWarningEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl FromRawGameEvent for MerasmusEscapeWarningEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let time_remaining: u8 = {
            let value = values.get("time_remaining").ok_or(ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u8::from_value(value.clone(), "time_remaining")?
        };
        Ok(MerasmusEscapeWarningEvent {
            level,
            time_remaining
        })
    }
}

pub struct MerasmusEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for MerasmusEscapedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MerasmusEscapedEvent {
            level
        })
    }
}

pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossSummonedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossSummonedEvent {
            level
        })
    }
}

pub struct EyeballBossStunnedEvent {
    pub level: u16,
    pub player_entindex: u8,
}
impl FromRawGameEvent for EyeballBossStunnedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let player_entindex: u8 = {
            let value = values.get("player_entindex").ok_or(ParseError::UnknownGameEvent("player_entindex".to_string()))?;
            u8::from_value(value.clone(), "player_entindex")?
        };
        Ok(EyeballBossStunnedEvent {
            level,
            player_entindex
        })
    }
}

pub struct EyeballBossKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossKilledEvent {
            level
        })
    }
}

pub struct EyeballBossKillerEvent {
    pub level: u16,
    pub player_entindex: u8,
}
impl FromRawGameEvent for EyeballBossKillerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let player_entindex: u8 = {
            let value = values.get("player_entindex").ok_or(ParseError::UnknownGameEvent("player_entindex".to_string()))?;
            u8::from_value(value.clone(), "player_entindex")?
        };
        Ok(EyeballBossKillerEvent {
            level,
            player_entindex
        })
    }
}

pub struct EyeballBossEscapeImminentEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl FromRawGameEvent for EyeballBossEscapeImminentEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        let time_remaining: u8 = {
            let value = values.get("time_remaining").ok_or(ParseError::UnknownGameEvent("time_remaining".to_string()))?;
            u8::from_value(value.clone(), "time_remaining")?
        };
        Ok(EyeballBossEscapeImminentEvent {
            level,
            time_remaining
        })
    }
}

pub struct EyeballBossEscapedEvent {
    pub level: u16,
}
impl FromRawGameEvent for EyeballBossEscapedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(EyeballBossEscapedEvent {
            level
        })
    }
}

pub struct NpcHurtEvent {
    pub entindex: u16,
    pub health: u16,
    pub attacker_player: u16,
    pub weaponid: u16,
    pub damageamount: u16,
    pub crit: bool,
}
impl FromRawGameEvent for NpcHurtEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        let health: u16 = {
            let value = values.get("health").ok_or(ParseError::UnknownGameEvent("health".to_string()))?;
            u16::from_value(value.clone(), "health")?
        };
        let attacker_player: u16 = {
            let value = values.get("attacker_player").ok_or(ParseError::UnknownGameEvent("attacker_player".to_string()))?;
            u16::from_value(value.clone(), "attacker_player")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damageamount: u16 = {
            let value = values.get("damageamount").ok_or(ParseError::UnknownGameEvent("damageamount".to_string()))?;
            u16::from_value(value.clone(), "damageamount")?
        };
        let crit: bool = {
            let value = values.get("crit").ok_or(ParseError::UnknownGameEvent("crit".to_string()))?;
            bool::from_value(value.clone(), "crit")?
        };
        Ok(NpcHurtEvent {
            entindex,
            health,
            attacker_player,
            weaponid,
            damageamount,
            crit
        })
    }
}

pub struct ControlPointTimerUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl FromRawGameEvent for ControlPointTimerUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let time: f32 = {
            let value = values.get("time").ok_or(ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(ControlPointTimerUpdatedEvent {
            index,
            time
        })
    }
}

pub struct PlayerHighfiveStartEvent {
    pub entindex: u8,
}
impl FromRawGameEvent for PlayerHighfiveStartEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u8 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u8::from_value(value.clone(), "entindex")?
        };
        Ok(PlayerHighfiveStartEvent {
            entindex
        })
    }
}

pub struct PlayerHighfiveCancelEvent {
    pub entindex: u8,
}
impl FromRawGameEvent for PlayerHighfiveCancelEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u8 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u8::from_value(value.clone(), "entindex")?
        };
        Ok(PlayerHighfiveCancelEvent {
            entindex
        })
    }
}

pub struct PlayerHighfiveSuccessEvent {
    pub initiator_entindex: u8,
    pub partner_entindex: u8,
}
impl FromRawGameEvent for PlayerHighfiveSuccessEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let initiator_entindex: u8 = {
            let value = values.get("initiator_entindex").ok_or(ParseError::UnknownGameEvent("initiator_entindex".to_string()))?;
            u8::from_value(value.clone(), "initiator_entindex")?
        };
        let partner_entindex: u8 = {
            let value = values.get("partner_entindex").ok_or(ParseError::UnknownGameEvent("partner_entindex".to_string()))?;
            u8::from_value(value.clone(), "partner_entindex")?
        };
        Ok(PlayerHighfiveSuccessEvent {
            initiator_entindex,
            partner_entindex
        })
    }
}

pub struct PlayerBonusPointsEvent {
    pub points: u16,
    pub player_entindex: u16,
    pub source_entindex: u16,
}
impl FromRawGameEvent for PlayerBonusPointsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let points: u16 = {
            let value = values.get("points").ok_or(ParseError::UnknownGameEvent("points".to_string()))?;
            u16::from_value(value.clone(), "points")?
        };
        let player_entindex: u16 = {
            let value = values.get("player_entindex").ok_or(ParseError::UnknownGameEvent("player_entindex".to_string()))?;
            u16::from_value(value.clone(), "player_entindex")?
        };
        let source_entindex: u16 = {
            let value = values.get("source_entindex").ok_or(ParseError::UnknownGameEvent("source_entindex".to_string()))?;
            u16::from_value(value.clone(), "source_entindex")?
        };
        Ok(PlayerBonusPointsEvent {
            points,
            player_entindex,
            source_entindex
        })
    }
}

pub struct PlayerUpgradedEvent {

}
impl FromRawGameEvent for PlayerUpgradedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PlayerUpgradedEvent {

        })
    }
}

pub struct PlayerBuybackEvent {
    pub player: u16,
    pub cost: u16,
}
impl FromRawGameEvent for PlayerBuybackEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let cost: u16 = {
            let value = values.get("cost").ok_or(ParseError::UnknownGameEvent("cost".to_string()))?;
            u16::from_value(value.clone(), "cost")?
        };
        Ok(PlayerBuybackEvent {
            player,
            cost
        })
    }
}

pub struct PlayerUsedPowerUpBottleEvent {
    pub player: u16,
    pub kind: u16,
    pub time: f32,
}
impl FromRawGameEvent for PlayerUsedPowerUpBottleEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let kind: u16 = {
            let value = values.get("kind").ok_or(ParseError::UnknownGameEvent("kind".to_string()))?;
            u16::from_value(value.clone(), "kind")?
        };
        let time: f32 = {
            let value = values.get("time").ok_or(ParseError::UnknownGameEvent("time".to_string()))?;
            f32::from_value(value.clone(), "time")?
        };
        Ok(PlayerUsedPowerUpBottleEvent {
            player,
            kind,
            time
        })
    }
}

pub struct ChristmasGiftGrabEvent {
    pub userid: u16,
}
impl FromRawGameEvent for ChristmasGiftGrabEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(ChristmasGiftGrabEvent {
            userid
        })
    }
}

pub struct PlayerKilledAchievementZoneEvent {
    pub attacker: u16,
    pub victim: u16,
    pub zone_id: u16,
}
impl FromRawGameEvent for PlayerKilledAchievementZoneEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let victim: u16 = {
            let value = values.get("victim").ok_or(ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        let zone_id: u16 = {
            let value = values.get("zone_id").ok_or(ParseError::UnknownGameEvent("zone_id".to_string()))?;
            u16::from_value(value.clone(), "zone_id")?
        };
        Ok(PlayerKilledAchievementZoneEvent {
            attacker,
            victim,
            zone_id
        })
    }
}

pub struct PartyUpdatedEvent {

}
impl FromRawGameEvent for PartyUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(PartyUpdatedEvent {

        })
    }
}

pub struct LobbyUpdatedEvent {

}
impl FromRawGameEvent for LobbyUpdatedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(LobbyUpdatedEvent {

        })
    }
}

pub struct MvmMissionUpdateEvent {
    pub class: u16,
    pub count: u16,
}
impl FromRawGameEvent for MvmMissionUpdateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let class: u16 = {
            let value = values.get("class").ok_or(ParseError::UnknownGameEvent("class".to_string()))?;
            u16::from_value(value.clone(), "class")?
        };
        let count: u16 = {
            let value = values.get("count").ok_or(ParseError::UnknownGameEvent("count".to_string()))?;
            u16::from_value(value.clone(), "count")?
        };
        Ok(MvmMissionUpdateEvent {
            class,
            count
        })
    }
}

pub struct RecalculateHolidaysEvent {

}
impl FromRawGameEvent for RecalculateHolidaysEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(RecalculateHolidaysEvent {

        })
    }
}

pub struct PlayerCurrencyChangedEvent {
    pub currency: u16,
}
impl FromRawGameEvent for PlayerCurrencyChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let currency: u16 = {
            let value = values.get("currency").ok_or(ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(PlayerCurrencyChangedEvent {
            currency
        })
    }
}

pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl FromRawGameEvent for DoomsdayRocketOpenEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        Ok(DoomsdayRocketOpenEvent {
            team
        })
    }
}

pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl FromRawGameEvent for RemoveNemesisRelationshipsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(RemoveNemesisRelationshipsEvent {
            player
        })
    }
}

pub struct MvmCreditBonusWaveEvent {

}
impl FromRawGameEvent for MvmCreditBonusWaveEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusWaveEvent {

        })
    }
}

pub struct MvmCreditBonusAllEvent {

}
impl FromRawGameEvent for MvmCreditBonusAllEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusAllEvent {

        })
    }
}

pub struct MvmCreditBonusAllAdvancedEvent {

}
impl FromRawGameEvent for MvmCreditBonusAllAdvancedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmCreditBonusAllAdvancedEvent {

        })
    }
}

pub struct MvmQuickSentryUpgradeEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmQuickSentryUpgradeEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmQuickSentryUpgradeEvent {
            player
        })
    }
}

pub struct MvmTankDestroyedByPlayersEvent {

}
impl FromRawGameEvent for MvmTankDestroyedByPlayersEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmTankDestroyedByPlayersEvent {

        })
    }
}

pub struct MvmKillRobotDeliveringBombEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmKillRobotDeliveringBombEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmKillRobotDeliveringBombEvent {
            player
        })
    }
}

pub struct MvmPickupCurrencyEvent {
    pub player: u16,
    pub currency: u16,
}
impl FromRawGameEvent for MvmPickupCurrencyEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let currency: u16 = {
            let value = values.get("currency").ok_or(ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(MvmPickupCurrencyEvent {
            player,
            currency
        })
    }
}

pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl FromRawGameEvent for MvmBombCarrierKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(MvmBombCarrierKilledEvent {
            level
        })
    }
}

pub struct MvmSentryBusterDetonateEvent {
    pub player: u16,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}
impl FromRawGameEvent for MvmSentryBusterDetonateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        let det_x: f32 = {
            let value = values.get("det_x").ok_or(ParseError::UnknownGameEvent("det_x".to_string()))?;
            f32::from_value(value.clone(), "det_x")?
        };
        let det_y: f32 = {
            let value = values.get("det_y").ok_or(ParseError::UnknownGameEvent("det_y".to_string()))?;
            f32::from_value(value.clone(), "det_y")?
        };
        let det_z: f32 = {
            let value = values.get("det_z").ok_or(ParseError::UnknownGameEvent("det_z".to_string()))?;
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

pub struct MvmScoutMarkedForDeathEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmScoutMarkedForDeathEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmScoutMarkedForDeathEvent {
            player
        })
    }
}

pub struct MvmMedicPowerupSharedEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmMedicPowerupSharedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmMedicPowerupSharedEvent {
            player
        })
    }
}

pub struct MvmBeginWaveEvent {
    pub wave_index: u16,
    pub max_waves: u16,
    pub advanced: u16,
}
impl FromRawGameEvent for MvmBeginWaveEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let wave_index: u16 = {
            let value = values.get("wave_index").ok_or(ParseError::UnknownGameEvent("wave_index".to_string()))?;
            u16::from_value(value.clone(), "wave_index")?
        };
        let max_waves: u16 = {
            let value = values.get("max_waves").ok_or(ParseError::UnknownGameEvent("max_waves".to_string()))?;
            u16::from_value(value.clone(), "max_waves")?
        };
        let advanced: u16 = {
            let value = values.get("advanced").ok_or(ParseError::UnknownGameEvent("advanced".to_string()))?;
            u16::from_value(value.clone(), "advanced")?
        };
        Ok(MvmBeginWaveEvent {
            wave_index,
            max_waves,
            advanced
        })
    }
}

pub struct MvmWaveCompleteEvent {
    pub advanced: bool,
}
impl FromRawGameEvent for MvmWaveCompleteEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let advanced: bool = {
            let value = values.get("advanced").ok_or(ParseError::UnknownGameEvent("advanced".to_string()))?;
            bool::from_value(value.clone(), "advanced")?
        };
        Ok(MvmWaveCompleteEvent {
            advanced
        })
    }
}

pub struct MvmMissionCompleteEvent {
    pub mission: String,
}
impl FromRawGameEvent for MvmMissionCompleteEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let mission: String = {
            let value = values.get("mission").ok_or(ParseError::UnknownGameEvent("mission".to_string()))?;
            String::from_value(value.clone(), "mission")?
        };
        Ok(MvmMissionCompleteEvent {
            mission
        })
    }
}

pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombResetByPlayerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmBombResetByPlayerEvent {
            player
        })
    }
}

pub struct MvmBombAlarmTriggeredEvent {

}
impl FromRawGameEvent for MvmBombAlarmTriggeredEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmBombAlarmTriggeredEvent {

        })
    }
}

pub struct MvmBombDeployResetByPlayerEvent {
    pub player: u16,
}
impl FromRawGameEvent for MvmBombDeployResetByPlayerEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let player: u16 = {
            let value = values.get("player").ok_or(ParseError::UnknownGameEvent("player".to_string()))?;
            u16::from_value(value.clone(), "player")?
        };
        Ok(MvmBombDeployResetByPlayerEvent {
            player
        })
    }
}

pub struct MvmWaveFailedEvent {

}
impl FromRawGameEvent for MvmWaveFailedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmWaveFailedEvent {

        })
    }
}

pub struct MvmResetStatsEvent {

}
impl FromRawGameEvent for MvmResetStatsEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmResetStatsEvent {

        })
    }
}

pub struct DamageResistedEvent {
    pub entindex: u8,
}
impl FromRawGameEvent for DamageResistedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u8 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u8::from_value(value.clone(), "entindex")?
        };
        Ok(DamageResistedEvent {
            entindex
        })
    }
}

pub struct RevivePlayerNotifyEvent {
    pub entindex: u16,
    pub marker_entindex: u16,
}
impl FromRawGameEvent for RevivePlayerNotifyEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        let marker_entindex: u16 = {
            let value = values.get("marker_entindex").ok_or(ParseError::UnknownGameEvent("marker_entindex".to_string()))?;
            u16::from_value(value.clone(), "marker_entindex")?
        };
        Ok(RevivePlayerNotifyEvent {
            entindex,
            marker_entindex
        })
    }
}

pub struct RevivePlayerStoppedEvent {
    pub entindex: u16,
}
impl FromRawGameEvent for RevivePlayerStoppedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        Ok(RevivePlayerStoppedEvent {
            entindex
        })
    }
}

pub struct RevivePlayerCompleteEvent {
    pub entindex: u16,
}
impl FromRawGameEvent for RevivePlayerCompleteEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        Ok(RevivePlayerCompleteEvent {
            entindex
        })
    }
}

pub struct PlayerTurnedToGhostEvent {
    pub userid: u16,
}
impl FromRawGameEvent for PlayerTurnedToGhostEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        Ok(PlayerTurnedToGhostEvent {
            userid
        })
    }
}

pub struct MedigunShieldBlockedDamageEvent {
    pub userid: u16,
    pub damage: f32,
}
impl FromRawGameEvent for MedigunShieldBlockedDamageEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let damage: f32 = {
            let value = values.get("damage").ok_or(ParseError::UnknownGameEvent("damage".to_string()))?;
            f32::from_value(value.clone(), "damage")?
        };
        Ok(MedigunShieldBlockedDamageEvent {
            userid,
            damage
        })
    }
}

pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl FromRawGameEvent for MvmAdvWaveCompleteNoGatesEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(MvmAdvWaveCompleteNoGatesEvent {
            index
        })
    }
}

pub struct MvmSniperHeadshotCurrencyEvent {
    pub userid: u16,
    pub currency: u16,
}
impl FromRawGameEvent for MvmSniperHeadshotCurrencyEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let currency: u16 = {
            let value = values.get("currency").ok_or(ParseError::UnknownGameEvent("currency".to_string()))?;
            u16::from_value(value.clone(), "currency")?
        };
        Ok(MvmSniperHeadshotCurrencyEvent {
            userid,
            currency
        })
    }
}

pub struct MvmMannhattanPitEvent {

}
impl FromRawGameEvent for MvmMannhattanPitEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmMannhattanPitEvent {

        })
    }
}

pub struct FlagCarriedInDetectionZoneEvent {

}
impl FromRawGameEvent for FlagCarriedInDetectionZoneEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(FlagCarriedInDetectionZoneEvent {

        })
    }
}

pub struct MvmAdvWaveKilledStunRadioEvent {

}
impl FromRawGameEvent for MvmAdvWaveKilledStunRadioEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(MvmAdvWaveKilledStunRadioEvent {

        })
    }
}

pub struct PlayerDirecthitStunEvent {
    pub attacker: u16,
    pub victim: u16,
}
impl FromRawGameEvent for PlayerDirecthitStunEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let victim: u16 = {
            let value = values.get("victim").ok_or(ParseError::UnknownGameEvent("victim".to_string()))?;
            u16::from_value(value.clone(), "victim")?
        };
        Ok(PlayerDirecthitStunEvent {
            attacker,
            victim
        })
    }
}

pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl FromRawGameEvent for MvmSentryBusterKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let sentry_buster: u16 = {
            let value = values.get("sentry_buster").ok_or(ParseError::UnknownGameEvent("sentry_buster".to_string()))?;
            u16::from_value(value.clone(), "sentry_buster")?
        };
        Ok(MvmSentryBusterKilledEvent {
            sentry_buster
        })
    }
}

pub struct UpgradesFileChangedEvent {
    pub path: String,
}
impl FromRawGameEvent for UpgradesFileChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let path: String = {
            let value = values.get("path").ok_or(ParseError::UnknownGameEvent("path".to_string()))?;
            String::from_value(value.clone(), "path")?
        };
        Ok(UpgradesFileChangedEvent {
            path
        })
    }
}

pub struct RdTeamPointsChangedEvent {
    pub points: u16,
    pub team: u8,
    pub method: u8,
}
impl FromRawGameEvent for RdTeamPointsChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let points: u16 = {
            let value = values.get("points").ok_or(ParseError::UnknownGameEvent("points".to_string()))?;
            u16::from_value(value.clone(), "points")?
        };
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let method: u8 = {
            let value = values.get("method").ok_or(ParseError::UnknownGameEvent("method".to_string()))?;
            u8::from_value(value.clone(), "method")?
        };
        Ok(RdTeamPointsChangedEvent {
            points,
            team,
            method
        })
    }
}

pub struct RdRulesStateChangedEvent {

}
impl FromRawGameEvent for RdRulesStateChangedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(RdRulesStateChangedEvent {

        })
    }
}

pub struct RdRobotKilledEvent {
    pub userid: u16,
    pub victim_entindex: u32,
    pub inflictor_entindex: u32,
    pub attacker: u16,
    pub weapon: String,
    pub weaponid: u16,
    pub damagebits: u32,
    pub customkill: u16,
    pub weapon_logclassname: String,
}
impl FromRawGameEvent for RdRobotKilledEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let userid: u16 = {
            let value = values.get("userid").ok_or(ParseError::UnknownGameEvent("userid".to_string()))?;
            u16::from_value(value.clone(), "userid")?
        };
        let victim_entindex: u32 = {
            let value = values.get("victim_entindex").ok_or(ParseError::UnknownGameEvent("victim_entindex".to_string()))?;
            u32::from_value(value.clone(), "victim_entindex")?
        };
        let inflictor_entindex: u32 = {
            let value = values.get("inflictor_entindex").ok_or(ParseError::UnknownGameEvent("inflictor_entindex".to_string()))?;
            u32::from_value(value.clone(), "inflictor_entindex")?
        };
        let attacker: u16 = {
            let value = values.get("attacker").ok_or(ParseError::UnknownGameEvent("attacker".to_string()))?;
            u16::from_value(value.clone(), "attacker")?
        };
        let weapon: String = {
            let value = values.get("weapon").ok_or(ParseError::UnknownGameEvent("weapon".to_string()))?;
            String::from_value(value.clone(), "weapon")?
        };
        let weaponid: u16 = {
            let value = values.get("weaponid").ok_or(ParseError::UnknownGameEvent("weaponid".to_string()))?;
            u16::from_value(value.clone(), "weaponid")?
        };
        let damagebits: u32 = {
            let value = values.get("damagebits").ok_or(ParseError::UnknownGameEvent("damagebits".to_string()))?;
            u32::from_value(value.clone(), "damagebits")?
        };
        let customkill: u16 = {
            let value = values.get("customkill").ok_or(ParseError::UnknownGameEvent("customkill".to_string()))?;
            u16::from_value(value.clone(), "customkill")?
        };
        let weapon_logclassname: String = {
            let value = values.get("weapon_logclassname").ok_or(ParseError::UnknownGameEvent("weapon_logclassname".to_string()))?;
            String::from_value(value.clone(), "weapon_logclassname")?
        };
        Ok(RdRobotKilledEvent {
            userid,
            victim_entindex,
            inflictor_entindex,
            attacker,
            weapon,
            weaponid,
            damagebits,
            customkill,
            weapon_logclassname
        })
    }
}

pub struct RdRobotImpactEvent {
    pub entindex: u16,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}
impl FromRawGameEvent for RdRobotImpactEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let entindex: u16 = {
            let value = values.get("entindex").ok_or(ParseError::UnknownGameEvent("entindex".to_string()))?;
            u16::from_value(value.clone(), "entindex")?
        };
        let impulse_x: f32 = {
            let value = values.get("impulse_x").ok_or(ParseError::UnknownGameEvent("impulse_x".to_string()))?;
            f32::from_value(value.clone(), "impulse_x")?
        };
        let impulse_y: f32 = {
            let value = values.get("impulse_y").ok_or(ParseError::UnknownGameEvent("impulse_y".to_string()))?;
            f32::from_value(value.clone(), "impulse_y")?
        };
        let impulse_z: f32 = {
            let value = values.get("impulse_z").ok_or(ParseError::UnknownGameEvent("impulse_z".to_string()))?;
            f32::from_value(value.clone(), "impulse_z")?
        };
        Ok(RdRobotImpactEvent {
            entindex,
            impulse_x,
            impulse_y,
            impulse_z
        })
    }
}

pub struct TeamPlayPreRoundTimeLeftEvent {
    pub time: u16,
}
impl FromRawGameEvent for TeamPlayPreRoundTimeLeftEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let time: u16 = {
            let value = values.get("time").ok_or(ParseError::UnknownGameEvent("time".to_string()))?;
            u16::from_value(value.clone(), "time")?
        };
        Ok(TeamPlayPreRoundTimeLeftEvent {
            time
        })
    }
}

pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteDeployEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ParachuteDeployEvent {
            index
        })
    }
}

pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl FromRawGameEvent for ParachuteHolsterEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(ParachuteHolsterEvent {
            index
        })
    }
}

pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl FromRawGameEvent for KillRefillsMeterEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(KillRefillsMeterEvent {
            index
        })
    }
}

pub struct RpsTauntEventEvent {
    pub winner: u16,
    pub winner_rps: u8,
    pub loser: u16,
    pub loser_rps: u8,
}
impl FromRawGameEvent for RpsTauntEventEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let winner: u16 = {
            let value = values.get("winner").ok_or(ParseError::UnknownGameEvent("winner".to_string()))?;
            u16::from_value(value.clone(), "winner")?
        };
        let winner_rps: u8 = {
            let value = values.get("winner_rps").ok_or(ParseError::UnknownGameEvent("winner_rps".to_string()))?;
            u8::from_value(value.clone(), "winner_rps")?
        };
        let loser: u16 = {
            let value = values.get("loser").ok_or(ParseError::UnknownGameEvent("loser".to_string()))?;
            u16::from_value(value.clone(), "loser")?
        };
        let loser_rps: u8 = {
            let value = values.get("loser_rps").ok_or(ParseError::UnknownGameEvent("loser_rps".to_string()))?;
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

pub struct CongaKillEvent {
    pub index: u16,
}
impl FromRawGameEvent for CongaKillEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(CongaKillEvent {
            index
        })
    }
}

pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl FromRawGameEvent for PlayerInitialSpawnEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(PlayerInitialSpawnEvent {
            index
        })
    }
}

pub struct CompetitiveVictoryEvent {

}
impl FromRawGameEvent for CompetitiveVictoryEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(CompetitiveVictoryEvent {

        })
    }
}

pub struct CompetitiveSkillratingUpdateEvent {
    pub index: u16,
    pub rating: u16,
    pub delta: u16,
}
impl FromRawGameEvent for CompetitiveSkillratingUpdateEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let rating: u16 = {
            let value = values.get("rating").ok_or(ParseError::UnknownGameEvent("rating".to_string()))?;
            u16::from_value(value.clone(), "rating")?
        };
        let delta: u16 = {
            let value = values.get("delta").ok_or(ParseError::UnknownGameEvent("delta".to_string()))?;
            u16::from_value(value.clone(), "delta")?
        };
        Ok(CompetitiveSkillratingUpdateEvent {
            index,
            rating,
            delta
        })
    }
}

pub struct MiniGameWinEvent {
    pub team: u8,
    pub kind: u8,
}
impl FromRawGameEvent for MiniGameWinEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let team: u8 = {
            let value = values.get("team").ok_or(ParseError::UnknownGameEvent("team".to_string()))?;
            u8::from_value(value.clone(), "team")?
        };
        let kind: u8 = {
            let value = values.get("kind").ok_or(ParseError::UnknownGameEvent("kind".to_string()))?;
            u8::from_value(value.clone(), "kind")?
        };
        Ok(MiniGameWinEvent {
            team,
            kind
        })
    }
}

pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl FromRawGameEvent for SentryOnGoActiveEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(SentryOnGoActiveEvent {
            index
        })
    }
}

pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl FromRawGameEvent for DuckXpLevelUpEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let level: u16 = {
            let value = values.get("level").ok_or(ParseError::UnknownGameEvent("level".to_string()))?;
            u16::from_value(value.clone(), "level")?
        };
        Ok(DuckXpLevelUpEvent {
            level
        })
    }
}

pub struct HLTVStatusEvent {
    pub clients: u32,
    pub slots: u32,
    pub proxies: u16,
    pub master: String,
}
impl FromRawGameEvent for HLTVStatusEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let clients: u32 = {
            let value = values.get("clients").ok_or(ParseError::UnknownGameEvent("clients".to_string()))?;
            u32::from_value(value.clone(), "clients")?
        };
        let slots: u32 = {
            let value = values.get("slots").ok_or(ParseError::UnknownGameEvent("slots".to_string()))?;
            u32::from_value(value.clone(), "slots")?
        };
        let proxies: u16 = {
            let value = values.get("proxies").ok_or(ParseError::UnknownGameEvent("proxies".to_string()))?;
            u16::from_value(value.clone(), "proxies")?
        };
        let master: String = {
            let value = values.get("master").ok_or(ParseError::UnknownGameEvent("master".to_string()))?;
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

pub struct HLTVCameramanEvent {
    pub index: u16,
}
impl FromRawGameEvent for HLTVCameramanEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        Ok(HLTVCameramanEvent {
            index
        })
    }
}

pub struct HLTVRankCameraEvent {
    pub index: u8,
    pub rank: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVRankCameraEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u8 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u8::from_value(value.clone(), "index")?
        };
        let rank: f32 = {
            let value = values.get("rank").ok_or(ParseError::UnknownGameEvent("rank".to_string()))?;
            f32::from_value(value.clone(), "rank")?
        };
        let target: u16 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVRankCameraEvent {
            index,
            rank,
            target
        })
    }
}

pub struct HLTVRankEntityEvent {
    pub index: u16,
    pub rank: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVRankEntityEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let index: u16 = {
            let value = values.get("index").ok_or(ParseError::UnknownGameEvent("index".to_string()))?;
            u16::from_value(value.clone(), "index")?
        };
        let rank: f32 = {
            let value = values.get("rank").ok_or(ParseError::UnknownGameEvent("rank".to_string()))?;
            f32::from_value(value.clone(), "rank")?
        };
        let target: u16 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVRankEntityEvent {
            index,
            rank,
            target
        })
    }
}

pub struct HLTVFixedEvent {
    pub posx: u32,
    pub posy: u32,
    pub posz: u32,
    pub theta: u16,
    pub phi: u16,
    pub offset: u16,
    pub fov: f32,
    pub target: u16,
}
impl FromRawGameEvent for HLTVFixedEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let posx: u32 = {
            let value = values.get("posx").ok_or(ParseError::UnknownGameEvent("posx".to_string()))?;
            u32::from_value(value.clone(), "posx")?
        };
        let posy: u32 = {
            let value = values.get("posy").ok_or(ParseError::UnknownGameEvent("posy".to_string()))?;
            u32::from_value(value.clone(), "posy")?
        };
        let posz: u32 = {
            let value = values.get("posz").ok_or(ParseError::UnknownGameEvent("posz".to_string()))?;
            u32::from_value(value.clone(), "posz")?
        };
        let theta: u16 = {
            let value = values.get("theta").ok_or(ParseError::UnknownGameEvent("theta".to_string()))?;
            u16::from_value(value.clone(), "theta")?
        };
        let phi: u16 = {
            let value = values.get("phi").ok_or(ParseError::UnknownGameEvent("phi".to_string()))?;
            u16::from_value(value.clone(), "phi")?
        };
        let offset: u16 = {
            let value = values.get("offset").ok_or(ParseError::UnknownGameEvent("offset".to_string()))?;
            u16::from_value(value.clone(), "offset")?
        };
        let fov: f32 = {
            let value = values.get("fov").ok_or(ParseError::UnknownGameEvent("fov".to_string()))?;
            f32::from_value(value.clone(), "fov")?
        };
        let target: u16 = {
            let value = values.get("target").ok_or(ParseError::UnknownGameEvent("target".to_string()))?;
            u16::from_value(value.clone(), "target")?
        };
        Ok(HLTVFixedEvent {
            posx,
            posy,
            posz,
            theta,
            phi,
            offset,
            fov,
            target
        })
    }
}

pub struct HLTVChaseEvent {
    pub target1: u16,
    pub target2: u16,
    pub distance: u16,
    pub theta: u16,
    pub phi: u16,
    pub inertia: u8,
    pub ineye: u8,
}
impl FromRawGameEvent for HLTVChaseEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let target1: u16 = {
            let value = values.get("target1").ok_or(ParseError::UnknownGameEvent("target1".to_string()))?;
            u16::from_value(value.clone(), "target1")?
        };
        let target2: u16 = {
            let value = values.get("target2").ok_or(ParseError::UnknownGameEvent("target2".to_string()))?;
            u16::from_value(value.clone(), "target2")?
        };
        let distance: u16 = {
            let value = values.get("distance").ok_or(ParseError::UnknownGameEvent("distance".to_string()))?;
            u16::from_value(value.clone(), "distance")?
        };
        let theta: u16 = {
            let value = values.get("theta").ok_or(ParseError::UnknownGameEvent("theta".to_string()))?;
            u16::from_value(value.clone(), "theta")?
        };
        let phi: u16 = {
            let value = values.get("phi").ok_or(ParseError::UnknownGameEvent("phi".to_string()))?;
            u16::from_value(value.clone(), "phi")?
        };
        let inertia: u8 = {
            let value = values.get("inertia").ok_or(ParseError::UnknownGameEvent("inertia".to_string()))?;
            u8::from_value(value.clone(), "inertia")?
        };
        let ineye: u8 = {
            let value = values.get("ineye").ok_or(ParseError::UnknownGameEvent("ineye".to_string()))?;
            u8::from_value(value.clone(), "ineye")?
        };
        Ok(HLTVChaseEvent {
            target1,
            target2,
            distance,
            theta,
            phi,
            inertia,
            ineye
        })
    }
}

pub struct HLTVMessageEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVMessageEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVMessageEvent {
            text
        })
    }
}

pub struct HLTVTitleEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVTitleEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVTitleEvent {
            text
        })
    }
}

pub struct HLTVChatEvent {
    pub text: String,
}
impl FromRawGameEvent for HLTVChatEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let text: String = {
            let value = values.get("text").ok_or(ParseError::UnknownGameEvent("text".to_string()))?;
            String::from_value(value.clone(), "text")?
        };
        Ok(HLTVChatEvent {
            text
        })
    }
}

pub struct ReplayStartRecordEvent {

}
impl FromRawGameEvent for ReplayStartRecordEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ReplayStartRecordEvent {

        })
    }
}

pub struct ReplaySessionInfoEvent {
    pub sn: String,
    pub di: u8,
    pub cb: u32,
    pub st: u32,
}
impl FromRawGameEvent for ReplaySessionInfoEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let sn: String = {
            let value = values.get("sn").ok_or(ParseError::UnknownGameEvent("sn".to_string()))?;
            String::from_value(value.clone(), "sn")?
        };
        let di: u8 = {
            let value = values.get("di").ok_or(ParseError::UnknownGameEvent("di".to_string()))?;
            u8::from_value(value.clone(), "di")?
        };
        let cb: u32 = {
            let value = values.get("cb").ok_or(ParseError::UnknownGameEvent("cb".to_string()))?;
            u32::from_value(value.clone(), "cb")?
        };
        let st: u32 = {
            let value = values.get("st").ok_or(ParseError::UnknownGameEvent("st".to_string()))?;
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

pub struct ReplayEndRecordEvent {

}
impl FromRawGameEvent for ReplayEndRecordEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ReplayEndRecordEvent {

        })
    }
}

pub struct ReplayReplaysAvailableEvent {

}
impl FromRawGameEvent for ReplayReplaysAvailableEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {

        Ok(ReplayReplaysAvailableEvent {

        })
    }
}

pub struct ReplayServerErrorEvent {
    pub error: String,
}
impl FromRawGameEvent for ReplayServerErrorEvent {
    fn from_raw_event(values: HashMap<String, GameEventValue>) -> Result<Self> {
        let error: String = {
            let value = values.get("error").ok_or(ParseError::UnknownGameEvent("error".to_string()))?;
            String::from_value(value.clone(), "error")?
        };
        Ok(ReplayServerErrorEvent {
            error
        })
    }
}


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
    StorePricesheetUpdated(StorePricesheetUpdatedEvent),
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
    ControlPointFakeCaptureMult(ControlPointFakeCaptureMultEvent),
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
    TeamPlayTeambalancedPlayer(TeamPlayTeambalancedPlayerEvent),
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
    PlayerChargedeployed(PlayerChargedeployedEvent),
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
    LocalPlayerWinddown(LocalPlayerWinddownEvent),
    PlayerInvulned(PlayerInvulnedEvent),
    EscortSpeed(EscortSpeedEvent),
    EscortProgress(EscortProgressEvent),
    EscortRecede(EscortRecedeEvent),
    GameUIActivated(GameUIActivatedEvent),
    GameUIHidden(GameUIHiddenEvent),
    PlayerEscortScore(PlayerEscortScoreEvent),
    PlayerHealOnHit(PlayerHealOnHitEvent),
    PlayerStealsandvich(PlayerStealsandvichEvent),
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
    PlayerHighfiveStart(PlayerHighfiveStartEvent),
    PlayerHighfiveCancel(PlayerHighfiveCancelEvent),
    PlayerHighfiveSuccess(PlayerHighfiveSuccessEvent),
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
    MvmMedicPowerupShared(MvmMedicPowerupSharedEvent),
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
    PlayerDirecthitStun(PlayerDirecthitStunEvent),
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
    CompetitiveSkillratingUpdate(CompetitiveSkillratingUpdateEvent),
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
    ReplayServerError(ReplayServerErrorEvent)
}

impl GameEvent {
    fn from_raw_event(event: RawGameEvent) -> Result<Self> {
        Ok(match event.name.as_str() {
            "server_spawn" => GameEvent::ServerSpawn(ServerSpawnEvent::from_raw_event(event.values)?),
            "server_shutdown" => GameEvent::ServerShutdown(ServerShutdownEvent::from_raw_event(event.values)?),
            "server_cvar" => GameEvent::ServerCvar(ServerCvarEvent::from_raw_event(event.values)?),
            "server_message" => GameEvent::ServerMessage(ServerMessageEvent::from_raw_event(event.values)?),
            "server_addban" => GameEvent::ServerAddBan(ServerAddBanEvent::from_raw_event(event.values)?),
            "server_removeban" => GameEvent::ServerRemoveBan(ServerRemoveBanEvent::from_raw_event(event.values)?),
            "player_connect" => GameEvent::PlayerConnect(PlayerConnectEvent::from_raw_event(event.values)?),
            "player_connect_client" => GameEvent::PlayerConnectClient(PlayerConnectClientEvent::from_raw_event(event.values)?),
            "player_info" => GameEvent::PlayerInfo(PlayerInfoEvent::from_raw_event(event.values)?),
            "player_disconnect" => GameEvent::PlayerDisconnect(PlayerDisconnectEvent::from_raw_event(event.values)?),
            "player_activate" => GameEvent::PlayerActivate(PlayerActivateEvent::from_raw_event(event.values)?),
            "player_say" => GameEvent::PlayerSay(PlayerSayEvent::from_raw_event(event.values)?),
            "client_disconnect" => GameEvent::ClientDisconnect(ClientDisconnectEvent::from_raw_event(event.values)?),
            "client_beginconnect" => GameEvent::ClientBeginConnect(ClientBeginConnectEvent::from_raw_event(event.values)?),
            "client_connected" => GameEvent::ClientConnected(ClientConnectedEvent::from_raw_event(event.values)?),
            "client_fullconnect" => GameEvent::ClientFullConnect(ClientFullConnectEvent::from_raw_event(event.values)?),
            "host_quit" => GameEvent::HostQuit(HostQuitEvent::from_raw_event(event.values)?),
            "team_info" => GameEvent::TeamInfo(TeamInfoEvent::from_raw_event(event.values)?),
            "team_score" => GameEvent::TeamScore(TeamScoreEvent::from_raw_event(event.values)?),
            "teamplay_broadcast_audio" => GameEvent::TeamPlayBroadcastAudio(TeamPlayBroadcastAudioEvent::from_raw_event(event.values)?),
            "player_team" => GameEvent::PlayerTeam(PlayerTeamEvent::from_raw_event(event.values)?),
            "player_class" => GameEvent::PlayerClass(PlayerClassEvent::from_raw_event(event.values)?),
            "player_death" => GameEvent::PlayerDeath(PlayerDeathEvent::from_raw_event(event.values)?),
            "player_hurt" => GameEvent::PlayerHurt(PlayerHurtEvent::from_raw_event(event.values)?),
            "player_chat" => GameEvent::PlayerChat(PlayerChatEvent::from_raw_event(event.values)?),
            "player_score" => GameEvent::PlayerScore(PlayerScoreEvent::from_raw_event(event.values)?),
            "player_spawn" => GameEvent::PlayerSpawn(PlayerSpawnEvent::from_raw_event(event.values)?),
            "player_shoot" => GameEvent::PlayerShoot(PlayerShootEvent::from_raw_event(event.values)?),
            "player_use" => GameEvent::PlayerUse(PlayerUseEvent::from_raw_event(event.values)?),
            "player_changename" => GameEvent::PlayerChangeName(PlayerChangeNameEvent::from_raw_event(event.values)?),
            "player_hintmessage" => GameEvent::PlayerHintMessage(PlayerHintMessageEvent::from_raw_event(event.values)?),
            "base_player_teleported" => GameEvent::BasePlayerTeleported(BasePlayerTeleportedEvent::from_raw_event(event.values)?),
            "game_init" => GameEvent::GameInit(GameInitEvent::from_raw_event(event.values)?),
            "game_newmap" => GameEvent::GameNewMap(GameNewMapEvent::from_raw_event(event.values)?),
            "game_start" => GameEvent::GameStart(GameStartEvent::from_raw_event(event.values)?),
            "game_end" => GameEvent::GameEnd(GameEndEvent::from_raw_event(event.values)?),
            "round_start" => GameEvent::RoundStart(RoundStartEvent::from_raw_event(event.values)?),
            "round_end" => GameEvent::RoundEnd(RoundEndEvent::from_raw_event(event.values)?),
            "game_message" => GameEvent::GameMessage(GameMessageEvent::from_raw_event(event.values)?),
            "break_breakable" => GameEvent::BreakBreakable(BreakBreakableEvent::from_raw_event(event.values)?),
            "break_prop" => GameEvent::BreakProp(BreakPropEvent::from_raw_event(event.values)?),
            "entity_killed" => GameEvent::EntityKilled(EntityKilledEvent::from_raw_event(event.values)?),
            "bonus_updated" => GameEvent::BonusUpdated(BonusUpdatedEvent::from_raw_event(event.values)?),
            "achievement_event" => GameEvent::AchievementEvent(AchievementEventEvent::from_raw_event(event.values)?),
            "achievement_increment" => GameEvent::AchievementIncrement(AchievementIncrementEvent::from_raw_event(event.values)?),
            "physgun_pickup" => GameEvent::PhysgunPickup(PhysgunPickupEvent::from_raw_event(event.values)?),
            "flare_ignite_npc" => GameEvent::FlareIgniteNpc(FlareIgniteNpcEvent::from_raw_event(event.values)?),
            "helicopter_grenade_punt_miss" => GameEvent::HelicopterGrenadePuntMiss(HelicopterGrenadePuntMissEvent::from_raw_event(event.values)?),
            "user_data_downloaded" => GameEvent::UserDataDownloaded(UserDataDownloadedEvent::from_raw_event(event.values)?),
            "ragdoll_dissolved" => GameEvent::RagdollDissolved(RagdollDissolvedEvent::from_raw_event(event.values)?),
            "hltv_changed_mode" => GameEvent::HLTVChangedMode(HLTVChangedModeEvent::from_raw_event(event.values)?),
            "hltv_changed_target" => GameEvent::HLTVChangedTarget(HLTVChangedTargetEvent::from_raw_event(event.values)?),
            "vote_ended" => GameEvent::VoteEnded(VoteEndedEvent::from_raw_event(event.values)?),
            "vote_started" => GameEvent::VoteStarted(VoteStartedEvent::from_raw_event(event.values)?),
            "vote_changed" => GameEvent::VoteChanged(VoteChangedEvent::from_raw_event(event.values)?),
            "vote_passed" => GameEvent::VotePassed(VotePassedEvent::from_raw_event(event.values)?),
            "vote_failed" => GameEvent::VoteFailed(VoteFailedEvent::from_raw_event(event.values)?),
            "vote_cast" => GameEvent::VoteCast(VoteCastEvent::from_raw_event(event.values)?),
            "vote_options" => GameEvent::VoteOptions(VoteOptionsEvent::from_raw_event(event.values)?),
            "replay_saved" => GameEvent::ReplaySaved(ReplaySavedEvent::from_raw_event(event.values)?),
            "entered_performance_mode" => GameEvent::EnteredPerformanceMode(EnteredPerformanceModeEvent::from_raw_event(event.values)?),
            "browse_replays" => GameEvent::BrowseReplays(BrowseReplaysEvent::from_raw_event(event.values)?),
            "replay_youtube_stats" => GameEvent::ReplayYoutubeStats(ReplayYoutubeStatsEvent::from_raw_event(event.values)?),
            "inventory_updated" => GameEvent::InventoryUpdated(InventoryUpdatedEvent::from_raw_event(event.values)?),
            "cart_updated" => GameEvent::CartUpdated(CartUpdatedEvent::from_raw_event(event.values)?),
            "store_pricesheet_updated" => GameEvent::StorePricesheetUpdated(StorePricesheetUpdatedEvent::from_raw_event(event.values)?),
            "gc_connected" => GameEvent::GcConnected(GcConnectedEvent::from_raw_event(event.values)?),
            "item_schema_initialized" => GameEvent::ItemSchemaInitialized(ItemSchemaInitializedEvent::from_raw_event(event.values)?),
            "intro_finish" => GameEvent::IntroFinish(IntroFinishEvent::from_raw_event(event.values)?),
            "intro_nextcamera" => GameEvent::IntroNextCamera(IntroNextCameraEvent::from_raw_event(event.values)?),
            "mm_lobby_chat" => GameEvent::MmLobbyChat(MmLobbyChatEvent::from_raw_event(event.values)?),
            "mm_lobby_member_join" => GameEvent::MmLobbyMemberJoin(MmLobbyMemberJoinEvent::from_raw_event(event.values)?),
            "mm_lobby_member_leave" => GameEvent::MmLobbyMemberLeave(MmLobbyMemberLeaveEvent::from_raw_event(event.values)?),
            "player_changeclass" => GameEvent::PlayerChangeClass(PlayerChangeClassEvent::from_raw_event(event.values)?),
            "tf_map_time_remaining" => GameEvent::TfMapTimeRemaining(TfMapTimeRemainingEvent::from_raw_event(event.values)?),
            "tf_game_over" => GameEvent::TfGameOver(TfGameOverEvent::from_raw_event(event.values)?),
            "ctf_flag_captured" => GameEvent::CtfFlagCaptured(CtfFlagCapturedEvent::from_raw_event(event.values)?),
            "controlpoint_initialized" => GameEvent::ControlPointInitialized(ControlPointInitializedEvent::from_raw_event(event.values)?),
            "controlpoint_updateimages" => GameEvent::ControlPointUpdateImages(ControlPointUpdateImagesEvent::from_raw_event(event.values)?),
            "controlpoint_updatelayout" => GameEvent::ControlPointUpdateLayout(ControlPointUpdateLayoutEvent::from_raw_event(event.values)?),
            "controlpoint_updatecapping" => GameEvent::ControlPointUpdateCapping(ControlPointUpdateCappingEvent::from_raw_event(event.values)?),
            "controlpoint_updateowner" => GameEvent::ControlPointUpdateOwner(ControlPointUpdateOwnerEvent::from_raw_event(event.values)?),
            "controlpoint_starttouch" => GameEvent::ControlPointStartTouch(ControlPointStartTouchEvent::from_raw_event(event.values)?),
            "controlpoint_endtouch" => GameEvent::ControlPointEndTouch(ControlPointEndTouchEvent::from_raw_event(event.values)?),
            "controlpoint_pulse_element" => GameEvent::ControlPointPulseElement(ControlPointPulseElementEvent::from_raw_event(event.values)?),
            "controlpoint_fake_capture" => GameEvent::ControlPointFakeCapture(ControlPointFakeCaptureEvent::from_raw_event(event.values)?),
            "controlpoint_fake_capture_mult" => GameEvent::ControlPointFakeCaptureMult(ControlPointFakeCaptureMultEvent::from_raw_event(event.values)?),
            "teamplay_round_selected" => GameEvent::TeamPlayRoundSelected(TeamPlayRoundSelectedEvent::from_raw_event(event.values)?),
            "teamplay_round_start" => GameEvent::TeamPlayRoundStart(TeamPlayRoundStartEvent::from_raw_event(event.values)?),
            "teamplay_round_active" => GameEvent::TeamPlayRoundActive(TeamPlayRoundActiveEvent::from_raw_event(event.values)?),
            "teamplay_waiting_begins" => GameEvent::TeamPlayWaitingBegins(TeamPlayWaitingBeginsEvent::from_raw_event(event.values)?),
            "teamplay_waiting_ends" => GameEvent::TeamPlayWaitingEnds(TeamPlayWaitingEndsEvent::from_raw_event(event.values)?),
            "teamplay_waiting_abouttoend" => GameEvent::TeamPlayWaitingAboutToEnd(TeamPlayWaitingAboutToEndEvent::from_raw_event(event.values)?),
            "teamplay_restart_round" => GameEvent::TeamPlayRestartRound(TeamPlayRestartRoundEvent::from_raw_event(event.values)?),
            "teamplay_ready_restart" => GameEvent::TeamPlayReadyRestart(TeamPlayReadyRestartEvent::from_raw_event(event.values)?),
            "teamplay_round_restart_seconds" => GameEvent::TeamPlayRoundRestartSeconds(TeamPlayRoundRestartSecondsEvent::from_raw_event(event.values)?),
            "teamplay_team_ready" => GameEvent::TeamPlayTeamReady(TeamPlayTeamReadyEvent::from_raw_event(event.values)?),
            "teamplay_round_win" => GameEvent::TeamPlayRoundWin(TeamPlayRoundWinEvent::from_raw_event(event.values)?),
            "teamplay_update_timer" => GameEvent::TeamPlayUpdateTimer(TeamPlayUpdateTimerEvent::from_raw_event(event.values)?),
            "teamplay_round_stalemate" => GameEvent::TeamPlayRoundStalemate(TeamPlayRoundStalemateEvent::from_raw_event(event.values)?),
            "teamplay_overtime_begin" => GameEvent::TeamPlayOvertimeBegin(TeamPlayOvertimeBeginEvent::from_raw_event(event.values)?),
            "teamplay_overtime_end" => GameEvent::TeamPlayOvertimeEnd(TeamPlayOvertimeEndEvent::from_raw_event(event.values)?),
            "teamplay_suddendeath_begin" => GameEvent::TeamPlaySuddenDeathBegin(TeamPlaySuddenDeathBeginEvent::from_raw_event(event.values)?),
            "teamplay_suddendeath_end" => GameEvent::TeamPlaySuddenDeathEnd(TeamPlaySuddenDeathEndEvent::from_raw_event(event.values)?),
            "teamplay_game_over" => GameEvent::TeamPlayGameOver(TeamPlayGameOverEvent::from_raw_event(event.values)?),
            "teamplay_map_time_remaining" => GameEvent::TeamPlayMapTimeRemaining(TeamPlayMapTimeRemainingEvent::from_raw_event(event.values)?),
            "teamplay_timer_flash" => GameEvent::TeamPlayTimerFlash(TeamPlayTimerFlashEvent::from_raw_event(event.values)?),
            "teamplay_timer_time_added" => GameEvent::TeamPlayTimerTimeAdded(TeamPlayTimerTimeAddedEvent::from_raw_event(event.values)?),
            "teamplay_point_startcapture" => GameEvent::TeamPlayPointStartCapture(TeamPlayPointStartCaptureEvent::from_raw_event(event.values)?),
            "teamplay_point_captured" => GameEvent::TeamPlayPointCaptured(TeamPlayPointCapturedEvent::from_raw_event(event.values)?),
            "teamplay_point_locked" => GameEvent::TeamPlayPointLocked(TeamPlayPointLockedEvent::from_raw_event(event.values)?),
            "teamplay_point_unlocked" => GameEvent::TeamPlayPointUnlocked(TeamPlayPointUnlockedEvent::from_raw_event(event.values)?),
            "teamplay_capture_broken" => GameEvent::TeamPlayCaptureBroken(TeamPlayCaptureBrokenEvent::from_raw_event(event.values)?),
            "teamplay_capture_blocked" => GameEvent::TeamPlayCaptureBlocked(TeamPlayCaptureBlockedEvent::from_raw_event(event.values)?),
            "teamplay_flag_event" => GameEvent::TeamPlayFlagEvent(TeamPlayFlagEventEvent::from_raw_event(event.values)?),
            "teamplay_win_panel" => GameEvent::TeamPlayWinPanel(TeamPlayWinPanelEvent::from_raw_event(event.values)?),
            "teamplay_teambalanced_player" => GameEvent::TeamPlayTeambalancedPlayer(TeamPlayTeambalancedPlayerEvent::from_raw_event(event.values)?),
            "teamplay_setup_finished" => GameEvent::TeamPlaySetupFinished(TeamPlaySetupFinishedEvent::from_raw_event(event.values)?),
            "teamplay_alert" => GameEvent::TeamPlayAlert(TeamPlayAlertEvent::from_raw_event(event.values)?),
            "training_complete" => GameEvent::TrainingComplete(TrainingCompleteEvent::from_raw_event(event.values)?),
            "show_freezepanel" => GameEvent::ShowFreezePanel(ShowFreezePanelEvent::from_raw_event(event.values)?),
            "hide_freezepanel" => GameEvent::HideFreezePanel(HideFreezePanelEvent::from_raw_event(event.values)?),
            "freezecam_started" => GameEvent::FreezeCamStarted(FreezeCamStartedEvent::from_raw_event(event.values)?),
            "localplayer_changeteam" => GameEvent::LocalPlayerChangeTeam(LocalPlayerChangeTeamEvent::from_raw_event(event.values)?),
            "localplayer_score_changed" => GameEvent::LocalPlayerScoreChanged(LocalPlayerScoreChangedEvent::from_raw_event(event.values)?),
            "localplayer_changeclass" => GameEvent::LocalPlayerChangeClass(LocalPlayerChangeClassEvent::from_raw_event(event.values)?),
            "localplayer_respawn" => GameEvent::LocalPlayerRespawn(LocalPlayerRespawnEvent::from_raw_event(event.values)?),
            "building_info_changed" => GameEvent::BuildingInfoChanged(BuildingInfoChangedEvent::from_raw_event(event.values)?),
            "localplayer_changedisguise" => GameEvent::LocalPlayerChangeDisguise(LocalPlayerChangeDisguiseEvent::from_raw_event(event.values)?),
            "player_account_changed" => GameEvent::PlayerAccountChanged(PlayerAccountChangedEvent::from_raw_event(event.values)?),
            "spy_pda_reset" => GameEvent::SpyPdaReset(SpyPdaResetEvent::from_raw_event(event.values)?),
            "flagstatus_update" => GameEvent::FlagStatusUpdate(FlagStatusUpdateEvent::from_raw_event(event.values)?),
            "player_stats_updated" => GameEvent::PlayerStatsUpdated(PlayerStatsUpdatedEvent::from_raw_event(event.values)?),
            "playing_commentary" => GameEvent::PlayingCommentary(PlayingCommentaryEvent::from_raw_event(event.values)?),
            "player_chargedeployed" => GameEvent::PlayerChargedeployed(PlayerChargedeployedEvent::from_raw_event(event.values)?),
            "player_builtobject" => GameEvent::PlayerBuiltObject(PlayerBuiltObjectEvent::from_raw_event(event.values)?),
            "player_upgradedobject" => GameEvent::PlayerUpgradedObject(PlayerUpgradedObjectEvent::from_raw_event(event.values)?),
            "player_carryobject" => GameEvent::PlayerCarryObject(PlayerCarryObjectEvent::from_raw_event(event.values)?),
            "player_dropobject" => GameEvent::PlayerDropObject(PlayerDropObjectEvent::from_raw_event(event.values)?),
            "object_removed" => GameEvent::ObjectRemoved(ObjectRemovedEvent::from_raw_event(event.values)?),
            "object_destroyed" => GameEvent::ObjectDestroyed(ObjectDestroyedEvent::from_raw_event(event.values)?),
            "object_detonated" => GameEvent::ObjectDetonated(ObjectDetonatedEvent::from_raw_event(event.values)?),
            "achievement_earned" => GameEvent::AchievementEarned(AchievementEarnedEvent::from_raw_event(event.values)?),
            "spec_target_updated" => GameEvent::SpecTargetUpdated(SpecTargetUpdatedEvent::from_raw_event(event.values)?),
            "tournament_stateupdate" => GameEvent::TournamentStateUpdate(TournamentStateUpdateEvent::from_raw_event(event.values)?),
            "tournament_enablecountdown" => GameEvent::TournamentEnableCountdown(TournamentEnableCountdownEvent::from_raw_event(event.values)?),
            "player_calledformedic" => GameEvent::PlayerCalledForMedic(PlayerCalledForMedicEvent::from_raw_event(event.values)?),
            "localplayer_becameobserver" => GameEvent::LocalPlayerBecameObserver(LocalPlayerBecameObserverEvent::from_raw_event(event.values)?),
            "player_ignited_inv" => GameEvent::PlayerIgnitedInv(PlayerIgnitedInvEvent::from_raw_event(event.values)?),
            "player_ignited" => GameEvent::PlayerIgnited(PlayerIgnitedEvent::from_raw_event(event.values)?),
            "player_extinguished" => GameEvent::PlayerExtinguished(PlayerExtinguishedEvent::from_raw_event(event.values)?),
            "player_teleported" => GameEvent::PlayerTeleported(PlayerTeleportedEvent::from_raw_event(event.values)?),
            "player_healedmediccall" => GameEvent::PlayerHealedMedicCall(PlayerHealedMedicCallEvent::from_raw_event(event.values)?),
            "localplayer_chargeready" => GameEvent::LocalPlayerChargeReady(LocalPlayerChargeReadyEvent::from_raw_event(event.values)?),
            "localplayer_winddown" => GameEvent::LocalPlayerWinddown(LocalPlayerWinddownEvent::from_raw_event(event.values)?),
            "player_invulned" => GameEvent::PlayerInvulned(PlayerInvulnedEvent::from_raw_event(event.values)?),
            "escort_speed" => GameEvent::EscortSpeed(EscortSpeedEvent::from_raw_event(event.values)?),
            "escort_progress" => GameEvent::EscortProgress(EscortProgressEvent::from_raw_event(event.values)?),
            "escort_recede" => GameEvent::EscortRecede(EscortRecedeEvent::from_raw_event(event.values)?),
            "gameui_activated" => GameEvent::GameUIActivated(GameUIActivatedEvent::from_raw_event(event.values)?),
            "gameui_hidden" => GameEvent::GameUIHidden(GameUIHiddenEvent::from_raw_event(event.values)?),
            "player_escort_score" => GameEvent::PlayerEscortScore(PlayerEscortScoreEvent::from_raw_event(event.values)?),
            "player_healonhit" => GameEvent::PlayerHealOnHit(PlayerHealOnHitEvent::from_raw_event(event.values)?),
            "player_stealsandvich" => GameEvent::PlayerStealsandvich(PlayerStealsandvichEvent::from_raw_event(event.values)?),
            "show_class_layout" => GameEvent::ShowClassLayout(ShowClassLayoutEvent::from_raw_event(event.values)?),
            "show_vs_panel" => GameEvent::ShowVsPanel(ShowVsPanelEvent::from_raw_event(event.values)?),
            "player_damaged" => GameEvent::PlayerDamaged(PlayerDamagedEvent::from_raw_event(event.values)?),
            "arena_player_notification" => GameEvent::ArenaPlayerNotification(ArenaPlayerNotificationEvent::from_raw_event(event.values)?),
            "arena_match_maxstreak" => GameEvent::ArenaMatchMaxStreak(ArenaMatchMaxStreakEvent::from_raw_event(event.values)?),
            "arena_round_start" => GameEvent::ArenaRoundStart(ArenaRoundStartEvent::from_raw_event(event.values)?),
            "arena_win_panel" => GameEvent::ArenaWinPanel(ArenaWinPanelEvent::from_raw_event(event.values)?),
            "pve_win_panel" => GameEvent::PveWinPanel(PveWinPanelEvent::from_raw_event(event.values)?),
            "air_dash" => GameEvent::AirDash(AirDashEvent::from_raw_event(event.values)?),
            "landed" => GameEvent::Landed(LandedEvent::from_raw_event(event.values)?),
            "player_damage_dodged" => GameEvent::PlayerDamageDodged(PlayerDamageDodgedEvent::from_raw_event(event.values)?),
            "player_stunned" => GameEvent::PlayerStunned(PlayerStunnedEvent::from_raw_event(event.values)?),
            "scout_grand_slam" => GameEvent::ScoutGrandSlam(ScoutGrandSlamEvent::from_raw_event(event.values)?),
            "scout_slamdoll_landed" => GameEvent::ScoutSlamdollLanded(ScoutSlamdollLandedEvent::from_raw_event(event.values)?),
            "arrow_impact" => GameEvent::ArrowImpact(ArrowImpactEvent::from_raw_event(event.values)?),
            "player_jarated" => GameEvent::PlayerJarated(PlayerJaratedEvent::from_raw_event(event.values)?),
            "player_jarated_fade" => GameEvent::PlayerJaratedFade(PlayerJaratedFadeEvent::from_raw_event(event.values)?),
            "player_shield_blocked" => GameEvent::PlayerShieldBlocked(PlayerShieldBlockedEvent::from_raw_event(event.values)?),
            "player_pinned" => GameEvent::PlayerPinned(PlayerPinnedEvent::from_raw_event(event.values)?),
            "player_healedbymedic" => GameEvent::PlayerHealedByMedic(PlayerHealedByMedicEvent::from_raw_event(event.values)?),
            "player_sapped_object" => GameEvent::PlayerSappedObject(PlayerSappedObjectEvent::from_raw_event(event.values)?),
            "item_found" => GameEvent::ItemFound(ItemFoundEvent::from_raw_event(event.values)?),
            "show_annotation" => GameEvent::ShowAnnotation(ShowAnnotationEvent::from_raw_event(event.values)?),
            "hide_annotation" => GameEvent::HideAnnotation(HideAnnotationEvent::from_raw_event(event.values)?),
            "post_inventory_application" => GameEvent::PostInventoryApplication(PostInventoryApplicationEvent::from_raw_event(event.values)?),
            "controlpoint_unlock_updated" => GameEvent::ControlPointUnlockUpdated(ControlPointUnlockUpdatedEvent::from_raw_event(event.values)?),
            "deploy_buff_banner" => GameEvent::DeployBuffBanner(DeployBuffBannerEvent::from_raw_event(event.values)?),
            "player_buff" => GameEvent::PlayerBuff(PlayerBuffEvent::from_raw_event(event.values)?),
            "medic_death" => GameEvent::MedicDeath(MedicDeathEvent::from_raw_event(event.values)?),
            "overtime_nag" => GameEvent::OvertimeNag(OvertimeNagEvent::from_raw_event(event.values)?),
            "teams_changed" => GameEvent::TeamsChanged(TeamsChangedEvent::from_raw_event(event.values)?),
            "halloween_pumpkin_grab" => GameEvent::HalloweenPumpkinGrab(HalloweenPumpkinGrabEvent::from_raw_event(event.values)?),
            "rocket_jump" => GameEvent::RocketJump(RocketJumpEvent::from_raw_event(event.values)?),
            "rocket_jump_landed" => GameEvent::RocketJumpLanded(RocketJumpLandedEvent::from_raw_event(event.values)?),
            "sticky_jump" => GameEvent::StickyJump(StickyJumpEvent::from_raw_event(event.values)?),
            "sticky_jump_landed" => GameEvent::StickyJumpLanded(StickyJumpLandedEvent::from_raw_event(event.values)?),
            "medic_defended" => GameEvent::MedicDefended(MedicDefendedEvent::from_raw_event(event.values)?),
            "localplayer_healed" => GameEvent::LocalPlayerHealed(LocalPlayerHealedEvent::from_raw_event(event.values)?),
            "player_destroyed_pipebomb" => GameEvent::PlayerDestroyedPipeBomb(PlayerDestroyedPipeBombEvent::from_raw_event(event.values)?),
            "object_deflected" => GameEvent::ObjectDeflected(ObjectDeflectedEvent::from_raw_event(event.values)?),
            "player_mvp" => GameEvent::PlayerMvp(PlayerMvpEvent::from_raw_event(event.values)?),
            "raid_spawn_mob" => GameEvent::RaidSpawnMob(RaidSpawnMobEvent::from_raw_event(event.values)?),
            "raid_spawn_squad" => GameEvent::RaidSpawnSquad(RaidSpawnSquadEvent::from_raw_event(event.values)?),
            "nav_blocked" => GameEvent::NavBlocked(NavBlockedEvent::from_raw_event(event.values)?),
            "path_track_passed" => GameEvent::PathTrackPassed(PathTrackPassedEvent::from_raw_event(event.values)?),
            "num_cappers_changed" => GameEvent::NumCappersChanged(NumCappersChangedEvent::from_raw_event(event.values)?),
            "player_regenerate" => GameEvent::PlayerRegenerate(PlayerRegenerateEvent::from_raw_event(event.values)?),
            "update_status_item" => GameEvent::UpdateStatusItem(UpdateStatusItemEvent::from_raw_event(event.values)?),
            "stats_resetround" => GameEvent::StatsResetRound(StatsResetRoundEvent::from_raw_event(event.values)?),
            "scorestats_accumulated_update" => GameEvent::ScoreStatsAccumulatedUpdate(ScoreStatsAccumulatedUpdateEvent::from_raw_event(event.values)?),
            "scorestats_accumulated_reset" => GameEvent::ScoreStatsAccumulatedReset(ScoreStatsAccumulatedResetEvent::from_raw_event(event.values)?),
            "achievement_earned_local" => GameEvent::AchievementEarnedLocal(AchievementEarnedLocalEvent::from_raw_event(event.values)?),
            "player_healed" => GameEvent::PlayerHealed(PlayerHealedEvent::from_raw_event(event.values)?),
            "building_healed" => GameEvent::BuildingHealed(BuildingHealedEvent::from_raw_event(event.values)?),
            "item_pickup" => GameEvent::ItemPickup(ItemPickupEvent::from_raw_event(event.values)?),
            "duel_status" => GameEvent::DuelStatus(DuelStatusEvent::from_raw_event(event.values)?),
            "fish_notice" => GameEvent::FishNotice(FishNoticeEvent::from_raw_event(event.values)?),
            "fish_notice__arm" => GameEvent::FishNoticeArm(FishNoticeArmEvent::from_raw_event(event.values)?),
            "throwable_hit" => GameEvent::ThrowableHit(ThrowableHitEvent::from_raw_event(event.values)?),
            "pumpkin_lord_summoned" => GameEvent::PumpkinLordSummoned(PumpkinLordSummonedEvent::from_raw_event(event.values)?),
            "pumpkin_lord_killed" => GameEvent::PumpkinLordKilled(PumpkinLordKilledEvent::from_raw_event(event.values)?),
            "merasmus_summoned" => GameEvent::MerasmusSummoned(MerasmusSummonedEvent::from_raw_event(event.values)?),
            "merasmus_killed" => GameEvent::MerasmusKilled(MerasmusKilledEvent::from_raw_event(event.values)?),
            "merasmus_escape_warning" => GameEvent::MerasmusEscapeWarning(MerasmusEscapeWarningEvent::from_raw_event(event.values)?),
            "merasmus_escaped" => GameEvent::MerasmusEscaped(MerasmusEscapedEvent::from_raw_event(event.values)?),
            "eyeball_boss_summoned" => GameEvent::EyeballBossSummoned(EyeballBossSummonedEvent::from_raw_event(event.values)?),
            "eyeball_boss_stunned" => GameEvent::EyeballBossStunned(EyeballBossStunnedEvent::from_raw_event(event.values)?),
            "eyeball_boss_killed" => GameEvent::EyeballBossKilled(EyeballBossKilledEvent::from_raw_event(event.values)?),
            "eyeball_boss_killer" => GameEvent::EyeballBossKiller(EyeballBossKillerEvent::from_raw_event(event.values)?),
            "eyeball_boss_escape_imminent" => GameEvent::EyeballBossEscapeImminent(EyeballBossEscapeImminentEvent::from_raw_event(event.values)?),
            "eyeball_boss_escaped" => GameEvent::EyeballBossEscaped(EyeballBossEscapedEvent::from_raw_event(event.values)?),
            "npc_hurt" => GameEvent::NpcHurt(NpcHurtEvent::from_raw_event(event.values)?),
            "controlpoint_timer_updated" => GameEvent::ControlPointTimerUpdated(ControlPointTimerUpdatedEvent::from_raw_event(event.values)?),
            "player_highfive_start" => GameEvent::PlayerHighfiveStart(PlayerHighfiveStartEvent::from_raw_event(event.values)?),
            "player_highfive_cancel" => GameEvent::PlayerHighfiveCancel(PlayerHighfiveCancelEvent::from_raw_event(event.values)?),
            "player_highfive_success" => GameEvent::PlayerHighfiveSuccess(PlayerHighfiveSuccessEvent::from_raw_event(event.values)?),
            "player_bonuspoints" => GameEvent::PlayerBonusPoints(PlayerBonusPointsEvent::from_raw_event(event.values)?),
            "player_upgraded" => GameEvent::PlayerUpgraded(PlayerUpgradedEvent::from_raw_event(event.values)?),
            "player_buyback" => GameEvent::PlayerBuyback(PlayerBuybackEvent::from_raw_event(event.values)?),
            "player_used_powerup_bottle" => GameEvent::PlayerUsedPowerUpBottle(PlayerUsedPowerUpBottleEvent::from_raw_event(event.values)?),
            "christmas_gift_grab" => GameEvent::ChristmasGiftGrab(ChristmasGiftGrabEvent::from_raw_event(event.values)?),
            "player_killed_achievement_zone" => GameEvent::PlayerKilledAchievementZone(PlayerKilledAchievementZoneEvent::from_raw_event(event.values)?),
            "party_updated" => GameEvent::PartyUpdated(PartyUpdatedEvent::from_raw_event(event.values)?),
            "lobby_updated" => GameEvent::LobbyUpdated(LobbyUpdatedEvent::from_raw_event(event.values)?),
            "mvm_mission_update" => GameEvent::MvmMissionUpdate(MvmMissionUpdateEvent::from_raw_event(event.values)?),
            "recalculate_holidays" => GameEvent::RecalculateHolidays(RecalculateHolidaysEvent::from_raw_event(event.values)?),
            "player_currency_changed" => GameEvent::PlayerCurrencyChanged(PlayerCurrencyChangedEvent::from_raw_event(event.values)?),
            "doomsday_rocket_open" => GameEvent::DoomsdayRocketOpen(DoomsdayRocketOpenEvent::from_raw_event(event.values)?),
            "remove_nemesis_relationships" => GameEvent::RemoveNemesisRelationships(RemoveNemesisRelationshipsEvent::from_raw_event(event.values)?),
            "mvm_creditbonus_wave" => GameEvent::MvmCreditBonusWave(MvmCreditBonusWaveEvent::from_raw_event(event.values)?),
            "mvm_creditbonus_all" => GameEvent::MvmCreditBonusAll(MvmCreditBonusAllEvent::from_raw_event(event.values)?),
            "mvm_creditbonus_all_advanced" => GameEvent::MvmCreditBonusAllAdvanced(MvmCreditBonusAllAdvancedEvent::from_raw_event(event.values)?),
            "mvm_quick_sentry_upgrade" => GameEvent::MvmQuickSentryUpgrade(MvmQuickSentryUpgradeEvent::from_raw_event(event.values)?),
            "mvm_tank_destroyed_by_players" => GameEvent::MvmTankDestroyedByPlayers(MvmTankDestroyedByPlayersEvent::from_raw_event(event.values)?),
            "mvm_kill_robot_delivering_bomb" => GameEvent::MvmKillRobotDeliveringBomb(MvmKillRobotDeliveringBombEvent::from_raw_event(event.values)?),
            "mvm_pickup_currency" => GameEvent::MvmPickupCurrency(MvmPickupCurrencyEvent::from_raw_event(event.values)?),
            "mvm_bomb_carrier_killed" => GameEvent::MvmBombCarrierKilled(MvmBombCarrierKilledEvent::from_raw_event(event.values)?),
            "mvm_sentrybuster_detonate" => GameEvent::MvmSentryBusterDetonate(MvmSentryBusterDetonateEvent::from_raw_event(event.values)?),
            "mvm_scout_marked_for_death" => GameEvent::MvmScoutMarkedForDeath(MvmScoutMarkedForDeathEvent::from_raw_event(event.values)?),
            "mvm_medic_powerup_shared" => GameEvent::MvmMedicPowerupShared(MvmMedicPowerupSharedEvent::from_raw_event(event.values)?),
            "mvm_begin_wave" => GameEvent::MvmBeginWave(MvmBeginWaveEvent::from_raw_event(event.values)?),
            "mvm_wave_complete" => GameEvent::MvmWaveComplete(MvmWaveCompleteEvent::from_raw_event(event.values)?),
            "mvm_mission_complete" => GameEvent::MvmMissionComplete(MvmMissionCompleteEvent::from_raw_event(event.values)?),
            "mvm_bomb_reset_by_player" => GameEvent::MvmBombResetByPlayer(MvmBombResetByPlayerEvent::from_raw_event(event.values)?),
            "mvm_bomb_alarm_triggered" => GameEvent::MvmBombAlarmTriggered(MvmBombAlarmTriggeredEvent::from_raw_event(event.values)?),
            "mvm_bomb_deploy_reset_by_player" => GameEvent::MvmBombDeployResetByPlayer(MvmBombDeployResetByPlayerEvent::from_raw_event(event.values)?),
            "mvm_wave_failed" => GameEvent::MvmWaveFailed(MvmWaveFailedEvent::from_raw_event(event.values)?),
            "mvm_reset_stats" => GameEvent::MvmResetStats(MvmResetStatsEvent::from_raw_event(event.values)?),
            "damage_resisted" => GameEvent::DamageResisted(DamageResistedEvent::from_raw_event(event.values)?),
            "revive_player_notify" => GameEvent::RevivePlayerNotify(RevivePlayerNotifyEvent::from_raw_event(event.values)?),
            "revive_player_stopped" => GameEvent::RevivePlayerStopped(RevivePlayerStoppedEvent::from_raw_event(event.values)?),
            "revive_player_complete" => GameEvent::RevivePlayerComplete(RevivePlayerCompleteEvent::from_raw_event(event.values)?),
            "player_turned_to_ghost" => GameEvent::PlayerTurnedToGhost(PlayerTurnedToGhostEvent::from_raw_event(event.values)?),
            "medigun_shield_blocked_damage" => GameEvent::MedigunShieldBlockedDamage(MedigunShieldBlockedDamageEvent::from_raw_event(event.values)?),
            "mvm_adv_wave_complete_no_gates" => GameEvent::MvmAdvWaveCompleteNoGates(MvmAdvWaveCompleteNoGatesEvent::from_raw_event(event.values)?),
            "mvm_sniper_headshot_currency" => GameEvent::MvmSniperHeadshotCurrency(MvmSniperHeadshotCurrencyEvent::from_raw_event(event.values)?),
            "mvm_mannhattan_pit" => GameEvent::MvmMannhattanPit(MvmMannhattanPitEvent::from_raw_event(event.values)?),
            "flag_carried_in_detection_zone" => GameEvent::FlagCarriedInDetectionZone(FlagCarriedInDetectionZoneEvent::from_raw_event(event.values)?),
            "mvm_adv_wave_killed_stun_radio" => GameEvent::MvmAdvWaveKilledStunRadio(MvmAdvWaveKilledStunRadioEvent::from_raw_event(event.values)?),
            "player_directhit_stun" => GameEvent::PlayerDirecthitStun(PlayerDirecthitStunEvent::from_raw_event(event.values)?),
            "mvm_sentrybuster_killed" => GameEvent::MvmSentryBusterKilled(MvmSentryBusterKilledEvent::from_raw_event(event.values)?),
            "upgrades_file_changed" => GameEvent::UpgradesFileChanged(UpgradesFileChangedEvent::from_raw_event(event.values)?),
            "rd_team_points_changed" => GameEvent::RdTeamPointsChanged(RdTeamPointsChangedEvent::from_raw_event(event.values)?),
            "rd_rules_state_changed" => GameEvent::RdRulesStateChanged(RdRulesStateChangedEvent::from_raw_event(event.values)?),
            "rd_robot_killed" => GameEvent::RdRobotKilled(RdRobotKilledEvent::from_raw_event(event.values)?),
            "rd_robot_impact" => GameEvent::RdRobotImpact(RdRobotImpactEvent::from_raw_event(event.values)?),
            "teamplay_pre_round_time_left" => GameEvent::TeamPlayPreRoundTimeLeft(TeamPlayPreRoundTimeLeftEvent::from_raw_event(event.values)?),
            "parachute_deploy" => GameEvent::ParachuteDeploy(ParachuteDeployEvent::from_raw_event(event.values)?),
            "parachute_holster" => GameEvent::ParachuteHolster(ParachuteHolsterEvent::from_raw_event(event.values)?),
            "kill_refills_meter" => GameEvent::KillRefillsMeter(KillRefillsMeterEvent::from_raw_event(event.values)?),
            "rps_taunt_event" => GameEvent::RpsTauntEvent(RpsTauntEventEvent::from_raw_event(event.values)?),
            "conga_kill" => GameEvent::CongaKill(CongaKillEvent::from_raw_event(event.values)?),
            "player_initial_spawn" => GameEvent::PlayerInitialSpawn(PlayerInitialSpawnEvent::from_raw_event(event.values)?),
            "competitive_victory" => GameEvent::CompetitiveVictory(CompetitiveVictoryEvent::from_raw_event(event.values)?),
            "competitive_skillrating_update" => GameEvent::CompetitiveSkillratingUpdate(CompetitiveSkillratingUpdateEvent::from_raw_event(event.values)?),
            "minigame_win" => GameEvent::MiniGameWin(MiniGameWinEvent::from_raw_event(event.values)?),
            "sentry_on_go_active" => GameEvent::SentryOnGoActive(SentryOnGoActiveEvent::from_raw_event(event.values)?),
            "duck_xp_level_up" => GameEvent::DuckXpLevelUp(DuckXpLevelUpEvent::from_raw_event(event.values)?),
            "hltv_status" => GameEvent::HLTVStatus(HLTVStatusEvent::from_raw_event(event.values)?),
            "hltv_cameraman" => GameEvent::HLTVCameraman(HLTVCameramanEvent::from_raw_event(event.values)?),
            "hltv_rank_camera" => GameEvent::HLTVRankCamera(HLTVRankCameraEvent::from_raw_event(event.values)?),
            "hltv_rank_entity" => GameEvent::HLTVRankEntity(HLTVRankEntityEvent::from_raw_event(event.values)?),
            "hltv_fixed" => GameEvent::HLTVFixed(HLTVFixedEvent::from_raw_event(event.values)?),
            "hltv_chase" => GameEvent::HLTVChase(HLTVChaseEvent::from_raw_event(event.values)?),
            "hltv_message" => GameEvent::HLTVMessage(HLTVMessageEvent::from_raw_event(event.values)?),
            "hltv_title" => GameEvent::HLTVTitle(HLTVTitleEvent::from_raw_event(event.values)?),
            "hltv_chat" => GameEvent::HLTVChat(HLTVChatEvent::from_raw_event(event.values)?),
            "replay_startrecord" => GameEvent::ReplayStartRecord(ReplayStartRecordEvent::from_raw_event(event.values)?),
            "replay_sessioninfo" => GameEvent::ReplaySessionInfo(ReplaySessionInfoEvent::from_raw_event(event.values)?),
            "replay_endrecord" => GameEvent::ReplayEndRecord(ReplayEndRecordEvent::from_raw_event(event.values)?),
            "replay_replaysavailable" => GameEvent::ReplayReplaysAvailable(ReplayReplaysAvailableEvent::from_raw_event(event.values)?),
            "replay_servererror" => GameEvent::ReplayServerError(ReplayServerErrorEvent::from_raw_event(event.values)?),
            _ => return Err(ParseError::UnknownGameEvent(event.name))
        })
    }
}

