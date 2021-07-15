use bitbuffer::{BitRead, BitReadStream, Endianness};

use crate::ReadResult;

#[derive(Debug)]
pub struct ConVar {
    key: String,
    value: String,
}

impl<E: Endianness> BitRead<'_, E> for ConVar {
    fn read(stream: &mut BitReadStream<'_, E>) -> ReadResult<Self> {
        let key = stream
            .read()
            .unwrap_or_else(|_| "Malformed cvar name".to_string());
        let value = stream
            .read()
            .unwrap_or_else(|_| "Malformed cvar value".to_string());
        Ok(ConVar { key, value })
    }
}

#[derive(Debug, BitRead)]
pub struct SetConVarMessage {
    length: u8,
    #[size = "length"]
    vars: Vec<ConVar>,
}
