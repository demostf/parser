use crate::demo::message::packetentities::EntityId;
use crate::demo::parser::gamestateanalyser::UserId;
use crate::{ReadResult, Stream};

pub struct UserInfo {
    pub steam_id: String,
    pub user_id: UserId,
    pub entity_id: EntityId,
    pub name: String,
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
            let steam_id: String = data.read()?;

            match text
                .map(|text| text.parse::<u32>().map(|id| (id + 1).into()))
                .unwrap_or_else(|| Ok((user_id.0 as u32).into()))
            {
                Ok(entity_id) if !steam_id.is_empty() => Ok(Some(UserInfo {
                    steam_id,
                    user_id,
                    entity_id,
                    name,
                })),
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
