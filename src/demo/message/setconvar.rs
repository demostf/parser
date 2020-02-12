use bitbuffer::{BitRead, BitReadSized, LittleEndian};

use crate::demo::message::stringtable::log_base2;
use crate::{ReadResult, Stream};
use std::cmp::min;

#[derive(Debug)]
pub struct SetConVarMessage {
    vars: Vec<(String, String)>,
}

impl BitRead<LittleEndian> for SetConVarMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let count: u8 = stream.read()?;
        let mut vars: Vec<(String, String)> = Vec::with_capacity(min(count, 128) as usize);
        for _ in 0..count {
            let key = stream
                .read()
                .unwrap_or_else(|_| "Malformed cvar name".to_string());
            let value = stream
                .read()
                .unwrap_or_else(|_| "Malformed cvar value".to_string());
            vars.push((key, value));
        }
        Ok(SetConVarMessage { vars })
    }
}
