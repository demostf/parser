use bitbuffer::{BitRead, BitReadSized, LittleEndian};

use crate::demo::message::stringtable::log_base2;
use crate::{ReadResult, Stream};
use std::cmp::min;

#[derive(BitReadSized, Debug)]
pub struct ClassInfoEntry {
    #[size = "input_size"]
    class_id: u16,
    class_name: String,
    table_name: String,
}

#[derive(Debug)]
pub struct ClassInfoMessage {
    count: u16,
    create: bool,
    entries: Vec<ClassInfoEntry>,
}

impl BitRead<'_, LittleEndian> for ClassInfoMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let count: u16 = stream.read()?;
        let create: bool = stream.read()?;
        let entries = if !create {
            let mut entries = Vec::with_capacity(min(count, 128) as usize);
            let bits = log_base2(count);
            for _ in 0..count {
                entries.push(stream.read_sized(bits as usize)?);
            }
            entries
        } else {
            Vec::new()
        };

        Ok(ClassInfoMessage {
            count,
            create,
            entries,
        })
    }
}
