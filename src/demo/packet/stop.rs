use bitbuffer::{BitRead, LittleEndian};

use crate::{ReadResult, Stream};

#[derive(Debug)]
pub struct StopPacket;

impl BitRead<'_, LittleEndian> for StopPacket {
    fn read(_stream: &mut Stream) -> ReadResult<Self> {
        Ok(StopPacket)
    }
}
