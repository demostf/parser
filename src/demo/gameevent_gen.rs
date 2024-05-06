use super::gamevent::{
    EventValue, GameEventDefinition, GameEventEntry, GameEventValue, RawGameEvent,
};
use crate::demo::data::MaybeUtf8String;
use crate::demo::Stream;
use crate::{ParseError, Result};
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};
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
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ServerSpawnEvent {
            hostname: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("hostname"),
                "hostname",
            )?,
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("address"),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry("ip"), "ip")?,
            port: read_value::<u16>(stream, definition.get_entry("port"), "port")?,
            game: read_value::<MaybeUtf8String>(stream, definition.get_entry("game"), "game")?,
            map_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("mapname"),
                "map_name",
            )?,
            max_players: read_value::<u32>(
                stream,
                definition.get_entry("maxplayers"),
                "max_players",
            )?,
            os: read_value::<MaybeUtf8String>(stream, definition.get_entry("os"), "os")?,
            dedicated: read_value::<bool>(stream, definition.get_entry("dedicated"), "dedicated")?,
            password: read_value::<bool>(stream, definition.get_entry("password"), "password")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "hostname" => Ok(self.hostname.clone().into()),
            "address" => Ok(self.address.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "port" => Ok(self.port.clone().into()),
            "game" => Ok(self.game.clone().into()),
            "mapname" => Ok(self.map_name.clone().into()),
            "maxplayers" => Ok(self.max_players.clone().into()),
            "os" => Ok(self.os.clone().into()),
            "dedicated" => Ok(self.dedicated.clone().into()),
            "password" => Ok(self.password.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerSpawn",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerChangeLevelFailedEvent {
    pub level_name: MaybeUtf8String,
}
impl ServerChangeLevelFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerChangeLevelFailedEvent {
            level_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("levelname"),
                "level_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "levelname" => Ok(self.level_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerChangeLevelFailed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerShutdownEvent {
    pub reason: MaybeUtf8String,
}
impl ServerShutdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerShutdownEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("reason"),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "reason" => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerShutdown",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerCvarEvent {
    pub cvar_name: MaybeUtf8String,
    pub cvar_value: MaybeUtf8String,
}
impl ServerCvarEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerCvarEvent {
            cvar_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cvarname"),
                "cvar_name",
            )?,
            cvar_value: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cvarvalue"),
                "cvar_value",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cvarname" => Ok(self.cvar_name.clone().into()),
            "cvarvalue" => Ok(self.cvar_value.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerCvar",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerMessageEvent {
    pub text: MaybeUtf8String,
}
impl ServerMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerMessageEvent {
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerMessage",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ServerAddBanEvent {
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            ip: read_value::<MaybeUtf8String>(stream, definition.get_entry("ip"), "ip")?,
            duration: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("duration"),
                "duration",
            )?,
            by: read_value::<MaybeUtf8String>(stream, definition.get_entry("by"), "by")?,
            kicked: read_value::<bool>(stream, definition.get_entry("kicked"), "kicked")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "name" => Ok(self.name.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "networkid" => Ok(self.network_id.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "duration" => Ok(self.duration.clone().into()),
            "by" => Ok(self.by.clone().into()),
            "kicked" => Ok(self.kicked.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerAddBan",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerRemoveBanEvent {
    pub network_id: MaybeUtf8String,
    pub ip: MaybeUtf8String,
    pub by: MaybeUtf8String,
}
impl ServerRemoveBanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ServerRemoveBanEvent {
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            ip: read_value::<MaybeUtf8String>(stream, definition.get_entry("ip"), "ip")?,
            by: read_value::<MaybeUtf8String>(stream, definition.get_entry("by"), "by")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "networkid" => Ok(self.network_id.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "by" => Ok(self.by.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ServerRemoveBan",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerConnectEvent {
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
            index: read_value::<u8>(stream, definition.get_entry("index"), "index")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("address"),
                "address",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry("bot"), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "name" => Ok(self.name.clone().into()),
            "index" => Ok(self.index.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "networkid" => Ok(self.network_id.clone().into()),
            "address" => Ok(self.address.clone().into()),
            "bot" => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerConnect",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerConnectClientEvent {
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
            index: read_value::<u8>(stream, definition.get_entry("index"), "index")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry("bot"), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "name" => Ok(self.name.clone().into()),
            "index" => Ok(self.index.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "networkid" => Ok(self.network_id.clone().into()),
            "bot" => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerConnectClient",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerInfoEvent {
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
            index: read_value::<u8>(stream, definition.get_entry("index"), "index")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            bot: read_value::<bool>(stream, definition.get_entry("bot"), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "name" => Ok(self.name.clone().into()),
            "index" => Ok(self.index.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "networkid" => Ok(self.network_id.clone().into()),
            "bot" => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerInfo",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerDisconnectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("reason"),
                "reason",
            )?,
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
            network_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("networkid"),
                "network_id",
            )?,
            bot: read_value::<u16>(stream, definition.get_entry("bot"), "bot")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "reason" => Ok(self.reason.clone().into()),
            "name" => Ok(self.name.clone().into()),
            "networkid" => Ok(self.network_id.clone().into()),
            "bot" => Ok(self.bot.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDisconnect",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerActivateEvent {
    pub user_id: u16,
}
impl PlayerActivateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerActivateEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerActivate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSayEvent {
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerSayEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSayEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSay",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientDisconnectEvent {
    pub message: MaybeUtf8String,
}
impl ClientDisconnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientDisconnectEvent {
            message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("message"),
                "message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "message" => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientDisconnect",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientBeginConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
    pub source: MaybeUtf8String,
}
impl ClientBeginConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientBeginConnectEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("address"),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry("ip"), "ip")?,
            port: read_value::<u16>(stream, definition.get_entry("port"), "port")?,
            source: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("source"),
                "source",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "address" => Ok(self.address.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "port" => Ok(self.port.clone().into()),
            "source" => Ok(self.source.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientBeginConnect",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientConnectedEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientConnectedEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("address"),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry("ip"), "ip")?,
            port: read_value::<u16>(stream, definition.get_entry("port"), "port")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "address" => Ok(self.address.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "port" => Ok(self.port.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientConnected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClientFullConnectEvent {
    pub address: MaybeUtf8String,
    pub ip: u32,
    pub port: u16,
}
impl ClientFullConnectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ClientFullConnectEvent {
            address: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("address"),
                "address",
            )?,
            ip: read_value::<u32>(stream, definition.get_entry("ip"), "ip")?,
            port: read_value::<u16>(stream, definition.get_entry("port"), "port")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "address" => Ok(self.address.clone().into()),
            "ip" => Ok(self.ip.clone().into()),
            "port" => Ok(self.port.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClientFullConnect",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HostQuitEvent {}
impl HostQuitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HostQuitEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HostQuit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamInfoEvent {
    pub team_id: u8,
    pub team_name: MaybeUtf8String,
}
impl TeamInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamInfoEvent {
            team_id: read_value::<u8>(stream, definition.get_entry("teamid"), "team_id")?,
            team_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("teamname"),
                "team_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "teamid" => Ok(self.team_id.clone().into()),
            "teamname" => Ok(self.team_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamInfo",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamScoreEvent {
    pub team_id: u8,
    pub score: u16,
}
impl TeamScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamScoreEvent {
            team_id: read_value::<u8>(stream, definition.get_entry("teamid"), "team_id")?,
            score: read_value::<u16>(stream, definition.get_entry("score"), "score")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "teamid" => Ok(self.team_id.clone().into()),
            "score" => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamScore",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayBroadcastAudioEvent {
    pub team: u8,
    pub sound: MaybeUtf8String,
    pub additional_flags: u16,
    pub player: u16,
}
impl TeamPlayBroadcastAudioEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayBroadcastAudioEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            sound: read_value::<MaybeUtf8String>(stream, definition.get_entry("sound"), "sound")?,
            additional_flags: read_value::<u16>(
                stream,
                definition.get_entry("additional_flags"),
                "additional_flags",
            )?,
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "sound" => Ok(self.sound.clone().into()),
            "additional_flags" => Ok(self.additional_flags.clone().into()),
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayBroadcastAudio",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerTeamEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            old_team: read_value::<u8>(stream, definition.get_entry("oldteam"), "old_team")?,
            disconnect: read_value::<bool>(
                stream,
                definition.get_entry("disconnect"),
                "disconnect",
            )?,
            auto_team: read_value::<bool>(stream, definition.get_entry("autoteam"), "auto_team")?,
            silent: read_value::<bool>(stream, definition.get_entry("silent"), "silent")?,
            name: read_value::<MaybeUtf8String>(stream, definition.get_entry("name"), "name")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "oldteam" => Ok(self.old_team.clone().into()),
            "disconnect" => Ok(self.disconnect.clone().into()),
            "autoteam" => Ok(self.auto_team.clone().into()),
            "silent" => Ok(self.silent.clone().into()),
            "name" => Ok(self.name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerTeam",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerClassEvent {
    pub user_id: u16,
    pub class: MaybeUtf8String,
}
impl PlayerClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerClassEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            class: read_value::<MaybeUtf8String>(stream, definition.get_entry("class"), "class")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "class" => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerClass",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerDeathEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry("stun_flags"),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry("death_flags"),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry("silent_kill"),
                "silent_kill",
            )?,
            player_penetrate_count: read_value::<u16>(
                stream,
                definition.get_entry("playerpenetratecount"),
                "player_penetrate_count",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("assister_fallback"),
                "assister_fallback",
            )?,
            kill_streak_total: read_value::<u16>(
                stream,
                definition.get_entry("kill_streak_total"),
                "kill_streak_total",
            )?,
            kill_streak_wep: read_value::<u16>(
                stream,
                definition.get_entry("kill_streak_wep"),
                "kill_streak_wep",
            )?,
            kill_streak_assist: read_value::<u16>(
                stream,
                definition.get_entry("kill_streak_assist"),
                "kill_streak_assist",
            )?,
            kill_streak_victim: read_value::<u16>(
                stream,
                definition.get_entry("kill_streak_victim"),
                "kill_streak_victim",
            )?,
            ducks_streaked: read_value::<u16>(
                stream,
                definition.get_entry("ducks_streaked"),
                "ducks_streaked",
            )?,
            duck_streak_total: read_value::<u16>(
                stream,
                definition.get_entry("duck_streak_total"),
                "duck_streak_total",
            )?,
            duck_streak_assist: read_value::<u16>(
                stream,
                definition.get_entry("duck_streak_assist"),
                "duck_streak_assist",
            )?,
            duck_streak_victim: read_value::<u16>(
                stream,
                definition.get_entry("duck_streak_victim"),
                "duck_streak_victim",
            )?,
            rocket_jump: read_value::<bool>(
                stream,
                definition.get_entry("rocket_jump"),
                "rocket_jump",
            )?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry("weapon_def_index"),
                "weapon_def_index",
            )?,
            crit_type: read_value::<u16>(stream, definition.get_entry("crit_type"), "crit_type")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            "stun_flags" => Ok(self.stun_flags.clone().into()),
            "death_flags" => Ok(self.death_flags.clone().into()),
            "silent_kill" => Ok(self.silent_kill.clone().into()),
            "playerpenetratecount" => Ok(self.player_penetrate_count.clone().into()),
            "assister_fallback" => Ok(self.assister_fallback.clone().into()),
            "kill_streak_total" => Ok(self.kill_streak_total.clone().into()),
            "kill_streak_wep" => Ok(self.kill_streak_wep.clone().into()),
            "kill_streak_assist" => Ok(self.kill_streak_assist.clone().into()),
            "kill_streak_victim" => Ok(self.kill_streak_victim.clone().into()),
            "ducks_streaked" => Ok(self.ducks_streaked.clone().into()),
            "duck_streak_total" => Ok(self.duck_streak_total.clone().into()),
            "duck_streak_assist" => Ok(self.duck_streak_assist.clone().into()),
            "duck_streak_victim" => Ok(self.duck_streak_victim.clone().into()),
            "rocket_jump" => Ok(self.rocket_jump.clone().into()),
            "weapon_def_index" => Ok(self.weapon_def_index.clone().into()),
            "crit_type" => Ok(self.crit_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDeath",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(PlayerHurtEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            health: read_value::<u16>(stream, definition.get_entry("health"), "health")?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            damage_amount: read_value::<u16>(
                stream,
                definition.get_entry("damageamount"),
                "damage_amount",
            )?,
            custom: read_value::<u16>(stream, definition.get_entry("custom"), "custom")?,
            show_disguised_crit: read_value::<bool>(
                stream,
                definition.get_entry("showdisguisedcrit"),
                "show_disguised_crit",
            )?,
            crit: read_value::<bool>(stream, definition.get_entry("crit"), "crit")?,
            mini_crit: read_value::<bool>(stream, definition.get_entry("minicrit"), "mini_crit")?,
            all_see_crit: read_value::<bool>(
                stream,
                definition.get_entry("allseecrit"),
                "all_see_crit",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            bonus_effect: read_value::<u8>(
                stream,
                definition.get_entry("bonuseffect"),
                "bonus_effect",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "health" => Ok(self.health.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "damageamount" => Ok(self.damage_amount.clone().into()),
            "custom" => Ok(self.custom.clone().into()),
            "showdisguisedcrit" => Ok(self.show_disguised_crit.clone().into()),
            "crit" => Ok(self.crit.clone().into()),
            "minicrit" => Ok(self.mini_crit.clone().into()),
            "allseecrit" => Ok(self.all_see_crit.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "bonuseffect" => Ok(self.bonus_effect.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHurt",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChatEvent {
    pub team_only: bool,
    pub user_id: u16,
    pub text: MaybeUtf8String,
}
impl PlayerChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChatEvent {
            team_only: read_value::<bool>(stream, definition.get_entry("teamonly"), "team_only")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "teamonly" => Ok(self.team_only.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChat",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerScoreEvent {
    pub user_id: u16,
    pub kills: u16,
    pub deaths: u16,
    pub score: u16,
}
impl PlayerScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerScoreEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            kills: read_value::<u16>(stream, definition.get_entry("kills"), "kills")?,
            deaths: read_value::<u16>(stream, definition.get_entry("deaths"), "deaths")?,
            score: read_value::<u16>(stream, definition.get_entry("score"), "score")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "kills" => Ok(self.kills.clone().into()),
            "deaths" => Ok(self.deaths.clone().into()),
            "score" => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerScore",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSpawnEvent {
    pub user_id: u16,
    pub team: u16,
    pub class: u16,
}
impl PlayerSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSpawnEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            team: read_value::<u16>(stream, definition.get_entry("team"), "team")?,
            class: read_value::<u16>(stream, definition.get_entry("class"), "class")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "class" => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSpawn",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShootEvent {
    pub user_id: u16,
    pub weapon: u8,
    pub mode: u8,
}
impl PlayerShootEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerShootEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            weapon: read_value::<u8>(stream, definition.get_entry("weapon"), "weapon")?,
            mode: read_value::<u8>(stream, definition.get_entry("mode"), "mode")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "mode" => Ok(self.mode.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerShoot",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUseEvent {
    pub user_id: u16,
    pub entity: u16,
}
impl PlayerUseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUseEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            entity: read_value::<u16>(stream, definition.get_entry("entity"), "entity")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "entity" => Ok(self.entity.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUse",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeNameEvent {
    pub user_id: u16,
    pub old_name: MaybeUtf8String,
    pub new_name: MaybeUtf8String,
}
impl PlayerChangeNameEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChangeNameEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            old_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("oldname"),
                "old_name",
            )?,
            new_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("newname"),
                "new_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "oldname" => Ok(self.old_name.clone().into()),
            "newname" => Ok(self.new_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChangeName",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHintMessageEvent {
    pub hint_message: MaybeUtf8String,
}
impl PlayerHintMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHintMessageEvent {
            hint_message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("hintmessage"),
                "hint_message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "hintmessage" => Ok(self.hint_message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHintMessage",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BasePlayerTeleportedEvent {
    pub ent_index: u16,
}
impl BasePlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BasePlayerTeleportedEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BasePlayerTeleported",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameInitEvent {}
impl GameInitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameInitEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameInit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameNewMapEvent {
    pub map_name: MaybeUtf8String,
}
impl GameNewMapEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameNewMapEvent {
            map_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("mapname"),
                "map_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "mapname" => Ok(self.map_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameNewMap",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameStartEvent {
    pub rounds_limit: u32,
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl GameStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameStartEvent {
            rounds_limit: read_value::<u32>(
                stream,
                definition.get_entry("roundslimit"),
                "rounds_limit",
            )?,
            time_limit: read_value::<u32>(stream, definition.get_entry("timelimit"), "time_limit")?,
            frag_limit: read_value::<u32>(stream, definition.get_entry("fraglimit"), "frag_limit")?,
            objective: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("objective"),
                "objective",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "roundslimit" => Ok(self.rounds_limit.clone().into()),
            "timelimit" => Ok(self.time_limit.clone().into()),
            "fraglimit" => Ok(self.frag_limit.clone().into()),
            "objective" => Ok(self.objective.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameStart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameEndEvent {
    pub winner: u8,
}
impl GameEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameEndEvent {
            winner: read_value::<u8>(stream, definition.get_entry("winner"), "winner")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "winner" => Ok(self.winner.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameEnd",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundStartEvent {
    pub time_limit: u32,
    pub frag_limit: u32,
    pub objective: MaybeUtf8String,
}
impl RoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RoundStartEvent {
            time_limit: read_value::<u32>(stream, definition.get_entry("timelimit"), "time_limit")?,
            frag_limit: read_value::<u32>(stream, definition.get_entry("fraglimit"), "frag_limit")?,
            objective: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("objective"),
                "objective",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "timelimit" => Ok(self.time_limit.clone().into()),
            "fraglimit" => Ok(self.frag_limit.clone().into()),
            "objective" => Ok(self.objective.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RoundStart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RoundEndEvent {
    pub winner: u8,
    pub reason: u8,
    pub message: MaybeUtf8String,
}
impl RoundEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RoundEndEvent {
            winner: read_value::<u8>(stream, definition.get_entry("winner"), "winner")?,
            reason: read_value::<u8>(stream, definition.get_entry("reason"), "reason")?,
            message: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("message"),
                "message",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "winner" => Ok(self.winner.clone().into()),
            "reason" => Ok(self.reason.clone().into()),
            "message" => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RoundEnd",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameMessageEvent {
    pub target: u8,
    pub text: MaybeUtf8String,
}
impl GameMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameMessageEvent {
            target: read_value::<u8>(stream, definition.get_entry("target"), "target")?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "target" => Ok(self.target.clone().into()),
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameMessage",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakBreakableEvent {
    pub ent_index: u32,
    pub user_id: u16,
    pub material: u8,
}
impl BreakBreakableEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BreakBreakableEvent {
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            material: read_value::<u8>(stream, definition.get_entry("material"), "material")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            "material" => Ok(self.material.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BreakBreakable",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BreakPropEvent {
    pub ent_index: u32,
    pub user_id: u16,
}
impl BreakPropEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BreakPropEvent {
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BreakProp",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EntityKilledEvent {
    pub ent_index_killed: u32,
    pub ent_index_attacker: u32,
    pub ent_index_inflictor: u32,
    pub damage_bits: u32,
}
impl EntityKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EntityKilledEvent {
            ent_index_killed: read_value::<u32>(
                stream,
                definition.get_entry("entindex_killed"),
                "ent_index_killed",
            )?,
            ent_index_attacker: read_value::<u32>(
                stream,
                definition.get_entry("entindex_attacker"),
                "ent_index_attacker",
            )?,
            ent_index_inflictor: read_value::<u32>(
                stream,
                definition.get_entry("entindex_inflictor"),
                "ent_index_inflictor",
            )?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex_killed" => Ok(self.ent_index_killed.clone().into()),
            "entindex_attacker" => Ok(self.ent_index_attacker.clone().into()),
            "entindex_inflictor" => Ok(self.ent_index_inflictor.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EntityKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BonusUpdatedEvent {
    pub num_advanced: u16,
    pub num_bronze: u16,
    pub num_silver: u16,
    pub num_gold: u16,
}
impl BonusUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BonusUpdatedEvent {
            num_advanced: read_value::<u16>(
                stream,
                definition.get_entry("numadvanced"),
                "num_advanced",
            )?,
            num_bronze: read_value::<u16>(stream, definition.get_entry("numbronze"), "num_bronze")?,
            num_silver: read_value::<u16>(stream, definition.get_entry("numsilver"), "num_silver")?,
            num_gold: read_value::<u16>(stream, definition.get_entry("numgold"), "num_gold")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "numadvanced" => Ok(self.num_advanced.clone().into()),
            "numbronze" => Ok(self.num_bronze.clone().into()),
            "numsilver" => Ok(self.num_silver.clone().into()),
            "numgold" => Ok(self.num_gold.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BonusUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEventEvent {
    pub achievement_name: MaybeUtf8String,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementEventEvent {
            achievement_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("achievement_name"),
                "achievement_name",
            )?,
            cur_val: read_value::<u16>(stream, definition.get_entry("cur_val"), "cur_val")?,
            max_val: read_value::<u16>(stream, definition.get_entry("max_val"), "max_val")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "achievement_name" => Ok(self.achievement_name.clone().into()),
            "cur_val" => Ok(self.cur_val.clone().into()),
            "max_val" => Ok(self.max_val.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementEvent",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementIncrementEvent {
    pub achievement_id: u32,
    pub cur_val: u16,
    pub max_val: u16,
}
impl AchievementIncrementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementIncrementEvent {
            achievement_id: read_value::<u32>(
                stream,
                definition.get_entry("achievement_id"),
                "achievement_id",
            )?,
            cur_val: read_value::<u16>(stream, definition.get_entry("cur_val"), "cur_val")?,
            max_val: read_value::<u16>(stream, definition.get_entry("max_val"), "max_val")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "achievement_id" => Ok(self.achievement_id.clone().into()),
            "cur_val" => Ok(self.cur_val.clone().into()),
            "max_val" => Ok(self.max_val.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementIncrement",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PhysgunPickupEvent {
    pub ent_index: u32,
}
impl PhysgunPickupEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PhysgunPickupEvent {
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PhysgunPickup",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlareIgniteNpcEvent {
    pub ent_index: u32,
}
impl FlareIgniteNpcEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlareIgniteNpcEvent {
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FlareIgniteNpc",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HelicopterGrenadePuntMissEvent {}
impl HelicopterGrenadePuntMissEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HelicopterGrenadePuntMissEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HelicopterGrenadePuntMiss",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserDataDownloadedEvent {}
impl UserDataDownloadedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(UserDataDownloadedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "UserDataDownloaded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RagdollDissolvedEvent {
    pub ent_index: u32,
}
impl RagdollDissolvedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RagdollDissolvedEvent {
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RagdollDissolved",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedModeEvent {
    pub old_mode: u16,
    pub new_mode: u16,
    pub obs_target: u16,
}
impl HLTVChangedModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVChangedModeEvent {
            old_mode: read_value::<u16>(stream, definition.get_entry("oldmode"), "old_mode")?,
            new_mode: read_value::<u16>(stream, definition.get_entry("newmode"), "new_mode")?,
            obs_target: read_value::<u16>(
                stream,
                definition.get_entry("obs_target"),
                "obs_target",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "oldmode" => Ok(self.old_mode.clone().into()),
            "newmode" => Ok(self.new_mode.clone().into()),
            "obs_target" => Ok(self.obs_target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChangedMode",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChangedTargetEvent {
    pub mode: u16,
    pub old_target: u16,
    pub obs_target: u16,
}
impl HLTVChangedTargetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVChangedTargetEvent {
            mode: read_value::<u16>(stream, definition.get_entry("mode"), "mode")?,
            old_target: read_value::<u16>(
                stream,
                definition.get_entry("old_target"),
                "old_target",
            )?,
            obs_target: read_value::<u16>(
                stream,
                definition.get_entry("obs_target"),
                "obs_target",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "mode" => Ok(self.mode.clone().into()),
            "old_target" => Ok(self.old_target.clone().into()),
            "obs_target" => Ok(self.obs_target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChangedTarget",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteEndedEvent {}
impl VoteEndedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteEndedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteEnded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteStartedEvent {
    pub issue: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
    pub initiator: u32,
    pub voteidx: u32,
}
impl VoteStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteStartedEvent {
            issue: read_value::<MaybeUtf8String>(stream, definition.get_entry("issue"), "issue")?,
            param_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("param1"),
                "param_1",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            initiator: read_value::<u32>(stream, definition.get_entry("initiator"), "initiator")?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "issue" => Ok(self.issue.clone().into()),
            "param1" => Ok(self.param_1.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "initiator" => Ok(self.initiator.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteStarted",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteChangedEvent {
    pub vote_option_1: u8,
    pub vote_option_2: u8,
    pub vote_option_3: u8,
    pub vote_option_4: u8,
    pub vote_option_5: u8,
    pub potential_votes: u8,
    pub voteidx: u32,
}
impl VoteChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteChangedEvent {
            vote_option_1: read_value::<u8>(
                stream,
                definition.get_entry("vote_option1"),
                "vote_option_1",
            )?,
            vote_option_2: read_value::<u8>(
                stream,
                definition.get_entry("vote_option2"),
                "vote_option_2",
            )?,
            vote_option_3: read_value::<u8>(
                stream,
                definition.get_entry("vote_option3"),
                "vote_option_3",
            )?,
            vote_option_4: read_value::<u8>(
                stream,
                definition.get_entry("vote_option4"),
                "vote_option_4",
            )?,
            vote_option_5: read_value::<u8>(
                stream,
                definition.get_entry("vote_option5"),
                "vote_option_5",
            )?,
            potential_votes: read_value::<u8>(
                stream,
                definition.get_entry("potentialVotes"),
                "potential_votes",
            )?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "vote_option1" => Ok(self.vote_option_1.clone().into()),
            "vote_option2" => Ok(self.vote_option_2.clone().into()),
            "vote_option3" => Ok(self.vote_option_3.clone().into()),
            "vote_option4" => Ok(self.vote_option_4.clone().into()),
            "vote_option5" => Ok(self.vote_option_5.clone().into()),
            "potentialVotes" => Ok(self.potential_votes.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VotePassedEvent {
    pub details: MaybeUtf8String,
    pub param_1: MaybeUtf8String,
    pub team: u8,
    pub voteidx: u32,
}
impl VotePassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VotePassedEvent {
            details: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("details"),
                "details",
            )?,
            param_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("param1"),
                "param_1",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "details" => Ok(self.details.clone().into()),
            "param1" => Ok(self.param_1.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VotePassed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteFailedEvent {
    pub team: u8,
    pub voteidx: u32,
}
impl VoteFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteFailedEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteFailed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteCastEvent {
    pub vote_option: u8,
    pub team: u16,
    pub entity_id: u32,
    pub voteidx: u32,
}
impl VoteCastEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteCastEvent {
            vote_option: read_value::<u8>(
                stream,
                definition.get_entry("vote_option"),
                "vote_option",
            )?,
            team: read_value::<u16>(stream, definition.get_entry("team"), "team")?,
            entity_id: read_value::<u32>(stream, definition.get_entry("entityid"), "entity_id")?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "vote_option" => Ok(self.vote_option.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "entityid" => Ok(self.entity_id.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteCast",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteOptionsEvent {
    pub count: u8,
    pub option_1: MaybeUtf8String,
    pub option_2: MaybeUtf8String,
    pub option_3: MaybeUtf8String,
    pub option_4: MaybeUtf8String,
    pub option_5: MaybeUtf8String,
    pub voteidx: u32,
}
impl VoteOptionsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteOptionsEvent {
            count: read_value::<u8>(stream, definition.get_entry("count"), "count")?,
            option_1: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("option1"),
                "option_1",
            )?,
            option_2: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("option2"),
                "option_2",
            )?,
            option_3: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("option3"),
                "option_3",
            )?,
            option_4: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("option4"),
                "option_4",
            )?,
            option_5: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("option5"),
                "option_5",
            )?,
            voteidx: read_value::<u32>(stream, definition.get_entry("voteidx"), "voteidx")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "count" => Ok(self.count.clone().into()),
            "option1" => Ok(self.option_1.clone().into()),
            "option2" => Ok(self.option_2.clone().into()),
            "option3" => Ok(self.option_3.clone().into()),
            "option4" => Ok(self.option_4.clone().into()),
            "option5" => Ok(self.option_5.clone().into()),
            "voteidx" => Ok(self.voteidx.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteOptions",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplaySavedEvent {}
impl ReplaySavedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplaySavedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplaySaved",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnteredPerformanceModeEvent {}
impl EnteredPerformanceModeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EnteredPerformanceModeEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EnteredPerformanceMode",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BrowseReplaysEvent {}
impl BrowseReplaysEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BrowseReplaysEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BrowseReplays",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayYoutubeStatsEvent {
    pub views: u32,
    pub likes: u32,
    pub favorited: u32,
}
impl ReplayYoutubeStatsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayYoutubeStatsEvent {
            views: read_value::<u32>(stream, definition.get_entry("views"), "views")?,
            likes: read_value::<u32>(stream, definition.get_entry("likes"), "likes")?,
            favorited: read_value::<u32>(stream, definition.get_entry("favorited"), "favorited")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "views" => Ok(self.views.clone().into()),
            "likes" => Ok(self.likes.clone().into()),
            "favorited" => Ok(self.favorited.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayYoutubeStats",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct InventoryUpdatedEvent {}
impl InventoryUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(InventoryUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "InventoryUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CartUpdatedEvent {}
impl CartUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CartUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CartUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StorePriceSheetUpdatedEvent {}
impl StorePriceSheetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StorePriceSheetUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StorePriceSheetUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EconInventoryConnectedEvent {}
impl EconInventoryConnectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EconInventoryConnectedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EconInventoryConnected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemSchemaInitializedEvent {}
impl ItemSchemaInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemSchemaInitializedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemSchemaInitialized",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcNewSessionEvent {}
impl GcNewSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcNewSessionEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GcNewSession",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GcLostSessionEvent {}
impl GcLostSessionEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GcLostSessionEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GcLostSession",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroFinishEvent {
    pub player: u16,
}
impl IntroFinishEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(IntroFinishEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "IntroFinish",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct IntroNextCameraEvent {
    pub player: u16,
}
impl IntroNextCameraEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(IntroNextCameraEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "IntroNextCamera",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChangeClassEvent {
    pub user_id: u16,
    pub class: u16,
}
impl PlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChangeClassEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            class: read_value::<u16>(stream, definition.get_entry("class"), "class")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "class" => Ok(self.class.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChangeClass",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfMapTimeRemainingEvent {
    pub seconds: u32,
}
impl TfMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TfMapTimeRemainingEvent {
            seconds: read_value::<u32>(stream, definition.get_entry("seconds"), "seconds")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "seconds" => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TfMapTimeRemaining",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TfGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TfGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TfGameOverEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("reason"),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "reason" => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TfGameOver",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CtfFlagCapturedEvent {
    pub capping_team: u16,
    pub capping_team_score: u16,
}
impl CtfFlagCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CtfFlagCapturedEvent {
            capping_team: read_value::<u16>(
                stream,
                definition.get_entry("capping_team"),
                "capping_team",
            )?,
            capping_team_score: read_value::<u16>(
                stream,
                definition.get_entry("capping_team_score"),
                "capping_team_score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "capping_team" => Ok(self.capping_team.clone().into()),
            "capping_team_score" => Ok(self.capping_team_score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CtfFlagCaptured",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointInitializedEvent {}
impl ControlPointInitializedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointInitializedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointInitialized",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateImagesEvent {
    pub index: u16,
}
impl ControlPointUpdateImagesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateImagesEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateImages",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateLayoutEvent {
    pub index: u16,
}
impl ControlPointUpdateLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateLayoutEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateLayout",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateCappingEvent {
    pub index: u16,
}
impl ControlPointUpdateCappingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateCappingEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateCapping",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUpdateOwnerEvent {
    pub index: u16,
}
impl ControlPointUpdateOwnerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUpdateOwnerEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUpdateOwner",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointStartTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointStartTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointStartTouchEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            area: read_value::<u16>(stream, definition.get_entry("area"), "area")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "area" => Ok(self.area.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointStartTouch",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointEndTouchEvent {
    pub player: u16,
    pub area: u16,
}
impl ControlPointEndTouchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointEndTouchEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            area: read_value::<u16>(stream, definition.get_entry("area"), "area")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "area" => Ok(self.area.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointEndTouch",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointPulseElementEvent {
    pub player: u16,
}
impl ControlPointPulseElementEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointPulseElementEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointPulseElement",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointFakeCaptureEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            int_data: read_value::<u16>(stream, definition.get_entry("int_data"), "int_data")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "int_data" => Ok(self.int_data.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointFakeCapture",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointFakeCaptureMultiplierEvent {
    pub player: u16,
    pub int_data: u16,
}
impl ControlPointFakeCaptureMultiplierEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointFakeCaptureMultiplierEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            int_data: read_value::<u16>(stream, definition.get_entry("int_data"), "int_data")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "int_data" => Ok(self.int_data.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointFakeCaptureMultiplier",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundSelectedEvent {
    pub round: MaybeUtf8String,
}
impl TeamPlayRoundSelectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundSelectedEvent {
            round: read_value::<MaybeUtf8String>(stream, definition.get_entry("round"), "round")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "round" => Ok(self.round.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundSelected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStartEvent {
    pub full_reset: bool,
}
impl TeamPlayRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundStartEvent {
            full_reset: read_value::<bool>(
                stream,
                definition.get_entry("full_reset"),
                "full_reset",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "full_reset" => Ok(self.full_reset.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundStart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundActiveEvent {}
impl TeamPlayRoundActiveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundActiveEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundActive",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingBeginsEvent {}
impl TeamPlayWaitingBeginsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingBeginsEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingBegins",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingEndsEvent {}
impl TeamPlayWaitingEndsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingEndsEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingEnds",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayWaitingAboutToEndEvent {}
impl TeamPlayWaitingAboutToEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayWaitingAboutToEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWaitingAboutToEnd",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRestartRoundEvent {}
impl TeamPlayRestartRoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRestartRoundEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRestartRound",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayReadyRestartEvent {}
impl TeamPlayReadyRestartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayReadyRestartEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayReadyRestart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundRestartSecondsEvent {
    pub seconds: u16,
}
impl TeamPlayRoundRestartSecondsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundRestartSecondsEvent {
            seconds: read_value::<u16>(stream, definition.get_entry("seconds"), "seconds")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "seconds" => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundRestartSeconds",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamReadyEvent {
    pub team: u8,
}
impl TeamPlayTeamReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTeamReadyEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTeamReady",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(TeamPlayRoundWinEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            win_reason: read_value::<u8>(stream, definition.get_entry("winreason"), "win_reason")?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry("flagcaplimit"),
                "flag_cap_limit",
            )?,
            full_round: read_value::<u16>(
                stream,
                definition.get_entry("full_round"),
                "full_round",
            )?,
            round_time: read_value::<f32>(
                stream,
                definition.get_entry("round_time"),
                "round_time",
            )?,
            losing_team_num_caps: read_value::<u16>(
                stream,
                definition.get_entry("losing_team_num_caps"),
                "losing_team_num_caps",
            )?,
            was_sudden_death: read_value::<u8>(
                stream,
                definition.get_entry("was_sudden_death"),
                "was_sudden_death",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "winreason" => Ok(self.win_reason.clone().into()),
            "flagcaplimit" => Ok(self.flag_cap_limit.clone().into()),
            "full_round" => Ok(self.full_round.clone().into()),
            "round_time" => Ok(self.round_time.clone().into()),
            "losing_team_num_caps" => Ok(self.losing_team_num_caps.clone().into()),
            "was_sudden_death" => Ok(self.was_sudden_death.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundWin",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayUpdateTimerEvent {}
impl TeamPlayUpdateTimerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayUpdateTimerEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayUpdateTimer",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayRoundStalemateEvent {
    pub reason: u8,
}
impl TeamPlayRoundStalemateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayRoundStalemateEvent {
            reason: read_value::<u8>(stream, definition.get_entry("reason"), "reason")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "reason" => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayRoundStalemate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeBeginEvent {}
impl TeamPlayOvertimeBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeBeginEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayOvertimeBegin",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayOvertimeEndEvent {}
impl TeamPlayOvertimeEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayOvertimeEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayOvertimeEnd",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathBeginEvent {}
impl TeamPlaySuddenDeathBeginEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathBeginEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySuddenDeathBegin",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySuddenDeathEndEvent {}
impl TeamPlaySuddenDeathEndEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySuddenDeathEndEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySuddenDeathEnd",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayGameOverEvent {
    pub reason: MaybeUtf8String,
}
impl TeamPlayGameOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayGameOverEvent {
            reason: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("reason"),
                "reason",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "reason" => Ok(self.reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayGameOver",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayMapTimeRemainingEvent {
    pub seconds: u16,
}
impl TeamPlayMapTimeRemainingEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayMapTimeRemainingEvent {
            seconds: read_value::<u16>(stream, definition.get_entry("seconds"), "seconds")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "seconds" => Ok(self.seconds.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayMapTimeRemaining",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerFlashEvent {
    pub time_remaining: u16,
}
impl TeamPlayTimerFlashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTimerFlashEvent {
            time_remaining: read_value::<u16>(
                stream,
                definition.get_entry("time_remaining"),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "time_remaining" => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTimerFlash",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTimerTimeAddedEvent {
    pub timer: u16,
    pub seconds_added: u16,
}
impl TeamPlayTimerTimeAddedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTimerTimeAddedEvent {
            timer: read_value::<u16>(stream, definition.get_entry("timer"), "timer")?,
            seconds_added: read_value::<u16>(
                stream,
                definition.get_entry("seconds_added"),
                "seconds_added",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "timer" => Ok(self.timer.clone().into()),
            "seconds_added" => Ok(self.seconds_added.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTimerTimeAdded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(TeamPlayPointStartCaptureEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            cap_team: read_value::<u8>(stream, definition.get_entry("capteam"), "cap_team")?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cappers"),
                "cappers",
            )?,
            cap_time: read_value::<f32>(stream, definition.get_entry("captime"), "cap_time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "capteam" => Ok(self.cap_team.clone().into()),
            "cappers" => Ok(self.cappers.clone().into()),
            "captime" => Ok(self.cap_time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointStartCapture",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointCapturedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
    pub cappers: MaybeUtf8String,
}
impl TeamPlayPointCapturedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointCapturedEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cappers"),
                "cappers",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "cappers" => Ok(self.cappers.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointCaptured",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointLockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointLockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointLockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointLocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPointUnlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub team: u8,
}
impl TeamPlayPointUnlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPointUnlockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPointUnlocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBrokenEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub time_remaining: f32,
}
impl TeamPlayCaptureBrokenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayCaptureBrokenEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            time_remaining: read_value::<f32>(
                stream,
                definition.get_entry("time_remaining"),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "time_remaining" => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayCaptureBroken",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayCaptureBlockedEvent {
    pub cp: u8,
    pub cp_name: MaybeUtf8String,
    pub blocker: u8,
    pub victim: u8,
}
impl TeamPlayCaptureBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayCaptureBlockedEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            cp_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cpname"),
                "cp_name",
            )?,
            blocker: read_value::<u8>(stream, definition.get_entry("blocker"), "blocker")?,
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "cpname" => Ok(self.cp_name.clone().into()),
            "blocker" => Ok(self.blocker.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayCaptureBlocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(TeamPlayFlagEventEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            carrier: read_value::<u16>(stream, definition.get_entry("carrier"), "carrier")?,
            event_type: read_value::<u16>(stream, definition.get_entry("eventtype"), "event_type")?,
            home: read_value::<u8>(stream, definition.get_entry("home"), "home")?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "carrier" => Ok(self.carrier.clone().into()),
            "eventtype" => Ok(self.event_type.clone().into()),
            "home" => Ok(self.home.clone().into()),
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayFlagEvent",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(TeamPlayWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry("panel_style"),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry("winning_team"),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(stream, definition.get_entry("winreason"), "win_reason")?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cappers"),
                "cappers",
            )?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry("flagcaplimit"),
                "flag_cap_limit",
            )?,
            blue_score: read_value::<u16>(
                stream,
                definition.get_entry("blue_score"),
                "blue_score",
            )?,
            red_score: read_value::<u16>(stream, definition.get_entry("red_score"), "red_score")?,
            blue_score_prev: read_value::<u16>(
                stream,
                definition.get_entry("blue_score_prev"),
                "blue_score_prev",
            )?,
            red_score_prev: read_value::<u16>(
                stream,
                definition.get_entry("red_score_prev"),
                "red_score_prev",
            )?,
            round_complete: read_value::<u16>(
                stream,
                definition.get_entry("round_complete"),
                "round_complete",
            )?,
            rounds_remaining: read_value::<u16>(
                stream,
                definition.get_entry("rounds_remaining"),
                "rounds_remaining",
            )?,
            player_1: read_value::<u16>(stream, definition.get_entry("player_1"), "player_1")?,
            player_1_points: read_value::<u16>(
                stream,
                definition.get_entry("player_1_points"),
                "player_1_points",
            )?,
            player_2: read_value::<u16>(stream, definition.get_entry("player_2"), "player_2")?,
            player_2_points: read_value::<u16>(
                stream,
                definition.get_entry("player_2_points"),
                "player_2_points",
            )?,
            player_3: read_value::<u16>(stream, definition.get_entry("player_3"), "player_3")?,
            player_3_points: read_value::<u16>(
                stream,
                definition.get_entry("player_3_points"),
                "player_3_points",
            )?,
            kill_stream_player_1: read_value::<u16>(
                stream,
                definition.get_entry("killstreak_player_1"),
                "kill_stream_player_1",
            )?,
            kill_stream_player_1_count: read_value::<u16>(
                stream,
                definition.get_entry("killstreak_player_1_count"),
                "kill_stream_player_1_count",
            )?,
            game_over: read_value::<u8>(stream, definition.get_entry("game_over"), "game_over")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "panel_style" => Ok(self.panel_style.clone().into()),
            "winning_team" => Ok(self.winning_team.clone().into()),
            "winreason" => Ok(self.win_reason.clone().into()),
            "cappers" => Ok(self.cappers.clone().into()),
            "flagcaplimit" => Ok(self.flag_cap_limit.clone().into()),
            "blue_score" => Ok(self.blue_score.clone().into()),
            "red_score" => Ok(self.red_score.clone().into()),
            "blue_score_prev" => Ok(self.blue_score_prev.clone().into()),
            "red_score_prev" => Ok(self.red_score_prev.clone().into()),
            "round_complete" => Ok(self.round_complete.clone().into()),
            "rounds_remaining" => Ok(self.rounds_remaining.clone().into()),
            "player_1" => Ok(self.player_1.clone().into()),
            "player_1_points" => Ok(self.player_1_points.clone().into()),
            "player_2" => Ok(self.player_2.clone().into()),
            "player_2_points" => Ok(self.player_2_points.clone().into()),
            "player_3" => Ok(self.player_3.clone().into()),
            "player_3_points" => Ok(self.player_3_points.clone().into()),
            "killstreak_player_1" => Ok(self.kill_stream_player_1.clone().into()),
            "killstreak_player_1_count" => Ok(self.kill_stream_player_1_count.clone().into()),
            "game_over" => Ok(self.game_over.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayWinPanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayTeamBalancedPlayerEvent {
    pub player: u16,
    pub team: u8,
}
impl TeamPlayTeamBalancedPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayTeamBalancedPlayerEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayTeamBalancedPlayer",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlaySetupFinishedEvent {}
impl TeamPlaySetupFinishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlaySetupFinishedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlaySetupFinished",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayAlertEvent {
    pub alert_type: u16,
}
impl TeamPlayAlertEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayAlertEvent {
            alert_type: read_value::<u16>(
                stream,
                definition.get_entry("alert_type"),
                "alert_type",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "alert_type" => Ok(self.alert_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayAlert",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TrainingCompleteEvent {
    pub next_map: MaybeUtf8String,
    pub map: MaybeUtf8String,
    pub text: MaybeUtf8String,
}
impl TrainingCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TrainingCompleteEvent {
            next_map: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("next_map"),
                "next_map",
            )?,
            map: read_value::<MaybeUtf8String>(stream, definition.get_entry("map"), "map")?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "next_map" => Ok(self.next_map.clone().into()),
            "map" => Ok(self.map.clone().into()),
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TrainingComplete",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowFreezePanelEvent {
    pub killer: u16,
}
impl ShowFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowFreezePanelEvent {
            killer: read_value::<u16>(stream, definition.get_entry("killer"), "killer")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "killer" => Ok(self.killer.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowFreezePanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideFreezePanelEvent {}
impl HideFreezePanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HideFreezePanelEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HideFreezePanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FreezeCamStartedEvent {}
impl FreezeCamStartedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FreezeCamStartedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FreezeCamStarted",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeTeamEvent {}
impl LocalPlayerChangeTeamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeTeamEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeTeam",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerScoreChangedEvent {
    pub score: u16,
}
impl LocalPlayerScoreChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerScoreChangedEvent {
            score: read_value::<u16>(stream, definition.get_entry("score"), "score")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "score" => Ok(self.score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerScoreChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeClassEvent {}
impl LocalPlayerChangeClassEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeClassEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeClass",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerRespawnEvent {}
impl LocalPlayerRespawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerRespawnEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerRespawn",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildingInfoChangedEvent {
    pub building_type: u8,
    pub object_mode: u8,
    pub remove: u8,
}
impl BuildingInfoChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BuildingInfoChangedEvent {
            building_type: read_value::<u8>(
                stream,
                definition.get_entry("building_type"),
                "building_type",
            )?,
            object_mode: read_value::<u8>(
                stream,
                definition.get_entry("object_mode"),
                "object_mode",
            )?,
            remove: read_value::<u8>(stream, definition.get_entry("remove"), "remove")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "building_type" => Ok(self.building_type.clone().into()),
            "object_mode" => Ok(self.object_mode.clone().into()),
            "remove" => Ok(self.remove.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BuildingInfoChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChangeDisguiseEvent {
    pub disguised: bool,
}
impl LocalPlayerChangeDisguiseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChangeDisguiseEvent {
            disguised: read_value::<bool>(stream, definition.get_entry("disguised"), "disguised")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "disguised" => Ok(self.disguised.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChangeDisguise",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAccountChangedEvent {
    pub old_value: u16,
    pub new_value: u16,
}
impl PlayerAccountChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerAccountChangedEvent {
            old_value: read_value::<u16>(stream, definition.get_entry("old_value"), "old_value")?,
            new_value: read_value::<u16>(stream, definition.get_entry("new_value"), "new_value")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "old_value" => Ok(self.old_value.clone().into()),
            "new_value" => Ok(self.new_value.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerAccountChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpyPdaResetEvent {}
impl SpyPdaResetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpyPdaResetEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SpyPdaReset",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlagStatusUpdateEvent {
    pub user_id: u16,
    pub ent_index: u32,
}
impl FlagStatusUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlagStatusUpdateEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            ent_index: read_value::<u32>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FlagStatusUpdate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStatsUpdatedEvent {
    pub force_upload: bool,
}
impl PlayerStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStatsUpdatedEvent {
            force_upload: read_value::<bool>(
                stream,
                definition.get_entry("forceupload"),
                "force_upload",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "forceupload" => Ok(self.force_upload.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStatsUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayingCommentaryEvent {}
impl PlayingCommentaryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayingCommentaryEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayingCommentary",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerChargeDeployedEvent {
    pub user_id: u16,
    pub target_id: u16,
}
impl PlayerChargeDeployedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerChargeDeployedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            target_id: read_value::<u16>(stream, definition.get_entry("targetid"), "target_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "targetid" => Ok(self.target_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerChargeDeployed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuiltObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerBuiltObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBuiltObjectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object: read_value::<u16>(stream, definition.get_entry("object"), "object")?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "object" => Ok(self.object.clone().into()),
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBuiltObject",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUpgradedObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
    pub is_builder: bool,
}
impl PlayerUpgradedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUpgradedObjectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object: read_value::<u16>(stream, definition.get_entry("object"), "object")?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            is_builder: read_value::<bool>(
                stream,
                definition.get_entry("isbuilder"),
                "is_builder",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "object" => Ok(self.object.clone().into()),
            "index" => Ok(self.index.clone().into()),
            "isbuilder" => Ok(self.is_builder.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUpgradedObject",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCarryObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerCarryObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerCarryObjectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object: read_value::<u16>(stream, definition.get_entry("object"), "object")?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "object" => Ok(self.object.clone().into()),
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerCarryObject",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDropObjectEvent {
    pub user_id: u16,
    pub object: u16,
    pub index: u16,
}
impl PlayerDropObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDropObjectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object: read_value::<u16>(stream, definition.get_entry("object"), "object")?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "object" => Ok(self.object.clone().into()),
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDropObject",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectRemovedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectRemovedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectRemovedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry("objecttype"),
                "object_type",
            )?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "objecttype" => Ok(self.object_type.clone().into()),
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectRemoved",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ObjectDestroyedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry("objecttype"),
                "object_type",
            )?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            was_building: read_value::<bool>(
                stream,
                definition.get_entry("was_building"),
                "was_building",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "objecttype" => Ok(self.object_type.clone().into()),
            "index" => Ok(self.index.clone().into()),
            "was_building" => Ok(self.was_building.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDestroyed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDetonatedEvent {
    pub user_id: u16,
    pub object_type: u16,
    pub index: u16,
}
impl ObjectDetonatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectDetonatedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            object_type: read_value::<u16>(
                stream,
                definition.get_entry("objecttype"),
                "object_type",
            )?,
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "objecttype" => Ok(self.object_type.clone().into()),
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDetonated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEarnedEvent {
    pub player: u8,
    pub achievement: u16,
}
impl AchievementEarnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementEarnedEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            achievement: read_value::<u16>(
                stream,
                definition.get_entry("achievement"),
                "achievement",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "achievement" => Ok(self.achievement.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementEarned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecTargetUpdatedEvent {}
impl SpecTargetUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpecTargetUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SpecTargetUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentStateUpdateEvent {
    pub user_id: u16,
    pub name_change: bool,
    pub ready_state: u16,
    pub new_name: MaybeUtf8String,
}
impl TournamentStateUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TournamentStateUpdateEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            name_change: read_value::<bool>(
                stream,
                definition.get_entry("namechange"),
                "name_change",
            )?,
            ready_state: read_value::<u16>(
                stream,
                definition.get_entry("readystate"),
                "ready_state",
            )?,
            new_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("newname"),
                "new_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "namechange" => Ok(self.name_change.clone().into()),
            "readystate" => Ok(self.ready_state.clone().into()),
            "newname" => Ok(self.new_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TournamentStateUpdate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TournamentEnableCountdownEvent {}
impl TournamentEnableCountdownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TournamentEnableCountdownEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TournamentEnableCountdown",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCalledForMedicEvent {
    pub user_id: u16,
}
impl PlayerCalledForMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerCalledForMedicEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerCalledForMedic",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAskedForBallEvent {
    pub user_id: u16,
}
impl PlayerAskedForBallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerAskedForBallEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerAskedForBall",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerBecameObserverEvent {}
impl LocalPlayerBecameObserverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerBecameObserverEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerBecameObserver",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedInvEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub medic_ent_index: u8,
}
impl PlayerIgnitedInvEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerIgnitedInvEvent {
            pyro_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("pyro_entindex"),
                "pyro_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            medic_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("medic_entindex"),
                "medic_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "pyro_entindex" => Ok(self.pyro_ent_index.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "medic_entindex" => Ok(self.medic_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerIgnitedInv",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerIgnitedEvent {
    pub pyro_ent_index: u8,
    pub victim_ent_index: u8,
    pub weapon_id: u8,
}
impl PlayerIgnitedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerIgnitedEvent {
            pyro_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("pyro_entindex"),
                "pyro_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            weapon_id: read_value::<u8>(stream, definition.get_entry("weaponid"), "weapon_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "pyro_entindex" => Ok(self.pyro_ent_index.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerIgnited",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerExtinguishedEvent {
    pub victim: u8,
    pub healer: u8,
    pub item_definition_index: u16,
}
impl PlayerExtinguishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerExtinguishedEvent {
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
            healer: read_value::<u8>(stream, definition.get_entry("healer"), "healer")?,
            item_definition_index: read_value::<u16>(
                stream,
                definition.get_entry("itemdefindex"),
                "item_definition_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "victim" => Ok(self.victim.clone().into()),
            "healer" => Ok(self.healer.clone().into()),
            "itemdefindex" => Ok(self.item_definition_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerExtinguished",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTeleportedEvent {
    pub user_id: u16,
    pub builder_id: u16,
    pub dist: f32,
}
impl PlayerTeleportedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerTeleportedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            builder_id: read_value::<u16>(stream, definition.get_entry("builderid"), "builder_id")?,
            dist: read_value::<f32>(stream, definition.get_entry("dist"), "dist")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "builderid" => Ok(self.builder_id.clone().into()),
            "dist" => Ok(self.dist.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerTeleported",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedMedicCallEvent {
    pub user_id: u16,
}
impl PlayerHealedMedicCallEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealedMedicCallEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealedMedicCall",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerChargeReadyEvent {}
impl LocalPlayerChargeReadyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerChargeReadyEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerChargeReady",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerWindDownEvent {}
impl LocalPlayerWindDownEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerWindDownEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerWindDown",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInvulnedEvent {
    pub user_id: u16,
    pub medic_user_id: u16,
}
impl PlayerInvulnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerInvulnedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            medic_user_id: read_value::<u16>(
                stream,
                definition.get_entry("medic_userid"),
                "medic_user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "medic_userid" => Ok(self.medic_user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerInvulned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortSpeedEvent {
    pub team: u8,
    pub speed: u8,
    pub players: u8,
}
impl EscortSpeedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortSpeedEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            speed: read_value::<u8>(stream, definition.get_entry("speed"), "speed")?,
            players: read_value::<u8>(stream, definition.get_entry("players"), "players")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "speed" => Ok(self.speed.clone().into()),
            "players" => Ok(self.players.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortSpeed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortProgressEvent {
    pub team: u8,
    pub progress: f32,
    pub reset: bool,
}
impl EscortProgressEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortProgressEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            progress: read_value::<f32>(stream, definition.get_entry("progress"), "progress")?,
            reset: read_value::<bool>(stream, definition.get_entry("reset"), "reset")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "progress" => Ok(self.progress.clone().into()),
            "reset" => Ok(self.reset.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortProgress",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscortRecedeEvent {
    pub team: u8,
    pub recede_time: f32,
}
impl EscortRecedeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscortRecedeEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            recede_time: read_value::<f32>(
                stream,
                definition.get_entry("recedetime"),
                "recede_time",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "recedetime" => Ok(self.recede_time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscortRecede",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIActivatedEvent {}
impl GameUIActivatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIActivatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameUIActivated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GameUIHiddenEvent {}
impl GameUIHiddenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GameUIHiddenEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GameUIHidden",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerEscortScoreEvent {
    pub player: u8,
    pub points: u8,
}
impl PlayerEscortScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerEscortScoreEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            points: read_value::<u8>(stream, definition.get_entry("points"), "points")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "points" => Ok(self.points.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerEscortScore",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealOnHitEvent {
    pub amount: u16,
    pub ent_index: u8,
    pub weapon_def_index: u32,
}
impl PlayerHealOnHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealOnHitEvent {
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
            ent_index: read_value::<u8>(stream, definition.get_entry("entindex"), "ent_index")?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry("weapon_def_index"),
                "weapon_def_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "amount" => Ok(self.amount.clone().into()),
            "entindex" => Ok(self.ent_index.clone().into()),
            "weapon_def_index" => Ok(self.weapon_def_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealOnHit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStealSandvichEvent {
    pub owner: u16,
    pub target: u16,
}
impl PlayerStealSandvichEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStealSandvichEvent {
            owner: read_value::<u16>(stream, definition.get_entry("owner"), "owner")?,
            target: read_value::<u16>(stream, definition.get_entry("target"), "target")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "owner" => Ok(self.owner.clone().into()),
            "target" => Ok(self.target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStealSandvich",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowClassLayoutEvent {
    pub show: bool,
}
impl ShowClassLayoutEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowClassLayoutEvent {
            show: read_value::<bool>(stream, definition.get_entry("show"), "show")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "show" => Ok(self.show.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowClassLayout",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowVsPanelEvent {
    pub show: bool,
}
impl ShowVsPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowVsPanelEvent {
            show: read_value::<bool>(stream, definition.get_entry("show"), "show")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "show" => Ok(self.show.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowVsPanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamagedEvent {
    pub amount: u16,
    pub kind: u32,
}
impl PlayerDamagedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDamagedEvent {
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
            kind: read_value::<u32>(stream, definition.get_entry("type"), "kind")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "amount" => Ok(self.amount.clone().into()),
            "type" => Ok(self.kind.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDamaged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaPlayerNotificationEvent {
    pub player: u8,
    pub message: u8,
}
impl ArenaPlayerNotificationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaPlayerNotificationEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            message: read_value::<u8>(stream, definition.get_entry("message"), "message")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "message" => Ok(self.message.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaPlayerNotification",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaMatchMaxStreakEvent {
    pub team: u8,
    pub streak: u8,
}
impl ArenaMatchMaxStreakEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaMatchMaxStreakEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            streak: read_value::<u8>(stream, definition.get_entry("streak"), "streak")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "streak" => Ok(self.streak.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaMatchMaxStreak",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ArenaRoundStartEvent {}
impl ArenaRoundStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ArenaRoundStartEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaRoundStart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ArenaWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry("panel_style"),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry("winning_team"),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(stream, definition.get_entry("winreason"), "win_reason")?,
            cappers: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("cappers"),
                "cappers",
            )?,
            flag_cap_limit: read_value::<u16>(
                stream,
                definition.get_entry("flagcaplimit"),
                "flag_cap_limit",
            )?,
            blue_score: read_value::<u16>(
                stream,
                definition.get_entry("blue_score"),
                "blue_score",
            )?,
            red_score: read_value::<u16>(stream, definition.get_entry("red_score"), "red_score")?,
            blue_score_prev: read_value::<u16>(
                stream,
                definition.get_entry("blue_score_prev"),
                "blue_score_prev",
            )?,
            red_score_prev: read_value::<u16>(
                stream,
                definition.get_entry("red_score_prev"),
                "red_score_prev",
            )?,
            round_complete: read_value::<u16>(
                stream,
                definition.get_entry("round_complete"),
                "round_complete",
            )?,
            player_1: read_value::<u16>(stream, definition.get_entry("player_1"), "player_1")?,
            player_1_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_1_damage"),
                "player_1_damage",
            )?,
            player_1_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_1_healing"),
                "player_1_healing",
            )?,
            player_1_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_1_lifetime"),
                "player_1_lifetime",
            )?,
            player_1_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_1_kills"),
                "player_1_kills",
            )?,
            player_2: read_value::<u16>(stream, definition.get_entry("player_2"), "player_2")?,
            player_2_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_2_damage"),
                "player_2_damage",
            )?,
            player_2_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_2_healing"),
                "player_2_healing",
            )?,
            player_2_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_2_lifetime"),
                "player_2_lifetime",
            )?,
            player_2_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_2_kills"),
                "player_2_kills",
            )?,
            player_3: read_value::<u16>(stream, definition.get_entry("player_3"), "player_3")?,
            player_3_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_3_damage"),
                "player_3_damage",
            )?,
            player_3_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_3_healing"),
                "player_3_healing",
            )?,
            player_3_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_3_lifetime"),
                "player_3_lifetime",
            )?,
            player_3_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_3_kills"),
                "player_3_kills",
            )?,
            player_4: read_value::<u16>(stream, definition.get_entry("player_4"), "player_4")?,
            player_4_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_4_damage"),
                "player_4_damage",
            )?,
            player_4_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_4_healing"),
                "player_4_healing",
            )?,
            player_4_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_4_lifetime"),
                "player_4_lifetime",
            )?,
            player_4_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_4_kills"),
                "player_4_kills",
            )?,
            player_5: read_value::<u16>(stream, definition.get_entry("player_5"), "player_5")?,
            player_5_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_5_damage"),
                "player_5_damage",
            )?,
            player_5_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_5_healing"),
                "player_5_healing",
            )?,
            player_5_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_5_lifetime"),
                "player_5_lifetime",
            )?,
            player_5_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_5_kills"),
                "player_5_kills",
            )?,
            player_6: read_value::<u16>(stream, definition.get_entry("player_6"), "player_6")?,
            player_6_damage: read_value::<u16>(
                stream,
                definition.get_entry("player_6_damage"),
                "player_6_damage",
            )?,
            player_6_healing: read_value::<u16>(
                stream,
                definition.get_entry("player_6_healing"),
                "player_6_healing",
            )?,
            player_6_lifetime: read_value::<u16>(
                stream,
                definition.get_entry("player_6_lifetime"),
                "player_6_lifetime",
            )?,
            player_6_kills: read_value::<u16>(
                stream,
                definition.get_entry("player_6_kills"),
                "player_6_kills",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "panel_style" => Ok(self.panel_style.clone().into()),
            "winning_team" => Ok(self.winning_team.clone().into()),
            "winreason" => Ok(self.win_reason.clone().into()),
            "cappers" => Ok(self.cappers.clone().into()),
            "flagcaplimit" => Ok(self.flag_cap_limit.clone().into()),
            "blue_score" => Ok(self.blue_score.clone().into()),
            "red_score" => Ok(self.red_score.clone().into()),
            "blue_score_prev" => Ok(self.blue_score_prev.clone().into()),
            "red_score_prev" => Ok(self.red_score_prev.clone().into()),
            "round_complete" => Ok(self.round_complete.clone().into()),
            "player_1" => Ok(self.player_1.clone().into()),
            "player_1_damage" => Ok(self.player_1_damage.clone().into()),
            "player_1_healing" => Ok(self.player_1_healing.clone().into()),
            "player_1_lifetime" => Ok(self.player_1_lifetime.clone().into()),
            "player_1_kills" => Ok(self.player_1_kills.clone().into()),
            "player_2" => Ok(self.player_2.clone().into()),
            "player_2_damage" => Ok(self.player_2_damage.clone().into()),
            "player_2_healing" => Ok(self.player_2_healing.clone().into()),
            "player_2_lifetime" => Ok(self.player_2_lifetime.clone().into()),
            "player_2_kills" => Ok(self.player_2_kills.clone().into()),
            "player_3" => Ok(self.player_3.clone().into()),
            "player_3_damage" => Ok(self.player_3_damage.clone().into()),
            "player_3_healing" => Ok(self.player_3_healing.clone().into()),
            "player_3_lifetime" => Ok(self.player_3_lifetime.clone().into()),
            "player_3_kills" => Ok(self.player_3_kills.clone().into()),
            "player_4" => Ok(self.player_4.clone().into()),
            "player_4_damage" => Ok(self.player_4_damage.clone().into()),
            "player_4_healing" => Ok(self.player_4_healing.clone().into()),
            "player_4_lifetime" => Ok(self.player_4_lifetime.clone().into()),
            "player_4_kills" => Ok(self.player_4_kills.clone().into()),
            "player_5" => Ok(self.player_5.clone().into()),
            "player_5_damage" => Ok(self.player_5_damage.clone().into()),
            "player_5_healing" => Ok(self.player_5_healing.clone().into()),
            "player_5_lifetime" => Ok(self.player_5_lifetime.clone().into()),
            "player_5_kills" => Ok(self.player_5_kills.clone().into()),
            "player_6" => Ok(self.player_6.clone().into()),
            "player_6_damage" => Ok(self.player_6_damage.clone().into()),
            "player_6_healing" => Ok(self.player_6_healing.clone().into()),
            "player_6_lifetime" => Ok(self.player_6_lifetime.clone().into()),
            "player_6_kills" => Ok(self.player_6_kills.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArenaWinPanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PveWinPanelEvent {
    pub panel_style: u8,
    pub winning_team: u8,
    pub win_reason: u8,
}
impl PveWinPanelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PveWinPanelEvent {
            panel_style: read_value::<u8>(
                stream,
                definition.get_entry("panel_style"),
                "panel_style",
            )?,
            winning_team: read_value::<u8>(
                stream,
                definition.get_entry("winning_team"),
                "winning_team",
            )?,
            win_reason: read_value::<u8>(stream, definition.get_entry("winreason"), "win_reason")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "panel_style" => Ok(self.panel_style.clone().into()),
            "winning_team" => Ok(self.winning_team.clone().into()),
            "winreason" => Ok(self.win_reason.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PveWinPanel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AirDashEvent {
    pub player: u8,
}
impl AirDashEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AirDashEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AirDash",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LandedEvent {
    pub player: u8,
}
impl LandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LandedEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "Landed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDamageDodgedEvent {
    pub damage: u16,
}
impl PlayerDamageDodgedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDamageDodgedEvent {
            damage: read_value::<u16>(stream, definition.get_entry("damage"), "damage")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "damage" => Ok(self.damage.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDamageDodged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerStunnedEvent {
    pub stunner: u16,
    pub victim: u16,
    pub victim_capping: bool,
    pub big_stun: bool,
}
impl PlayerStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerStunnedEvent {
            stunner: read_value::<u16>(stream, definition.get_entry("stunner"), "stunner")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
            victim_capping: read_value::<bool>(
                stream,
                definition.get_entry("victim_capping"),
                "victim_capping",
            )?,
            big_stun: read_value::<bool>(stream, definition.get_entry("big_stun"), "big_stun")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "stunner" => Ok(self.stunner.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            "victim_capping" => Ok(self.victim_capping.clone().into()),
            "big_stun" => Ok(self.big_stun.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerStunned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutGrandSlamEvent {
    pub scout_id: u16,
    pub target_id: u16,
}
impl ScoutGrandSlamEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoutGrandSlamEvent {
            scout_id: read_value::<u16>(stream, definition.get_entry("scout_id"), "scout_id")?,
            target_id: read_value::<u16>(stream, definition.get_entry("target_id"), "target_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "scout_id" => Ok(self.scout_id.clone().into()),
            "target_id" => Ok(self.target_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoutGrandSlam",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoutSlamdollLandedEvent {
    pub target_index: u16,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl ScoutSlamdollLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoutSlamdollLandedEvent {
            target_index: read_value::<u16>(
                stream,
                definition.get_entry("target_index"),
                "target_index",
            )?,
            x: read_value::<f32>(stream, definition.get_entry("x"), "x")?,
            y: read_value::<f32>(stream, definition.get_entry("y"), "y")?,
            z: read_value::<f32>(stream, definition.get_entry("z"), "z")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "target_index" => Ok(self.target_index.clone().into()),
            "x" => Ok(self.x.clone().into()),
            "y" => Ok(self.y.clone().into()),
            "z" => Ok(self.z.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoutSlamdollLanded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ArrowImpactEvent {
            attached_entity: read_value::<u16>(
                stream,
                definition.get_entry("attachedEntity"),
                "attached_entity",
            )?,
            shooter: read_value::<u16>(stream, definition.get_entry("shooter"), "shooter")?,
            bone_index_attached: read_value::<u16>(
                stream,
                definition.get_entry("boneIndexAttached"),
                "bone_index_attached",
            )?,
            bone_position_x: read_value::<f32>(
                stream,
                definition.get_entry("bonePositionX"),
                "bone_position_x",
            )?,
            bone_position_y: read_value::<f32>(
                stream,
                definition.get_entry("bonePositionY"),
                "bone_position_y",
            )?,
            bone_position_z: read_value::<f32>(
                stream,
                definition.get_entry("bonePositionZ"),
                "bone_position_z",
            )?,
            bone_angles_x: read_value::<f32>(
                stream,
                definition.get_entry("boneAnglesX"),
                "bone_angles_x",
            )?,
            bone_angles_y: read_value::<f32>(
                stream,
                definition.get_entry("boneAnglesY"),
                "bone_angles_y",
            )?,
            bone_angles_z: read_value::<f32>(
                stream,
                definition.get_entry("boneAnglesZ"),
                "bone_angles_z",
            )?,
            projectile_type: read_value::<u16>(
                stream,
                definition.get_entry("projectileType"),
                "projectile_type",
            )?,
            is_crit: read_value::<bool>(stream, definition.get_entry("isCrit"), "is_crit")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attachedEntity" => Ok(self.attached_entity.clone().into()),
            "shooter" => Ok(self.shooter.clone().into()),
            "boneIndexAttached" => Ok(self.bone_index_attached.clone().into()),
            "bonePositionX" => Ok(self.bone_position_x.clone().into()),
            "bonePositionY" => Ok(self.bone_position_y.clone().into()),
            "bonePositionZ" => Ok(self.bone_position_z.clone().into()),
            "boneAnglesX" => Ok(self.bone_angles_x.clone().into()),
            "boneAnglesY" => Ok(self.bone_angles_y.clone().into()),
            "boneAnglesZ" => Ok(self.bone_angles_z.clone().into()),
            "projectileType" => Ok(self.projectile_type.clone().into()),
            "isCrit" => Ok(self.is_crit.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ArrowImpact",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerJaratedEvent {
            thrower_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("thrower_entindex"),
                "thrower_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "thrower_entindex" => Ok(self.thrower_ent_index.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerJarated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerJaratedFadeEvent {
    pub thrower_ent_index: u8,
    pub victim_ent_index: u8,
}
impl PlayerJaratedFadeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerJaratedFadeEvent {
            thrower_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("thrower_entindex"),
                "thrower_ent_index",
            )?,
            victim_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "thrower_entindex" => Ok(self.thrower_ent_index.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerJaratedFade",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerShieldBlockedEvent {
    pub attacker_ent_index: u8,
    pub blocker_ent_index: u8,
}
impl PlayerShieldBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerShieldBlockedEvent {
            attacker_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("attacker_entindex"),
                "attacker_ent_index",
            )?,
            blocker_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("blocker_entindex"),
                "blocker_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attacker_entindex" => Ok(self.attacker_ent_index.clone().into()),
            "blocker_entindex" => Ok(self.blocker_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerShieldBlocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerPinnedEvent {
    pub pinned: u8,
}
impl PlayerPinnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerPinnedEvent {
            pinned: read_value::<u8>(stream, definition.get_entry("pinned"), "pinned")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "pinned" => Ok(self.pinned.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerPinned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedByMedicEvent {
    pub medic: u8,
}
impl PlayerHealedByMedicEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealedByMedicEvent {
            medic: read_value::<u8>(stream, definition.get_entry("medic"), "medic")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "medic" => Ok(self.medic.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealedByMedic",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerSappedObjectEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub object: u8,
    pub sapper_id: u16,
}
impl PlayerSappedObjectEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerSappedObjectEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            owner_id: read_value::<u16>(stream, definition.get_entry("ownerid"), "owner_id")?,
            object: read_value::<u8>(stream, definition.get_entry("object"), "object")?,
            sapper_id: read_value::<u16>(stream, definition.get_entry("sapperid"), "sapper_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "ownerid" => Ok(self.owner_id.clone().into()),
            "object" => Ok(self.object.clone().into()),
            "sapperid" => Ok(self.sapper_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerSappedObject",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ItemFoundEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            quality: read_value::<u8>(stream, definition.get_entry("quality"), "quality")?,
            method: read_value::<u8>(stream, definition.get_entry("method"), "method")?,
            item_def: read_value::<u32>(stream, definition.get_entry("itemdef"), "item_def")?,
            is_strange: read_value::<u8>(stream, definition.get_entry("isstrange"), "is_strange")?,
            is_unusual: read_value::<u8>(stream, definition.get_entry("isunusual"), "is_unusual")?,
            wear: read_value::<f32>(stream, definition.get_entry("wear"), "wear")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "quality" => Ok(self.quality.clone().into()),
            "method" => Ok(self.method.clone().into()),
            "itemdef" => Ok(self.item_def.clone().into()),
            "isstrange" => Ok(self.is_strange.clone().into()),
            "isunusual" => Ok(self.is_unusual.clone().into()),
            "wear" => Ok(self.wear.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemFound",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ShowAnnotationEvent {
            world_pos_x: read_value::<f32>(
                stream,
                definition.get_entry("worldPosX"),
                "world_pos_x",
            )?,
            world_pos_y: read_value::<f32>(
                stream,
                definition.get_entry("worldPosY"),
                "world_pos_y",
            )?,
            world_pos_z: read_value::<f32>(
                stream,
                definition.get_entry("worldPosZ"),
                "world_pos_z",
            )?,
            world_normal_x: read_value::<f32>(
                stream,
                definition.get_entry("worldNormalX"),
                "world_normal_x",
            )?,
            world_normal_y: read_value::<f32>(
                stream,
                definition.get_entry("worldNormalY"),
                "world_normal_y",
            )?,
            world_normal_z: read_value::<f32>(
                stream,
                definition.get_entry("worldNormalZ"),
                "world_normal_z",
            )?,
            id: read_value::<u32>(stream, definition.get_entry("id"), "id")?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
            lifetime: read_value::<f32>(stream, definition.get_entry("lifetime"), "lifetime")?,
            visibility_bit_field: read_value::<u32>(
                stream,
                definition.get_entry("visibilityBitfield"),
                "visibility_bit_field",
            )?,
            follow_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("follow_entindex"),
                "follow_ent_index",
            )?,
            show_distance: read_value::<bool>(
                stream,
                definition.get_entry("show_distance"),
                "show_distance",
            )?,
            play_sound: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("play_sound"),
                "play_sound",
            )?,
            show_effect: read_value::<bool>(
                stream,
                definition.get_entry("show_effect"),
                "show_effect",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "worldPosX" => Ok(self.world_pos_x.clone().into()),
            "worldPosY" => Ok(self.world_pos_y.clone().into()),
            "worldPosZ" => Ok(self.world_pos_z.clone().into()),
            "worldNormalX" => Ok(self.world_normal_x.clone().into()),
            "worldNormalY" => Ok(self.world_normal_y.clone().into()),
            "worldNormalZ" => Ok(self.world_normal_z.clone().into()),
            "id" => Ok(self.id.clone().into()),
            "text" => Ok(self.text.clone().into()),
            "lifetime" => Ok(self.lifetime.clone().into()),
            "visibilityBitfield" => Ok(self.visibility_bit_field.clone().into()),
            "follow_entindex" => Ok(self.follow_ent_index.clone().into()),
            "show_distance" => Ok(self.show_distance.clone().into()),
            "play_sound" => Ok(self.play_sound.clone().into()),
            "show_effect" => Ok(self.show_effect.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowAnnotation",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HideAnnotationEvent {
    pub id: u32,
}
impl HideAnnotationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HideAnnotationEvent {
            id: read_value::<u32>(stream, definition.get_entry("id"), "id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "id" => Ok(self.id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HideAnnotation",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PostInventoryApplicationEvent {
    pub user_id: u16,
}
impl PostInventoryApplicationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PostInventoryApplicationEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PostInventoryApplication",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointUnlockUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl ControlPointUnlockUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointUnlockUpdatedEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            time: read_value::<f32>(stream, definition.get_entry("time"), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "time" => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointUnlockUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeployBuffBannerEvent {
    pub buff_type: u8,
    pub buff_owner: u16,
}
impl DeployBuffBannerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DeployBuffBannerEvent {
            buff_type: read_value::<u8>(stream, definition.get_entry("buff_type"), "buff_type")?,
            buff_owner: read_value::<u16>(
                stream,
                definition.get_entry("buff_owner"),
                "buff_owner",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "buff_type" => Ok(self.buff_type.clone().into()),
            "buff_owner" => Ok(self.buff_owner.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DeployBuffBanner",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuffEvent {
    pub user_id: u16,
    pub buff_owner: u16,
    pub buff_type: u8,
}
impl PlayerBuffEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBuffEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            buff_owner: read_value::<u16>(
                stream,
                definition.get_entry("buff_owner"),
                "buff_owner",
            )?,
            buff_type: read_value::<u8>(stream, definition.get_entry("buff_type"), "buff_type")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "buff_owner" => Ok(self.buff_owner.clone().into()),
            "buff_type" => Ok(self.buff_type.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBuff",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDeathEvent {
    pub user_id: u16,
    pub attacker: u16,
    pub healing: u16,
    pub charged: bool,
}
impl MedicDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MedicDeathEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            healing: read_value::<u16>(stream, definition.get_entry("healing"), "healing")?,
            charged: read_value::<bool>(stream, definition.get_entry("charged"), "charged")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "healing" => Ok(self.healing.clone().into()),
            "charged" => Ok(self.charged.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MedicDeath",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct OvertimeNagEvent {}
impl OvertimeNagEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(OvertimeNagEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "OvertimeNag",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamsChangedEvent {}
impl TeamsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamsChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamsChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenPumpkinGrabEvent {
    pub user_id: u16,
}
impl HalloweenPumpkinGrabEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenPumpkinGrabEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenPumpkinGrab",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketJumpEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry("playsound"),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "playsound" => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketJump",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketJumpLandedEvent {
    pub user_id: u16,
}
impl RocketJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketJumpLandedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketJumpLanded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl StickyJumpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StickyJumpEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry("playsound"),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "playsound" => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StickyJump",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StickyJumpLandedEvent {
    pub user_id: u16,
}
impl StickyJumpLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StickyJumpLandedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StickyJumpLanded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLaunchEvent {
    pub user_id: u16,
    pub play_sound: bool,
}
impl RocketPackLaunchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketPackLaunchEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            play_sound: read_value::<bool>(
                stream,
                definition.get_entry("playsound"),
                "play_sound",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "playsound" => Ok(self.play_sound.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketPackLaunch",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RocketPackLandedEvent {
    pub user_id: u16,
}
impl RocketPackLandedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RocketPackLandedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RocketPackLanded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedicDefendedEvent {
    pub user_id: u16,
    pub medic: u16,
}
impl MedicDefendedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MedicDefendedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            medic: read_value::<u16>(stream, definition.get_entry("medic"), "medic")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "medic" => Ok(self.medic.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MedicDefended",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerHealedEvent {
    pub amount: u16,
}
impl LocalPlayerHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerHealedEvent {
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "amount" => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerHealed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDestroyedPipeBombEvent {
    pub user_id: u16,
}
impl PlayerDestroyedPipeBombEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDestroyedPipeBombEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDestroyedPipeBomb",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ObjectDeflectedEvent {
    pub user_id: u16,
    pub owner_id: u16,
    pub weapon_id: u16,
    pub object_ent_index: u16,
}
impl ObjectDeflectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ObjectDeflectedEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            owner_id: read_value::<u16>(stream, definition.get_entry("ownerid"), "owner_id")?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            object_ent_index: read_value::<u16>(
                stream,
                definition.get_entry("object_entindex"),
                "object_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "ownerid" => Ok(self.owner_id.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "object_entindex" => Ok(self.object_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ObjectDeflected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerMvpEvent {
    pub player: u16,
}
impl PlayerMvpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerMvpEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerMvp",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnMobEvent {}
impl RaidSpawnMobEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnMobEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RaidSpawnMob",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RaidSpawnSquadEvent {}
impl RaidSpawnSquadEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RaidSpawnSquadEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RaidSpawnSquad",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NavBlockedEvent {
    pub area: u32,
    pub blocked: bool,
}
impl NavBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(NavBlockedEvent {
            area: read_value::<u32>(stream, definition.get_entry("area"), "area")?,
            blocked: read_value::<bool>(stream, definition.get_entry("blocked"), "blocked")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "area" => Ok(self.area.clone().into()),
            "blocked" => Ok(self.blocked.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "NavBlocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PathTrackPassedEvent {
    pub index: u16,
}
impl PathTrackPassedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PathTrackPassedEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PathTrackPassed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct NumCappersChangedEvent {
    pub index: u16,
    pub count: u8,
}
impl NumCappersChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(NumCappersChangedEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            count: read_value::<u8>(stream, definition.get_entry("count"), "count")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "count" => Ok(self.count.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "NumCappersChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRegenerateEvent {}
impl PlayerRegenerateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerRegenerateEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerRegenerate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpdateStatusItemEvent {
    pub index: u8,
    pub object: u8,
}
impl UpdateStatusItemEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(UpdateStatusItemEvent {
            index: read_value::<u8>(stream, definition.get_entry("index"), "index")?,
            object: read_value::<u8>(stream, definition.get_entry("object"), "object")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "object" => Ok(self.object.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "UpdateStatusItem",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StatsResetRoundEvent {}
impl StatsResetRoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StatsResetRoundEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StatsResetRound",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoreStatsAccumulatedUpdateEvent {}
impl ScoreStatsAccumulatedUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoreStatsAccumulatedUpdateEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoreStatsAccumulatedUpdate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ScoreStatsAccumulatedResetEvent {}
impl ScoreStatsAccumulatedResetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ScoreStatsAccumulatedResetEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ScoreStatsAccumulatedReset",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AchievementEarnedLocalEvent {
    pub achievement: u16,
}
impl AchievementEarnedLocalEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(AchievementEarnedLocalEvent {
            achievement: read_value::<u16>(
                stream,
                definition.get_entry("achievement"),
                "achievement",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "achievement" => Ok(self.achievement.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "AchievementEarnedLocal",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHealedEvent {
    pub patient: u16,
    pub healer: u16,
    pub amount: u16,
}
impl PlayerHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHealedEvent {
            patient: read_value::<u16>(stream, definition.get_entry("patient"), "patient")?,
            healer: read_value::<u16>(stream, definition.get_entry("healer"), "healer")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "patient" => Ok(self.patient.clone().into()),
            "healer" => Ok(self.healer.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHealed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BuildingHealedEvent {
    pub building: u16,
    pub healer: u16,
    pub amount: u16,
}
impl BuildingHealedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BuildingHealedEvent {
            building: read_value::<u16>(stream, definition.get_entry("building"), "building")?,
            healer: read_value::<u16>(stream, definition.get_entry("healer"), "healer")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "building" => Ok(self.building.clone().into()),
            "healer" => Ok(self.healer.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BuildingHealed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemPickupEvent {
    pub user_id: u16,
    pub item: MaybeUtf8String,
}
impl ItemPickupEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemPickupEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            item: read_value::<MaybeUtf8String>(stream, definition.get_entry("item"), "item")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "item" => Ok(self.item.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemPickup",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(DuelStatusEvent {
            killer: read_value::<u16>(stream, definition.get_entry("killer"), "killer")?,
            score_type: read_value::<u16>(
                stream,
                definition.get_entry("score_type"),
                "score_type",
            )?,
            initiator: read_value::<u16>(stream, definition.get_entry("initiator"), "initiator")?,
            target: read_value::<u16>(stream, definition.get_entry("target"), "target")?,
            initiator_score: read_value::<u16>(
                stream,
                definition.get_entry("initiator_score"),
                "initiator_score",
            )?,
            target_score: read_value::<u16>(
                stream,
                definition.get_entry("target_score"),
                "target_score",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "killer" => Ok(self.killer.clone().into()),
            "score_type" => Ok(self.score_type.clone().into()),
            "initiator" => Ok(self.initiator.clone().into()),
            "target" => Ok(self.target.clone().into()),
            "initiator_score" => Ok(self.initiator_score.clone().into()),
            "target_score" => Ok(self.target_score.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DuelStatus",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(FishNoticeEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry("stun_flags"),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry("death_flags"),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry("silent_kill"),
                "silent_kill",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("assister_fallback"),
                "assister_fallback",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            "stun_flags" => Ok(self.stun_flags.clone().into()),
            "death_flags" => Ok(self.death_flags.clone().into()),
            "silent_kill" => Ok(self.silent_kill.clone().into()),
            "assister_fallback" => Ok(self.assister_fallback.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FishNotice",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(FishNoticeArmEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry("stun_flags"),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry("death_flags"),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry("silent_kill"),
                "silent_kill",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("assister_fallback"),
                "assister_fallback",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            "stun_flags" => Ok(self.stun_flags.clone().into()),
            "death_flags" => Ok(self.death_flags.clone().into()),
            "silent_kill" => Ok(self.silent_kill.clone().into()),
            "assister_fallback" => Ok(self.assister_fallback.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FishNoticeArm",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(SlapNoticeEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry("stun_flags"),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry("death_flags"),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry("silent_kill"),
                "silent_kill",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("assister_fallback"),
                "assister_fallback",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            "stun_flags" => Ok(self.stun_flags.clone().into()),
            "death_flags" => Ok(self.death_flags.clone().into()),
            "silent_kill" => Ok(self.silent_kill.clone().into()),
            "assister_fallback" => Ok(self.assister_fallback.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SlapNotice",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ThrowableHitEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
            stun_flags: read_value::<u16>(
                stream,
                definition.get_entry("stun_flags"),
                "stun_flags",
            )?,
            death_flags: read_value::<u16>(
                stream,
                definition.get_entry("death_flags"),
                "death_flags",
            )?,
            silent_kill: read_value::<bool>(
                stream,
                definition.get_entry("silent_kill"),
                "silent_kill",
            )?,
            assister_fallback: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("assister_fallback"),
                "assister_fallback",
            )?,
            total_hits: read_value::<u16>(stream, definition.get_entry("totalhits"), "total_hits")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            "stun_flags" => Ok(self.stun_flags.clone().into()),
            "death_flags" => Ok(self.death_flags.clone().into()),
            "silent_kill" => Ok(self.silent_kill.clone().into()),
            "assister_fallback" => Ok(self.assister_fallback.clone().into()),
            "totalhits" => Ok(self.total_hits.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ThrowableHit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PumpkinLordSummonedEvent {}
impl PumpkinLordSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PumpkinLordSummonedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PumpkinLordSummoned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PumpkinLordKilledEvent {}
impl PumpkinLordKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PumpkinLordKilledEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PumpkinLordKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusSummonedEvent {
    pub level: u16,
}
impl MerasmusSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusSummonedEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusSummoned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusKilledEvent {
    pub level: u16,
}
impl MerasmusKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusKilledEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusEscapeWarningEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl MerasmusEscapeWarningEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusEscapeWarningEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
            time_remaining: read_value::<u8>(
                stream,
                definition.get_entry("time_remaining"),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            "time_remaining" => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusEscapeWarning",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusEscapedEvent {
    pub level: u16,
}
impl MerasmusEscapedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusEscapedEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusEscaped",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossSummonedEvent {
    pub level: u16,
}
impl EyeballBossSummonedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossSummonedEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossSummoned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossStunnedEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl EyeballBossStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossStunnedEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
            player_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("player_entindex"),
                "player_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            "player_entindex" => Ok(self.player_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossStunned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossKilledEvent {
    pub level: u16,
}
impl EyeballBossKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossKilledEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossKillerEvent {
    pub level: u16,
    pub player_ent_index: u8,
}
impl EyeballBossKillerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossKillerEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
            player_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("player_entindex"),
                "player_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            "player_entindex" => Ok(self.player_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossKiller",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossEscapeImminentEvent {
    pub level: u16,
    pub time_remaining: u8,
}
impl EyeballBossEscapeImminentEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossEscapeImminentEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
            time_remaining: read_value::<u8>(
                stream,
                definition.get_entry("time_remaining"),
                "time_remaining",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            "time_remaining" => Ok(self.time_remaining.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossEscapeImminent",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EyeballBossEscapedEvent {
    pub level: u16,
}
impl EyeballBossEscapedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EyeballBossEscapedEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EyeballBossEscaped",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(NpcHurtEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
            health: read_value::<u16>(stream, definition.get_entry("health"), "health")?,
            attacker_player: read_value::<u16>(
                stream,
                definition.get_entry("attacker_player"),
                "attacker_player",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_amount: read_value::<u16>(
                stream,
                definition.get_entry("damageamount"),
                "damage_amount",
            )?,
            crit: read_value::<bool>(stream, definition.get_entry("crit"), "crit")?,
            boss: read_value::<u16>(stream, definition.get_entry("boss"), "boss")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            "health" => Ok(self.health.clone().into()),
            "attacker_player" => Ok(self.attacker_player.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damageamount" => Ok(self.damage_amount.clone().into()),
            "crit" => Ok(self.crit.clone().into()),
            "boss" => Ok(self.boss.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "NpcHurt",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ControlPointTimerUpdatedEvent {
    pub index: u16,
    pub time: f32,
}
impl ControlPointTimerUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ControlPointTimerUpdatedEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            time: read_value::<f32>(stream, definition.get_entry("time"), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "time" => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ControlPointTimerUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveStartEvent {
    pub ent_index: u8,
}
impl PlayerHighFiveStartEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHighFiveStartEvent {
            ent_index: read_value::<u8>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHighFiveStart",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveCancelEvent {
    pub ent_index: u8,
}
impl PlayerHighFiveCancelEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHighFiveCancelEvent {
            ent_index: read_value::<u8>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHighFiveCancel",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerHighFiveSuccessEvent {
    pub initiator_ent_index: u8,
    pub partner_ent_index: u8,
}
impl PlayerHighFiveSuccessEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerHighFiveSuccessEvent {
            initiator_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("initiator_entindex"),
                "initiator_ent_index",
            )?,
            partner_ent_index: read_value::<u8>(
                stream,
                definition.get_entry("partner_entindex"),
                "partner_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "initiator_entindex" => Ok(self.initiator_ent_index.clone().into()),
            "partner_entindex" => Ok(self.partner_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerHighFiveSuccess",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBonusPointsEvent {
    pub points: u16,
    pub player_ent_index: u16,
    pub source_ent_index: u16,
}
impl PlayerBonusPointsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBonusPointsEvent {
            points: read_value::<u16>(stream, definition.get_entry("points"), "points")?,
            player_ent_index: read_value::<u16>(
                stream,
                definition.get_entry("player_entindex"),
                "player_ent_index",
            )?,
            source_ent_index: read_value::<u16>(
                stream,
                definition.get_entry("source_entindex"),
                "source_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "points" => Ok(self.points.clone().into()),
            "player_entindex" => Ok(self.player_ent_index.clone().into()),
            "source_entindex" => Ok(self.source_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBonusPoints",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUpgradedEvent {}
impl PlayerUpgradedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUpgradedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUpgraded",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerBuybackEvent {
    pub player: u16,
    pub cost: u16,
}
impl PlayerBuybackEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerBuybackEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            cost: read_value::<u16>(stream, definition.get_entry("cost"), "cost")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "cost" => Ok(self.cost.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerBuyback",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerUsedPowerUpBottleEvent {
    pub player: u16,
    pub kind: u16,
    pub time: f32,
}
impl PlayerUsedPowerUpBottleEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerUsedPowerUpBottleEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            kind: read_value::<u16>(stream, definition.get_entry("type"), "kind")?,
            time: read_value::<f32>(stream, definition.get_entry("time"), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "type" => Ok(self.kind.clone().into()),
            "time" => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerUsedPowerUpBottle",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ChristmasGiftGrabEvent {
    pub user_id: u16,
}
impl ChristmasGiftGrabEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ChristmasGiftGrabEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ChristmasGiftGrab",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerKilledAchievementZoneEvent {
    pub attacker: u16,
    pub victim: u16,
    pub zone_id: u16,
}
impl PlayerKilledAchievementZoneEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerKilledAchievementZoneEvent {
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
            zone_id: read_value::<u16>(stream, definition.get_entry("zone_id"), "zone_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attacker" => Ok(self.attacker.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            "zone_id" => Ok(self.zone_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerKilledAchievementZone",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyUpdatedEvent {}
impl PartyUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyPrefChangedEvent {}
impl PartyPrefChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyPrefChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyPrefChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyCriteriaChangedEvent {}
impl PartyCriteriaChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyCriteriaChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyCriteriaChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyInvitesChangedEvent {}
impl PartyInvitesChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyInvitesChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyInvitesChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyQueueStateChangedEvent {
    pub match_group: u16,
}
impl PartyQueueStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyQueueStateChangedEvent {
            match_group: read_value::<u16>(
                stream,
                definition.get_entry("matchgroup"),
                "match_group",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "matchgroup" => Ok(self.match_group.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyQueueStateChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyChatEvent {
    pub steam_id: MaybeUtf8String,
    pub text: MaybeUtf8String,
    pub kind: u16,
}
impl PartyChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyChatEvent {
            steam_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("steamid"),
                "steam_id",
            )?,
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
            kind: read_value::<u16>(stream, definition.get_entry("type"), "kind")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "steamid" => Ok(self.steam_id.clone().into()),
            "text" => Ok(self.text.clone().into()),
            "type" => Ok(self.kind.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyChat",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyMemberJoinEvent {
    pub steam_id: MaybeUtf8String,
}
impl PartyMemberJoinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyMemberJoinEvent {
            steam_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("steamid"),
                "steam_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "steamid" => Ok(self.steam_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyMemberJoin",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PartyMemberLeaveEvent {
    pub steam_id: MaybeUtf8String,
}
impl PartyMemberLeaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PartyMemberLeaveEvent {
            steam_id: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("steamid"),
                "steam_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "steamid" => Ok(self.steam_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PartyMemberLeave",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MatchInvitesUpdatedEvent {}
impl MatchInvitesUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MatchInvitesUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MatchInvitesUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LobbyUpdatedEvent {}
impl LobbyUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LobbyUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LobbyUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMissionUpdateEvent {
    pub class: u16,
    pub count: u16,
}
impl MvmMissionUpdateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmMissionUpdateEvent {
            class: read_value::<u16>(stream, definition.get_entry("class"), "class")?,
            count: read_value::<u16>(stream, definition.get_entry("count"), "count")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "class" => Ok(self.class.clone().into()),
            "count" => Ok(self.count.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmMissionUpdate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecalculateHolidaysEvent {}
impl RecalculateHolidaysEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RecalculateHolidaysEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RecalculateHolidays",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerCurrencyChangedEvent {
    pub currency: u16,
}
impl PlayerCurrencyChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerCurrencyChangedEvent {
            currency: read_value::<u16>(stream, definition.get_entry("currency"), "currency")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "currency" => Ok(self.currency.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerCurrencyChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DoomsdayRocketOpenEvent {
    pub team: u8,
}
impl DoomsdayRocketOpenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DoomsdayRocketOpenEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DoomsdayRocketOpen",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RemoveNemesisRelationshipsEvent {
    pub player: u16,
}
impl RemoveNemesisRelationshipsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RemoveNemesisRelationshipsEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RemoveNemesisRelationships",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusWaveEvent {}
impl MvmCreditBonusWaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusWaveEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmCreditBonusWave",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusAllEvent {}
impl MvmCreditBonusAllEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusAllEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmCreditBonusAll",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmCreditBonusAllAdvancedEvent {}
impl MvmCreditBonusAllAdvancedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmCreditBonusAllAdvancedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmCreditBonusAllAdvanced",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmQuickSentryUpgradeEvent {
    pub player: u16,
}
impl MvmQuickSentryUpgradeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmQuickSentryUpgradeEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmQuickSentryUpgrade",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmTankDestroyedByPlayersEvent {}
impl MvmTankDestroyedByPlayersEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmTankDestroyedByPlayersEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmTankDestroyedByPlayers",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmKillRobotDeliveringBombEvent {
    pub player: u16,
}
impl MvmKillRobotDeliveringBombEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmKillRobotDeliveringBombEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmKillRobotDeliveringBomb",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmPickupCurrencyEvent {
    pub player: u16,
    pub currency: u16,
}
impl MvmPickupCurrencyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmPickupCurrencyEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            currency: read_value::<u16>(stream, definition.get_entry("currency"), "currency")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "currency" => Ok(self.currency.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmPickupCurrency",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombCarrierKilledEvent {
    pub level: u16,
}
impl MvmBombCarrierKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBombCarrierKilledEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmBombCarrierKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSentryBusterDetonateEvent {
    pub player: u16,
    pub det_x: f32,
    pub det_y: f32,
    pub det_z: f32,
}
impl MvmSentryBusterDetonateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmSentryBusterDetonateEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            det_x: read_value::<f32>(stream, definition.get_entry("det_x"), "det_x")?,
            det_y: read_value::<f32>(stream, definition.get_entry("det_y"), "det_y")?,
            det_z: read_value::<f32>(stream, definition.get_entry("det_z"), "det_z")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "det_x" => Ok(self.det_x.clone().into()),
            "det_y" => Ok(self.det_y.clone().into()),
            "det_z" => Ok(self.det_z.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmSentryBusterDetonate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmScoutMarkedForDeathEvent {
    pub player: u16,
}
impl MvmScoutMarkedForDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmScoutMarkedForDeathEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmScoutMarkedForDeath",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMedicPowerUpSharedEvent {
    pub player: u16,
}
impl MvmMedicPowerUpSharedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmMedicPowerUpSharedEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmMedicPowerUpShared",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBeginWaveEvent {
    pub wave_index: u16,
    pub max_waves: u16,
    pub advanced: u16,
}
impl MvmBeginWaveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBeginWaveEvent {
            wave_index: read_value::<u16>(
                stream,
                definition.get_entry("wave_index"),
                "wave_index",
            )?,
            max_waves: read_value::<u16>(stream, definition.get_entry("max_waves"), "max_waves")?,
            advanced: read_value::<u16>(stream, definition.get_entry("advanced"), "advanced")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "wave_index" => Ok(self.wave_index.clone().into()),
            "max_waves" => Ok(self.max_waves.clone().into()),
            "advanced" => Ok(self.advanced.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmBeginWave",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmWaveCompleteEvent {
    pub advanced: bool,
}
impl MvmWaveCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmWaveCompleteEvent {
            advanced: read_value::<bool>(stream, definition.get_entry("advanced"), "advanced")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "advanced" => Ok(self.advanced.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmWaveComplete",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMissionCompleteEvent {
    pub mission: MaybeUtf8String,
}
impl MvmMissionCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmMissionCompleteEvent {
            mission: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("mission"),
                "mission",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "mission" => Ok(self.mission.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmMissionComplete",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombResetByPlayerEvent {
    pub player: u16,
}
impl MvmBombResetByPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBombResetByPlayerEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmBombResetByPlayer",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombAlarmTriggeredEvent {}
impl MvmBombAlarmTriggeredEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBombAlarmTriggeredEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmBombAlarmTriggered",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmBombDeployResetByPlayerEvent {
    pub player: u16,
}
impl MvmBombDeployResetByPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmBombDeployResetByPlayerEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmBombDeployResetByPlayer",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmWaveFailedEvent {}
impl MvmWaveFailedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmWaveFailedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmWaveFailed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmResetStatsEvent {}
impl MvmResetStatsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmResetStatsEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmResetStats",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamageResistedEvent {
    pub ent_index: u8,
}
impl DamageResistedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DamageResistedEvent {
            ent_index: read_value::<u8>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DamageResisted",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerNotifyEvent {
    pub ent_index: u16,
    pub marker_ent_index: u16,
}
impl RevivePlayerNotifyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RevivePlayerNotifyEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
            marker_ent_index: read_value::<u16>(
                stream,
                definition.get_entry("marker_entindex"),
                "marker_ent_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            "marker_entindex" => Ok(self.marker_ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RevivePlayerNotify",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerStoppedEvent {
    pub ent_index: u16,
}
impl RevivePlayerStoppedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RevivePlayerStoppedEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RevivePlayerStopped",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RevivePlayerCompleteEvent {
    pub ent_index: u16,
}
impl RevivePlayerCompleteEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RevivePlayerCompleteEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RevivePlayerComplete",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerTurnedToGhostEvent {
    pub user_id: u16,
}
impl PlayerTurnedToGhostEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerTurnedToGhostEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerTurnedToGhost",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MedigunShieldBlockedDamageEvent {
    pub user_id: u16,
    pub damage: f32,
}
impl MedigunShieldBlockedDamageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MedigunShieldBlockedDamageEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            damage: read_value::<f32>(stream, definition.get_entry("damage"), "damage")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "damage" => Ok(self.damage.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MedigunShieldBlockedDamage",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmAdvWaveCompleteNoGatesEvent {
    pub index: u16,
}
impl MvmAdvWaveCompleteNoGatesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmAdvWaveCompleteNoGatesEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmAdvWaveCompleteNoGates",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSniperHeadshotCurrencyEvent {
    pub user_id: u16,
    pub currency: u16,
}
impl MvmSniperHeadshotCurrencyEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmSniperHeadshotCurrencyEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            currency: read_value::<u16>(stream, definition.get_entry("currency"), "currency")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "currency" => Ok(self.currency.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmSniperHeadshotCurrency",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmMannhattanPitEvent {}
impl MvmMannhattanPitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmMannhattanPitEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmMannhattanPit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct FlagCarriedInDetectionZoneEvent {}
impl FlagCarriedInDetectionZoneEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(FlagCarriedInDetectionZoneEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "FlagCarriedInDetectionZone",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmAdvWaveKilledStunRadioEvent {}
impl MvmAdvWaveKilledStunRadioEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmAdvWaveKilledStunRadioEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmAdvWaveKilledStunRadio",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDirectHitStunEvent {
    pub attacker: u16,
    pub victim: u16,
}
impl PlayerDirectHitStunEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDirectHitStunEvent {
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attacker" => Ok(self.attacker.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDirectHitStun",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MvmSentryBusterKilledEvent {
    pub sentry_buster: u16,
}
impl MvmSentryBusterKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MvmSentryBusterKilledEvent {
            sentry_buster: read_value::<u16>(
                stream,
                definition.get_entry("sentry_buster"),
                "sentry_buster",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "sentry_buster" => Ok(self.sentry_buster.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MvmSentryBusterKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UpgradesFileChangedEvent {
    pub path: MaybeUtf8String,
}
impl UpgradesFileChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(UpgradesFileChangedEvent {
            path: read_value::<MaybeUtf8String>(stream, definition.get_entry("path"), "path")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "path" => Ok(self.path.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "UpgradesFileChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdTeamPointsChangedEvent {
    pub points: u16,
    pub team: u8,
    pub method: u8,
}
impl RdTeamPointsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RdTeamPointsChangedEvent {
            points: read_value::<u16>(stream, definition.get_entry("points"), "points")?,
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            method: read_value::<u8>(stream, definition.get_entry("method"), "method")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "points" => Ok(self.points.clone().into()),
            "team" => Ok(self.team.clone().into()),
            "method" => Ok(self.method.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RdTeamPointsChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdRulesStateChangedEvent {}
impl RdRulesStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RdRulesStateChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RdRulesStateChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(RdRobotKilledEvent {
            user_id: read_value::<u16>(stream, definition.get_entry("userid"), "user_id")?,
            victim_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("victim_entindex"),
                "victim_ent_index",
            )?,
            inflictor_ent_index: read_value::<u32>(
                stream,
                definition.get_entry("inflictor_entindex"),
                "inflictor_ent_index",
            )?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon"),
                "weapon",
            )?,
            weapon_id: read_value::<u16>(stream, definition.get_entry("weaponid"), "weapon_id")?,
            damage_bits: read_value::<u32>(
                stream,
                definition.get_entry("damagebits"),
                "damage_bits",
            )?,
            custom_kill: read_value::<u16>(
                stream,
                definition.get_entry("customkill"),
                "custom_kill",
            )?,
            weapon_log_class_name: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("weapon_logclassname"),
                "weapon_log_class_name",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "userid" => Ok(self.user_id.clone().into()),
            "victim_entindex" => Ok(self.victim_ent_index.clone().into()),
            "inflictor_entindex" => Ok(self.inflictor_ent_index.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon" => Ok(self.weapon.clone().into()),
            "weaponid" => Ok(self.weapon_id.clone().into()),
            "damagebits" => Ok(self.damage_bits.clone().into()),
            "customkill" => Ok(self.custom_kill.clone().into()),
            "weapon_logclassname" => Ok(self.weapon_log_class_name.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RdRobotKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdRobotImpactEvent {
    pub ent_index: u16,
    pub impulse_x: f32,
    pub impulse_y: f32,
    pub impulse_z: f32,
}
impl RdRobotImpactEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RdRobotImpactEvent {
            ent_index: read_value::<u16>(stream, definition.get_entry("entindex"), "ent_index")?,
            impulse_x: read_value::<f32>(stream, definition.get_entry("impulse_x"), "impulse_x")?,
            impulse_y: read_value::<f32>(stream, definition.get_entry("impulse_y"), "impulse_y")?,
            impulse_z: read_value::<f32>(stream, definition.get_entry("impulse_z"), "impulse_z")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "entindex" => Ok(self.ent_index.clone().into()),
            "impulse_x" => Ok(self.impulse_x.clone().into()),
            "impulse_y" => Ok(self.impulse_y.clone().into()),
            "impulse_z" => Ok(self.impulse_z.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RdRobotImpact",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamPlayPreRoundTimeLeftEvent {
    pub time: u16,
}
impl TeamPlayPreRoundTimeLeftEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamPlayPreRoundTimeLeftEvent {
            time: read_value::<u16>(stream, definition.get_entry("time"), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "time" => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamPlayPreRoundTimeLeft",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ParachuteDeployEvent {
    pub index: u16,
}
impl ParachuteDeployEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ParachuteDeployEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ParachuteDeploy",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ParachuteHolsterEvent {
    pub index: u16,
}
impl ParachuteHolsterEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ParachuteHolsterEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ParachuteHolster",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct KillRefillsMeterEvent {
    pub index: u16,
}
impl KillRefillsMeterEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(KillRefillsMeterEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "KillRefillsMeter",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RpsTauntEventEvent {
    pub winner: u16,
    pub winner_rps: u8,
    pub loser: u16,
    pub loser_rps: u8,
}
impl RpsTauntEventEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RpsTauntEventEvent {
            winner: read_value::<u16>(stream, definition.get_entry("winner"), "winner")?,
            winner_rps: read_value::<u8>(stream, definition.get_entry("winner_rps"), "winner_rps")?,
            loser: read_value::<u16>(stream, definition.get_entry("loser"), "loser")?,
            loser_rps: read_value::<u8>(stream, definition.get_entry("loser_rps"), "loser_rps")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "winner" => Ok(self.winner.clone().into()),
            "winner_rps" => Ok(self.winner_rps.clone().into()),
            "loser" => Ok(self.loser.clone().into()),
            "loser_rps" => Ok(self.loser_rps.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RpsTauntEvent",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CongaKillEvent {
    pub index: u16,
}
impl CongaKillEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CongaKillEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CongaKill",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerInitialSpawnEvent {
    pub index: u16,
}
impl PlayerInitialSpawnEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerInitialSpawnEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerInitialSpawn",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompetitiveVictoryEvent {}
impl CompetitiveVictoryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CompetitiveVictoryEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CompetitiveVictory",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(CompetitiveStatsUpdateEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            kills_rank: read_value::<u8>(stream, definition.get_entry("kills_rank"), "kills_rank")?,
            score_rank: read_value::<u8>(stream, definition.get_entry("score_rank"), "score_rank")?,
            damage_rank: read_value::<u8>(
                stream,
                definition.get_entry("damage_rank"),
                "damage_rank",
            )?,
            healing_rank: read_value::<u8>(
                stream,
                definition.get_entry("healing_rank"),
                "healing_rank",
            )?,
            support_rank: read_value::<u8>(
                stream,
                definition.get_entry("support_rank"),
                "support_rank",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "kills_rank" => Ok(self.kills_rank.clone().into()),
            "score_rank" => Ok(self.score_rank.clone().into()),
            "damage_rank" => Ok(self.damage_rank.clone().into()),
            "healing_rank" => Ok(self.healing_rank.clone().into()),
            "support_rank" => Ok(self.support_rank.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CompetitiveStatsUpdate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MiniGameWinEvent {
    pub team: u8,
    pub kind: u8,
}
impl MiniGameWinEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MiniGameWinEvent {
            team: read_value::<u8>(stream, definition.get_entry("team"), "team")?,
            kind: read_value::<u8>(stream, definition.get_entry("type"), "kind")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "team" => Ok(self.team.clone().into()),
            "type" => Ok(self.kind.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MiniGameWin",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SentryOnGoActiveEvent {
    pub index: u16,
}
impl SentryOnGoActiveEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SentryOnGoActiveEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SentryOnGoActive",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DuckXpLevelUpEvent {
    pub level: u16,
}
impl DuckXpLevelUpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DuckXpLevelUpEvent {
            level: read_value::<u16>(stream, definition.get_entry("level"), "level")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "level" => Ok(self.level.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DuckXpLevelUp",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestLogOpenedEvent {}
impl QuestLogOpenedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestLogOpenedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestLogOpened",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SchemaUpdatedEvent {}
impl SchemaUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SchemaUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SchemaUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct LocalPlayerPickupWeaponEvent {}
impl LocalPlayerPickupWeaponEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(LocalPlayerPickupWeaponEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "LocalPlayerPickupWeapon",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RdPlayerScorePointsEvent {
    pub player: u16,
    pub method: u16,
    pub amount: u16,
}
impl RdPlayerScorePointsEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RdPlayerScorePointsEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            method: read_value::<u16>(stream, definition.get_entry("method"), "method")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "method" => Ok(self.method.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RdPlayerScorePoints",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DemomanDetStickiesEvent {
    pub player: u16,
}
impl DemomanDetStickiesEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DemomanDetStickiesEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DemomanDetStickies",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestObjectiveCompletedEvent {
    pub quest_item_id_low: u32,
    pub quest_item_id_hi: u32,
    pub quest_objective_id: u32,
    pub scorer_user_id: u16,
}
impl QuestObjectiveCompletedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestObjectiveCompletedEvent {
            quest_item_id_low: read_value::<u32>(
                stream,
                definition.get_entry("quest_item_id_low"),
                "quest_item_id_low",
            )?,
            quest_item_id_hi: read_value::<u32>(
                stream,
                definition.get_entry("quest_item_id_hi"),
                "quest_item_id_hi",
            )?,
            quest_objective_id: read_value::<u32>(
                stream,
                definition.get_entry("quest_objective_id"),
                "quest_objective_id",
            )?,
            scorer_user_id: read_value::<u16>(
                stream,
                definition.get_entry("scorer_user_id"),
                "scorer_user_id",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "quest_item_id_low" => Ok(self.quest_item_id_low.clone().into()),
            "quest_item_id_hi" => Ok(self.quest_item_id_hi.clone().into()),
            "quest_objective_id" => Ok(self.quest_objective_id.clone().into()),
            "scorer_user_id" => Ok(self.scorer_user_id.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestObjectiveCompleted",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerScoreChangedEvent {
    pub player: u8,
    pub delta: u16,
}
impl PlayerScoreChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerScoreChangedEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            delta: read_value::<u16>(stream, definition.get_entry("delta"), "delta")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "delta" => Ok(self.delta.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerScoreChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct KilledCappingPlayerEvent {
    pub cp: u8,
    pub killer: u8,
    pub victim: u8,
    pub assister: u8,
}
impl KilledCappingPlayerEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(KilledCappingPlayerEvent {
            cp: read_value::<u8>(stream, definition.get_entry("cp"), "cp")?,
            killer: read_value::<u8>(stream, definition.get_entry("killer"), "killer")?,
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
            assister: read_value::<u8>(stream, definition.get_entry("assister"), "assister")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "cp" => Ok(self.cp.clone().into()),
            "killer" => Ok(self.killer.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "KilledCappingPlayer",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EnvironmentalDeathEvent {
    pub killer: u8,
    pub victim: u8,
}
impl EnvironmentalDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EnvironmentalDeathEvent {
            killer: read_value::<u8>(stream, definition.get_entry("killer"), "killer")?,
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "killer" => Ok(self.killer.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EnvironmentalDeath",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectileDirectHitEvent {
    pub attacker: u8,
    pub victim: u8,
    pub weapon_def_index: u32,
}
impl ProjectileDirectHitEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ProjectileDirectHitEvent {
            attacker: read_value::<u8>(stream, definition.get_entry("attacker"), "attacker")?,
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry("weapon_def_index"),
                "weapon_def_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attacker" => Ok(self.attacker.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            "weapon_def_index" => Ok(self.weapon_def_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ProjectileDirectHit",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassGetEvent {
    pub owner: u16,
}
impl PassGetEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassGetEvent {
            owner: read_value::<u16>(stream, definition.get_entry("owner"), "owner")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "owner" => Ok(self.owner.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassGet",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassScoreEvent {
    pub scorer: u16,
    pub assister: u16,
    pub points: u8,
}
impl PassScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassScoreEvent {
            scorer: read_value::<u16>(stream, definition.get_entry("scorer"), "scorer")?,
            assister: read_value::<u16>(stream, definition.get_entry("assister"), "assister")?,
            points: read_value::<u8>(stream, definition.get_entry("points"), "points")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "scorer" => Ok(self.scorer.clone().into()),
            "assister" => Ok(self.assister.clone().into()),
            "points" => Ok(self.points.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassScore",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassFreeEvent {
    pub owner: u16,
    pub attacker: u16,
}
impl PassFreeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassFreeEvent {
            owner: read_value::<u16>(stream, definition.get_entry("owner"), "owner")?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "owner" => Ok(self.owner.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassFree",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassPassCaughtEvent {
    pub passer: u16,
    pub catcher: u16,
    pub dist: f32,
    pub duration: f32,
}
impl PassPassCaughtEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassPassCaughtEvent {
            passer: read_value::<u16>(stream, definition.get_entry("passer"), "passer")?,
            catcher: read_value::<u16>(stream, definition.get_entry("catcher"), "catcher")?,
            dist: read_value::<f32>(stream, definition.get_entry("dist"), "dist")?,
            duration: read_value::<f32>(stream, definition.get_entry("duration"), "duration")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "passer" => Ok(self.passer.clone().into()),
            "catcher" => Ok(self.catcher.clone().into()),
            "dist" => Ok(self.dist.clone().into()),
            "duration" => Ok(self.duration.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassPassCaught",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassBallStolenEvent {
    pub victim: u16,
    pub attacker: u16,
}
impl PassBallStolenEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassBallStolenEvent {
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
            attacker: read_value::<u16>(stream, definition.get_entry("attacker"), "attacker")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "victim" => Ok(self.victim.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassBallStolen",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PassBallBlockedEvent {
    pub owner: u16,
    pub blocker: u16,
}
impl PassBallBlockedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PassBallBlockedEvent {
            owner: read_value::<u16>(stream, definition.get_entry("owner"), "owner")?,
            blocker: read_value::<u16>(stream, definition.get_entry("blocker"), "blocker")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "owner" => Ok(self.owner.clone().into()),
            "blocker" => Ok(self.blocker.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PassBallBlocked",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamagePreventedEvent {
    pub preventor: u16,
    pub victim: u16,
    pub amount: u16,
    pub condition: u16,
}
impl DamagePreventedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DamagePreventedEvent {
            preventor: read_value::<u16>(stream, definition.get_entry("preventor"), "preventor")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
            condition: read_value::<u16>(stream, definition.get_entry("condition"), "condition")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "preventor" => Ok(self.preventor.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            "condition" => Ok(self.condition.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DamagePrevented",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenBossKilledEvent {
    pub boss: u16,
    pub killer: u16,
}
impl HalloweenBossKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenBossKilledEvent {
            boss: read_value::<u16>(stream, definition.get_entry("boss"), "boss")?,
            killer: read_value::<u16>(stream, definition.get_entry("killer"), "killer")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "boss" => Ok(self.boss.clone().into()),
            "killer" => Ok(self.killer.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenBossKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscapedLootIslandEvent {
    pub player: u16,
}
impl EscapedLootIslandEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscapedLootIslandEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscapedLootIsland",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TaggedPlayerAsItEvent {
    pub player: u16,
}
impl TaggedPlayerAsItEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TaggedPlayerAsItEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TaggedPlayerAsIt",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusStunnedEvent {
    pub player: u16,
}
impl MerasmusStunnedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusStunnedEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusStunned",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MerasmusPropFoundEvent {
    pub player: u16,
}
impl MerasmusPropFoundEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MerasmusPropFoundEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MerasmusPropFound",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenSkeletonKilledEvent {
    pub player: u16,
}
impl HalloweenSkeletonKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenSkeletonKilledEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenSkeletonKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SkeletonKilledQuestEvent {
    pub player: u16,
}
impl SkeletonKilledQuestEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SkeletonKilledQuestEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SkeletonKilledQuest",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SkeletonKingKilledQuestEvent {
    pub player: u16,
}
impl SkeletonKingKilledQuestEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SkeletonKingKilledQuestEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SkeletonKingKilledQuest",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct EscapeHellEvent {
    pub player: u16,
}
impl EscapeHellEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(EscapeHellEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "EscapeHell",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CrossSpectralBridgeEvent {
    pub player: u16,
}
impl CrossSpectralBridgeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CrossSpectralBridgeEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CrossSpectralBridge",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MiniGameWonEvent {
    pub player: u16,
    pub game: u16,
}
impl MiniGameWonEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MiniGameWonEvent {
            player: read_value::<u16>(stream, definition.get_entry("player"), "player")?,
            game: read_value::<u16>(stream, definition.get_entry("game"), "game")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "game" => Ok(self.game.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MiniGameWon",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RespawnGhostEvent {
    pub reviver: u16,
    pub ghost: u16,
}
impl RespawnGhostEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RespawnGhostEvent {
            reviver: read_value::<u16>(stream, definition.get_entry("reviver"), "reviver")?,
            ghost: read_value::<u16>(stream, definition.get_entry("ghost"), "ghost")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "reviver" => Ok(self.reviver.clone().into()),
            "ghost" => Ok(self.ghost.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RespawnGhost",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct KillInHellEvent {
    pub killer: u16,
    pub victim: u16,
}
impl KillInHellEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(KillInHellEvent {
            killer: read_value::<u16>(stream, definition.get_entry("killer"), "killer")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "killer" => Ok(self.killer.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "KillInHell",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenDuckCollectedEvent {
    pub collector: u16,
}
impl HalloweenDuckCollectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenDuckCollectedEvent {
            collector: read_value::<u16>(stream, definition.get_entry("collector"), "collector")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "collector" => Ok(self.collector.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenDuckCollected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct SpecialScoreEvent {
    pub player: u8,
}
impl SpecialScoreEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(SpecialScoreEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "SpecialScore",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TeamLeaderKilledEvent {
    pub killer: u8,
    pub victim: u8,
}
impl TeamLeaderKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TeamLeaderKilledEvent {
            killer: read_value::<u8>(stream, definition.get_entry("killer"), "killer")?,
            victim: read_value::<u8>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "killer" => Ok(self.killer.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TeamLeaderKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HalloweenSoulCollectedEvent {
    pub intended_target: u8,
    pub collecting_player: u8,
    pub soul_count: u8,
}
impl HalloweenSoulCollectedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HalloweenSoulCollectedEvent {
            intended_target: read_value::<u8>(
                stream,
                definition.get_entry("intended_target"),
                "intended_target",
            )?,
            collecting_player: read_value::<u8>(
                stream,
                definition.get_entry("collecting_player"),
                "collecting_player",
            )?,
            soul_count: read_value::<u8>(stream, definition.get_entry("soul_count"), "soul_count")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "intended_target" => Ok(self.intended_target.clone().into()),
            "collecting_player" => Ok(self.collecting_player.clone().into()),
            "soul_count" => Ok(self.soul_count.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HalloweenSoulCollected",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RecalculateTruceEvent {}
impl RecalculateTruceEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RecalculateTruceEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RecalculateTruce",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DeadRingerCheatDeathEvent {
    pub spy: u8,
    pub attacker: u8,
}
impl DeadRingerCheatDeathEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DeadRingerCheatDeathEvent {
            spy: read_value::<u8>(stream, definition.get_entry("spy"), "spy")?,
            attacker: read_value::<u8>(stream, definition.get_entry("attacker"), "attacker")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "spy" => Ok(self.spy.clone().into()),
            "attacker" => Ok(self.attacker.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DeadRingerCheatDeath",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CrossbowHealEvent {
    pub healer: u8,
    pub target: u8,
    pub amount: u16,
}
impl CrossbowHealEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CrossbowHealEvent {
            healer: read_value::<u8>(stream, definition.get_entry("healer"), "healer")?,
            target: read_value::<u8>(stream, definition.get_entry("target"), "target")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "healer" => Ok(self.healer.clone().into()),
            "target" => Ok(self.target.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CrossbowHeal",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DamageMitigatedEvent {
    pub mitigator: u8,
    pub damaged: u8,
    pub amount: u16,
    pub item_definition_index: u16,
}
impl DamageMitigatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DamageMitigatedEvent {
            mitigator: read_value::<u8>(stream, definition.get_entry("mitigator"), "mitigator")?,
            damaged: read_value::<u8>(stream, definition.get_entry("damaged"), "damaged")?,
            amount: read_value::<u16>(stream, definition.get_entry("amount"), "amount")?,
            item_definition_index: read_value::<u16>(
                stream,
                definition.get_entry("itemdefindex"),
                "item_definition_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "mitigator" => Ok(self.mitigator.clone().into()),
            "damaged" => Ok(self.damaged.clone().into()),
            "amount" => Ok(self.amount.clone().into()),
            "itemdefindex" => Ok(self.item_definition_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DamageMitigated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PayloadPushedEvent {
    pub pusher: u8,
    pub distance: u16,
}
impl PayloadPushedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PayloadPushedEvent {
            pusher: read_value::<u8>(stream, definition.get_entry("pusher"), "pusher")?,
            distance: read_value::<u16>(stream, definition.get_entry("distance"), "distance")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "pusher" => Ok(self.pusher.clone().into()),
            "distance" => Ok(self.distance.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PayloadPushed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerAbandonedMatchEvent {
    pub game_over: bool,
}
impl PlayerAbandonedMatchEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerAbandonedMatchEvent {
            game_over: read_value::<bool>(stream, definition.get_entry("game_over"), "game_over")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "game_over" => Ok(self.game_over.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerAbandonedMatch",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ClDrawlineEvent {
            player: read_value::<u8>(stream, definition.get_entry("player"), "player")?,
            panel: read_value::<u8>(stream, definition.get_entry("panel"), "panel")?,
            line: read_value::<u8>(stream, definition.get_entry("line"), "line")?,
            x: read_value::<f32>(stream, definition.get_entry("x"), "x")?,
            y: read_value::<f32>(stream, definition.get_entry("y"), "y")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "player" => Ok(self.player.clone().into()),
            "panel" => Ok(self.panel.clone().into()),
            "line" => Ok(self.line.clone().into()),
            "x" => Ok(self.x.clone().into()),
            "y" => Ok(self.y.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ClDrawline",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RestartTimerTimeEvent {
    pub time: u8,
}
impl RestartTimerTimeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RestartTimerTimeEvent {
            time: read_value::<u8>(stream, definition.get_entry("time"), "time")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "time" => Ok(self.time.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RestartTimerTime",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WinLimitChangedEvent {}
impl WinLimitChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WinLimitChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "WinLimitChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WinPanelShowScoresEvent {}
impl WinPanelShowScoresEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WinPanelShowScoresEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "WinPanelShowScores",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct TopStreamsRequestFinishedEvent {}
impl TopStreamsRequestFinishedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(TopStreamsRequestFinishedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "TopStreamsRequestFinished",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CompetitiveStateChangedEvent {}
impl CompetitiveStateChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CompetitiveStateChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CompetitiveStateChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GlobalWarDataUpdatedEvent {}
impl GlobalWarDataUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GlobalWarDataUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GlobalWarDataUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct StopWatchChangedEvent {}
impl StopWatchChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(StopWatchChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "StopWatchChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DsStopEvent {}
impl DsStopEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DsStopEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DsStop",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct DsScreenshotEvent {
    pub delay: f32,
}
impl DsScreenshotEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(DsScreenshotEvent {
            delay: read_value::<f32>(stream, definition.get_entry("delay"), "delay")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "delay" => Ok(self.delay.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "DsScreenshot",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ShowMatchSummaryEvent {}
impl ShowMatchSummaryEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ShowMatchSummaryEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ShowMatchSummary",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ExperienceChangedEvent {}
impl ExperienceChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ExperienceChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ExperienceChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct BeginXpLerpEvent {}
impl BeginXpLerpEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(BeginXpLerpEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "BeginXpLerp",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MatchmakerStatsUpdatedEvent {}
impl MatchmakerStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MatchmakerStatsUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MatchmakerStatsUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RematchVotePeriodOverEvent {
    pub success: bool,
}
impl RematchVotePeriodOverEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RematchVotePeriodOverEvent {
            success: read_value::<bool>(stream, definition.get_entry("success"), "success")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "success" => Ok(self.success.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RematchVotePeriodOver",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct RematchFailedToCreateEvent {}
impl RematchFailedToCreateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(RematchFailedToCreateEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "RematchFailedToCreate",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRematchChangeEvent {}
impl PlayerRematchChangeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerRematchChangeEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerRematchChange",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PingUpdatedEvent {}
impl PingUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PingUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PingUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MMStatsUpdatedEvent {}
impl MMStatsUpdatedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MMStatsUpdatedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MMStatsUpdated",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerNextMapVoteChangeEvent {
    pub map_index: u8,
    pub vote: u8,
}
impl PlayerNextMapVoteChangeEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerNextMapVoteChangeEvent {
            map_index: read_value::<u8>(stream, definition.get_entry("map_index"), "map_index")?,
            vote: read_value::<u8>(stream, definition.get_entry("vote"), "vote")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "map_index" => Ok(self.map_index.clone().into()),
            "vote" => Ok(self.vote.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerNextMapVoteChange",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct VoteMapsChangedEvent {}
impl VoteMapsChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(VoteMapsChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "VoteMapsChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(ProtoDefChangedEvent {
            kind: read_value::<u8>(stream, definition.get_entry("type"), "kind")?,
            definition_index: read_value::<u32>(
                stream,
                definition.get_entry("defindex"),
                "definition_index",
            )?,
            created: read_value::<bool>(stream, definition.get_entry("created"), "created")?,
            deleted: read_value::<bool>(stream, definition.get_entry("deleted"), "deleted")?,
            erase_history: read_value::<bool>(
                stream,
                definition.get_entry("erase_history"),
                "erase_history",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "type" => Ok(self.kind.clone().into()),
            "defindex" => Ok(self.definition_index.clone().into()),
            "created" => Ok(self.created.clone().into()),
            "deleted" => Ok(self.deleted.clone().into()),
            "erase_history" => Ok(self.erase_history.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ProtoDefChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerDominationEvent {
    pub dominator: u16,
    pub dominated: u16,
    pub dominations: u16,
}
impl PlayerDominationEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerDominationEvent {
            dominator: read_value::<u16>(stream, definition.get_entry("dominator"), "dominator")?,
            dominated: read_value::<u16>(stream, definition.get_entry("dominated"), "dominated")?,
            dominations: read_value::<u16>(
                stream,
                definition.get_entry("dominations"),
                "dominations",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "dominator" => Ok(self.dominator.clone().into()),
            "dominated" => Ok(self.dominated.clone().into()),
            "dominations" => Ok(self.dominations.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerDomination",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct PlayerRocketPackPushedEvent {
    pub pusher: u16,
    pub pushed: u16,
}
impl PlayerRocketPackPushedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(PlayerRocketPackPushedEvent {
            pusher: read_value::<u16>(stream, definition.get_entry("pusher"), "pusher")?,
            pushed: read_value::<u16>(stream, definition.get_entry("pushed"), "pushed")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "pusher" => Ok(self.pusher.clone().into()),
            "pushed" => Ok(self.pushed.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "PlayerRocketPackPushed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestRequestEvent {
    pub request: u32,
    pub msg: MaybeUtf8String,
}
impl QuestRequestEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestRequestEvent {
            request: read_value::<u32>(stream, definition.get_entry("request"), "request")?,
            msg: read_value::<MaybeUtf8String>(stream, definition.get_entry("msg"), "msg")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "request" => Ok(self.request.clone().into()),
            "msg" => Ok(self.msg.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestRequest",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestResponseEvent {
    pub request: u32,
    pub success: bool,
    pub msg: MaybeUtf8String,
}
impl QuestResponseEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestResponseEvent {
            request: read_value::<u32>(stream, definition.get_entry("request"), "request")?,
            success: read_value::<bool>(stream, definition.get_entry("success"), "success")?,
            msg: read_value::<MaybeUtf8String>(stream, definition.get_entry("msg"), "msg")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "request" => Ok(self.request.clone().into()),
            "success" => Ok(self.success.clone().into()),
            "msg" => Ok(self.msg.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestResponse",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(QuestProgressEvent {
            owner: read_value::<u16>(stream, definition.get_entry("owner"), "owner")?,
            scorer: read_value::<u16>(stream, definition.get_entry("scorer"), "scorer")?,
            kind: read_value::<u8>(stream, definition.get_entry("type"), "kind")?,
            completed: read_value::<bool>(stream, definition.get_entry("completed"), "completed")?,
            quest_definition_index: read_value::<u32>(
                stream,
                definition.get_entry("quest_defindex"),
                "quest_definition_index",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "owner" => Ok(self.owner.clone().into()),
            "scorer" => Ok(self.scorer.clone().into()),
            "type" => Ok(self.kind.clone().into()),
            "completed" => Ok(self.completed.clone().into()),
            "quest_defindex" => Ok(self.quest_definition_index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestProgress",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ProjectileRemovedEvent {
    pub attacker: u8,
    pub weapon_def_index: u32,
    pub num_hit: u8,
    pub num_direct_hit: u8,
}
impl ProjectileRemovedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ProjectileRemovedEvent {
            attacker: read_value::<u8>(stream, definition.get_entry("attacker"), "attacker")?,
            weapon_def_index: read_value::<u32>(
                stream,
                definition.get_entry("weapon_def_index"),
                "weapon_def_index",
            )?,
            num_hit: read_value::<u8>(stream, definition.get_entry("num_hit"), "num_hit")?,
            num_direct_hit: read_value::<u8>(
                stream,
                definition.get_entry("num_direct_hit"),
                "num_direct_hit",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "attacker" => Ok(self.attacker.clone().into()),
            "weapon_def_index" => Ok(self.weapon_def_index.clone().into()),
            "num_hit" => Ok(self.num_hit.clone().into()),
            "num_direct_hit" => Ok(self.num_direct_hit.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ProjectileRemoved",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestMapDataChangedEvent {}
impl QuestMapDataChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestMapDataChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestMapDataChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct GasDousedPlayerIgnitedEvent {
    pub igniter: u16,
    pub douser: u16,
    pub victim: u16,
}
impl GasDousedPlayerIgnitedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(GasDousedPlayerIgnitedEvent {
            igniter: read_value::<u16>(stream, definition.get_entry("igniter"), "igniter")?,
            douser: read_value::<u16>(stream, definition.get_entry("douser"), "douser")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "igniter" => Ok(self.igniter.clone().into()),
            "douser" => Ok(self.douser.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "GasDousedPlayerIgnited",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct QuestTurnInStateEvent {
    pub state: u16,
}
impl QuestTurnInStateEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(QuestTurnInStateEvent {
            state: read_value::<u16>(stream, definition.get_entry("state"), "state")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "state" => Ok(self.state.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "QuestTurnInState",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ItemsAcknowledgedEvent {}
impl ItemsAcknowledgedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ItemsAcknowledgedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ItemsAcknowledged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CapperKilledEvent {
    pub blocker: u16,
    pub victim: u16,
}
impl CapperKilledEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(CapperKilledEvent {
            blocker: read_value::<u16>(stream, definition.get_entry("blocker"), "blocker")?,
            victim: read_value::<u16>(stream, definition.get_entry("victim"), "victim")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "blocker" => Ok(self.blocker.clone().into()),
            "victim" => Ok(self.victim.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "CapperKilled",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct MainMenuStabilizedEvent {}
impl MainMenuStabilizedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(MainMenuStabilizedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "MainMenuStabilized",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorldStatusChangedEvent {}
impl WorldStatusChangedEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(WorldStatusChangedEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "WorldStatusChanged",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVStatusEvent {
    pub clients: u32,
    pub slots: u32,
    pub proxies: u16,
    pub master: MaybeUtf8String,
}
impl HLTVStatusEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVStatusEvent {
            clients: read_value::<u32>(stream, definition.get_entry("clients"), "clients")?,
            slots: read_value::<u32>(stream, definition.get_entry("slots"), "slots")?,
            proxies: read_value::<u16>(stream, definition.get_entry("proxies"), "proxies")?,
            master: read_value::<MaybeUtf8String>(
                stream,
                definition.get_entry("master"),
                "master",
            )?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "clients" => Ok(self.clients.clone().into()),
            "slots" => Ok(self.slots.clone().into()),
            "proxies" => Ok(self.proxies.clone().into()),
            "master" => Ok(self.master.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVStatus",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVCameramanEvent {
    pub index: u16,
}
impl HLTVCameramanEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVCameramanEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVCameraman",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVRankCameraEvent {
    pub index: u8,
    pub rank: f32,
    pub target: u16,
}
impl HLTVRankCameraEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVRankCameraEvent {
            index: read_value::<u8>(stream, definition.get_entry("index"), "index")?,
            rank: read_value::<f32>(stream, definition.get_entry("rank"), "rank")?,
            target: read_value::<u16>(stream, definition.get_entry("target"), "target")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "rank" => Ok(self.rank.clone().into()),
            "target" => Ok(self.target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVRankCamera",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVRankEntityEvent {
    pub index: u16,
    pub rank: f32,
    pub target: u16,
}
impl HLTVRankEntityEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVRankEntityEvent {
            index: read_value::<u16>(stream, definition.get_entry("index"), "index")?,
            rank: read_value::<f32>(stream, definition.get_entry("rank"), "rank")?,
            target: read_value::<u16>(stream, definition.get_entry("target"), "target")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "index" => Ok(self.index.clone().into()),
            "rank" => Ok(self.rank.clone().into()),
            "target" => Ok(self.target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVRankEntity",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(HLTVFixedEvent {
            pos_x: read_value::<u32>(stream, definition.get_entry("posx"), "pos_x")?,
            pos_y: read_value::<u32>(stream, definition.get_entry("posy"), "pos_y")?,
            pos_z: read_value::<u32>(stream, definition.get_entry("posz"), "pos_z")?,
            theta: read_value::<u16>(stream, definition.get_entry("theta"), "theta")?,
            phi: read_value::<u16>(stream, definition.get_entry("phi"), "phi")?,
            offset: read_value::<u16>(stream, definition.get_entry("offset"), "offset")?,
            fov: read_value::<f32>(stream, definition.get_entry("fov"), "fov")?,
            target: read_value::<u16>(stream, definition.get_entry("target"), "target")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "posx" => Ok(self.pos_x.clone().into()),
            "posy" => Ok(self.pos_y.clone().into()),
            "posz" => Ok(self.pos_z.clone().into()),
            "theta" => Ok(self.theta.clone().into()),
            "phi" => Ok(self.phi.clone().into()),
            "offset" => Ok(self.offset.clone().into()),
            "fov" => Ok(self.fov.clone().into()),
            "target" => Ok(self.target.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVFixed",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
        Ok(HLTVChaseEvent {
            target_1: read_value::<u16>(stream, definition.get_entry("target1"), "target_1")?,
            target_2: read_value::<u16>(stream, definition.get_entry("target2"), "target_2")?,
            distance: read_value::<u16>(stream, definition.get_entry("distance"), "distance")?,
            theta: read_value::<u16>(stream, definition.get_entry("theta"), "theta")?,
            phi: read_value::<u16>(stream, definition.get_entry("phi"), "phi")?,
            inertia: read_value::<u8>(stream, definition.get_entry("inertia"), "inertia")?,
            in_eye: read_value::<u8>(stream, definition.get_entry("ineye"), "in_eye")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "target1" => Ok(self.target_1.clone().into()),
            "target2" => Ok(self.target_2.clone().into()),
            "distance" => Ok(self.distance.clone().into()),
            "theta" => Ok(self.theta.clone().into()),
            "phi" => Ok(self.phi.clone().into()),
            "inertia" => Ok(self.inertia.clone().into()),
            "ineye" => Ok(self.in_eye.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChase",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVMessageEvent {
    pub text: MaybeUtf8String,
}
impl HLTVMessageEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVMessageEvent {
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVMessage",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVTitleEvent {
    pub text: MaybeUtf8String,
}
impl HLTVTitleEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVTitleEvent {
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVTitle",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct HLTVChatEvent {
    pub text: MaybeUtf8String,
}
impl HLTVChatEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(HLTVChatEvent {
            text: read_value::<MaybeUtf8String>(stream, definition.get_entry("text"), "text")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "text" => Ok(self.text.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "HLTVChat",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayStartRecordEvent {}
impl ReplayStartRecordEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayStartRecordEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayStartRecord",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplaySessionInfoEvent {
    pub sn: MaybeUtf8String,
    pub di: u8,
    pub cb: u32,
    pub st: u32,
}
impl ReplaySessionInfoEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplaySessionInfoEvent {
            sn: read_value::<MaybeUtf8String>(stream, definition.get_entry("sn"), "sn")?,
            di: read_value::<u8>(stream, definition.get_entry("di"), "di")?,
            cb: read_value::<u32>(stream, definition.get_entry("cb"), "cb")?,
            st: read_value::<u32>(stream, definition.get_entry("st"), "st")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "sn" => Ok(self.sn.clone().into()),
            "di" => Ok(self.di.clone().into()),
            "cb" => Ok(self.cb.clone().into()),
            "st" => Ok(self.st.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplaySessionInfo",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayEndRecordEvent {}
impl ReplayEndRecordEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayEndRecordEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayEndRecord",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayReplaysAvailableEvent {}
impl ReplayReplaysAvailableEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayReplaysAvailableEvent {})
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayReplaysAvailable",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
    }
}
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ReplayServerErrorEvent {
    pub error: MaybeUtf8String,
}
impl ReplayServerErrorEvent {
    #[allow(unused_variables)]
    fn read(stream: &mut Stream, definition: &GameEventDefinition) -> Result<Self> {
        Ok(ReplayServerErrorEvent {
            error: read_value::<MaybeUtf8String>(stream, definition.get_entry("error"), "error")?,
        })
    }
    #[allow(unused_variables)]
    fn get_field(&self, field: &str) -> Result<GameEventValue> {
        #[allow(clippy::clone_on_copy, clippy::match_single_binding)]
        match field {
            "error" => Ok(self.error.clone().into()),
            _ => Err(ParseError::MissingGameEventValue {
                ty: "ReplayServerError",
                field: field.into(),
            }),
        }
    }
    #[allow(unused_variables)]
    fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        for entry in &definition.entries {
            let value = self
                .get_field(&entry.name)
                .unwrap_or_else(|_| entry.kind.default_value());
            stream.write(&value)?;
        }
        Ok(())
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
    SkeletonKilledQuest(SkeletonKilledQuestEvent),
    SkeletonKingKilledQuest(SkeletonKingKilledQuestEvent),
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
    #[serde(rename = "server_spawn")]
    ServerSpawn,
    #[serde(rename = "server_changelevel_failed")]
    ServerChangeLevelFailed,
    #[serde(rename = "server_shutdown")]
    ServerShutdown,
    #[serde(rename = "server_cvar")]
    ServerCvar,
    #[serde(rename = "server_message")]
    ServerMessage,
    #[serde(rename = "server_addban")]
    ServerAddBan,
    #[serde(rename = "server_removeban")]
    ServerRemoveBan,
    #[serde(rename = "player_connect")]
    PlayerConnect,
    #[serde(rename = "player_connect_client")]
    PlayerConnectClient,
    #[serde(rename = "player_info")]
    PlayerInfo,
    #[serde(rename = "player_disconnect")]
    PlayerDisconnect,
    #[serde(rename = "player_activate")]
    PlayerActivate,
    #[serde(rename = "player_say")]
    PlayerSay,
    #[serde(rename = "client_disconnect")]
    ClientDisconnect,
    #[serde(rename = "client_beginconnect")]
    ClientBeginConnect,
    #[serde(rename = "client_connected")]
    ClientConnected,
    #[serde(rename = "client_fullconnect")]
    ClientFullConnect,
    #[serde(rename = "host_quit")]
    HostQuit,
    #[serde(rename = "team_info")]
    TeamInfo,
    #[serde(rename = "team_score")]
    TeamScore,
    #[serde(rename = "teamplay_broadcast_audio")]
    TeamPlayBroadcastAudio,
    #[serde(rename = "player_team")]
    PlayerTeam,
    #[serde(rename = "player_class")]
    PlayerClass,
    #[serde(rename = "player_death")]
    PlayerDeath,
    #[serde(rename = "player_hurt")]
    PlayerHurt,
    #[serde(rename = "player_chat")]
    PlayerChat,
    #[serde(rename = "player_score")]
    PlayerScore,
    #[serde(rename = "player_spawn")]
    PlayerSpawn,
    #[serde(rename = "player_shoot")]
    PlayerShoot,
    #[serde(rename = "player_use")]
    PlayerUse,
    #[serde(rename = "player_changename")]
    PlayerChangeName,
    #[serde(rename = "player_hintmessage")]
    PlayerHintMessage,
    #[serde(rename = "base_player_teleported")]
    BasePlayerTeleported,
    #[serde(rename = "game_init")]
    GameInit,
    #[serde(rename = "game_newmap")]
    GameNewMap,
    #[serde(rename = "game_start")]
    GameStart,
    #[serde(rename = "game_end")]
    GameEnd,
    #[serde(rename = "round_start")]
    RoundStart,
    #[serde(rename = "round_end")]
    RoundEnd,
    #[serde(rename = "game_message")]
    GameMessage,
    #[serde(rename = "break_breakable")]
    BreakBreakable,
    #[serde(rename = "break_prop")]
    BreakProp,
    #[serde(rename = "entity_killed")]
    EntityKilled,
    #[serde(rename = "bonus_updated")]
    BonusUpdated,
    #[serde(rename = "achievement_event")]
    AchievementEvent,
    #[serde(rename = "achievement_increment")]
    AchievementIncrement,
    #[serde(rename = "physgun_pickup")]
    PhysgunPickup,
    #[serde(rename = "flare_ignite_npc")]
    FlareIgniteNpc,
    #[serde(rename = "helicopter_grenade_punt_miss")]
    HelicopterGrenadePuntMiss,
    #[serde(rename = "user_data_downloaded")]
    UserDataDownloaded,
    #[serde(rename = "ragdoll_dissolved")]
    RagdollDissolved,
    #[serde(rename = "hltv_changed_mode")]
    HLTVChangedMode,
    #[serde(rename = "hltv_changed_target")]
    HLTVChangedTarget,
    #[serde(rename = "vote_ended")]
    VoteEnded,
    #[serde(rename = "vote_started")]
    VoteStarted,
    #[serde(rename = "vote_changed")]
    VoteChanged,
    #[serde(rename = "vote_passed")]
    VotePassed,
    #[serde(rename = "vote_failed")]
    VoteFailed,
    #[serde(rename = "vote_cast")]
    VoteCast,
    #[serde(rename = "vote_options")]
    VoteOptions,
    #[serde(rename = "replay_saved")]
    ReplaySaved,
    #[serde(rename = "entered_performance_mode")]
    EnteredPerformanceMode,
    #[serde(rename = "browse_replays")]
    BrowseReplays,
    #[serde(rename = "replay_youtube_stats")]
    ReplayYoutubeStats,
    #[serde(rename = "inventory_updated")]
    InventoryUpdated,
    #[serde(rename = "cart_updated")]
    CartUpdated,
    #[serde(rename = "store_pricesheet_updated")]
    StorePriceSheetUpdated,
    #[serde(rename = "econ_inventory_connected")]
    EconInventoryConnected,
    #[serde(rename = "item_schema_initialized")]
    ItemSchemaInitialized,
    #[serde(rename = "gc_new_session")]
    GcNewSession,
    #[serde(rename = "gc_lost_session")]
    GcLostSession,
    #[serde(rename = "intro_finish")]
    IntroFinish,
    #[serde(rename = "intro_nextcamera")]
    IntroNextCamera,
    #[serde(rename = "player_changeclass")]
    PlayerChangeClass,
    #[serde(rename = "tf_map_time_remaining")]
    TfMapTimeRemaining,
    #[serde(rename = "tf_game_over")]
    TfGameOver,
    #[serde(rename = "ctf_flag_captured")]
    CtfFlagCaptured,
    #[serde(rename = "controlpoint_initialized")]
    ControlPointInitialized,
    #[serde(rename = "controlpoint_updateimages")]
    ControlPointUpdateImages,
    #[serde(rename = "controlpoint_updatelayout")]
    ControlPointUpdateLayout,
    #[serde(rename = "controlpoint_updatecapping")]
    ControlPointUpdateCapping,
    #[serde(rename = "controlpoint_updateowner")]
    ControlPointUpdateOwner,
    #[serde(rename = "controlpoint_starttouch")]
    ControlPointStartTouch,
    #[serde(rename = "controlpoint_endtouch")]
    ControlPointEndTouch,
    #[serde(rename = "controlpoint_pulse_element")]
    ControlPointPulseElement,
    #[serde(rename = "controlpoint_fake_capture")]
    ControlPointFakeCapture,
    #[serde(rename = "controlpoint_fake_capture_mult")]
    ControlPointFakeCaptureMultiplier,
    #[serde(rename = "teamplay_round_selected")]
    TeamPlayRoundSelected,
    #[serde(rename = "teamplay_round_start")]
    TeamPlayRoundStart,
    #[serde(rename = "teamplay_round_active")]
    TeamPlayRoundActive,
    #[serde(rename = "teamplay_waiting_begins")]
    TeamPlayWaitingBegins,
    #[serde(rename = "teamplay_waiting_ends")]
    TeamPlayWaitingEnds,
    #[serde(rename = "teamplay_waiting_abouttoend")]
    TeamPlayWaitingAboutToEnd,
    #[serde(rename = "teamplay_restart_round")]
    TeamPlayRestartRound,
    #[serde(rename = "teamplay_ready_restart")]
    TeamPlayReadyRestart,
    #[serde(rename = "teamplay_round_restart_seconds")]
    TeamPlayRoundRestartSeconds,
    #[serde(rename = "teamplay_team_ready")]
    TeamPlayTeamReady,
    #[serde(rename = "teamplay_round_win")]
    TeamPlayRoundWin,
    #[serde(rename = "teamplay_update_timer")]
    TeamPlayUpdateTimer,
    #[serde(rename = "teamplay_round_stalemate")]
    TeamPlayRoundStalemate,
    #[serde(rename = "teamplay_overtime_begin")]
    TeamPlayOvertimeBegin,
    #[serde(rename = "teamplay_overtime_end")]
    TeamPlayOvertimeEnd,
    #[serde(rename = "teamplay_suddendeath_begin")]
    TeamPlaySuddenDeathBegin,
    #[serde(rename = "teamplay_suddendeath_end")]
    TeamPlaySuddenDeathEnd,
    #[serde(rename = "teamplay_game_over")]
    TeamPlayGameOver,
    #[serde(rename = "teamplay_map_time_remaining")]
    TeamPlayMapTimeRemaining,
    #[serde(rename = "teamplay_timer_flash")]
    TeamPlayTimerFlash,
    #[serde(rename = "teamplay_timer_time_added")]
    TeamPlayTimerTimeAdded,
    #[serde(rename = "teamplay_point_startcapture")]
    TeamPlayPointStartCapture,
    #[serde(rename = "teamplay_point_captured")]
    TeamPlayPointCaptured,
    #[serde(rename = "teamplay_point_locked")]
    TeamPlayPointLocked,
    #[serde(rename = "teamplay_point_unlocked")]
    TeamPlayPointUnlocked,
    #[serde(rename = "teamplay_capture_broken")]
    TeamPlayCaptureBroken,
    #[serde(rename = "teamplay_capture_blocked")]
    TeamPlayCaptureBlocked,
    #[serde(rename = "teamplay_flag_event")]
    TeamPlayFlagEvent,
    #[serde(rename = "teamplay_win_panel")]
    TeamPlayWinPanel,
    #[serde(rename = "teamplay_teambalanced_player")]
    TeamPlayTeamBalancedPlayer,
    #[serde(rename = "teamplay_setup_finished")]
    TeamPlaySetupFinished,
    #[serde(rename = "teamplay_alert")]
    TeamPlayAlert,
    #[serde(rename = "training_complete")]
    TrainingComplete,
    #[serde(rename = "show_freezepanel")]
    ShowFreezePanel,
    #[serde(rename = "hide_freezepanel")]
    HideFreezePanel,
    #[serde(rename = "freezecam_started")]
    FreezeCamStarted,
    #[serde(rename = "localplayer_changeteam")]
    LocalPlayerChangeTeam,
    #[serde(rename = "localplayer_score_changed")]
    LocalPlayerScoreChanged,
    #[serde(rename = "localplayer_changeclass")]
    LocalPlayerChangeClass,
    #[serde(rename = "localplayer_respawn")]
    LocalPlayerRespawn,
    #[serde(rename = "building_info_changed")]
    BuildingInfoChanged,
    #[serde(rename = "localplayer_changedisguise")]
    LocalPlayerChangeDisguise,
    #[serde(rename = "player_account_changed")]
    PlayerAccountChanged,
    #[serde(rename = "spy_pda_reset")]
    SpyPdaReset,
    #[serde(rename = "flagstatus_update")]
    FlagStatusUpdate,
    #[serde(rename = "player_stats_updated")]
    PlayerStatsUpdated,
    #[serde(rename = "playing_commentary")]
    PlayingCommentary,
    #[serde(rename = "player_chargedeployed")]
    PlayerChargeDeployed,
    #[serde(rename = "player_builtobject")]
    PlayerBuiltObject,
    #[serde(rename = "player_upgradedobject")]
    PlayerUpgradedObject,
    #[serde(rename = "player_carryobject")]
    PlayerCarryObject,
    #[serde(rename = "player_dropobject")]
    PlayerDropObject,
    #[serde(rename = "object_removed")]
    ObjectRemoved,
    #[serde(rename = "object_destroyed")]
    ObjectDestroyed,
    #[serde(rename = "object_detonated")]
    ObjectDetonated,
    #[serde(rename = "achievement_earned")]
    AchievementEarned,
    #[serde(rename = "spec_target_updated")]
    SpecTargetUpdated,
    #[serde(rename = "tournament_stateupdate")]
    TournamentStateUpdate,
    #[serde(rename = "tournament_enablecountdown")]
    TournamentEnableCountdown,
    #[serde(rename = "player_calledformedic")]
    PlayerCalledForMedic,
    #[serde(rename = "player_askedforball")]
    PlayerAskedForBall,
    #[serde(rename = "localplayer_becameobserver")]
    LocalPlayerBecameObserver,
    #[serde(rename = "player_ignited_inv")]
    PlayerIgnitedInv,
    #[serde(rename = "player_ignited")]
    PlayerIgnited,
    #[serde(rename = "player_extinguished")]
    PlayerExtinguished,
    #[serde(rename = "player_teleported")]
    PlayerTeleported,
    #[serde(rename = "player_healedmediccall")]
    PlayerHealedMedicCall,
    #[serde(rename = "localplayer_chargeready")]
    LocalPlayerChargeReady,
    #[serde(rename = "localplayer_winddown")]
    LocalPlayerWindDown,
    #[serde(rename = "player_invulned")]
    PlayerInvulned,
    #[serde(rename = "escort_speed")]
    EscortSpeed,
    #[serde(rename = "escort_progress")]
    EscortProgress,
    #[serde(rename = "escort_recede")]
    EscortRecede,
    #[serde(rename = "gameui_activated")]
    GameUIActivated,
    #[serde(rename = "gameui_hidden")]
    GameUIHidden,
    #[serde(rename = "player_escort_score")]
    PlayerEscortScore,
    #[serde(rename = "player_healonhit")]
    PlayerHealOnHit,
    #[serde(rename = "player_stealsandvich")]
    PlayerStealSandvich,
    #[serde(rename = "show_class_layout")]
    ShowClassLayout,
    #[serde(rename = "show_vs_panel")]
    ShowVsPanel,
    #[serde(rename = "player_damaged")]
    PlayerDamaged,
    #[serde(rename = "arena_player_notification")]
    ArenaPlayerNotification,
    #[serde(rename = "arena_match_maxstreak")]
    ArenaMatchMaxStreak,
    #[serde(rename = "arena_round_start")]
    ArenaRoundStart,
    #[serde(rename = "arena_win_panel")]
    ArenaWinPanel,
    #[serde(rename = "pve_win_panel")]
    PveWinPanel,
    #[serde(rename = "air_dash")]
    AirDash,
    #[serde(rename = "landed")]
    Landed,
    #[serde(rename = "player_damage_dodged")]
    PlayerDamageDodged,
    #[serde(rename = "player_stunned")]
    PlayerStunned,
    #[serde(rename = "scout_grand_slam")]
    ScoutGrandSlam,
    #[serde(rename = "scout_slamdoll_landed")]
    ScoutSlamdollLanded,
    #[serde(rename = "arrow_impact")]
    ArrowImpact,
    #[serde(rename = "player_jarated")]
    PlayerJarated,
    #[serde(rename = "player_jarated_fade")]
    PlayerJaratedFade,
    #[serde(rename = "player_shield_blocked")]
    PlayerShieldBlocked,
    #[serde(rename = "player_pinned")]
    PlayerPinned,
    #[serde(rename = "player_healedbymedic")]
    PlayerHealedByMedic,
    #[serde(rename = "player_sapped_object")]
    PlayerSappedObject,
    #[serde(rename = "item_found")]
    ItemFound,
    #[serde(rename = "show_annotation")]
    ShowAnnotation,
    #[serde(rename = "hide_annotation")]
    HideAnnotation,
    #[serde(rename = "post_inventory_application")]
    PostInventoryApplication,
    #[serde(rename = "controlpoint_unlock_updated")]
    ControlPointUnlockUpdated,
    #[serde(rename = "deploy_buff_banner")]
    DeployBuffBanner,
    #[serde(rename = "player_buff")]
    PlayerBuff,
    #[serde(rename = "medic_death")]
    MedicDeath,
    #[serde(rename = "overtime_nag")]
    OvertimeNag,
    #[serde(rename = "teams_changed")]
    TeamsChanged,
    #[serde(rename = "halloween_pumpkin_grab")]
    HalloweenPumpkinGrab,
    #[serde(rename = "rocket_jump")]
    RocketJump,
    #[serde(rename = "rocket_jump_landed")]
    RocketJumpLanded,
    #[serde(rename = "sticky_jump")]
    StickyJump,
    #[serde(rename = "sticky_jump_landed")]
    StickyJumpLanded,
    #[serde(rename = "rocketpack_launch")]
    RocketPackLaunch,
    #[serde(rename = "rocketpack_landed")]
    RocketPackLanded,
    #[serde(rename = "medic_defended")]
    MedicDefended,
    #[serde(rename = "localplayer_healed")]
    LocalPlayerHealed,
    #[serde(rename = "player_destroyed_pipebomb")]
    PlayerDestroyedPipeBomb,
    #[serde(rename = "object_deflected")]
    ObjectDeflected,
    #[serde(rename = "player_mvp")]
    PlayerMvp,
    #[serde(rename = "raid_spawn_mob")]
    RaidSpawnMob,
    #[serde(rename = "raid_spawn_squad")]
    RaidSpawnSquad,
    #[serde(rename = "nav_blocked")]
    NavBlocked,
    #[serde(rename = "path_track_passed")]
    PathTrackPassed,
    #[serde(rename = "num_cappers_changed")]
    NumCappersChanged,
    #[serde(rename = "player_regenerate")]
    PlayerRegenerate,
    #[serde(rename = "update_status_item")]
    UpdateStatusItem,
    #[serde(rename = "stats_resetround")]
    StatsResetRound,
    #[serde(rename = "scorestats_accumulated_update")]
    ScoreStatsAccumulatedUpdate,
    #[serde(rename = "scorestats_accumulated_reset")]
    ScoreStatsAccumulatedReset,
    #[serde(rename = "achievement_earned_local")]
    AchievementEarnedLocal,
    #[serde(rename = "player_healed")]
    PlayerHealed,
    #[serde(rename = "building_healed")]
    BuildingHealed,
    #[serde(rename = "item_pickup")]
    ItemPickup,
    #[serde(rename = "duel_status")]
    DuelStatus,
    #[serde(rename = "fish_notice")]
    FishNotice,
    #[serde(rename = "fish_notice__arm")]
    FishNoticeArm,
    #[serde(rename = "slap_notice")]
    SlapNotice,
    #[serde(rename = "throwable_hit")]
    ThrowableHit,
    #[serde(rename = "pumpkin_lord_summoned")]
    PumpkinLordSummoned,
    #[serde(rename = "pumpkin_lord_killed")]
    PumpkinLordKilled,
    #[serde(rename = "merasmus_summoned")]
    MerasmusSummoned,
    #[serde(rename = "merasmus_killed")]
    MerasmusKilled,
    #[serde(rename = "merasmus_escape_warning")]
    MerasmusEscapeWarning,
    #[serde(rename = "merasmus_escaped")]
    MerasmusEscaped,
    #[serde(rename = "eyeball_boss_summoned")]
    EyeballBossSummoned,
    #[serde(rename = "eyeball_boss_stunned")]
    EyeballBossStunned,
    #[serde(rename = "eyeball_boss_killed")]
    EyeballBossKilled,
    #[serde(rename = "eyeball_boss_killer")]
    EyeballBossKiller,
    #[serde(rename = "eyeball_boss_escape_imminent")]
    EyeballBossEscapeImminent,
    #[serde(rename = "eyeball_boss_escaped")]
    EyeballBossEscaped,
    #[serde(rename = "npc_hurt")]
    NpcHurt,
    #[serde(rename = "controlpoint_timer_updated")]
    ControlPointTimerUpdated,
    #[serde(rename = "player_highfive_start")]
    PlayerHighFiveStart,
    #[serde(rename = "player_highfive_cancel")]
    PlayerHighFiveCancel,
    #[serde(rename = "player_highfive_success")]
    PlayerHighFiveSuccess,
    #[serde(rename = "player_bonuspoints")]
    PlayerBonusPoints,
    #[serde(rename = "player_upgraded")]
    PlayerUpgraded,
    #[serde(rename = "player_buyback")]
    PlayerBuyback,
    #[serde(rename = "player_used_powerup_bottle")]
    PlayerUsedPowerUpBottle,
    #[serde(rename = "christmas_gift_grab")]
    ChristmasGiftGrab,
    #[serde(rename = "player_killed_achievement_zone")]
    PlayerKilledAchievementZone,
    #[serde(rename = "party_updated")]
    PartyUpdated,
    #[serde(rename = "party_pref_changed")]
    PartyPrefChanged,
    #[serde(rename = "party_criteria_changed")]
    PartyCriteriaChanged,
    #[serde(rename = "party_invites_changed")]
    PartyInvitesChanged,
    #[serde(rename = "party_queue_state_changed")]
    PartyQueueStateChanged,
    #[serde(rename = "party_chat")]
    PartyChat,
    #[serde(rename = "party_member_join")]
    PartyMemberJoin,
    #[serde(rename = "party_member_leave")]
    PartyMemberLeave,
    #[serde(rename = "match_invites_updated")]
    MatchInvitesUpdated,
    #[serde(rename = "lobby_updated")]
    LobbyUpdated,
    #[serde(rename = "mvm_mission_update")]
    MvmMissionUpdate,
    #[serde(rename = "recalculate_holidays")]
    RecalculateHolidays,
    #[serde(rename = "player_currency_changed")]
    PlayerCurrencyChanged,
    #[serde(rename = "doomsday_rocket_open")]
    DoomsdayRocketOpen,
    #[serde(rename = "remove_nemesis_relationships")]
    RemoveNemesisRelationships,
    #[serde(rename = "mvm_creditbonus_wave")]
    MvmCreditBonusWave,
    #[serde(rename = "mvm_creditbonus_all")]
    MvmCreditBonusAll,
    #[serde(rename = "mvm_creditbonus_all_advanced")]
    MvmCreditBonusAllAdvanced,
    #[serde(rename = "mvm_quick_sentry_upgrade")]
    MvmQuickSentryUpgrade,
    #[serde(rename = "mvm_tank_destroyed_by_players")]
    MvmTankDestroyedByPlayers,
    #[serde(rename = "mvm_kill_robot_delivering_bomb")]
    MvmKillRobotDeliveringBomb,
    #[serde(rename = "mvm_pickup_currency")]
    MvmPickupCurrency,
    #[serde(rename = "mvm_bomb_carrier_killed")]
    MvmBombCarrierKilled,
    #[serde(rename = "mvm_sentrybuster_detonate")]
    MvmSentryBusterDetonate,
    #[serde(rename = "mvm_scout_marked_for_death")]
    MvmScoutMarkedForDeath,
    #[serde(rename = "mvm_medic_powerup_shared")]
    MvmMedicPowerUpShared,
    #[serde(rename = "mvm_begin_wave")]
    MvmBeginWave,
    #[serde(rename = "mvm_wave_complete")]
    MvmWaveComplete,
    #[serde(rename = "mvm_mission_complete")]
    MvmMissionComplete,
    #[serde(rename = "mvm_bomb_reset_by_player")]
    MvmBombResetByPlayer,
    #[serde(rename = "mvm_bomb_alarm_triggered")]
    MvmBombAlarmTriggered,
    #[serde(rename = "mvm_bomb_deploy_reset_by_player")]
    MvmBombDeployResetByPlayer,
    #[serde(rename = "mvm_wave_failed")]
    MvmWaveFailed,
    #[serde(rename = "mvm_reset_stats")]
    MvmResetStats,
    #[serde(rename = "damage_resisted")]
    DamageResisted,
    #[serde(rename = "revive_player_notify")]
    RevivePlayerNotify,
    #[serde(rename = "revive_player_stopped")]
    RevivePlayerStopped,
    #[serde(rename = "revive_player_complete")]
    RevivePlayerComplete,
    #[serde(rename = "player_turned_to_ghost")]
    PlayerTurnedToGhost,
    #[serde(rename = "medigun_shield_blocked_damage")]
    MedigunShieldBlockedDamage,
    #[serde(rename = "mvm_adv_wave_complete_no_gates")]
    MvmAdvWaveCompleteNoGates,
    #[serde(rename = "mvm_sniper_headshot_currency")]
    MvmSniperHeadshotCurrency,
    #[serde(rename = "mvm_mannhattan_pit")]
    MvmMannhattanPit,
    #[serde(rename = "flag_carried_in_detection_zone")]
    FlagCarriedInDetectionZone,
    #[serde(rename = "mvm_adv_wave_killed_stun_radio")]
    MvmAdvWaveKilledStunRadio,
    #[serde(rename = "player_directhit_stun")]
    PlayerDirectHitStun,
    #[serde(rename = "mvm_sentrybuster_killed")]
    MvmSentryBusterKilled,
    #[serde(rename = "upgrades_file_changed")]
    UpgradesFileChanged,
    #[serde(rename = "rd_team_points_changed")]
    RdTeamPointsChanged,
    #[serde(rename = "rd_rules_state_changed")]
    RdRulesStateChanged,
    #[serde(rename = "rd_robot_killed")]
    RdRobotKilled,
    #[serde(rename = "rd_robot_impact")]
    RdRobotImpact,
    #[serde(rename = "teamplay_pre_round_time_left")]
    TeamPlayPreRoundTimeLeft,
    #[serde(rename = "parachute_deploy")]
    ParachuteDeploy,
    #[serde(rename = "parachute_holster")]
    ParachuteHolster,
    #[serde(rename = "kill_refills_meter")]
    KillRefillsMeter,
    #[serde(rename = "rps_taunt_event")]
    RpsTauntEvent,
    #[serde(rename = "conga_kill")]
    CongaKill,
    #[serde(rename = "player_initial_spawn")]
    PlayerInitialSpawn,
    #[serde(rename = "competitive_victory")]
    CompetitiveVictory,
    #[serde(rename = "competitive_stats_update")]
    CompetitiveStatsUpdate,
    #[serde(rename = "minigame_win")]
    MiniGameWin,
    #[serde(rename = "sentry_on_go_active")]
    SentryOnGoActive,
    #[serde(rename = "duck_xp_level_up")]
    DuckXpLevelUp,
    #[serde(rename = "questlog_opened")]
    QuestLogOpened,
    #[serde(rename = "schema_updated")]
    SchemaUpdated,
    #[serde(rename = "localplayer_pickup_weapon")]
    LocalPlayerPickupWeapon,
    #[serde(rename = "rd_player_score_points")]
    RdPlayerScorePoints,
    #[serde(rename = "demoman_det_stickies")]
    DemomanDetStickies,
    #[serde(rename = "quest_objective_completed")]
    QuestObjectiveCompleted,
    #[serde(rename = "player_score_changed")]
    PlayerScoreChanged,
    #[serde(rename = "killed_capping_player")]
    KilledCappingPlayer,
    #[serde(rename = "environmental_death")]
    EnvironmentalDeath,
    #[serde(rename = "projectile_direct_hit")]
    ProjectileDirectHit,
    #[serde(rename = "pass_get")]
    PassGet,
    #[serde(rename = "pass_score")]
    PassScore,
    #[serde(rename = "pass_free")]
    PassFree,
    #[serde(rename = "pass_pass_caught")]
    PassPassCaught,
    #[serde(rename = "pass_ball_stolen")]
    PassBallStolen,
    #[serde(rename = "pass_ball_blocked")]
    PassBallBlocked,
    #[serde(rename = "damage_prevented")]
    DamagePrevented,
    #[serde(rename = "halloween_boss_killed")]
    HalloweenBossKilled,
    #[serde(rename = "escaped_loot_island")]
    EscapedLootIsland,
    #[serde(rename = "tagged_player_as_it")]
    TaggedPlayerAsIt,
    #[serde(rename = "merasmus_stunned")]
    MerasmusStunned,
    #[serde(rename = "merasmus_prop_found")]
    MerasmusPropFound,
    #[serde(rename = "halloween_skeleton_killed")]
    HalloweenSkeletonKilled,
    #[serde(rename = "skeleton_killed_quest")]
    SkeletonKilledQuest,
    #[serde(rename = "skeleton_king_killed_quest")]
    SkeletonKingKilledQuest,
    #[serde(rename = "escape_hell")]
    EscapeHell,
    #[serde(rename = "cross_spectral_bridge")]
    CrossSpectralBridge,
    #[serde(rename = "minigame_won")]
    MiniGameWon,
    #[serde(rename = "respawn_ghost")]
    RespawnGhost,
    #[serde(rename = "kill_in_hell")]
    KillInHell,
    #[serde(rename = "halloween_duck_collected")]
    HalloweenDuckCollected,
    #[serde(rename = "special_score")]
    SpecialScore,
    #[serde(rename = "team_leader_killed")]
    TeamLeaderKilled,
    #[serde(rename = "halloween_soul_collected")]
    HalloweenSoulCollected,
    #[serde(rename = "recalculate_truce")]
    RecalculateTruce,
    #[serde(rename = "deadringer_cheat_death")]
    DeadRingerCheatDeath,
    #[serde(rename = "crossbow_heal")]
    CrossbowHeal,
    #[serde(rename = "damage_mitigated")]
    DamageMitigated,
    #[serde(rename = "payload_pushed")]
    PayloadPushed,
    #[serde(rename = "player_abandoned_match")]
    PlayerAbandonedMatch,
    #[serde(rename = "cl_drawline")]
    ClDrawline,
    #[serde(rename = "restart_timer_time")]
    RestartTimerTime,
    #[serde(rename = "winlimit_changed")]
    WinLimitChanged,
    #[serde(rename = "winpanel_show_scores")]
    WinPanelShowScores,
    #[serde(rename = "top_streams_request_finished")]
    TopStreamsRequestFinished,
    #[serde(rename = "competitive_state_changed")]
    CompetitiveStateChanged,
    #[serde(rename = "global_war_data_updated")]
    GlobalWarDataUpdated,
    #[serde(rename = "stop_watch_changed")]
    StopWatchChanged,
    #[serde(rename = "ds_stop")]
    DsStop,
    #[serde(rename = "ds_screenshot")]
    DsScreenshot,
    #[serde(rename = "show_match_summary")]
    ShowMatchSummary,
    #[serde(rename = "experience_changed")]
    ExperienceChanged,
    #[serde(rename = "begin_xp_lerp")]
    BeginXpLerp,
    #[serde(rename = "matchmaker_stats_updated")]
    MatchmakerStatsUpdated,
    #[serde(rename = "rematch_vote_period_over")]
    RematchVotePeriodOver,
    #[serde(rename = "rematch_failed_to_create")]
    RematchFailedToCreate,
    #[serde(rename = "player_rematch_change")]
    PlayerRematchChange,
    #[serde(rename = "ping_updated")]
    PingUpdated,
    #[serde(rename = "mmstats_updated")]
    MMStatsUpdated,
    #[serde(rename = "player_next_map_vote_change")]
    PlayerNextMapVoteChange,
    #[serde(rename = "vote_maps_changed")]
    VoteMapsChanged,
    #[serde(rename = "proto_def_changed")]
    ProtoDefChanged,
    #[serde(rename = "player_domination")]
    PlayerDomination,
    #[serde(rename = "player_rocketpack_pushed")]
    PlayerRocketPackPushed,
    #[serde(rename = "quest_request")]
    QuestRequest,
    #[serde(rename = "quest_response")]
    QuestResponse,
    #[serde(rename = "quest_progress")]
    QuestProgress,
    #[serde(rename = "projectile_removed")]
    ProjectileRemoved,
    #[serde(rename = "quest_map_data_changed")]
    QuestMapDataChanged,
    #[serde(rename = "gas_doused_player_ignited")]
    GasDousedPlayerIgnited,
    #[serde(rename = "quest_turn_in_state")]
    QuestTurnInState,
    #[serde(rename = "items_acknowledged")]
    ItemsAcknowledged,
    #[serde(rename = "capper_killed")]
    CapperKilled,
    #[serde(rename = "mainmenu_stabilized")]
    MainMenuStabilized,
    #[serde(rename = "world_status_changed")]
    WorldStatusChanged,
    #[serde(rename = "hltv_status")]
    HLTVStatus,
    #[serde(rename = "hltv_cameraman")]
    HLTVCameraman,
    #[serde(rename = "hltv_rank_camera")]
    HLTVRankCamera,
    #[serde(rename = "hltv_rank_entity")]
    HLTVRankEntity,
    #[serde(rename = "hltv_fixed")]
    HLTVFixed,
    #[serde(rename = "hltv_chase")]
    HLTVChase,
    #[serde(rename = "hltv_message")]
    HLTVMessage,
    #[serde(rename = "hltv_title")]
    HLTVTitle,
    #[serde(rename = "hltv_chat")]
    HLTVChat,
    #[serde(rename = "replay_startrecord")]
    ReplayStartRecord,
    #[serde(rename = "replay_sessioninfo")]
    ReplaySessionInfo,
    #[serde(rename = "replay_endrecord")]
    ReplayEndRecord,
    #[serde(rename = "replay_replaysavailable")]
    ReplayReplaysAvailable,
    #[serde(rename = "replay_servererror")]
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
            "skeleton_killed_quest" => GameEventType::SkeletonKilledQuest,
            "skeleton_king_killed_quest" => GameEventType::SkeletonKingKilledQuest,
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
            GameEventType::ControlPointFakeCaptureMultiplier => "controlpoint_fake_capture_mult",
            GameEventType::TeamPlayRoundSelected => "teamplay_round_selected",
            GameEventType::TeamPlayRoundStart => "teamplay_round_start",
            GameEventType::TeamPlayRoundActive => "teamplay_round_active",
            GameEventType::TeamPlayWaitingBegins => "teamplay_waiting_begins",
            GameEventType::TeamPlayWaitingEnds => "teamplay_waiting_ends",
            GameEventType::TeamPlayWaitingAboutToEnd => "teamplay_waiting_abouttoend",
            GameEventType::TeamPlayRestartRound => "teamplay_restart_round",
            GameEventType::TeamPlayReadyRestart => "teamplay_ready_restart",
            GameEventType::TeamPlayRoundRestartSeconds => "teamplay_round_restart_seconds",
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
            GameEventType::PlayerKilledAchievementZone => "player_killed_achievement_zone",
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
            GameEventType::MvmBombDeployResetByPlayer => "mvm_bomb_deploy_reset_by_player",
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
            GameEventType::SkeletonKilledQuest => "skeleton_killed_quest",
            GameEventType::SkeletonKingKilledQuest => "skeleton_king_killed_quest",
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
        Ok(match definition.event_type {
            GameEventType::ServerSpawn => {
                GameEvent::ServerSpawn(Box::new(<ServerSpawnEvent>::read(stream, definition)?))
            }
            GameEventType::ServerChangeLevelFailed => GameEvent::ServerChangeLevelFailed(
                ServerChangeLevelFailedEvent::read(stream, definition)?,
            ),
            GameEventType::ServerShutdown => {
                GameEvent::ServerShutdown(ServerShutdownEvent::read(stream, definition)?)
            }
            GameEventType::ServerCvar => {
                GameEvent::ServerCvar(ServerCvarEvent::read(stream, definition)?)
            }
            GameEventType::ServerMessage => {
                GameEvent::ServerMessage(ServerMessageEvent::read(stream, definition)?)
            }
            GameEventType::ServerAddBan => {
                GameEvent::ServerAddBan(Box::new(<ServerAddBanEvent>::read(stream, definition)?))
            }
            GameEventType::ServerRemoveBan => {
                GameEvent::ServerRemoveBan(ServerRemoveBanEvent::read(stream, definition)?)
            }
            GameEventType::PlayerConnect => {
                GameEvent::PlayerConnect(PlayerConnectEvent::read(stream, definition)?)
            }
            GameEventType::PlayerConnectClient => {
                GameEvent::PlayerConnectClient(PlayerConnectClientEvent::read(stream, definition)?)
            }
            GameEventType::PlayerInfo => {
                GameEvent::PlayerInfo(PlayerInfoEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDisconnect => {
                GameEvent::PlayerDisconnect(PlayerDisconnectEvent::read(stream, definition)?)
            }
            GameEventType::PlayerActivate => {
                GameEvent::PlayerActivate(PlayerActivateEvent::read(stream, definition)?)
            }
            GameEventType::PlayerSay => {
                GameEvent::PlayerSay(PlayerSayEvent::read(stream, definition)?)
            }
            GameEventType::ClientDisconnect => {
                GameEvent::ClientDisconnect(ClientDisconnectEvent::read(stream, definition)?)
            }
            GameEventType::ClientBeginConnect => {
                GameEvent::ClientBeginConnect(ClientBeginConnectEvent::read(stream, definition)?)
            }
            GameEventType::ClientConnected => {
                GameEvent::ClientConnected(ClientConnectedEvent::read(stream, definition)?)
            }
            GameEventType::ClientFullConnect => {
                GameEvent::ClientFullConnect(ClientFullConnectEvent::read(stream, definition)?)
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
            GameEventType::TeamPlayBroadcastAudio => GameEvent::TeamPlayBroadcastAudio(
                TeamPlayBroadcastAudioEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerTeam => {
                GameEvent::PlayerTeam(PlayerTeamEvent::read(stream, definition)?)
            }
            GameEventType::PlayerClass => {
                GameEvent::PlayerClass(PlayerClassEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDeath => {
                GameEvent::PlayerDeath(Box::new(<PlayerDeathEvent>::read(stream, definition)?))
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
                GameEvent::PlayerChangeName(PlayerChangeNameEvent::read(stream, definition)?)
            }
            GameEventType::PlayerHintMessage => {
                GameEvent::PlayerHintMessage(PlayerHintMessageEvent::read(stream, definition)?)
            }
            GameEventType::BasePlayerTeleported => GameEvent::BasePlayerTeleported(
                BasePlayerTeleportedEvent::read(stream, definition)?,
            ),
            GameEventType::GameInit => {
                GameEvent::GameInit(GameInitEvent::read(stream, definition)?)
            }
            GameEventType::GameNewMap => {
                GameEvent::GameNewMap(GameNewMapEvent::read(stream, definition)?)
            }
            GameEventType::GameStart => {
                GameEvent::GameStart(GameStartEvent::read(stream, definition)?)
            }
            GameEventType::GameEnd => GameEvent::GameEnd(GameEndEvent::read(stream, definition)?),
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
                GameEvent::BreakBreakable(BreakBreakableEvent::read(stream, definition)?)
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
                GameEvent::AchievementEvent(AchievementEventEvent::read(stream, definition)?)
            }
            GameEventType::AchievementIncrement => GameEvent::AchievementIncrement(
                AchievementIncrementEvent::read(stream, definition)?,
            ),
            GameEventType::PhysgunPickup => {
                GameEvent::PhysgunPickup(PhysgunPickupEvent::read(stream, definition)?)
            }
            GameEventType::FlareIgniteNpc => {
                GameEvent::FlareIgniteNpc(FlareIgniteNpcEvent::read(stream, definition)?)
            }
            GameEventType::HelicopterGrenadePuntMiss => GameEvent::HelicopterGrenadePuntMiss(
                HelicopterGrenadePuntMissEvent::read(stream, definition)?,
            ),
            GameEventType::UserDataDownloaded => {
                GameEvent::UserDataDownloaded(UserDataDownloadedEvent::read(stream, definition)?)
            }
            GameEventType::RagdollDissolved => {
                GameEvent::RagdollDissolved(RagdollDissolvedEvent::read(stream, definition)?)
            }
            GameEventType::HLTVChangedMode => {
                GameEvent::HLTVChangedMode(HLTVChangedModeEvent::read(stream, definition)?)
            }
            GameEventType::HLTVChangedTarget => {
                GameEvent::HLTVChangedTarget(HLTVChangedTargetEvent::read(stream, definition)?)
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
                GameEvent::VoteOptions(Box::new(<VoteOptionsEvent>::read(stream, definition)?))
            }
            GameEventType::ReplaySaved => {
                GameEvent::ReplaySaved(ReplaySavedEvent::read(stream, definition)?)
            }
            GameEventType::EnteredPerformanceMode => GameEvent::EnteredPerformanceMode(
                EnteredPerformanceModeEvent::read(stream, definition)?,
            ),
            GameEventType::BrowseReplays => {
                GameEvent::BrowseReplays(BrowseReplaysEvent::read(stream, definition)?)
            }
            GameEventType::ReplayYoutubeStats => {
                GameEvent::ReplayYoutubeStats(ReplayYoutubeStatsEvent::read(stream, definition)?)
            }
            GameEventType::InventoryUpdated => {
                GameEvent::InventoryUpdated(InventoryUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::CartUpdated => {
                GameEvent::CartUpdated(CartUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::StorePriceSheetUpdated => GameEvent::StorePriceSheetUpdated(
                StorePriceSheetUpdatedEvent::read(stream, definition)?,
            ),
            GameEventType::EconInventoryConnected => GameEvent::EconInventoryConnected(
                EconInventoryConnectedEvent::read(stream, definition)?,
            ),
            GameEventType::ItemSchemaInitialized => GameEvent::ItemSchemaInitialized(
                ItemSchemaInitializedEvent::read(stream, definition)?,
            ),
            GameEventType::GcNewSession => {
                GameEvent::GcNewSession(GcNewSessionEvent::read(stream, definition)?)
            }
            GameEventType::GcLostSession => {
                GameEvent::GcLostSession(GcLostSessionEvent::read(stream, definition)?)
            }
            GameEventType::IntroFinish => {
                GameEvent::IntroFinish(IntroFinishEvent::read(stream, definition)?)
            }
            GameEventType::IntroNextCamera => {
                GameEvent::IntroNextCamera(IntroNextCameraEvent::read(stream, definition)?)
            }
            GameEventType::PlayerChangeClass => {
                GameEvent::PlayerChangeClass(PlayerChangeClassEvent::read(stream, definition)?)
            }
            GameEventType::TfMapTimeRemaining => {
                GameEvent::TfMapTimeRemaining(TfMapTimeRemainingEvent::read(stream, definition)?)
            }
            GameEventType::TfGameOver => {
                GameEvent::TfGameOver(TfGameOverEvent::read(stream, definition)?)
            }
            GameEventType::CtfFlagCaptured => {
                GameEvent::CtfFlagCaptured(CtfFlagCapturedEvent::read(stream, definition)?)
            }
            GameEventType::ControlPointInitialized => GameEvent::ControlPointInitialized(
                ControlPointInitializedEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointUpdateImages => GameEvent::ControlPointUpdateImages(
                ControlPointUpdateImagesEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointUpdateLayout => GameEvent::ControlPointUpdateLayout(
                ControlPointUpdateLayoutEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointUpdateCapping => GameEvent::ControlPointUpdateCapping(
                ControlPointUpdateCappingEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointUpdateOwner => GameEvent::ControlPointUpdateOwner(
                ControlPointUpdateOwnerEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointStartTouch => GameEvent::ControlPointStartTouch(
                ControlPointStartTouchEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointEndTouch => GameEvent::ControlPointEndTouch(
                ControlPointEndTouchEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointPulseElement => GameEvent::ControlPointPulseElement(
                ControlPointPulseElementEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointFakeCapture => GameEvent::ControlPointFakeCapture(
                ControlPointFakeCaptureEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointFakeCaptureMultiplier => {
                GameEvent::ControlPointFakeCaptureMultiplier(
                    ControlPointFakeCaptureMultiplierEvent::read(stream, definition)?,
                )
            }
            GameEventType::TeamPlayRoundSelected => GameEvent::TeamPlayRoundSelected(
                TeamPlayRoundSelectedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayRoundStart => {
                GameEvent::TeamPlayRoundStart(TeamPlayRoundStartEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayRoundActive => {
                GameEvent::TeamPlayRoundActive(TeamPlayRoundActiveEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayWaitingBegins => GameEvent::TeamPlayWaitingBegins(
                TeamPlayWaitingBeginsEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayWaitingEnds => {
                GameEvent::TeamPlayWaitingEnds(TeamPlayWaitingEndsEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayWaitingAboutToEnd => GameEvent::TeamPlayWaitingAboutToEnd(
                TeamPlayWaitingAboutToEndEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayRestartRound => GameEvent::TeamPlayRestartRound(
                TeamPlayRestartRoundEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayReadyRestart => GameEvent::TeamPlayReadyRestart(
                TeamPlayReadyRestartEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayRoundRestartSeconds => GameEvent::TeamPlayRoundRestartSeconds(
                TeamPlayRoundRestartSecondsEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayTeamReady => {
                GameEvent::TeamPlayTeamReady(TeamPlayTeamReadyEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayRoundWin => {
                GameEvent::TeamPlayRoundWin(TeamPlayRoundWinEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayUpdateTimer => {
                GameEvent::TeamPlayUpdateTimer(TeamPlayUpdateTimerEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayRoundStalemate => GameEvent::TeamPlayRoundStalemate(
                TeamPlayRoundStalemateEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayOvertimeBegin => GameEvent::TeamPlayOvertimeBegin(
                TeamPlayOvertimeBeginEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayOvertimeEnd => {
                GameEvent::TeamPlayOvertimeEnd(TeamPlayOvertimeEndEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlaySuddenDeathBegin => GameEvent::TeamPlaySuddenDeathBegin(
                TeamPlaySuddenDeathBeginEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlaySuddenDeathEnd => GameEvent::TeamPlaySuddenDeathEnd(
                TeamPlaySuddenDeathEndEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayGameOver => {
                GameEvent::TeamPlayGameOver(TeamPlayGameOverEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayMapTimeRemaining => GameEvent::TeamPlayMapTimeRemaining(
                TeamPlayMapTimeRemainingEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayTimerFlash => {
                GameEvent::TeamPlayTimerFlash(TeamPlayTimerFlashEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayTimerTimeAdded => GameEvent::TeamPlayTimerTimeAdded(
                TeamPlayTimerTimeAddedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayPointStartCapture => GameEvent::TeamPlayPointStartCapture(
                TeamPlayPointStartCaptureEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayPointCaptured => GameEvent::TeamPlayPointCaptured(
                TeamPlayPointCapturedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayPointLocked => {
                GameEvent::TeamPlayPointLocked(TeamPlayPointLockedEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayPointUnlocked => GameEvent::TeamPlayPointUnlocked(
                TeamPlayPointUnlockedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayCaptureBroken => GameEvent::TeamPlayCaptureBroken(
                TeamPlayCaptureBrokenEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayCaptureBlocked => GameEvent::TeamPlayCaptureBlocked(
                TeamPlayCaptureBlockedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayFlagEvent => {
                GameEvent::TeamPlayFlagEvent(TeamPlayFlagEventEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayWinPanel => {
                GameEvent::TeamPlayWinPanel(TeamPlayWinPanelEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayTeamBalancedPlayer => GameEvent::TeamPlayTeamBalancedPlayer(
                TeamPlayTeamBalancedPlayerEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlaySetupFinished => GameEvent::TeamPlaySetupFinished(
                TeamPlaySetupFinishedEvent::read(stream, definition)?,
            ),
            GameEventType::TeamPlayAlert => {
                GameEvent::TeamPlayAlert(TeamPlayAlertEvent::read(stream, definition)?)
            }
            GameEventType::TrainingComplete => {
                GameEvent::TrainingComplete(TrainingCompleteEvent::read(stream, definition)?)
            }
            GameEventType::ShowFreezePanel => {
                GameEvent::ShowFreezePanel(ShowFreezePanelEvent::read(stream, definition)?)
            }
            GameEventType::HideFreezePanel => {
                GameEvent::HideFreezePanel(HideFreezePanelEvent::read(stream, definition)?)
            }
            GameEventType::FreezeCamStarted => {
                GameEvent::FreezeCamStarted(FreezeCamStartedEvent::read(stream, definition)?)
            }
            GameEventType::LocalPlayerChangeTeam => GameEvent::LocalPlayerChangeTeam(
                LocalPlayerChangeTeamEvent::read(stream, definition)?,
            ),
            GameEventType::LocalPlayerScoreChanged => GameEvent::LocalPlayerScoreChanged(
                LocalPlayerScoreChangedEvent::read(stream, definition)?,
            ),
            GameEventType::LocalPlayerChangeClass => GameEvent::LocalPlayerChangeClass(
                LocalPlayerChangeClassEvent::read(stream, definition)?,
            ),
            GameEventType::LocalPlayerRespawn => {
                GameEvent::LocalPlayerRespawn(LocalPlayerRespawnEvent::read(stream, definition)?)
            }
            GameEventType::BuildingInfoChanged => {
                GameEvent::BuildingInfoChanged(BuildingInfoChangedEvent::read(stream, definition)?)
            }
            GameEventType::LocalPlayerChangeDisguise => GameEvent::LocalPlayerChangeDisguise(
                LocalPlayerChangeDisguiseEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerAccountChanged => GameEvent::PlayerAccountChanged(
                PlayerAccountChangedEvent::read(stream, definition)?,
            ),
            GameEventType::SpyPdaReset => {
                GameEvent::SpyPdaReset(SpyPdaResetEvent::read(stream, definition)?)
            }
            GameEventType::FlagStatusUpdate => {
                GameEvent::FlagStatusUpdate(FlagStatusUpdateEvent::read(stream, definition)?)
            }
            GameEventType::PlayerStatsUpdated => {
                GameEvent::PlayerStatsUpdated(PlayerStatsUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::PlayingCommentary => {
                GameEvent::PlayingCommentary(PlayingCommentaryEvent::read(stream, definition)?)
            }
            GameEventType::PlayerChargeDeployed => GameEvent::PlayerChargeDeployed(
                PlayerChargeDeployedEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerBuiltObject => {
                GameEvent::PlayerBuiltObject(PlayerBuiltObjectEvent::read(stream, definition)?)
            }
            GameEventType::PlayerUpgradedObject => GameEvent::PlayerUpgradedObject(
                PlayerUpgradedObjectEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerCarryObject => {
                GameEvent::PlayerCarryObject(PlayerCarryObjectEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDropObject => {
                GameEvent::PlayerDropObject(PlayerDropObjectEvent::read(stream, definition)?)
            }
            GameEventType::ObjectRemoved => {
                GameEvent::ObjectRemoved(ObjectRemovedEvent::read(stream, definition)?)
            }
            GameEventType::ObjectDestroyed => {
                GameEvent::ObjectDestroyed(ObjectDestroyedEvent::read(stream, definition)?)
            }
            GameEventType::ObjectDetonated => {
                GameEvent::ObjectDetonated(ObjectDetonatedEvent::read(stream, definition)?)
            }
            GameEventType::AchievementEarned => {
                GameEvent::AchievementEarned(AchievementEarnedEvent::read(stream, definition)?)
            }
            GameEventType::SpecTargetUpdated => {
                GameEvent::SpecTargetUpdated(SpecTargetUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::TournamentStateUpdate => GameEvent::TournamentStateUpdate(
                TournamentStateUpdateEvent::read(stream, definition)?,
            ),
            GameEventType::TournamentEnableCountdown => GameEvent::TournamentEnableCountdown(
                TournamentEnableCountdownEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerCalledForMedic => GameEvent::PlayerCalledForMedic(
                PlayerCalledForMedicEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerAskedForBall => {
                GameEvent::PlayerAskedForBall(PlayerAskedForBallEvent::read(stream, definition)?)
            }
            GameEventType::LocalPlayerBecameObserver => GameEvent::LocalPlayerBecameObserver(
                LocalPlayerBecameObserverEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerIgnitedInv => {
                GameEvent::PlayerIgnitedInv(PlayerIgnitedInvEvent::read(stream, definition)?)
            }
            GameEventType::PlayerIgnited => {
                GameEvent::PlayerIgnited(PlayerIgnitedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerExtinguished => {
                GameEvent::PlayerExtinguished(PlayerExtinguishedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerTeleported => {
                GameEvent::PlayerTeleported(PlayerTeleportedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerHealedMedicCall => GameEvent::PlayerHealedMedicCall(
                PlayerHealedMedicCallEvent::read(stream, definition)?,
            ),
            GameEventType::LocalPlayerChargeReady => GameEvent::LocalPlayerChargeReady(
                LocalPlayerChargeReadyEvent::read(stream, definition)?,
            ),
            GameEventType::LocalPlayerWindDown => {
                GameEvent::LocalPlayerWindDown(LocalPlayerWindDownEvent::read(stream, definition)?)
            }
            GameEventType::PlayerInvulned => {
                GameEvent::PlayerInvulned(PlayerInvulnedEvent::read(stream, definition)?)
            }
            GameEventType::EscortSpeed => {
                GameEvent::EscortSpeed(EscortSpeedEvent::read(stream, definition)?)
            }
            GameEventType::EscortProgress => {
                GameEvent::EscortProgress(EscortProgressEvent::read(stream, definition)?)
            }
            GameEventType::EscortRecede => {
                GameEvent::EscortRecede(EscortRecedeEvent::read(stream, definition)?)
            }
            GameEventType::GameUIActivated => {
                GameEvent::GameUIActivated(GameUIActivatedEvent::read(stream, definition)?)
            }
            GameEventType::GameUIHidden => {
                GameEvent::GameUIHidden(GameUIHiddenEvent::read(stream, definition)?)
            }
            GameEventType::PlayerEscortScore => {
                GameEvent::PlayerEscortScore(PlayerEscortScoreEvent::read(stream, definition)?)
            }
            GameEventType::PlayerHealOnHit => {
                GameEvent::PlayerHealOnHit(PlayerHealOnHitEvent::read(stream, definition)?)
            }
            GameEventType::PlayerStealSandvich => {
                GameEvent::PlayerStealSandvich(PlayerStealSandvichEvent::read(stream, definition)?)
            }
            GameEventType::ShowClassLayout => {
                GameEvent::ShowClassLayout(ShowClassLayoutEvent::read(stream, definition)?)
            }
            GameEventType::ShowVsPanel => {
                GameEvent::ShowVsPanel(ShowVsPanelEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDamaged => {
                GameEvent::PlayerDamaged(PlayerDamagedEvent::read(stream, definition)?)
            }
            GameEventType::ArenaPlayerNotification => GameEvent::ArenaPlayerNotification(
                ArenaPlayerNotificationEvent::read(stream, definition)?,
            ),
            GameEventType::ArenaMatchMaxStreak => {
                GameEvent::ArenaMatchMaxStreak(ArenaMatchMaxStreakEvent::read(stream, definition)?)
            }
            GameEventType::ArenaRoundStart => {
                GameEvent::ArenaRoundStart(ArenaRoundStartEvent::read(stream, definition)?)
            }
            GameEventType::ArenaWinPanel => {
                GameEvent::ArenaWinPanel(ArenaWinPanelEvent::read(stream, definition)?)
            }
            GameEventType::PveWinPanel => {
                GameEvent::PveWinPanel(PveWinPanelEvent::read(stream, definition)?)
            }
            GameEventType::AirDash => GameEvent::AirDash(AirDashEvent::read(stream, definition)?),
            GameEventType::Landed => GameEvent::Landed(LandedEvent::read(stream, definition)?),
            GameEventType::PlayerDamageDodged => {
                GameEvent::PlayerDamageDodged(PlayerDamageDodgedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerStunned => {
                GameEvent::PlayerStunned(PlayerStunnedEvent::read(stream, definition)?)
            }
            GameEventType::ScoutGrandSlam => {
                GameEvent::ScoutGrandSlam(ScoutGrandSlamEvent::read(stream, definition)?)
            }
            GameEventType::ScoutSlamdollLanded => {
                GameEvent::ScoutSlamdollLanded(ScoutSlamdollLandedEvent::read(stream, definition)?)
            }
            GameEventType::ArrowImpact => {
                GameEvent::ArrowImpact(ArrowImpactEvent::read(stream, definition)?)
            }
            GameEventType::PlayerJarated => {
                GameEvent::PlayerJarated(PlayerJaratedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerJaratedFade => {
                GameEvent::PlayerJaratedFade(PlayerJaratedFadeEvent::read(stream, definition)?)
            }
            GameEventType::PlayerShieldBlocked => {
                GameEvent::PlayerShieldBlocked(PlayerShieldBlockedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerPinned => {
                GameEvent::PlayerPinned(PlayerPinnedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerHealedByMedic => {
                GameEvent::PlayerHealedByMedic(PlayerHealedByMedicEvent::read(stream, definition)?)
            }
            GameEventType::PlayerSappedObject => {
                GameEvent::PlayerSappedObject(PlayerSappedObjectEvent::read(stream, definition)?)
            }
            GameEventType::ItemFound => {
                GameEvent::ItemFound(ItemFoundEvent::read(stream, definition)?)
            }
            GameEventType::ShowAnnotation => {
                GameEvent::ShowAnnotation(ShowAnnotationEvent::read(stream, definition)?)
            }
            GameEventType::HideAnnotation => {
                GameEvent::HideAnnotation(HideAnnotationEvent::read(stream, definition)?)
            }
            GameEventType::PostInventoryApplication => GameEvent::PostInventoryApplication(
                PostInventoryApplicationEvent::read(stream, definition)?,
            ),
            GameEventType::ControlPointUnlockUpdated => GameEvent::ControlPointUnlockUpdated(
                ControlPointUnlockUpdatedEvent::read(stream, definition)?,
            ),
            GameEventType::DeployBuffBanner => {
                GameEvent::DeployBuffBanner(DeployBuffBannerEvent::read(stream, definition)?)
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
            GameEventType::HalloweenPumpkinGrab => GameEvent::HalloweenPumpkinGrab(
                HalloweenPumpkinGrabEvent::read(stream, definition)?,
            ),
            GameEventType::RocketJump => {
                GameEvent::RocketJump(RocketJumpEvent::read(stream, definition)?)
            }
            GameEventType::RocketJumpLanded => {
                GameEvent::RocketJumpLanded(RocketJumpLandedEvent::read(stream, definition)?)
            }
            GameEventType::StickyJump => {
                GameEvent::StickyJump(StickyJumpEvent::read(stream, definition)?)
            }
            GameEventType::StickyJumpLanded => {
                GameEvent::StickyJumpLanded(StickyJumpLandedEvent::read(stream, definition)?)
            }
            GameEventType::RocketPackLaunch => {
                GameEvent::RocketPackLaunch(RocketPackLaunchEvent::read(stream, definition)?)
            }
            GameEventType::RocketPackLanded => {
                GameEvent::RocketPackLanded(RocketPackLandedEvent::read(stream, definition)?)
            }
            GameEventType::MedicDefended => {
                GameEvent::MedicDefended(MedicDefendedEvent::read(stream, definition)?)
            }
            GameEventType::LocalPlayerHealed => {
                GameEvent::LocalPlayerHealed(LocalPlayerHealedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDestroyedPipeBomb => GameEvent::PlayerDestroyedPipeBomb(
                PlayerDestroyedPipeBombEvent::read(stream, definition)?,
            ),
            GameEventType::ObjectDeflected => {
                GameEvent::ObjectDeflected(ObjectDeflectedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerMvp => {
                GameEvent::PlayerMvp(PlayerMvpEvent::read(stream, definition)?)
            }
            GameEventType::RaidSpawnMob => {
                GameEvent::RaidSpawnMob(RaidSpawnMobEvent::read(stream, definition)?)
            }
            GameEventType::RaidSpawnSquad => {
                GameEvent::RaidSpawnSquad(RaidSpawnSquadEvent::read(stream, definition)?)
            }
            GameEventType::NavBlocked => {
                GameEvent::NavBlocked(NavBlockedEvent::read(stream, definition)?)
            }
            GameEventType::PathTrackPassed => {
                GameEvent::PathTrackPassed(PathTrackPassedEvent::read(stream, definition)?)
            }
            GameEventType::NumCappersChanged => {
                GameEvent::NumCappersChanged(NumCappersChangedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerRegenerate => {
                GameEvent::PlayerRegenerate(PlayerRegenerateEvent::read(stream, definition)?)
            }
            GameEventType::UpdateStatusItem => {
                GameEvent::UpdateStatusItem(UpdateStatusItemEvent::read(stream, definition)?)
            }
            GameEventType::StatsResetRound => {
                GameEvent::StatsResetRound(StatsResetRoundEvent::read(stream, definition)?)
            }
            GameEventType::ScoreStatsAccumulatedUpdate => GameEvent::ScoreStatsAccumulatedUpdate(
                ScoreStatsAccumulatedUpdateEvent::read(stream, definition)?,
            ),
            GameEventType::ScoreStatsAccumulatedReset => GameEvent::ScoreStatsAccumulatedReset(
                ScoreStatsAccumulatedResetEvent::read(stream, definition)?,
            ),
            GameEventType::AchievementEarnedLocal => GameEvent::AchievementEarnedLocal(
                AchievementEarnedLocalEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerHealed => {
                GameEvent::PlayerHealed(PlayerHealedEvent::read(stream, definition)?)
            }
            GameEventType::BuildingHealed => {
                GameEvent::BuildingHealed(BuildingHealedEvent::read(stream, definition)?)
            }
            GameEventType::ItemPickup => {
                GameEvent::ItemPickup(ItemPickupEvent::read(stream, definition)?)
            }
            GameEventType::DuelStatus => {
                GameEvent::DuelStatus(DuelStatusEvent::read(stream, definition)?)
            }
            GameEventType::FishNotice => {
                GameEvent::FishNotice(Box::new(<FishNoticeEvent>::read(stream, definition)?))
            }
            GameEventType::FishNoticeArm => {
                GameEvent::FishNoticeArm(Box::new(<FishNoticeArmEvent>::read(stream, definition)?))
            }
            GameEventType::SlapNotice => {
                GameEvent::SlapNotice(Box::new(<SlapNoticeEvent>::read(stream, definition)?))
            }
            GameEventType::ThrowableHit => {
                GameEvent::ThrowableHit(Box::new(<ThrowableHitEvent>::read(stream, definition)?))
            }
            GameEventType::PumpkinLordSummoned => {
                GameEvent::PumpkinLordSummoned(PumpkinLordSummonedEvent::read(stream, definition)?)
            }
            GameEventType::PumpkinLordKilled => {
                GameEvent::PumpkinLordKilled(PumpkinLordKilledEvent::read(stream, definition)?)
            }
            GameEventType::MerasmusSummoned => {
                GameEvent::MerasmusSummoned(MerasmusSummonedEvent::read(stream, definition)?)
            }
            GameEventType::MerasmusKilled => {
                GameEvent::MerasmusKilled(MerasmusKilledEvent::read(stream, definition)?)
            }
            GameEventType::MerasmusEscapeWarning => GameEvent::MerasmusEscapeWarning(
                MerasmusEscapeWarningEvent::read(stream, definition)?,
            ),
            GameEventType::MerasmusEscaped => {
                GameEvent::MerasmusEscaped(MerasmusEscapedEvent::read(stream, definition)?)
            }
            GameEventType::EyeballBossSummoned => {
                GameEvent::EyeballBossSummoned(EyeballBossSummonedEvent::read(stream, definition)?)
            }
            GameEventType::EyeballBossStunned => {
                GameEvent::EyeballBossStunned(EyeballBossStunnedEvent::read(stream, definition)?)
            }
            GameEventType::EyeballBossKilled => {
                GameEvent::EyeballBossKilled(EyeballBossKilledEvent::read(stream, definition)?)
            }
            GameEventType::EyeballBossKiller => {
                GameEvent::EyeballBossKiller(EyeballBossKillerEvent::read(stream, definition)?)
            }
            GameEventType::EyeballBossEscapeImminent => GameEvent::EyeballBossEscapeImminent(
                EyeballBossEscapeImminentEvent::read(stream, definition)?,
            ),
            GameEventType::EyeballBossEscaped => {
                GameEvent::EyeballBossEscaped(EyeballBossEscapedEvent::read(stream, definition)?)
            }
            GameEventType::NpcHurt => GameEvent::NpcHurt(NpcHurtEvent::read(stream, definition)?),
            GameEventType::ControlPointTimerUpdated => GameEvent::ControlPointTimerUpdated(
                ControlPointTimerUpdatedEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerHighFiveStart => {
                GameEvent::PlayerHighFiveStart(PlayerHighFiveStartEvent::read(stream, definition)?)
            }
            GameEventType::PlayerHighFiveCancel => GameEvent::PlayerHighFiveCancel(
                PlayerHighFiveCancelEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerHighFiveSuccess => GameEvent::PlayerHighFiveSuccess(
                PlayerHighFiveSuccessEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerBonusPoints => {
                GameEvent::PlayerBonusPoints(PlayerBonusPointsEvent::read(stream, definition)?)
            }
            GameEventType::PlayerUpgraded => {
                GameEvent::PlayerUpgraded(PlayerUpgradedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerBuyback => {
                GameEvent::PlayerBuyback(PlayerBuybackEvent::read(stream, definition)?)
            }
            GameEventType::PlayerUsedPowerUpBottle => GameEvent::PlayerUsedPowerUpBottle(
                PlayerUsedPowerUpBottleEvent::read(stream, definition)?,
            ),
            GameEventType::ChristmasGiftGrab => {
                GameEvent::ChristmasGiftGrab(ChristmasGiftGrabEvent::read(stream, definition)?)
            }
            GameEventType::PlayerKilledAchievementZone => GameEvent::PlayerKilledAchievementZone(
                PlayerKilledAchievementZoneEvent::read(stream, definition)?,
            ),
            GameEventType::PartyUpdated => {
                GameEvent::PartyUpdated(PartyUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::PartyPrefChanged => {
                GameEvent::PartyPrefChanged(PartyPrefChangedEvent::read(stream, definition)?)
            }
            GameEventType::PartyCriteriaChanged => GameEvent::PartyCriteriaChanged(
                PartyCriteriaChangedEvent::read(stream, definition)?,
            ),
            GameEventType::PartyInvitesChanged => {
                GameEvent::PartyInvitesChanged(PartyInvitesChangedEvent::read(stream, definition)?)
            }
            GameEventType::PartyQueueStateChanged => GameEvent::PartyQueueStateChanged(
                PartyQueueStateChangedEvent::read(stream, definition)?,
            ),
            GameEventType::PartyChat => {
                GameEvent::PartyChat(PartyChatEvent::read(stream, definition)?)
            }
            GameEventType::PartyMemberJoin => {
                GameEvent::PartyMemberJoin(PartyMemberJoinEvent::read(stream, definition)?)
            }
            GameEventType::PartyMemberLeave => {
                GameEvent::PartyMemberLeave(PartyMemberLeaveEvent::read(stream, definition)?)
            }
            GameEventType::MatchInvitesUpdated => {
                GameEvent::MatchInvitesUpdated(MatchInvitesUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::LobbyUpdated => {
                GameEvent::LobbyUpdated(LobbyUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::MvmMissionUpdate => {
                GameEvent::MvmMissionUpdate(MvmMissionUpdateEvent::read(stream, definition)?)
            }
            GameEventType::RecalculateHolidays => {
                GameEvent::RecalculateHolidays(RecalculateHolidaysEvent::read(stream, definition)?)
            }
            GameEventType::PlayerCurrencyChanged => GameEvent::PlayerCurrencyChanged(
                PlayerCurrencyChangedEvent::read(stream, definition)?,
            ),
            GameEventType::DoomsdayRocketOpen => {
                GameEvent::DoomsdayRocketOpen(DoomsdayRocketOpenEvent::read(stream, definition)?)
            }
            GameEventType::RemoveNemesisRelationships => GameEvent::RemoveNemesisRelationships(
                RemoveNemesisRelationshipsEvent::read(stream, definition)?,
            ),
            GameEventType::MvmCreditBonusWave => {
                GameEvent::MvmCreditBonusWave(MvmCreditBonusWaveEvent::read(stream, definition)?)
            }
            GameEventType::MvmCreditBonusAll => {
                GameEvent::MvmCreditBonusAll(MvmCreditBonusAllEvent::read(stream, definition)?)
            }
            GameEventType::MvmCreditBonusAllAdvanced => GameEvent::MvmCreditBonusAllAdvanced(
                MvmCreditBonusAllAdvancedEvent::read(stream, definition)?,
            ),
            GameEventType::MvmQuickSentryUpgrade => GameEvent::MvmQuickSentryUpgrade(
                MvmQuickSentryUpgradeEvent::read(stream, definition)?,
            ),
            GameEventType::MvmTankDestroyedByPlayers => GameEvent::MvmTankDestroyedByPlayers(
                MvmTankDestroyedByPlayersEvent::read(stream, definition)?,
            ),
            GameEventType::MvmKillRobotDeliveringBomb => GameEvent::MvmKillRobotDeliveringBomb(
                MvmKillRobotDeliveringBombEvent::read(stream, definition)?,
            ),
            GameEventType::MvmPickupCurrency => {
                GameEvent::MvmPickupCurrency(MvmPickupCurrencyEvent::read(stream, definition)?)
            }
            GameEventType::MvmBombCarrierKilled => GameEvent::MvmBombCarrierKilled(
                MvmBombCarrierKilledEvent::read(stream, definition)?,
            ),
            GameEventType::MvmSentryBusterDetonate => GameEvent::MvmSentryBusterDetonate(
                MvmSentryBusterDetonateEvent::read(stream, definition)?,
            ),
            GameEventType::MvmScoutMarkedForDeath => GameEvent::MvmScoutMarkedForDeath(
                MvmScoutMarkedForDeathEvent::read(stream, definition)?,
            ),
            GameEventType::MvmMedicPowerUpShared => GameEvent::MvmMedicPowerUpShared(
                MvmMedicPowerUpSharedEvent::read(stream, definition)?,
            ),
            GameEventType::MvmBeginWave => {
                GameEvent::MvmBeginWave(MvmBeginWaveEvent::read(stream, definition)?)
            }
            GameEventType::MvmWaveComplete => {
                GameEvent::MvmWaveComplete(MvmWaveCompleteEvent::read(stream, definition)?)
            }
            GameEventType::MvmMissionComplete => {
                GameEvent::MvmMissionComplete(MvmMissionCompleteEvent::read(stream, definition)?)
            }
            GameEventType::MvmBombResetByPlayer => GameEvent::MvmBombResetByPlayer(
                MvmBombResetByPlayerEvent::read(stream, definition)?,
            ),
            GameEventType::MvmBombAlarmTriggered => GameEvent::MvmBombAlarmTriggered(
                MvmBombAlarmTriggeredEvent::read(stream, definition)?,
            ),
            GameEventType::MvmBombDeployResetByPlayer => GameEvent::MvmBombDeployResetByPlayer(
                MvmBombDeployResetByPlayerEvent::read(stream, definition)?,
            ),
            GameEventType::MvmWaveFailed => {
                GameEvent::MvmWaveFailed(MvmWaveFailedEvent::read(stream, definition)?)
            }
            GameEventType::MvmResetStats => {
                GameEvent::MvmResetStats(MvmResetStatsEvent::read(stream, definition)?)
            }
            GameEventType::DamageResisted => {
                GameEvent::DamageResisted(DamageResistedEvent::read(stream, definition)?)
            }
            GameEventType::RevivePlayerNotify => {
                GameEvent::RevivePlayerNotify(RevivePlayerNotifyEvent::read(stream, definition)?)
            }
            GameEventType::RevivePlayerStopped => {
                GameEvent::RevivePlayerStopped(RevivePlayerStoppedEvent::read(stream, definition)?)
            }
            GameEventType::RevivePlayerComplete => GameEvent::RevivePlayerComplete(
                RevivePlayerCompleteEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerTurnedToGhost => {
                GameEvent::PlayerTurnedToGhost(PlayerTurnedToGhostEvent::read(stream, definition)?)
            }
            GameEventType::MedigunShieldBlockedDamage => GameEvent::MedigunShieldBlockedDamage(
                MedigunShieldBlockedDamageEvent::read(stream, definition)?,
            ),
            GameEventType::MvmAdvWaveCompleteNoGates => GameEvent::MvmAdvWaveCompleteNoGates(
                MvmAdvWaveCompleteNoGatesEvent::read(stream, definition)?,
            ),
            GameEventType::MvmSniperHeadshotCurrency => GameEvent::MvmSniperHeadshotCurrency(
                MvmSniperHeadshotCurrencyEvent::read(stream, definition)?,
            ),
            GameEventType::MvmMannhattanPit => {
                GameEvent::MvmMannhattanPit(MvmMannhattanPitEvent::read(stream, definition)?)
            }
            GameEventType::FlagCarriedInDetectionZone => GameEvent::FlagCarriedInDetectionZone(
                FlagCarriedInDetectionZoneEvent::read(stream, definition)?,
            ),
            GameEventType::MvmAdvWaveKilledStunRadio => GameEvent::MvmAdvWaveKilledStunRadio(
                MvmAdvWaveKilledStunRadioEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerDirectHitStun => {
                GameEvent::PlayerDirectHitStun(PlayerDirectHitStunEvent::read(stream, definition)?)
            }
            GameEventType::MvmSentryBusterKilled => GameEvent::MvmSentryBusterKilled(
                MvmSentryBusterKilledEvent::read(stream, definition)?,
            ),
            GameEventType::UpgradesFileChanged => {
                GameEvent::UpgradesFileChanged(UpgradesFileChangedEvent::read(stream, definition)?)
            }
            GameEventType::RdTeamPointsChanged => {
                GameEvent::RdTeamPointsChanged(RdTeamPointsChangedEvent::read(stream, definition)?)
            }
            GameEventType::RdRulesStateChanged => {
                GameEvent::RdRulesStateChanged(RdRulesStateChangedEvent::read(stream, definition)?)
            }
            GameEventType::RdRobotKilled => {
                GameEvent::RdRobotKilled(RdRobotKilledEvent::read(stream, definition)?)
            }
            GameEventType::RdRobotImpact => {
                GameEvent::RdRobotImpact(RdRobotImpactEvent::read(stream, definition)?)
            }
            GameEventType::TeamPlayPreRoundTimeLeft => GameEvent::TeamPlayPreRoundTimeLeft(
                TeamPlayPreRoundTimeLeftEvent::read(stream, definition)?,
            ),
            GameEventType::ParachuteDeploy => {
                GameEvent::ParachuteDeploy(ParachuteDeployEvent::read(stream, definition)?)
            }
            GameEventType::ParachuteHolster => {
                GameEvent::ParachuteHolster(ParachuteHolsterEvent::read(stream, definition)?)
            }
            GameEventType::KillRefillsMeter => {
                GameEvent::KillRefillsMeter(KillRefillsMeterEvent::read(stream, definition)?)
            }
            GameEventType::RpsTauntEvent => {
                GameEvent::RpsTauntEvent(RpsTauntEventEvent::read(stream, definition)?)
            }
            GameEventType::CongaKill => {
                GameEvent::CongaKill(CongaKillEvent::read(stream, definition)?)
            }
            GameEventType::PlayerInitialSpawn => {
                GameEvent::PlayerInitialSpawn(PlayerInitialSpawnEvent::read(stream, definition)?)
            }
            GameEventType::CompetitiveVictory => {
                GameEvent::CompetitiveVictory(CompetitiveVictoryEvent::read(stream, definition)?)
            }
            GameEventType::CompetitiveStatsUpdate => GameEvent::CompetitiveStatsUpdate(
                CompetitiveStatsUpdateEvent::read(stream, definition)?,
            ),
            GameEventType::MiniGameWin => {
                GameEvent::MiniGameWin(MiniGameWinEvent::read(stream, definition)?)
            }
            GameEventType::SentryOnGoActive => {
                GameEvent::SentryOnGoActive(SentryOnGoActiveEvent::read(stream, definition)?)
            }
            GameEventType::DuckXpLevelUp => {
                GameEvent::DuckXpLevelUp(DuckXpLevelUpEvent::read(stream, definition)?)
            }
            GameEventType::QuestLogOpened => {
                GameEvent::QuestLogOpened(QuestLogOpenedEvent::read(stream, definition)?)
            }
            GameEventType::SchemaUpdated => {
                GameEvent::SchemaUpdated(SchemaUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::LocalPlayerPickupWeapon => GameEvent::LocalPlayerPickupWeapon(
                LocalPlayerPickupWeaponEvent::read(stream, definition)?,
            ),
            GameEventType::RdPlayerScorePoints => {
                GameEvent::RdPlayerScorePoints(RdPlayerScorePointsEvent::read(stream, definition)?)
            }
            GameEventType::DemomanDetStickies => {
                GameEvent::DemomanDetStickies(DemomanDetStickiesEvent::read(stream, definition)?)
            }
            GameEventType::QuestObjectiveCompleted => GameEvent::QuestObjectiveCompleted(
                QuestObjectiveCompletedEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerScoreChanged => {
                GameEvent::PlayerScoreChanged(PlayerScoreChangedEvent::read(stream, definition)?)
            }
            GameEventType::KilledCappingPlayer => {
                GameEvent::KilledCappingPlayer(KilledCappingPlayerEvent::read(stream, definition)?)
            }
            GameEventType::EnvironmentalDeath => {
                GameEvent::EnvironmentalDeath(EnvironmentalDeathEvent::read(stream, definition)?)
            }
            GameEventType::ProjectileDirectHit => {
                GameEvent::ProjectileDirectHit(ProjectileDirectHitEvent::read(stream, definition)?)
            }
            GameEventType::PassGet => GameEvent::PassGet(PassGetEvent::read(stream, definition)?),
            GameEventType::PassScore => {
                GameEvent::PassScore(PassScoreEvent::read(stream, definition)?)
            }
            GameEventType::PassFree => {
                GameEvent::PassFree(PassFreeEvent::read(stream, definition)?)
            }
            GameEventType::PassPassCaught => {
                GameEvent::PassPassCaught(PassPassCaughtEvent::read(stream, definition)?)
            }
            GameEventType::PassBallStolen => {
                GameEvent::PassBallStolen(PassBallStolenEvent::read(stream, definition)?)
            }
            GameEventType::PassBallBlocked => {
                GameEvent::PassBallBlocked(PassBallBlockedEvent::read(stream, definition)?)
            }
            GameEventType::DamagePrevented => {
                GameEvent::DamagePrevented(DamagePreventedEvent::read(stream, definition)?)
            }
            GameEventType::HalloweenBossKilled => {
                GameEvent::HalloweenBossKilled(HalloweenBossKilledEvent::read(stream, definition)?)
            }
            GameEventType::EscapedLootIsland => {
                GameEvent::EscapedLootIsland(EscapedLootIslandEvent::read(stream, definition)?)
            }
            GameEventType::TaggedPlayerAsIt => {
                GameEvent::TaggedPlayerAsIt(TaggedPlayerAsItEvent::read(stream, definition)?)
            }
            GameEventType::MerasmusStunned => {
                GameEvent::MerasmusStunned(MerasmusStunnedEvent::read(stream, definition)?)
            }
            GameEventType::MerasmusPropFound => {
                GameEvent::MerasmusPropFound(MerasmusPropFoundEvent::read(stream, definition)?)
            }
            GameEventType::HalloweenSkeletonKilled => GameEvent::HalloweenSkeletonKilled(
                HalloweenSkeletonKilledEvent::read(stream, definition)?,
            ),
            GameEventType::SkeletonKilledQuest => {
                GameEvent::SkeletonKilledQuest(SkeletonKilledQuestEvent::read(stream, definition)?)
            }
            GameEventType::SkeletonKingKilledQuest => GameEvent::SkeletonKingKilledQuest(
                SkeletonKingKilledQuestEvent::read(stream, definition)?,
            ),
            GameEventType::EscapeHell => {
                GameEvent::EscapeHell(EscapeHellEvent::read(stream, definition)?)
            }
            GameEventType::CrossSpectralBridge => {
                GameEvent::CrossSpectralBridge(CrossSpectralBridgeEvent::read(stream, definition)?)
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
            GameEventType::HalloweenDuckCollected => GameEvent::HalloweenDuckCollected(
                HalloweenDuckCollectedEvent::read(stream, definition)?,
            ),
            GameEventType::SpecialScore => {
                GameEvent::SpecialScore(SpecialScoreEvent::read(stream, definition)?)
            }
            GameEventType::TeamLeaderKilled => {
                GameEvent::TeamLeaderKilled(TeamLeaderKilledEvent::read(stream, definition)?)
            }
            GameEventType::HalloweenSoulCollected => GameEvent::HalloweenSoulCollected(
                HalloweenSoulCollectedEvent::read(stream, definition)?,
            ),
            GameEventType::RecalculateTruce => {
                GameEvent::RecalculateTruce(RecalculateTruceEvent::read(stream, definition)?)
            }
            GameEventType::DeadRingerCheatDeath => GameEvent::DeadRingerCheatDeath(
                DeadRingerCheatDeathEvent::read(stream, definition)?,
            ),
            GameEventType::CrossbowHeal => {
                GameEvent::CrossbowHeal(CrossbowHealEvent::read(stream, definition)?)
            }
            GameEventType::DamageMitigated => {
                GameEvent::DamageMitigated(DamageMitigatedEvent::read(stream, definition)?)
            }
            GameEventType::PayloadPushed => {
                GameEvent::PayloadPushed(PayloadPushedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerAbandonedMatch => GameEvent::PlayerAbandonedMatch(
                PlayerAbandonedMatchEvent::read(stream, definition)?,
            ),
            GameEventType::ClDrawline => {
                GameEvent::ClDrawline(ClDrawlineEvent::read(stream, definition)?)
            }
            GameEventType::RestartTimerTime => {
                GameEvent::RestartTimerTime(RestartTimerTimeEvent::read(stream, definition)?)
            }
            GameEventType::WinLimitChanged => {
                GameEvent::WinLimitChanged(WinLimitChangedEvent::read(stream, definition)?)
            }
            GameEventType::WinPanelShowScores => {
                GameEvent::WinPanelShowScores(WinPanelShowScoresEvent::read(stream, definition)?)
            }
            GameEventType::TopStreamsRequestFinished => GameEvent::TopStreamsRequestFinished(
                TopStreamsRequestFinishedEvent::read(stream, definition)?,
            ),
            GameEventType::CompetitiveStateChanged => GameEvent::CompetitiveStateChanged(
                CompetitiveStateChangedEvent::read(stream, definition)?,
            ),
            GameEventType::GlobalWarDataUpdated => GameEvent::GlobalWarDataUpdated(
                GlobalWarDataUpdatedEvent::read(stream, definition)?,
            ),
            GameEventType::StopWatchChanged => {
                GameEvent::StopWatchChanged(StopWatchChangedEvent::read(stream, definition)?)
            }
            GameEventType::DsStop => GameEvent::DsStop(DsStopEvent::read(stream, definition)?),
            GameEventType::DsScreenshot => {
                GameEvent::DsScreenshot(DsScreenshotEvent::read(stream, definition)?)
            }
            GameEventType::ShowMatchSummary => {
                GameEvent::ShowMatchSummary(ShowMatchSummaryEvent::read(stream, definition)?)
            }
            GameEventType::ExperienceChanged => {
                GameEvent::ExperienceChanged(ExperienceChangedEvent::read(stream, definition)?)
            }
            GameEventType::BeginXpLerp => {
                GameEvent::BeginXpLerp(BeginXpLerpEvent::read(stream, definition)?)
            }
            GameEventType::MatchmakerStatsUpdated => GameEvent::MatchmakerStatsUpdated(
                MatchmakerStatsUpdatedEvent::read(stream, definition)?,
            ),
            GameEventType::RematchVotePeriodOver => GameEvent::RematchVotePeriodOver(
                RematchVotePeriodOverEvent::read(stream, definition)?,
            ),
            GameEventType::RematchFailedToCreate => GameEvent::RematchFailedToCreate(
                RematchFailedToCreateEvent::read(stream, definition)?,
            ),
            GameEventType::PlayerRematchChange => {
                GameEvent::PlayerRematchChange(PlayerRematchChangeEvent::read(stream, definition)?)
            }
            GameEventType::PingUpdated => {
                GameEvent::PingUpdated(PingUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::MMStatsUpdated => {
                GameEvent::MMStatsUpdated(MMStatsUpdatedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerNextMapVoteChange => GameEvent::PlayerNextMapVoteChange(
                PlayerNextMapVoteChangeEvent::read(stream, definition)?,
            ),
            GameEventType::VoteMapsChanged => {
                GameEvent::VoteMapsChanged(VoteMapsChangedEvent::read(stream, definition)?)
            }
            GameEventType::ProtoDefChanged => {
                GameEvent::ProtoDefChanged(ProtoDefChangedEvent::read(stream, definition)?)
            }
            GameEventType::PlayerDomination => {
                GameEvent::PlayerDomination(PlayerDominationEvent::read(stream, definition)?)
            }
            GameEventType::PlayerRocketPackPushed => GameEvent::PlayerRocketPackPushed(
                PlayerRocketPackPushedEvent::read(stream, definition)?,
            ),
            GameEventType::QuestRequest => {
                GameEvent::QuestRequest(QuestRequestEvent::read(stream, definition)?)
            }
            GameEventType::QuestResponse => {
                GameEvent::QuestResponse(QuestResponseEvent::read(stream, definition)?)
            }
            GameEventType::QuestProgress => {
                GameEvent::QuestProgress(QuestProgressEvent::read(stream, definition)?)
            }
            GameEventType::ProjectileRemoved => {
                GameEvent::ProjectileRemoved(ProjectileRemovedEvent::read(stream, definition)?)
            }
            GameEventType::QuestMapDataChanged => {
                GameEvent::QuestMapDataChanged(QuestMapDataChangedEvent::read(stream, definition)?)
            }
            GameEventType::GasDousedPlayerIgnited => GameEvent::GasDousedPlayerIgnited(
                GasDousedPlayerIgnitedEvent::read(stream, definition)?,
            ),
            GameEventType::QuestTurnInState => {
                GameEvent::QuestTurnInState(QuestTurnInStateEvent::read(stream, definition)?)
            }
            GameEventType::ItemsAcknowledged => {
                GameEvent::ItemsAcknowledged(ItemsAcknowledgedEvent::read(stream, definition)?)
            }
            GameEventType::CapperKilled => {
                GameEvent::CapperKilled(CapperKilledEvent::read(stream, definition)?)
            }
            GameEventType::MainMenuStabilized => {
                GameEvent::MainMenuStabilized(MainMenuStabilizedEvent::read(stream, definition)?)
            }
            GameEventType::WorldStatusChanged => {
                GameEvent::WorldStatusChanged(WorldStatusChangedEvent::read(stream, definition)?)
            }
            GameEventType::HLTVStatus => {
                GameEvent::HLTVStatus(HLTVStatusEvent::read(stream, definition)?)
            }
            GameEventType::HLTVCameraman => {
                GameEvent::HLTVCameraman(HLTVCameramanEvent::read(stream, definition)?)
            }
            GameEventType::HLTVRankCamera => {
                GameEvent::HLTVRankCamera(HLTVRankCameraEvent::read(stream, definition)?)
            }
            GameEventType::HLTVRankEntity => {
                GameEvent::HLTVRankEntity(HLTVRankEntityEvent::read(stream, definition)?)
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
                GameEvent::ReplayStartRecord(ReplayStartRecordEvent::read(stream, definition)?)
            }
            GameEventType::ReplaySessionInfo => {
                GameEvent::ReplaySessionInfo(ReplaySessionInfoEvent::read(stream, definition)?)
            }
            GameEventType::ReplayEndRecord => {
                GameEvent::ReplayEndRecord(ReplayEndRecordEvent::read(stream, definition)?)
            }
            GameEventType::ReplayReplaysAvailable => GameEvent::ReplayReplaysAvailable(
                ReplayReplaysAvailableEvent::read(stream, definition)?,
            ),
            GameEventType::ReplayServerError => {
                GameEvent::ReplayServerError(ReplayServerErrorEvent::read(stream, definition)?)
            }
            GameEventType::Unknown(_) => {
                GameEvent::Unknown(RawGameEvent::read(stream, definition)?)
            }
        })
    }
    pub fn write(
        &self,
        stream: &mut BitWriteStream<LittleEndian>,
        definition: &GameEventDefinition,
    ) -> Result<()> {
        match &self {
            GameEvent::ServerSpawn(event) => event.write(stream, definition),
            GameEvent::ServerChangeLevelFailed(event) => event.write(stream, definition),
            GameEvent::ServerShutdown(event) => event.write(stream, definition),
            GameEvent::ServerCvar(event) => event.write(stream, definition),
            GameEvent::ServerMessage(event) => event.write(stream, definition),
            GameEvent::ServerAddBan(event) => event.write(stream, definition),
            GameEvent::ServerRemoveBan(event) => event.write(stream, definition),
            GameEvent::PlayerConnect(event) => event.write(stream, definition),
            GameEvent::PlayerConnectClient(event) => event.write(stream, definition),
            GameEvent::PlayerInfo(event) => event.write(stream, definition),
            GameEvent::PlayerDisconnect(event) => event.write(stream, definition),
            GameEvent::PlayerActivate(event) => event.write(stream, definition),
            GameEvent::PlayerSay(event) => event.write(stream, definition),
            GameEvent::ClientDisconnect(event) => event.write(stream, definition),
            GameEvent::ClientBeginConnect(event) => event.write(stream, definition),
            GameEvent::ClientConnected(event) => event.write(stream, definition),
            GameEvent::ClientFullConnect(event) => event.write(stream, definition),
            GameEvent::HostQuit(event) => event.write(stream, definition),
            GameEvent::TeamInfo(event) => event.write(stream, definition),
            GameEvent::TeamScore(event) => event.write(stream, definition),
            GameEvent::TeamPlayBroadcastAudio(event) => event.write(stream, definition),
            GameEvent::PlayerTeam(event) => event.write(stream, definition),
            GameEvent::PlayerClass(event) => event.write(stream, definition),
            GameEvent::PlayerDeath(event) => event.write(stream, definition),
            GameEvent::PlayerHurt(event) => event.write(stream, definition),
            GameEvent::PlayerChat(event) => event.write(stream, definition),
            GameEvent::PlayerScore(event) => event.write(stream, definition),
            GameEvent::PlayerSpawn(event) => event.write(stream, definition),
            GameEvent::PlayerShoot(event) => event.write(stream, definition),
            GameEvent::PlayerUse(event) => event.write(stream, definition),
            GameEvent::PlayerChangeName(event) => event.write(stream, definition),
            GameEvent::PlayerHintMessage(event) => event.write(stream, definition),
            GameEvent::BasePlayerTeleported(event) => event.write(stream, definition),
            GameEvent::GameInit(event) => event.write(stream, definition),
            GameEvent::GameNewMap(event) => event.write(stream, definition),
            GameEvent::GameStart(event) => event.write(stream, definition),
            GameEvent::GameEnd(event) => event.write(stream, definition),
            GameEvent::RoundStart(event) => event.write(stream, definition),
            GameEvent::RoundEnd(event) => event.write(stream, definition),
            GameEvent::GameMessage(event) => event.write(stream, definition),
            GameEvent::BreakBreakable(event) => event.write(stream, definition),
            GameEvent::BreakProp(event) => event.write(stream, definition),
            GameEvent::EntityKilled(event) => event.write(stream, definition),
            GameEvent::BonusUpdated(event) => event.write(stream, definition),
            GameEvent::AchievementEvent(event) => event.write(stream, definition),
            GameEvent::AchievementIncrement(event) => event.write(stream, definition),
            GameEvent::PhysgunPickup(event) => event.write(stream, definition),
            GameEvent::FlareIgniteNpc(event) => event.write(stream, definition),
            GameEvent::HelicopterGrenadePuntMiss(event) => event.write(stream, definition),
            GameEvent::UserDataDownloaded(event) => event.write(stream, definition),
            GameEvent::RagdollDissolved(event) => event.write(stream, definition),
            GameEvent::HLTVChangedMode(event) => event.write(stream, definition),
            GameEvent::HLTVChangedTarget(event) => event.write(stream, definition),
            GameEvent::VoteEnded(event) => event.write(stream, definition),
            GameEvent::VoteStarted(event) => event.write(stream, definition),
            GameEvent::VoteChanged(event) => event.write(stream, definition),
            GameEvent::VotePassed(event) => event.write(stream, definition),
            GameEvent::VoteFailed(event) => event.write(stream, definition),
            GameEvent::VoteCast(event) => event.write(stream, definition),
            GameEvent::VoteOptions(event) => event.write(stream, definition),
            GameEvent::ReplaySaved(event) => event.write(stream, definition),
            GameEvent::EnteredPerformanceMode(event) => event.write(stream, definition),
            GameEvent::BrowseReplays(event) => event.write(stream, definition),
            GameEvent::ReplayYoutubeStats(event) => event.write(stream, definition),
            GameEvent::InventoryUpdated(event) => event.write(stream, definition),
            GameEvent::CartUpdated(event) => event.write(stream, definition),
            GameEvent::StorePriceSheetUpdated(event) => event.write(stream, definition),
            GameEvent::EconInventoryConnected(event) => event.write(stream, definition),
            GameEvent::ItemSchemaInitialized(event) => event.write(stream, definition),
            GameEvent::GcNewSession(event) => event.write(stream, definition),
            GameEvent::GcLostSession(event) => event.write(stream, definition),
            GameEvent::IntroFinish(event) => event.write(stream, definition),
            GameEvent::IntroNextCamera(event) => event.write(stream, definition),
            GameEvent::PlayerChangeClass(event) => event.write(stream, definition),
            GameEvent::TfMapTimeRemaining(event) => event.write(stream, definition),
            GameEvent::TfGameOver(event) => event.write(stream, definition),
            GameEvent::CtfFlagCaptured(event) => event.write(stream, definition),
            GameEvent::ControlPointInitialized(event) => event.write(stream, definition),
            GameEvent::ControlPointUpdateImages(event) => event.write(stream, definition),
            GameEvent::ControlPointUpdateLayout(event) => event.write(stream, definition),
            GameEvent::ControlPointUpdateCapping(event) => event.write(stream, definition),
            GameEvent::ControlPointUpdateOwner(event) => event.write(stream, definition),
            GameEvent::ControlPointStartTouch(event) => event.write(stream, definition),
            GameEvent::ControlPointEndTouch(event) => event.write(stream, definition),
            GameEvent::ControlPointPulseElement(event) => event.write(stream, definition),
            GameEvent::ControlPointFakeCapture(event) => event.write(stream, definition),
            GameEvent::ControlPointFakeCaptureMultiplier(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundSelected(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundStart(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundActive(event) => event.write(stream, definition),
            GameEvent::TeamPlayWaitingBegins(event) => event.write(stream, definition),
            GameEvent::TeamPlayWaitingEnds(event) => event.write(stream, definition),
            GameEvent::TeamPlayWaitingAboutToEnd(event) => event.write(stream, definition),
            GameEvent::TeamPlayRestartRound(event) => event.write(stream, definition),
            GameEvent::TeamPlayReadyRestart(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundRestartSeconds(event) => event.write(stream, definition),
            GameEvent::TeamPlayTeamReady(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundWin(event) => event.write(stream, definition),
            GameEvent::TeamPlayUpdateTimer(event) => event.write(stream, definition),
            GameEvent::TeamPlayRoundStalemate(event) => event.write(stream, definition),
            GameEvent::TeamPlayOvertimeBegin(event) => event.write(stream, definition),
            GameEvent::TeamPlayOvertimeEnd(event) => event.write(stream, definition),
            GameEvent::TeamPlaySuddenDeathBegin(event) => event.write(stream, definition),
            GameEvent::TeamPlaySuddenDeathEnd(event) => event.write(stream, definition),
            GameEvent::TeamPlayGameOver(event) => event.write(stream, definition),
            GameEvent::TeamPlayMapTimeRemaining(event) => event.write(stream, definition),
            GameEvent::TeamPlayTimerFlash(event) => event.write(stream, definition),
            GameEvent::TeamPlayTimerTimeAdded(event) => event.write(stream, definition),
            GameEvent::TeamPlayPointStartCapture(event) => event.write(stream, definition),
            GameEvent::TeamPlayPointCaptured(event) => event.write(stream, definition),
            GameEvent::TeamPlayPointLocked(event) => event.write(stream, definition),
            GameEvent::TeamPlayPointUnlocked(event) => event.write(stream, definition),
            GameEvent::TeamPlayCaptureBroken(event) => event.write(stream, definition),
            GameEvent::TeamPlayCaptureBlocked(event) => event.write(stream, definition),
            GameEvent::TeamPlayFlagEvent(event) => event.write(stream, definition),
            GameEvent::TeamPlayWinPanel(event) => event.write(stream, definition),
            GameEvent::TeamPlayTeamBalancedPlayer(event) => event.write(stream, definition),
            GameEvent::TeamPlaySetupFinished(event) => event.write(stream, definition),
            GameEvent::TeamPlayAlert(event) => event.write(stream, definition),
            GameEvent::TrainingComplete(event) => event.write(stream, definition),
            GameEvent::ShowFreezePanel(event) => event.write(stream, definition),
            GameEvent::HideFreezePanel(event) => event.write(stream, definition),
            GameEvent::FreezeCamStarted(event) => event.write(stream, definition),
            GameEvent::LocalPlayerChangeTeam(event) => event.write(stream, definition),
            GameEvent::LocalPlayerScoreChanged(event) => event.write(stream, definition),
            GameEvent::LocalPlayerChangeClass(event) => event.write(stream, definition),
            GameEvent::LocalPlayerRespawn(event) => event.write(stream, definition),
            GameEvent::BuildingInfoChanged(event) => event.write(stream, definition),
            GameEvent::LocalPlayerChangeDisguise(event) => event.write(stream, definition),
            GameEvent::PlayerAccountChanged(event) => event.write(stream, definition),
            GameEvent::SpyPdaReset(event) => event.write(stream, definition),
            GameEvent::FlagStatusUpdate(event) => event.write(stream, definition),
            GameEvent::PlayerStatsUpdated(event) => event.write(stream, definition),
            GameEvent::PlayingCommentary(event) => event.write(stream, definition),
            GameEvent::PlayerChargeDeployed(event) => event.write(stream, definition),
            GameEvent::PlayerBuiltObject(event) => event.write(stream, definition),
            GameEvent::PlayerUpgradedObject(event) => event.write(stream, definition),
            GameEvent::PlayerCarryObject(event) => event.write(stream, definition),
            GameEvent::PlayerDropObject(event) => event.write(stream, definition),
            GameEvent::ObjectRemoved(event) => event.write(stream, definition),
            GameEvent::ObjectDestroyed(event) => event.write(stream, definition),
            GameEvent::ObjectDetonated(event) => event.write(stream, definition),
            GameEvent::AchievementEarned(event) => event.write(stream, definition),
            GameEvent::SpecTargetUpdated(event) => event.write(stream, definition),
            GameEvent::TournamentStateUpdate(event) => event.write(stream, definition),
            GameEvent::TournamentEnableCountdown(event) => event.write(stream, definition),
            GameEvent::PlayerCalledForMedic(event) => event.write(stream, definition),
            GameEvent::PlayerAskedForBall(event) => event.write(stream, definition),
            GameEvent::LocalPlayerBecameObserver(event) => event.write(stream, definition),
            GameEvent::PlayerIgnitedInv(event) => event.write(stream, definition),
            GameEvent::PlayerIgnited(event) => event.write(stream, definition),
            GameEvent::PlayerExtinguished(event) => event.write(stream, definition),
            GameEvent::PlayerTeleported(event) => event.write(stream, definition),
            GameEvent::PlayerHealedMedicCall(event) => event.write(stream, definition),
            GameEvent::LocalPlayerChargeReady(event) => event.write(stream, definition),
            GameEvent::LocalPlayerWindDown(event) => event.write(stream, definition),
            GameEvent::PlayerInvulned(event) => event.write(stream, definition),
            GameEvent::EscortSpeed(event) => event.write(stream, definition),
            GameEvent::EscortProgress(event) => event.write(stream, definition),
            GameEvent::EscortRecede(event) => event.write(stream, definition),
            GameEvent::GameUIActivated(event) => event.write(stream, definition),
            GameEvent::GameUIHidden(event) => event.write(stream, definition),
            GameEvent::PlayerEscortScore(event) => event.write(stream, definition),
            GameEvent::PlayerHealOnHit(event) => event.write(stream, definition),
            GameEvent::PlayerStealSandvich(event) => event.write(stream, definition),
            GameEvent::ShowClassLayout(event) => event.write(stream, definition),
            GameEvent::ShowVsPanel(event) => event.write(stream, definition),
            GameEvent::PlayerDamaged(event) => event.write(stream, definition),
            GameEvent::ArenaPlayerNotification(event) => event.write(stream, definition),
            GameEvent::ArenaMatchMaxStreak(event) => event.write(stream, definition),
            GameEvent::ArenaRoundStart(event) => event.write(stream, definition),
            GameEvent::ArenaWinPanel(event) => event.write(stream, definition),
            GameEvent::PveWinPanel(event) => event.write(stream, definition),
            GameEvent::AirDash(event) => event.write(stream, definition),
            GameEvent::Landed(event) => event.write(stream, definition),
            GameEvent::PlayerDamageDodged(event) => event.write(stream, definition),
            GameEvent::PlayerStunned(event) => event.write(stream, definition),
            GameEvent::ScoutGrandSlam(event) => event.write(stream, definition),
            GameEvent::ScoutSlamdollLanded(event) => event.write(stream, definition),
            GameEvent::ArrowImpact(event) => event.write(stream, definition),
            GameEvent::PlayerJarated(event) => event.write(stream, definition),
            GameEvent::PlayerJaratedFade(event) => event.write(stream, definition),
            GameEvent::PlayerShieldBlocked(event) => event.write(stream, definition),
            GameEvent::PlayerPinned(event) => event.write(stream, definition),
            GameEvent::PlayerHealedByMedic(event) => event.write(stream, definition),
            GameEvent::PlayerSappedObject(event) => event.write(stream, definition),
            GameEvent::ItemFound(event) => event.write(stream, definition),
            GameEvent::ShowAnnotation(event) => event.write(stream, definition),
            GameEvent::HideAnnotation(event) => event.write(stream, definition),
            GameEvent::PostInventoryApplication(event) => event.write(stream, definition),
            GameEvent::ControlPointUnlockUpdated(event) => event.write(stream, definition),
            GameEvent::DeployBuffBanner(event) => event.write(stream, definition),
            GameEvent::PlayerBuff(event) => event.write(stream, definition),
            GameEvent::MedicDeath(event) => event.write(stream, definition),
            GameEvent::OvertimeNag(event) => event.write(stream, definition),
            GameEvent::TeamsChanged(event) => event.write(stream, definition),
            GameEvent::HalloweenPumpkinGrab(event) => event.write(stream, definition),
            GameEvent::RocketJump(event) => event.write(stream, definition),
            GameEvent::RocketJumpLanded(event) => event.write(stream, definition),
            GameEvent::StickyJump(event) => event.write(stream, definition),
            GameEvent::StickyJumpLanded(event) => event.write(stream, definition),
            GameEvent::RocketPackLaunch(event) => event.write(stream, definition),
            GameEvent::RocketPackLanded(event) => event.write(stream, definition),
            GameEvent::MedicDefended(event) => event.write(stream, definition),
            GameEvent::LocalPlayerHealed(event) => event.write(stream, definition),
            GameEvent::PlayerDestroyedPipeBomb(event) => event.write(stream, definition),
            GameEvent::ObjectDeflected(event) => event.write(stream, definition),
            GameEvent::PlayerMvp(event) => event.write(stream, definition),
            GameEvent::RaidSpawnMob(event) => event.write(stream, definition),
            GameEvent::RaidSpawnSquad(event) => event.write(stream, definition),
            GameEvent::NavBlocked(event) => event.write(stream, definition),
            GameEvent::PathTrackPassed(event) => event.write(stream, definition),
            GameEvent::NumCappersChanged(event) => event.write(stream, definition),
            GameEvent::PlayerRegenerate(event) => event.write(stream, definition),
            GameEvent::UpdateStatusItem(event) => event.write(stream, definition),
            GameEvent::StatsResetRound(event) => event.write(stream, definition),
            GameEvent::ScoreStatsAccumulatedUpdate(event) => event.write(stream, definition),
            GameEvent::ScoreStatsAccumulatedReset(event) => event.write(stream, definition),
            GameEvent::AchievementEarnedLocal(event) => event.write(stream, definition),
            GameEvent::PlayerHealed(event) => event.write(stream, definition),
            GameEvent::BuildingHealed(event) => event.write(stream, definition),
            GameEvent::ItemPickup(event) => event.write(stream, definition),
            GameEvent::DuelStatus(event) => event.write(stream, definition),
            GameEvent::FishNotice(event) => event.write(stream, definition),
            GameEvent::FishNoticeArm(event) => event.write(stream, definition),
            GameEvent::SlapNotice(event) => event.write(stream, definition),
            GameEvent::ThrowableHit(event) => event.write(stream, definition),
            GameEvent::PumpkinLordSummoned(event) => event.write(stream, definition),
            GameEvent::PumpkinLordKilled(event) => event.write(stream, definition),
            GameEvent::MerasmusSummoned(event) => event.write(stream, definition),
            GameEvent::MerasmusKilled(event) => event.write(stream, definition),
            GameEvent::MerasmusEscapeWarning(event) => event.write(stream, definition),
            GameEvent::MerasmusEscaped(event) => event.write(stream, definition),
            GameEvent::EyeballBossSummoned(event) => event.write(stream, definition),
            GameEvent::EyeballBossStunned(event) => event.write(stream, definition),
            GameEvent::EyeballBossKilled(event) => event.write(stream, definition),
            GameEvent::EyeballBossKiller(event) => event.write(stream, definition),
            GameEvent::EyeballBossEscapeImminent(event) => event.write(stream, definition),
            GameEvent::EyeballBossEscaped(event) => event.write(stream, definition),
            GameEvent::NpcHurt(event) => event.write(stream, definition),
            GameEvent::ControlPointTimerUpdated(event) => event.write(stream, definition),
            GameEvent::PlayerHighFiveStart(event) => event.write(stream, definition),
            GameEvent::PlayerHighFiveCancel(event) => event.write(stream, definition),
            GameEvent::PlayerHighFiveSuccess(event) => event.write(stream, definition),
            GameEvent::PlayerBonusPoints(event) => event.write(stream, definition),
            GameEvent::PlayerUpgraded(event) => event.write(stream, definition),
            GameEvent::PlayerBuyback(event) => event.write(stream, definition),
            GameEvent::PlayerUsedPowerUpBottle(event) => event.write(stream, definition),
            GameEvent::ChristmasGiftGrab(event) => event.write(stream, definition),
            GameEvent::PlayerKilledAchievementZone(event) => event.write(stream, definition),
            GameEvent::PartyUpdated(event) => event.write(stream, definition),
            GameEvent::PartyPrefChanged(event) => event.write(stream, definition),
            GameEvent::PartyCriteriaChanged(event) => event.write(stream, definition),
            GameEvent::PartyInvitesChanged(event) => event.write(stream, definition),
            GameEvent::PartyQueueStateChanged(event) => event.write(stream, definition),
            GameEvent::PartyChat(event) => event.write(stream, definition),
            GameEvent::PartyMemberJoin(event) => event.write(stream, definition),
            GameEvent::PartyMemberLeave(event) => event.write(stream, definition),
            GameEvent::MatchInvitesUpdated(event) => event.write(stream, definition),
            GameEvent::LobbyUpdated(event) => event.write(stream, definition),
            GameEvent::MvmMissionUpdate(event) => event.write(stream, definition),
            GameEvent::RecalculateHolidays(event) => event.write(stream, definition),
            GameEvent::PlayerCurrencyChanged(event) => event.write(stream, definition),
            GameEvent::DoomsdayRocketOpen(event) => event.write(stream, definition),
            GameEvent::RemoveNemesisRelationships(event) => event.write(stream, definition),
            GameEvent::MvmCreditBonusWave(event) => event.write(stream, definition),
            GameEvent::MvmCreditBonusAll(event) => event.write(stream, definition),
            GameEvent::MvmCreditBonusAllAdvanced(event) => event.write(stream, definition),
            GameEvent::MvmQuickSentryUpgrade(event) => event.write(stream, definition),
            GameEvent::MvmTankDestroyedByPlayers(event) => event.write(stream, definition),
            GameEvent::MvmKillRobotDeliveringBomb(event) => event.write(stream, definition),
            GameEvent::MvmPickupCurrency(event) => event.write(stream, definition),
            GameEvent::MvmBombCarrierKilled(event) => event.write(stream, definition),
            GameEvent::MvmSentryBusterDetonate(event) => event.write(stream, definition),
            GameEvent::MvmScoutMarkedForDeath(event) => event.write(stream, definition),
            GameEvent::MvmMedicPowerUpShared(event) => event.write(stream, definition),
            GameEvent::MvmBeginWave(event) => event.write(stream, definition),
            GameEvent::MvmWaveComplete(event) => event.write(stream, definition),
            GameEvent::MvmMissionComplete(event) => event.write(stream, definition),
            GameEvent::MvmBombResetByPlayer(event) => event.write(stream, definition),
            GameEvent::MvmBombAlarmTriggered(event) => event.write(stream, definition),
            GameEvent::MvmBombDeployResetByPlayer(event) => event.write(stream, definition),
            GameEvent::MvmWaveFailed(event) => event.write(stream, definition),
            GameEvent::MvmResetStats(event) => event.write(stream, definition),
            GameEvent::DamageResisted(event) => event.write(stream, definition),
            GameEvent::RevivePlayerNotify(event) => event.write(stream, definition),
            GameEvent::RevivePlayerStopped(event) => event.write(stream, definition),
            GameEvent::RevivePlayerComplete(event) => event.write(stream, definition),
            GameEvent::PlayerTurnedToGhost(event) => event.write(stream, definition),
            GameEvent::MedigunShieldBlockedDamage(event) => event.write(stream, definition),
            GameEvent::MvmAdvWaveCompleteNoGates(event) => event.write(stream, definition),
            GameEvent::MvmSniperHeadshotCurrency(event) => event.write(stream, definition),
            GameEvent::MvmMannhattanPit(event) => event.write(stream, definition),
            GameEvent::FlagCarriedInDetectionZone(event) => event.write(stream, definition),
            GameEvent::MvmAdvWaveKilledStunRadio(event) => event.write(stream, definition),
            GameEvent::PlayerDirectHitStun(event) => event.write(stream, definition),
            GameEvent::MvmSentryBusterKilled(event) => event.write(stream, definition),
            GameEvent::UpgradesFileChanged(event) => event.write(stream, definition),
            GameEvent::RdTeamPointsChanged(event) => event.write(stream, definition),
            GameEvent::RdRulesStateChanged(event) => event.write(stream, definition),
            GameEvent::RdRobotKilled(event) => event.write(stream, definition),
            GameEvent::RdRobotImpact(event) => event.write(stream, definition),
            GameEvent::TeamPlayPreRoundTimeLeft(event) => event.write(stream, definition),
            GameEvent::ParachuteDeploy(event) => event.write(stream, definition),
            GameEvent::ParachuteHolster(event) => event.write(stream, definition),
            GameEvent::KillRefillsMeter(event) => event.write(stream, definition),
            GameEvent::RpsTauntEvent(event) => event.write(stream, definition),
            GameEvent::CongaKill(event) => event.write(stream, definition),
            GameEvent::PlayerInitialSpawn(event) => event.write(stream, definition),
            GameEvent::CompetitiveVictory(event) => event.write(stream, definition),
            GameEvent::CompetitiveStatsUpdate(event) => event.write(stream, definition),
            GameEvent::MiniGameWin(event) => event.write(stream, definition),
            GameEvent::SentryOnGoActive(event) => event.write(stream, definition),
            GameEvent::DuckXpLevelUp(event) => event.write(stream, definition),
            GameEvent::QuestLogOpened(event) => event.write(stream, definition),
            GameEvent::SchemaUpdated(event) => event.write(stream, definition),
            GameEvent::LocalPlayerPickupWeapon(event) => event.write(stream, definition),
            GameEvent::RdPlayerScorePoints(event) => event.write(stream, definition),
            GameEvent::DemomanDetStickies(event) => event.write(stream, definition),
            GameEvent::QuestObjectiveCompleted(event) => event.write(stream, definition),
            GameEvent::PlayerScoreChanged(event) => event.write(stream, definition),
            GameEvent::KilledCappingPlayer(event) => event.write(stream, definition),
            GameEvent::EnvironmentalDeath(event) => event.write(stream, definition),
            GameEvent::ProjectileDirectHit(event) => event.write(stream, definition),
            GameEvent::PassGet(event) => event.write(stream, definition),
            GameEvent::PassScore(event) => event.write(stream, definition),
            GameEvent::PassFree(event) => event.write(stream, definition),
            GameEvent::PassPassCaught(event) => event.write(stream, definition),
            GameEvent::PassBallStolen(event) => event.write(stream, definition),
            GameEvent::PassBallBlocked(event) => event.write(stream, definition),
            GameEvent::DamagePrevented(event) => event.write(stream, definition),
            GameEvent::HalloweenBossKilled(event) => event.write(stream, definition),
            GameEvent::EscapedLootIsland(event) => event.write(stream, definition),
            GameEvent::TaggedPlayerAsIt(event) => event.write(stream, definition),
            GameEvent::MerasmusStunned(event) => event.write(stream, definition),
            GameEvent::MerasmusPropFound(event) => event.write(stream, definition),
            GameEvent::HalloweenSkeletonKilled(event) => event.write(stream, definition),
            GameEvent::SkeletonKilledQuest(event) => event.write(stream, definition),
            GameEvent::SkeletonKingKilledQuest(event) => event.write(stream, definition),
            GameEvent::EscapeHell(event) => event.write(stream, definition),
            GameEvent::CrossSpectralBridge(event) => event.write(stream, definition),
            GameEvent::MiniGameWon(event) => event.write(stream, definition),
            GameEvent::RespawnGhost(event) => event.write(stream, definition),
            GameEvent::KillInHell(event) => event.write(stream, definition),
            GameEvent::HalloweenDuckCollected(event) => event.write(stream, definition),
            GameEvent::SpecialScore(event) => event.write(stream, definition),
            GameEvent::TeamLeaderKilled(event) => event.write(stream, definition),
            GameEvent::HalloweenSoulCollected(event) => event.write(stream, definition),
            GameEvent::RecalculateTruce(event) => event.write(stream, definition),
            GameEvent::DeadRingerCheatDeath(event) => event.write(stream, definition),
            GameEvent::CrossbowHeal(event) => event.write(stream, definition),
            GameEvent::DamageMitigated(event) => event.write(stream, definition),
            GameEvent::PayloadPushed(event) => event.write(stream, definition),
            GameEvent::PlayerAbandonedMatch(event) => event.write(stream, definition),
            GameEvent::ClDrawline(event) => event.write(stream, definition),
            GameEvent::RestartTimerTime(event) => event.write(stream, definition),
            GameEvent::WinLimitChanged(event) => event.write(stream, definition),
            GameEvent::WinPanelShowScores(event) => event.write(stream, definition),
            GameEvent::TopStreamsRequestFinished(event) => event.write(stream, definition),
            GameEvent::CompetitiveStateChanged(event) => event.write(stream, definition),
            GameEvent::GlobalWarDataUpdated(event) => event.write(stream, definition),
            GameEvent::StopWatchChanged(event) => event.write(stream, definition),
            GameEvent::DsStop(event) => event.write(stream, definition),
            GameEvent::DsScreenshot(event) => event.write(stream, definition),
            GameEvent::ShowMatchSummary(event) => event.write(stream, definition),
            GameEvent::ExperienceChanged(event) => event.write(stream, definition),
            GameEvent::BeginXpLerp(event) => event.write(stream, definition),
            GameEvent::MatchmakerStatsUpdated(event) => event.write(stream, definition),
            GameEvent::RematchVotePeriodOver(event) => event.write(stream, definition),
            GameEvent::RematchFailedToCreate(event) => event.write(stream, definition),
            GameEvent::PlayerRematchChange(event) => event.write(stream, definition),
            GameEvent::PingUpdated(event) => event.write(stream, definition),
            GameEvent::MMStatsUpdated(event) => event.write(stream, definition),
            GameEvent::PlayerNextMapVoteChange(event) => event.write(stream, definition),
            GameEvent::VoteMapsChanged(event) => event.write(stream, definition),
            GameEvent::ProtoDefChanged(event) => event.write(stream, definition),
            GameEvent::PlayerDomination(event) => event.write(stream, definition),
            GameEvent::PlayerRocketPackPushed(event) => event.write(stream, definition),
            GameEvent::QuestRequest(event) => event.write(stream, definition),
            GameEvent::QuestResponse(event) => event.write(stream, definition),
            GameEvent::QuestProgress(event) => event.write(stream, definition),
            GameEvent::ProjectileRemoved(event) => event.write(stream, definition),
            GameEvent::QuestMapDataChanged(event) => event.write(stream, definition),
            GameEvent::GasDousedPlayerIgnited(event) => event.write(stream, definition),
            GameEvent::QuestTurnInState(event) => event.write(stream, definition),
            GameEvent::ItemsAcknowledged(event) => event.write(stream, definition),
            GameEvent::CapperKilled(event) => event.write(stream, definition),
            GameEvent::MainMenuStabilized(event) => event.write(stream, definition),
            GameEvent::WorldStatusChanged(event) => event.write(stream, definition),
            GameEvent::HLTVStatus(event) => event.write(stream, definition),
            GameEvent::HLTVCameraman(event) => event.write(stream, definition),
            GameEvent::HLTVRankCamera(event) => event.write(stream, definition),
            GameEvent::HLTVRankEntity(event) => event.write(stream, definition),
            GameEvent::HLTVFixed(event) => event.write(stream, definition),
            GameEvent::HLTVChase(event) => event.write(stream, definition),
            GameEvent::HLTVMessage(event) => event.write(stream, definition),
            GameEvent::HLTVTitle(event) => event.write(stream, definition),
            GameEvent::HLTVChat(event) => event.write(stream, definition),
            GameEvent::ReplayStartRecord(event) => event.write(stream, definition),
            GameEvent::ReplaySessionInfo(event) => event.write(stream, definition),
            GameEvent::ReplayEndRecord(event) => event.write(stream, definition),
            GameEvent::ReplayReplaysAvailable(event) => event.write(stream, definition),
            GameEvent::ReplayServerError(event) => event.write(stream, definition),
            GameEvent::Unknown(raw) => Ok(raw.write(stream)?),
        }
    }
    pub fn event_type(&self) -> GameEventType {
        match &self {
            GameEvent::ServerSpawn(_) => GameEventType::ServerSpawn,
            GameEvent::ServerChangeLevelFailed(_) => GameEventType::ServerChangeLevelFailed,
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
            GameEvent::HelicopterGrenadePuntMiss(_) => GameEventType::HelicopterGrenadePuntMiss,
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
            GameEvent::ControlPointInitialized(_) => GameEventType::ControlPointInitialized,
            GameEvent::ControlPointUpdateImages(_) => GameEventType::ControlPointUpdateImages,
            GameEvent::ControlPointUpdateLayout(_) => GameEventType::ControlPointUpdateLayout,
            GameEvent::ControlPointUpdateCapping(_) => GameEventType::ControlPointUpdateCapping,
            GameEvent::ControlPointUpdateOwner(_) => GameEventType::ControlPointUpdateOwner,
            GameEvent::ControlPointStartTouch(_) => GameEventType::ControlPointStartTouch,
            GameEvent::ControlPointEndTouch(_) => GameEventType::ControlPointEndTouch,
            GameEvent::ControlPointPulseElement(_) => GameEventType::ControlPointPulseElement,
            GameEvent::ControlPointFakeCapture(_) => GameEventType::ControlPointFakeCapture,
            GameEvent::ControlPointFakeCaptureMultiplier(_) => {
                GameEventType::ControlPointFakeCaptureMultiplier
            }
            GameEvent::TeamPlayRoundSelected(_) => GameEventType::TeamPlayRoundSelected,
            GameEvent::TeamPlayRoundStart(_) => GameEventType::TeamPlayRoundStart,
            GameEvent::TeamPlayRoundActive(_) => GameEventType::TeamPlayRoundActive,
            GameEvent::TeamPlayWaitingBegins(_) => GameEventType::TeamPlayWaitingBegins,
            GameEvent::TeamPlayWaitingEnds(_) => GameEventType::TeamPlayWaitingEnds,
            GameEvent::TeamPlayWaitingAboutToEnd(_) => GameEventType::TeamPlayWaitingAboutToEnd,
            GameEvent::TeamPlayRestartRound(_) => GameEventType::TeamPlayRestartRound,
            GameEvent::TeamPlayReadyRestart(_) => GameEventType::TeamPlayReadyRestart,
            GameEvent::TeamPlayRoundRestartSeconds(_) => GameEventType::TeamPlayRoundRestartSeconds,
            GameEvent::TeamPlayTeamReady(_) => GameEventType::TeamPlayTeamReady,
            GameEvent::TeamPlayRoundWin(_) => GameEventType::TeamPlayRoundWin,
            GameEvent::TeamPlayUpdateTimer(_) => GameEventType::TeamPlayUpdateTimer,
            GameEvent::TeamPlayRoundStalemate(_) => GameEventType::TeamPlayRoundStalemate,
            GameEvent::TeamPlayOvertimeBegin(_) => GameEventType::TeamPlayOvertimeBegin,
            GameEvent::TeamPlayOvertimeEnd(_) => GameEventType::TeamPlayOvertimeEnd,
            GameEvent::TeamPlaySuddenDeathBegin(_) => GameEventType::TeamPlaySuddenDeathBegin,
            GameEvent::TeamPlaySuddenDeathEnd(_) => GameEventType::TeamPlaySuddenDeathEnd,
            GameEvent::TeamPlayGameOver(_) => GameEventType::TeamPlayGameOver,
            GameEvent::TeamPlayMapTimeRemaining(_) => GameEventType::TeamPlayMapTimeRemaining,
            GameEvent::TeamPlayTimerFlash(_) => GameEventType::TeamPlayTimerFlash,
            GameEvent::TeamPlayTimerTimeAdded(_) => GameEventType::TeamPlayTimerTimeAdded,
            GameEvent::TeamPlayPointStartCapture(_) => GameEventType::TeamPlayPointStartCapture,
            GameEvent::TeamPlayPointCaptured(_) => GameEventType::TeamPlayPointCaptured,
            GameEvent::TeamPlayPointLocked(_) => GameEventType::TeamPlayPointLocked,
            GameEvent::TeamPlayPointUnlocked(_) => GameEventType::TeamPlayPointUnlocked,
            GameEvent::TeamPlayCaptureBroken(_) => GameEventType::TeamPlayCaptureBroken,
            GameEvent::TeamPlayCaptureBlocked(_) => GameEventType::TeamPlayCaptureBlocked,
            GameEvent::TeamPlayFlagEvent(_) => GameEventType::TeamPlayFlagEvent,
            GameEvent::TeamPlayWinPanel(_) => GameEventType::TeamPlayWinPanel,
            GameEvent::TeamPlayTeamBalancedPlayer(_) => GameEventType::TeamPlayTeamBalancedPlayer,
            GameEvent::TeamPlaySetupFinished(_) => GameEventType::TeamPlaySetupFinished,
            GameEvent::TeamPlayAlert(_) => GameEventType::TeamPlayAlert,
            GameEvent::TrainingComplete(_) => GameEventType::TrainingComplete,
            GameEvent::ShowFreezePanel(_) => GameEventType::ShowFreezePanel,
            GameEvent::HideFreezePanel(_) => GameEventType::HideFreezePanel,
            GameEvent::FreezeCamStarted(_) => GameEventType::FreezeCamStarted,
            GameEvent::LocalPlayerChangeTeam(_) => GameEventType::LocalPlayerChangeTeam,
            GameEvent::LocalPlayerScoreChanged(_) => GameEventType::LocalPlayerScoreChanged,
            GameEvent::LocalPlayerChangeClass(_) => GameEventType::LocalPlayerChangeClass,
            GameEvent::LocalPlayerRespawn(_) => GameEventType::LocalPlayerRespawn,
            GameEvent::BuildingInfoChanged(_) => GameEventType::BuildingInfoChanged,
            GameEvent::LocalPlayerChangeDisguise(_) => GameEventType::LocalPlayerChangeDisguise,
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
            GameEvent::TournamentEnableCountdown(_) => GameEventType::TournamentEnableCountdown,
            GameEvent::PlayerCalledForMedic(_) => GameEventType::PlayerCalledForMedic,
            GameEvent::PlayerAskedForBall(_) => GameEventType::PlayerAskedForBall,
            GameEvent::LocalPlayerBecameObserver(_) => GameEventType::LocalPlayerBecameObserver,
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
            GameEvent::ArenaPlayerNotification(_) => GameEventType::ArenaPlayerNotification,
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
            GameEvent::PostInventoryApplication(_) => GameEventType::PostInventoryApplication,
            GameEvent::ControlPointUnlockUpdated(_) => GameEventType::ControlPointUnlockUpdated,
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
            GameEvent::PlayerDestroyedPipeBomb(_) => GameEventType::PlayerDestroyedPipeBomb,
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
            GameEvent::ScoreStatsAccumulatedUpdate(_) => GameEventType::ScoreStatsAccumulatedUpdate,
            GameEvent::ScoreStatsAccumulatedReset(_) => GameEventType::ScoreStatsAccumulatedReset,
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
            GameEvent::EyeballBossEscapeImminent(_) => GameEventType::EyeballBossEscapeImminent,
            GameEvent::EyeballBossEscaped(_) => GameEventType::EyeballBossEscaped,
            GameEvent::NpcHurt(_) => GameEventType::NpcHurt,
            GameEvent::ControlPointTimerUpdated(_) => GameEventType::ControlPointTimerUpdated,
            GameEvent::PlayerHighFiveStart(_) => GameEventType::PlayerHighFiveStart,
            GameEvent::PlayerHighFiveCancel(_) => GameEventType::PlayerHighFiveCancel,
            GameEvent::PlayerHighFiveSuccess(_) => GameEventType::PlayerHighFiveSuccess,
            GameEvent::PlayerBonusPoints(_) => GameEventType::PlayerBonusPoints,
            GameEvent::PlayerUpgraded(_) => GameEventType::PlayerUpgraded,
            GameEvent::PlayerBuyback(_) => GameEventType::PlayerBuyback,
            GameEvent::PlayerUsedPowerUpBottle(_) => GameEventType::PlayerUsedPowerUpBottle,
            GameEvent::ChristmasGiftGrab(_) => GameEventType::ChristmasGiftGrab,
            GameEvent::PlayerKilledAchievementZone(_) => GameEventType::PlayerKilledAchievementZone,
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
            GameEvent::RemoveNemesisRelationships(_) => GameEventType::RemoveNemesisRelationships,
            GameEvent::MvmCreditBonusWave(_) => GameEventType::MvmCreditBonusWave,
            GameEvent::MvmCreditBonusAll(_) => GameEventType::MvmCreditBonusAll,
            GameEvent::MvmCreditBonusAllAdvanced(_) => GameEventType::MvmCreditBonusAllAdvanced,
            GameEvent::MvmQuickSentryUpgrade(_) => GameEventType::MvmQuickSentryUpgrade,
            GameEvent::MvmTankDestroyedByPlayers(_) => GameEventType::MvmTankDestroyedByPlayers,
            GameEvent::MvmKillRobotDeliveringBomb(_) => GameEventType::MvmKillRobotDeliveringBomb,
            GameEvent::MvmPickupCurrency(_) => GameEventType::MvmPickupCurrency,
            GameEvent::MvmBombCarrierKilled(_) => GameEventType::MvmBombCarrierKilled,
            GameEvent::MvmSentryBusterDetonate(_) => GameEventType::MvmSentryBusterDetonate,
            GameEvent::MvmScoutMarkedForDeath(_) => GameEventType::MvmScoutMarkedForDeath,
            GameEvent::MvmMedicPowerUpShared(_) => GameEventType::MvmMedicPowerUpShared,
            GameEvent::MvmBeginWave(_) => GameEventType::MvmBeginWave,
            GameEvent::MvmWaveComplete(_) => GameEventType::MvmWaveComplete,
            GameEvent::MvmMissionComplete(_) => GameEventType::MvmMissionComplete,
            GameEvent::MvmBombResetByPlayer(_) => GameEventType::MvmBombResetByPlayer,
            GameEvent::MvmBombAlarmTriggered(_) => GameEventType::MvmBombAlarmTriggered,
            GameEvent::MvmBombDeployResetByPlayer(_) => GameEventType::MvmBombDeployResetByPlayer,
            GameEvent::MvmWaveFailed(_) => GameEventType::MvmWaveFailed,
            GameEvent::MvmResetStats(_) => GameEventType::MvmResetStats,
            GameEvent::DamageResisted(_) => GameEventType::DamageResisted,
            GameEvent::RevivePlayerNotify(_) => GameEventType::RevivePlayerNotify,
            GameEvent::RevivePlayerStopped(_) => GameEventType::RevivePlayerStopped,
            GameEvent::RevivePlayerComplete(_) => GameEventType::RevivePlayerComplete,
            GameEvent::PlayerTurnedToGhost(_) => GameEventType::PlayerTurnedToGhost,
            GameEvent::MedigunShieldBlockedDamage(_) => GameEventType::MedigunShieldBlockedDamage,
            GameEvent::MvmAdvWaveCompleteNoGates(_) => GameEventType::MvmAdvWaveCompleteNoGates,
            GameEvent::MvmSniperHeadshotCurrency(_) => GameEventType::MvmSniperHeadshotCurrency,
            GameEvent::MvmMannhattanPit(_) => GameEventType::MvmMannhattanPit,
            GameEvent::FlagCarriedInDetectionZone(_) => GameEventType::FlagCarriedInDetectionZone,
            GameEvent::MvmAdvWaveKilledStunRadio(_) => GameEventType::MvmAdvWaveKilledStunRadio,
            GameEvent::PlayerDirectHitStun(_) => GameEventType::PlayerDirectHitStun,
            GameEvent::MvmSentryBusterKilled(_) => GameEventType::MvmSentryBusterKilled,
            GameEvent::UpgradesFileChanged(_) => GameEventType::UpgradesFileChanged,
            GameEvent::RdTeamPointsChanged(_) => GameEventType::RdTeamPointsChanged,
            GameEvent::RdRulesStateChanged(_) => GameEventType::RdRulesStateChanged,
            GameEvent::RdRobotKilled(_) => GameEventType::RdRobotKilled,
            GameEvent::RdRobotImpact(_) => GameEventType::RdRobotImpact,
            GameEvent::TeamPlayPreRoundTimeLeft(_) => GameEventType::TeamPlayPreRoundTimeLeft,
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
            GameEvent::LocalPlayerPickupWeapon(_) => GameEventType::LocalPlayerPickupWeapon,
            GameEvent::RdPlayerScorePoints(_) => GameEventType::RdPlayerScorePoints,
            GameEvent::DemomanDetStickies(_) => GameEventType::DemomanDetStickies,
            GameEvent::QuestObjectiveCompleted(_) => GameEventType::QuestObjectiveCompleted,
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
            GameEvent::HalloweenSkeletonKilled(_) => GameEventType::HalloweenSkeletonKilled,
            GameEvent::SkeletonKilledQuest(_) => GameEventType::SkeletonKilledQuest,
            GameEvent::SkeletonKingKilledQuest(_) => GameEventType::SkeletonKingKilledQuest,
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
            GameEvent::TopStreamsRequestFinished(_) => GameEventType::TopStreamsRequestFinished,
            GameEvent::CompetitiveStateChanged(_) => GameEventType::CompetitiveStateChanged,
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
            GameEvent::PlayerNextMapVoteChange(_) => GameEventType::PlayerNextMapVoteChange,
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
        (
            "ServerChangeLevelFailed",
            std::mem::size_of::<ServerChangeLevelFailedEvent>(),
        ),
        ("ServerShutdown", std::mem::size_of::<ServerShutdownEvent>()),
        ("ServerCvar", std::mem::size_of::<ServerCvarEvent>()),
        ("ServerMessage", std::mem::size_of::<ServerMessageEvent>()),
        ("ServerAddBan", std::mem::size_of::<ServerAddBanEvent>()),
        (
            "ServerRemoveBan",
            std::mem::size_of::<ServerRemoveBanEvent>(),
        ),
        ("PlayerConnect", std::mem::size_of::<PlayerConnectEvent>()),
        (
            "PlayerConnectClient",
            std::mem::size_of::<PlayerConnectClientEvent>(),
        ),
        ("PlayerInfo", std::mem::size_of::<PlayerInfoEvent>()),
        (
            "PlayerDisconnect",
            std::mem::size_of::<PlayerDisconnectEvent>(),
        ),
        ("PlayerActivate", std::mem::size_of::<PlayerActivateEvent>()),
        ("PlayerSay", std::mem::size_of::<PlayerSayEvent>()),
        (
            "ClientDisconnect",
            std::mem::size_of::<ClientDisconnectEvent>(),
        ),
        (
            "ClientBeginConnect",
            std::mem::size_of::<ClientBeginConnectEvent>(),
        ),
        (
            "ClientConnected",
            std::mem::size_of::<ClientConnectedEvent>(),
        ),
        (
            "ClientFullConnect",
            std::mem::size_of::<ClientFullConnectEvent>(),
        ),
        ("HostQuit", std::mem::size_of::<HostQuitEvent>()),
        ("TeamInfo", std::mem::size_of::<TeamInfoEvent>()),
        ("TeamScore", std::mem::size_of::<TeamScoreEvent>()),
        (
            "TeamPlayBroadcastAudio",
            std::mem::size_of::<TeamPlayBroadcastAudioEvent>(),
        ),
        ("PlayerTeam", std::mem::size_of::<PlayerTeamEvent>()),
        ("PlayerClass", std::mem::size_of::<PlayerClassEvent>()),
        ("PlayerDeath", std::mem::size_of::<PlayerDeathEvent>()),
        ("PlayerHurt", std::mem::size_of::<PlayerHurtEvent>()),
        ("PlayerChat", std::mem::size_of::<PlayerChatEvent>()),
        ("PlayerScore", std::mem::size_of::<PlayerScoreEvent>()),
        ("PlayerSpawn", std::mem::size_of::<PlayerSpawnEvent>()),
        ("PlayerShoot", std::mem::size_of::<PlayerShootEvent>()),
        ("PlayerUse", std::mem::size_of::<PlayerUseEvent>()),
        (
            "PlayerChangeName",
            std::mem::size_of::<PlayerChangeNameEvent>(),
        ),
        (
            "PlayerHintMessage",
            std::mem::size_of::<PlayerHintMessageEvent>(),
        ),
        (
            "BasePlayerTeleported",
            std::mem::size_of::<BasePlayerTeleportedEvent>(),
        ),
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
        (
            "AchievementEvent",
            std::mem::size_of::<AchievementEventEvent>(),
        ),
        (
            "AchievementIncrement",
            std::mem::size_of::<AchievementIncrementEvent>(),
        ),
        ("PhysgunPickup", std::mem::size_of::<PhysgunPickupEvent>()),
        ("FlareIgniteNpc", std::mem::size_of::<FlareIgniteNpcEvent>()),
        (
            "HelicopterGrenadePuntMiss",
            std::mem::size_of::<HelicopterGrenadePuntMissEvent>(),
        ),
        (
            "UserDataDownloaded",
            std::mem::size_of::<UserDataDownloadedEvent>(),
        ),
        (
            "RagdollDissolved",
            std::mem::size_of::<RagdollDissolvedEvent>(),
        ),
        (
            "HLTVChangedMode",
            std::mem::size_of::<HLTVChangedModeEvent>(),
        ),
        (
            "HLTVChangedTarget",
            std::mem::size_of::<HLTVChangedTargetEvent>(),
        ),
        ("VoteEnded", std::mem::size_of::<VoteEndedEvent>()),
        ("VoteStarted", std::mem::size_of::<VoteStartedEvent>()),
        ("VoteChanged", std::mem::size_of::<VoteChangedEvent>()),
        ("VotePassed", std::mem::size_of::<VotePassedEvent>()),
        ("VoteFailed", std::mem::size_of::<VoteFailedEvent>()),
        ("VoteCast", std::mem::size_of::<VoteCastEvent>()),
        ("VoteOptions", std::mem::size_of::<VoteOptionsEvent>()),
        ("ReplaySaved", std::mem::size_of::<ReplaySavedEvent>()),
        (
            "EnteredPerformanceMode",
            std::mem::size_of::<EnteredPerformanceModeEvent>(),
        ),
        ("BrowseReplays", std::mem::size_of::<BrowseReplaysEvent>()),
        (
            "ReplayYoutubeStats",
            std::mem::size_of::<ReplayYoutubeStatsEvent>(),
        ),
        (
            "InventoryUpdated",
            std::mem::size_of::<InventoryUpdatedEvent>(),
        ),
        ("CartUpdated", std::mem::size_of::<CartUpdatedEvent>()),
        (
            "StorePriceSheetUpdated",
            std::mem::size_of::<StorePriceSheetUpdatedEvent>(),
        ),
        (
            "EconInventoryConnected",
            std::mem::size_of::<EconInventoryConnectedEvent>(),
        ),
        (
            "ItemSchemaInitialized",
            std::mem::size_of::<ItemSchemaInitializedEvent>(),
        ),
        ("GcNewSession", std::mem::size_of::<GcNewSessionEvent>()),
        ("GcLostSession", std::mem::size_of::<GcLostSessionEvent>()),
        ("IntroFinish", std::mem::size_of::<IntroFinishEvent>()),
        (
            "IntroNextCamera",
            std::mem::size_of::<IntroNextCameraEvent>(),
        ),
        (
            "PlayerChangeClass",
            std::mem::size_of::<PlayerChangeClassEvent>(),
        ),
        (
            "TfMapTimeRemaining",
            std::mem::size_of::<TfMapTimeRemainingEvent>(),
        ),
        ("TfGameOver", std::mem::size_of::<TfGameOverEvent>()),
        (
            "CtfFlagCaptured",
            std::mem::size_of::<CtfFlagCapturedEvent>(),
        ),
        (
            "ControlPointInitialized",
            std::mem::size_of::<ControlPointInitializedEvent>(),
        ),
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
        (
            "ControlPointUpdateOwner",
            std::mem::size_of::<ControlPointUpdateOwnerEvent>(),
        ),
        (
            "ControlPointStartTouch",
            std::mem::size_of::<ControlPointStartTouchEvent>(),
        ),
        (
            "ControlPointEndTouch",
            std::mem::size_of::<ControlPointEndTouchEvent>(),
        ),
        (
            "ControlPointPulseElement",
            std::mem::size_of::<ControlPointPulseElementEvent>(),
        ),
        (
            "ControlPointFakeCapture",
            std::mem::size_of::<ControlPointFakeCaptureEvent>(),
        ),
        (
            "ControlPointFakeCaptureMultiplier",
            std::mem::size_of::<ControlPointFakeCaptureMultiplierEvent>(),
        ),
        (
            "TeamPlayRoundSelected",
            std::mem::size_of::<TeamPlayRoundSelectedEvent>(),
        ),
        (
            "TeamPlayRoundStart",
            std::mem::size_of::<TeamPlayRoundStartEvent>(),
        ),
        (
            "TeamPlayRoundActive",
            std::mem::size_of::<TeamPlayRoundActiveEvent>(),
        ),
        (
            "TeamPlayWaitingBegins",
            std::mem::size_of::<TeamPlayWaitingBeginsEvent>(),
        ),
        (
            "TeamPlayWaitingEnds",
            std::mem::size_of::<TeamPlayWaitingEndsEvent>(),
        ),
        (
            "TeamPlayWaitingAboutToEnd",
            std::mem::size_of::<TeamPlayWaitingAboutToEndEvent>(),
        ),
        (
            "TeamPlayRestartRound",
            std::mem::size_of::<TeamPlayRestartRoundEvent>(),
        ),
        (
            "TeamPlayReadyRestart",
            std::mem::size_of::<TeamPlayReadyRestartEvent>(),
        ),
        (
            "TeamPlayRoundRestartSeconds",
            std::mem::size_of::<TeamPlayRoundRestartSecondsEvent>(),
        ),
        (
            "TeamPlayTeamReady",
            std::mem::size_of::<TeamPlayTeamReadyEvent>(),
        ),
        (
            "TeamPlayRoundWin",
            std::mem::size_of::<TeamPlayRoundWinEvent>(),
        ),
        (
            "TeamPlayUpdateTimer",
            std::mem::size_of::<TeamPlayUpdateTimerEvent>(),
        ),
        (
            "TeamPlayRoundStalemate",
            std::mem::size_of::<TeamPlayRoundStalemateEvent>(),
        ),
        (
            "TeamPlayOvertimeBegin",
            std::mem::size_of::<TeamPlayOvertimeBeginEvent>(),
        ),
        (
            "TeamPlayOvertimeEnd",
            std::mem::size_of::<TeamPlayOvertimeEndEvent>(),
        ),
        (
            "TeamPlaySuddenDeathBegin",
            std::mem::size_of::<TeamPlaySuddenDeathBeginEvent>(),
        ),
        (
            "TeamPlaySuddenDeathEnd",
            std::mem::size_of::<TeamPlaySuddenDeathEndEvent>(),
        ),
        (
            "TeamPlayGameOver",
            std::mem::size_of::<TeamPlayGameOverEvent>(),
        ),
        (
            "TeamPlayMapTimeRemaining",
            std::mem::size_of::<TeamPlayMapTimeRemainingEvent>(),
        ),
        (
            "TeamPlayTimerFlash",
            std::mem::size_of::<TeamPlayTimerFlashEvent>(),
        ),
        (
            "TeamPlayTimerTimeAdded",
            std::mem::size_of::<TeamPlayTimerTimeAddedEvent>(),
        ),
        (
            "TeamPlayPointStartCapture",
            std::mem::size_of::<TeamPlayPointStartCaptureEvent>(),
        ),
        (
            "TeamPlayPointCaptured",
            std::mem::size_of::<TeamPlayPointCapturedEvent>(),
        ),
        (
            "TeamPlayPointLocked",
            std::mem::size_of::<TeamPlayPointLockedEvent>(),
        ),
        (
            "TeamPlayPointUnlocked",
            std::mem::size_of::<TeamPlayPointUnlockedEvent>(),
        ),
        (
            "TeamPlayCaptureBroken",
            std::mem::size_of::<TeamPlayCaptureBrokenEvent>(),
        ),
        (
            "TeamPlayCaptureBlocked",
            std::mem::size_of::<TeamPlayCaptureBlockedEvent>(),
        ),
        (
            "TeamPlayFlagEvent",
            std::mem::size_of::<TeamPlayFlagEventEvent>(),
        ),
        (
            "TeamPlayWinPanel",
            std::mem::size_of::<TeamPlayWinPanelEvent>(),
        ),
        (
            "TeamPlayTeamBalancedPlayer",
            std::mem::size_of::<TeamPlayTeamBalancedPlayerEvent>(),
        ),
        (
            "TeamPlaySetupFinished",
            std::mem::size_of::<TeamPlaySetupFinishedEvent>(),
        ),
        ("TeamPlayAlert", std::mem::size_of::<TeamPlayAlertEvent>()),
        (
            "TrainingComplete",
            std::mem::size_of::<TrainingCompleteEvent>(),
        ),
        (
            "ShowFreezePanel",
            std::mem::size_of::<ShowFreezePanelEvent>(),
        ),
        (
            "HideFreezePanel",
            std::mem::size_of::<HideFreezePanelEvent>(),
        ),
        (
            "FreezeCamStarted",
            std::mem::size_of::<FreezeCamStartedEvent>(),
        ),
        (
            "LocalPlayerChangeTeam",
            std::mem::size_of::<LocalPlayerChangeTeamEvent>(),
        ),
        (
            "LocalPlayerScoreChanged",
            std::mem::size_of::<LocalPlayerScoreChangedEvent>(),
        ),
        (
            "LocalPlayerChangeClass",
            std::mem::size_of::<LocalPlayerChangeClassEvent>(),
        ),
        (
            "LocalPlayerRespawn",
            std::mem::size_of::<LocalPlayerRespawnEvent>(),
        ),
        (
            "BuildingInfoChanged",
            std::mem::size_of::<BuildingInfoChangedEvent>(),
        ),
        (
            "LocalPlayerChangeDisguise",
            std::mem::size_of::<LocalPlayerChangeDisguiseEvent>(),
        ),
        (
            "PlayerAccountChanged",
            std::mem::size_of::<PlayerAccountChangedEvent>(),
        ),
        ("SpyPdaReset", std::mem::size_of::<SpyPdaResetEvent>()),
        (
            "FlagStatusUpdate",
            std::mem::size_of::<FlagStatusUpdateEvent>(),
        ),
        (
            "PlayerStatsUpdated",
            std::mem::size_of::<PlayerStatsUpdatedEvent>(),
        ),
        (
            "PlayingCommentary",
            std::mem::size_of::<PlayingCommentaryEvent>(),
        ),
        (
            "PlayerChargeDeployed",
            std::mem::size_of::<PlayerChargeDeployedEvent>(),
        ),
        (
            "PlayerBuiltObject",
            std::mem::size_of::<PlayerBuiltObjectEvent>(),
        ),
        (
            "PlayerUpgradedObject",
            std::mem::size_of::<PlayerUpgradedObjectEvent>(),
        ),
        (
            "PlayerCarryObject",
            std::mem::size_of::<PlayerCarryObjectEvent>(),
        ),
        (
            "PlayerDropObject",
            std::mem::size_of::<PlayerDropObjectEvent>(),
        ),
        ("ObjectRemoved", std::mem::size_of::<ObjectRemovedEvent>()),
        (
            "ObjectDestroyed",
            std::mem::size_of::<ObjectDestroyedEvent>(),
        ),
        (
            "ObjectDetonated",
            std::mem::size_of::<ObjectDetonatedEvent>(),
        ),
        (
            "AchievementEarned",
            std::mem::size_of::<AchievementEarnedEvent>(),
        ),
        (
            "SpecTargetUpdated",
            std::mem::size_of::<SpecTargetUpdatedEvent>(),
        ),
        (
            "TournamentStateUpdate",
            std::mem::size_of::<TournamentStateUpdateEvent>(),
        ),
        (
            "TournamentEnableCountdown",
            std::mem::size_of::<TournamentEnableCountdownEvent>(),
        ),
        (
            "PlayerCalledForMedic",
            std::mem::size_of::<PlayerCalledForMedicEvent>(),
        ),
        (
            "PlayerAskedForBall",
            std::mem::size_of::<PlayerAskedForBallEvent>(),
        ),
        (
            "LocalPlayerBecameObserver",
            std::mem::size_of::<LocalPlayerBecameObserverEvent>(),
        ),
        (
            "PlayerIgnitedInv",
            std::mem::size_of::<PlayerIgnitedInvEvent>(),
        ),
        ("PlayerIgnited", std::mem::size_of::<PlayerIgnitedEvent>()),
        (
            "PlayerExtinguished",
            std::mem::size_of::<PlayerExtinguishedEvent>(),
        ),
        (
            "PlayerTeleported",
            std::mem::size_of::<PlayerTeleportedEvent>(),
        ),
        (
            "PlayerHealedMedicCall",
            std::mem::size_of::<PlayerHealedMedicCallEvent>(),
        ),
        (
            "LocalPlayerChargeReady",
            std::mem::size_of::<LocalPlayerChargeReadyEvent>(),
        ),
        (
            "LocalPlayerWindDown",
            std::mem::size_of::<LocalPlayerWindDownEvent>(),
        ),
        ("PlayerInvulned", std::mem::size_of::<PlayerInvulnedEvent>()),
        ("EscortSpeed", std::mem::size_of::<EscortSpeedEvent>()),
        ("EscortProgress", std::mem::size_of::<EscortProgressEvent>()),
        ("EscortRecede", std::mem::size_of::<EscortRecedeEvent>()),
        (
            "GameUIActivated",
            std::mem::size_of::<GameUIActivatedEvent>(),
        ),
        ("GameUIHidden", std::mem::size_of::<GameUIHiddenEvent>()),
        (
            "PlayerEscortScore",
            std::mem::size_of::<PlayerEscortScoreEvent>(),
        ),
        (
            "PlayerHealOnHit",
            std::mem::size_of::<PlayerHealOnHitEvent>(),
        ),
        (
            "PlayerStealSandvich",
            std::mem::size_of::<PlayerStealSandvichEvent>(),
        ),
        (
            "ShowClassLayout",
            std::mem::size_of::<ShowClassLayoutEvent>(),
        ),
        ("ShowVsPanel", std::mem::size_of::<ShowVsPanelEvent>()),
        ("PlayerDamaged", std::mem::size_of::<PlayerDamagedEvent>()),
        (
            "ArenaPlayerNotification",
            std::mem::size_of::<ArenaPlayerNotificationEvent>(),
        ),
        (
            "ArenaMatchMaxStreak",
            std::mem::size_of::<ArenaMatchMaxStreakEvent>(),
        ),
        (
            "ArenaRoundStart",
            std::mem::size_of::<ArenaRoundStartEvent>(),
        ),
        ("ArenaWinPanel", std::mem::size_of::<ArenaWinPanelEvent>()),
        ("PveWinPanel", std::mem::size_of::<PveWinPanelEvent>()),
        ("AirDash", std::mem::size_of::<AirDashEvent>()),
        ("Landed", std::mem::size_of::<LandedEvent>()),
        (
            "PlayerDamageDodged",
            std::mem::size_of::<PlayerDamageDodgedEvent>(),
        ),
        ("PlayerStunned", std::mem::size_of::<PlayerStunnedEvent>()),
        ("ScoutGrandSlam", std::mem::size_of::<ScoutGrandSlamEvent>()),
        (
            "ScoutSlamdollLanded",
            std::mem::size_of::<ScoutSlamdollLandedEvent>(),
        ),
        ("ArrowImpact", std::mem::size_of::<ArrowImpactEvent>()),
        ("PlayerJarated", std::mem::size_of::<PlayerJaratedEvent>()),
        (
            "PlayerJaratedFade",
            std::mem::size_of::<PlayerJaratedFadeEvent>(),
        ),
        (
            "PlayerShieldBlocked",
            std::mem::size_of::<PlayerShieldBlockedEvent>(),
        ),
        ("PlayerPinned", std::mem::size_of::<PlayerPinnedEvent>()),
        (
            "PlayerHealedByMedic",
            std::mem::size_of::<PlayerHealedByMedicEvent>(),
        ),
        (
            "PlayerSappedObject",
            std::mem::size_of::<PlayerSappedObjectEvent>(),
        ),
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
        (
            "DeployBuffBanner",
            std::mem::size_of::<DeployBuffBannerEvent>(),
        ),
        ("PlayerBuff", std::mem::size_of::<PlayerBuffEvent>()),
        ("MedicDeath", std::mem::size_of::<MedicDeathEvent>()),
        ("OvertimeNag", std::mem::size_of::<OvertimeNagEvent>()),
        ("TeamsChanged", std::mem::size_of::<TeamsChangedEvent>()),
        (
            "HalloweenPumpkinGrab",
            std::mem::size_of::<HalloweenPumpkinGrabEvent>(),
        ),
        ("RocketJump", std::mem::size_of::<RocketJumpEvent>()),
        (
            "RocketJumpLanded",
            std::mem::size_of::<RocketJumpLandedEvent>(),
        ),
        ("StickyJump", std::mem::size_of::<StickyJumpEvent>()),
        (
            "StickyJumpLanded",
            std::mem::size_of::<StickyJumpLandedEvent>(),
        ),
        (
            "RocketPackLaunch",
            std::mem::size_of::<RocketPackLaunchEvent>(),
        ),
        (
            "RocketPackLanded",
            std::mem::size_of::<RocketPackLandedEvent>(),
        ),
        ("MedicDefended", std::mem::size_of::<MedicDefendedEvent>()),
        (
            "LocalPlayerHealed",
            std::mem::size_of::<LocalPlayerHealedEvent>(),
        ),
        (
            "PlayerDestroyedPipeBomb",
            std::mem::size_of::<PlayerDestroyedPipeBombEvent>(),
        ),
        (
            "ObjectDeflected",
            std::mem::size_of::<ObjectDeflectedEvent>(),
        ),
        ("PlayerMvp", std::mem::size_of::<PlayerMvpEvent>()),
        ("RaidSpawnMob", std::mem::size_of::<RaidSpawnMobEvent>()),
        ("RaidSpawnSquad", std::mem::size_of::<RaidSpawnSquadEvent>()),
        ("NavBlocked", std::mem::size_of::<NavBlockedEvent>()),
        (
            "PathTrackPassed",
            std::mem::size_of::<PathTrackPassedEvent>(),
        ),
        (
            "NumCappersChanged",
            std::mem::size_of::<NumCappersChangedEvent>(),
        ),
        (
            "PlayerRegenerate",
            std::mem::size_of::<PlayerRegenerateEvent>(),
        ),
        (
            "UpdateStatusItem",
            std::mem::size_of::<UpdateStatusItemEvent>(),
        ),
        (
            "StatsResetRound",
            std::mem::size_of::<StatsResetRoundEvent>(),
        ),
        (
            "ScoreStatsAccumulatedUpdate",
            std::mem::size_of::<ScoreStatsAccumulatedUpdateEvent>(),
        ),
        (
            "ScoreStatsAccumulatedReset",
            std::mem::size_of::<ScoreStatsAccumulatedResetEvent>(),
        ),
        (
            "AchievementEarnedLocal",
            std::mem::size_of::<AchievementEarnedLocalEvent>(),
        ),
        ("PlayerHealed", std::mem::size_of::<PlayerHealedEvent>()),
        ("BuildingHealed", std::mem::size_of::<BuildingHealedEvent>()),
        ("ItemPickup", std::mem::size_of::<ItemPickupEvent>()),
        ("DuelStatus", std::mem::size_of::<DuelStatusEvent>()),
        ("FishNotice", std::mem::size_of::<FishNoticeEvent>()),
        ("FishNoticeArm", std::mem::size_of::<FishNoticeArmEvent>()),
        ("SlapNotice", std::mem::size_of::<SlapNoticeEvent>()),
        ("ThrowableHit", std::mem::size_of::<ThrowableHitEvent>()),
        (
            "PumpkinLordSummoned",
            std::mem::size_of::<PumpkinLordSummonedEvent>(),
        ),
        (
            "PumpkinLordKilled",
            std::mem::size_of::<PumpkinLordKilledEvent>(),
        ),
        (
            "MerasmusSummoned",
            std::mem::size_of::<MerasmusSummonedEvent>(),
        ),
        ("MerasmusKilled", std::mem::size_of::<MerasmusKilledEvent>()),
        (
            "MerasmusEscapeWarning",
            std::mem::size_of::<MerasmusEscapeWarningEvent>(),
        ),
        (
            "MerasmusEscaped",
            std::mem::size_of::<MerasmusEscapedEvent>(),
        ),
        (
            "EyeballBossSummoned",
            std::mem::size_of::<EyeballBossSummonedEvent>(),
        ),
        (
            "EyeballBossStunned",
            std::mem::size_of::<EyeballBossStunnedEvent>(),
        ),
        (
            "EyeballBossKilled",
            std::mem::size_of::<EyeballBossKilledEvent>(),
        ),
        (
            "EyeballBossKiller",
            std::mem::size_of::<EyeballBossKillerEvent>(),
        ),
        (
            "EyeballBossEscapeImminent",
            std::mem::size_of::<EyeballBossEscapeImminentEvent>(),
        ),
        (
            "EyeballBossEscaped",
            std::mem::size_of::<EyeballBossEscapedEvent>(),
        ),
        ("NpcHurt", std::mem::size_of::<NpcHurtEvent>()),
        (
            "ControlPointTimerUpdated",
            std::mem::size_of::<ControlPointTimerUpdatedEvent>(),
        ),
        (
            "PlayerHighFiveStart",
            std::mem::size_of::<PlayerHighFiveStartEvent>(),
        ),
        (
            "PlayerHighFiveCancel",
            std::mem::size_of::<PlayerHighFiveCancelEvent>(),
        ),
        (
            "PlayerHighFiveSuccess",
            std::mem::size_of::<PlayerHighFiveSuccessEvent>(),
        ),
        (
            "PlayerBonusPoints",
            std::mem::size_of::<PlayerBonusPointsEvent>(),
        ),
        ("PlayerUpgraded", std::mem::size_of::<PlayerUpgradedEvent>()),
        ("PlayerBuyback", std::mem::size_of::<PlayerBuybackEvent>()),
        (
            "PlayerUsedPowerUpBottle",
            std::mem::size_of::<PlayerUsedPowerUpBottleEvent>(),
        ),
        (
            "ChristmasGiftGrab",
            std::mem::size_of::<ChristmasGiftGrabEvent>(),
        ),
        (
            "PlayerKilledAchievementZone",
            std::mem::size_of::<PlayerKilledAchievementZoneEvent>(),
        ),
        ("PartyUpdated", std::mem::size_of::<PartyUpdatedEvent>()),
        (
            "PartyPrefChanged",
            std::mem::size_of::<PartyPrefChangedEvent>(),
        ),
        (
            "PartyCriteriaChanged",
            std::mem::size_of::<PartyCriteriaChangedEvent>(),
        ),
        (
            "PartyInvitesChanged",
            std::mem::size_of::<PartyInvitesChangedEvent>(),
        ),
        (
            "PartyQueueStateChanged",
            std::mem::size_of::<PartyQueueStateChangedEvent>(),
        ),
        ("PartyChat", std::mem::size_of::<PartyChatEvent>()),
        (
            "PartyMemberJoin",
            std::mem::size_of::<PartyMemberJoinEvent>(),
        ),
        (
            "PartyMemberLeave",
            std::mem::size_of::<PartyMemberLeaveEvent>(),
        ),
        (
            "MatchInvitesUpdated",
            std::mem::size_of::<MatchInvitesUpdatedEvent>(),
        ),
        ("LobbyUpdated", std::mem::size_of::<LobbyUpdatedEvent>()),
        (
            "MvmMissionUpdate",
            std::mem::size_of::<MvmMissionUpdateEvent>(),
        ),
        (
            "RecalculateHolidays",
            std::mem::size_of::<RecalculateHolidaysEvent>(),
        ),
        (
            "PlayerCurrencyChanged",
            std::mem::size_of::<PlayerCurrencyChangedEvent>(),
        ),
        (
            "DoomsdayRocketOpen",
            std::mem::size_of::<DoomsdayRocketOpenEvent>(),
        ),
        (
            "RemoveNemesisRelationships",
            std::mem::size_of::<RemoveNemesisRelationshipsEvent>(),
        ),
        (
            "MvmCreditBonusWave",
            std::mem::size_of::<MvmCreditBonusWaveEvent>(),
        ),
        (
            "MvmCreditBonusAll",
            std::mem::size_of::<MvmCreditBonusAllEvent>(),
        ),
        (
            "MvmCreditBonusAllAdvanced",
            std::mem::size_of::<MvmCreditBonusAllAdvancedEvent>(),
        ),
        (
            "MvmQuickSentryUpgrade",
            std::mem::size_of::<MvmQuickSentryUpgradeEvent>(),
        ),
        (
            "MvmTankDestroyedByPlayers",
            std::mem::size_of::<MvmTankDestroyedByPlayersEvent>(),
        ),
        (
            "MvmKillRobotDeliveringBomb",
            std::mem::size_of::<MvmKillRobotDeliveringBombEvent>(),
        ),
        (
            "MvmPickupCurrency",
            std::mem::size_of::<MvmPickupCurrencyEvent>(),
        ),
        (
            "MvmBombCarrierKilled",
            std::mem::size_of::<MvmBombCarrierKilledEvent>(),
        ),
        (
            "MvmSentryBusterDetonate",
            std::mem::size_of::<MvmSentryBusterDetonateEvent>(),
        ),
        (
            "MvmScoutMarkedForDeath",
            std::mem::size_of::<MvmScoutMarkedForDeathEvent>(),
        ),
        (
            "MvmMedicPowerUpShared",
            std::mem::size_of::<MvmMedicPowerUpSharedEvent>(),
        ),
        ("MvmBeginWave", std::mem::size_of::<MvmBeginWaveEvent>()),
        (
            "MvmWaveComplete",
            std::mem::size_of::<MvmWaveCompleteEvent>(),
        ),
        (
            "MvmMissionComplete",
            std::mem::size_of::<MvmMissionCompleteEvent>(),
        ),
        (
            "MvmBombResetByPlayer",
            std::mem::size_of::<MvmBombResetByPlayerEvent>(),
        ),
        (
            "MvmBombAlarmTriggered",
            std::mem::size_of::<MvmBombAlarmTriggeredEvent>(),
        ),
        (
            "MvmBombDeployResetByPlayer",
            std::mem::size_of::<MvmBombDeployResetByPlayerEvent>(),
        ),
        ("MvmWaveFailed", std::mem::size_of::<MvmWaveFailedEvent>()),
        ("MvmResetStats", std::mem::size_of::<MvmResetStatsEvent>()),
        ("DamageResisted", std::mem::size_of::<DamageResistedEvent>()),
        (
            "RevivePlayerNotify",
            std::mem::size_of::<RevivePlayerNotifyEvent>(),
        ),
        (
            "RevivePlayerStopped",
            std::mem::size_of::<RevivePlayerStoppedEvent>(),
        ),
        (
            "RevivePlayerComplete",
            std::mem::size_of::<RevivePlayerCompleteEvent>(),
        ),
        (
            "PlayerTurnedToGhost",
            std::mem::size_of::<PlayerTurnedToGhostEvent>(),
        ),
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
        (
            "MvmMannhattanPit",
            std::mem::size_of::<MvmMannhattanPitEvent>(),
        ),
        (
            "FlagCarriedInDetectionZone",
            std::mem::size_of::<FlagCarriedInDetectionZoneEvent>(),
        ),
        (
            "MvmAdvWaveKilledStunRadio",
            std::mem::size_of::<MvmAdvWaveKilledStunRadioEvent>(),
        ),
        (
            "PlayerDirectHitStun",
            std::mem::size_of::<PlayerDirectHitStunEvent>(),
        ),
        (
            "MvmSentryBusterKilled",
            std::mem::size_of::<MvmSentryBusterKilledEvent>(),
        ),
        (
            "UpgradesFileChanged",
            std::mem::size_of::<UpgradesFileChangedEvent>(),
        ),
        (
            "RdTeamPointsChanged",
            std::mem::size_of::<RdTeamPointsChangedEvent>(),
        ),
        (
            "RdRulesStateChanged",
            std::mem::size_of::<RdRulesStateChangedEvent>(),
        ),
        ("RdRobotKilled", std::mem::size_of::<RdRobotKilledEvent>()),
        ("RdRobotImpact", std::mem::size_of::<RdRobotImpactEvent>()),
        (
            "TeamPlayPreRoundTimeLeft",
            std::mem::size_of::<TeamPlayPreRoundTimeLeftEvent>(),
        ),
        (
            "ParachuteDeploy",
            std::mem::size_of::<ParachuteDeployEvent>(),
        ),
        (
            "ParachuteHolster",
            std::mem::size_of::<ParachuteHolsterEvent>(),
        ),
        (
            "KillRefillsMeter",
            std::mem::size_of::<KillRefillsMeterEvent>(),
        ),
        ("RpsTauntEvent", std::mem::size_of::<RpsTauntEventEvent>()),
        ("CongaKill", std::mem::size_of::<CongaKillEvent>()),
        (
            "PlayerInitialSpawn",
            std::mem::size_of::<PlayerInitialSpawnEvent>(),
        ),
        (
            "CompetitiveVictory",
            std::mem::size_of::<CompetitiveVictoryEvent>(),
        ),
        (
            "CompetitiveStatsUpdate",
            std::mem::size_of::<CompetitiveStatsUpdateEvent>(),
        ),
        ("MiniGameWin", std::mem::size_of::<MiniGameWinEvent>()),
        (
            "SentryOnGoActive",
            std::mem::size_of::<SentryOnGoActiveEvent>(),
        ),
        ("DuckXpLevelUp", std::mem::size_of::<DuckXpLevelUpEvent>()),
        ("QuestLogOpened", std::mem::size_of::<QuestLogOpenedEvent>()),
        ("SchemaUpdated", std::mem::size_of::<SchemaUpdatedEvent>()),
        (
            "LocalPlayerPickupWeapon",
            std::mem::size_of::<LocalPlayerPickupWeaponEvent>(),
        ),
        (
            "RdPlayerScorePoints",
            std::mem::size_of::<RdPlayerScorePointsEvent>(),
        ),
        (
            "DemomanDetStickies",
            std::mem::size_of::<DemomanDetStickiesEvent>(),
        ),
        (
            "QuestObjectiveCompleted",
            std::mem::size_of::<QuestObjectiveCompletedEvent>(),
        ),
        (
            "PlayerScoreChanged",
            std::mem::size_of::<PlayerScoreChangedEvent>(),
        ),
        (
            "KilledCappingPlayer",
            std::mem::size_of::<KilledCappingPlayerEvent>(),
        ),
        (
            "EnvironmentalDeath",
            std::mem::size_of::<EnvironmentalDeathEvent>(),
        ),
        (
            "ProjectileDirectHit",
            std::mem::size_of::<ProjectileDirectHitEvent>(),
        ),
        ("PassGet", std::mem::size_of::<PassGetEvent>()),
        ("PassScore", std::mem::size_of::<PassScoreEvent>()),
        ("PassFree", std::mem::size_of::<PassFreeEvent>()),
        ("PassPassCaught", std::mem::size_of::<PassPassCaughtEvent>()),
        ("PassBallStolen", std::mem::size_of::<PassBallStolenEvent>()),
        (
            "PassBallBlocked",
            std::mem::size_of::<PassBallBlockedEvent>(),
        ),
        (
            "DamagePrevented",
            std::mem::size_of::<DamagePreventedEvent>(),
        ),
        (
            "HalloweenBossKilled",
            std::mem::size_of::<HalloweenBossKilledEvent>(),
        ),
        (
            "EscapedLootIsland",
            std::mem::size_of::<EscapedLootIslandEvent>(),
        ),
        (
            "TaggedPlayerAsIt",
            std::mem::size_of::<TaggedPlayerAsItEvent>(),
        ),
        (
            "MerasmusStunned",
            std::mem::size_of::<MerasmusStunnedEvent>(),
        ),
        (
            "MerasmusPropFound",
            std::mem::size_of::<MerasmusPropFoundEvent>(),
        ),
        (
            "HalloweenSkeletonKilled",
            std::mem::size_of::<HalloweenSkeletonKilledEvent>(),
        ),
        (
            "SkeletonKilledQuest",
            std::mem::size_of::<SkeletonKilledQuestEvent>(),
        ),
        (
            "SkeletonKingKilledQuest",
            std::mem::size_of::<SkeletonKingKilledQuestEvent>(),
        ),
        ("EscapeHell", std::mem::size_of::<EscapeHellEvent>()),
        (
            "CrossSpectralBridge",
            std::mem::size_of::<CrossSpectralBridgeEvent>(),
        ),
        ("MiniGameWon", std::mem::size_of::<MiniGameWonEvent>()),
        ("RespawnGhost", std::mem::size_of::<RespawnGhostEvent>()),
        ("KillInHell", std::mem::size_of::<KillInHellEvent>()),
        (
            "HalloweenDuckCollected",
            std::mem::size_of::<HalloweenDuckCollectedEvent>(),
        ),
        ("SpecialScore", std::mem::size_of::<SpecialScoreEvent>()),
        (
            "TeamLeaderKilled",
            std::mem::size_of::<TeamLeaderKilledEvent>(),
        ),
        (
            "HalloweenSoulCollected",
            std::mem::size_of::<HalloweenSoulCollectedEvent>(),
        ),
        (
            "RecalculateTruce",
            std::mem::size_of::<RecalculateTruceEvent>(),
        ),
        (
            "DeadRingerCheatDeath",
            std::mem::size_of::<DeadRingerCheatDeathEvent>(),
        ),
        ("CrossbowHeal", std::mem::size_of::<CrossbowHealEvent>()),
        (
            "DamageMitigated",
            std::mem::size_of::<DamageMitigatedEvent>(),
        ),
        ("PayloadPushed", std::mem::size_of::<PayloadPushedEvent>()),
        (
            "PlayerAbandonedMatch",
            std::mem::size_of::<PlayerAbandonedMatchEvent>(),
        ),
        ("ClDrawline", std::mem::size_of::<ClDrawlineEvent>()),
        (
            "RestartTimerTime",
            std::mem::size_of::<RestartTimerTimeEvent>(),
        ),
        (
            "WinLimitChanged",
            std::mem::size_of::<WinLimitChangedEvent>(),
        ),
        (
            "WinPanelShowScores",
            std::mem::size_of::<WinPanelShowScoresEvent>(),
        ),
        (
            "TopStreamsRequestFinished",
            std::mem::size_of::<TopStreamsRequestFinishedEvent>(),
        ),
        (
            "CompetitiveStateChanged",
            std::mem::size_of::<CompetitiveStateChangedEvent>(),
        ),
        (
            "GlobalWarDataUpdated",
            std::mem::size_of::<GlobalWarDataUpdatedEvent>(),
        ),
        (
            "StopWatchChanged",
            std::mem::size_of::<StopWatchChangedEvent>(),
        ),
        ("DsStop", std::mem::size_of::<DsStopEvent>()),
        ("DsScreenshot", std::mem::size_of::<DsScreenshotEvent>()),
        (
            "ShowMatchSummary",
            std::mem::size_of::<ShowMatchSummaryEvent>(),
        ),
        (
            "ExperienceChanged",
            std::mem::size_of::<ExperienceChangedEvent>(),
        ),
        ("BeginXpLerp", std::mem::size_of::<BeginXpLerpEvent>()),
        (
            "MatchmakerStatsUpdated",
            std::mem::size_of::<MatchmakerStatsUpdatedEvent>(),
        ),
        (
            "RematchVotePeriodOver",
            std::mem::size_of::<RematchVotePeriodOverEvent>(),
        ),
        (
            "RematchFailedToCreate",
            std::mem::size_of::<RematchFailedToCreateEvent>(),
        ),
        (
            "PlayerRematchChange",
            std::mem::size_of::<PlayerRematchChangeEvent>(),
        ),
        ("PingUpdated", std::mem::size_of::<PingUpdatedEvent>()),
        ("MMStatsUpdated", std::mem::size_of::<MMStatsUpdatedEvent>()),
        (
            "PlayerNextMapVoteChange",
            std::mem::size_of::<PlayerNextMapVoteChangeEvent>(),
        ),
        (
            "VoteMapsChanged",
            std::mem::size_of::<VoteMapsChangedEvent>(),
        ),
        (
            "ProtoDefChanged",
            std::mem::size_of::<ProtoDefChangedEvent>(),
        ),
        (
            "PlayerDomination",
            std::mem::size_of::<PlayerDominationEvent>(),
        ),
        (
            "PlayerRocketPackPushed",
            std::mem::size_of::<PlayerRocketPackPushedEvent>(),
        ),
        ("QuestRequest", std::mem::size_of::<QuestRequestEvent>()),
        ("QuestResponse", std::mem::size_of::<QuestResponseEvent>()),
        ("QuestProgress", std::mem::size_of::<QuestProgressEvent>()),
        (
            "ProjectileRemoved",
            std::mem::size_of::<ProjectileRemovedEvent>(),
        ),
        (
            "QuestMapDataChanged",
            std::mem::size_of::<QuestMapDataChangedEvent>(),
        ),
        (
            "GasDousedPlayerIgnited",
            std::mem::size_of::<GasDousedPlayerIgnitedEvent>(),
        ),
        (
            "QuestTurnInState",
            std::mem::size_of::<QuestTurnInStateEvent>(),
        ),
        (
            "ItemsAcknowledged",
            std::mem::size_of::<ItemsAcknowledgedEvent>(),
        ),
        ("CapperKilled", std::mem::size_of::<CapperKilledEvent>()),
        (
            "MainMenuStabilized",
            std::mem::size_of::<MainMenuStabilizedEvent>(),
        ),
        (
            "WorldStatusChanged",
            std::mem::size_of::<WorldStatusChangedEvent>(),
        ),
        ("HLTVStatus", std::mem::size_of::<HLTVStatusEvent>()),
        ("HLTVCameraman", std::mem::size_of::<HLTVCameramanEvent>()),
        ("HLTVRankCamera", std::mem::size_of::<HLTVRankCameraEvent>()),
        ("HLTVRankEntity", std::mem::size_of::<HLTVRankEntityEvent>()),
        ("HLTVFixed", std::mem::size_of::<HLTVFixedEvent>()),
        ("HLTVChase", std::mem::size_of::<HLTVChaseEvent>()),
        ("HLTVMessage", std::mem::size_of::<HLTVMessageEvent>()),
        ("HLTVTitle", std::mem::size_of::<HLTVTitleEvent>()),
        ("HLTVChat", std::mem::size_of::<HLTVChatEvent>()),
        (
            "ReplayStartRecord",
            std::mem::size_of::<ReplayStartRecordEvent>(),
        ),
        (
            "ReplaySessionInfo",
            std::mem::size_of::<ReplaySessionInfoEvent>(),
        ),
        (
            "ReplayEndRecord",
            std::mem::size_of::<ReplayEndRecordEvent>(),
        ),
        (
            "ReplayReplaysAvailable",
            std::mem::size_of::<ReplayReplaysAvailableEvent>(),
        ),
        (
            "ReplayServerError",
            std::mem::size_of::<ReplayServerErrorEvent>(),
        ),
    ]
    .iter()
    .copied()
    .collect()
}
