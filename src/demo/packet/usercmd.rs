use crate::demo::data::DemoTick;
use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, LittleEndian};
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct UserCmdPacket {
    pub tick: DemoTick,
    pub sequence_out: u32,
    pub cmd: UserCmd,
}

impl<'a> BitRead<'a, LittleEndian> for UserCmdPacket {
    fn read(stream: &mut BitReadStream<'a, LittleEndian>) -> bitbuffer::Result<Self> {
        let tick = stream.read()?;
        let sequence_out = stream.read()?;
        let len: u32 = stream.read()?;
        let mut data = stream.read_bits(len as usize * 8)?;
        let cmd = data.read()?;
        Ok(UserCmdPacket {
            tick,
            sequence_out,
            cmd,
        })
    }
}

impl BitWrite<LittleEndian> for UserCmdPacket {
    fn write(&self, stream: &mut BitWriteStream<LittleEndian>) -> bitbuffer::Result<()> {
        self.tick.write(stream)?;
        self.sequence_out.write(stream)?;
        stream.reserve_byte_length(32, |stream| self.cmd.write(stream))
    }
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, BitRead, BitWrite, Serialize, Deserialize, Clone)]
pub struct UserCmd {
    pub command_number: Option<u32>,
    pub tick_count: Option<u32>,
    pub view_angles: [Option<f32>; 3],
    pub movement: [Option<f32>; 3],
    pub buttons: Option<u32>,
    pub impulse: Option<u8>,
    pub weapon_select: Option<WeaponSelect>,
    pub mouse_dx: Option<u16>,
    pub mouse_dy: Option<u16>,
}

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Debug, PartialEq, BitRead, BitWrite, Serialize, Deserialize, Clone)]
pub struct WeaponSelect {
    #[size = 11]
    select: u16,
    #[size = 6]
    subtype: Option<u8>,
}
