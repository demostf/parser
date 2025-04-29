use crate::demo::parser::{Encode, ParseBitSkip};
use crate::{Parse, ParserState, Result, Stream};
use bitbuffer::{BitRead, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ServerInfoMessage {
    pub version: u16,
    pub server_count: u32,
    pub stv: bool,
    pub dedicated: bool,
    pub max_crc: u32,
    pub max_classes: u16,
    pub map_hash: [u8; 16],
    pub player_slot: u8,
    pub max_player_count: u8,
    pub interval_per_tick: f32,
    pub platform: String,
    pub game: String,
    pub map: String,
    pub skybox: String,
    pub server_name: String,
    pub replay: bool,
}

impl<'a> Parse<'a> for ServerInfoMessage {
    fn parse(stream: &mut Stream<'a>, state: &ParserState) -> Result<Self> {
        let part1 = ServerInfoMessagePart1::read(stream)?;
        let map_hash = if state.protocol_version > 17 {
            <[u8; 16]>::read(stream)?
        } else {
            let mut hash = [0; 16];
            let crc = u32::read(stream)?;
            hash[0..4].copy_from_slice(&crc.to_le_bytes());
            hash
        };
        let part2 = ServerInfoMessagePart2::read(stream)?;
        let replay = if state.protocol_version > 15 {
            bool::read(stream)?
        } else {
            false
        };
        Ok(ServerInfoMessage {
            version: part1.version,
            server_count: part1.server_count,
            stv: part1.stv,
            dedicated: part1.dedicated,
            max_crc: part1.max_crc,
            max_classes: part1.max_classes,
            map_hash,
            player_slot: part2.player_slot,
            max_player_count: part2.max_player_count,
            interval_per_tick: part2.interval_per_tick,
            platform: part2.platform,
            game: part2.game,
            map: part2.map,
            skybox: part2.skybox,
            server_name: part2.server_name,
            replay,
        })
    }
}

impl<'a> ParseBitSkip<'a> for ServerInfoMessage {
    fn parse_skip(stream: &mut Stream<'a>, state: &ParserState) -> Result<()> {
        let version_dependent_size = match state.protocol_version {
            0..=15 => 4 * 8, // only the 4 byte crc
            16..=17 => 4 * 8 + 1, // adds the 1 bit replay flag
            18.. => 16 * 8 + 1, // replaces 4 byte crc with an 16 byte hash
        };
        let size = <ServerInfoMessagePart1 as BitRead<LittleEndian>>::bit_size().unwrap_or_default()
            + <ServerInfoMessagePart2 as BitRead<LittleEndian>>::bit_size().unwrap_or_default()
            + version_dependent_size;
        stream.skip_bits(size)?;
        Ok(())
    }
}

impl Encode for ServerInfoMessage {
    fn encode(&self, stream: &mut BitWriteStream<LittleEndian>, state: &ParserState) -> Result<()> {
        let part1 = ServerInfoMessagePart1 {
            version: self.version,
            server_count: self.server_count,
            stv: self.stv,
            dedicated: self.dedicated,
            max_crc: self.max_crc,
            max_classes: self.max_classes,
        };
        part1.write(stream)?;
        if state.protocol_version > 17 {
            self.map_hash.write(stream)?;
        } else {
            let crc = u32::from_le_bytes([
                self.map_hash[0],
                self.map_hash[1],
                self.map_hash[2],
                self.map_hash[3],
            ]);
            crc.write(stream)?;
        };
        let part2 = ServerInfoMessagePart2 {
            player_slot: self.player_slot,
            max_player_count: self.max_player_count,
            interval_per_tick: self.interval_per_tick,
            platform: self.platform.clone(),
            game: self.game.clone(),
            map: self.map.clone(),
            skybox: self.skybox.clone(),
            server_name: self.server_name.clone(),
        };
        part2.write(stream)?;
        if state.protocol_version > 15 {
            self.replay.write(stream)?;
        }
        Ok(())
    }
}

#[derive(BitRead, BitWrite)]
pub struct ServerInfoMessagePart1 {
    pub version: u16,
    pub server_count: u32,
    pub stv: bool,
    pub dedicated: bool,
    pub max_crc: u32,
    pub max_classes: u16,
}

#[derive(BitRead, BitWrite)]
pub struct ServerInfoMessagePart2 {
    pub player_slot: u8,
    pub max_player_count: u8,
    pub interval_per_tick: f32,
    #[size = 1]
    pub platform: String,
    pub game: String,
    pub map: String,
    pub skybox: String,
    pub server_name: String,
}
