use bitbuffer::{BitRead, BitReadSized, BitWrite, BitWriteSized, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

use crate::demo::message::stringtable::log_base2;
use crate::{ReadResult, Stream};
use std::cmp::min;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(BitReadSized, BitWriteSized, Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct ClassInfoEntry {
    #[size = "input_size"]
    class_id: u16,
    class_name: String,
    table_name: String,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
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
            let bits = log_base2(count) + 1;
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

impl BitWrite<LittleEndian> for ClassInfoMessage {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        self.count.write(stream)?;
        self.create.write(stream)?;
        if !self.create {
            let bits = log_base2(self.entries.len()) as usize + 1;
            for entry in self.entries.iter() {
                entry.write_sized(stream, bits)?;
            }
        }

        Ok(())
    }
}

#[test]
fn test_class_info_roundtrip() {
    crate::test_roundtrip_write(ClassInfoMessage {
        count: 8,
        create: true,
        entries: Vec::new(),
    });
    crate::test_roundtrip_write(ClassInfoMessage {
        count: 3,
        create: false,
        entries: vec![
            ClassInfoEntry {
                class_id: 0,
                class_name: "foo1".to_string(),
                table_name: "bar1".to_string(),
            },
            ClassInfoEntry {
                class_id: 1,
                class_name: "foo2".to_string(),
                table_name: "bar2".to_string(),
            },
            ClassInfoEntry {
                class_id: 2,
                class_name: "foo3".to_string(),
                table_name: "bar3".to_string(),
            },
        ],
    });
}
