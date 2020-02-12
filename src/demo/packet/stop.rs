use bitbuffer::{BitRead, LittleEndian};

use crate::{ReadResult, Stream};

#[derive(Debug)]
pub struct StopPacket;

impl BitRead<LittleEndian> for StopPacket {
    fn read(_stream: &mut Stream) -> ReadResult<Self> {
        Ok(StopPacket)
    }
}
