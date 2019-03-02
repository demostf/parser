use bitstream_reader::{BitRead, BitReadSized, LittleEndian};

use crate::{ReadResult, Stream};

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

impl BitRead<LittleEndian> for ClassInfoMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let count: u16 = stream.read()?;
        let create = stream.read()?;
        let mut entries = Vec::with_capacity(count as usize);
        let bits = 16 - count.leading_zeros();
        for _ in 0..count {
            entries.push(stream.read_sized(bits as usize)?);
        }

        Ok(ClassInfoMessage {
            count,
            create,
            entries,
        })
    }
}
