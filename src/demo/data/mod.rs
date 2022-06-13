pub mod userinfo;

use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, Endianness};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

pub use userinfo::UserInfo;

#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(Eq, PartialEq, Serialize, Deserialize, Clone)]
pub enum MaybeUtf8String {
    Valid(String),
    Invalid(Vec<u8>),
}

impl Default for MaybeUtf8String {
    fn default() -> Self {
        MaybeUtf8String::Valid(String::new())
    }
}

impl AsRef<str> for MaybeUtf8String {
    fn as_ref(&self) -> &str {
        match self {
            MaybeUtf8String::Valid(s) => s.as_str(),
            MaybeUtf8String::Invalid(_) => "-- Malformed utf8 --",
        }
    }
}

impl Debug for MaybeUtf8String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeUtf8String::Valid(s) => Debug::fmt(s, f),
            MaybeUtf8String::Invalid(b) => f
                .debug_struct("MaybeUtf8String::Invalid")
                .field("data", b)
                .finish(),
        }
    }
}

impl Display for MaybeUtf8String {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeUtf8String::Valid(s) => Display::fmt(s, f),
            MaybeUtf8String::Invalid(_) => write!(f, "-- Malformed utf8 --"),
        }
    }
}

impl MaybeUtf8String {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            MaybeUtf8String::Valid(s) => s.as_bytes(),
            MaybeUtf8String::Invalid(b) => b.as_ref(),
        }
    }
}

impl<'a, E: Endianness> BitRead<'a, E> for MaybeUtf8String {
    fn read(stream: &mut BitReadStream<'a, E>) -> bitbuffer::Result<Self> {
        match String::read(stream) {
            Ok(str) => Ok(MaybeUtf8String::Valid(str)),
            Err(bitbuffer::BitError::Utf8Error(_, size)) => {
                stream.set_pos(stream.pos() - size * 8)?;
                let data = stream.read_sized(size)?;
                Ok(MaybeUtf8String::Invalid(data))
            }
            Err(e) => Err(e),
        }
    }
}

impl<E: Endianness> BitWrite<E> for MaybeUtf8String {
    fn write(&self, stream: &mut BitWriteStream<E>) -> bitbuffer::Result<()> {
        stream.write_bytes(self.as_bytes())?;
        stream.write(&0u8)
    }
}

impl Into<String> for MaybeUtf8String {
    fn into(self) -> String {
        match self {
            MaybeUtf8String::Valid(s) => s,
            MaybeUtf8String::Invalid(_) => "-- Malformed utf8 --".into(),
        }
    }
}
