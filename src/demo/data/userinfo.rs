use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::stringtable::{ExtraData, StringTableEntry};
use crate::demo::parser::gamestateanalyser::UserId;
use crate::{ReadResult, Stream};
use bitbuffer::{
    BitReadBuffer, BitReadStream, BitWrite, BitWriteSized, BitWriteStream, LittleEndian,
};

#[derive(Clone, Debug)]
pub struct UserInfo {
    pub steam_id: String,
    pub user_id: UserId,
    pub entity_id: EntityId,
    pub name: String,
    pub rest_data: Vec<u8>,
}

impl UserInfo {
    pub fn parse_from_string_table(
        text: Option<&str>,
        data: Option<Stream>,
    ) -> ReadResult<Option<Self>> {
        if let Some(mut data) = data {
            let name: String = data
                .read_sized(32)
                .unwrap_or_else(|_| "Malformed Name".into());
            let user_id: UserId = data.read::<u32>()?.into();
            let steam_id: String = data.read_sized(32)?;

            let rest_data = data.read_sized(data.bits_left() / 8)?;

            match text
                .map(|text| text.parse::<u32>().map(|id| (id + 1).into()))
                .unwrap_or_else(|| Ok((user_id.0 as u32).into()))
            {
                Ok(entity_id) if !steam_id.is_empty() => Ok(Some(UserInfo {
                    steam_id,
                    user_id,
                    entity_id,
                    name,
                    rest_data,
                })),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }

    pub fn encode_to_string_table(&self) -> ReadResult<StringTableEntry<'static>> {
        let text = format!("{}", self.entity_id);
        let mut extra_data = Vec::with_capacity(128);
        {
            let mut stream = BitWriteStream::new(&mut extra_data, LittleEndian);
            self.name.write_sized(&mut stream, 32)?;
            (self.user_id.0 as u32).write(&mut stream)?;
            self.steam_id.write_sized(&mut stream, 32)?;
            self.rest_data.write(&mut stream)?;
        }

        Ok(StringTableEntry {
            text: Some(text.into()),
            extra_data: Some(ExtraData::new(BitReadStream::new(
                BitReadBuffer::new_owned(extra_data, LittleEndian),
            ))),
        })
    }
}
