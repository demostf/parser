use bitstream_reader::BitRead;

#[derive(BitRead, Debug)]
pub struct SyncTickPacket {
    pub tick: u32,
}
