use bitstream_reader::{BitRead, BitStream, LittleEndian};

use crate::{Parse, ParseError, ParserState, ReadResult, Result, Stream};

#[derive(Debug, Clone)]
pub struct VoiceInitMessage {
    codec: String,
    quality: u8,
    extra_data: u16,
}

impl BitRead<LittleEndian> for VoiceInitMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let codec = stream.read()?;
        let quality = stream.read()?;

        let extra_data = if quality == 255 {
            stream.read()?
        } else if codec == "vaudio_celt" {
            11025
        } else {
            0
        };

        Ok(VoiceInitMessage {
            codec,
            quality,
            extra_data,
        })
    }
}

#[derive(BitRead, Debug, Clone)]
#[endianness = "LittleEndian"]
pub struct VoiceDataMessage {
    client: u8,
    proximity: u8,
    length: u16,
    #[size = "length"]
    data: Stream,
}

#[derive(Debug, Clone)]
pub struct ParseSoundsMessage {
    pub reliable: bool,
    pub num: u8,
    pub length: u16,
    pub data: Stream,
}

impl BitRead<LittleEndian> for ParseSoundsMessage {
    fn read(stream: &mut Stream) -> ReadResult<Self> {
        let reliable = stream.read()?;
        let num = if reliable { 1u8 } else { stream.read()? };
        let length = if reliable { stream.read_sized::<u16>(8)? } else { stream.read()? };
        let data = stream.read_sized(length as usize)?;

        Ok(ParseSoundsMessage {
            reliable,
            num,
            length,
            data,
        })
    }
}