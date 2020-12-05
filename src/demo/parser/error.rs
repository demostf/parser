use crate::demo::gamevent::GameEventValueType;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{ClassId, SendTableName};
use bitbuffer::{BitError, FromUtf8Error};
use err_derive::Error;

/// Errors that can occur during parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error(display = "Error while reading bits from stream: {}", _0)]
    ReadError(#[error(source, no_from)] BitError),
    #[error(display = "Malformed utf8 while reading string")]
    MalformedUTF8(#[error(source)] FromUtf8Error),
    #[error(display = "Unexpected type of compressed data: {}", _0)]
    UnexpectedCompressionType(String),
    #[error(
        display = "Error while decompressing SNAP compressed string table: {}",
        _0
    )]
    SnapError(#[error(source)] snap::Error),
    #[error(
        display = "Unexpected size after decompressing SNAP data, got {} bytes, expected {} bytes",
        size,
        expected
    )]
    UnexpectedDecompressedSize {
        /// Expected decompressed size
        expected: u32,
        /// Actual decompressed size
        size: u32,
    },
    #[error(display = "Malformed demo file: {}", _0)]
    InvalidDemo(&'static str),
    #[error(display = "Packet identifier is invalid: {}", _0)]
    InvalidPacketType(u8),
    #[error(display = "Message identifier is invalid: {}", _0)]
    InvalidMessageType(u8),
    #[error(display = "Invalid SendProp type: {}", _0)]
    InvalidSendPropType(u8),
    #[error(display = "Invalid SendProp: {}", _0)]
    InvalidSendProp(#[error(source)] MalformedSendPropDefinitionError),
    #[error(
        display = "Unexpected amount of data left after parsing an object, {} bits remaining",
        _0
    )]
    DataRemaining(usize),
    #[error(display = "String table with index {} not found", _0)]
    StringTableNotFound(u8),
    #[error(display = "A malformed game event was read: {}", _0)]
    MalformedGameEvent(#[error(source)] GameEventError),
    #[error(
        display = "A read game event doesn't contain the expected values, expected type {} for {} event, got type {}",
        expected_type,
        name,
        found_type
    )]
    InvalidGameEvent {
        expected_type: GameEventValueType,
        name: &'static str,
        found_type: GameEventValueType,
    },
    #[error(display = "An entity with an unknown server class({}) was read", _0)]
    UnknownServerClass(ClassId),
    #[error(display = "Unknown send table: {}", _0)]
    UnknownSendTable(SendTableName),
    #[error(
        display = "Property index out of bounds, got {} but only {} props exist",
        _0,
        _1
    )]
    PropIndexOutOfBounds { index: i32, prop_count: usize },
    #[error(display = "An attempt was made to update an unknown entity: {}", _0)]
    UnknownEntity(EntityId),
}

#[derive(Debug, Error)]
pub enum MalformedSendPropDefinitionError {
    #[error(display = "Float property without defined size")]
    UnsizedFloat,
    #[error(display = "Array property without defined size")]
    UnsizedArray,
    #[error(display = "Array property without defined inner type")]
    UntypedArray,
    #[error(display = "Property used that can't be read")]
    InvalidPropType,
    #[error(display = "Array contents can't have the 'ChangesOften' flag")]
    ArrayChangesOften,
    #[error(display = "SendProp value out of range")]
    OutOfRange,
}

#[derive(Debug, Error)]
pub enum GameEventError {
    #[error(display = "Incorrect number of values")]
    IncorrectValueCount,
    #[error(display = "Event with 'none' value")]
    NoneValue,
    #[error(display = "Unknown type: {}", _0)]
    UnknownType(GameEventTypeId),
}

impl From<BitError> for ParseError {
    fn from(err: BitError) -> ParseError {
        match err {
            BitError::Utf8Error(utf8_error) => ParseError::MalformedUTF8(utf8_error),
            _ => ParseError::ReadError(err),
        }
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
