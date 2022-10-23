pub mod userinfo;

use bitbuffer::{BitRead, BitReadStream, BitWrite, BitWriteStream, Endianness};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub};

pub use userinfo::UserInfo;

#[derive(Eq, PartialEq, Clone)]
pub enum MaybeUtf8String {
    Valid(String),
    Invalid(Vec<u8>),
}

impl From<&'_ str> for MaybeUtf8String {
    fn from(str: &'_ str) -> Self {
        MaybeUtf8String::Valid(str.into())
    }
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
                stream.set_pos(stream.pos().saturating_sub(size * 8))?;
                let mut data: Vec<u8> = stream.read_sized(size)?;
                while data.last() == Some(&0) {
                    data.pop();
                }
                match String::from_utf8(data) {
                    Ok(str) => Ok(MaybeUtf8String::Valid(str)),
                    Err(e) => Ok(MaybeUtf8String::Invalid(e.into_bytes())),
                }
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

impl Serialize for MaybeUtf8String {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_ref().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for MaybeUtf8String {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).map(MaybeUtf8String::Valid)
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for MaybeUtf8String {
    fn schema_name() -> String {
        String::schema_name()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        String::json_schema(gen)
    }
}

/// Tick relative to the start of the game on the server
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    BitRead,
    BitWrite,
    Serialize,
    Deserialize,
    Default,
)]
pub struct ServerTick(u32);

impl ServerTick {
    pub fn range_inclusive(&self, till: Self) -> impl Iterator<Item = Self> {
        (self.0..=till.0).into_iter().map(Self::from)
    }
}

impl PartialEq<u32> for ServerTick {
    fn eq(&self, other: &u32) -> bool {
        *other == self.0
    }
}

impl PartialOrd<u32> for ServerTick {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        other.partial_cmp(&self.0)
    }
}

impl PartialEq<ServerTick> for u32 {
    fn eq(&self, other: &ServerTick) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<ServerTick> for u32 {
    fn partial_cmp(&self, other: &ServerTick) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<u32> for ServerTick {
    fn from(tick: u32) -> Self {
        ServerTick(tick)
    }
}

impl From<ServerTick> for u32 {
    fn from(tick: ServerTick) -> Self {
        tick.0
    }
}

impl Add<u32> for ServerTick {
    type Output = ServerTick;

    fn add(self, rhs: u32) -> Self::Output {
        ServerTick(self.0 + rhs)
    }
}

impl Add<ServerTick> for ServerTick {
    type Output = ServerTick;

    fn add(self, rhs: ServerTick) -> Self::Output {
        ServerTick(self.0 + rhs.0)
    }
}

impl Sub<u32> for ServerTick {
    type Output = ServerTick;

    fn sub(self, rhs: u32) -> Self::Output {
        ServerTick(self.0 - rhs)
    }
}

impl Sub<ServerTick> for ServerTick {
    type Output = ServerTick;

    fn sub(self, rhs: ServerTick) -> Self::Output {
        ServerTick(self.0 - rhs.0)
    }
}

/// Tick relative to the start of the demo
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[derive(
    Debug,
    Clone,
    Copy,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    BitRead,
    BitWrite,
    Serialize,
    Deserialize,
    Default,
)]
pub struct DemoTick(u32);

impl DemoTick {
    pub fn range_inclusive(&self, till: Self) -> impl Iterator<Item = Self> {
        (self.0..=till.0).into_iter().map(Self::from)
    }
}

impl PartialEq<u32> for DemoTick {
    fn eq(&self, other: &u32) -> bool {
        *other == self.0
    }
}

impl PartialOrd<u32> for DemoTick {
    fn partial_cmp(&self, other: &u32) -> Option<Ordering> {
        other.partial_cmp(&self.0)
    }
}

impl PartialEq<DemoTick> for u32 {
    fn eq(&self, other: &DemoTick) -> bool {
        self.eq(&other.0)
    }
}

impl PartialOrd<DemoTick> for u32 {
    fn partial_cmp(&self, other: &DemoTick) -> Option<Ordering> {
        self.partial_cmp(&other.0)
    }
}

impl From<u32> for DemoTick {
    fn from(tick: u32) -> Self {
        DemoTick(tick)
    }
}

impl From<DemoTick> for u32 {
    fn from(tick: DemoTick) -> Self {
        tick.0
    }
}

impl Add<u32> for DemoTick {
    type Output = DemoTick;

    fn add(self, rhs: u32) -> Self::Output {
        DemoTick(self.0 + rhs)
    }
}

impl Add<DemoTick> for DemoTick {
    type Output = DemoTick;

    fn add(self, rhs: DemoTick) -> Self::Output {
        DemoTick(self.0 + rhs.0)
    }
}

impl Sub<u32> for DemoTick {
    type Output = DemoTick;

    fn sub(self, rhs: u32) -> Self::Output {
        DemoTick(self.0 - rhs)
    }
}

impl Sub<DemoTick> for DemoTick {
    type Output = DemoTick;

    fn sub(self, rhs: DemoTick) -> Self::Output {
        DemoTick(self.0 - rhs.0)
    }
}
