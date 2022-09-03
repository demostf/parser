use super::gamevent::{EventValue, GameEventDefinition, GameEventEntry, RawGameEvent};
use crate::demo::Stream;
use crate::{ParseError, Result};
use bitbuffer::{BitRead, LittleEndian, BitWrite, BitWriteStream};
use serde::{Deserialize, Serialize};
use crate::demo::data::MaybeUtf8String;
fn read_value<'a, T: EventValue + BitRead<'a, LittleEndian> + Default>(
    stream: &mut Stream<'a>,
    entry: Option<&GameEventEntry>,
    name: &'static str,
) -> Result<T> {
    let entry = match entry {
        Some(entry) => entry,
        None => {
            return Ok(T::default());
        }
    };
    if T::value_type() != entry.kind {
        return Err(ParseError::InvalidGameEvent {
            expected_type: T::value_type(),
            name,
            found_type: entry.kind,
        });
    }
    Ok(T::read(stream)?)
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerSpawnEvent {
    pub hostname: MaybeUtf8String,
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
    pub game: MaybeUtf8String,
    pub map_name: MaybeUtf8String,
    pub max_players: u32,
    pub os: MaybeUtf8String,
    pub dedicated: bool,
    pub password: bool,
}
impl ServerSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerSpawnEvent {
            hostname: read_value::<MaybeUtf8String>(stream, iter.next(), "hostname")?,
            address: read_value::<MaybeUtf8String>(stream, iter.next(), "address")?,
            ip: read_value::<u32>(stream, iter.next(), "ip")?,
            port: read_value::<u16>(stream, iter.next(), "port")?,
            game: read_value::<MaybeUtf8String>(stream, iter.next(), "game")?,
            map_name: read_value::<MaybeUtf8String>(stream, iter.next(), "map_name")?,
            max_players: read_value::<u32>(stream, iter.next(), "max_players")?,
            os: read_value::<MaybeUtf8String>(stream, iter.next(), "os")?,
            dedicated: read_value::<bool>(stream, iter.next(), "dedicated")?,
            password: read_value::<bool>(stream, iter.next(), "password")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerChangeLevelFailedEvent {
    pub level_name: MaybeUtf8String,
}
impl ServerChangeLevelFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerChangeLevelFailedEvent {
            level_name: read_value::<MaybeUtf8String>(stream, iter.next(), "level_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerShutdownEvent {
    pub reason: MaybeUtf8String,
}
impl ServerShutdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerShutdownEvent {
            reason: read_value::<MaybeUtf8String>(stream, iter.next(), "reason")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerCvarEvent {
    pub cvar_name: MaybeUtf8String,
    pub cvar_value: MaybeUtf8String,
}
impl ServerCvarEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerCvarEvent {
            cvar_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cvar_name")?,
            cvar_value: read_value::<MaybeUtf8String>(stream, iter.next(), "cvar_value")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerMessageEvent {
    pub text: MaybeUtf8String,
}
impl ServerMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerMessageEvent {
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerAddBanEvent {
    pub name: MaybeUtf8String,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub ip: MaybeUtf8String,
    pub duration: MaybeUtf8String,
    pub by: MaybeUtf8String,
    pub kicked: bool,
}
impl ServerAddBanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerAddBanEvent {
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            ip: read_value::<MaybeUtf8String>(stream, iter.next(), "ip")?,
            duration: read_value::<MaybeUtf8String>(stream, iter.next(), "duration")?,
            by: read_value::<MaybeUtf8String>(stream, iter.next(), "by")?,
            kicked: read_value::<bool>(stream, iter.next(), "kicked")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerRemoveBanEvent {
    pub network_id: MaybeUtf8String,
    pub ip: MaybeUtf8String,
    pub by: MaybeUtf8String,
}
impl ServerRemoveBanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ServerRemoveBanEvent {
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            ip: read_value::<MaybeUtf8String>(stream, iter.next(), "ip")?,
            by: read_value::<MaybeUtf8String>(stream, iter.next(), "by")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerConnectEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub address: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerConnectEvent {
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
            index: read_value::<u8>(stream, iter.next(), "index")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            address: read_value::<MaybeUtf8String>(stream, iter.next(), "address")?,
            bot: read_value::<u16>(stream, iter.next(), "bot")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerConnectClientEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerConnectClientEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerConnectClientEvent {
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
            index: read_value::<u8>(stream, iter.next(), "index")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            bot: read_value::<u16>(stream, iter.next(), "bot")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInfoEvent {
    pub name: MaybeUtf8String,
    pub index: u8,
    pub user_id: u16,
    pub network_id: MaybeUtf8String,
    pub bot: bool,
}
impl PlayerInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerInfoEvent {
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
            index: read_value::<u8>(stream, iter.next(), "index")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            bot: read_value::<bool>(stream, iter.next(), "bot")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDisconnectEvent {
    pub user_id: u16,
    pub reason: MaybeUtf8String,
    pub name: MaybeUtf8String,
    pub network_id: MaybeUtf8String,
    pub bot: u16,
}
impl PlayerDisconnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDisconnectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            reason: read_value::<MaybeUtf8String>(stream, iter.next(), "reason")?,
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
            network_id: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "network_id")?,
            bot: read_value::<u16>(stream, iter.next(), "bot")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerActivateEvent {
    pub user_id: u16,
}
impl PlayerActivateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerActivateEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSayEvent {
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerSayEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerSayEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientDisconnectEvent {
    pub message: MaybeUtf8String,
}
impl ClientDisconnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ClientDisconnectEvent {
            message: read_value::<MaybeUtf8String>(stream, iter.next(), "message")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientBeginConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
    pub source: MaybeUtf8String,
}
impl ClientBeginConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ClientBeginConnectEvent {
            address: read_value::<MaybeUtf8String>(stream, iter.next(), "address")?,
            ip: read_value::<u32>(stream, iter.next(), "ip")?,
            port: read_value::<u16>(stream, iter.next(), "port")?,
            source: read_value::<MaybeUtf8String>(stream, iter.next(), "source")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientConnectedEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ClientConnectedEvent {
            address: read_value::<MaybeUtf8String>(stream, iter.next(), "address")?,
            ip: read_value::<u32>(stream, iter.next(), "ip")?,
            port: read_value::<u16>(stream, iter.next(), "port")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientFullConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientFullConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ClientFullConnectEvent {
            address: read_value::<MaybeUtf8String>(stream, iter.next(), "address")?,
            ip: read_value::<u32>(stream, iter.next(), "ip")?,
            port: read_value::<u16>(stream, iter.next(), "port")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HostQuitEvent {}
impl HostQuitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HostQuitEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamInfoEvent {
    pub team_id: u8,
    pub team_name: MaybeUtf8String,
}
impl TeamInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamInfoEvent {
            team_id: read_value::<u8>(stream, iter.next(), "team_id")?,
            team_name: read_value::<MaybeUtf8String>(stream, iter.next(), "team_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamScoreEvent {
    pub team_id: u8,
    pub score: u16,
}
impl TeamScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamScoreEvent {
            team_id: read_value::<u8>(stream, iter.next(), "team_id")?,
            score: read_value::<u16>(stream, iter.next(), "score")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayBroadcastAudioEvent {
    pub team: u8,
    pub sound: MaybeUtf8String,
    pub additional_flags: u16,
}
impl TeamPlayBroadcastAudioEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayBroadcastAudioEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            sound: read_value::<MaybeUtf8String>(stream, iter.next(), "sound")?,
            additional_flags: read_value::<u16>(stream, iter.next(), "additional_flags")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTeamEvent {
    pub user_id: u16,
    pub team: u8,
    pub old_team: u8,
    pub disconnect: bool,
    pub auto_team: bool,
    pub silent: bool,
    pub name: MaybeUtf8String,
}
impl PlayerTeamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerTeamEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
            old_team: read_value::<u8>(stream, iter.next(), "old_team")?,
            disconnect: read_value::<bool>(stream, iter.next(), "disconnect")?,
            auto_team: read_value::<bool>(stream, iter.next(), "auto_team")?,
            silent: read_value::<bool>(stream, iter.next(), "silent")?,
            name: read_value::<MaybeUtf8String>(stream, iter.next(), "name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerClassEvent {
    pub user_id: u16,
    pub class: MaybeUtf8String,
}
impl PlayerClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerClassEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            class: read_value::<MaybeUtf8String>(stream, iter.next(), "class")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDeathEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub player_penetrate_count: u16,
    pub assister_fallback: MaybeUtf8String,
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
impl PlayerDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDeathEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
            stun_flags: read_value::<u16>(stream, iter.next(), "stun_flags")?,
            death_flags: read_value::<u16>(stream, iter.next(), "death_flags")?,
            silent_kill: read_value::<bool>(stream, iter.next(), "silent_kill")?,
            player_penetrate_count: read_value::<
                u16,
            >(stream, iter.next(), "player_penetrate_count")?,
            assister_fallback: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "assister_fallback")?,
            kill_streak_total: read_value::<
                u16,
            >(stream, iter.next(), "kill_streak_total")?,
            kill_streak_wep: read_value::<u16>(stream, iter.next(), "kill_streak_wep")?,
            kill_streak_assist: read_value::<
                u16,
            >(stream, iter.next(), "kill_streak_assist")?,
            kill_streak_victim: read_value::<
                u16,
            >(stream, iter.next(), "kill_streak_victim")?,
            ducks_streaked: read_value::<u16>(stream, iter.next(), "ducks_streaked")?,
            duck_streak_total: read_value::<
                u16,
            >(stream, iter.next(), "duck_streak_total")?,
            duck_streak_assist: read_value::<
                u16,
            >(stream, iter.next(), "duck_streak_assist")?,
            duck_streak_victim: read_value::<
                u16,
            >(stream, iter.next(), "duck_streak_victim")?,
            rocket_jump: read_value::<bool>(stream, iter.next(), "rocket_jump")?,
            weapon_def_index: read_value::<
                u32,
            >(stream, iter.next(), "weapon_def_index")?,
            crit_type: read_value::<u16>(stream, iter.next(), "crit_type")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
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
impl PlayerHurtEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHurtEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            health: read_value::<u16>(stream, iter.next(), "health")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            damage_amount: read_value::<u16>(stream, iter.next(), "damage_amount")?,
            custom: read_value::<u16>(stream, iter.next(), "custom")?,
            show_disguised_crit: read_value::<
                bool,
            >(stream, iter.next(), "show_disguised_crit")?,
            crit: read_value::<bool>(stream, iter.next(), "crit")?,
            mini_crit: read_value::<bool>(stream, iter.next(), "mini_crit")?,
            all_see_crit: read_value::<bool>(stream, iter.next(), "all_see_crit")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            bonus_effect: read_value::<u8>(stream, iter.next(), "bonus_effect")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChatEvent {
    pub team_only: bool,
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerChatEvent {
            team_only: read_value::<bool>(stream, iter.next(), "team_only")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerScoreEvent {
    pub user_id: u16,
    pub kills: u16,
    pub deaths: u16,
    pub score: u16,
}
impl PlayerScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerScoreEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            kills: read_value::<u16>(stream, iter.next(), "kills")?,
            deaths: read_value::<u16>(stream, iter.next(), "deaths")?,
            score: read_value::<u16>(stream, iter.next(), "score")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSpawnEvent {
    pub user_id: u16,
    pub team: u16,
    pub class: u16,
}
impl PlayerSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerSpawnEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            team: read_value::<u16>(stream, iter.next(), "team")?,
            class: read_value::<u16>(stream, iter.next(), "class")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShootEvent {
    pub user_id: u16,
    pub weapon: u8,
    pub mode: u8,
}
impl PlayerShootEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerShootEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            weapon: read_value::<u8>(stream, iter.next(), "weapon")?,
            mode: read_value::<u8>(stream, iter.next(), "mode")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUseEvent {
    pub user_id: u16,
    pub entity: u16,
}
impl PlayerUseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerUseEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            entity: read_value::<u16>(stream, iter.next(), "entity")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeNameEvent {
    pub user_id: u16,
    pub old_name: MaybeUtf8String,
    pub new_name: MaybeUtf8String,
}
impl PlayerChangeNameEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerChangeNameEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            old_name: read_value::<MaybeUtf8String>(stream, iter.next(), "old_name")?,
            new_name: read_value::<MaybeUtf8String>(stream, iter.next(), "new_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHintMessageEvent {
    pub hint_message: MaybeUtf8String,
}
impl PlayerHintMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHintMessageEvent {
            hint_message: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "hint_message")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl BasePlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BasePlayerTeleportedEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameInitEvent {}
impl GameInitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameInitEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameNewMapEvent {
    pub map_name: MaybeUtf8String,
}
impl GameNewMapEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(GameNewMapEvent {
            map_name: read_value::<MaybeUtf8String>(stream, iter.next(), "map_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameStartEvent {
    pub rounds_limit: u32,
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl GameStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(GameStartEvent {
            rounds_limit: read_value::<u32>(stream, iter.next(), "rounds_limit")?,
            time_limit: read_value::<u32>(stream, iter.next(), "time_limit")?,
            frag_limit: read_value::<u32>(stream, iter.next(), "frag_limit")?,
            objective: read_value::<MaybeUtf8String>(stream, iter.next(), "objective")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameEndEvent {
    pub winner: u8,
}
impl GameEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(GameEndEvent {
            winner: read_value::<u8>(stream, iter.next(), "winner")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundStartEvent {
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl RoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RoundStartEvent {
            time_limit: read_value::<u32>(stream, iter.next(), "time_limit")?,
            frag_limit: read_value::<u32>(stream, iter.next(), "frag_limit")?,
            objective: read_value::<MaybeUtf8String>(stream, iter.next(), "objective")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundEndEvent {
    pub winner: u8,
    pub reason: u8,
    pub message: MaybeUtf8String,
}
impl RoundEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RoundEndEvent {
            winner: read_value::<u8>(stream, iter.next(), "winner")?,
            reason: read_value::<u8>(stream, iter.next(), "reason")?,
            message: read_value::<MaybeUtf8String>(stream, iter.next(), "message")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameMessageEvent {
    pub target: u8,
    pub text: MaybeUtf8String,
}
impl GameMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(GameMessageEvent {
            target: read_value::<u8>(stream, iter.next(), "target")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakBreakableEvent {
    pub ent_index: u32,
    pub user_id: u16,
    pub material: u8,
}
impl BreakBreakableEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BreakBreakableEvent {
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            material: read_value::<u8>(stream, iter.next(), "material")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakPropEvent {
    pub ent_index: u32,
    pub user_id: u16,
}
impl BreakPropEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BreakPropEvent {
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EntityKilledEvent {
    pub ent_index_killed: u32,
    pub ent_index_attacker: u32,
    pub ent_index_inflictor: u32,
    pub damage_bits: u32,
}
impl EntityKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EntityKilledEvent {
            ent_index_killed: read_value::<
                u32,
            >(stream, iter.next(), "ent_index_killed")?,
            ent_index_attacker: read_value::<
                u32,
            >(stream, iter.next(), "ent_index_attacker")?,
            ent_index_inflictor: read_value::<
                u32,
            >(stream, iter.next(), "ent_index_inflictor")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BonusUpdatedEvent {
    pub num_advanced: u16,
    pub num_bronze: u16,
    pub num_silver: u16,
    pub num_gold: u16,
}
impl BonusUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BonusUpdatedEvent {
            num_advanced: read_value::<u16>(stream, iter.next(), "num_advanced")?,
            num_bronze: read_value::<u16>(stream, iter.next(), "num_bronze")?,
            num_silver: read_value::<u16>(stream, iter.next(), "num_silver")?,
            num_gold: read_value::<u16>(stream, iter.next(), "num_gold")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEventEvent {
    pub achievement_name: MaybeUtf8String,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(AchievementEventEvent {
            achievement_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "achievement_name")?,
            cur_val: read_value::<u16>(stream, iter.next(), "cur_val")?,
            max_val: read_value::<u16>(stream, iter.next(), "max_val")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementIncrementEvent {
    pub achievement_id: u32,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementIncrementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(AchievementIncrementEvent {
            achievement_id: read_value::<u32>(stream, iter.next(), "achievement_id")?,
            cur_val: read_value::<u16>(stream, iter.next(), "cur_val")?,
            max_val: read_value::<u16>(stream, iter.next(), "max_val")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PhysgunPickupEvent {
    pub ent_index: u32,
}
impl PhysgunPickupEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PhysgunPickupEvent {
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FlareIgniteNpcEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(FlareIgniteNpcEvent {
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HelicopterGrenadePuntMissEvent {}
impl HelicopterGrenadePuntMissEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HelicopterGrenadePuntMissEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserDataDownloadedEvent {}
impl UserDataDownloadedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(UserDataDownloadedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RagdollDissolvedEvent {
    pub ent_index: u32,
}
impl RagdollDissolvedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RagdollDissolvedEvent {
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedModeEvent {
    pub old_mode: u16,
    pub new_mode: u16,
    pub obs_target: u16,
}
impl HLTVChangedModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVChangedModeEvent {
            old_mode: read_value::<u16>(stream, iter.next(), "old_mode")?,
            new_mode: read_value::<u16>(stream, iter.next(), "new_mode")?,
            obs_target: read_value::<u16>(stream, iter.next(), "obs_target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedTargetEvent {
    pub mode: u16,
    pub old_target: u16,
    pub obs_target: u16,
}
impl HLTVChangedTargetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVChangedTargetEvent {
            mode: read_value::<u16>(stream, iter.next(), "mode")?,
            old_target: read_value::<u16>(stream, iter.next(), "old_target")?,
            obs_target: read_value::<u16>(stream, iter.next(), "obs_target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteEndedEvent {}
impl VoteEndedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteEndedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteStartedEvent {
    pub issue: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
    pub initiator: u32,
}
impl VoteStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VoteStartedEvent {
            issue: read_value::<MaybeUtf8String>(stream, iter.next(), "issue")?,
            param_1: read_value::<MaybeUtf8String>(stream, iter.next(), "param_1")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
            initiator: read_value::<u32>(stream, iter.next(), "initiator")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteChangedEvent {
    pub vote_option_1: u8,
    pub vote_option_2: u8,
    pub vote_option_3: u8,
    pub vote_option_4: u8,
    pub vote_option_5: u8,
    pub potential_votes: u8,
}
impl VoteChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VoteChangedEvent {
            vote_option_1: read_value::<u8>(stream, iter.next(), "vote_option_1")?,
            vote_option_2: read_value::<u8>(stream, iter.next(), "vote_option_2")?,
            vote_option_3: read_value::<u8>(stream, iter.next(), "vote_option_3")?,
            vote_option_4: read_value::<u8>(stream, iter.next(), "vote_option_4")?,
            vote_option_5: read_value::<u8>(stream, iter.next(), "vote_option_5")?,
            potential_votes: read_value::<u8>(stream, iter.next(), "potential_votes")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VotePassedEvent {
    pub details: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
}
impl VotePassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VotePassedEvent {
            details: read_value::<MaybeUtf8String>(stream, iter.next(), "details")?,
            param_1: read_value::<MaybeUtf8String>(stream, iter.next(), "param_1")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteFailedEvent {
    pub team: u8,
}
impl VoteFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VoteFailedEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteCastEvent {
    pub vote_option: u8,
    pub team: u16,
    pub entity_id: u32,
}
impl VoteCastEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VoteCastEvent {
            vote_option: read_value::<u8>(stream, iter.next(), "vote_option")?,
            team: read_value::<u16>(stream, iter.next(), "team")?,
            entity_id: read_value::<u32>(stream, iter.next(), "entity_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteOptionsEvent {
    pub count: u8,
    pub option_1: MaybeUtf8String,
    pub option_2: MaybeUtf8String,
    pub option_3: MaybeUtf8String,
    pub option_4: MaybeUtf8String,
    pub option_5: MaybeUtf8String,
}
impl VoteOptionsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(VoteOptionsEvent {
            count: read_value::<u8>(stream, iter.next(), "count")?,
            option_1: read_value::<MaybeUtf8String>(stream, iter.next(), "option_1")?,
            option_2: read_value::<MaybeUtf8String>(stream, iter.next(), "option_2")?,
            option_3: read_value::<MaybeUtf8String>(stream, iter.next(), "option_3")?,
            option_4: read_value::<MaybeUtf8String>(stream, iter.next(), "option_4")?,
            option_5: read_value::<MaybeUtf8String>(stream, iter.next(), "option_5")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplaySavedEvent {}
impl ReplaySavedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplaySavedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnteredPerformanceModeEvent {}
impl EnteredPerformanceModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EnteredPerformanceModeEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BrowseReplaysEvent {}
impl BrowseReplaysEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BrowseReplaysEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayYoutubeStatsEvent {
    pub views: u32,
    pub likes: u32,
    pub favorited: u32,
}
impl ReplayYoutubeStatsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ReplayYoutubeStatsEvent {
            views: read_value::<u32>(stream, iter.next(), "views")?,
            likes: read_value::<u32>(stream, iter.next(), "likes")?,
            favorited: read_value::<u32>(stream, iter.next(), "favorited")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct InventoryUpdatedEvent {}
impl InventoryUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(InventoryUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CartUpdatedEvent {}
impl CartUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CartUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct StorePriceSheetUpdatedEvent {}
impl StorePriceSheetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StorePriceSheetUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EconInventoryConnectedEvent {}
impl EconInventoryConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EconInventoryConnectedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemSchemaInitializedEvent {}
impl ItemSchemaInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemSchemaInitializedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcNewSessionEvent {}
impl GcNewSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcNewSessionEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcLostSessionEvent {}
impl GcLostSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcLostSessionEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroFinishEvent {
    pub player: u16,
}
impl IntroFinishEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(IntroFinishEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl IntroNextCameraEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(IntroNextCameraEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeClassEvent {
    pub user_id: u16,
    pub class: u16,
}
impl PlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerChangeClassEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            class: read_value::<u16>(stream, iter.next(), "class")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl TfMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TfMapTimeRemainingEvent {
            seconds: read_value::<u32>(stream, iter.next(), "seconds")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TfGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TfGameOverEvent {
            reason: read_value::<MaybeUtf8String>(stream, iter.next(), "reason")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CtfFlagCapturedEvent {
    pub capping_team: u16,
    pub capping_team_score: u16,
}
impl CtfFlagCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CtfFlagCapturedEvent {
            capping_team: read_value::<u16>(stream, iter.next(), "capping_team")?,
            capping_team_score: read_value::<
                u16,
            >(stream, iter.next(), "capping_team_score")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointInitializedEvent {}
impl ControlPointInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointInitializedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl ControlPointUpdateImagesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointUpdateImagesEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl ControlPointUpdateLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointUpdateLayoutEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl ControlPointUpdateCappingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointUpdateCappingEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl ControlPointUpdateOwnerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointUpdateOwnerEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointStartTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointStartTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointStartTouchEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            area: read_value::<u16>(stream, iter.next(), "area")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointEndTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointEndTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointEndTouchEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            area: read_value::<u16>(stream, iter.next(), "area")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl ControlPointPulseElementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointPulseElementEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointFakeCaptureEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            int_data: read_value::<u16>(stream, iter.next(), "int_data")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureMultiplierEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureMultiplierEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointFakeCaptureMultiplierEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            int_data: read_value::<u16>(stream, iter.next(), "int_data")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: MaybeUtf8String,
}
impl TeamPlayRoundSelectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayRoundSelectedEvent {
            round: read_value::<MaybeUtf8String>(stream, iter.next(), "round")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl TeamPlayRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayRoundStartEvent {
            full_reset: read_value::<bool>(stream, iter.next(), "full_reset")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundActiveEvent {}
impl TeamPlayRoundActiveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundActiveEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingBeginsEvent {}
impl TeamPlayWaitingBeginsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingBeginsEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingEndsEvent {}
impl TeamPlayWaitingEndsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingEndsEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingAboutToEndEvent {}
impl TeamPlayWaitingAboutToEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingAboutToEndEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRestartRoundEvent {}
impl TeamPlayRestartRoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRestartRoundEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayReadyRestartEvent {}
impl TeamPlayReadyRestartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayReadyRestartEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl TeamPlayRoundRestartSecondsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds: read_value::<u16>(stream, iter.next(), "seconds")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl TeamPlayTeamReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayTeamReadyEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundWinEvent {
    pub team: u8,
    pub win_reason: u8,
    pub flag_cap_limit: u16,
    pub full_round: u16,
    pub round_time: f32,
    pub losing_team_num_caps: u16,
    pub was_sudden_death: u8,
}
impl TeamPlayRoundWinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayRoundWinEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            win_reason: read_value::<u8>(stream, iter.next(), "win_reason")?,
            flag_cap_limit: read_value::<u16>(stream, iter.next(), "flag_cap_limit")?,
            full_round: read_value::<u16>(stream, iter.next(), "full_round")?,
            round_time: read_value::<f32>(stream, iter.next(), "round_time")?,
            losing_team_num_caps: read_value::<
                u16,
            >(stream, iter.next(), "losing_team_num_caps")?,
            was_sudden_death: read_value::<u8>(stream, iter.next(), "was_sudden_death")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayUpdateTimerEvent {}
impl TeamPlayUpdateTimerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayUpdateTimerEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl TeamPlayRoundStalemateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayRoundStalemateEvent {
            reason: read_value::<u8>(stream, iter.next(), "reason")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeBeginEvent {}
impl TeamPlayOvertimeBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeBeginEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeEndEvent {}
impl TeamPlayOvertimeEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeEndEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathBeginEvent {}
impl TeamPlaySuddenDeathBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathBeginEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathEndEvent {}
impl TeamPlaySuddenDeathEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathEndEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TeamPlayGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayGameOverEvent {
            reason: read_value::<MaybeUtf8String>(stream, iter.next(), "reason")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl TeamPlayMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds: read_value::<u16>(stream, iter.next(), "seconds")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl TeamPlayTimerFlashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayTimerFlashEvent {
            time_remaining: read_value::<u16>(stream, iter.next(), "time_remaining")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerTimeAddedEvent {
    pub timer: u16,
    pub seconds_added: u16,
}
impl TeamPlayTimerTimeAddedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayTimerTimeAddedEvent {
            timer: read_value::<u16>(stream, iter.next(), "timer")?,
            seconds_added: read_value::<u16>(stream, iter.next(), "seconds_added")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointStartCaptureEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
    pub cap_team: u8,
    pub cappers: MaybeUtf8String,
    pub cap_time: f32,
}
impl TeamPlayPointStartCaptureEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayPointStartCaptureEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
            cap_team: read_value::<u8>(stream, iter.next(), "cap_team")?,
            cappers: read_value::<MaybeUtf8String>(stream, iter.next(), "cappers")?,
            cap_time: read_value::<f32>(stream, iter.next(), "cap_time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointCapturedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
    pub cappers: MaybeUtf8String,
}
impl TeamPlayPointCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayPointCapturedEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
            cappers: read_value::<MaybeUtf8String>(stream, iter.next(), "cappers")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointLockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointLockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayPointLockedEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointUnlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointUnlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayPointUnlockedEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBrokenEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub time_remaining: f32,
}
impl TeamPlayCaptureBrokenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayCaptureBrokenEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            time_remaining: read_value::<f32>(stream, iter.next(), "time_remaining")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub blocker: u8,
    pub victim: u8,
}
impl TeamPlayCaptureBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayCaptureBlockedEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(stream, iter.next(), "cp_name")?,
            blocker: read_value::<u8>(stream, iter.next(), "blocker")?,
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayFlagEventEvent {
    pub player: u16,
    pub carrier: u16,
    pub event_type: u16,
    pub home: u8,
    pub team: u8,
}
impl TeamPlayFlagEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayFlagEventEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            carrier: read_value::<u16>(stream, iter.next(), "carrier")?,
            event_type: read_value::<u16>(stream, iter.next(), "event_type")?,
            home: read_value::<u8>(stream, iter.next(), "home")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: MaybeUtf8String,
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
impl TeamPlayWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayWinPanelEvent {
            panel_style: read_value::<u8>(stream, iter.next(), "panel_style")?,
            winning_team: read_value::<u8>(stream, iter.next(), "winning_team")?,
            win_reason: read_value::<u8>(stream, iter.next(), "win_reason")?,
            cappers: read_value::<MaybeUtf8String>(stream, iter.next(), "cappers")?,
            flag_cap_limit: read_value::<u16>(stream, iter.next(), "flag_cap_limit")?,
            blue_score: read_value::<u16>(stream, iter.next(), "blue_score")?,
            red_score: read_value::<u16>(stream, iter.next(), "red_score")?,
            blue_score_prev: read_value::<u16>(stream, iter.next(), "blue_score_prev")?,
            red_score_prev: read_value::<u16>(stream, iter.next(), "red_score_prev")?,
            round_complete: read_value::<u16>(stream, iter.next(), "round_complete")?,
            rounds_remaining: read_value::<
                u16,
            >(stream, iter.next(), "rounds_remaining")?,
            player_1: read_value::<u16>(stream, iter.next(), "player_1")?,
            player_1_points: read_value::<u16>(stream, iter.next(), "player_1_points")?,
            player_2: read_value::<u16>(stream, iter.next(), "player_2")?,
            player_2_points: read_value::<u16>(stream, iter.next(), "player_2_points")?,
            player_3: read_value::<u16>(stream, iter.next(), "player_3")?,
            player_3_points: read_value::<u16>(stream, iter.next(), "player_3_points")?,
            kill_stream_player_1: read_value::<
                u16,
            >(stream, iter.next(), "kill_stream_player_1")?,
            kill_stream_player_1_count: read_value::<
                u16,
            >(stream, iter.next(), "kill_stream_player_1_count")?,
            game_over: read_value::<u8>(stream, iter.next(), "game_over")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamBalancedPlayerEvent {
    pub player: u16,
    pub team: u8,
}
impl TeamPlayTeamBalancedPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayTeamBalancedPlayerEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySetupFinishedEvent {}
impl TeamPlaySetupFinishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySetupFinishedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl TeamPlayAlertEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayAlertEvent {
            alert_type: read_value::<u16>(stream, iter.next(), "alert_type")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TrainingCompleteEvent {
    pub next_map: MaybeUtf8String,
    pub map: MaybeUtf8String,
    pub text: MaybeUtf8String,
}
impl TrainingCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TrainingCompleteEvent {
            next_map: read_value::<MaybeUtf8String>(stream, iter.next(), "next_map")?,
            map: read_value::<MaybeUtf8String>(stream, iter.next(), "map")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl ShowFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ShowFreezePanelEvent {
            killer: read_value::<u16>(stream, iter.next(), "killer")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideFreezePanelEvent {}
impl HideFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HideFreezePanelEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FreezeCamStartedEvent {}
impl FreezeCamStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FreezeCamStartedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeTeamEvent {}
impl LocalPlayerChangeTeamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeTeamEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl LocalPlayerScoreChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(LocalPlayerScoreChangedEvent {
            score: read_value::<u16>(stream, iter.next(), "score")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeClassEvent {}
impl LocalPlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeClassEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerRespawnEvent {}
impl LocalPlayerRespawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerRespawnEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildingInfoChangedEvent {
    pub building_type: u8,
    pub object_mode: u8,
    pub remove: u8,
}
impl BuildingInfoChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BuildingInfoChangedEvent {
            building_type: read_value::<u8>(stream, iter.next(), "building_type")?,
            object_mode: read_value::<u8>(stream, iter.next(), "object_mode")?,
            remove: read_value::<u8>(stream, iter.next(), "remove")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl LocalPlayerChangeDisguiseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised: read_value::<bool>(stream, iter.next(), "disguised")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAccountChangedEvent {
    pub old_value: u16,
    pub new_value: u16,
}
impl PlayerAccountChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerAccountChangedEvent {
            old_value: read_value::<u16>(stream, iter.next(), "old_value")?,
            new_value: read_value::<u16>(stream, iter.next(), "new_value")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpyPdaResetEvent {}
impl SpyPdaResetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpyPdaResetEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlagStatusUpdateEvent {
    pub user_id: u16,
    pub ent_index: u32,
}
impl FlagStatusUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(FlagStatusUpdateEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            ent_index: read_value::<u32>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl PlayerStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerStatsUpdatedEvent {
            force_upload: read_value::<bool>(stream, iter.next(), "force_upload")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayingCommentaryEvent {}
impl PlayingCommentaryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayingCommentaryEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChargeDeployedEvent {
    pub user_id: u16,
    pub target_id: u16,
}
impl PlayerChargeDeployedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerChargeDeployedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            target_id: read_value::<u16>(stream, iter.next(), "target_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuiltObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerBuiltObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerBuiltObjectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object: read_value::<u16>(stream, iter.next(), "object")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUpgradedObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
    pub is_builder: bool,
}
impl PlayerUpgradedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerUpgradedObjectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object: read_value::<u16>(stream, iter.next(), "object")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
            is_builder: read_value::<bool>(stream, iter.next(), "is_builder")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCarryObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerCarryObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerCarryObjectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object: read_value::<u16>(stream, iter.next(), "object")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDropObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerDropObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDropObjectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object: read_value::<u16>(stream, iter.next(), "object")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectRemovedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectRemovedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ObjectRemovedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object_type: read_value::<u16>(stream, iter.next(), "object_type")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDestroyedEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub assister: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub object_type: u16,
    pub index: u16,
    pub was_building: bool,
}
impl ObjectDestroyedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ObjectDestroyedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            object_type: read_value::<u16>(stream, iter.next(), "object_type")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
            was_building: read_value::<bool>(stream, iter.next(), "was_building")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDetonatedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectDetonatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ObjectDetonatedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            object_type: read_value::<u16>(stream, iter.next(), "object_type")?,
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEarnedEvent {
    pub player: u8,
    pub achievement: u16,
}
impl AchievementEarnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(AchievementEarnedEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            achievement: read_value::<u16>(stream, iter.next(), "achievement")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecTargetUpdatedEvent {}
impl SpecTargetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpecTargetUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentStateUpdateEvent {
    pub user_id: u16,
    pub name_change: bool,
    pub ready_state: u16,
    pub new_name: MaybeUtf8String,
}
impl TournamentStateUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TournamentStateUpdateEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            name_change: read_value::<bool>(stream, iter.next(), "name_change")?,
            ready_state: read_value::<u16>(stream, iter.next(), "ready_state")?,
            new_name: read_value::<MaybeUtf8String>(stream, iter.next(), "new_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentEnableCountdownEvent {}
impl TournamentEnableCountdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TournamentEnableCountdownEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCalledForMedicEvent {
    pub user_id: u16,
}
impl PlayerCalledForMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerCalledForMedicEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAskedForBallEvent {
    pub user_id: u16,
}
impl PlayerAskedForBallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerAskedForBallEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerBecameObserverEvent {}
impl LocalPlayerBecameObserverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerBecameObserverEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedInvEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub medic_ent_index: u8,
}
impl PlayerIgnitedInvEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index: read_value::<u8>(stream, iter.next(), "pyro_ent_index")?,
            victim_ent_index: read_value::<u8>(stream, iter.next(), "victim_ent_index")?,
            medic_ent_index: read_value::<u8>(stream, iter.next(), "medic_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub weapon_id: u8,
}
impl PlayerIgnitedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerIgnitedEvent {
            pyro_ent_index: read_value::<u8>(stream, iter.next(), "pyro_ent_index")?,
            victim_ent_index: read_value::<u8>(stream, iter.next(), "victim_ent_index")?,
            weapon_id: read_value::<u8>(stream, iter.next(), "weapon_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
    pub item_definition_index: u16,
}
impl PlayerExtinguishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerExtinguishedEvent {
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
            healer: read_value::<u8>(stream, iter.next(), "healer")?,
            item_definition_index: read_value::<
                u16,
            >(stream, iter.next(), "item_definition_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTeleportedEvent {
    pub user_id: u16,
    pub builder_id: u16,
    pub dist: f32,
}
impl PlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerTeleportedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            builder_id: read_value::<u16>(stream, iter.next(), "builder_id")?,
            dist: read_value::<f32>(stream, iter.next(), "dist")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedMedicCallEvent {
    pub user_id: u16,
}
impl PlayerHealedMedicCallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHealedMedicCallEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChargeReadyEvent {}
impl LocalPlayerChargeReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChargeReadyEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerWindDownEvent {}
impl LocalPlayerWindDownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerWindDownEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInvulnedEvent {
    pub user_id: u16,
    pub medic_user_id: u16,
}
impl PlayerInvulnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerInvulnedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            medic_user_id: read_value::<u16>(stream, iter.next(), "medic_user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortSpeedEvent {
    pub team: u8,
    pub speed: u8,
    pub players: u8,
}
impl EscortSpeedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EscortSpeedEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            speed: read_value::<u8>(stream, iter.next(), "speed")?,
            players: read_value::<u8>(stream, iter.next(), "players")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortProgressEvent {
    pub team: u8,
    pub progress: f32,
    pub reset: bool,
}
impl EscortProgressEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EscortProgressEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            progress: read_value::<f32>(stream, iter.next(), "progress")?,
            reset: read_value::<bool>(stream, iter.next(), "reset")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortRecedeEvent {
    pub team: u8,
    pub recede_time: f32,
}
impl EscortRecedeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EscortRecedeEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            recede_time: read_value::<f32>(stream, iter.next(), "recede_time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIActivatedEvent {}
impl GameUIActivatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIActivatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIHiddenEvent {}
impl GameUIHiddenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIHiddenEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl PlayerEscortScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerEscortScoreEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            points: read_value::<u8>(stream, iter.next(), "points")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub ent_index: u8,
    pub weapon_def_index: u32,
}
impl PlayerHealOnHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHealOnHitEvent {
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
            ent_index: read_value::<u8>(stream, iter.next(), "ent_index")?,
            weapon_def_index: read_value::<u32>(stream, iter.next(), "weapon_def_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStealSandvichEvent {
    pub owner: u16,
    pub target: u16,
}
impl PlayerStealSandvichEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerStealSandvichEvent {
            owner: read_value::<u16>(stream, iter.next(), "owner")?,
            target: read_value::<u16>(stream, iter.next(), "target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl ShowClassLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ShowClassLayoutEvent {
            show: read_value::<bool>(stream, iter.next(), "show")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl ShowVsPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ShowVsPanelEvent {
            show: read_value::<bool>(stream, iter.next(), "show")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamagedEvent {
    pub amount: u16,
    pub kind: u32,
}
impl PlayerDamagedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDamagedEvent {
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
            kind: read_value::<u32>(stream, iter.next(), "kind")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaPlayerNotificationEvent {
    pub player: u8,
    pub message: u8,
}
impl ArenaPlayerNotificationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ArenaPlayerNotificationEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            message: read_value::<u8>(stream, iter.next(), "message")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaMatchMaxStreakEvent {
    pub team: u8,
    pub streak: u8,
}
impl ArenaMatchMaxStreakEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ArenaMatchMaxStreakEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            streak: read_value::<u8>(stream, iter.next(), "streak")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaRoundStartEvent {}
impl ArenaRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaRoundStartEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
    pub cappers: MaybeUtf8String,
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
impl ArenaWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ArenaWinPanelEvent {
            panel_style: read_value::<u8>(stream, iter.next(), "panel_style")?,
            winning_team: read_value::<u8>(stream, iter.next(), "winning_team")?,
            win_reason: read_value::<u8>(stream, iter.next(), "win_reason")?,
            cappers: read_value::<MaybeUtf8String>(stream, iter.next(), "cappers")?,
            flag_cap_limit: read_value::<u16>(stream, iter.next(), "flag_cap_limit")?,
            blue_score: read_value::<u16>(stream, iter.next(), "blue_score")?,
            red_score: read_value::<u16>(stream, iter.next(), "red_score")?,
            blue_score_prev: read_value::<u16>(stream, iter.next(), "blue_score_prev")?,
            red_score_prev: read_value::<u16>(stream, iter.next(), "red_score_prev")?,
            round_complete: read_value::<u16>(stream, iter.next(), "round_complete")?,
            player_1: read_value::<u16>(stream, iter.next(), "player_1")?,
            player_1_damage: read_value::<u16>(stream, iter.next(), "player_1_damage")?,
            player_1_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_1_healing")?,
            player_1_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_1_lifetime")?,
            player_1_kills: read_value::<u16>(stream, iter.next(), "player_1_kills")?,
            player_2: read_value::<u16>(stream, iter.next(), "player_2")?,
            player_2_damage: read_value::<u16>(stream, iter.next(), "player_2_damage")?,
            player_2_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_2_healing")?,
            player_2_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_2_lifetime")?,
            player_2_kills: read_value::<u16>(stream, iter.next(), "player_2_kills")?,
            player_3: read_value::<u16>(stream, iter.next(), "player_3")?,
            player_3_damage: read_value::<u16>(stream, iter.next(), "player_3_damage")?,
            player_3_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_3_healing")?,
            player_3_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_3_lifetime")?,
            player_3_kills: read_value::<u16>(stream, iter.next(), "player_3_kills")?,
            player_4: read_value::<u16>(stream, iter.next(), "player_4")?,
            player_4_damage: read_value::<u16>(stream, iter.next(), "player_4_damage")?,
            player_4_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_4_healing")?,
            player_4_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_4_lifetime")?,
            player_4_kills: read_value::<u16>(stream, iter.next(), "player_4_kills")?,
            player_5: read_value::<u16>(stream, iter.next(), "player_5")?,
            player_5_damage: read_value::<u16>(stream, iter.next(), "player_5_damage")?,
            player_5_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_5_healing")?,
            player_5_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_5_lifetime")?,
            player_5_kills: read_value::<u16>(stream, iter.next(), "player_5_kills")?,
            player_6: read_value::<u16>(stream, iter.next(), "player_6")?,
            player_6_damage: read_value::<u16>(stream, iter.next(), "player_6_damage")?,
            player_6_healing: read_value::<
                u16,
            >(stream, iter.next(), "player_6_healing")?,
            player_6_lifetime: read_value::<
                u16,
            >(stream, iter.next(), "player_6_lifetime")?,
            player_6_kills: read_value::<u16>(stream, iter.next(), "player_6_kills")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PveWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
}
impl PveWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PveWinPanelEvent {
            panel_style: read_value::<u8>(stream, iter.next(), "panel_style")?,
            winning_team: read_value::<u8>(stream, iter.next(), "winning_team")?,
            win_reason: read_value::<u8>(stream, iter.next(), "win_reason")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct AirDashEvent {
    pub player: u8,
}
impl AirDashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(AirDashEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LandedEvent {
    pub player: u8,
}
impl LandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(LandedEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl PlayerDamageDodgedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDamageDodgedEvent {
            damage: read_value::<u16>(stream, iter.next(), "damage")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStunnedEvent {
    pub stunner: u16,
    pub victim: u16,
    pub victim_capping: bool,
    pub big_stun: bool,
}
impl PlayerStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerStunnedEvent {
            stunner: read_value::<u16>(stream, iter.next(), "stunner")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
            victim_capping: read_value::<bool>(stream, iter.next(), "victim_capping")?,
            big_stun: read_value::<bool>(stream, iter.next(), "big_stun")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutGrandSlamEvent {
    pub scout_id: u16,
    pub target_id: u16,
}
impl ScoutGrandSlamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ScoutGrandSlamEvent {
            scout_id: read_value::<u16>(stream, iter.next(), "scout_id")?,
            target_id: read_value::<u16>(stream, iter.next(), "target_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutSlamdollLandedEvent {
    pub target_index: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ScoutSlamdollLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ScoutSlamdollLandedEvent {
            target_index: read_value::<u16>(stream, iter.next(), "target_index")?,
            x: read_value::<f32>(stream, iter.next(), "x")?,
            y: read_value::<f32>(stream, iter.next(), "y")?,
            z: read_value::<f32>(stream, iter.next(), "z")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
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
impl ArrowImpactEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ArrowImpactEvent {
            attached_entity: read_value::<u16>(stream, iter.next(), "attached_entity")?,
            shooter: read_value::<u16>(stream, iter.next(), "shooter")?,
            bone_index_attached: read_value::<
                u16,
            >(stream, iter.next(), "bone_index_attached")?,
            bone_position_x: read_value::<f32>(stream, iter.next(), "bone_position_x")?,
            bone_position_y: read_value::<f32>(stream, iter.next(), "bone_position_y")?,
            bone_position_z: read_value::<f32>(stream, iter.next(), "bone_position_z")?,
            bone_angles_x: read_value::<f32>(stream, iter.next(), "bone_angles_x")?,
            bone_angles_y: read_value::<f32>(stream, iter.next(), "bone_angles_y")?,
            bone_angles_z: read_value::<f32>(stream, iter.next(), "bone_angles_z")?,
            projectile_type: read_value::<u16>(stream, iter.next(), "projectile_type")?,
            is_crit: read_value::<bool>(stream, iter.next(), "is_crit")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerJaratedEvent {
            thrower_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "thrower_ent_index")?,
            victim_ent_index: read_value::<u8>(stream, iter.next(), "victim_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedFadeEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedFadeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "thrower_ent_index")?,
            victim_ent_index: read_value::<u8>(stream, iter.next(), "victim_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShieldBlockedEvent {
    pub attacker_ent_index: u8,
    pub blocker_ent_index: u8,
}
impl PlayerShieldBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "attacker_ent_index")?,
            blocker_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "blocker_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl PlayerPinnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerPinnedEvent {
            pinned: read_value::<u8>(stream, iter.next(), "pinned")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl PlayerHealedByMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHealedByMedicEvent {
            medic: read_value::<u8>(stream, iter.next(), "medic")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSappedObjectEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub object: u8,
    pub sapper_id: u16,
}
impl PlayerSappedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerSappedObjectEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            owner_id: read_value::<u16>(stream, iter.next(), "owner_id")?,
            object: read_value::<u8>(stream, iter.next(), "object")?,
            sapper_id: read_value::<u16>(stream, iter.next(), "sapper_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemFoundEvent {
    pub player: u8,
    pub quality: u8,
    pub method: u8,
    pub item_def: u32,
    pub is_strange: u8,
    pub is_unusual: u8,
    pub wear: f32,
}
impl ItemFoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ItemFoundEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            quality: read_value::<u8>(stream, iter.next(), "quality")?,
            method: read_value::<u8>(stream, iter.next(), "method")?,
            item_def: read_value::<u32>(stream, iter.next(), "item_def")?,
            is_strange: read_value::<u8>(stream, iter.next(), "is_strange")?,
            is_unusual: read_value::<u8>(stream, iter.next(), "is_unusual")?,
            wear: read_value::<f32>(stream, iter.next(), "wear")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowAnnotationEvent {
    pub world_pos_x: f32,
    pub world_pos_y: f32,
    pub world_pos_z: f32,
    pub world_normal_x: f32,
    pub world_normal_y: f32,
    pub world_normal_z: f32,
    pub id: u32,
    pub text: MaybeUtf8String,
    pub lifetime: f32,
    pub visibility_bit_field: u32,
    pub follow_ent_index: u32,
    pub show_distance: bool,
    pub play_sound: MaybeUtf8String,
    pub show_effect: bool,
}
impl ShowAnnotationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ShowAnnotationEvent {
            world_pos_x: read_value::<f32>(stream, iter.next(), "world_pos_x")?,
            world_pos_y: read_value::<f32>(stream, iter.next(), "world_pos_y")?,
            world_pos_z: read_value::<f32>(stream, iter.next(), "world_pos_z")?,
            world_normal_x: read_value::<f32>(stream, iter.next(), "world_normal_x")?,
            world_normal_y: read_value::<f32>(stream, iter.next(), "world_normal_y")?,
            world_normal_z: read_value::<f32>(stream, iter.next(), "world_normal_z")?,
            id: read_value::<u32>(stream, iter.next(), "id")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
            lifetime: read_value::<f32>(stream, iter.next(), "lifetime")?,
            visibility_bit_field: read_value::<
                u32,
            >(stream, iter.next(), "visibility_bit_field")?,
            follow_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "follow_ent_index")?,
            show_distance: read_value::<bool>(stream, iter.next(), "show_distance")?,
            play_sound: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "play_sound")?,
            show_effect: read_value::<bool>(stream, iter.next(), "show_effect")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideAnnotationEvent {
    pub id: u32,
}
impl HideAnnotationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HideAnnotationEvent {
            id: read_value::<u32>(stream, iter.next(), "id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl PostInventoryApplicationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PostInventoryApplicationEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUnlockUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl ControlPointUnlockUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointUnlockUpdatedEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
            time: read_value::<f32>(stream, iter.next(), "time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeployBuffBannerEvent {
    pub buff_type: u8,
    pub buff_owner: u16,
}
impl DeployBuffBannerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DeployBuffBannerEvent {
            buff_type: read_value::<u8>(stream, iter.next(), "buff_type")?,
            buff_owner: read_value::<u16>(stream, iter.next(), "buff_owner")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuffEvent {
    pub user_id: u16,
    pub buff_owner: u16,
    pub buff_type: u8,
}
impl PlayerBuffEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerBuffEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            buff_owner: read_value::<u16>(stream, iter.next(), "buff_owner")?,
            buff_type: read_value::<u8>(stream, iter.next(), "buff_type")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDeathEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub healing: u16,
    pub charged: bool,
}
impl MedicDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MedicDeathEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            healing: read_value::<u16>(stream, iter.next(), "healing")?,
            charged: read_value::<bool>(stream, iter.next(), "charged")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct OvertimeNagEvent {}
impl OvertimeNagEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(OvertimeNagEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamsChangedEvent {}
impl TeamsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamsChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenPumpkinGrabEvent {
    pub user_id: u16,
}
impl HalloweenPumpkinGrabEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HalloweenPumpkinGrabEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RocketJumpEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            play_sound: read_value::<bool>(stream, iter.next(), "play_sound")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpLandedEvent {
    pub user_id: u16,
}
impl RocketJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RocketJumpLandedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl StickyJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(StickyJumpEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            play_sound: read_value::<bool>(stream, iter.next(), "play_sound")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpLandedEvent {
    pub user_id: u16,
}
impl StickyJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(StickyJumpLandedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLaunchEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketPackLaunchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RocketPackLaunchEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            play_sound: read_value::<bool>(stream, iter.next(), "play_sound")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLandedEvent {
    pub user_id: u16,
}
impl RocketPackLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RocketPackLandedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDefendedEvent {
    pub user_id: u16,
    pub medic: u16,
}
impl MedicDefendedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MedicDefendedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            medic: read_value::<u16>(stream, iter.next(), "medic")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl LocalPlayerHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(LocalPlayerHealedEvent {
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl PlayerDestroyedPipeBombEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDestroyedPipeBombEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDeflectedEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub weapon_id: u16,
    pub object_ent_index: u16,
}
impl ObjectDeflectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ObjectDeflectedEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            owner_id: read_value::<u16>(stream, iter.next(), "owner_id")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            object_ent_index: read_value::<u16>(stream, iter.next(), "object_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerMvpEvent {
    pub player: u16,
}
impl PlayerMvpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerMvpEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnMobEvent {}
impl RaidSpawnMobEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnMobEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnSquadEvent {}
impl RaidSpawnSquadEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnSquadEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl NavBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(NavBlockedEvent {
            area: read_value::<u32>(stream, iter.next(), "area")?,
            blocked: read_value::<bool>(stream, iter.next(), "blocked")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl PathTrackPassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PathTrackPassedEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct NumCappersChangedEvent {
    pub index: u16,
    pub count: u8,
}
impl NumCappersChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(NumCappersChangedEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
            count: read_value::<u8>(stream, iter.next(), "count")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRegenerateEvent {}
impl PlayerRegenerateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerRegenerateEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpdateStatusItemEvent {
    pub index: u8,
    pub object: u8,
}
impl UpdateStatusItemEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(UpdateStatusItemEvent {
            index: read_value::<u8>(stream, iter.next(), "index")?,
            object: read_value::<u8>(stream, iter.next(), "object")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct StatsResetRoundEvent {}
impl StatsResetRoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StatsResetRoundEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoreStatsAccumulatedUpdateEvent {}
impl ScoreStatsAccumulatedUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoreStatsAccumulatedUpdateEvent {
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoreStatsAccumulatedResetEvent {}
impl ScoreStatsAccumulatedResetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoreStatsAccumulatedResetEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEarnedLocalEvent {
    pub achievement: u16,
}
impl AchievementEarnedLocalEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(AchievementEarnedLocalEvent {
            achievement: read_value::<u16>(stream, iter.next(), "achievement")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedEvent {
    pub patient: u16,
    pub healer: u16,
    pub amount: u16,
}
impl PlayerHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHealedEvent {
            patient: read_value::<u16>(stream, iter.next(), "patient")?,
            healer: read_value::<u16>(stream, iter.next(), "healer")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildingHealedEvent {
    pub building: u16,
    pub healer: u16,
    pub amount: u16,
}
impl BuildingHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(BuildingHealedEvent {
            building: read_value::<u16>(stream, iter.next(), "building")?,
            healer: read_value::<u16>(stream, iter.next(), "healer")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemPickupEvent {
    pub user_id: u16,
    pub item: MaybeUtf8String,
}
impl ItemPickupEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ItemPickupEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            item: read_value::<MaybeUtf8String>(stream, iter.next(), "item")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DuelStatusEvent {
    pub killer: u16,
    pub score_type: u16,
    pub initiator: u16,
    pub target: u16,
    pub initiator_score: u16,
    pub target_score: u16,
}
impl DuelStatusEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DuelStatusEvent {
            killer: read_value::<u16>(stream, iter.next(), "killer")?,
            score_type: read_value::<u16>(stream, iter.next(), "score_type")?,
            initiator: read_value::<u16>(stream, iter.next(), "initiator")?,
            target: read_value::<u16>(stream, iter.next(), "target")?,
            initiator_score: read_value::<u16>(stream, iter.next(), "initiator_score")?,
            target_score: read_value::<u16>(stream, iter.next(), "target_score")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FishNoticeEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: MaybeUtf8String,
}
impl FishNoticeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(FishNoticeEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
            stun_flags: read_value::<u16>(stream, iter.next(), "stun_flags")?,
            death_flags: read_value::<u16>(stream, iter.next(), "death_flags")?,
            silent_kill: read_value::<bool>(stream, iter.next(), "silent_kill")?,
            assister_fallback: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "assister_fallback")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FishNoticeArmEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: MaybeUtf8String,
}
impl FishNoticeArmEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(FishNoticeArmEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
            stun_flags: read_value::<u16>(stream, iter.next(), "stun_flags")?,
            death_flags: read_value::<u16>(stream, iter.next(), "death_flags")?,
            silent_kill: read_value::<bool>(stream, iter.next(), "silent_kill")?,
            assister_fallback: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "assister_fallback")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SlapNoticeEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: MaybeUtf8String,
}
impl SlapNoticeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(SlapNoticeEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
            stun_flags: read_value::<u16>(stream, iter.next(), "stun_flags")?,
            death_flags: read_value::<u16>(stream, iter.next(), "death_flags")?,
            silent_kill: read_value::<bool>(stream, iter.next(), "silent_kill")?,
            assister_fallback: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "assister_fallback")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ThrowableHitEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub assister: u16,
    pub weapon_log_class_name: MaybeUtf8String,
    pub stun_flags: u16,
    pub death_flags: u16,
    pub silent_kill: bool,
    pub assister_fallback: MaybeUtf8String,
    pub total_hits: u16,
}
impl ThrowableHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ThrowableHitEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
            stun_flags: read_value::<u16>(stream, iter.next(), "stun_flags")?,
            death_flags: read_value::<u16>(stream, iter.next(), "death_flags")?,
            silent_kill: read_value::<bool>(stream, iter.next(), "silent_kill")?,
            assister_fallback: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "assister_fallback")?,
            total_hits: read_value::<u16>(stream, iter.next(), "total_hits")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PumpkinLordSummonedEvent {}
impl PumpkinLordSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PumpkinLordSummonedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PumpkinLordKilledEvent {}
impl PumpkinLordKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PumpkinLordKilledEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusSummonedEvent {
    pub level: u16,
}
impl MerasmusSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusSummonedEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl MerasmusKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusKilledEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusEscapeWarningEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl MerasmusEscapeWarningEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusEscapeWarningEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
            time_remaining: read_value::<u8>(stream, iter.next(), "time_remaining")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusEscapedEvent {
    pub level: u16,
}
impl MerasmusEscapedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusEscapedEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl EyeballBossSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossSummonedEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossStunnedEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl EyeballBossStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossStunnedEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
            player_ent_index: read_value::<u8>(stream, iter.next(), "player_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossKilledEvent {
    pub level: u16,
}
impl EyeballBossKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossKilledEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossKillerEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl EyeballBossKillerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossKillerEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
            player_ent_index: read_value::<u8>(stream, iter.next(), "player_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossEscapeImminentEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl EyeballBossEscapeImminentEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossEscapeImminentEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
            time_remaining: read_value::<u8>(stream, iter.next(), "time_remaining")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossEscapedEvent {
    pub level: u16,
}
impl EyeballBossEscapedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EyeballBossEscapedEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct NpcHurtEvent {
    pub ent_index: u16,
    pub health: u16,
    pub attacker_player: u16,
    pub weapon_id: u16,
    pub damage_amount: u16,
    pub crit: bool,
    pub boss: u16,
}
impl NpcHurtEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(NpcHurtEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
            health: read_value::<u16>(stream, iter.next(), "health")?,
            attacker_player: read_value::<u16>(stream, iter.next(), "attacker_player")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_amount: read_value::<u16>(stream, iter.next(), "damage_amount")?,
            crit: read_value::<bool>(stream, iter.next(), "crit")?,
            boss: read_value::<u16>(stream, iter.next(), "boss")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointTimerUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl ControlPointTimerUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ControlPointTimerUpdatedEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
            time: read_value::<f32>(stream, iter.next(), "time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveStartEvent {
    pub ent_index: u8,
}
impl PlayerHighFiveStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHighFiveStartEvent {
            ent_index: read_value::<u8>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveCancelEvent {
    pub ent_index: u8,
}
impl PlayerHighFiveCancelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHighFiveCancelEvent {
            ent_index: read_value::<u8>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveSuccessEvent {
    pub initiator_ent_index: u8,
    pub partner_ent_index: u8,
}
impl PlayerHighFiveSuccessEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerHighFiveSuccessEvent {
            initiator_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "initiator_ent_index")?,
            partner_ent_index: read_value::<
                u8,
            >(stream, iter.next(), "partner_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBonusPointsEvent {
    pub points: u16,
    pub player_ent_index: u16,
    pub source_ent_index: u16,
}
impl PlayerBonusPointsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerBonusPointsEvent {
            points: read_value::<u16>(stream, iter.next(), "points")?,
            player_ent_index: read_value::<
                u16,
            >(stream, iter.next(), "player_ent_index")?,
            source_ent_index: read_value::<u16>(stream, iter.next(), "source_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUpgradedEvent {}
impl PlayerUpgradedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUpgradedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuybackEvent {
    pub player: u16,
    pub cost: u16,
}
impl PlayerBuybackEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerBuybackEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            cost: read_value::<u16>(stream, iter.next(), "cost")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUsedPowerUpBottleEvent {
    pub player: u16,
    pub kind: u16,
    pub time: f32,
}
impl PlayerUsedPowerUpBottleEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerUsedPowerUpBottleEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            kind: read_value::<u16>(stream, iter.next(), "kind")?,
            time: read_value::<f32>(stream, iter.next(), "time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChristmasGiftGrabEvent {
    pub user_id: u16,
}
impl ChristmasGiftGrabEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ChristmasGiftGrabEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerKilledAchievementZoneEvent {
    pub attacker: u16,
    pub victim: u16,
    pub zone_id: u16,
}
impl PlayerKilledAchievementZoneEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerKilledAchievementZoneEvent {
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
            zone_id: read_value::<u16>(stream, iter.next(), "zone_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyUpdatedEvent {}
impl PartyUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyPrefChangedEvent {}
impl PartyPrefChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyPrefChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyCriteriaChangedEvent {}
impl PartyCriteriaChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyCriteriaChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyInvitesChangedEvent {}
impl PartyInvitesChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyInvitesChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyQueueStateChangedEvent {
    pub match_group: u16,
}
impl PartyQueueStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PartyQueueStateChangedEvent {
            match_group: read_value::<u16>(stream, iter.next(), "match_group")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyChatEvent {
    pub steam_id: MaybeUtf8String,
    pub text: MaybeUtf8String,
    pub kind: u16,
}
impl PartyChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PartyChatEvent {
            steam_id: read_value::<MaybeUtf8String>(stream, iter.next(), "steam_id")?,
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
            kind: read_value::<u16>(stream, iter.next(), "kind")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyMemberJoinEvent {
    pub steam_id: MaybeUtf8String,
}
impl PartyMemberJoinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PartyMemberJoinEvent {
            steam_id: read_value::<MaybeUtf8String>(stream, iter.next(), "steam_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyMemberLeaveEvent {
    pub steam_id: MaybeUtf8String,
}
impl PartyMemberLeaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PartyMemberLeaveEvent {
            steam_id: read_value::<MaybeUtf8String>(stream, iter.next(), "steam_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MatchInvitesUpdatedEvent {}
impl MatchInvitesUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MatchInvitesUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LobbyUpdatedEvent {}
impl LobbyUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LobbyUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMissionUpdateEvent {
    pub class: u16,
    pub count: u16,
}
impl MvmMissionUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmMissionUpdateEvent {
            class: read_value::<u16>(stream, iter.next(), "class")?,
            count: read_value::<u16>(stream, iter.next(), "count")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecalculateHolidaysEvent {}
impl RecalculateHolidaysEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RecalculateHolidaysEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCurrencyChangedEvent {
    pub currency: u16,
}
impl PlayerCurrencyChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerCurrencyChangedEvent {
            currency: read_value::<u16>(stream, iter.next(), "currency")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl DoomsdayRocketOpenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DoomsdayRocketOpenEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl RemoveNemesisRelationshipsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RemoveNemesisRelationshipsEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusWaveEvent {}
impl MvmCreditBonusWaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusWaveEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusAllEvent {}
impl MvmCreditBonusAllEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusAllEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusAllAdvancedEvent {}
impl MvmCreditBonusAllAdvancedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusAllAdvancedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmQuickSentryUpgradeEvent {
    pub player: u16,
}
impl MvmQuickSentryUpgradeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmQuickSentryUpgradeEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmTankDestroyedByPlayersEvent {}
impl MvmTankDestroyedByPlayersEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmTankDestroyedByPlayersEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmKillRobotDeliveringBombEvent {
    pub player: u16,
}
impl MvmKillRobotDeliveringBombEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmKillRobotDeliveringBombEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmPickupCurrencyEvent {
    pub player: u16,
    pub currency: u16,
}
impl MvmPickupCurrencyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmPickupCurrencyEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            currency: read_value::<u16>(stream, iter.next(), "currency")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl MvmBombCarrierKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmBombCarrierKilledEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSentryBusterDetonateEvent {
    pub player: u16,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}
impl MvmSentryBusterDetonateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmSentryBusterDetonateEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            det_x: read_value::<f32>(stream, iter.next(), "det_x")?,
            det_y: read_value::<f32>(stream, iter.next(), "det_y")?,
            det_z: read_value::<f32>(stream, iter.next(), "det_z")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmScoutMarkedForDeathEvent {
    pub player: u16,
}
impl MvmScoutMarkedForDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmScoutMarkedForDeathEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMedicPowerUpSharedEvent {
    pub player: u16,
}
impl MvmMedicPowerUpSharedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmMedicPowerUpSharedEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBeginWaveEvent {
    pub wave_index: u16,
    pub max_waves: u16,
    pub advanced: u16,
}
impl MvmBeginWaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmBeginWaveEvent {
            wave_index: read_value::<u16>(stream, iter.next(), "wave_index")?,
            max_waves: read_value::<u16>(stream, iter.next(), "max_waves")?,
            advanced: read_value::<u16>(stream, iter.next(), "advanced")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmWaveCompleteEvent {
    pub advanced: bool,
}
impl MvmWaveCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmWaveCompleteEvent {
            advanced: read_value::<bool>(stream, iter.next(), "advanced")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMissionCompleteEvent {
    pub mission: MaybeUtf8String,
}
impl MvmMissionCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmMissionCompleteEvent {
            mission: read_value::<MaybeUtf8String>(stream, iter.next(), "mission")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl MvmBombResetByPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmBombResetByPlayerEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombAlarmTriggeredEvent {}
impl MvmBombAlarmTriggeredEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBombAlarmTriggeredEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombDeployResetByPlayerEvent {
    pub player: u16,
}
impl MvmBombDeployResetByPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmBombDeployResetByPlayerEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmWaveFailedEvent {}
impl MvmWaveFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmWaveFailedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmResetStatsEvent {}
impl MvmResetStatsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmResetStatsEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamageResistedEvent {
    pub ent_index: u8,
}
impl DamageResistedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DamageResistedEvent {
            ent_index: read_value::<u8>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerNotifyEvent {
    pub ent_index: u16,
    pub marker_ent_index: u16,
}
impl RevivePlayerNotifyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RevivePlayerNotifyEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
            marker_ent_index: read_value::<u16>(stream, iter.next(), "marker_ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerStoppedEvent {
    pub ent_index: u16,
}
impl RevivePlayerStoppedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RevivePlayerStoppedEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerCompleteEvent {
    pub ent_index: u16,
}
impl RevivePlayerCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RevivePlayerCompleteEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTurnedToGhostEvent {
    pub user_id: u16,
}
impl PlayerTurnedToGhostEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerTurnedToGhostEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedigunShieldBlockedDamageEvent {
    pub user_id: u16,
    pub damage: f32,
}
impl MedigunShieldBlockedDamageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MedigunShieldBlockedDamageEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            damage: read_value::<f32>(stream, iter.next(), "damage")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl MvmAdvWaveCompleteNoGatesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmAdvWaveCompleteNoGatesEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSniperHeadshotCurrencyEvent {
    pub user_id: u16,
    pub currency: u16,
}
impl MvmSniperHeadshotCurrencyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmSniperHeadshotCurrencyEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            currency: read_value::<u16>(stream, iter.next(), "currency")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMannhattanPitEvent {}
impl MvmMannhattanPitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmMannhattanPitEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlagCarriedInDetectionZoneEvent {}
impl FlagCarriedInDetectionZoneEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlagCarriedInDetectionZoneEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmAdvWaveKilledStunRadioEvent {}
impl MvmAdvWaveKilledStunRadioEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmAdvWaveKilledStunRadioEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDirectHitStunEvent {
    pub attacker: u16,
    pub victim: u16,
}
impl PlayerDirectHitStunEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDirectHitStunEvent {
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl MvmSentryBusterKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MvmSentryBusterKilledEvent {
            sentry_buster: read_value::<u16>(stream, iter.next(), "sentry_buster")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpgradesFileChangedEvent {
    pub path: MaybeUtf8String,
}
impl UpgradesFileChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(UpgradesFileChangedEvent {
            path: read_value::<MaybeUtf8String>(stream, iter.next(), "path")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdTeamPointsChangedEvent {
    pub points: u16,
    pub team: u8,
    pub method: u8,
}
impl RdTeamPointsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RdTeamPointsChangedEvent {
            points: read_value::<u16>(stream, iter.next(), "points")?,
            team: read_value::<u8>(stream, iter.next(), "team")?,
            method: read_value::<u8>(stream, iter.next(), "method")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdRulesStateChangedEvent {}
impl RdRulesStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RdRulesStateChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdRobotKilledEvent {
    pub user_id: u16,
    pub victim_ent_index: u32,
    pub inflictor_ent_index: u32,
    pub attacker: u16,
    pub weapon: MaybeUtf8String,
    pub weapon_id: u16,
    pub damage_bits: u32,
    pub custom_kill: u16,
    pub weapon_log_class_name: MaybeUtf8String,
}
impl RdRobotKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RdRobotKilledEvent {
            user_id: read_value::<u16>(stream, iter.next(), "user_id")?,
            victim_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "victim_ent_index")?,
            inflictor_ent_index: read_value::<
                u32,
            >(stream, iter.next(), "inflictor_ent_index")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(stream, iter.next(), "weapon")?,
            weapon_id: read_value::<u16>(stream, iter.next(), "weapon_id")?,
            damage_bits: read_value::<u32>(stream, iter.next(), "damage_bits")?,
            custom_kill: read_value::<u16>(stream, iter.next(), "custom_kill")?,
            weapon_log_class_name: read_value::<
                MaybeUtf8String,
            >(stream, iter.next(), "weapon_log_class_name")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdRobotImpactEvent {
    pub ent_index: u16,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}
impl RdRobotImpactEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RdRobotImpactEvent {
            ent_index: read_value::<u16>(stream, iter.next(), "ent_index")?,
            impulse_x: read_value::<f32>(stream, iter.next(), "impulse_x")?,
            impulse_y: read_value::<f32>(stream, iter.next(), "impulse_y")?,
            impulse_z: read_value::<f32>(stream, iter.next(), "impulse_z")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPreRoundTimeLeftEvent {
    pub time: u16,
}
impl TeamPlayPreRoundTimeLeftEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamPlayPreRoundTimeLeftEvent {
            time: read_value::<u16>(stream, iter.next(), "time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl ParachuteDeployEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ParachuteDeployEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl ParachuteHolsterEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ParachuteHolsterEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl KillRefillsMeterEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(KillRefillsMeterEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RpsTauntEventEvent {
    pub winner: u16,
    pub winner_rps: u8,
    pub loser: u16,
    pub loser_rps: u8,
}
impl RpsTauntEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RpsTauntEventEvent {
            winner: read_value::<u16>(stream, iter.next(), "winner")?,
            winner_rps: read_value::<u8>(stream, iter.next(), "winner_rps")?,
            loser: read_value::<u16>(stream, iter.next(), "loser")?,
            loser_rps: read_value::<u8>(stream, iter.next(), "loser_rps")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CongaKillEvent {
    pub index: u16,
}
impl CongaKillEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CongaKillEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl PlayerInitialSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerInitialSpawnEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompetitiveVictoryEvent {}
impl CompetitiveVictoryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CompetitiveVictoryEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompetitiveStatsUpdateEvent {
    pub index: u16,
    pub kills_rank: u8,
    pub score_rank: u8,
    pub damage_rank: u8,
    pub healing_rank: u8,
    pub support_rank: u8,
}
impl CompetitiveStatsUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CompetitiveStatsUpdateEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
            kills_rank: read_value::<u8>(stream, iter.next(), "kills_rank")?,
            score_rank: read_value::<u8>(stream, iter.next(), "score_rank")?,
            damage_rank: read_value::<u8>(stream, iter.next(), "damage_rank")?,
            healing_rank: read_value::<u8>(stream, iter.next(), "healing_rank")?,
            support_rank: read_value::<u8>(stream, iter.next(), "support_rank")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MiniGameWinEvent {
    pub team: u8,
    pub kind: u8,
}
impl MiniGameWinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MiniGameWinEvent {
            team: read_value::<u8>(stream, iter.next(), "team")?,
            kind: read_value::<u8>(stream, iter.next(), "kind")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl SentryOnGoActiveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(SentryOnGoActiveEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl DuckXpLevelUpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DuckXpLevelUpEvent {
            level: read_value::<u16>(stream, iter.next(), "level")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestLogOpenedEvent {}
impl QuestLogOpenedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestLogOpenedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SchemaUpdatedEvent {}
impl SchemaUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SchemaUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerPickupWeaponEvent {}
impl LocalPlayerPickupWeaponEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerPickupWeaponEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdPlayerScorePointsEvent {
    pub player: u16,
    pub method: u16,
    pub amount: u16,
}
impl RdPlayerScorePointsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RdPlayerScorePointsEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            method: read_value::<u16>(stream, iter.next(), "method")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DemomanDetStickiesEvent {
    pub player: u16,
}
impl DemomanDetStickiesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DemomanDetStickiesEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestObjectiveCompletedEvent {
    pub quest_item_id_low: u32,
    pub quest_item_id_hi: u32,
    pub quest_objective_id: u32,
    pub scorer_user_id: u16,
}
impl QuestObjectiveCompletedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(QuestObjectiveCompletedEvent {
            quest_item_id_low: read_value::<
                u32,
            >(stream, iter.next(), "quest_item_id_low")?,
            quest_item_id_hi: read_value::<
                u32,
            >(stream, iter.next(), "quest_item_id_hi")?,
            quest_objective_id: read_value::<
                u32,
            >(stream, iter.next(), "quest_objective_id")?,
            scorer_user_id: read_value::<u16>(stream, iter.next(), "scorer_user_id")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerScoreChangedEvent {
    pub player: u8,
    pub delta: u16,
}
impl PlayerScoreChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerScoreChangedEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            delta: read_value::<u16>(stream, iter.next(), "delta")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct KilledCappingPlayerEvent {
    pub cp: u8,
    pub killer: u8,
    pub victim: u8,
    pub assister: u8,
}
impl KilledCappingPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(KilledCappingPlayerEvent {
            cp: read_value::<u8>(stream, iter.next(), "cp")?,
            killer: read_value::<u8>(stream, iter.next(), "killer")?,
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
            assister: read_value::<u8>(stream, iter.next(), "assister")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnvironmentalDeathEvent {
    pub killer: u8,
    pub victim: u8,
}
impl EnvironmentalDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EnvironmentalDeathEvent {
            killer: read_value::<u8>(stream, iter.next(), "killer")?,
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectileDirectHitEvent {
    pub attacker: u8,
    pub victim: u8,
    pub weapon_def_index: u32,
}
impl ProjectileDirectHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ProjectileDirectHitEvent {
            attacker: read_value::<u8>(stream, iter.next(), "attacker")?,
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
            weapon_def_index: read_value::<u32>(stream, iter.next(), "weapon_def_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassGetEvent {
    pub owner: u16,
}
impl PassGetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassGetEvent {
            owner: read_value::<u16>(stream, iter.next(), "owner")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassScoreEvent {
    pub scorer: u16,
    pub assister: u16,
    pub points: u8,
}
impl PassScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassScoreEvent {
            scorer: read_value::<u16>(stream, iter.next(), "scorer")?,
            assister: read_value::<u16>(stream, iter.next(), "assister")?,
            points: read_value::<u8>(stream, iter.next(), "points")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassFreeEvent {
    pub owner: u16,
    pub attacker: u16,
}
impl PassFreeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassFreeEvent {
            owner: read_value::<u16>(stream, iter.next(), "owner")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassPassCaughtEvent {
    pub passer: u16,
    pub catcher: u16,
    pub dist: f32,
    pub duration: f32,
}
impl PassPassCaughtEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassPassCaughtEvent {
            passer: read_value::<u16>(stream, iter.next(), "passer")?,
            catcher: read_value::<u16>(stream, iter.next(), "catcher")?,
            dist: read_value::<f32>(stream, iter.next(), "dist")?,
            duration: read_value::<f32>(stream, iter.next(), "duration")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassBallStolenEvent {
    pub victim: u16,
    pub attacker: u16,
}
impl PassBallStolenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassBallStolenEvent {
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
            attacker: read_value::<u16>(stream, iter.next(), "attacker")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassBallBlockedEvent {
    pub owner: u16,
    pub blocker: u16,
}
impl PassBallBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PassBallBlockedEvent {
            owner: read_value::<u16>(stream, iter.next(), "owner")?,
            blocker: read_value::<u16>(stream, iter.next(), "blocker")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamagePreventedEvent {
    pub preventor: u16,
    pub victim: u16,
    pub amount: u16,
    pub condition: u16,
}
impl DamagePreventedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DamagePreventedEvent {
            preventor: read_value::<u16>(stream, iter.next(), "preventor")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
            condition: read_value::<u16>(stream, iter.next(), "condition")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenBossKilledEvent {
    pub boss: u16,
    pub killer: u16,
}
impl HalloweenBossKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HalloweenBossKilledEvent {
            boss: read_value::<u16>(stream, iter.next(), "boss")?,
            killer: read_value::<u16>(stream, iter.next(), "killer")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscapedLootIslandEvent {
    pub player: u16,
}
impl EscapedLootIslandEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EscapedLootIslandEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TaggedPlayerAsItEvent {
    pub player: u16,
}
impl TaggedPlayerAsItEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TaggedPlayerAsItEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusStunnedEvent {
    pub player: u16,
}
impl MerasmusStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusStunnedEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusPropFoundEvent {
    pub player: u16,
}
impl MerasmusPropFoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MerasmusPropFoundEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenSkeletonKilledEvent {
    pub player: u16,
}
impl HalloweenSkeletonKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HalloweenSkeletonKilledEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscapeHellEvent {
    pub player: u16,
}
impl EscapeHellEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(EscapeHellEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CrossSpectralBridgeEvent {
    pub player: u16,
}
impl CrossSpectralBridgeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CrossSpectralBridgeEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MiniGameWonEvent {
    pub player: u16,
    pub game: u16,
}
impl MiniGameWonEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(MiniGameWonEvent {
            player: read_value::<u16>(stream, iter.next(), "player")?,
            game: read_value::<u16>(stream, iter.next(), "game")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RespawnGhostEvent {
    pub reviver: u16,
    pub ghost: u16,
}
impl RespawnGhostEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RespawnGhostEvent {
            reviver: read_value::<u16>(stream, iter.next(), "reviver")?,
            ghost: read_value::<u16>(stream, iter.next(), "ghost")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct KillInHellEvent {
    pub killer: u16,
    pub victim: u16,
}
impl KillInHellEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(KillInHellEvent {
            killer: read_value::<u16>(stream, iter.next(), "killer")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenDuckCollectedEvent {
    pub collector: u16,
}
impl HalloweenDuckCollectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HalloweenDuckCollectedEvent {
            collector: read_value::<u16>(stream, iter.next(), "collector")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecialScoreEvent {
    pub player: u8,
}
impl SpecialScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(SpecialScoreEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamLeaderKilledEvent {
    pub killer: u8,
    pub victim: u8,
}
impl TeamLeaderKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(TeamLeaderKilledEvent {
            killer: read_value::<u8>(stream, iter.next(), "killer")?,
            victim: read_value::<u8>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenSoulCollectedEvent {
    pub intended_target: u8,
    pub collecting_player: u8,
    pub soul_count: u8,
}
impl HalloweenSoulCollectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HalloweenSoulCollectedEvent {
            intended_target: read_value::<u8>(stream, iter.next(), "intended_target")?,
            collecting_player: read_value::<
                u8,
            >(stream, iter.next(), "collecting_player")?,
            soul_count: read_value::<u8>(stream, iter.next(), "soul_count")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecalculateTruceEvent {}
impl RecalculateTruceEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RecalculateTruceEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeadRingerCheatDeathEvent {
    pub spy: u8,
    pub attacker: u8,
}
impl DeadRingerCheatDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DeadRingerCheatDeathEvent {
            spy: read_value::<u8>(stream, iter.next(), "spy")?,
            attacker: read_value::<u8>(stream, iter.next(), "attacker")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CrossbowHealEvent {
    pub healer: u8,
    pub target: u8,
    pub amount: u16,
}
impl CrossbowHealEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CrossbowHealEvent {
            healer: read_value::<u8>(stream, iter.next(), "healer")?,
            target: read_value::<u8>(stream, iter.next(), "target")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamageMitigatedEvent {
    pub mitigator: u8,
    pub damaged: u8,
    pub amount: u16,
    pub item_definition_index: u16,
}
impl DamageMitigatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DamageMitigatedEvent {
            mitigator: read_value::<u8>(stream, iter.next(), "mitigator")?,
            damaged: read_value::<u8>(stream, iter.next(), "damaged")?,
            amount: read_value::<u16>(stream, iter.next(), "amount")?,
            item_definition_index: read_value::<
                u16,
            >(stream, iter.next(), "item_definition_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PayloadPushedEvent {
    pub pusher: u8,
    pub distance: u16,
}
impl PayloadPushedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PayloadPushedEvent {
            pusher: read_value::<u8>(stream, iter.next(), "pusher")?,
            distance: read_value::<u16>(stream, iter.next(), "distance")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAbandonedMatchEvent {
    pub game_over: bool,
}
impl PlayerAbandonedMatchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerAbandonedMatchEvent {
            game_over: read_value::<bool>(stream, iter.next(), "game_over")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClDrawlineEvent {
    pub player: u8,
    pub panel: u8,
    pub line: u8,
    pub x: f32,
    pub y: f32,
}
impl ClDrawlineEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ClDrawlineEvent {
            player: read_value::<u8>(stream, iter.next(), "player")?,
            panel: read_value::<u8>(stream, iter.next(), "panel")?,
            line: read_value::<u8>(stream, iter.next(), "line")?,
            x: read_value::<f32>(stream, iter.next(), "x")?,
            y: read_value::<f32>(stream, iter.next(), "y")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RestartTimerTimeEvent {
    pub time: u8,
}
impl RestartTimerTimeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RestartTimerTimeEvent {
            time: read_value::<u8>(stream, iter.next(), "time")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct WinLimitChangedEvent {}
impl WinLimitChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WinLimitChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct WinPanelShowScoresEvent {}
impl WinPanelShowScoresEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WinPanelShowScoresEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct TopStreamsRequestFinishedEvent {}
impl TopStreamsRequestFinishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TopStreamsRequestFinishedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompetitiveStateChangedEvent {}
impl CompetitiveStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CompetitiveStateChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GlobalWarDataUpdatedEvent {}
impl GlobalWarDataUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GlobalWarDataUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct StopWatchChangedEvent {}
impl StopWatchChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StopWatchChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DsStopEvent {}
impl DsStopEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DsStopEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct DsScreenshotEvent {
    pub delay: f32,
}
impl DsScreenshotEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(DsScreenshotEvent {
            delay: read_value::<f32>(stream, iter.next(), "delay")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowMatchSummaryEvent {}
impl ShowMatchSummaryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowMatchSummaryEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExperienceChangedEvent {}
impl ExperienceChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ExperienceChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct BeginXpLerpEvent {}
impl BeginXpLerpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BeginXpLerpEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MatchmakerStatsUpdatedEvent {}
impl MatchmakerStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MatchmakerStatsUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RematchVotePeriodOverEvent {
    pub success: bool,
}
impl RematchVotePeriodOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(RematchVotePeriodOverEvent {
            success: read_value::<bool>(stream, iter.next(), "success")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct RematchFailedToCreateEvent {}
impl RematchFailedToCreateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RematchFailedToCreateEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRematchChangeEvent {}
impl PlayerRematchChangeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerRematchChangeEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PingUpdatedEvent {}
impl PingUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PingUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MMStatsUpdatedEvent {}
impl MMStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MMStatsUpdatedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerNextMapVoteChangeEvent {
    pub map_index: u8,
    pub vote: u8,
}
impl PlayerNextMapVoteChangeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerNextMapVoteChangeEvent {
            map_index: read_value::<u8>(stream, iter.next(), "map_index")?,
            vote: read_value::<u8>(stream, iter.next(), "vote")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteMapsChangedEvent {}
impl VoteMapsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteMapsChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProtoDefChangedEvent {
    pub kind: u8,
    pub definition_index: u32,
    pub created: bool,
    pub deleted: bool,
    pub erase_history: bool,
}
impl ProtoDefChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ProtoDefChangedEvent {
            kind: read_value::<u8>(stream, iter.next(), "kind")?,
            definition_index: read_value::<
                u32,
            >(stream, iter.next(), "definition_index")?,
            created: read_value::<bool>(stream, iter.next(), "created")?,
            deleted: read_value::<bool>(stream, iter.next(), "deleted")?,
            erase_history: read_value::<bool>(stream, iter.next(), "erase_history")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDominationEvent {
    pub dominator: u16,
    pub dominated: u16,
    pub dominations: u16,
}
impl PlayerDominationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerDominationEvent {
            dominator: read_value::<u16>(stream, iter.next(), "dominator")?,
            dominated: read_value::<u16>(stream, iter.next(), "dominated")?,
            dominations: read_value::<u16>(stream, iter.next(), "dominations")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRocketPackPushedEvent {
    pub pusher: u16,
    pub pushed: u16,
}
impl PlayerRocketPackPushedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(PlayerRocketPackPushedEvent {
            pusher: read_value::<u16>(stream, iter.next(), "pusher")?,
            pushed: read_value::<u16>(stream, iter.next(), "pushed")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestRequestEvent {
    pub request: u32,
    pub msg: MaybeUtf8String,
}
impl QuestRequestEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(QuestRequestEvent {
            request: read_value::<u32>(stream, iter.next(), "request")?,
            msg: read_value::<MaybeUtf8String>(stream, iter.next(), "msg")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestResponseEvent {
    pub request: u32,
    pub success: bool,
    pub msg: MaybeUtf8String,
}
impl QuestResponseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(QuestResponseEvent {
            request: read_value::<u32>(stream, iter.next(), "request")?,
            success: read_value::<bool>(stream, iter.next(), "success")?,
            msg: read_value::<MaybeUtf8String>(stream, iter.next(), "msg")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestProgressEvent {
    pub owner: u16,
    pub scorer: u16,
    pub kind: u8,
    pub completed: bool,
    pub quest_definition_index: u32,
}
impl QuestProgressEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(QuestProgressEvent {
            owner: read_value::<u16>(stream, iter.next(), "owner")?,
            scorer: read_value::<u16>(stream, iter.next(), "scorer")?,
            kind: read_value::<u8>(stream, iter.next(), "kind")?,
            completed: read_value::<bool>(stream, iter.next(), "completed")?,
            quest_definition_index: read_value::<
                u32,
            >(stream, iter.next(), "quest_definition_index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectileRemovedEvent {
    pub attacker: u8,
    pub weapon_def_index: u32,
    pub num_hit: u8,
    pub num_direct_hit: u8,
}
impl ProjectileRemovedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ProjectileRemovedEvent {
            attacker: read_value::<u8>(stream, iter.next(), "attacker")?,
            weapon_def_index: read_value::<
                u32,
            >(stream, iter.next(), "weapon_def_index")?,
            num_hit: read_value::<u8>(stream, iter.next(), "num_hit")?,
            num_direct_hit: read_value::<u8>(stream, iter.next(), "num_direct_hit")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestMapDataChangedEvent {}
impl QuestMapDataChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestMapDataChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct GasDousedPlayerIgnitedEvent {
    pub igniter: u16,
    pub douser: u16,
    pub victim: u16,
}
impl GasDousedPlayerIgnitedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(GasDousedPlayerIgnitedEvent {
            igniter: read_value::<u16>(stream, iter.next(), "igniter")?,
            douser: read_value::<u16>(stream, iter.next(), "douser")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestTurnInStateEvent {
    pub state: u16,
}
impl QuestTurnInStateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(QuestTurnInStateEvent {
            state: read_value::<u16>(stream, iter.next(), "state")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemsAcknowledgedEvent {}
impl ItemsAcknowledgedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemsAcknowledgedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct CapperKilledEvent {
    pub blocker: u16,
    pub victim: u16,
}
impl CapperKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(CapperKilledEvent {
            blocker: read_value::<u16>(stream, iter.next(), "blocker")?,
            victim: read_value::<u16>(stream, iter.next(), "victim")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct MainMenuStabilizedEvent {}
impl MainMenuStabilizedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MainMenuStabilizedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorldStatusChangedEvent {}
impl WorldStatusChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WorldStatusChangedEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVStatusEvent {
    pub clients: u32,
    pub slots: u32,
    pub proxies: u16,
    pub master: MaybeUtf8String,
}
impl HLTVStatusEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVStatusEvent {
            clients: read_value::<u32>(stream, iter.next(), "clients")?,
            slots: read_value::<u32>(stream, iter.next(), "slots")?,
            proxies: read_value::<u16>(stream, iter.next(), "proxies")?,
            master: read_value::<MaybeUtf8String>(stream, iter.next(), "master")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVCameramanEvent {
    pub index: u16,
}
impl HLTVCameramanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVCameramanEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVRankCameraEvent {
    pub index: u8,
    pub rank: f32,
    pub target: u16,
}
impl HLTVRankCameraEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVRankCameraEvent {
            index: read_value::<u8>(stream, iter.next(), "index")?,
            rank: read_value::<f32>(stream, iter.next(), "rank")?,
            target: read_value::<u16>(stream, iter.next(), "target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVRankEntityEvent {
    pub index: u16,
    pub rank: f32,
    pub target: u16,
}
impl HLTVRankEntityEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVRankEntityEvent {
            index: read_value::<u16>(stream, iter.next(), "index")?,
            rank: read_value::<f32>(stream, iter.next(), "rank")?,
            target: read_value::<u16>(stream, iter.next(), "target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
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
impl HLTVFixedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVFixedEvent {
            pos_x: read_value::<u32>(stream, iter.next(), "pos_x")?,
            pos_y: read_value::<u32>(stream, iter.next(), "pos_y")?,
            pos_z: read_value::<u32>(stream, iter.next(), "pos_z")?,
            theta: read_value::<u16>(stream, iter.next(), "theta")?,
            phi: read_value::<u16>(stream, iter.next(), "phi")?,
            offset: read_value::<u16>(stream, iter.next(), "offset")?,
            fov: read_value::<f32>(stream, iter.next(), "fov")?,
            target: read_value::<u16>(stream, iter.next(), "target")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChaseEvent {
    pub target_1: u16,
    pub target_2: u16,
    pub distance: u16,
    pub theta: u16,
    pub phi: u16,
    pub inertia: u8,
    pub in_eye: u8,
}
impl HLTVChaseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVChaseEvent {
            target_1: read_value::<u16>(stream, iter.next(), "target_1")?,
            target_2: read_value::<u16>(stream, iter.next(), "target_2")?,
            distance: read_value::<u16>(stream, iter.next(), "distance")?,
            theta: read_value::<u16>(stream, iter.next(), "theta")?,
            phi: read_value::<u16>(stream, iter.next(), "phi")?,
            inertia: read_value::<u8>(stream, iter.next(), "inertia")?,
            in_eye: read_value::<u8>(stream, iter.next(), "in_eye")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVMessageEvent {
    pub text: MaybeUtf8String,
}
impl HLTVMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVMessageEvent {
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVTitleEvent {
    pub text: MaybeUtf8String,
}
impl HLTVTitleEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVTitleEvent {
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChatEvent {
    pub text: MaybeUtf8String,
}
impl HLTVChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(HLTVChatEvent {
            text: read_value::<MaybeUtf8String>(stream, iter.next(), "text")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayStartRecordEvent {}
impl ReplayStartRecordEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayStartRecordEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplaySessionInfoEvent {
    pub sn: MaybeUtf8String,
    pub di: u8,
    pub cb: u32,
    pub st: u32,
}
impl ReplaySessionInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ReplaySessionInfoEvent {
            sn: read_value::<MaybeUtf8String>(stream, iter.next(), "sn")?,
            di: read_value::<u8>(stream, iter.next(), "di")?,
            cb: read_value::<u32>(stream, iter.next(), "cb")?,
            st: read_value::<u32>(stream, iter.next(), "st")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayEndRecordEvent {}
impl ReplayEndRecordEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayEndRecordEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayReplaysAvailableEvent {}
impl ReplayReplaysAvailableEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayReplaysAvailableEvent {})
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, BitWrite, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayServerErrorEvent {
    pub error: MaybeUtf8String,
}
impl ReplayServerErrorEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        let mut iter = definition.entries.iter();
        Ok(ReplayServerErrorEvent {
            error: read_value::<MaybeUtf8String>(stream, iter.next(), "error")?,
        })
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum GameEvent {
    ServerSpawn(Box<ServerSpawnEvent>),
    ServerChangeLevelFailed(ServerChangeLevelFailedEvent),
    ServerShutdown(ServerShutdownEvent),
    ServerCvar(ServerCvarEvent),
    ServerMessage(ServerMessageEvent),
    ServerAddBan(Box<ServerAddBanEvent>),
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
    PlayerDeath(Box<PlayerDeathEvent>),
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
    VoteOptions(Box<VoteOptionsEvent>),
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
    FishNotice(Box<FishNoticeEvent>),
    FishNoticeArm(Box<FishNoticeArmEvent>),
    SlapNotice(Box<SlapNoticeEvent>),
    ThrowableHit(Box<ThrowableHitEvent>),
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
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameEventType {
    ServerSpawn,
    ServerChangeLevelFailed,
    ServerShutdown,
    ServerCvar,
    ServerMessage,
    ServerAddBan,
    ServerRemoveBan,
    PlayerConnect,
    PlayerConnectClient,
    PlayerInfo,
    PlayerDisconnect,
    PlayerActivate,
    PlayerSay,
    ClientDisconnect,
    ClientBeginConnect,
    ClientConnected,
    ClientFullConnect,
    HostQuit,
    TeamInfo,
    TeamScore,
    TeamPlayBroadcastAudio,
    PlayerTeam,
    PlayerClass,
    PlayerDeath,
    PlayerHurt,
    PlayerChat,
    PlayerScore,
    PlayerSpawn,
    PlayerShoot,
    PlayerUse,
    PlayerChangeName,
    PlayerHintMessage,
    BasePlayerTeleported,
    GameInit,
    GameNewMap,
    GameStart,
    GameEnd,
    RoundStart,
    RoundEnd,
    GameMessage,
    BreakBreakable,
    BreakProp,
    EntityKilled,
    BonusUpdated,
    AchievementEvent,
    AchievementIncrement,
    PhysgunPickup,
    FlareIgniteNpc,
    HelicopterGrenadePuntMiss,
    UserDataDownloaded,
    RagdollDissolved,
    HLTVChangedMode,
    HLTVChangedTarget,
    VoteEnded,
    VoteStarted,
    VoteChanged,
    VotePassed,
    VoteFailed,
    VoteCast,
    VoteOptions,
    ReplaySaved,
    EnteredPerformanceMode,
    BrowseReplays,
    ReplayYoutubeStats,
    InventoryUpdated,
    CartUpdated,
    StorePriceSheetUpdated,
    EconInventoryConnected,
    ItemSchemaInitialized,
    GcNewSession,
    GcLostSession,
    IntroFinish,
    IntroNextCamera,
    PlayerChangeClass,
    TfMapTimeRemaining,
    TfGameOver,
    CtfFlagCaptured,
    ControlPointInitialized,
    ControlPointUpdateImages,
    ControlPointUpdateLayout,
    ControlPointUpdateCapping,
    ControlPointUpdateOwner,
    ControlPointStartTouch,
    ControlPointEndTouch,
    ControlPointPulseElement,
    ControlPointFakeCapture,
    ControlPointFakeCaptureMultiplier,
    TeamPlayRoundSelected,
    TeamPlayRoundStart,
    TeamPlayRoundActive,
    TeamPlayWaitingBegins,
    TeamPlayWaitingEnds,
    TeamPlayWaitingAboutToEnd,
    TeamPlayRestartRound,
    TeamPlayReadyRestart,
    TeamPlayRoundRestartSeconds,
    TeamPlayTeamReady,
    TeamPlayRoundWin,
    TeamPlayUpdateTimer,
    TeamPlayRoundStalemate,
    TeamPlayOvertimeBegin,
    TeamPlayOvertimeEnd,
    TeamPlaySuddenDeathBegin,
    TeamPlaySuddenDeathEnd,
    TeamPlayGameOver,
    TeamPlayMapTimeRemaining,
    TeamPlayTimerFlash,
    TeamPlayTimerTimeAdded,
    TeamPlayPointStartCapture,
    TeamPlayPointCaptured,
    TeamPlayPointLocked,
    TeamPlayPointUnlocked,
    TeamPlayCaptureBroken,
    TeamPlayCaptureBlocked,
    TeamPlayFlagEvent,
    TeamPlayWinPanel,
    TeamPlayTeamBalancedPlayer,
    TeamPlaySetupFinished,
    TeamPlayAlert,
    TrainingComplete,
    ShowFreezePanel,
    HideFreezePanel,
    FreezeCamStarted,
    LocalPlayerChangeTeam,
    LocalPlayerScoreChanged,
    LocalPlayerChangeClass,
    LocalPlayerRespawn,
    BuildingInfoChanged,
    LocalPlayerChangeDisguise,
    PlayerAccountChanged,
    SpyPdaReset,
    FlagStatusUpdate,
    PlayerStatsUpdated,
    PlayingCommentary,
    PlayerChargeDeployed,
    PlayerBuiltObject,
    PlayerUpgradedObject,
    PlayerCarryObject,
    PlayerDropObject,
    ObjectRemoved,
    ObjectDestroyed,
    ObjectDetonated,
    AchievementEarned,
    SpecTargetUpdated,
    TournamentStateUpdate,
    TournamentEnableCountdown,
    PlayerCalledForMedic,
    PlayerAskedForBall,
    LocalPlayerBecameObserver,
    PlayerIgnitedInv,
    PlayerIgnited,
    PlayerExtinguished,
    PlayerTeleported,
    PlayerHealedMedicCall,
    LocalPlayerChargeReady,
    LocalPlayerWindDown,
    PlayerInvulned,
    EscortSpeed,
    EscortProgress,
    EscortRecede,
    GameUIActivated,
    GameUIHidden,
    PlayerEscortScore,
    PlayerHealOnHit,
    PlayerStealSandvich,
    ShowClassLayout,
    ShowVsPanel,
    PlayerDamaged,
    ArenaPlayerNotification,
    ArenaMatchMaxStreak,
    ArenaRoundStart,
    ArenaWinPanel,
    PveWinPanel,
    AirDash,
    Landed,
    PlayerDamageDodged,
    PlayerStunned,
    ScoutGrandSlam,
    ScoutSlamdollLanded,
    ArrowImpact,
    PlayerJarated,
    PlayerJaratedFade,
    PlayerShieldBlocked,
    PlayerPinned,
    PlayerHealedByMedic,
    PlayerSappedObject,
    ItemFound,
    ShowAnnotation,
    HideAnnotation,
    PostInventoryApplication,
    ControlPointUnlockUpdated,
    DeployBuffBanner,
    PlayerBuff,
    MedicDeath,
    OvertimeNag,
    TeamsChanged,
    HalloweenPumpkinGrab,
    RocketJump,
    RocketJumpLanded,
    StickyJump,
    StickyJumpLanded,
    RocketPackLaunch,
    RocketPackLanded,
    MedicDefended,
    LocalPlayerHealed,
    PlayerDestroyedPipeBomb,
    ObjectDeflected,
    PlayerMvp,
    RaidSpawnMob,
    RaidSpawnSquad,
    NavBlocked,
    PathTrackPassed,
    NumCappersChanged,
    PlayerRegenerate,
    UpdateStatusItem,
    StatsResetRound,
    ScoreStatsAccumulatedUpdate,
    ScoreStatsAccumulatedReset,
    AchievementEarnedLocal,
    PlayerHealed,
    BuildingHealed,
    ItemPickup,
    DuelStatus,
    FishNotice,
    FishNoticeArm,
    SlapNotice,
    ThrowableHit,
    PumpkinLordSummoned,
    PumpkinLordKilled,
    MerasmusSummoned,
    MerasmusKilled,
    MerasmusEscapeWarning,
    MerasmusEscaped,
    EyeballBossSummoned,
    EyeballBossStunned,
    EyeballBossKilled,
    EyeballBossKiller,
    EyeballBossEscapeImminent,
    EyeballBossEscaped,
    NpcHurt,
    ControlPointTimerUpdated,
    PlayerHighFiveStart,
    PlayerHighFiveCancel,
    PlayerHighFiveSuccess,
    PlayerBonusPoints,
    PlayerUpgraded,
    PlayerBuyback,
    PlayerUsedPowerUpBottle,
    ChristmasGiftGrab,
    PlayerKilledAchievementZone,
    PartyUpdated,
    PartyPrefChanged,
    PartyCriteriaChanged,
    PartyInvitesChanged,
    PartyQueueStateChanged,
    PartyChat,
    PartyMemberJoin,
    PartyMemberLeave,
    MatchInvitesUpdated,
    LobbyUpdated,
    MvmMissionUpdate,
    RecalculateHolidays,
    PlayerCurrencyChanged,
    DoomsdayRocketOpen,
    RemoveNemesisRelationships,
    MvmCreditBonusWave,
    MvmCreditBonusAll,
    MvmCreditBonusAllAdvanced,
    MvmQuickSentryUpgrade,
    MvmTankDestroyedByPlayers,
    MvmKillRobotDeliveringBomb,
    MvmPickupCurrency,
    MvmBombCarrierKilled,
    MvmSentryBusterDetonate,
    MvmScoutMarkedForDeath,
    MvmMedicPowerUpShared,
    MvmBeginWave,
    MvmWaveComplete,
    MvmMissionComplete,
    MvmBombResetByPlayer,
    MvmBombAlarmTriggered,
    MvmBombDeployResetByPlayer,
    MvmWaveFailed,
    MvmResetStats,
    DamageResisted,
    RevivePlayerNotify,
    RevivePlayerStopped,
    RevivePlayerComplete,
    PlayerTurnedToGhost,
    MedigunShieldBlockedDamage,
    MvmAdvWaveCompleteNoGates,
    MvmSniperHeadshotCurrency,
    MvmMannhattanPit,
    FlagCarriedInDetectionZone,
    MvmAdvWaveKilledStunRadio,
    PlayerDirectHitStun,
    MvmSentryBusterKilled,
    UpgradesFileChanged,
    RdTeamPointsChanged,
    RdRulesStateChanged,
    RdRobotKilled,
    RdRobotImpact,
    TeamPlayPreRoundTimeLeft,
    ParachuteDeploy,
    ParachuteHolster,
    KillRefillsMeter,
    RpsTauntEvent,
    CongaKill,
    PlayerInitialSpawn,
    CompetitiveVictory,
    CompetitiveStatsUpdate,
    MiniGameWin,
    SentryOnGoActive,
    DuckXpLevelUp,
    QuestLogOpened,
    SchemaUpdated,
    LocalPlayerPickupWeapon,
    RdPlayerScorePoints,
    DemomanDetStickies,
    QuestObjectiveCompleted,
    PlayerScoreChanged,
    KilledCappingPlayer,
    EnvironmentalDeath,
    ProjectileDirectHit,
    PassGet,
    PassScore,
    PassFree,
    PassPassCaught,
    PassBallStolen,
    PassBallBlocked,
    DamagePrevented,
    HalloweenBossKilled,
    EscapedLootIsland,
    TaggedPlayerAsIt,
    MerasmusStunned,
    MerasmusPropFound,
    HalloweenSkeletonKilled,
    EscapeHell,
    CrossSpectralBridge,
    MiniGameWon,
    RespawnGhost,
    KillInHell,
    HalloweenDuckCollected,
    SpecialScore,
    TeamLeaderKilled,
    HalloweenSoulCollected,
    RecalculateTruce,
    DeadRingerCheatDeath,
    CrossbowHeal,
    DamageMitigated,
    PayloadPushed,
    PlayerAbandonedMatch,
    ClDrawline,
    RestartTimerTime,
    WinLimitChanged,
    WinPanelShowScores,
    TopStreamsRequestFinished,
    CompetitiveStateChanged,
    GlobalWarDataUpdated,
    StopWatchChanged,
    DsStop,
    DsScreenshot,
    ShowMatchSummary,
    ExperienceChanged,
    BeginXpLerp,
    MatchmakerStatsUpdated,
    RematchVotePeriodOver,
    RematchFailedToCreate,
    PlayerRematchChange,
    PingUpdated,
    MMStatsUpdated,
    PlayerNextMapVoteChange,
    VoteMapsChanged,
    ProtoDefChanged,
    PlayerDomination,
    PlayerRocketPackPushed,
    QuestRequest,
    QuestResponse,
    QuestProgress,
    ProjectileRemoved,
    QuestMapDataChanged,
    GasDousedPlayerIgnited,
    QuestTurnInState,
    ItemsAcknowledged,
    CapperKilled,
    MainMenuStabilized,
    WorldStatusChanged,
    HLTVStatus,
    HLTVCameraman,
    HLTVRankCamera,
    HLTVRankEntity,
    HLTVFixed,
    HLTVChase,
    HLTVMessage,
    HLTVTitle,
    HLTVChat,
    ReplayStartRecord,
    ReplaySessionInfo,
    ReplayEndRecord,
    ReplayReplaysAvailable,
    ReplayServerError,
    Unknown(String),
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
            "controlpoint_fake_capture_mult" => {
                GameEventType::ControlPointFakeCaptureMultiplier
            }
            "teamplay_round_selected" => GameEventType::TeamPlayRoundSelected,
            "teamplay_round_start" => GameEventType::TeamPlayRoundStart,
            "teamplay_round_active" => GameEventType::TeamPlayRoundActive,
            "teamplay_waiting_begins" => GameEventType::TeamPlayWaitingBegins,
            "teamplay_waiting_ends" => GameEventType::TeamPlayWaitingEnds,
            "teamplay_waiting_abouttoend" => GameEventType::TeamPlayWaitingAboutToEnd,
            "teamplay_restart_round" => GameEventType::TeamPlayRestartRound,
            "teamplay_ready_restart" => GameEventType::TeamPlayReadyRestart,
            "teamplay_round_restart_seconds" => {
                GameEventType::TeamPlayRoundRestartSeconds
            }
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
            "player_killed_achievement_zone" => {
                GameEventType::PlayerKilledAchievementZone
            }
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
            "mvm_bomb_deploy_reset_by_player" => {
                GameEventType::MvmBombDeployResetByPlayer
            }
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
            ty => GameEventType::Unknown(ty.into()),
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            GameEventType::ServerSpawn => "server_spawn",
            GameEventType::ServerChangeLevelFailed => "server_changelevel_failed",
            GameEventType::ServerShutdown => "server_shutdown",
            GameEventType::ServerCvar => "server_cvar",
            GameEventType::ServerMessage => "server_message",
            GameEventType::ServerAddBan => "server_addban",
            GameEventType::ServerRemoveBan => "server_removeban",
            GameEventType::PlayerConnect => "player_connect",
            GameEventType::PlayerConnectClient => "player_connect_client",
            GameEventType::PlayerInfo => "player_info",
            GameEventType::PlayerDisconnect => "player_disconnect",
            GameEventType::PlayerActivate => "player_activate",
            GameEventType::PlayerSay => "player_say",
            GameEventType::ClientDisconnect => "client_disconnect",
            GameEventType::ClientBeginConnect => "client_beginconnect",
            GameEventType::ClientConnected => "client_connected",
            GameEventType::ClientFullConnect => "client_fullconnect",
            GameEventType::HostQuit => "host_quit",
            GameEventType::TeamInfo => "team_info",
            GameEventType::TeamScore => "team_score",
            GameEventType::TeamPlayBroadcastAudio => "teamplay_broadcast_audio",
            GameEventType::PlayerTeam => "player_team",
            GameEventType::PlayerClass => "player_class",
            GameEventType::PlayerDeath => "player_death",
            GameEventType::PlayerHurt => "player_hurt",
            GameEventType::PlayerChat => "player_chat",
            GameEventType::PlayerScore => "player_score",
            GameEventType::PlayerSpawn => "player_spawn",
            GameEventType::PlayerShoot => "player_shoot",
            GameEventType::PlayerUse => "player_use",
            GameEventType::PlayerChangeName => "player_changename",
            GameEventType::PlayerHintMessage => "player_hintmessage",
            GameEventType::BasePlayerTeleported => "base_player_teleported",
            GameEventType::GameInit => "game_init",
            GameEventType::GameNewMap => "game_newmap",
            GameEventType::GameStart => "game_start",
            GameEventType::GameEnd => "game_end",
            GameEventType::RoundStart => "round_start",
            GameEventType::RoundEnd => "round_end",
            GameEventType::GameMessage => "game_message",
            GameEventType::BreakBreakable => "break_breakable",
            GameEventType::BreakProp => "break_prop",
            GameEventType::EntityKilled => "entity_killed",
            GameEventType::BonusUpdated => "bonus_updated",
            GameEventType::AchievementEvent => "achievement_event",
            GameEventType::AchievementIncrement => "achievement_increment",
            GameEventType::PhysgunPickup => "physgun_pickup",
            GameEventType::FlareIgniteNpc => "flare_ignite_npc",
            GameEventType::HelicopterGrenadePuntMiss => "helicopter_grenade_punt_miss",
            GameEventType::UserDataDownloaded => "user_data_downloaded",
            GameEventType::RagdollDissolved => "ragdoll_dissolved",
            GameEventType::HLTVChangedMode => "hltv_changed_mode",
            GameEventType::HLTVChangedTarget => "hltv_changed_target",
            GameEventType::VoteEnded => "vote_ended",
            GameEventType::VoteStarted => "vote_started",
            GameEventType::VoteChanged => "vote_changed",
            GameEventType::VotePassed => "vote_passed",
            GameEventType::VoteFailed => "vote_failed",
            GameEventType::VoteCast => "vote_cast",
            GameEventType::VoteOptions => "vote_options",
            GameEventType::ReplaySaved => "replay_saved",
            GameEventType::EnteredPerformanceMode => "entered_performance_mode",
            GameEventType::BrowseReplays => "browse_replays",
            GameEventType::ReplayYoutubeStats => "replay_youtube_stats",
            GameEventType::InventoryUpdated => "inventory_updated",
            GameEventType::CartUpdated => "cart_updated",
            GameEventType::StorePriceSheetUpdated => "store_pricesheet_updated",
            GameEventType::EconInventoryConnected => "econ_inventory_connected",
            GameEventType::ItemSchemaInitialized => "item_schema_initialized",
            GameEventType::GcNewSession => "gc_new_session",
            GameEventType::GcLostSession => "gc_lost_session",
            GameEventType::IntroFinish => "intro_finish",
            GameEventType::IntroNextCamera => "intro_nextcamera",
            GameEventType::PlayerChangeClass => "player_changeclass",
            GameEventType::TfMapTimeRemaining => "tf_map_time_remaining",
            GameEventType::TfGameOver => "tf_game_over",
            GameEventType::CtfFlagCaptured => "ctf_flag_captured",
            GameEventType::ControlPointInitialized => "controlpoint_initialized",
            GameEventType::ControlPointUpdateImages => "controlpoint_updateimages",
            GameEventType::ControlPointUpdateLayout => "controlpoint_updatelayout",
            GameEventType::ControlPointUpdateCapping => "controlpoint_updatecapping",
            GameEventType::ControlPointUpdateOwner => "controlpoint_updateowner",
            GameEventType::ControlPointStartTouch => "controlpoint_starttouch",
            GameEventType::ControlPointEndTouch => "controlpoint_endtouch",
            GameEventType::ControlPointPulseElement => "controlpoint_pulse_element",
            GameEventType::ControlPointFakeCapture => "controlpoint_fake_capture",
            GameEventType::ControlPointFakeCaptureMultiplier => {
                "controlpoint_fake_capture_mult"
            }
            GameEventType::TeamPlayRoundSelected => "teamplay_round_selected",
            GameEventType::TeamPlayRoundStart => "teamplay_round_start",
            GameEventType::TeamPlayRoundActive => "teamplay_round_active",
            GameEventType::TeamPlayWaitingBegins => "teamplay_waiting_begins",
            GameEventType::TeamPlayWaitingEnds => "teamplay_waiting_ends",
            GameEventType::TeamPlayWaitingAboutToEnd => "teamplay_waiting_abouttoend",
            GameEventType::TeamPlayRestartRound => "teamplay_restart_round",
            GameEventType::TeamPlayReadyRestart => "teamplay_ready_restart",
            GameEventType::TeamPlayRoundRestartSeconds => {
                "teamplay_round_restart_seconds"
            }
            GameEventType::TeamPlayTeamReady => "teamplay_team_ready",
            GameEventType::TeamPlayRoundWin => "teamplay_round_win",
            GameEventType::TeamPlayUpdateTimer => "teamplay_update_timer",
            GameEventType::TeamPlayRoundStalemate => "teamplay_round_stalemate",
            GameEventType::TeamPlayOvertimeBegin => "teamplay_overtime_begin",
            GameEventType::TeamPlayOvertimeEnd => "teamplay_overtime_end",
            GameEventType::TeamPlaySuddenDeathBegin => "teamplay_suddendeath_begin",
            GameEventType::TeamPlaySuddenDeathEnd => "teamplay_suddendeath_end",
            GameEventType::TeamPlayGameOver => "teamplay_game_over",
            GameEventType::TeamPlayMapTimeRemaining => "teamplay_map_time_remaining",
            GameEventType::TeamPlayTimerFlash => "teamplay_timer_flash",
            GameEventType::TeamPlayTimerTimeAdded => "teamplay_timer_time_added",
            GameEventType::TeamPlayPointStartCapture => "teamplay_point_startcapture",
            GameEventType::TeamPlayPointCaptured => "teamplay_point_captured",
            GameEventType::TeamPlayPointLocked => "teamplay_point_locked",
            GameEventType::TeamPlayPointUnlocked => "teamplay_point_unlocked",
            GameEventType::TeamPlayCaptureBroken => "teamplay_capture_broken",
            GameEventType::TeamPlayCaptureBlocked => "teamplay_capture_blocked",
            GameEventType::TeamPlayFlagEvent => "teamplay_flag_event",
            GameEventType::TeamPlayWinPanel => "teamplay_win_panel",
            GameEventType::TeamPlayTeamBalancedPlayer => "teamplay_teambalanced_player",
            GameEventType::TeamPlaySetupFinished => "teamplay_setup_finished",
            GameEventType::TeamPlayAlert => "teamplay_alert",
            GameEventType::TrainingComplete => "training_complete",
            GameEventType::ShowFreezePanel => "show_freezepanel",
            GameEventType::HideFreezePanel => "hide_freezepanel",
            GameEventType::FreezeCamStarted => "freezecam_started",
            GameEventType::LocalPlayerChangeTeam => "localplayer_changeteam",
            GameEventType::LocalPlayerScoreChanged => "localplayer_score_changed",
            GameEventType::LocalPlayerChangeClass => "localplayer_changeclass",
            GameEventType::LocalPlayerRespawn => "localplayer_respawn",
            GameEventType::BuildingInfoChanged => "building_info_changed",
            GameEventType::LocalPlayerChangeDisguise => "localplayer_changedisguise",
            GameEventType::PlayerAccountChanged => "player_account_changed",
            GameEventType::SpyPdaReset => "spy_pda_reset",
            GameEventType::FlagStatusUpdate => "flagstatus_update",
            GameEventType::PlayerStatsUpdated => "player_stats_updated",
            GameEventType::PlayingCommentary => "playing_commentary",
            GameEventType::PlayerChargeDeployed => "player_chargedeployed",
            GameEventType::PlayerBuiltObject => "player_builtobject",
            GameEventType::PlayerUpgradedObject => "player_upgradedobject",
            GameEventType::PlayerCarryObject => "player_carryobject",
            GameEventType::PlayerDropObject => "player_dropobject",
            GameEventType::ObjectRemoved => "object_removed",
            GameEventType::ObjectDestroyed => "object_destroyed",
            GameEventType::ObjectDetonated => "object_detonated",
            GameEventType::AchievementEarned => "achievement_earned",
            GameEventType::SpecTargetUpdated => "spec_target_updated",
            GameEventType::TournamentStateUpdate => "tournament_stateupdate",
            GameEventType::TournamentEnableCountdown => "tournament_enablecountdown",
            GameEventType::PlayerCalledForMedic => "player_calledformedic",
            GameEventType::PlayerAskedForBall => "player_askedforball",
            GameEventType::LocalPlayerBecameObserver => "localplayer_becameobserver",
            GameEventType::PlayerIgnitedInv => "player_ignited_inv",
            GameEventType::PlayerIgnited => "player_ignited",
            GameEventType::PlayerExtinguished => "player_extinguished",
            GameEventType::PlayerTeleported => "player_teleported",
            GameEventType::PlayerHealedMedicCall => "player_healedmediccall",
            GameEventType::LocalPlayerChargeReady => "localplayer_chargeready",
            GameEventType::LocalPlayerWindDown => "localplayer_winddown",
            GameEventType::PlayerInvulned => "player_invulned",
            GameEventType::EscortSpeed => "escort_speed",
            GameEventType::EscortProgress => "escort_progress",
            GameEventType::EscortRecede => "escort_recede",
            GameEventType::GameUIActivated => "gameui_activated",
            GameEventType::GameUIHidden => "gameui_hidden",
            GameEventType::PlayerEscortScore => "player_escort_score",
            GameEventType::PlayerHealOnHit => "player_healonhit",
            GameEventType::PlayerStealSandvich => "player_stealsandvich",
            GameEventType::ShowClassLayout => "show_class_layout",
            GameEventType::ShowVsPanel => "show_vs_panel",
            GameEventType::PlayerDamaged => "player_damaged",
            GameEventType::ArenaPlayerNotification => "arena_player_notification",
            GameEventType::ArenaMatchMaxStreak => "arena_match_maxstreak",
            GameEventType::ArenaRoundStart => "arena_round_start",
            GameEventType::ArenaWinPanel => "arena_win_panel",
            GameEventType::PveWinPanel => "pve_win_panel",
            GameEventType::AirDash => "air_dash",
            GameEventType::Landed => "landed",
            GameEventType::PlayerDamageDodged => "player_damage_dodged",
            GameEventType::PlayerStunned => "player_stunned",
            GameEventType::ScoutGrandSlam => "scout_grand_slam",
            GameEventType::ScoutSlamdollLanded => "scout_slamdoll_landed",
            GameEventType::ArrowImpact => "arrow_impact",
            GameEventType::PlayerJarated => "player_jarated",
            GameEventType::PlayerJaratedFade => "player_jarated_fade",
            GameEventType::PlayerShieldBlocked => "player_shield_blocked",
            GameEventType::PlayerPinned => "player_pinned",
            GameEventType::PlayerHealedByMedic => "player_healedbymedic",
            GameEventType::PlayerSappedObject => "player_sapped_object",
            GameEventType::ItemFound => "item_found",
            GameEventType::ShowAnnotation => "show_annotation",
            GameEventType::HideAnnotation => "hide_annotation",
            GameEventType::PostInventoryApplication => "post_inventory_application",
            GameEventType::ControlPointUnlockUpdated => "controlpoint_unlock_updated",
            GameEventType::DeployBuffBanner => "deploy_buff_banner",
            GameEventType::PlayerBuff => "player_buff",
            GameEventType::MedicDeath => "medic_death",
            GameEventType::OvertimeNag => "overtime_nag",
            GameEventType::TeamsChanged => "teams_changed",
            GameEventType::HalloweenPumpkinGrab => "halloween_pumpkin_grab",
            GameEventType::RocketJump => "rocket_jump",
            GameEventType::RocketJumpLanded => "rocket_jump_landed",
            GameEventType::StickyJump => "sticky_jump",
            GameEventType::StickyJumpLanded => "sticky_jump_landed",
            GameEventType::RocketPackLaunch => "rocketpack_launch",
            GameEventType::RocketPackLanded => "rocketpack_landed",
            GameEventType::MedicDefended => "medic_defended",
            GameEventType::LocalPlayerHealed => "localplayer_healed",
            GameEventType::PlayerDestroyedPipeBomb => "player_destroyed_pipebomb",
            GameEventType::ObjectDeflected => "object_deflected",
            GameEventType::PlayerMvp => "player_mvp",
            GameEventType::RaidSpawnMob => "raid_spawn_mob",
            GameEventType::RaidSpawnSquad => "raid_spawn_squad",
            GameEventType::NavBlocked => "nav_blocked",
            GameEventType::PathTrackPassed => "path_track_passed",
            GameEventType::NumCappersChanged => "num_cappers_changed",
            GameEventType::PlayerRegenerate => "player_regenerate",
            GameEventType::UpdateStatusItem => "update_status_item",
            GameEventType::StatsResetRound => "stats_resetround",
            GameEventType::ScoreStatsAccumulatedUpdate => "scorestats_accumulated_update",
            GameEventType::ScoreStatsAccumulatedReset => "scorestats_accumulated_reset",
            GameEventType::AchievementEarnedLocal => "achievement_earned_local",
            GameEventType::PlayerHealed => "player_healed",
            GameEventType::BuildingHealed => "building_healed",
            GameEventType::ItemPickup => "item_pickup",
            GameEventType::DuelStatus => "duel_status",
            GameEventType::FishNotice => "fish_notice",
            GameEventType::FishNoticeArm => "fish_notice__arm",
            GameEventType::SlapNotice => "slap_notice",
            GameEventType::ThrowableHit => "throwable_hit",
            GameEventType::PumpkinLordSummoned => "pumpkin_lord_summoned",
            GameEventType::PumpkinLordKilled => "pumpkin_lord_killed",
            GameEventType::MerasmusSummoned => "merasmus_summoned",
            GameEventType::MerasmusKilled => "merasmus_killed",
            GameEventType::MerasmusEscapeWarning => "merasmus_escape_warning",
            GameEventType::MerasmusEscaped => "merasmus_escaped",
            GameEventType::EyeballBossSummoned => "eyeball_boss_summoned",
            GameEventType::EyeballBossStunned => "eyeball_boss_stunned",
            GameEventType::EyeballBossKilled => "eyeball_boss_killed",
            GameEventType::EyeballBossKiller => "eyeball_boss_killer",
            GameEventType::EyeballBossEscapeImminent => "eyeball_boss_escape_imminent",
            GameEventType::EyeballBossEscaped => "eyeball_boss_escaped",
            GameEventType::NpcHurt => "npc_hurt",
            GameEventType::ControlPointTimerUpdated => "controlpoint_timer_updated",
            GameEventType::PlayerHighFiveStart => "player_highfive_start",
            GameEventType::PlayerHighFiveCancel => "player_highfive_cancel",
            GameEventType::PlayerHighFiveSuccess => "player_highfive_success",
            GameEventType::PlayerBonusPoints => "player_bonuspoints",
            GameEventType::PlayerUpgraded => "player_upgraded",
            GameEventType::PlayerBuyback => "player_buyback",
            GameEventType::PlayerUsedPowerUpBottle => "player_used_powerup_bottle",
            GameEventType::ChristmasGiftGrab => "christmas_gift_grab",
            GameEventType::PlayerKilledAchievementZone => {
                "player_killed_achievement_zone"
            }
            GameEventType::PartyUpdated => "party_updated",
            GameEventType::PartyPrefChanged => "party_pref_changed",
            GameEventType::PartyCriteriaChanged => "party_criteria_changed",
            GameEventType::PartyInvitesChanged => "party_invites_changed",
            GameEventType::PartyQueueStateChanged => "party_queue_state_changed",
            GameEventType::PartyChat => "party_chat",
            GameEventType::PartyMemberJoin => "party_member_join",
            GameEventType::PartyMemberLeave => "party_member_leave",
            GameEventType::MatchInvitesUpdated => "match_invites_updated",
            GameEventType::LobbyUpdated => "lobby_updated",
            GameEventType::MvmMissionUpdate => "mvm_mission_update",
            GameEventType::RecalculateHolidays => "recalculate_holidays",
            GameEventType::PlayerCurrencyChanged => "player_currency_changed",
            GameEventType::DoomsdayRocketOpen => "doomsday_rocket_open",
            GameEventType::RemoveNemesisRelationships => "remove_nemesis_relationships",
            GameEventType::MvmCreditBonusWave => "mvm_creditbonus_wave",
            GameEventType::MvmCreditBonusAll => "mvm_creditbonus_all",
            GameEventType::MvmCreditBonusAllAdvanced => "mvm_creditbonus_all_advanced",
            GameEventType::MvmQuickSentryUpgrade => "mvm_quick_sentry_upgrade",
            GameEventType::MvmTankDestroyedByPlayers => "mvm_tank_destroyed_by_players",
            GameEventType::MvmKillRobotDeliveringBomb => "mvm_kill_robot_delivering_bomb",
            GameEventType::MvmPickupCurrency => "mvm_pickup_currency",
            GameEventType::MvmBombCarrierKilled => "mvm_bomb_carrier_killed",
            GameEventType::MvmSentryBusterDetonate => "mvm_sentrybuster_detonate",
            GameEventType::MvmScoutMarkedForDeath => "mvm_scout_marked_for_death",
            GameEventType::MvmMedicPowerUpShared => "mvm_medic_powerup_shared",
            GameEventType::MvmBeginWave => "mvm_begin_wave",
            GameEventType::MvmWaveComplete => "mvm_wave_complete",
            GameEventType::MvmMissionComplete => "mvm_mission_complete",
            GameEventType::MvmBombResetByPlayer => "mvm_bomb_reset_by_player",
            GameEventType::MvmBombAlarmTriggered => "mvm_bomb_alarm_triggered",
            GameEventType::MvmBombDeployResetByPlayer => {
                "mvm_bomb_deploy_reset_by_player"
            }
            GameEventType::MvmWaveFailed => "mvm_wave_failed",
            GameEventType::MvmResetStats => "mvm_reset_stats",
            GameEventType::DamageResisted => "damage_resisted",
            GameEventType::RevivePlayerNotify => "revive_player_notify",
            GameEventType::RevivePlayerStopped => "revive_player_stopped",
            GameEventType::RevivePlayerComplete => "revive_player_complete",
            GameEventType::PlayerTurnedToGhost => "player_turned_to_ghost",
            GameEventType::MedigunShieldBlockedDamage => "medigun_shield_blocked_damage",
            GameEventType::MvmAdvWaveCompleteNoGates => "mvm_adv_wave_complete_no_gates",
            GameEventType::MvmSniperHeadshotCurrency => "mvm_sniper_headshot_currency",
            GameEventType::MvmMannhattanPit => "mvm_mannhattan_pit",
            GameEventType::FlagCarriedInDetectionZone => "flag_carried_in_detection_zone",
            GameEventType::MvmAdvWaveKilledStunRadio => "mvm_adv_wave_killed_stun_radio",
            GameEventType::PlayerDirectHitStun => "player_directhit_stun",
            GameEventType::MvmSentryBusterKilled => "mvm_sentrybuster_killed",
            GameEventType::UpgradesFileChanged => "upgrades_file_changed",
            GameEventType::RdTeamPointsChanged => "rd_team_points_changed",
            GameEventType::RdRulesStateChanged => "rd_rules_state_changed",
            GameEventType::RdRobotKilled => "rd_robot_killed",
            GameEventType::RdRobotImpact => "rd_robot_impact",
            GameEventType::TeamPlayPreRoundTimeLeft => "teamplay_pre_round_time_left",
            GameEventType::ParachuteDeploy => "parachute_deploy",
            GameEventType::ParachuteHolster => "parachute_holster",
            GameEventType::KillRefillsMeter => "kill_refills_meter",
            GameEventType::RpsTauntEvent => "rps_taunt_event",
            GameEventType::CongaKill => "conga_kill",
            GameEventType::PlayerInitialSpawn => "player_initial_spawn",
            GameEventType::CompetitiveVictory => "competitive_victory",
            GameEventType::CompetitiveStatsUpdate => "competitive_stats_update",
            GameEventType::MiniGameWin => "minigame_win",
            GameEventType::SentryOnGoActive => "sentry_on_go_active",
            GameEventType::DuckXpLevelUp => "duck_xp_level_up",
            GameEventType::QuestLogOpened => "questlog_opened",
            GameEventType::SchemaUpdated => "schema_updated",
            GameEventType::LocalPlayerPickupWeapon => "localplayer_pickup_weapon",
            GameEventType::RdPlayerScorePoints => "rd_player_score_points",
            GameEventType::DemomanDetStickies => "demoman_det_stickies",
            GameEventType::QuestObjectiveCompleted => "quest_objective_completed",
            GameEventType::PlayerScoreChanged => "player_score_changed",
            GameEventType::KilledCappingPlayer => "killed_capping_player",
            GameEventType::EnvironmentalDeath => "environmental_death",
            GameEventType::ProjectileDirectHit => "projectile_direct_hit",
            GameEventType::PassGet => "pass_get",
            GameEventType::PassScore => "pass_score",
            GameEventType::PassFree => "pass_free",
            GameEventType::PassPassCaught => "pass_pass_caught",
            GameEventType::PassBallStolen => "pass_ball_stolen",
            GameEventType::PassBallBlocked => "pass_ball_blocked",
            GameEventType::DamagePrevented => "damage_prevented",
            GameEventType::HalloweenBossKilled => "halloween_boss_killed",
            GameEventType::EscapedLootIsland => "escaped_loot_island",
            GameEventType::TaggedPlayerAsIt => "tagged_player_as_it",
            GameEventType::MerasmusStunned => "merasmus_stunned",
            GameEventType::MerasmusPropFound => "merasmus_prop_found",
            GameEventType::HalloweenSkeletonKilled => "halloween_skeleton_killed",
            GameEventType::EscapeHell => "escape_hell",
            GameEventType::CrossSpectralBridge => "cross_spectral_bridge",
            GameEventType::MiniGameWon => "minigame_won",
            GameEventType::RespawnGhost => "respawn_ghost",
            GameEventType::KillInHell => "kill_in_hell",
            GameEventType::HalloweenDuckCollected => "halloween_duck_collected",
            GameEventType::SpecialScore => "special_score",
            GameEventType::TeamLeaderKilled => "team_leader_killed",
            GameEventType::HalloweenSoulCollected => "halloween_soul_collected",
            GameEventType::RecalculateTruce => "recalculate_truce",
            GameEventType::DeadRingerCheatDeath => "deadringer_cheat_death",
            GameEventType::CrossbowHeal => "crossbow_heal",
            GameEventType::DamageMitigated => "damage_mitigated",
            GameEventType::PayloadPushed => "payload_pushed",
            GameEventType::PlayerAbandonedMatch => "player_abandoned_match",
            GameEventType::ClDrawline => "cl_drawline",
            GameEventType::RestartTimerTime => "restart_timer_time",
            GameEventType::WinLimitChanged => "winlimit_changed",
            GameEventType::WinPanelShowScores => "winpanel_show_scores",
            GameEventType::TopStreamsRequestFinished => "top_streams_request_finished",
            GameEventType::CompetitiveStateChanged => "competitive_state_changed",
            GameEventType::GlobalWarDataUpdated => "global_war_data_updated",
            GameEventType::StopWatchChanged => "stop_watch_changed",
            GameEventType::DsStop => "ds_stop",
            GameEventType::DsScreenshot => "ds_screenshot",
            GameEventType::ShowMatchSummary => "show_match_summary",
            GameEventType::ExperienceChanged => "experience_changed",
            GameEventType::BeginXpLerp => "begin_xp_lerp",
            GameEventType::MatchmakerStatsUpdated => "matchmaker_stats_updated",
            GameEventType::RematchVotePeriodOver => "rematch_vote_period_over",
            GameEventType::RematchFailedToCreate => "rematch_failed_to_create",
            GameEventType::PlayerRematchChange => "player_rematch_change",
            GameEventType::PingUpdated => "ping_updated",
            GameEventType::MMStatsUpdated => "mmstats_updated",
            GameEventType::PlayerNextMapVoteChange => "player_next_map_vote_change",
            GameEventType::VoteMapsChanged => "vote_maps_changed",
            GameEventType::ProtoDefChanged => "proto_def_changed",
            GameEventType::PlayerDomination => "player_domination",
            GameEventType::PlayerRocketPackPushed => "player_rocketpack_pushed",
            GameEventType::QuestRequest => "quest_request",
            GameEventType::QuestResponse => "quest_response",
            GameEventType::QuestProgress => "quest_progress",
            GameEventType::ProjectileRemoved => "projectile_removed",
            GameEventType::QuestMapDataChanged => "quest_map_data_changed",
            GameEventType::GasDousedPlayerIgnited => "gas_doused_player_ignited",
            GameEventType::QuestTurnInState => "quest_turn_in_state",
            GameEventType::ItemsAcknowledged => "items_acknowledged",
            GameEventType::CapperKilled => "capper_killed",
            GameEventType::MainMenuStabilized => "mainmenu_stabilized",
            GameEventType::WorldStatusChanged => "world_status_changed",
            GameEventType::HLTVStatus => "hltv_status",
            GameEventType::HLTVCameraman => "hltv_cameraman",
            GameEventType::HLTVRankCamera => "hltv_rank_camera",
            GameEventType::HLTVRankEntity => "hltv_rank_entity",
            GameEventType::HLTVFixed => "hltv_fixed",
            GameEventType::HLTVChase => "hltv_chase",
            GameEventType::HLTVMessage => "hltv_message",
            GameEventType::HLTVTitle => "hltv_title",
            GameEventType::HLTVChat => "hltv_chat",
            GameEventType::ReplayStartRecord => "replay_startrecord",
            GameEventType::ReplaySessionInfo => "replay_sessioninfo",
            GameEventType::ReplayEndRecord => "replay_endrecord",
            GameEventType::ReplayReplaysAvailable => "replay_replaysavailable",
            GameEventType::ReplayServerError => "replay_servererror",
            GameEventType::Unknown(ty) => ty,
        }
    }
}
impl GameEvent {
    pub fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(
            match definition.event_type {
                GameEventType::ServerSpawn => {
                    GameEvent::ServerSpawn(
                        Box::new(<ServerSpawnEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::ServerChangeLevelFailed => {
                    GameEvent::ServerChangeLevelFailed(
                        ServerChangeLevelFailedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ServerShutdown => {
                    GameEvent::ServerShutdown(
                        ServerShutdownEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ServerCvar => {
                    GameEvent::ServerCvar(ServerCvarEvent::read(stream, definition)?)
                }
                GameEventType::ServerMessage => {
                    GameEvent::ServerMessage(
                        ServerMessageEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ServerAddBan => {
                    GameEvent::ServerAddBan(
                        Box::new(<ServerAddBanEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::ServerRemoveBan => {
                    GameEvent::ServerRemoveBan(
                        ServerRemoveBanEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerConnect => {
                    GameEvent::PlayerConnect(
                        PlayerConnectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerConnectClient => {
                    GameEvent::PlayerConnectClient(
                        PlayerConnectClientEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerInfo => {
                    GameEvent::PlayerInfo(PlayerInfoEvent::read(stream, definition)?)
                }
                GameEventType::PlayerDisconnect => {
                    GameEvent::PlayerDisconnect(
                        PlayerDisconnectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerActivate => {
                    GameEvent::PlayerActivate(
                        PlayerActivateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerSay => {
                    GameEvent::PlayerSay(PlayerSayEvent::read(stream, definition)?)
                }
                GameEventType::ClientDisconnect => {
                    GameEvent::ClientDisconnect(
                        ClientDisconnectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ClientBeginConnect => {
                    GameEvent::ClientBeginConnect(
                        ClientBeginConnectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ClientConnected => {
                    GameEvent::ClientConnected(
                        ClientConnectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ClientFullConnect => {
                    GameEvent::ClientFullConnect(
                        ClientFullConnectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HostQuit => {
                    GameEvent::HostQuit(HostQuitEvent::read(stream, definition)?)
                }
                GameEventType::TeamInfo => {
                    GameEvent::TeamInfo(TeamInfoEvent::read(stream, definition)?)
                }
                GameEventType::TeamScore => {
                    GameEvent::TeamScore(TeamScoreEvent::read(stream, definition)?)
                }
                GameEventType::TeamPlayBroadcastAudio => {
                    GameEvent::TeamPlayBroadcastAudio(
                        TeamPlayBroadcastAudioEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerTeam => {
                    GameEvent::PlayerTeam(PlayerTeamEvent::read(stream, definition)?)
                }
                GameEventType::PlayerClass => {
                    GameEvent::PlayerClass(PlayerClassEvent::read(stream, definition)?)
                }
                GameEventType::PlayerDeath => {
                    GameEvent::PlayerDeath(
                        Box::new(<PlayerDeathEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::PlayerHurt => {
                    GameEvent::PlayerHurt(PlayerHurtEvent::read(stream, definition)?)
                }
                GameEventType::PlayerChat => {
                    GameEvent::PlayerChat(PlayerChatEvent::read(stream, definition)?)
                }
                GameEventType::PlayerScore => {
                    GameEvent::PlayerScore(PlayerScoreEvent::read(stream, definition)?)
                }
                GameEventType::PlayerSpawn => {
                    GameEvent::PlayerSpawn(PlayerSpawnEvent::read(stream, definition)?)
                }
                GameEventType::PlayerShoot => {
                    GameEvent::PlayerShoot(PlayerShootEvent::read(stream, definition)?)
                }
                GameEventType::PlayerUse => {
                    GameEvent::PlayerUse(PlayerUseEvent::read(stream, definition)?)
                }
                GameEventType::PlayerChangeName => {
                    GameEvent::PlayerChangeName(
                        PlayerChangeNameEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHintMessage => {
                    GameEvent::PlayerHintMessage(
                        PlayerHintMessageEvent::read(stream, definition)?,
                    )
                }
                GameEventType::BasePlayerTeleported => {
                    GameEvent::BasePlayerTeleported(
                        BasePlayerTeleportedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::GameInit => {
                    GameEvent::GameInit(GameInitEvent::read(stream, definition)?)
                }
                GameEventType::GameNewMap => {
                    GameEvent::GameNewMap(GameNewMapEvent::read(stream, definition)?)
                }
                GameEventType::GameStart => {
                    GameEvent::GameStart(GameStartEvent::read(stream, definition)?)
                }
                GameEventType::GameEnd => {
                    GameEvent::GameEnd(GameEndEvent::read(stream, definition)?)
                }
                GameEventType::RoundStart => {
                    GameEvent::RoundStart(RoundStartEvent::read(stream, definition)?)
                }
                GameEventType::RoundEnd => {
                    GameEvent::RoundEnd(RoundEndEvent::read(stream, definition)?)
                }
                GameEventType::GameMessage => {
                    GameEvent::GameMessage(GameMessageEvent::read(stream, definition)?)
                }
                GameEventType::BreakBreakable => {
                    GameEvent::BreakBreakable(
                        BreakBreakableEvent::read(stream, definition)?,
                    )
                }
                GameEventType::BreakProp => {
                    GameEvent::BreakProp(BreakPropEvent::read(stream, definition)?)
                }
                GameEventType::EntityKilled => {
                    GameEvent::EntityKilled(EntityKilledEvent::read(stream, definition)?)
                }
                GameEventType::BonusUpdated => {
                    GameEvent::BonusUpdated(BonusUpdatedEvent::read(stream, definition)?)
                }
                GameEventType::AchievementEvent => {
                    GameEvent::AchievementEvent(
                        AchievementEventEvent::read(stream, definition)?,
                    )
                }
                GameEventType::AchievementIncrement => {
                    GameEvent::AchievementIncrement(
                        AchievementIncrementEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PhysgunPickup => {
                    GameEvent::PhysgunPickup(
                        PhysgunPickupEvent::read(stream, definition)?,
                    )
                }
                GameEventType::FlareIgniteNpc => {
                    GameEvent::FlareIgniteNpc(
                        FlareIgniteNpcEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HelicopterGrenadePuntMiss => {
                    GameEvent::HelicopterGrenadePuntMiss(
                        HelicopterGrenadePuntMissEvent::read(stream, definition)?,
                    )
                }
                GameEventType::UserDataDownloaded => {
                    GameEvent::UserDataDownloaded(
                        UserDataDownloadedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RagdollDissolved => {
                    GameEvent::RagdollDissolved(
                        RagdollDissolvedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVChangedMode => {
                    GameEvent::HLTVChangedMode(
                        HLTVChangedModeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVChangedTarget => {
                    GameEvent::HLTVChangedTarget(
                        HLTVChangedTargetEvent::read(stream, definition)?,
                    )
                }
                GameEventType::VoteEnded => {
                    GameEvent::VoteEnded(VoteEndedEvent::read(stream, definition)?)
                }
                GameEventType::VoteStarted => {
                    GameEvent::VoteStarted(VoteStartedEvent::read(stream, definition)?)
                }
                GameEventType::VoteChanged => {
                    GameEvent::VoteChanged(VoteChangedEvent::read(stream, definition)?)
                }
                GameEventType::VotePassed => {
                    GameEvent::VotePassed(VotePassedEvent::read(stream, definition)?)
                }
                GameEventType::VoteFailed => {
                    GameEvent::VoteFailed(VoteFailedEvent::read(stream, definition)?)
                }
                GameEventType::VoteCast => {
                    GameEvent::VoteCast(VoteCastEvent::read(stream, definition)?)
                }
                GameEventType::VoteOptions => {
                    GameEvent::VoteOptions(
                        Box::new(<VoteOptionsEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::ReplaySaved => {
                    GameEvent::ReplaySaved(ReplaySavedEvent::read(stream, definition)?)
                }
                GameEventType::EnteredPerformanceMode => {
                    GameEvent::EnteredPerformanceMode(
                        EnteredPerformanceModeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::BrowseReplays => {
                    GameEvent::BrowseReplays(
                        BrowseReplaysEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ReplayYoutubeStats => {
                    GameEvent::ReplayYoutubeStats(
                        ReplayYoutubeStatsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::InventoryUpdated => {
                    GameEvent::InventoryUpdated(
                        InventoryUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CartUpdated => {
                    GameEvent::CartUpdated(CartUpdatedEvent::read(stream, definition)?)
                }
                GameEventType::StorePriceSheetUpdated => {
                    GameEvent::StorePriceSheetUpdated(
                        StorePriceSheetUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EconInventoryConnected => {
                    GameEvent::EconInventoryConnected(
                        EconInventoryConnectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ItemSchemaInitialized => {
                    GameEvent::ItemSchemaInitialized(
                        ItemSchemaInitializedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::GcNewSession => {
                    GameEvent::GcNewSession(GcNewSessionEvent::read(stream, definition)?)
                }
                GameEventType::GcLostSession => {
                    GameEvent::GcLostSession(
                        GcLostSessionEvent::read(stream, definition)?,
                    )
                }
                GameEventType::IntroFinish => {
                    GameEvent::IntroFinish(IntroFinishEvent::read(stream, definition)?)
                }
                GameEventType::IntroNextCamera => {
                    GameEvent::IntroNextCamera(
                        IntroNextCameraEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerChangeClass => {
                    GameEvent::PlayerChangeClass(
                        PlayerChangeClassEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TfMapTimeRemaining => {
                    GameEvent::TfMapTimeRemaining(
                        TfMapTimeRemainingEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TfGameOver => {
                    GameEvent::TfGameOver(TfGameOverEvent::read(stream, definition)?)
                }
                GameEventType::CtfFlagCaptured => {
                    GameEvent::CtfFlagCaptured(
                        CtfFlagCapturedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointInitialized => {
                    GameEvent::ControlPointInitialized(
                        ControlPointInitializedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointUpdateImages => {
                    GameEvent::ControlPointUpdateImages(
                        ControlPointUpdateImagesEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointUpdateLayout => {
                    GameEvent::ControlPointUpdateLayout(
                        ControlPointUpdateLayoutEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointUpdateCapping => {
                    GameEvent::ControlPointUpdateCapping(
                        ControlPointUpdateCappingEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointUpdateOwner => {
                    GameEvent::ControlPointUpdateOwner(
                        ControlPointUpdateOwnerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointStartTouch => {
                    GameEvent::ControlPointStartTouch(
                        ControlPointStartTouchEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointEndTouch => {
                    GameEvent::ControlPointEndTouch(
                        ControlPointEndTouchEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointPulseElement => {
                    GameEvent::ControlPointPulseElement(
                        ControlPointPulseElementEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointFakeCapture => {
                    GameEvent::ControlPointFakeCapture(
                        ControlPointFakeCaptureEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointFakeCaptureMultiplier => {
                    GameEvent::ControlPointFakeCaptureMultiplier(
                        ControlPointFakeCaptureMultiplierEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundSelected => {
                    GameEvent::TeamPlayRoundSelected(
                        TeamPlayRoundSelectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundStart => {
                    GameEvent::TeamPlayRoundStart(
                        TeamPlayRoundStartEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundActive => {
                    GameEvent::TeamPlayRoundActive(
                        TeamPlayRoundActiveEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayWaitingBegins => {
                    GameEvent::TeamPlayWaitingBegins(
                        TeamPlayWaitingBeginsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayWaitingEnds => {
                    GameEvent::TeamPlayWaitingEnds(
                        TeamPlayWaitingEndsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayWaitingAboutToEnd => {
                    GameEvent::TeamPlayWaitingAboutToEnd(
                        TeamPlayWaitingAboutToEndEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRestartRound => {
                    GameEvent::TeamPlayRestartRound(
                        TeamPlayRestartRoundEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayReadyRestart => {
                    GameEvent::TeamPlayReadyRestart(
                        TeamPlayReadyRestartEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundRestartSeconds => {
                    GameEvent::TeamPlayRoundRestartSeconds(
                        TeamPlayRoundRestartSecondsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayTeamReady => {
                    GameEvent::TeamPlayTeamReady(
                        TeamPlayTeamReadyEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundWin => {
                    GameEvent::TeamPlayRoundWin(
                        TeamPlayRoundWinEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayUpdateTimer => {
                    GameEvent::TeamPlayUpdateTimer(
                        TeamPlayUpdateTimerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayRoundStalemate => {
                    GameEvent::TeamPlayRoundStalemate(
                        TeamPlayRoundStalemateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayOvertimeBegin => {
                    GameEvent::TeamPlayOvertimeBegin(
                        TeamPlayOvertimeBeginEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayOvertimeEnd => {
                    GameEvent::TeamPlayOvertimeEnd(
                        TeamPlayOvertimeEndEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlaySuddenDeathBegin => {
                    GameEvent::TeamPlaySuddenDeathBegin(
                        TeamPlaySuddenDeathBeginEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlaySuddenDeathEnd => {
                    GameEvent::TeamPlaySuddenDeathEnd(
                        TeamPlaySuddenDeathEndEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayGameOver => {
                    GameEvent::TeamPlayGameOver(
                        TeamPlayGameOverEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayMapTimeRemaining => {
                    GameEvent::TeamPlayMapTimeRemaining(
                        TeamPlayMapTimeRemainingEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayTimerFlash => {
                    GameEvent::TeamPlayTimerFlash(
                        TeamPlayTimerFlashEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayTimerTimeAdded => {
                    GameEvent::TeamPlayTimerTimeAdded(
                        TeamPlayTimerTimeAddedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayPointStartCapture => {
                    GameEvent::TeamPlayPointStartCapture(
                        TeamPlayPointStartCaptureEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayPointCaptured => {
                    GameEvent::TeamPlayPointCaptured(
                        TeamPlayPointCapturedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayPointLocked => {
                    GameEvent::TeamPlayPointLocked(
                        TeamPlayPointLockedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayPointUnlocked => {
                    GameEvent::TeamPlayPointUnlocked(
                        TeamPlayPointUnlockedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayCaptureBroken => {
                    GameEvent::TeamPlayCaptureBroken(
                        TeamPlayCaptureBrokenEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayCaptureBlocked => {
                    GameEvent::TeamPlayCaptureBlocked(
                        TeamPlayCaptureBlockedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayFlagEvent => {
                    GameEvent::TeamPlayFlagEvent(
                        TeamPlayFlagEventEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayWinPanel => {
                    GameEvent::TeamPlayWinPanel(
                        TeamPlayWinPanelEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayTeamBalancedPlayer => {
                    GameEvent::TeamPlayTeamBalancedPlayer(
                        TeamPlayTeamBalancedPlayerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlaySetupFinished => {
                    GameEvent::TeamPlaySetupFinished(
                        TeamPlaySetupFinishedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayAlert => {
                    GameEvent::TeamPlayAlert(
                        TeamPlayAlertEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TrainingComplete => {
                    GameEvent::TrainingComplete(
                        TrainingCompleteEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ShowFreezePanel => {
                    GameEvent::ShowFreezePanel(
                        ShowFreezePanelEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HideFreezePanel => {
                    GameEvent::HideFreezePanel(
                        HideFreezePanelEvent::read(stream, definition)?,
                    )
                }
                GameEventType::FreezeCamStarted => {
                    GameEvent::FreezeCamStarted(
                        FreezeCamStartedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerChangeTeam => {
                    GameEvent::LocalPlayerChangeTeam(
                        LocalPlayerChangeTeamEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerScoreChanged => {
                    GameEvent::LocalPlayerScoreChanged(
                        LocalPlayerScoreChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerChangeClass => {
                    GameEvent::LocalPlayerChangeClass(
                        LocalPlayerChangeClassEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerRespawn => {
                    GameEvent::LocalPlayerRespawn(
                        LocalPlayerRespawnEvent::read(stream, definition)?,
                    )
                }
                GameEventType::BuildingInfoChanged => {
                    GameEvent::BuildingInfoChanged(
                        BuildingInfoChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerChangeDisguise => {
                    GameEvent::LocalPlayerChangeDisguise(
                        LocalPlayerChangeDisguiseEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerAccountChanged => {
                    GameEvent::PlayerAccountChanged(
                        PlayerAccountChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::SpyPdaReset => {
                    GameEvent::SpyPdaReset(SpyPdaResetEvent::read(stream, definition)?)
                }
                GameEventType::FlagStatusUpdate => {
                    GameEvent::FlagStatusUpdate(
                        FlagStatusUpdateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerStatsUpdated => {
                    GameEvent::PlayerStatsUpdated(
                        PlayerStatsUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayingCommentary => {
                    GameEvent::PlayingCommentary(
                        PlayingCommentaryEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerChargeDeployed => {
                    GameEvent::PlayerChargeDeployed(
                        PlayerChargeDeployedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerBuiltObject => {
                    GameEvent::PlayerBuiltObject(
                        PlayerBuiltObjectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerUpgradedObject => {
                    GameEvent::PlayerUpgradedObject(
                        PlayerUpgradedObjectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerCarryObject => {
                    GameEvent::PlayerCarryObject(
                        PlayerCarryObjectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerDropObject => {
                    GameEvent::PlayerDropObject(
                        PlayerDropObjectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ObjectRemoved => {
                    GameEvent::ObjectRemoved(
                        ObjectRemovedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ObjectDestroyed => {
                    GameEvent::ObjectDestroyed(
                        ObjectDestroyedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ObjectDetonated => {
                    GameEvent::ObjectDetonated(
                        ObjectDetonatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::AchievementEarned => {
                    GameEvent::AchievementEarned(
                        AchievementEarnedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::SpecTargetUpdated => {
                    GameEvent::SpecTargetUpdated(
                        SpecTargetUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TournamentStateUpdate => {
                    GameEvent::TournamentStateUpdate(
                        TournamentStateUpdateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TournamentEnableCountdown => {
                    GameEvent::TournamentEnableCountdown(
                        TournamentEnableCountdownEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerCalledForMedic => {
                    GameEvent::PlayerCalledForMedic(
                        PlayerCalledForMedicEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerAskedForBall => {
                    GameEvent::PlayerAskedForBall(
                        PlayerAskedForBallEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerBecameObserver => {
                    GameEvent::LocalPlayerBecameObserver(
                        LocalPlayerBecameObserverEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerIgnitedInv => {
                    GameEvent::PlayerIgnitedInv(
                        PlayerIgnitedInvEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerIgnited => {
                    GameEvent::PlayerIgnited(
                        PlayerIgnitedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerExtinguished => {
                    GameEvent::PlayerExtinguished(
                        PlayerExtinguishedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerTeleported => {
                    GameEvent::PlayerTeleported(
                        PlayerTeleportedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHealedMedicCall => {
                    GameEvent::PlayerHealedMedicCall(
                        PlayerHealedMedicCallEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerChargeReady => {
                    GameEvent::LocalPlayerChargeReady(
                        LocalPlayerChargeReadyEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerWindDown => {
                    GameEvent::LocalPlayerWindDown(
                        LocalPlayerWindDownEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerInvulned => {
                    GameEvent::PlayerInvulned(
                        PlayerInvulnedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EscortSpeed => {
                    GameEvent::EscortSpeed(EscortSpeedEvent::read(stream, definition)?)
                }
                GameEventType::EscortProgress => {
                    GameEvent::EscortProgress(
                        EscortProgressEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EscortRecede => {
                    GameEvent::EscortRecede(EscortRecedeEvent::read(stream, definition)?)
                }
                GameEventType::GameUIActivated => {
                    GameEvent::GameUIActivated(
                        GameUIActivatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::GameUIHidden => {
                    GameEvent::GameUIHidden(GameUIHiddenEvent::read(stream, definition)?)
                }
                GameEventType::PlayerEscortScore => {
                    GameEvent::PlayerEscortScore(
                        PlayerEscortScoreEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHealOnHit => {
                    GameEvent::PlayerHealOnHit(
                        PlayerHealOnHitEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerStealSandvich => {
                    GameEvent::PlayerStealSandvich(
                        PlayerStealSandvichEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ShowClassLayout => {
                    GameEvent::ShowClassLayout(
                        ShowClassLayoutEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ShowVsPanel => {
                    GameEvent::ShowVsPanel(ShowVsPanelEvent::read(stream, definition)?)
                }
                GameEventType::PlayerDamaged => {
                    GameEvent::PlayerDamaged(
                        PlayerDamagedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ArenaPlayerNotification => {
                    GameEvent::ArenaPlayerNotification(
                        ArenaPlayerNotificationEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ArenaMatchMaxStreak => {
                    GameEvent::ArenaMatchMaxStreak(
                        ArenaMatchMaxStreakEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ArenaRoundStart => {
                    GameEvent::ArenaRoundStart(
                        ArenaRoundStartEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ArenaWinPanel => {
                    GameEvent::ArenaWinPanel(
                        ArenaWinPanelEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PveWinPanel => {
                    GameEvent::PveWinPanel(PveWinPanelEvent::read(stream, definition)?)
                }
                GameEventType::AirDash => {
                    GameEvent::AirDash(AirDashEvent::read(stream, definition)?)
                }
                GameEventType::Landed => {
                    GameEvent::Landed(LandedEvent::read(stream, definition)?)
                }
                GameEventType::PlayerDamageDodged => {
                    GameEvent::PlayerDamageDodged(
                        PlayerDamageDodgedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerStunned => {
                    GameEvent::PlayerStunned(
                        PlayerStunnedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ScoutGrandSlam => {
                    GameEvent::ScoutGrandSlam(
                        ScoutGrandSlamEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ScoutSlamdollLanded => {
                    GameEvent::ScoutSlamdollLanded(
                        ScoutSlamdollLandedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ArrowImpact => {
                    GameEvent::ArrowImpact(ArrowImpactEvent::read(stream, definition)?)
                }
                GameEventType::PlayerJarated => {
                    GameEvent::PlayerJarated(
                        PlayerJaratedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerJaratedFade => {
                    GameEvent::PlayerJaratedFade(
                        PlayerJaratedFadeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerShieldBlocked => {
                    GameEvent::PlayerShieldBlocked(
                        PlayerShieldBlockedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerPinned => {
                    GameEvent::PlayerPinned(PlayerPinnedEvent::read(stream, definition)?)
                }
                GameEventType::PlayerHealedByMedic => {
                    GameEvent::PlayerHealedByMedic(
                        PlayerHealedByMedicEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerSappedObject => {
                    GameEvent::PlayerSappedObject(
                        PlayerSappedObjectEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ItemFound => {
                    GameEvent::ItemFound(ItemFoundEvent::read(stream, definition)?)
                }
                GameEventType::ShowAnnotation => {
                    GameEvent::ShowAnnotation(
                        ShowAnnotationEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HideAnnotation => {
                    GameEvent::HideAnnotation(
                        HideAnnotationEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PostInventoryApplication => {
                    GameEvent::PostInventoryApplication(
                        PostInventoryApplicationEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ControlPointUnlockUpdated => {
                    GameEvent::ControlPointUnlockUpdated(
                        ControlPointUnlockUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DeployBuffBanner => {
                    GameEvent::DeployBuffBanner(
                        DeployBuffBannerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerBuff => {
                    GameEvent::PlayerBuff(PlayerBuffEvent::read(stream, definition)?)
                }
                GameEventType::MedicDeath => {
                    GameEvent::MedicDeath(MedicDeathEvent::read(stream, definition)?)
                }
                GameEventType::OvertimeNag => {
                    GameEvent::OvertimeNag(OvertimeNagEvent::read(stream, definition)?)
                }
                GameEventType::TeamsChanged => {
                    GameEvent::TeamsChanged(TeamsChangedEvent::read(stream, definition)?)
                }
                GameEventType::HalloweenPumpkinGrab => {
                    GameEvent::HalloweenPumpkinGrab(
                        HalloweenPumpkinGrabEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RocketJump => {
                    GameEvent::RocketJump(RocketJumpEvent::read(stream, definition)?)
                }
                GameEventType::RocketJumpLanded => {
                    GameEvent::RocketJumpLanded(
                        RocketJumpLandedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::StickyJump => {
                    GameEvent::StickyJump(StickyJumpEvent::read(stream, definition)?)
                }
                GameEventType::StickyJumpLanded => {
                    GameEvent::StickyJumpLanded(
                        StickyJumpLandedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RocketPackLaunch => {
                    GameEvent::RocketPackLaunch(
                        RocketPackLaunchEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RocketPackLanded => {
                    GameEvent::RocketPackLanded(
                        RocketPackLandedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MedicDefended => {
                    GameEvent::MedicDefended(
                        MedicDefendedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerHealed => {
                    GameEvent::LocalPlayerHealed(
                        LocalPlayerHealedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerDestroyedPipeBomb => {
                    GameEvent::PlayerDestroyedPipeBomb(
                        PlayerDestroyedPipeBombEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ObjectDeflected => {
                    GameEvent::ObjectDeflected(
                        ObjectDeflectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerMvp => {
                    GameEvent::PlayerMvp(PlayerMvpEvent::read(stream, definition)?)
                }
                GameEventType::RaidSpawnMob => {
                    GameEvent::RaidSpawnMob(RaidSpawnMobEvent::read(stream, definition)?)
                }
                GameEventType::RaidSpawnSquad => {
                    GameEvent::RaidSpawnSquad(
                        RaidSpawnSquadEvent::read(stream, definition)?,
                    )
                }
                GameEventType::NavBlocked => {
                    GameEvent::NavBlocked(NavBlockedEvent::read(stream, definition)?)
                }
                GameEventType::PathTrackPassed => {
                    GameEvent::PathTrackPassed(
                        PathTrackPassedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::NumCappersChanged => {
                    GameEvent::NumCappersChanged(
                        NumCappersChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerRegenerate => {
                    GameEvent::PlayerRegenerate(
                        PlayerRegenerateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::UpdateStatusItem => {
                    GameEvent::UpdateStatusItem(
                        UpdateStatusItemEvent::read(stream, definition)?,
                    )
                }
                GameEventType::StatsResetRound => {
                    GameEvent::StatsResetRound(
                        StatsResetRoundEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ScoreStatsAccumulatedUpdate => {
                    GameEvent::ScoreStatsAccumulatedUpdate(
                        ScoreStatsAccumulatedUpdateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ScoreStatsAccumulatedReset => {
                    GameEvent::ScoreStatsAccumulatedReset(
                        ScoreStatsAccumulatedResetEvent::read(stream, definition)?,
                    )
                }
                GameEventType::AchievementEarnedLocal => {
                    GameEvent::AchievementEarnedLocal(
                        AchievementEarnedLocalEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHealed => {
                    GameEvent::PlayerHealed(PlayerHealedEvent::read(stream, definition)?)
                }
                GameEventType::BuildingHealed => {
                    GameEvent::BuildingHealed(
                        BuildingHealedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ItemPickup => {
                    GameEvent::ItemPickup(ItemPickupEvent::read(stream, definition)?)
                }
                GameEventType::DuelStatus => {
                    GameEvent::DuelStatus(DuelStatusEvent::read(stream, definition)?)
                }
                GameEventType::FishNotice => {
                    GameEvent::FishNotice(
                        Box::new(<FishNoticeEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::FishNoticeArm => {
                    GameEvent::FishNoticeArm(
                        Box::new(<FishNoticeArmEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::SlapNotice => {
                    GameEvent::SlapNotice(
                        Box::new(<SlapNoticeEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::ThrowableHit => {
                    GameEvent::ThrowableHit(
                        Box::new(<ThrowableHitEvent>::read(stream, definition)?),
                    )
                }
                GameEventType::PumpkinLordSummoned => {
                    GameEvent::PumpkinLordSummoned(
                        PumpkinLordSummonedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PumpkinLordKilled => {
                    GameEvent::PumpkinLordKilled(
                        PumpkinLordKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusSummoned => {
                    GameEvent::MerasmusSummoned(
                        MerasmusSummonedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusKilled => {
                    GameEvent::MerasmusKilled(
                        MerasmusKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusEscapeWarning => {
                    GameEvent::MerasmusEscapeWarning(
                        MerasmusEscapeWarningEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusEscaped => {
                    GameEvent::MerasmusEscaped(
                        MerasmusEscapedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossSummoned => {
                    GameEvent::EyeballBossSummoned(
                        EyeballBossSummonedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossStunned => {
                    GameEvent::EyeballBossStunned(
                        EyeballBossStunnedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossKilled => {
                    GameEvent::EyeballBossKilled(
                        EyeballBossKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossKiller => {
                    GameEvent::EyeballBossKiller(
                        EyeballBossKillerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossEscapeImminent => {
                    GameEvent::EyeballBossEscapeImminent(
                        EyeballBossEscapeImminentEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EyeballBossEscaped => {
                    GameEvent::EyeballBossEscaped(
                        EyeballBossEscapedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::NpcHurt => {
                    GameEvent::NpcHurt(NpcHurtEvent::read(stream, definition)?)
                }
                GameEventType::ControlPointTimerUpdated => {
                    GameEvent::ControlPointTimerUpdated(
                        ControlPointTimerUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHighFiveStart => {
                    GameEvent::PlayerHighFiveStart(
                        PlayerHighFiveStartEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHighFiveCancel => {
                    GameEvent::PlayerHighFiveCancel(
                        PlayerHighFiveCancelEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerHighFiveSuccess => {
                    GameEvent::PlayerHighFiveSuccess(
                        PlayerHighFiveSuccessEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerBonusPoints => {
                    GameEvent::PlayerBonusPoints(
                        PlayerBonusPointsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerUpgraded => {
                    GameEvent::PlayerUpgraded(
                        PlayerUpgradedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerBuyback => {
                    GameEvent::PlayerBuyback(
                        PlayerBuybackEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerUsedPowerUpBottle => {
                    GameEvent::PlayerUsedPowerUpBottle(
                        PlayerUsedPowerUpBottleEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ChristmasGiftGrab => {
                    GameEvent::ChristmasGiftGrab(
                        ChristmasGiftGrabEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerKilledAchievementZone => {
                    GameEvent::PlayerKilledAchievementZone(
                        PlayerKilledAchievementZoneEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyUpdated => {
                    GameEvent::PartyUpdated(PartyUpdatedEvent::read(stream, definition)?)
                }
                GameEventType::PartyPrefChanged => {
                    GameEvent::PartyPrefChanged(
                        PartyPrefChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyCriteriaChanged => {
                    GameEvent::PartyCriteriaChanged(
                        PartyCriteriaChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyInvitesChanged => {
                    GameEvent::PartyInvitesChanged(
                        PartyInvitesChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyQueueStateChanged => {
                    GameEvent::PartyQueueStateChanged(
                        PartyQueueStateChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyChat => {
                    GameEvent::PartyChat(PartyChatEvent::read(stream, definition)?)
                }
                GameEventType::PartyMemberJoin => {
                    GameEvent::PartyMemberJoin(
                        PartyMemberJoinEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PartyMemberLeave => {
                    GameEvent::PartyMemberLeave(
                        PartyMemberLeaveEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MatchInvitesUpdated => {
                    GameEvent::MatchInvitesUpdated(
                        MatchInvitesUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LobbyUpdated => {
                    GameEvent::LobbyUpdated(LobbyUpdatedEvent::read(stream, definition)?)
                }
                GameEventType::MvmMissionUpdate => {
                    GameEvent::MvmMissionUpdate(
                        MvmMissionUpdateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RecalculateHolidays => {
                    GameEvent::RecalculateHolidays(
                        RecalculateHolidaysEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerCurrencyChanged => {
                    GameEvent::PlayerCurrencyChanged(
                        PlayerCurrencyChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DoomsdayRocketOpen => {
                    GameEvent::DoomsdayRocketOpen(
                        DoomsdayRocketOpenEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RemoveNemesisRelationships => {
                    GameEvent::RemoveNemesisRelationships(
                        RemoveNemesisRelationshipsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmCreditBonusWave => {
                    GameEvent::MvmCreditBonusWave(
                        MvmCreditBonusWaveEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmCreditBonusAll => {
                    GameEvent::MvmCreditBonusAll(
                        MvmCreditBonusAllEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmCreditBonusAllAdvanced => {
                    GameEvent::MvmCreditBonusAllAdvanced(
                        MvmCreditBonusAllAdvancedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmQuickSentryUpgrade => {
                    GameEvent::MvmQuickSentryUpgrade(
                        MvmQuickSentryUpgradeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmTankDestroyedByPlayers => {
                    GameEvent::MvmTankDestroyedByPlayers(
                        MvmTankDestroyedByPlayersEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmKillRobotDeliveringBomb => {
                    GameEvent::MvmKillRobotDeliveringBomb(
                        MvmKillRobotDeliveringBombEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmPickupCurrency => {
                    GameEvent::MvmPickupCurrency(
                        MvmPickupCurrencyEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmBombCarrierKilled => {
                    GameEvent::MvmBombCarrierKilled(
                        MvmBombCarrierKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmSentryBusterDetonate => {
                    GameEvent::MvmSentryBusterDetonate(
                        MvmSentryBusterDetonateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmScoutMarkedForDeath => {
                    GameEvent::MvmScoutMarkedForDeath(
                        MvmScoutMarkedForDeathEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmMedicPowerUpShared => {
                    GameEvent::MvmMedicPowerUpShared(
                        MvmMedicPowerUpSharedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmBeginWave => {
                    GameEvent::MvmBeginWave(MvmBeginWaveEvent::read(stream, definition)?)
                }
                GameEventType::MvmWaveComplete => {
                    GameEvent::MvmWaveComplete(
                        MvmWaveCompleteEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmMissionComplete => {
                    GameEvent::MvmMissionComplete(
                        MvmMissionCompleteEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmBombResetByPlayer => {
                    GameEvent::MvmBombResetByPlayer(
                        MvmBombResetByPlayerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmBombAlarmTriggered => {
                    GameEvent::MvmBombAlarmTriggered(
                        MvmBombAlarmTriggeredEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmBombDeployResetByPlayer => {
                    GameEvent::MvmBombDeployResetByPlayer(
                        MvmBombDeployResetByPlayerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmWaveFailed => {
                    GameEvent::MvmWaveFailed(
                        MvmWaveFailedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmResetStats => {
                    GameEvent::MvmResetStats(
                        MvmResetStatsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DamageResisted => {
                    GameEvent::DamageResisted(
                        DamageResistedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RevivePlayerNotify => {
                    GameEvent::RevivePlayerNotify(
                        RevivePlayerNotifyEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RevivePlayerStopped => {
                    GameEvent::RevivePlayerStopped(
                        RevivePlayerStoppedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RevivePlayerComplete => {
                    GameEvent::RevivePlayerComplete(
                        RevivePlayerCompleteEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerTurnedToGhost => {
                    GameEvent::PlayerTurnedToGhost(
                        PlayerTurnedToGhostEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MedigunShieldBlockedDamage => {
                    GameEvent::MedigunShieldBlockedDamage(
                        MedigunShieldBlockedDamageEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmAdvWaveCompleteNoGates => {
                    GameEvent::MvmAdvWaveCompleteNoGates(
                        MvmAdvWaveCompleteNoGatesEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmSniperHeadshotCurrency => {
                    GameEvent::MvmSniperHeadshotCurrency(
                        MvmSniperHeadshotCurrencyEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmMannhattanPit => {
                    GameEvent::MvmMannhattanPit(
                        MvmMannhattanPitEvent::read(stream, definition)?,
                    )
                }
                GameEventType::FlagCarriedInDetectionZone => {
                    GameEvent::FlagCarriedInDetectionZone(
                        FlagCarriedInDetectionZoneEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmAdvWaveKilledStunRadio => {
                    GameEvent::MvmAdvWaveKilledStunRadio(
                        MvmAdvWaveKilledStunRadioEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerDirectHitStun => {
                    GameEvent::PlayerDirectHitStun(
                        PlayerDirectHitStunEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MvmSentryBusterKilled => {
                    GameEvent::MvmSentryBusterKilled(
                        MvmSentryBusterKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::UpgradesFileChanged => {
                    GameEvent::UpgradesFileChanged(
                        UpgradesFileChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RdTeamPointsChanged => {
                    GameEvent::RdTeamPointsChanged(
                        RdTeamPointsChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RdRulesStateChanged => {
                    GameEvent::RdRulesStateChanged(
                        RdRulesStateChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RdRobotKilled => {
                    GameEvent::RdRobotKilled(
                        RdRobotKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RdRobotImpact => {
                    GameEvent::RdRobotImpact(
                        RdRobotImpactEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TeamPlayPreRoundTimeLeft => {
                    GameEvent::TeamPlayPreRoundTimeLeft(
                        TeamPlayPreRoundTimeLeftEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ParachuteDeploy => {
                    GameEvent::ParachuteDeploy(
                        ParachuteDeployEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ParachuteHolster => {
                    GameEvent::ParachuteHolster(
                        ParachuteHolsterEvent::read(stream, definition)?,
                    )
                }
                GameEventType::KillRefillsMeter => {
                    GameEvent::KillRefillsMeter(
                        KillRefillsMeterEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RpsTauntEvent => {
                    GameEvent::RpsTauntEvent(
                        RpsTauntEventEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CongaKill => {
                    GameEvent::CongaKill(CongaKillEvent::read(stream, definition)?)
                }
                GameEventType::PlayerInitialSpawn => {
                    GameEvent::PlayerInitialSpawn(
                        PlayerInitialSpawnEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CompetitiveVictory => {
                    GameEvent::CompetitiveVictory(
                        CompetitiveVictoryEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CompetitiveStatsUpdate => {
                    GameEvent::CompetitiveStatsUpdate(
                        CompetitiveStatsUpdateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MiniGameWin => {
                    GameEvent::MiniGameWin(MiniGameWinEvent::read(stream, definition)?)
                }
                GameEventType::SentryOnGoActive => {
                    GameEvent::SentryOnGoActive(
                        SentryOnGoActiveEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DuckXpLevelUp => {
                    GameEvent::DuckXpLevelUp(
                        DuckXpLevelUpEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestLogOpened => {
                    GameEvent::QuestLogOpened(
                        QuestLogOpenedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::SchemaUpdated => {
                    GameEvent::SchemaUpdated(
                        SchemaUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::LocalPlayerPickupWeapon => {
                    GameEvent::LocalPlayerPickupWeapon(
                        LocalPlayerPickupWeaponEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RdPlayerScorePoints => {
                    GameEvent::RdPlayerScorePoints(
                        RdPlayerScorePointsEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DemomanDetStickies => {
                    GameEvent::DemomanDetStickies(
                        DemomanDetStickiesEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestObjectiveCompleted => {
                    GameEvent::QuestObjectiveCompleted(
                        QuestObjectiveCompletedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerScoreChanged => {
                    GameEvent::PlayerScoreChanged(
                        PlayerScoreChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::KilledCappingPlayer => {
                    GameEvent::KilledCappingPlayer(
                        KilledCappingPlayerEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EnvironmentalDeath => {
                    GameEvent::EnvironmentalDeath(
                        EnvironmentalDeathEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ProjectileDirectHit => {
                    GameEvent::ProjectileDirectHit(
                        ProjectileDirectHitEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PassGet => {
                    GameEvent::PassGet(PassGetEvent::read(stream, definition)?)
                }
                GameEventType::PassScore => {
                    GameEvent::PassScore(PassScoreEvent::read(stream, definition)?)
                }
                GameEventType::PassFree => {
                    GameEvent::PassFree(PassFreeEvent::read(stream, definition)?)
                }
                GameEventType::PassPassCaught => {
                    GameEvent::PassPassCaught(
                        PassPassCaughtEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PassBallStolen => {
                    GameEvent::PassBallStolen(
                        PassBallStolenEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PassBallBlocked => {
                    GameEvent::PassBallBlocked(
                        PassBallBlockedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DamagePrevented => {
                    GameEvent::DamagePrevented(
                        DamagePreventedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HalloweenBossKilled => {
                    GameEvent::HalloweenBossKilled(
                        HalloweenBossKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EscapedLootIsland => {
                    GameEvent::EscapedLootIsland(
                        EscapedLootIslandEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TaggedPlayerAsIt => {
                    GameEvent::TaggedPlayerAsIt(
                        TaggedPlayerAsItEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusStunned => {
                    GameEvent::MerasmusStunned(
                        MerasmusStunnedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MerasmusPropFound => {
                    GameEvent::MerasmusPropFound(
                        MerasmusPropFoundEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HalloweenSkeletonKilled => {
                    GameEvent::HalloweenSkeletonKilled(
                        HalloweenSkeletonKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::EscapeHell => {
                    GameEvent::EscapeHell(EscapeHellEvent::read(stream, definition)?)
                }
                GameEventType::CrossSpectralBridge => {
                    GameEvent::CrossSpectralBridge(
                        CrossSpectralBridgeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::MiniGameWon => {
                    GameEvent::MiniGameWon(MiniGameWonEvent::read(stream, definition)?)
                }
                GameEventType::RespawnGhost => {
                    GameEvent::RespawnGhost(RespawnGhostEvent::read(stream, definition)?)
                }
                GameEventType::KillInHell => {
                    GameEvent::KillInHell(KillInHellEvent::read(stream, definition)?)
                }
                GameEventType::HalloweenDuckCollected => {
                    GameEvent::HalloweenDuckCollected(
                        HalloweenDuckCollectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::SpecialScore => {
                    GameEvent::SpecialScore(SpecialScoreEvent::read(stream, definition)?)
                }
                GameEventType::TeamLeaderKilled => {
                    GameEvent::TeamLeaderKilled(
                        TeamLeaderKilledEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HalloweenSoulCollected => {
                    GameEvent::HalloweenSoulCollected(
                        HalloweenSoulCollectedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RecalculateTruce => {
                    GameEvent::RecalculateTruce(
                        RecalculateTruceEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DeadRingerCheatDeath => {
                    GameEvent::DeadRingerCheatDeath(
                        DeadRingerCheatDeathEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CrossbowHeal => {
                    GameEvent::CrossbowHeal(CrossbowHealEvent::read(stream, definition)?)
                }
                GameEventType::DamageMitigated => {
                    GameEvent::DamageMitigated(
                        DamageMitigatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PayloadPushed => {
                    GameEvent::PayloadPushed(
                        PayloadPushedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerAbandonedMatch => {
                    GameEvent::PlayerAbandonedMatch(
                        PlayerAbandonedMatchEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ClDrawline => {
                    GameEvent::ClDrawline(ClDrawlineEvent::read(stream, definition)?)
                }
                GameEventType::RestartTimerTime => {
                    GameEvent::RestartTimerTime(
                        RestartTimerTimeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::WinLimitChanged => {
                    GameEvent::WinLimitChanged(
                        WinLimitChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::WinPanelShowScores => {
                    GameEvent::WinPanelShowScores(
                        WinPanelShowScoresEvent::read(stream, definition)?,
                    )
                }
                GameEventType::TopStreamsRequestFinished => {
                    GameEvent::TopStreamsRequestFinished(
                        TopStreamsRequestFinishedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CompetitiveStateChanged => {
                    GameEvent::CompetitiveStateChanged(
                        CompetitiveStateChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::GlobalWarDataUpdated => {
                    GameEvent::GlobalWarDataUpdated(
                        GlobalWarDataUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::StopWatchChanged => {
                    GameEvent::StopWatchChanged(
                        StopWatchChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::DsStop => {
                    GameEvent::DsStop(DsStopEvent::read(stream, definition)?)
                }
                GameEventType::DsScreenshot => {
                    GameEvent::DsScreenshot(DsScreenshotEvent::read(stream, definition)?)
                }
                GameEventType::ShowMatchSummary => {
                    GameEvent::ShowMatchSummary(
                        ShowMatchSummaryEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ExperienceChanged => {
                    GameEvent::ExperienceChanged(
                        ExperienceChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::BeginXpLerp => {
                    GameEvent::BeginXpLerp(BeginXpLerpEvent::read(stream, definition)?)
                }
                GameEventType::MatchmakerStatsUpdated => {
                    GameEvent::MatchmakerStatsUpdated(
                        MatchmakerStatsUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RematchVotePeriodOver => {
                    GameEvent::RematchVotePeriodOver(
                        RematchVotePeriodOverEvent::read(stream, definition)?,
                    )
                }
                GameEventType::RematchFailedToCreate => {
                    GameEvent::RematchFailedToCreate(
                        RematchFailedToCreateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerRematchChange => {
                    GameEvent::PlayerRematchChange(
                        PlayerRematchChangeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PingUpdated => {
                    GameEvent::PingUpdated(PingUpdatedEvent::read(stream, definition)?)
                }
                GameEventType::MMStatsUpdated => {
                    GameEvent::MMStatsUpdated(
                        MMStatsUpdatedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerNextMapVoteChange => {
                    GameEvent::PlayerNextMapVoteChange(
                        PlayerNextMapVoteChangeEvent::read(stream, definition)?,
                    )
                }
                GameEventType::VoteMapsChanged => {
                    GameEvent::VoteMapsChanged(
                        VoteMapsChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ProtoDefChanged => {
                    GameEvent::ProtoDefChanged(
                        ProtoDefChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerDomination => {
                    GameEvent::PlayerDomination(
                        PlayerDominationEvent::read(stream, definition)?,
                    )
                }
                GameEventType::PlayerRocketPackPushed => {
                    GameEvent::PlayerRocketPackPushed(
                        PlayerRocketPackPushedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestRequest => {
                    GameEvent::QuestRequest(QuestRequestEvent::read(stream, definition)?)
                }
                GameEventType::QuestResponse => {
                    GameEvent::QuestResponse(
                        QuestResponseEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestProgress => {
                    GameEvent::QuestProgress(
                        QuestProgressEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ProjectileRemoved => {
                    GameEvent::ProjectileRemoved(
                        ProjectileRemovedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestMapDataChanged => {
                    GameEvent::QuestMapDataChanged(
                        QuestMapDataChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::GasDousedPlayerIgnited => {
                    GameEvent::GasDousedPlayerIgnited(
                        GasDousedPlayerIgnitedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::QuestTurnInState => {
                    GameEvent::QuestTurnInState(
                        QuestTurnInStateEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ItemsAcknowledged => {
                    GameEvent::ItemsAcknowledged(
                        ItemsAcknowledgedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::CapperKilled => {
                    GameEvent::CapperKilled(CapperKilledEvent::read(stream, definition)?)
                }
                GameEventType::MainMenuStabilized => {
                    GameEvent::MainMenuStabilized(
                        MainMenuStabilizedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::WorldStatusChanged => {
                    GameEvent::WorldStatusChanged(
                        WorldStatusChangedEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVStatus => {
                    GameEvent::HLTVStatus(HLTVStatusEvent::read(stream, definition)?)
                }
                GameEventType::HLTVCameraman => {
                    GameEvent::HLTVCameraman(
                        HLTVCameramanEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVRankCamera => {
                    GameEvent::HLTVRankCamera(
                        HLTVRankCameraEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVRankEntity => {
                    GameEvent::HLTVRankEntity(
                        HLTVRankEntityEvent::read(stream, definition)?,
                    )
                }
                GameEventType::HLTVFixed => {
                    GameEvent::HLTVFixed(HLTVFixedEvent::read(stream, definition)?)
                }
                GameEventType::HLTVChase => {
                    GameEvent::HLTVChase(HLTVChaseEvent::read(stream, definition)?)
                }
                GameEventType::HLTVMessage => {
                    GameEvent::HLTVMessage(HLTVMessageEvent::read(stream, definition)?)
                }
                GameEventType::HLTVTitle => {
                    GameEvent::HLTVTitle(HLTVTitleEvent::read(stream, definition)?)
                }
                GameEventType::HLTVChat => {
                    GameEvent::HLTVChat(HLTVChatEvent::read(stream, definition)?)
                }
                GameEventType::ReplayStartRecord => {
                    GameEvent::ReplayStartRecord(
                        ReplayStartRecordEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ReplaySessionInfo => {
                    GameEvent::ReplaySessionInfo(
                        ReplaySessionInfoEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ReplayEndRecord => {
                    GameEvent::ReplayEndRecord(
                        ReplayEndRecordEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ReplayReplaysAvailable => {
                    GameEvent::ReplayReplaysAvailable(
                        ReplayReplaysAvailableEvent::read(stream, definition)?,
                    )
                }
                GameEventType::ReplayServerError => {
                    GameEvent::ReplayServerError(
                        ReplayServerErrorEvent::read(stream, definition)?,
                    )
                }
                GameEventType::Unknown(_) => {
                    GameEvent::Unknown(RawGameEvent::read(stream, definition)?)
                }
            },
        )
    }
    pub fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
    ) -> bitbuffer::Result<()> {
        match &self {
            GameEvent::ServerSpawn(event) => event.write(stream),
            GameEvent::ServerChangeLevelFailed(event) => event.write(stream),
            GameEvent::ServerShutdown(event) => event.write(stream),
            GameEvent::ServerCvar(event) => event.write(stream),
            GameEvent::ServerMessage(event) => event.write(stream),
            GameEvent::ServerAddBan(event) => event.write(stream),
            GameEvent::ServerRemoveBan(event) => event.write(stream),
            GameEvent::PlayerConnect(event) => event.write(stream),
            GameEvent::PlayerConnectClient(event) => event.write(stream),
            GameEvent::PlayerInfo(event) => event.write(stream),
            GameEvent::PlayerDisconnect(event) => event.write(stream),
            GameEvent::PlayerActivate(event) => event.write(stream),
            GameEvent::PlayerSay(event) => event.write(stream),
            GameEvent::ClientDisconnect(event) => event.write(stream),
            GameEvent::ClientBeginConnect(event) => event.write(stream),
            GameEvent::ClientConnected(event) => event.write(stream),
            GameEvent::ClientFullConnect(event) => event.write(stream),
            GameEvent::HostQuit(event) => event.write(stream),
            GameEvent::TeamInfo(event) => event.write(stream),
            GameEvent::TeamScore(event) => event.write(stream),
            GameEvent::TeamPlayBroadcastAudio(event) => event.write(stream),
            GameEvent::PlayerTeam(event) => event.write(stream),
            GameEvent::PlayerClass(event) => event.write(stream),
            GameEvent::PlayerDeath(event) => event.write(stream),
            GameEvent::PlayerHurt(event) => event.write(stream),
            GameEvent::PlayerChat(event) => event.write(stream),
            GameEvent::PlayerScore(event) => event.write(stream),
            GameEvent::PlayerSpawn(event) => event.write(stream),
            GameEvent::PlayerShoot(event) => event.write(stream),
            GameEvent::PlayerUse(event) => event.write(stream),
            GameEvent::PlayerChangeName(event) => event.write(stream),
            GameEvent::PlayerHintMessage(event) => event.write(stream),
            GameEvent::BasePlayerTeleported(event) => event.write(stream),
            GameEvent::GameInit(event) => event.write(stream),
            GameEvent::GameNewMap(event) => event.write(stream),
            GameEvent::GameStart(event) => event.write(stream),
            GameEvent::GameEnd(event) => event.write(stream),
            GameEvent::RoundStart(event) => event.write(stream),
            GameEvent::RoundEnd(event) => event.write(stream),
            GameEvent::GameMessage(event) => event.write(stream),
            GameEvent::BreakBreakable(event) => event.write(stream),
            GameEvent::BreakProp(event) => event.write(stream),
            GameEvent::EntityKilled(event) => event.write(stream),
            GameEvent::BonusUpdated(event) => event.write(stream),
            GameEvent::AchievementEvent(event) => event.write(stream),
            GameEvent::AchievementIncrement(event) => event.write(stream),
            GameEvent::PhysgunPickup(event) => event.write(stream),
            GameEvent::FlareIgniteNpc(event) => event.write(stream),
            GameEvent::HelicopterGrenadePuntMiss(event) => event.write(stream),
            GameEvent::UserDataDownloaded(event) => event.write(stream),
            GameEvent::RagdollDissolved(event) => event.write(stream),
            GameEvent::HLTVChangedMode(event) => event.write(stream),
            GameEvent::HLTVChangedTarget(event) => event.write(stream),
            GameEvent::VoteEnded(event) => event.write(stream),
            GameEvent::VoteStarted(event) => event.write(stream),
            GameEvent::VoteChanged(event) => event.write(stream),
            GameEvent::VotePassed(event) => event.write(stream),
            GameEvent::VoteFailed(event) => event.write(stream),
            GameEvent::VoteCast(event) => event.write(stream),
            GameEvent::VoteOptions(event) => event.write(stream),
            GameEvent::ReplaySaved(event) => event.write(stream),
            GameEvent::EnteredPerformanceMode(event) => event.write(stream),
            GameEvent::BrowseReplays(event) => event.write(stream),
            GameEvent::ReplayYoutubeStats(event) => event.write(stream),
            GameEvent::InventoryUpdated(event) => event.write(stream),
            GameEvent::CartUpdated(event) => event.write(stream),
            GameEvent::StorePriceSheetUpdated(event) => event.write(stream),
            GameEvent::EconInventoryConnected(event) => event.write(stream),
            GameEvent::ItemSchemaInitialized(event) => event.write(stream),
            GameEvent::GcNewSession(event) => event.write(stream),
            GameEvent::GcLostSession(event) => event.write(stream),
            GameEvent::IntroFinish(event) => event.write(stream),
            GameEvent::IntroNextCamera(event) => event.write(stream),
            GameEvent::PlayerChangeClass(event) => event.write(stream),
            GameEvent::TfMapTimeRemaining(event) => event.write(stream),
            GameEvent::TfGameOver(event) => event.write(stream),
            GameEvent::CtfFlagCaptured(event) => event.write(stream),
            GameEvent::ControlPointInitialized(event) => event.write(stream),
            GameEvent::ControlPointUpdateImages(event) => event.write(stream),
            GameEvent::ControlPointUpdateLayout(event) => event.write(stream),
            GameEvent::ControlPointUpdateCapping(event) => event.write(stream),
            GameEvent::ControlPointUpdateOwner(event) => event.write(stream),
            GameEvent::ControlPointStartTouch(event) => event.write(stream),
            GameEvent::ControlPointEndTouch(event) => event.write(stream),
            GameEvent::ControlPointPulseElement(event) => event.write(stream),
            GameEvent::ControlPointFakeCapture(event) => event.write(stream),
            GameEvent::ControlPointFakeCaptureMultiplier(event) => event.write(stream),
            GameEvent::TeamPlayRoundSelected(event) => event.write(stream),
            GameEvent::TeamPlayRoundStart(event) => event.write(stream),
            GameEvent::TeamPlayRoundActive(event) => event.write(stream),
            GameEvent::TeamPlayWaitingBegins(event) => event.write(stream),
            GameEvent::TeamPlayWaitingEnds(event) => event.write(stream),
            GameEvent::TeamPlayWaitingAboutToEnd(event) => event.write(stream),
            GameEvent::TeamPlayRestartRound(event) => event.write(stream),
            GameEvent::TeamPlayReadyRestart(event) => event.write(stream),
            GameEvent::TeamPlayRoundRestartSeconds(event) => event.write(stream),
            GameEvent::TeamPlayTeamReady(event) => event.write(stream),
            GameEvent::TeamPlayRoundWin(event) => event.write(stream),
            GameEvent::TeamPlayUpdateTimer(event) => event.write(stream),
            GameEvent::TeamPlayRoundStalemate(event) => event.write(stream),
            GameEvent::TeamPlayOvertimeBegin(event) => event.write(stream),
            GameEvent::TeamPlayOvertimeEnd(event) => event.write(stream),
            GameEvent::TeamPlaySuddenDeathBegin(event) => event.write(stream),
            GameEvent::TeamPlaySuddenDeathEnd(event) => event.write(stream),
            GameEvent::TeamPlayGameOver(event) => event.write(stream),
            GameEvent::TeamPlayMapTimeRemaining(event) => event.write(stream),
            GameEvent::TeamPlayTimerFlash(event) => event.write(stream),
            GameEvent::TeamPlayTimerTimeAdded(event) => event.write(stream),
            GameEvent::TeamPlayPointStartCapture(event) => event.write(stream),
            GameEvent::TeamPlayPointCaptured(event) => event.write(stream),
            GameEvent::TeamPlayPointLocked(event) => event.write(stream),
            GameEvent::TeamPlayPointUnlocked(event) => event.write(stream),
            GameEvent::TeamPlayCaptureBroken(event) => event.write(stream),
            GameEvent::TeamPlayCaptureBlocked(event) => event.write(stream),
            GameEvent::TeamPlayFlagEvent(event) => event.write(stream),
            GameEvent::TeamPlayWinPanel(event) => event.write(stream),
            GameEvent::TeamPlayTeamBalancedPlayer(event) => event.write(stream),
            GameEvent::TeamPlaySetupFinished(event) => event.write(stream),
            GameEvent::TeamPlayAlert(event) => event.write(stream),
            GameEvent::TrainingComplete(event) => event.write(stream),
            GameEvent::ShowFreezePanel(event) => event.write(stream),
            GameEvent::HideFreezePanel(event) => event.write(stream),
            GameEvent::FreezeCamStarted(event) => event.write(stream),
            GameEvent::LocalPlayerChangeTeam(event) => event.write(stream),
            GameEvent::LocalPlayerScoreChanged(event) => event.write(stream),
            GameEvent::LocalPlayerChangeClass(event) => event.write(stream),
            GameEvent::LocalPlayerRespawn(event) => event.write(stream),
            GameEvent::BuildingInfoChanged(event) => event.write(stream),
            GameEvent::LocalPlayerChangeDisguise(event) => event.write(stream),
            GameEvent::PlayerAccountChanged(event) => event.write(stream),
            GameEvent::SpyPdaReset(event) => event.write(stream),
            GameEvent::FlagStatusUpdate(event) => event.write(stream),
            GameEvent::PlayerStatsUpdated(event) => event.write(stream),
            GameEvent::PlayingCommentary(event) => event.write(stream),
            GameEvent::PlayerChargeDeployed(event) => event.write(stream),
            GameEvent::PlayerBuiltObject(event) => event.write(stream),
            GameEvent::PlayerUpgradedObject(event) => event.write(stream),
            GameEvent::PlayerCarryObject(event) => event.write(stream),
            GameEvent::PlayerDropObject(event) => event.write(stream),
            GameEvent::ObjectRemoved(event) => event.write(stream),
            GameEvent::ObjectDestroyed(event) => event.write(stream),
            GameEvent::ObjectDetonated(event) => event.write(stream),
            GameEvent::AchievementEarned(event) => event.write(stream),
            GameEvent::SpecTargetUpdated(event) => event.write(stream),
            GameEvent::TournamentStateUpdate(event) => event.write(stream),
            GameEvent::TournamentEnableCountdown(event) => event.write(stream),
            GameEvent::PlayerCalledForMedic(event) => event.write(stream),
            GameEvent::PlayerAskedForBall(event) => event.write(stream),
            GameEvent::LocalPlayerBecameObserver(event) => event.write(stream),
            GameEvent::PlayerIgnitedInv(event) => event.write(stream),
            GameEvent::PlayerIgnited(event) => event.write(stream),
            GameEvent::PlayerExtinguished(event) => event.write(stream),
            GameEvent::PlayerTeleported(event) => event.write(stream),
            GameEvent::PlayerHealedMedicCall(event) => event.write(stream),
            GameEvent::LocalPlayerChargeReady(event) => event.write(stream),
            GameEvent::LocalPlayerWindDown(event) => event.write(stream),
            GameEvent::PlayerInvulned(event) => event.write(stream),
            GameEvent::EscortSpeed(event) => event.write(stream),
            GameEvent::EscortProgress(event) => event.write(stream),
            GameEvent::EscortRecede(event) => event.write(stream),
            GameEvent::GameUIActivated(event) => event.write(stream),
            GameEvent::GameUIHidden(event) => event.write(stream),
            GameEvent::PlayerEscortScore(event) => event.write(stream),
            GameEvent::PlayerHealOnHit(event) => event.write(stream),
            GameEvent::PlayerStealSandvich(event) => event.write(stream),
            GameEvent::ShowClassLayout(event) => event.write(stream),
            GameEvent::ShowVsPanel(event) => event.write(stream),
            GameEvent::PlayerDamaged(event) => event.write(stream),
            GameEvent::ArenaPlayerNotification(event) => event.write(stream),
            GameEvent::ArenaMatchMaxStreak(event) => event.write(stream),
            GameEvent::ArenaRoundStart(event) => event.write(stream),
            GameEvent::ArenaWinPanel(event) => event.write(stream),
            GameEvent::PveWinPanel(event) => event.write(stream),
            GameEvent::AirDash(event) => event.write(stream),
            GameEvent::Landed(event) => event.write(stream),
            GameEvent::PlayerDamageDodged(event) => event.write(stream),
            GameEvent::PlayerStunned(event) => event.write(stream),
            GameEvent::ScoutGrandSlam(event) => event.write(stream),
            GameEvent::ScoutSlamdollLanded(event) => event.write(stream),
            GameEvent::ArrowImpact(event) => event.write(stream),
            GameEvent::PlayerJarated(event) => event.write(stream),
            GameEvent::PlayerJaratedFade(event) => event.write(stream),
            GameEvent::PlayerShieldBlocked(event) => event.write(stream),
            GameEvent::PlayerPinned(event) => event.write(stream),
            GameEvent::PlayerHealedByMedic(event) => event.write(stream),
            GameEvent::PlayerSappedObject(event) => event.write(stream),
            GameEvent::ItemFound(event) => event.write(stream),
            GameEvent::ShowAnnotation(event) => event.write(stream),
            GameEvent::HideAnnotation(event) => event.write(stream),
            GameEvent::PostInventoryApplication(event) => event.write(stream),
            GameEvent::ControlPointUnlockUpdated(event) => event.write(stream),
            GameEvent::DeployBuffBanner(event) => event.write(stream),
            GameEvent::PlayerBuff(event) => event.write(stream),
            GameEvent::MedicDeath(event) => event.write(stream),
            GameEvent::OvertimeNag(event) => event.write(stream),
            GameEvent::TeamsChanged(event) => event.write(stream),
            GameEvent::HalloweenPumpkinGrab(event) => event.write(stream),
            GameEvent::RocketJump(event) => event.write(stream),
            GameEvent::RocketJumpLanded(event) => event.write(stream),
            GameEvent::StickyJump(event) => event.write(stream),
            GameEvent::StickyJumpLanded(event) => event.write(stream),
            GameEvent::RocketPackLaunch(event) => event.write(stream),
            GameEvent::RocketPackLanded(event) => event.write(stream),
            GameEvent::MedicDefended(event) => event.write(stream),
            GameEvent::LocalPlayerHealed(event) => event.write(stream),
            GameEvent::PlayerDestroyedPipeBomb(event) => event.write(stream),
            GameEvent::ObjectDeflected(event) => event.write(stream),
            GameEvent::PlayerMvp(event) => event.write(stream),
            GameEvent::RaidSpawnMob(event) => event.write(stream),
            GameEvent::RaidSpawnSquad(event) => event.write(stream),
            GameEvent::NavBlocked(event) => event.write(stream),
            GameEvent::PathTrackPassed(event) => event.write(stream),
            GameEvent::NumCappersChanged(event) => event.write(stream),
            GameEvent::PlayerRegenerate(event) => event.write(stream),
            GameEvent::UpdateStatusItem(event) => event.write(stream),
            GameEvent::StatsResetRound(event) => event.write(stream),
            GameEvent::ScoreStatsAccumulatedUpdate(event) => event.write(stream),
            GameEvent::ScoreStatsAccumulatedReset(event) => event.write(stream),
            GameEvent::AchievementEarnedLocal(event) => event.write(stream),
            GameEvent::PlayerHealed(event) => event.write(stream),
            GameEvent::BuildingHealed(event) => event.write(stream),
            GameEvent::ItemPickup(event) => event.write(stream),
            GameEvent::DuelStatus(event) => event.write(stream),
            GameEvent::FishNotice(event) => event.write(stream),
            GameEvent::FishNoticeArm(event) => event.write(stream),
            GameEvent::SlapNotice(event) => event.write(stream),
            GameEvent::ThrowableHit(event) => event.write(stream),
            GameEvent::PumpkinLordSummoned(event) => event.write(stream),
            GameEvent::PumpkinLordKilled(event) => event.write(stream),
            GameEvent::MerasmusSummoned(event) => event.write(stream),
            GameEvent::MerasmusKilled(event) => event.write(stream),
            GameEvent::MerasmusEscapeWarning(event) => event.write(stream),
            GameEvent::MerasmusEscaped(event) => event.write(stream),
            GameEvent::EyeballBossSummoned(event) => event.write(stream),
            GameEvent::EyeballBossStunned(event) => event.write(stream),
            GameEvent::EyeballBossKilled(event) => event.write(stream),
            GameEvent::EyeballBossKiller(event) => event.write(stream),
            GameEvent::EyeballBossEscapeImminent(event) => event.write(stream),
            GameEvent::EyeballBossEscaped(event) => event.write(stream),
            GameEvent::NpcHurt(event) => event.write(stream),
            GameEvent::ControlPointTimerUpdated(event) => event.write(stream),
            GameEvent::PlayerHighFiveStart(event) => event.write(stream),
            GameEvent::PlayerHighFiveCancel(event) => event.write(stream),
            GameEvent::PlayerHighFiveSuccess(event) => event.write(stream),
            GameEvent::PlayerBonusPoints(event) => event.write(stream),
            GameEvent::PlayerUpgraded(event) => event.write(stream),
            GameEvent::PlayerBuyback(event) => event.write(stream),
            GameEvent::PlayerUsedPowerUpBottle(event) => event.write(stream),
            GameEvent::ChristmasGiftGrab(event) => event.write(stream),
            GameEvent::PlayerKilledAchievementZone(event) => event.write(stream),
            GameEvent::PartyUpdated(event) => event.write(stream),
            GameEvent::PartyPrefChanged(event) => event.write(stream),
            GameEvent::PartyCriteriaChanged(event) => event.write(stream),
            GameEvent::PartyInvitesChanged(event) => event.write(stream),
            GameEvent::PartyQueueStateChanged(event) => event.write(stream),
            GameEvent::PartyChat(event) => event.write(stream),
            GameEvent::PartyMemberJoin(event) => event.write(stream),
            GameEvent::PartyMemberLeave(event) => event.write(stream),
            GameEvent::MatchInvitesUpdated(event) => event.write(stream),
            GameEvent::LobbyUpdated(event) => event.write(stream),
            GameEvent::MvmMissionUpdate(event) => event.write(stream),
            GameEvent::RecalculateHolidays(event) => event.write(stream),
            GameEvent::PlayerCurrencyChanged(event) => event.write(stream),
            GameEvent::DoomsdayRocketOpen(event) => event.write(stream),
            GameEvent::RemoveNemesisRelationships(event) => event.write(stream),
            GameEvent::MvmCreditBonusWave(event) => event.write(stream),
            GameEvent::MvmCreditBonusAll(event) => event.write(stream),
            GameEvent::MvmCreditBonusAllAdvanced(event) => event.write(stream),
            GameEvent::MvmQuickSentryUpgrade(event) => event.write(stream),
            GameEvent::MvmTankDestroyedByPlayers(event) => event.write(stream),
            GameEvent::MvmKillRobotDeliveringBomb(event) => event.write(stream),
            GameEvent::MvmPickupCurrency(event) => event.write(stream),
            GameEvent::MvmBombCarrierKilled(event) => event.write(stream),
            GameEvent::MvmSentryBusterDetonate(event) => event.write(stream),
            GameEvent::MvmScoutMarkedForDeath(event) => event.write(stream),
            GameEvent::MvmMedicPowerUpShared(event) => event.write(stream),
            GameEvent::MvmBeginWave(event) => event.write(stream),
            GameEvent::MvmWaveComplete(event) => event.write(stream),
            GameEvent::MvmMissionComplete(event) => event.write(stream),
            GameEvent::MvmBombResetByPlayer(event) => event.write(stream),
            GameEvent::MvmBombAlarmTriggered(event) => event.write(stream),
            GameEvent::MvmBombDeployResetByPlayer(event) => event.write(stream),
            GameEvent::MvmWaveFailed(event) => event.write(stream),
            GameEvent::MvmResetStats(event) => event.write(stream),
            GameEvent::DamageResisted(event) => event.write(stream),
            GameEvent::RevivePlayerNotify(event) => event.write(stream),
            GameEvent::RevivePlayerStopped(event) => event.write(stream),
            GameEvent::RevivePlayerComplete(event) => event.write(stream),
            GameEvent::PlayerTurnedToGhost(event) => event.write(stream),
            GameEvent::MedigunShieldBlockedDamage(event) => event.write(stream),
            GameEvent::MvmAdvWaveCompleteNoGates(event) => event.write(stream),
            GameEvent::MvmSniperHeadshotCurrency(event) => event.write(stream),
            GameEvent::MvmMannhattanPit(event) => event.write(stream),
            GameEvent::FlagCarriedInDetectionZone(event) => event.write(stream),
            GameEvent::MvmAdvWaveKilledStunRadio(event) => event.write(stream),
            GameEvent::PlayerDirectHitStun(event) => event.write(stream),
            GameEvent::MvmSentryBusterKilled(event) => event.write(stream),
            GameEvent::UpgradesFileChanged(event) => event.write(stream),
            GameEvent::RdTeamPointsChanged(event) => event.write(stream),
            GameEvent::RdRulesStateChanged(event) => event.write(stream),
            GameEvent::RdRobotKilled(event) => event.write(stream),
            GameEvent::RdRobotImpact(event) => event.write(stream),
            GameEvent::TeamPlayPreRoundTimeLeft(event) => event.write(stream),
            GameEvent::ParachuteDeploy(event) => event.write(stream),
            GameEvent::ParachuteHolster(event) => event.write(stream),
            GameEvent::KillRefillsMeter(event) => event.write(stream),
            GameEvent::RpsTauntEvent(event) => event.write(stream),
            GameEvent::CongaKill(event) => event.write(stream),
            GameEvent::PlayerInitialSpawn(event) => event.write(stream),
            GameEvent::CompetitiveVictory(event) => event.write(stream),
            GameEvent::CompetitiveStatsUpdate(event) => event.write(stream),
            GameEvent::MiniGameWin(event) => event.write(stream),
            GameEvent::SentryOnGoActive(event) => event.write(stream),
            GameEvent::DuckXpLevelUp(event) => event.write(stream),
            GameEvent::QuestLogOpened(event) => event.write(stream),
            GameEvent::SchemaUpdated(event) => event.write(stream),
            GameEvent::LocalPlayerPickupWeapon(event) => event.write(stream),
            GameEvent::RdPlayerScorePoints(event) => event.write(stream),
            GameEvent::DemomanDetStickies(event) => event.write(stream),
            GameEvent::QuestObjectiveCompleted(event) => event.write(stream),
            GameEvent::PlayerScoreChanged(event) => event.write(stream),
            GameEvent::KilledCappingPlayer(event) => event.write(stream),
            GameEvent::EnvironmentalDeath(event) => event.write(stream),
            GameEvent::ProjectileDirectHit(event) => event.write(stream),
            GameEvent::PassGet(event) => event.write(stream),
            GameEvent::PassScore(event) => event.write(stream),
            GameEvent::PassFree(event) => event.write(stream),
            GameEvent::PassPassCaught(event) => event.write(stream),
            GameEvent::PassBallStolen(event) => event.write(stream),
            GameEvent::PassBallBlocked(event) => event.write(stream),
            GameEvent::DamagePrevented(event) => event.write(stream),
            GameEvent::HalloweenBossKilled(event) => event.write(stream),
            GameEvent::EscapedLootIsland(event) => event.write(stream),
            GameEvent::TaggedPlayerAsIt(event) => event.write(stream),
            GameEvent::MerasmusStunned(event) => event.write(stream),
            GameEvent::MerasmusPropFound(event) => event.write(stream),
            GameEvent::HalloweenSkeletonKilled(event) => event.write(stream),
            GameEvent::EscapeHell(event) => event.write(stream),
            GameEvent::CrossSpectralBridge(event) => event.write(stream),
            GameEvent::MiniGameWon(event) => event.write(stream),
            GameEvent::RespawnGhost(event) => event.write(stream),
            GameEvent::KillInHell(event) => event.write(stream),
            GameEvent::HalloweenDuckCollected(event) => event.write(stream),
            GameEvent::SpecialScore(event) => event.write(stream),
            GameEvent::TeamLeaderKilled(event) => event.write(stream),
            GameEvent::HalloweenSoulCollected(event) => event.write(stream),
            GameEvent::RecalculateTruce(event) => event.write(stream),
            GameEvent::DeadRingerCheatDeath(event) => event.write(stream),
            GameEvent::CrossbowHeal(event) => event.write(stream),
            GameEvent::DamageMitigated(event) => event.write(stream),
            GameEvent::PayloadPushed(event) => event.write(stream),
            GameEvent::PlayerAbandonedMatch(event) => event.write(stream),
            GameEvent::ClDrawline(event) => event.write(stream),
            GameEvent::RestartTimerTime(event) => event.write(stream),
            GameEvent::WinLimitChanged(event) => event.write(stream),
            GameEvent::WinPanelShowScores(event) => event.write(stream),
            GameEvent::TopStreamsRequestFinished(event) => event.write(stream),
            GameEvent::CompetitiveStateChanged(event) => event.write(stream),
            GameEvent::GlobalWarDataUpdated(event) => event.write(stream),
            GameEvent::StopWatchChanged(event) => event.write(stream),
            GameEvent::DsStop(event) => event.write(stream),
            GameEvent::DsScreenshot(event) => event.write(stream),
            GameEvent::ShowMatchSummary(event) => event.write(stream),
            GameEvent::ExperienceChanged(event) => event.write(stream),
            GameEvent::BeginXpLerp(event) => event.write(stream),
            GameEvent::MatchmakerStatsUpdated(event) => event.write(stream),
            GameEvent::RematchVotePeriodOver(event) => event.write(stream),
            GameEvent::RematchFailedToCreate(event) => event.write(stream),
            GameEvent::PlayerRematchChange(event) => event.write(stream),
            GameEvent::PingUpdated(event) => event.write(stream),
            GameEvent::MMStatsUpdated(event) => event.write(stream),
            GameEvent::PlayerNextMapVoteChange(event) => event.write(stream),
            GameEvent::VoteMapsChanged(event) => event.write(stream),
            GameEvent::ProtoDefChanged(event) => event.write(stream),
            GameEvent::PlayerDomination(event) => event.write(stream),
            GameEvent::PlayerRocketPackPushed(event) => event.write(stream),
            GameEvent::QuestRequest(event) => event.write(stream),
            GameEvent::QuestResponse(event) => event.write(stream),
            GameEvent::QuestProgress(event) => event.write(stream),
            GameEvent::ProjectileRemoved(event) => event.write(stream),
            GameEvent::QuestMapDataChanged(event) => event.write(stream),
            GameEvent::GasDousedPlayerIgnited(event) => event.write(stream),
            GameEvent::QuestTurnInState(event) => event.write(stream),
            GameEvent::ItemsAcknowledged(event) => event.write(stream),
            GameEvent::CapperKilled(event) => event.write(stream),
            GameEvent::MainMenuStabilized(event) => event.write(stream),
            GameEvent::WorldStatusChanged(event) => event.write(stream),
            GameEvent::HLTVStatus(event) => event.write(stream),
            GameEvent::HLTVCameraman(event) => event.write(stream),
            GameEvent::HLTVRankCamera(event) => event.write(stream),
            GameEvent::HLTVRankEntity(event) => event.write(stream),
            GameEvent::HLTVFixed(event) => event.write(stream),
            GameEvent::HLTVChase(event) => event.write(stream),
            GameEvent::HLTVMessage(event) => event.write(stream),
            GameEvent::HLTVTitle(event) => event.write(stream),
            GameEvent::HLTVChat(event) => event.write(stream),
            GameEvent::ReplayStartRecord(event) => event.write(stream),
            GameEvent::ReplaySessionInfo(event) => event.write(stream),
            GameEvent::ReplayEndRecord(event) => event.write(stream),
            GameEvent::ReplayReplaysAvailable(event) => event.write(stream),
            GameEvent::ReplayServerError(event) => event.write(stream),
            GameEvent::Unknown(raw) => raw.write(stream),
        }
    }
    pub fn event_type(&self) -> GameEventType {
        match &self {
            GameEvent::ServerSpawn(_) => GameEventType::ServerSpawn,
            GameEvent::ServerChangeLevelFailed(_) => {
                GameEventType::ServerChangeLevelFailed
            }
            GameEvent::ServerShutdown(_) => GameEventType::ServerShutdown,
            GameEvent::ServerCvar(_) => GameEventType::ServerCvar,
            GameEvent::ServerMessage(_) => GameEventType::ServerMessage,
            GameEvent::ServerAddBan(_) => GameEventType::ServerAddBan,
            GameEvent::ServerRemoveBan(_) => GameEventType::ServerRemoveBan,
            GameEvent::PlayerConnect(_) => GameEventType::PlayerConnect,
            GameEvent::PlayerConnectClient(_) => GameEventType::PlayerConnectClient,
            GameEvent::PlayerInfo(_) => GameEventType::PlayerInfo,
            GameEvent::PlayerDisconnect(_) => GameEventType::PlayerDisconnect,
            GameEvent::PlayerActivate(_) => GameEventType::PlayerActivate,
            GameEvent::PlayerSay(_) => GameEventType::PlayerSay,
            GameEvent::ClientDisconnect(_) => GameEventType::ClientDisconnect,
            GameEvent::ClientBeginConnect(_) => GameEventType::ClientBeginConnect,
            GameEvent::ClientConnected(_) => GameEventType::ClientConnected,
            GameEvent::ClientFullConnect(_) => GameEventType::ClientFullConnect,
            GameEvent::HostQuit(_) => GameEventType::HostQuit,
            GameEvent::TeamInfo(_) => GameEventType::TeamInfo,
            GameEvent::TeamScore(_) => GameEventType::TeamScore,
            GameEvent::TeamPlayBroadcastAudio(_) => GameEventType::TeamPlayBroadcastAudio,
            GameEvent::PlayerTeam(_) => GameEventType::PlayerTeam,
            GameEvent::PlayerClass(_) => GameEventType::PlayerClass,
            GameEvent::PlayerDeath(_) => GameEventType::PlayerDeath,
            GameEvent::PlayerHurt(_) => GameEventType::PlayerHurt,
            GameEvent::PlayerChat(_) => GameEventType::PlayerChat,
            GameEvent::PlayerScore(_) => GameEventType::PlayerScore,
            GameEvent::PlayerSpawn(_) => GameEventType::PlayerSpawn,
            GameEvent::PlayerShoot(_) => GameEventType::PlayerShoot,
            GameEvent::PlayerUse(_) => GameEventType::PlayerUse,
            GameEvent::PlayerChangeName(_) => GameEventType::PlayerChangeName,
            GameEvent::PlayerHintMessage(_) => GameEventType::PlayerHintMessage,
            GameEvent::BasePlayerTeleported(_) => GameEventType::BasePlayerTeleported,
            GameEvent::GameInit(_) => GameEventType::GameInit,
            GameEvent::GameNewMap(_) => GameEventType::GameNewMap,
            GameEvent::GameStart(_) => GameEventType::GameStart,
            GameEvent::GameEnd(_) => GameEventType::GameEnd,
            GameEvent::RoundStart(_) => GameEventType::RoundStart,
            GameEvent::RoundEnd(_) => GameEventType::RoundEnd,
            GameEvent::GameMessage(_) => GameEventType::GameMessage,
            GameEvent::BreakBreakable(_) => GameEventType::BreakBreakable,
            GameEvent::BreakProp(_) => GameEventType::BreakProp,
            GameEvent::EntityKilled(_) => GameEventType::EntityKilled,
            GameEvent::BonusUpdated(_) => GameEventType::BonusUpdated,
            GameEvent::AchievementEvent(_) => GameEventType::AchievementEvent,
            GameEvent::AchievementIncrement(_) => GameEventType::AchievementIncrement,
            GameEvent::PhysgunPickup(_) => GameEventType::PhysgunPickup,
            GameEvent::FlareIgniteNpc(_) => GameEventType::FlareIgniteNpc,
            GameEvent::HelicopterGrenadePuntMiss(_) => {
                GameEventType::HelicopterGrenadePuntMiss
            }
            GameEvent::UserDataDownloaded(_) => GameEventType::UserDataDownloaded,
            GameEvent::RagdollDissolved(_) => GameEventType::RagdollDissolved,
            GameEvent::HLTVChangedMode(_) => GameEventType::HLTVChangedMode,
            GameEvent::HLTVChangedTarget(_) => GameEventType::HLTVChangedTarget,
            GameEvent::VoteEnded(_) => GameEventType::VoteEnded,
            GameEvent::VoteStarted(_) => GameEventType::VoteStarted,
            GameEvent::VoteChanged(_) => GameEventType::VoteChanged,
            GameEvent::VotePassed(_) => GameEventType::VotePassed,
            GameEvent::VoteFailed(_) => GameEventType::VoteFailed,
            GameEvent::VoteCast(_) => GameEventType::VoteCast,
            GameEvent::VoteOptions(_) => GameEventType::VoteOptions,
            GameEvent::ReplaySaved(_) => GameEventType::ReplaySaved,
            GameEvent::EnteredPerformanceMode(_) => GameEventType::EnteredPerformanceMode,
            GameEvent::BrowseReplays(_) => GameEventType::BrowseReplays,
            GameEvent::ReplayYoutubeStats(_) => GameEventType::ReplayYoutubeStats,
            GameEvent::InventoryUpdated(_) => GameEventType::InventoryUpdated,
            GameEvent::CartUpdated(_) => GameEventType::CartUpdated,
            GameEvent::StorePriceSheetUpdated(_) => GameEventType::StorePriceSheetUpdated,
            GameEvent::EconInventoryConnected(_) => GameEventType::EconInventoryConnected,
            GameEvent::ItemSchemaInitialized(_) => GameEventType::ItemSchemaInitialized,
            GameEvent::GcNewSession(_) => GameEventType::GcNewSession,
            GameEvent::GcLostSession(_) => GameEventType::GcLostSession,
            GameEvent::IntroFinish(_) => GameEventType::IntroFinish,
            GameEvent::IntroNextCamera(_) => GameEventType::IntroNextCamera,
            GameEvent::PlayerChangeClass(_) => GameEventType::PlayerChangeClass,
            GameEvent::TfMapTimeRemaining(_) => GameEventType::TfMapTimeRemaining,
            GameEvent::TfGameOver(_) => GameEventType::TfGameOver,
            GameEvent::CtfFlagCaptured(_) => GameEventType::CtfFlagCaptured,
            GameEvent::ControlPointInitialized(_) => {
                GameEventType::ControlPointInitialized
            }
            GameEvent::ControlPointUpdateImages(_) => {
                GameEventType::ControlPointUpdateImages
            }
            GameEvent::ControlPointUpdateLayout(_) => {
                GameEventType::ControlPointUpdateLayout
            }
            GameEvent::ControlPointUpdateCapping(_) => {
                GameEventType::ControlPointUpdateCapping
            }
            GameEvent::ControlPointUpdateOwner(_) => {
                GameEventType::ControlPointUpdateOwner
            }
            GameEvent::ControlPointStartTouch(_) => GameEventType::ControlPointStartTouch,
            GameEvent::ControlPointEndTouch(_) => GameEventType::ControlPointEndTouch,
            GameEvent::ControlPointPulseElement(_) => {
                GameEventType::ControlPointPulseElement
            }
            GameEvent::ControlPointFakeCapture(_) => {
                GameEventType::ControlPointFakeCapture
            }
            GameEvent::ControlPointFakeCaptureMultiplier(_) => {
                GameEventType::ControlPointFakeCaptureMultiplier
            }
            GameEvent::TeamPlayRoundSelected(_) => GameEventType::TeamPlayRoundSelected,
            GameEvent::TeamPlayRoundStart(_) => GameEventType::TeamPlayRoundStart,
            GameEvent::TeamPlayRoundActive(_) => GameEventType::TeamPlayRoundActive,
            GameEvent::TeamPlayWaitingBegins(_) => GameEventType::TeamPlayWaitingBegins,
            GameEvent::TeamPlayWaitingEnds(_) => GameEventType::TeamPlayWaitingEnds,
            GameEvent::TeamPlayWaitingAboutToEnd(_) => {
                GameEventType::TeamPlayWaitingAboutToEnd
            }
            GameEvent::TeamPlayRestartRound(_) => GameEventType::TeamPlayRestartRound,
            GameEvent::TeamPlayReadyRestart(_) => GameEventType::TeamPlayReadyRestart,
            GameEvent::TeamPlayRoundRestartSeconds(_) => {
                GameEventType::TeamPlayRoundRestartSeconds
            }
            GameEvent::TeamPlayTeamReady(_) => GameEventType::TeamPlayTeamReady,
            GameEvent::TeamPlayRoundWin(_) => GameEventType::TeamPlayRoundWin,
            GameEvent::TeamPlayUpdateTimer(_) => GameEventType::TeamPlayUpdateTimer,
            GameEvent::TeamPlayRoundStalemate(_) => GameEventType::TeamPlayRoundStalemate,
            GameEvent::TeamPlayOvertimeBegin(_) => GameEventType::TeamPlayOvertimeBegin,
            GameEvent::TeamPlayOvertimeEnd(_) => GameEventType::TeamPlayOvertimeEnd,
            GameEvent::TeamPlaySuddenDeathBegin(_) => {
                GameEventType::TeamPlaySuddenDeathBegin
            }
            GameEvent::TeamPlaySuddenDeathEnd(_) => GameEventType::TeamPlaySuddenDeathEnd,
            GameEvent::TeamPlayGameOver(_) => GameEventType::TeamPlayGameOver,
            GameEvent::TeamPlayMapTimeRemaining(_) => {
                GameEventType::TeamPlayMapTimeRemaining
            }
            GameEvent::TeamPlayTimerFlash(_) => GameEventType::TeamPlayTimerFlash,
            GameEvent::TeamPlayTimerTimeAdded(_) => GameEventType::TeamPlayTimerTimeAdded,
            GameEvent::TeamPlayPointStartCapture(_) => {
                GameEventType::TeamPlayPointStartCapture
            }
            GameEvent::TeamPlayPointCaptured(_) => GameEventType::TeamPlayPointCaptured,
            GameEvent::TeamPlayPointLocked(_) => GameEventType::TeamPlayPointLocked,
            GameEvent::TeamPlayPointUnlocked(_) => GameEventType::TeamPlayPointUnlocked,
            GameEvent::TeamPlayCaptureBroken(_) => GameEventType::TeamPlayCaptureBroken,
            GameEvent::TeamPlayCaptureBlocked(_) => GameEventType::TeamPlayCaptureBlocked,
            GameEvent::TeamPlayFlagEvent(_) => GameEventType::TeamPlayFlagEvent,
            GameEvent::TeamPlayWinPanel(_) => GameEventType::TeamPlayWinPanel,
            GameEvent::TeamPlayTeamBalancedPlayer(_) => {
                GameEventType::TeamPlayTeamBalancedPlayer
            }
            GameEvent::TeamPlaySetupFinished(_) => GameEventType::TeamPlaySetupFinished,
            GameEvent::TeamPlayAlert(_) => GameEventType::TeamPlayAlert,
            GameEvent::TrainingComplete(_) => GameEventType::TrainingComplete,
            GameEvent::ShowFreezePanel(_) => GameEventType::ShowFreezePanel,
            GameEvent::HideFreezePanel(_) => GameEventType::HideFreezePanel,
            GameEvent::FreezeCamStarted(_) => GameEventType::FreezeCamStarted,
            GameEvent::LocalPlayerChangeTeam(_) => GameEventType::LocalPlayerChangeTeam,
            GameEvent::LocalPlayerScoreChanged(_) => {
                GameEventType::LocalPlayerScoreChanged
            }
            GameEvent::LocalPlayerChangeClass(_) => GameEventType::LocalPlayerChangeClass,
            GameEvent::LocalPlayerRespawn(_) => GameEventType::LocalPlayerRespawn,
            GameEvent::BuildingInfoChanged(_) => GameEventType::BuildingInfoChanged,
            GameEvent::LocalPlayerChangeDisguise(_) => {
                GameEventType::LocalPlayerChangeDisguise
            }
            GameEvent::PlayerAccountChanged(_) => GameEventType::PlayerAccountChanged,
            GameEvent::SpyPdaReset(_) => GameEventType::SpyPdaReset,
            GameEvent::FlagStatusUpdate(_) => GameEventType::FlagStatusUpdate,
            GameEvent::PlayerStatsUpdated(_) => GameEventType::PlayerStatsUpdated,
            GameEvent::PlayingCommentary(_) => GameEventType::PlayingCommentary,
            GameEvent::PlayerChargeDeployed(_) => GameEventType::PlayerChargeDeployed,
            GameEvent::PlayerBuiltObject(_) => GameEventType::PlayerBuiltObject,
            GameEvent::PlayerUpgradedObject(_) => GameEventType::PlayerUpgradedObject,
            GameEvent::PlayerCarryObject(_) => GameEventType::PlayerCarryObject,
            GameEvent::PlayerDropObject(_) => GameEventType::PlayerDropObject,
            GameEvent::ObjectRemoved(_) => GameEventType::ObjectRemoved,
            GameEvent::ObjectDestroyed(_) => GameEventType::ObjectDestroyed,
            GameEvent::ObjectDetonated(_) => GameEventType::ObjectDetonated,
            GameEvent::AchievementEarned(_) => GameEventType::AchievementEarned,
            GameEvent::SpecTargetUpdated(_) => GameEventType::SpecTargetUpdated,
            GameEvent::TournamentStateUpdate(_) => GameEventType::TournamentStateUpdate,
            GameEvent::TournamentEnableCountdown(_) => {
                GameEventType::TournamentEnableCountdown
            }
            GameEvent::PlayerCalledForMedic(_) => GameEventType::PlayerCalledForMedic,
            GameEvent::PlayerAskedForBall(_) => GameEventType::PlayerAskedForBall,
            GameEvent::LocalPlayerBecameObserver(_) => {
                GameEventType::LocalPlayerBecameObserver
            }
            GameEvent::PlayerIgnitedInv(_) => GameEventType::PlayerIgnitedInv,
            GameEvent::PlayerIgnited(_) => GameEventType::PlayerIgnited,
            GameEvent::PlayerExtinguished(_) => GameEventType::PlayerExtinguished,
            GameEvent::PlayerTeleported(_) => GameEventType::PlayerTeleported,
            GameEvent::PlayerHealedMedicCall(_) => GameEventType::PlayerHealedMedicCall,
            GameEvent::LocalPlayerChargeReady(_) => GameEventType::LocalPlayerChargeReady,
            GameEvent::LocalPlayerWindDown(_) => GameEventType::LocalPlayerWindDown,
            GameEvent::PlayerInvulned(_) => GameEventType::PlayerInvulned,
            GameEvent::EscortSpeed(_) => GameEventType::EscortSpeed,
            GameEvent::EscortProgress(_) => GameEventType::EscortProgress,
            GameEvent::EscortRecede(_) => GameEventType::EscortRecede,
            GameEvent::GameUIActivated(_) => GameEventType::GameUIActivated,
            GameEvent::GameUIHidden(_) => GameEventType::GameUIHidden,
            GameEvent::PlayerEscortScore(_) => GameEventType::PlayerEscortScore,
            GameEvent::PlayerHealOnHit(_) => GameEventType::PlayerHealOnHit,
            GameEvent::PlayerStealSandvich(_) => GameEventType::PlayerStealSandvich,
            GameEvent::ShowClassLayout(_) => GameEventType::ShowClassLayout,
            GameEvent::ShowVsPanel(_) => GameEventType::ShowVsPanel,
            GameEvent::PlayerDamaged(_) => GameEventType::PlayerDamaged,
            GameEvent::ArenaPlayerNotification(_) => {
                GameEventType::ArenaPlayerNotification
            }
            GameEvent::ArenaMatchMaxStreak(_) => GameEventType::ArenaMatchMaxStreak,
            GameEvent::ArenaRoundStart(_) => GameEventType::ArenaRoundStart,
            GameEvent::ArenaWinPanel(_) => GameEventType::ArenaWinPanel,
            GameEvent::PveWinPanel(_) => GameEventType::PveWinPanel,
            GameEvent::AirDash(_) => GameEventType::AirDash,
            GameEvent::Landed(_) => GameEventType::Landed,
            GameEvent::PlayerDamageDodged(_) => GameEventType::PlayerDamageDodged,
            GameEvent::PlayerStunned(_) => GameEventType::PlayerStunned,
            GameEvent::ScoutGrandSlam(_) => GameEventType::ScoutGrandSlam,
            GameEvent::ScoutSlamdollLanded(_) => GameEventType::ScoutSlamdollLanded,
            GameEvent::ArrowImpact(_) => GameEventType::ArrowImpact,
            GameEvent::PlayerJarated(_) => GameEventType::PlayerJarated,
            GameEvent::PlayerJaratedFade(_) => GameEventType::PlayerJaratedFade,
            GameEvent::PlayerShieldBlocked(_) => GameEventType::PlayerShieldBlocked,
            GameEvent::PlayerPinned(_) => GameEventType::PlayerPinned,
            GameEvent::PlayerHealedByMedic(_) => GameEventType::PlayerHealedByMedic,
            GameEvent::PlayerSappedObject(_) => GameEventType::PlayerSappedObject,
            GameEvent::ItemFound(_) => GameEventType::ItemFound,
            GameEvent::ShowAnnotation(_) => GameEventType::ShowAnnotation,
            GameEvent::HideAnnotation(_) => GameEventType::HideAnnotation,
            GameEvent::PostInventoryApplication(_) => {
                GameEventType::PostInventoryApplication
            }
            GameEvent::ControlPointUnlockUpdated(_) => {
                GameEventType::ControlPointUnlockUpdated
            }
            GameEvent::DeployBuffBanner(_) => GameEventType::DeployBuffBanner,
            GameEvent::PlayerBuff(_) => GameEventType::PlayerBuff,
            GameEvent::MedicDeath(_) => GameEventType::MedicDeath,
            GameEvent::OvertimeNag(_) => GameEventType::OvertimeNag,
            GameEvent::TeamsChanged(_) => GameEventType::TeamsChanged,
            GameEvent::HalloweenPumpkinGrab(_) => GameEventType::HalloweenPumpkinGrab,
            GameEvent::RocketJump(_) => GameEventType::RocketJump,
            GameEvent::RocketJumpLanded(_) => GameEventType::RocketJumpLanded,
            GameEvent::StickyJump(_) => GameEventType::StickyJump,
            GameEvent::StickyJumpLanded(_) => GameEventType::StickyJumpLanded,
            GameEvent::RocketPackLaunch(_) => GameEventType::RocketPackLaunch,
            GameEvent::RocketPackLanded(_) => GameEventType::RocketPackLanded,
            GameEvent::MedicDefended(_) => GameEventType::MedicDefended,
            GameEvent::LocalPlayerHealed(_) => GameEventType::LocalPlayerHealed,
            GameEvent::PlayerDestroyedPipeBomb(_) => {
                GameEventType::PlayerDestroyedPipeBomb
            }
            GameEvent::ObjectDeflected(_) => GameEventType::ObjectDeflected,
            GameEvent::PlayerMvp(_) => GameEventType::PlayerMvp,
            GameEvent::RaidSpawnMob(_) => GameEventType::RaidSpawnMob,
            GameEvent::RaidSpawnSquad(_) => GameEventType::RaidSpawnSquad,
            GameEvent::NavBlocked(_) => GameEventType::NavBlocked,
            GameEvent::PathTrackPassed(_) => GameEventType::PathTrackPassed,
            GameEvent::NumCappersChanged(_) => GameEventType::NumCappersChanged,
            GameEvent::PlayerRegenerate(_) => GameEventType::PlayerRegenerate,
            GameEvent::UpdateStatusItem(_) => GameEventType::UpdateStatusItem,
            GameEvent::StatsResetRound(_) => GameEventType::StatsResetRound,
            GameEvent::ScoreStatsAccumulatedUpdate(_) => {
                GameEventType::ScoreStatsAccumulatedUpdate
            }
            GameEvent::ScoreStatsAccumulatedReset(_) => {
                GameEventType::ScoreStatsAccumulatedReset
            }
            GameEvent::AchievementEarnedLocal(_) => GameEventType::AchievementEarnedLocal,
            GameEvent::PlayerHealed(_) => GameEventType::PlayerHealed,
            GameEvent::BuildingHealed(_) => GameEventType::BuildingHealed,
            GameEvent::ItemPickup(_) => GameEventType::ItemPickup,
            GameEvent::DuelStatus(_) => GameEventType::DuelStatus,
            GameEvent::FishNotice(_) => GameEventType::FishNotice,
            GameEvent::FishNoticeArm(_) => GameEventType::FishNoticeArm,
            GameEvent::SlapNotice(_) => GameEventType::SlapNotice,
            GameEvent::ThrowableHit(_) => GameEventType::ThrowableHit,
            GameEvent::PumpkinLordSummoned(_) => GameEventType::PumpkinLordSummoned,
            GameEvent::PumpkinLordKilled(_) => GameEventType::PumpkinLordKilled,
            GameEvent::MerasmusSummoned(_) => GameEventType::MerasmusSummoned,
            GameEvent::MerasmusKilled(_) => GameEventType::MerasmusKilled,
            GameEvent::MerasmusEscapeWarning(_) => GameEventType::MerasmusEscapeWarning,
            GameEvent::MerasmusEscaped(_) => GameEventType::MerasmusEscaped,
            GameEvent::EyeballBossSummoned(_) => GameEventType::EyeballBossSummoned,
            GameEvent::EyeballBossStunned(_) => GameEventType::EyeballBossStunned,
            GameEvent::EyeballBossKilled(_) => GameEventType::EyeballBossKilled,
            GameEvent::EyeballBossKiller(_) => GameEventType::EyeballBossKiller,
            GameEvent::EyeballBossEscapeImminent(_) => {
                GameEventType::EyeballBossEscapeImminent
            }
            GameEvent::EyeballBossEscaped(_) => GameEventType::EyeballBossEscaped,
            GameEvent::NpcHurt(_) => GameEventType::NpcHurt,
            GameEvent::ControlPointTimerUpdated(_) => {
                GameEventType::ControlPointTimerUpdated
            }
            GameEvent::PlayerHighFiveStart(_) => GameEventType::PlayerHighFiveStart,
            GameEvent::PlayerHighFiveCancel(_) => GameEventType::PlayerHighFiveCancel,
            GameEvent::PlayerHighFiveSuccess(_) => GameEventType::PlayerHighFiveSuccess,
            GameEvent::PlayerBonusPoints(_) => GameEventType::PlayerBonusPoints,
            GameEvent::PlayerUpgraded(_) => GameEventType::PlayerUpgraded,
            GameEvent::PlayerBuyback(_) => GameEventType::PlayerBuyback,
            GameEvent::PlayerUsedPowerUpBottle(_) => {
                GameEventType::PlayerUsedPowerUpBottle
            }
            GameEvent::ChristmasGiftGrab(_) => GameEventType::ChristmasGiftGrab,
            GameEvent::PlayerKilledAchievementZone(_) => {
                GameEventType::PlayerKilledAchievementZone
            }
            GameEvent::PartyUpdated(_) => GameEventType::PartyUpdated,
            GameEvent::PartyPrefChanged(_) => GameEventType::PartyPrefChanged,
            GameEvent::PartyCriteriaChanged(_) => GameEventType::PartyCriteriaChanged,
            GameEvent::PartyInvitesChanged(_) => GameEventType::PartyInvitesChanged,
            GameEvent::PartyQueueStateChanged(_) => GameEventType::PartyQueueStateChanged,
            GameEvent::PartyChat(_) => GameEventType::PartyChat,
            GameEvent::PartyMemberJoin(_) => GameEventType::PartyMemberJoin,
            GameEvent::PartyMemberLeave(_) => GameEventType::PartyMemberLeave,
            GameEvent::MatchInvitesUpdated(_) => GameEventType::MatchInvitesUpdated,
            GameEvent::LobbyUpdated(_) => GameEventType::LobbyUpdated,
            GameEvent::MvmMissionUpdate(_) => GameEventType::MvmMissionUpdate,
            GameEvent::RecalculateHolidays(_) => GameEventType::RecalculateHolidays,
            GameEvent::PlayerCurrencyChanged(_) => GameEventType::PlayerCurrencyChanged,
            GameEvent::DoomsdayRocketOpen(_) => GameEventType::DoomsdayRocketOpen,
            GameEvent::RemoveNemesisRelationships(_) => {
                GameEventType::RemoveNemesisRelationships
            }
            GameEvent::MvmCreditBonusWave(_) => GameEventType::MvmCreditBonusWave,
            GameEvent::MvmCreditBonusAll(_) => GameEventType::MvmCreditBonusAll,
            GameEvent::MvmCreditBonusAllAdvanced(_) => {
                GameEventType::MvmCreditBonusAllAdvanced
            }
            GameEvent::MvmQuickSentryUpgrade(_) => GameEventType::MvmQuickSentryUpgrade,
            GameEvent::MvmTankDestroyedByPlayers(_) => {
                GameEventType::MvmTankDestroyedByPlayers
            }
            GameEvent::MvmKillRobotDeliveringBomb(_) => {
                GameEventType::MvmKillRobotDeliveringBomb
            }
            GameEvent::MvmPickupCurrency(_) => GameEventType::MvmPickupCurrency,
            GameEvent::MvmBombCarrierKilled(_) => GameEventType::MvmBombCarrierKilled,
            GameEvent::MvmSentryBusterDetonate(_) => {
                GameEventType::MvmSentryBusterDetonate
            }
            GameEvent::MvmScoutMarkedForDeath(_) => GameEventType::MvmScoutMarkedForDeath,
            GameEvent::MvmMedicPowerUpShared(_) => GameEventType::MvmMedicPowerUpShared,
            GameEvent::MvmBeginWave(_) => GameEventType::MvmBeginWave,
            GameEvent::MvmWaveComplete(_) => GameEventType::MvmWaveComplete,
            GameEvent::MvmMissionComplete(_) => GameEventType::MvmMissionComplete,
            GameEvent::MvmBombResetByPlayer(_) => GameEventType::MvmBombResetByPlayer,
            GameEvent::MvmBombAlarmTriggered(_) => GameEventType::MvmBombAlarmTriggered,
            GameEvent::MvmBombDeployResetByPlayer(_) => {
                GameEventType::MvmBombDeployResetByPlayer
            }
            GameEvent::MvmWaveFailed(_) => GameEventType::MvmWaveFailed,
            GameEvent::MvmResetStats(_) => GameEventType::MvmResetStats,
            GameEvent::DamageResisted(_) => GameEventType::DamageResisted,
            GameEvent::RevivePlayerNotify(_) => GameEventType::RevivePlayerNotify,
            GameEvent::RevivePlayerStopped(_) => GameEventType::RevivePlayerStopped,
            GameEvent::RevivePlayerComplete(_) => GameEventType::RevivePlayerComplete,
            GameEvent::PlayerTurnedToGhost(_) => GameEventType::PlayerTurnedToGhost,
            GameEvent::MedigunShieldBlockedDamage(_) => {
                GameEventType::MedigunShieldBlockedDamage
            }
            GameEvent::MvmAdvWaveCompleteNoGates(_) => {
                GameEventType::MvmAdvWaveCompleteNoGates
            }
            GameEvent::MvmSniperHeadshotCurrency(_) => {
                GameEventType::MvmSniperHeadshotCurrency
            }
            GameEvent::MvmMannhattanPit(_) => GameEventType::MvmMannhattanPit,
            GameEvent::FlagCarriedInDetectionZone(_) => {
                GameEventType::FlagCarriedInDetectionZone
            }
            GameEvent::MvmAdvWaveKilledStunRadio(_) => {
                GameEventType::MvmAdvWaveKilledStunRadio
            }
            GameEvent::PlayerDirectHitStun(_) => GameEventType::PlayerDirectHitStun,
            GameEvent::MvmSentryBusterKilled(_) => GameEventType::MvmSentryBusterKilled,
            GameEvent::UpgradesFileChanged(_) => GameEventType::UpgradesFileChanged,
            GameEvent::RdTeamPointsChanged(_) => GameEventType::RdTeamPointsChanged,
            GameEvent::RdRulesStateChanged(_) => GameEventType::RdRulesStateChanged,
            GameEvent::RdRobotKilled(_) => GameEventType::RdRobotKilled,
            GameEvent::RdRobotImpact(_) => GameEventType::RdRobotImpact,
            GameEvent::TeamPlayPreRoundTimeLeft(_) => {
                GameEventType::TeamPlayPreRoundTimeLeft
            }
            GameEvent::ParachuteDeploy(_) => GameEventType::ParachuteDeploy,
            GameEvent::ParachuteHolster(_) => GameEventType::ParachuteHolster,
            GameEvent::KillRefillsMeter(_) => GameEventType::KillRefillsMeter,
            GameEvent::RpsTauntEvent(_) => GameEventType::RpsTauntEvent,
            GameEvent::CongaKill(_) => GameEventType::CongaKill,
            GameEvent::PlayerInitialSpawn(_) => GameEventType::PlayerInitialSpawn,
            GameEvent::CompetitiveVictory(_) => GameEventType::CompetitiveVictory,
            GameEvent::CompetitiveStatsUpdate(_) => GameEventType::CompetitiveStatsUpdate,
            GameEvent::MiniGameWin(_) => GameEventType::MiniGameWin,
            GameEvent::SentryOnGoActive(_) => GameEventType::SentryOnGoActive,
            GameEvent::DuckXpLevelUp(_) => GameEventType::DuckXpLevelUp,
            GameEvent::QuestLogOpened(_) => GameEventType::QuestLogOpened,
            GameEvent::SchemaUpdated(_) => GameEventType::SchemaUpdated,
            GameEvent::LocalPlayerPickupWeapon(_) => {
                GameEventType::LocalPlayerPickupWeapon
            }
            GameEvent::RdPlayerScorePoints(_) => GameEventType::RdPlayerScorePoints,
            GameEvent::DemomanDetStickies(_) => GameEventType::DemomanDetStickies,
            GameEvent::QuestObjectiveCompleted(_) => {
                GameEventType::QuestObjectiveCompleted
            }
            GameEvent::PlayerScoreChanged(_) => GameEventType::PlayerScoreChanged,
            GameEvent::KilledCappingPlayer(_) => GameEventType::KilledCappingPlayer,
            GameEvent::EnvironmentalDeath(_) => GameEventType::EnvironmentalDeath,
            GameEvent::ProjectileDirectHit(_) => GameEventType::ProjectileDirectHit,
            GameEvent::PassGet(_) => GameEventType::PassGet,
            GameEvent::PassScore(_) => GameEventType::PassScore,
            GameEvent::PassFree(_) => GameEventType::PassFree,
            GameEvent::PassPassCaught(_) => GameEventType::PassPassCaught,
            GameEvent::PassBallStolen(_) => GameEventType::PassBallStolen,
            GameEvent::PassBallBlocked(_) => GameEventType::PassBallBlocked,
            GameEvent::DamagePrevented(_) => GameEventType::DamagePrevented,
            GameEvent::HalloweenBossKilled(_) => GameEventType::HalloweenBossKilled,
            GameEvent::EscapedLootIsland(_) => GameEventType::EscapedLootIsland,
            GameEvent::TaggedPlayerAsIt(_) => GameEventType::TaggedPlayerAsIt,
            GameEvent::MerasmusStunned(_) => GameEventType::MerasmusStunned,
            GameEvent::MerasmusPropFound(_) => GameEventType::MerasmusPropFound,
            GameEvent::HalloweenSkeletonKilled(_) => {
                GameEventType::HalloweenSkeletonKilled
            }
            GameEvent::EscapeHell(_) => GameEventType::EscapeHell,
            GameEvent::CrossSpectralBridge(_) => GameEventType::CrossSpectralBridge,
            GameEvent::MiniGameWon(_) => GameEventType::MiniGameWon,
            GameEvent::RespawnGhost(_) => GameEventType::RespawnGhost,
            GameEvent::KillInHell(_) => GameEventType::KillInHell,
            GameEvent::HalloweenDuckCollected(_) => GameEventType::HalloweenDuckCollected,
            GameEvent::SpecialScore(_) => GameEventType::SpecialScore,
            GameEvent::TeamLeaderKilled(_) => GameEventType::TeamLeaderKilled,
            GameEvent::HalloweenSoulCollected(_) => GameEventType::HalloweenSoulCollected,
            GameEvent::RecalculateTruce(_) => GameEventType::RecalculateTruce,
            GameEvent::DeadRingerCheatDeath(_) => GameEventType::DeadRingerCheatDeath,
            GameEvent::CrossbowHeal(_) => GameEventType::CrossbowHeal,
            GameEvent::DamageMitigated(_) => GameEventType::DamageMitigated,
            GameEvent::PayloadPushed(_) => GameEventType::PayloadPushed,
            GameEvent::PlayerAbandonedMatch(_) => GameEventType::PlayerAbandonedMatch,
            GameEvent::ClDrawline(_) => GameEventType::ClDrawline,
            GameEvent::RestartTimerTime(_) => GameEventType::RestartTimerTime,
            GameEvent::WinLimitChanged(_) => GameEventType::WinLimitChanged,
            GameEvent::WinPanelShowScores(_) => GameEventType::WinPanelShowScores,
            GameEvent::TopStreamsRequestFinished(_) => {
                GameEventType::TopStreamsRequestFinished
            }
            GameEvent::CompetitiveStateChanged(_) => {
                GameEventType::CompetitiveStateChanged
            }
            GameEvent::GlobalWarDataUpdated(_) => GameEventType::GlobalWarDataUpdated,
            GameEvent::StopWatchChanged(_) => GameEventType::StopWatchChanged,
            GameEvent::DsStop(_) => GameEventType::DsStop,
            GameEvent::DsScreenshot(_) => GameEventType::DsScreenshot,
            GameEvent::ShowMatchSummary(_) => GameEventType::ShowMatchSummary,
            GameEvent::ExperienceChanged(_) => GameEventType::ExperienceChanged,
            GameEvent::BeginXpLerp(_) => GameEventType::BeginXpLerp,
            GameEvent::MatchmakerStatsUpdated(_) => GameEventType::MatchmakerStatsUpdated,
            GameEvent::RematchVotePeriodOver(_) => GameEventType::RematchVotePeriodOver,
            GameEvent::RematchFailedToCreate(_) => GameEventType::RematchFailedToCreate,
            GameEvent::PlayerRematchChange(_) => GameEventType::PlayerRematchChange,
            GameEvent::PingUpdated(_) => GameEventType::PingUpdated,
            GameEvent::MMStatsUpdated(_) => GameEventType::MMStatsUpdated,
            GameEvent::PlayerNextMapVoteChange(_) => {
                GameEventType::PlayerNextMapVoteChange
            }
            GameEvent::VoteMapsChanged(_) => GameEventType::VoteMapsChanged,
            GameEvent::ProtoDefChanged(_) => GameEventType::ProtoDefChanged,
            GameEvent::PlayerDomination(_) => GameEventType::PlayerDomination,
            GameEvent::PlayerRocketPackPushed(_) => GameEventType::PlayerRocketPackPushed,
            GameEvent::QuestRequest(_) => GameEventType::QuestRequest,
            GameEvent::QuestResponse(_) => GameEventType::QuestResponse,
            GameEvent::QuestProgress(_) => GameEventType::QuestProgress,
            GameEvent::ProjectileRemoved(_) => GameEventType::ProjectileRemoved,
            GameEvent::QuestMapDataChanged(_) => GameEventType::QuestMapDataChanged,
            GameEvent::GasDousedPlayerIgnited(_) => GameEventType::GasDousedPlayerIgnited,
            GameEvent::QuestTurnInState(_) => GameEventType::QuestTurnInState,
            GameEvent::ItemsAcknowledged(_) => GameEventType::ItemsAcknowledged,
            GameEvent::CapperKilled(_) => GameEventType::CapperKilled,
            GameEvent::MainMenuStabilized(_) => GameEventType::MainMenuStabilized,
            GameEvent::WorldStatusChanged(_) => GameEventType::WorldStatusChanged,
            GameEvent::HLTVStatus(_) => GameEventType::HLTVStatus,
            GameEvent::HLTVCameraman(_) => GameEventType::HLTVCameraman,
            GameEvent::HLTVRankCamera(_) => GameEventType::HLTVRankCamera,
            GameEvent::HLTVRankEntity(_) => GameEventType::HLTVRankEntity,
            GameEvent::HLTVFixed(_) => GameEventType::HLTVFixed,
            GameEvent::HLTVChase(_) => GameEventType::HLTVChase,
            GameEvent::HLTVMessage(_) => GameEventType::HLTVMessage,
            GameEvent::HLTVTitle(_) => GameEventType::HLTVTitle,
            GameEvent::HLTVChat(_) => GameEventType::HLTVChat,
            GameEvent::ReplayStartRecord(_) => GameEventType::ReplayStartRecord,
            GameEvent::ReplaySessionInfo(_) => GameEventType::ReplaySessionInfo,
            GameEvent::ReplayEndRecord(_) => GameEventType::ReplayEndRecord,
            GameEvent::ReplayReplaysAvailable(_) => GameEventType::ReplayReplaysAvailable,
            GameEvent::ReplayServerError(_) => GameEventType::ReplayServerError,
            GameEvent::Unknown(raw) => raw.event_type.clone(),
        }
    }
}
pub fn get_sizes() -> fnv::FnvHashMap<&'static str, usize> {
    [
        ("ServerSpawn", std::mem::size_of::<ServerSpawnEvent>()),
        ("ServerChangeLevelFailed", std::mem::size_of::<ServerChangeLevelFailedEvent>()),
        ("ServerShutdown", std::mem::size_of::<ServerShutdownEvent>()),
        ("ServerCvar", std::mem::size_of::<ServerCvarEvent>()),
        ("ServerMessage", std::mem::size_of::<ServerMessageEvent>()),
        ("ServerAddBan", std::mem::size_of::<ServerAddBanEvent>()),
        ("ServerRemoveBan", std::mem::size_of::<ServerRemoveBanEvent>()),
        ("PlayerConnect", std::mem::size_of::<PlayerConnectEvent>()),
        ("PlayerConnectClient", std::mem::size_of::<PlayerConnectClientEvent>()),
        ("PlayerInfo", std::mem::size_of::<PlayerInfoEvent>()),
        ("PlayerDisconnect", std::mem::size_of::<PlayerDisconnectEvent>()),
        ("PlayerActivate", std::mem::size_of::<PlayerActivateEvent>()),
        ("PlayerSay", std::mem::size_of::<PlayerSayEvent>()),
        ("ClientDisconnect", std::mem::size_of::<ClientDisconnectEvent>()),
        ("ClientBeginConnect", std::mem::size_of::<ClientBeginConnectEvent>()),
        ("ClientConnected", std::mem::size_of::<ClientConnectedEvent>()),
        ("ClientFullConnect", std::mem::size_of::<ClientFullConnectEvent>()),
        ("HostQuit", std::mem::size_of::<HostQuitEvent>()),
        ("TeamInfo", std::mem::size_of::<TeamInfoEvent>()),
        ("TeamScore", std::mem::size_of::<TeamScoreEvent>()),
        ("TeamPlayBroadcastAudio", std::mem::size_of::<TeamPlayBroadcastAudioEvent>()),
        ("PlayerTeam", std::mem::size_of::<PlayerTeamEvent>()),
        ("PlayerClass", std::mem::size_of::<PlayerClassEvent>()),
        ("PlayerDeath", std::mem::size_of::<PlayerDeathEvent>()),
        ("PlayerHurt", std::mem::size_of::<PlayerHurtEvent>()),
        ("PlayerChat", std::mem::size_of::<PlayerChatEvent>()),
        ("PlayerScore", std::mem::size_of::<PlayerScoreEvent>()),
        ("PlayerSpawn", std::mem::size_of::<PlayerSpawnEvent>()),
        ("PlayerShoot", std::mem::size_of::<PlayerShootEvent>()),
        ("PlayerUse", std::mem::size_of::<PlayerUseEvent>()),
        ("PlayerChangeName", std::mem::size_of::<PlayerChangeNameEvent>()),
        ("PlayerHintMessage", std::mem::size_of::<PlayerHintMessageEvent>()),
        ("BasePlayerTeleported", std::mem::size_of::<BasePlayerTeleportedEvent>()),
        ("GameInit", std::mem::size_of::<GameInitEvent>()),
        ("GameNewMap", std::mem::size_of::<GameNewMapEvent>()),
        ("GameStart", std::mem::size_of::<GameStartEvent>()),
        ("GameEnd", std::mem::size_of::<GameEndEvent>()),
        ("RoundStart", std::mem::size_of::<RoundStartEvent>()),
        ("RoundEnd", std::mem::size_of::<RoundEndEvent>()),
        ("GameMessage", std::mem::size_of::<GameMessageEvent>()),
        ("BreakBreakable", std::mem::size_of::<BreakBreakableEvent>()),
        ("BreakProp", std::mem::size_of::<BreakPropEvent>()),
        ("EntityKilled", std::mem::size_of::<EntityKilledEvent>()),
        ("BonusUpdated", std::mem::size_of::<BonusUpdatedEvent>()),
        ("AchievementEvent", std::mem::size_of::<AchievementEventEvent>()),
        ("AchievementIncrement", std::mem::size_of::<AchievementIncrementEvent>()),
        ("PhysgunPickup", std::mem::size_of::<PhysgunPickupEvent>()),
        ("FlareIgniteNpc", std::mem::size_of::<FlareIgniteNpcEvent>()),
        (
            "HelicopterGrenadePuntMiss",
            std::mem::size_of::<HelicopterGrenadePuntMissEvent>(),
        ),
        ("UserDataDownloaded", std::mem::size_of::<UserDataDownloadedEvent>()),
        ("RagdollDissolved", std::mem::size_of::<RagdollDissolvedEvent>()),
        ("HLTVChangedMode", std::mem::size_of::<HLTVChangedModeEvent>()),
        ("HLTVChangedTarget", std::mem::size_of::<HLTVChangedTargetEvent>()),
        ("VoteEnded", std::mem::size_of::<VoteEndedEvent>()),
        ("VoteStarted", std::mem::size_of::<VoteStartedEvent>()),
        ("VoteChanged", std::mem::size_of::<VoteChangedEvent>()),
        ("VotePassed", std::mem::size_of::<VotePassedEvent>()),
        ("VoteFailed", std::mem::size_of::<VoteFailedEvent>()),
        ("VoteCast", std::mem::size_of::<VoteCastEvent>()),
        ("VoteOptions", std::mem::size_of::<VoteOptionsEvent>()),
        ("ReplaySaved", std::mem::size_of::<ReplaySavedEvent>()),
        ("EnteredPerformanceMode", std::mem::size_of::<EnteredPerformanceModeEvent>()),
        ("BrowseReplays", std::mem::size_of::<BrowseReplaysEvent>()),
        ("ReplayYoutubeStats", std::mem::size_of::<ReplayYoutubeStatsEvent>()),
        ("InventoryUpdated", std::mem::size_of::<InventoryUpdatedEvent>()),
        ("CartUpdated", std::mem::size_of::<CartUpdatedEvent>()),
        ("StorePriceSheetUpdated", std::mem::size_of::<StorePriceSheetUpdatedEvent>()),
        ("EconInventoryConnected", std::mem::size_of::<EconInventoryConnectedEvent>()),
        ("ItemSchemaInitialized", std::mem::size_of::<ItemSchemaInitializedEvent>()),
        ("GcNewSession", std::mem::size_of::<GcNewSessionEvent>()),
        ("GcLostSession", std::mem::size_of::<GcLostSessionEvent>()),
        ("IntroFinish", std::mem::size_of::<IntroFinishEvent>()),
        ("IntroNextCamera", std::mem::size_of::<IntroNextCameraEvent>()),
        ("PlayerChangeClass", std::mem::size_of::<PlayerChangeClassEvent>()),
        ("TfMapTimeRemaining", std::mem::size_of::<TfMapTimeRemainingEvent>()),
        ("TfGameOver", std::mem::size_of::<TfGameOverEvent>()),
        ("CtfFlagCaptured", std::mem::size_of::<CtfFlagCapturedEvent>()),
        ("ControlPointInitialized", std::mem::size_of::<ControlPointInitializedEvent>()),
        (
            "ControlPointUpdateImages",
            std::mem::size_of::<ControlPointUpdateImagesEvent>(),
        ),
        (
            "ControlPointUpdateLayout",
            std::mem::size_of::<ControlPointUpdateLayoutEvent>(),
        ),
        (
            "ControlPointUpdateCapping",
            std::mem::size_of::<ControlPointUpdateCappingEvent>(),
        ),
        ("ControlPointUpdateOwner", std::mem::size_of::<ControlPointUpdateOwnerEvent>()),
        ("ControlPointStartTouch", std::mem::size_of::<ControlPointStartTouchEvent>()),
        ("ControlPointEndTouch", std::mem::size_of::<ControlPointEndTouchEvent>()),
        (
            "ControlPointPulseElement",
            std::mem::size_of::<ControlPointPulseElementEvent>(),
        ),
        ("ControlPointFakeCapture", std::mem::size_of::<ControlPointFakeCaptureEvent>()),
        (
            "ControlPointFakeCaptureMultiplier",
            std::mem::size_of::<ControlPointFakeCaptureMultiplierEvent>(),
        ),
        ("TeamPlayRoundSelected", std::mem::size_of::<TeamPlayRoundSelectedEvent>()),
        ("TeamPlayRoundStart", std::mem::size_of::<TeamPlayRoundStartEvent>()),
        ("TeamPlayRoundActive", std::mem::size_of::<TeamPlayRoundActiveEvent>()),
        ("TeamPlayWaitingBegins", std::mem::size_of::<TeamPlayWaitingBeginsEvent>()),
        ("TeamPlayWaitingEnds", std::mem::size_of::<TeamPlayWaitingEndsEvent>()),
        (
            "TeamPlayWaitingAboutToEnd",
            std::mem::size_of::<TeamPlayWaitingAboutToEndEvent>(),
        ),
        ("TeamPlayRestartRound", std::mem::size_of::<TeamPlayRestartRoundEvent>()),
        ("TeamPlayReadyRestart", std::mem::size_of::<TeamPlayReadyRestartEvent>()),
        (
            "TeamPlayRoundRestartSeconds",
            std::mem::size_of::<TeamPlayRoundRestartSecondsEvent>(),
        ),
        ("TeamPlayTeamReady", std::mem::size_of::<TeamPlayTeamReadyEvent>()),
        ("TeamPlayRoundWin", std::mem::size_of::<TeamPlayRoundWinEvent>()),
        ("TeamPlayUpdateTimer", std::mem::size_of::<TeamPlayUpdateTimerEvent>()),
        ("TeamPlayRoundStalemate", std::mem::size_of::<TeamPlayRoundStalemateEvent>()),
        ("TeamPlayOvertimeBegin", std::mem::size_of::<TeamPlayOvertimeBeginEvent>()),
        ("TeamPlayOvertimeEnd", std::mem::size_of::<TeamPlayOvertimeEndEvent>()),
        (
            "TeamPlaySuddenDeathBegin",
            std::mem::size_of::<TeamPlaySuddenDeathBeginEvent>(),
        ),
        ("TeamPlaySuddenDeathEnd", std::mem::size_of::<TeamPlaySuddenDeathEndEvent>()),
        ("TeamPlayGameOver", std::mem::size_of::<TeamPlayGameOverEvent>()),
        (
            "TeamPlayMapTimeRemaining",
            std::mem::size_of::<TeamPlayMapTimeRemainingEvent>(),
        ),
        ("TeamPlayTimerFlash", std::mem::size_of::<TeamPlayTimerFlashEvent>()),
        ("TeamPlayTimerTimeAdded", std::mem::size_of::<TeamPlayTimerTimeAddedEvent>()),
        (
            "TeamPlayPointStartCapture",
            std::mem::size_of::<TeamPlayPointStartCaptureEvent>(),
        ),
        ("TeamPlayPointCaptured", std::mem::size_of::<TeamPlayPointCapturedEvent>()),
        ("TeamPlayPointLocked", std::mem::size_of::<TeamPlayPointLockedEvent>()),
        ("TeamPlayPointUnlocked", std::mem::size_of::<TeamPlayPointUnlockedEvent>()),
        ("TeamPlayCaptureBroken", std::mem::size_of::<TeamPlayCaptureBrokenEvent>()),
        ("TeamPlayCaptureBlocked", std::mem::size_of::<TeamPlayCaptureBlockedEvent>()),
        ("TeamPlayFlagEvent", std::mem::size_of::<TeamPlayFlagEventEvent>()),
        ("TeamPlayWinPanel", std::mem::size_of::<TeamPlayWinPanelEvent>()),
        (
            "TeamPlayTeamBalancedPlayer",
            std::mem::size_of::<TeamPlayTeamBalancedPlayerEvent>(),
        ),
        ("TeamPlaySetupFinished", std::mem::size_of::<TeamPlaySetupFinishedEvent>()),
        ("TeamPlayAlert", std::mem::size_of::<TeamPlayAlertEvent>()),
        ("TrainingComplete", std::mem::size_of::<TrainingCompleteEvent>()),
        ("ShowFreezePanel", std::mem::size_of::<ShowFreezePanelEvent>()),
        ("HideFreezePanel", std::mem::size_of::<HideFreezePanelEvent>()),
        ("FreezeCamStarted", std::mem::size_of::<FreezeCamStartedEvent>()),
        ("LocalPlayerChangeTeam", std::mem::size_of::<LocalPlayerChangeTeamEvent>()),
        ("LocalPlayerScoreChanged", std::mem::size_of::<LocalPlayerScoreChangedEvent>()),
        ("LocalPlayerChangeClass", std::mem::size_of::<LocalPlayerChangeClassEvent>()),
        ("LocalPlayerRespawn", std::mem::size_of::<LocalPlayerRespawnEvent>()),
        ("BuildingInfoChanged", std::mem::size_of::<BuildingInfoChangedEvent>()),
        (
            "LocalPlayerChangeDisguise",
            std::mem::size_of::<LocalPlayerChangeDisguiseEvent>(),
        ),
        ("PlayerAccountChanged", std::mem::size_of::<PlayerAccountChangedEvent>()),
        ("SpyPdaReset", std::mem::size_of::<SpyPdaResetEvent>()),
        ("FlagStatusUpdate", std::mem::size_of::<FlagStatusUpdateEvent>()),
        ("PlayerStatsUpdated", std::mem::size_of::<PlayerStatsUpdatedEvent>()),
        ("PlayingCommentary", std::mem::size_of::<PlayingCommentaryEvent>()),
        ("PlayerChargeDeployed", std::mem::size_of::<PlayerChargeDeployedEvent>()),
        ("PlayerBuiltObject", std::mem::size_of::<PlayerBuiltObjectEvent>()),
        ("PlayerUpgradedObject", std::mem::size_of::<PlayerUpgradedObjectEvent>()),
        ("PlayerCarryObject", std::mem::size_of::<PlayerCarryObjectEvent>()),
        ("PlayerDropObject", std::mem::size_of::<PlayerDropObjectEvent>()),
        ("ObjectRemoved", std::mem::size_of::<ObjectRemovedEvent>()),
        ("ObjectDestroyed", std::mem::size_of::<ObjectDestroyedEvent>()),
        ("ObjectDetonated", std::mem::size_of::<ObjectDetonatedEvent>()),
        ("AchievementEarned", std::mem::size_of::<AchievementEarnedEvent>()),
        ("SpecTargetUpdated", std::mem::size_of::<SpecTargetUpdatedEvent>()),
        ("TournamentStateUpdate", std::mem::size_of::<TournamentStateUpdateEvent>()),
        (
            "TournamentEnableCountdown",
            std::mem::size_of::<TournamentEnableCountdownEvent>(),
        ),
        ("PlayerCalledForMedic", std::mem::size_of::<PlayerCalledForMedicEvent>()),
        ("PlayerAskedForBall", std::mem::size_of::<PlayerAskedForBallEvent>()),
        (
            "LocalPlayerBecameObserver",
            std::mem::size_of::<LocalPlayerBecameObserverEvent>(),
        ),
        ("PlayerIgnitedInv", std::mem::size_of::<PlayerIgnitedInvEvent>()),
        ("PlayerIgnited", std::mem::size_of::<PlayerIgnitedEvent>()),
        ("PlayerExtinguished", std::mem::size_of::<PlayerExtinguishedEvent>()),
        ("PlayerTeleported", std::mem::size_of::<PlayerTeleportedEvent>()),
        ("PlayerHealedMedicCall", std::mem::size_of::<PlayerHealedMedicCallEvent>()),
        ("LocalPlayerChargeReady", std::mem::size_of::<LocalPlayerChargeReadyEvent>()),
        ("LocalPlayerWindDown", std::mem::size_of::<LocalPlayerWindDownEvent>()),
        ("PlayerInvulned", std::mem::size_of::<PlayerInvulnedEvent>()),
        ("EscortSpeed", std::mem::size_of::<EscortSpeedEvent>()),
        ("EscortProgress", std::mem::size_of::<EscortProgressEvent>()),
        ("EscortRecede", std::mem::size_of::<EscortRecedeEvent>()),
        ("GameUIActivated", std::mem::size_of::<GameUIActivatedEvent>()),
        ("GameUIHidden", std::mem::size_of::<GameUIHiddenEvent>()),
        ("PlayerEscortScore", std::mem::size_of::<PlayerEscortScoreEvent>()),
        ("PlayerHealOnHit", std::mem::size_of::<PlayerHealOnHitEvent>()),
        ("PlayerStealSandvich", std::mem::size_of::<PlayerStealSandvichEvent>()),
        ("ShowClassLayout", std::mem::size_of::<ShowClassLayoutEvent>()),
        ("ShowVsPanel", std::mem::size_of::<ShowVsPanelEvent>()),
        ("PlayerDamaged", std::mem::size_of::<PlayerDamagedEvent>()),
        ("ArenaPlayerNotification", std::mem::size_of::<ArenaPlayerNotificationEvent>()),
        ("ArenaMatchMaxStreak", std::mem::size_of::<ArenaMatchMaxStreakEvent>()),
        ("ArenaRoundStart", std::mem::size_of::<ArenaRoundStartEvent>()),
        ("ArenaWinPanel", std::mem::size_of::<ArenaWinPanelEvent>()),
        ("PveWinPanel", std::mem::size_of::<PveWinPanelEvent>()),
        ("AirDash", std::mem::size_of::<AirDashEvent>()),
        ("Landed", std::mem::size_of::<LandedEvent>()),
        ("PlayerDamageDodged", std::mem::size_of::<PlayerDamageDodgedEvent>()),
        ("PlayerStunned", std::mem::size_of::<PlayerStunnedEvent>()),
        ("ScoutGrandSlam", std::mem::size_of::<ScoutGrandSlamEvent>()),
        ("ScoutSlamdollLanded", std::mem::size_of::<ScoutSlamdollLandedEvent>()),
        ("ArrowImpact", std::mem::size_of::<ArrowImpactEvent>()),
        ("PlayerJarated", std::mem::size_of::<PlayerJaratedEvent>()),
        ("PlayerJaratedFade", std::mem::size_of::<PlayerJaratedFadeEvent>()),
        ("PlayerShieldBlocked", std::mem::size_of::<PlayerShieldBlockedEvent>()),
        ("PlayerPinned", std::mem::size_of::<PlayerPinnedEvent>()),
        ("PlayerHealedByMedic", std::mem::size_of::<PlayerHealedByMedicEvent>()),
        ("PlayerSappedObject", std::mem::size_of::<PlayerSappedObjectEvent>()),
        ("ItemFound", std::mem::size_of::<ItemFoundEvent>()),
        ("ShowAnnotation", std::mem::size_of::<ShowAnnotationEvent>()),
        ("HideAnnotation", std::mem::size_of::<HideAnnotationEvent>()),
        (
            "PostInventoryApplication",
            std::mem::size_of::<PostInventoryApplicationEvent>(),
        ),
        (
            "ControlPointUnlockUpdated",
            std::mem::size_of::<ControlPointUnlockUpdatedEvent>(),
        ),
        ("DeployBuffBanner", std::mem::size_of::<DeployBuffBannerEvent>()),
        ("PlayerBuff", std::mem::size_of::<PlayerBuffEvent>()),
        ("MedicDeath", std::mem::size_of::<MedicDeathEvent>()),
        ("OvertimeNag", std::mem::size_of::<OvertimeNagEvent>()),
        ("TeamsChanged", std::mem::size_of::<TeamsChangedEvent>()),
        ("HalloweenPumpkinGrab", std::mem::size_of::<HalloweenPumpkinGrabEvent>()),
        ("RocketJump", std::mem::size_of::<RocketJumpEvent>()),
        ("RocketJumpLanded", std::mem::size_of::<RocketJumpLandedEvent>()),
        ("StickyJump", std::mem::size_of::<StickyJumpEvent>()),
        ("StickyJumpLanded", std::mem::size_of::<StickyJumpLandedEvent>()),
        ("RocketPackLaunch", std::mem::size_of::<RocketPackLaunchEvent>()),
        ("RocketPackLanded", std::mem::size_of::<RocketPackLandedEvent>()),
        ("MedicDefended", std::mem::size_of::<MedicDefendedEvent>()),
        ("LocalPlayerHealed", std::mem::size_of::<LocalPlayerHealedEvent>()),
        ("PlayerDestroyedPipeBomb", std::mem::size_of::<PlayerDestroyedPipeBombEvent>()),
        ("ObjectDeflected", std::mem::size_of::<ObjectDeflectedEvent>()),
        ("PlayerMvp", std::mem::size_of::<PlayerMvpEvent>()),
        ("RaidSpawnMob", std::mem::size_of::<RaidSpawnMobEvent>()),
        ("RaidSpawnSquad", std::mem::size_of::<RaidSpawnSquadEvent>()),
        ("NavBlocked", std::mem::size_of::<NavBlockedEvent>()),
        ("PathTrackPassed", std::mem::size_of::<PathTrackPassedEvent>()),
        ("NumCappersChanged", std::mem::size_of::<NumCappersChangedEvent>()),
        ("PlayerRegenerate", std::mem::size_of::<PlayerRegenerateEvent>()),
        ("UpdateStatusItem", std::mem::size_of::<UpdateStatusItemEvent>()),
        ("StatsResetRound", std::mem::size_of::<StatsResetRoundEvent>()),
        (
            "ScoreStatsAccumulatedUpdate",
            std::mem::size_of::<ScoreStatsAccumulatedUpdateEvent>(),
        ),
        (
            "ScoreStatsAccumulatedReset",
            std::mem::size_of::<ScoreStatsAccumulatedResetEvent>(),
        ),
        ("AchievementEarnedLocal", std::mem::size_of::<AchievementEarnedLocalEvent>()),
        ("PlayerHealed", std::mem::size_of::<PlayerHealedEvent>()),
        ("BuildingHealed", std::mem::size_of::<BuildingHealedEvent>()),
        ("ItemPickup", std::mem::size_of::<ItemPickupEvent>()),
        ("DuelStatus", std::mem::size_of::<DuelStatusEvent>()),
        ("FishNotice", std::mem::size_of::<FishNoticeEvent>()),
        ("FishNoticeArm", std::mem::size_of::<FishNoticeArmEvent>()),
        ("SlapNotice", std::mem::size_of::<SlapNoticeEvent>()),
        ("ThrowableHit", std::mem::size_of::<ThrowableHitEvent>()),
        ("PumpkinLordSummoned", std::mem::size_of::<PumpkinLordSummonedEvent>()),
        ("PumpkinLordKilled", std::mem::size_of::<PumpkinLordKilledEvent>()),
        ("MerasmusSummoned", std::mem::size_of::<MerasmusSummonedEvent>()),
        ("MerasmusKilled", std::mem::size_of::<MerasmusKilledEvent>()),
        ("MerasmusEscapeWarning", std::mem::size_of::<MerasmusEscapeWarningEvent>()),
        ("MerasmusEscaped", std::mem::size_of::<MerasmusEscapedEvent>()),
        ("EyeballBossSummoned", std::mem::size_of::<EyeballBossSummonedEvent>()),
        ("EyeballBossStunned", std::mem::size_of::<EyeballBossStunnedEvent>()),
        ("EyeballBossKilled", std::mem::size_of::<EyeballBossKilledEvent>()),
        ("EyeballBossKiller", std::mem::size_of::<EyeballBossKillerEvent>()),
        (
            "EyeballBossEscapeImminent",
            std::mem::size_of::<EyeballBossEscapeImminentEvent>(),
        ),
        ("EyeballBossEscaped", std::mem::size_of::<EyeballBossEscapedEvent>()),
        ("NpcHurt", std::mem::size_of::<NpcHurtEvent>()),
        (
            "ControlPointTimerUpdated",
            std::mem::size_of::<ControlPointTimerUpdatedEvent>(),
        ),
        ("PlayerHighFiveStart", std::mem::size_of::<PlayerHighFiveStartEvent>()),
        ("PlayerHighFiveCancel", std::mem::size_of::<PlayerHighFiveCancelEvent>()),
        ("PlayerHighFiveSuccess", std::mem::size_of::<PlayerHighFiveSuccessEvent>()),
        ("PlayerBonusPoints", std::mem::size_of::<PlayerBonusPointsEvent>()),
        ("PlayerUpgraded", std::mem::size_of::<PlayerUpgradedEvent>()),
        ("PlayerBuyback", std::mem::size_of::<PlayerBuybackEvent>()),
        ("PlayerUsedPowerUpBottle", std::mem::size_of::<PlayerUsedPowerUpBottleEvent>()),
        ("ChristmasGiftGrab", std::mem::size_of::<ChristmasGiftGrabEvent>()),
        (
            "PlayerKilledAchievementZone",
            std::mem::size_of::<PlayerKilledAchievementZoneEvent>(),
        ),
        ("PartyUpdated", std::mem::size_of::<PartyUpdatedEvent>()),
        ("PartyPrefChanged", std::mem::size_of::<PartyPrefChangedEvent>()),
        ("PartyCriteriaChanged", std::mem::size_of::<PartyCriteriaChangedEvent>()),
        ("PartyInvitesChanged", std::mem::size_of::<PartyInvitesChangedEvent>()),
        ("PartyQueueStateChanged", std::mem::size_of::<PartyQueueStateChangedEvent>()),
        ("PartyChat", std::mem::size_of::<PartyChatEvent>()),
        ("PartyMemberJoin", std::mem::size_of::<PartyMemberJoinEvent>()),
        ("PartyMemberLeave", std::mem::size_of::<PartyMemberLeaveEvent>()),
        ("MatchInvitesUpdated", std::mem::size_of::<MatchInvitesUpdatedEvent>()),
        ("LobbyUpdated", std::mem::size_of::<LobbyUpdatedEvent>()),
        ("MvmMissionUpdate", std::mem::size_of::<MvmMissionUpdateEvent>()),
        ("RecalculateHolidays", std::mem::size_of::<RecalculateHolidaysEvent>()),
        ("PlayerCurrencyChanged", std::mem::size_of::<PlayerCurrencyChangedEvent>()),
        ("DoomsdayRocketOpen", std::mem::size_of::<DoomsdayRocketOpenEvent>()),
        (
            "RemoveNemesisRelationships",
            std::mem::size_of::<RemoveNemesisRelationshipsEvent>(),
        ),
        ("MvmCreditBonusWave", std::mem::size_of::<MvmCreditBonusWaveEvent>()),
        ("MvmCreditBonusAll", std::mem::size_of::<MvmCreditBonusAllEvent>()),
        (
            "MvmCreditBonusAllAdvanced",
            std::mem::size_of::<MvmCreditBonusAllAdvancedEvent>(),
        ),
        ("MvmQuickSentryUpgrade", std::mem::size_of::<MvmQuickSentryUpgradeEvent>()),
        (
            "MvmTankDestroyedByPlayers",
            std::mem::size_of::<MvmTankDestroyedByPlayersEvent>(),
        ),
        (
            "MvmKillRobotDeliveringBomb",
            std::mem::size_of::<MvmKillRobotDeliveringBombEvent>(),
        ),
        ("MvmPickupCurrency", std::mem::size_of::<MvmPickupCurrencyEvent>()),
        ("MvmBombCarrierKilled", std::mem::size_of::<MvmBombCarrierKilledEvent>()),
        ("MvmSentryBusterDetonate", std::mem::size_of::<MvmSentryBusterDetonateEvent>()),
        ("MvmScoutMarkedForDeath", std::mem::size_of::<MvmScoutMarkedForDeathEvent>()),
        ("MvmMedicPowerUpShared", std::mem::size_of::<MvmMedicPowerUpSharedEvent>()),
        ("MvmBeginWave", std::mem::size_of::<MvmBeginWaveEvent>()),
        ("MvmWaveComplete", std::mem::size_of::<MvmWaveCompleteEvent>()),
        ("MvmMissionComplete", std::mem::size_of::<MvmMissionCompleteEvent>()),
        ("MvmBombResetByPlayer", std::mem::size_of::<MvmBombResetByPlayerEvent>()),
        ("MvmBombAlarmTriggered", std::mem::size_of::<MvmBombAlarmTriggeredEvent>()),
        (
            "MvmBombDeployResetByPlayer",
            std::mem::size_of::<MvmBombDeployResetByPlayerEvent>(),
        ),
        ("MvmWaveFailed", std::mem::size_of::<MvmWaveFailedEvent>()),
        ("MvmResetStats", std::mem::size_of::<MvmResetStatsEvent>()),
        ("DamageResisted", std::mem::size_of::<DamageResistedEvent>()),
        ("RevivePlayerNotify", std::mem::size_of::<RevivePlayerNotifyEvent>()),
        ("RevivePlayerStopped", std::mem::size_of::<RevivePlayerStoppedEvent>()),
        ("RevivePlayerComplete", std::mem::size_of::<RevivePlayerCompleteEvent>()),
        ("PlayerTurnedToGhost", std::mem::size_of::<PlayerTurnedToGhostEvent>()),
        (
            "MedigunShieldBlockedDamage",
            std::mem::size_of::<MedigunShieldBlockedDamageEvent>(),
        ),
        (
            "MvmAdvWaveCompleteNoGates",
            std::mem::size_of::<MvmAdvWaveCompleteNoGatesEvent>(),
        ),
        (
            "MvmSniperHeadshotCurrency",
            std::mem::size_of::<MvmSniperHeadshotCurrencyEvent>(),
        ),
        ("MvmMannhattanPit", std::mem::size_of::<MvmMannhattanPitEvent>()),
        (
            "FlagCarriedInDetectionZone",
            std::mem::size_of::<FlagCarriedInDetectionZoneEvent>(),
        ),
        (
            "MvmAdvWaveKilledStunRadio",
            std::mem::size_of::<MvmAdvWaveKilledStunRadioEvent>(),
        ),
        ("PlayerDirectHitStun", std::mem::size_of::<PlayerDirectHitStunEvent>()),
        ("MvmSentryBusterKilled", std::mem::size_of::<MvmSentryBusterKilledEvent>()),
        ("UpgradesFileChanged", std::mem::size_of::<UpgradesFileChangedEvent>()),
        ("RdTeamPointsChanged", std::mem::size_of::<RdTeamPointsChangedEvent>()),
        ("RdRulesStateChanged", std::mem::size_of::<RdRulesStateChangedEvent>()),
        ("RdRobotKilled", std::mem::size_of::<RdRobotKilledEvent>()),
        ("RdRobotImpact", std::mem::size_of::<RdRobotImpactEvent>()),
        (
            "TeamPlayPreRoundTimeLeft",
            std::mem::size_of::<TeamPlayPreRoundTimeLeftEvent>(),
        ),
        ("ParachuteDeploy", std::mem::size_of::<ParachuteDeployEvent>()),
        ("ParachuteHolster", std::mem::size_of::<ParachuteHolsterEvent>()),
        ("KillRefillsMeter", std::mem::size_of::<KillRefillsMeterEvent>()),
        ("RpsTauntEvent", std::mem::size_of::<RpsTauntEventEvent>()),
        ("CongaKill", std::mem::size_of::<CongaKillEvent>()),
        ("PlayerInitialSpawn", std::mem::size_of::<PlayerInitialSpawnEvent>()),
        ("CompetitiveVictory", std::mem::size_of::<CompetitiveVictoryEvent>()),
        ("CompetitiveStatsUpdate", std::mem::size_of::<CompetitiveStatsUpdateEvent>()),
        ("MiniGameWin", std::mem::size_of::<MiniGameWinEvent>()),
        ("SentryOnGoActive", std::mem::size_of::<SentryOnGoActiveEvent>()),
        ("DuckXpLevelUp", std::mem::size_of::<DuckXpLevelUpEvent>()),
        ("QuestLogOpened", std::mem::size_of::<QuestLogOpenedEvent>()),
        ("SchemaUpdated", std::mem::size_of::<SchemaUpdatedEvent>()),
        ("LocalPlayerPickupWeapon", std::mem::size_of::<LocalPlayerPickupWeaponEvent>()),
        ("RdPlayerScorePoints", std::mem::size_of::<RdPlayerScorePointsEvent>()),
        ("DemomanDetStickies", std::mem::size_of::<DemomanDetStickiesEvent>()),
        ("QuestObjectiveCompleted", std::mem::size_of::<QuestObjectiveCompletedEvent>()),
        ("PlayerScoreChanged", std::mem::size_of::<PlayerScoreChangedEvent>()),
        ("KilledCappingPlayer", std::mem::size_of::<KilledCappingPlayerEvent>()),
        ("EnvironmentalDeath", std::mem::size_of::<EnvironmentalDeathEvent>()),
        ("ProjectileDirectHit", std::mem::size_of::<ProjectileDirectHitEvent>()),
        ("PassGet", std::mem::size_of::<PassGetEvent>()),
        ("PassScore", std::mem::size_of::<PassScoreEvent>()),
        ("PassFree", std::mem::size_of::<PassFreeEvent>()),
        ("PassPassCaught", std::mem::size_of::<PassPassCaughtEvent>()),
        ("PassBallStolen", std::mem::size_of::<PassBallStolenEvent>()),
        ("PassBallBlocked", std::mem::size_of::<PassBallBlockedEvent>()),
        ("DamagePrevented", std::mem::size_of::<DamagePreventedEvent>()),
        ("HalloweenBossKilled", std::mem::size_of::<HalloweenBossKilledEvent>()),
        ("EscapedLootIsland", std::mem::size_of::<EscapedLootIslandEvent>()),
        ("TaggedPlayerAsIt", std::mem::size_of::<TaggedPlayerAsItEvent>()),
        ("MerasmusStunned", std::mem::size_of::<MerasmusStunnedEvent>()),
        ("MerasmusPropFound", std::mem::size_of::<MerasmusPropFoundEvent>()),
        ("HalloweenSkeletonKilled", std::mem::size_of::<HalloweenSkeletonKilledEvent>()),
        ("EscapeHell", std::mem::size_of::<EscapeHellEvent>()),
        ("CrossSpectralBridge", std::mem::size_of::<CrossSpectralBridgeEvent>()),
        ("MiniGameWon", std::mem::size_of::<MiniGameWonEvent>()),
        ("RespawnGhost", std::mem::size_of::<RespawnGhostEvent>()),
        ("KillInHell", std::mem::size_of::<KillInHellEvent>()),
        ("HalloweenDuckCollected", std::mem::size_of::<HalloweenDuckCollectedEvent>()),
        ("SpecialScore", std::mem::size_of::<SpecialScoreEvent>()),
        ("TeamLeaderKilled", std::mem::size_of::<TeamLeaderKilledEvent>()),
        ("HalloweenSoulCollected", std::mem::size_of::<HalloweenSoulCollectedEvent>()),
        ("RecalculateTruce", std::mem::size_of::<RecalculateTruceEvent>()),
        ("DeadRingerCheatDeath", std::mem::size_of::<DeadRingerCheatDeathEvent>()),
        ("CrossbowHeal", std::mem::size_of::<CrossbowHealEvent>()),
        ("DamageMitigated", std::mem::size_of::<DamageMitigatedEvent>()),
        ("PayloadPushed", std::mem::size_of::<PayloadPushedEvent>()),
        ("PlayerAbandonedMatch", std::mem::size_of::<PlayerAbandonedMatchEvent>()),
        ("ClDrawline", std::mem::size_of::<ClDrawlineEvent>()),
        ("RestartTimerTime", std::mem::size_of::<RestartTimerTimeEvent>()),
        ("WinLimitChanged", std::mem::size_of::<WinLimitChangedEvent>()),
        ("WinPanelShowScores", std::mem::size_of::<WinPanelShowScoresEvent>()),
        (
            "TopStreamsRequestFinished",
            std::mem::size_of::<TopStreamsRequestFinishedEvent>(),
        ),
        ("CompetitiveStateChanged", std::mem::size_of::<CompetitiveStateChangedEvent>()),
        ("GlobalWarDataUpdated", std::mem::size_of::<GlobalWarDataUpdatedEvent>()),
        ("StopWatchChanged", std::mem::size_of::<StopWatchChangedEvent>()),
        ("DsStop", std::mem::size_of::<DsStopEvent>()),
        ("DsScreenshot", std::mem::size_of::<DsScreenshotEvent>()),
        ("ShowMatchSummary", std::mem::size_of::<ShowMatchSummaryEvent>()),
        ("ExperienceChanged", std::mem::size_of::<ExperienceChangedEvent>()),
        ("BeginXpLerp", std::mem::size_of::<BeginXpLerpEvent>()),
        ("MatchmakerStatsUpdated", std::mem::size_of::<MatchmakerStatsUpdatedEvent>()),
        ("RematchVotePeriodOver", std::mem::size_of::<RematchVotePeriodOverEvent>()),
        ("RematchFailedToCreate", std::mem::size_of::<RematchFailedToCreateEvent>()),
        ("PlayerRematchChange", std::mem::size_of::<PlayerRematchChangeEvent>()),
        ("PingUpdated", std::mem::size_of::<PingUpdatedEvent>()),
        ("MMStatsUpdated", std::mem::size_of::<MMStatsUpdatedEvent>()),
        ("PlayerNextMapVoteChange", std::mem::size_of::<PlayerNextMapVoteChangeEvent>()),
        ("VoteMapsChanged", std::mem::size_of::<VoteMapsChangedEvent>()),
        ("ProtoDefChanged", std::mem::size_of::<ProtoDefChangedEvent>()),
        ("PlayerDomination", std::mem::size_of::<PlayerDominationEvent>()),
        ("PlayerRocketPackPushed", std::mem::size_of::<PlayerRocketPackPushedEvent>()),
        ("QuestRequest", std::mem::size_of::<QuestRequestEvent>()),
        ("QuestResponse", std::mem::size_of::<QuestResponseEvent>()),
        ("QuestProgress", std::mem::size_of::<QuestProgressEvent>()),
        ("ProjectileRemoved", std::mem::size_of::<ProjectileRemovedEvent>()),
        ("QuestMapDataChanged", std::mem::size_of::<QuestMapDataChangedEvent>()),
        ("GasDousedPlayerIgnited", std::mem::size_of::<GasDousedPlayerIgnitedEvent>()),
        ("QuestTurnInState", std::mem::size_of::<QuestTurnInStateEvent>()),
        ("ItemsAcknowledged", std::mem::size_of::<ItemsAcknowledgedEvent>()),
        ("CapperKilled", std::mem::size_of::<CapperKilledEvent>()),
        ("MainMenuStabilized", std::mem::size_of::<MainMenuStabilizedEvent>()),
        ("WorldStatusChanged", std::mem::size_of::<WorldStatusChangedEvent>()),
        ("HLTVStatus", std::mem::size_of::<HLTVStatusEvent>()),
        ("HLTVCameraman", std::mem::size_of::<HLTVCameramanEvent>()),
        ("HLTVRankCamera", std::mem::size_of::<HLTVRankCameraEvent>()),
        ("HLTVRankEntity", std::mem::size_of::<HLTVRankEntityEvent>()),
        ("HLTVFixed", std::mem::size_of::<HLTVFixedEvent>()),
        ("HLTVChase", std::mem::size_of::<HLTVChaseEvent>()),
        ("HLTVMessage", std::mem::size_of::<HLTVMessageEvent>()),
        ("HLTVTitle", std::mem::size_of::<HLTVTitleEvent>()),
        ("HLTVChat", std::mem::size_of::<HLTVChatEvent>()),
        ("ReplayStartRecord", std::mem::size_of::<ReplayStartRecordEvent>()),
        ("ReplaySessionInfo", std::mem::size_of::<ReplaySessionInfoEvent>()),
        ("ReplayEndRecord", std::mem::size_of::<ReplayEndRecordEvent>()),
        ("ReplayReplaysAvailable", std::mem::size_of::<ReplayReplaysAvailableEvent>()),
        ("ReplayServerError", std::mem::size_of::<ReplayServerErrorEvent>()),
    ]
        .iter()
        .copied()
        .collect()
}

