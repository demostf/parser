use crate::demo::sendprop::{read_bit_coord, write_bit_coord};
use crate::demo::vector::Vector;
use crate::{ReadResult, Stream};
use bitbuffer::{BitRead, BitWrite, BitWriteSized, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BSPDecalMessage {
    pub position: Vector,
    pub texture_index: u16,
    pub ent_index: u16,
    pub model_index: u16,
    pub low_priority: bool,
}

impl BitRead<'_, LittleEndian> for BSPDecalMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let position = {
            let (has_x, has_y, has_z) = stream.read()?;

            Vector {
                x: if has_x { read_bit_coord(stream)? } else { 0f32 },
                y: if has_y { read_bit_coord(stream)? } else { 0f32 },
                z: if has_z { read_bit_coord(stream)? } else { 0f32 },
            }
        };

        let texture_index = stream.read_sized(9)?;
        let (ent_index, model_index): (u16, u16) = if stream.read()? {
            (stream.read_sized(11)?, stream.read_sized(13)?)
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

impl BitWrite<LittleEndian> for BSPDecalMessage {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> ReadResult<()> {
        let has_x = self.position.x != 0.0;
        let has_y = self.position.y != 0.0;
        let has_z = self.position.z != 0.0;
        (has_x, has_y, has_z).write(stream)?;

        if has_x {
            write_bit_coord(self.position.x, stream)?;
        }
        if has_y {
            write_bit_coord(self.position.y, stream)?;
        }
        if has_z {
            write_bit_coord(self.position.z, stream)?;
        }
        self.texture_index.write_sized(stream, 9)?;
        if self.ent_index != 0 || self.model_index != 0 {
            true.write(stream)?;
            self.ent_index.write_sized(stream, 11)?;
            self.model_index.write_sized(stream, 13)?;
        } else {
            false.write(stream)?;
        }
        self.low_priority.write(stream)?;

        Ok(())
    }
}

#[test]
fn test_decal_roundtrip() {
    crate::test_roundtrip_write(BSPDecalMessage {
        position: Vector::default(),
        texture_index: 0,
        ent_index: 0,
        model_index: 0,
        low_priority: false,
    });
    crate::test_roundtrip_write(BSPDecalMessage {
        position: Vector {
            x: 1.0,
            y: 0.5,
            z: 0.0,
        },
        texture_index: 12,
        ent_index: 15,
        model_index: 2,
        low_priority: true,
    });
}
