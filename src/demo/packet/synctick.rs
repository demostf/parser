use bitbuffer::BitRead;

#[derive(BitRead, Debug)]
pub struct SyncTickPacket {
    pub tick: u32,
}
