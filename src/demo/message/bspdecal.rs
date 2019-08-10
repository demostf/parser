use bitstream_reader::{BitRead, BitSkip, LittleEndian};

use crate::demo::sendprop::read_bit_coord;
use crate::demo::vector::Vector;
use crate::{ReadResult, Stream};

#[derive(Debug, Clone)]
pub struct BSPDecalMessage {
    pub position: Vector,
    pub texture_index: u16,
    pub ent_index: u16,
    pub model_index: u16,
    pub low_priority: bool,
}

impl BitRead<LittleEndian> for BSPDecalMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let position = {
            let has_x = stream.read()?;
            let has_y = stream.read()?;
            let has_z = stream.read()?;

            Vector {
                x: if has_x { read_bit_coord(stream)? } else { 0f32 },
                y: if has_y { read_bit_coord(stream)? } else { 0f32 },
                z: if has_z { read_bit_coord(stream)? } else { 0f32 },
            }
        };

        let texture_index = stream.read_sized(9)?;
        let (ent_index, model_index): (u16, u16) = if stream.read()? {
            (stream.read_sized(11)?, stream.read_sized(12)?)
        } else {
            (0, 0)
        };
        let low_priority = stream.read()?;

        Ok(BSPDecalMessage {
            position,
            texture_index,
            ent_index,
            model_index,
            low_priority,
        })
    }
}

impl BitSkip<LittleEndian> for BSPDecalMessage {}
