use crate::demo::gamevent::GameEventValueType;
use crate::demo::message::gameevent::GameEventTypeId;
use crate::demo::message::packetentities::EntityId;
use crate::demo::packet::datatable::{ClassId, SendTableName};
use crate::demo::sendprop::{SendPropIdentifier, SendPropValue};
use bitbuffer::BitError;
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error;

/// Errors that can occur during parsing
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Error while reading bits from stream: {0}")]
    ReadError(#[source] BitError),
    #[error("Malformed utf8 while reading string")]
    MalformedUTF8(#[from] Utf8Error),
    #[error("Unexpected type of compressed data: {0}")]
    UnexpectedCompressionType(String),
    #[error("Error while decompressing SNAP compressed string table: {0}")]
    SnapError(#[from] snap::Error),
    #[error(
        "Unexpected size after decompressing SNAP data, got {} bytes, expected {} bytes",
        size,
        expected
    )]
    UnexpectedDecompressedSize {
        /// Expected decompressed size
        expected: u32,
        /// Actual decompressed size
        size: u32,
    },
    #[error("Malformed demo file: {0}")]
    InvalidDemo(&'static str),
    #[error("Packet identifier is invalid: {0}")]
    InvalidPacketType(u8),
    #[error("Message identifier is invalid: {0}")]
    InvalidMessageType(u8),
    #[error("Invalid SendProp type: {0}")]
    InvalidSendPropType(u8),
    #[error("Invalid SendProp: {0}")]
    InvalidSendProp(#[from] MalformedSendPropDefinitionError),
    #[error("Unexpected amount of data left after parsing an object, {0} bits remaining")]
    DataRemaining(usize),
    #[error("String table with index {0} not found")]
    StringTableNotFound(u8),
    #[error("A malformed game event was read: {0}")]
    MalformedGameEvent(#[from] GameEventError),
    #[error(
        "A read game event doesn't contain the expected values, expected type {expected_type} for {name} event, got type {found_type}"
    )]
    InvalidGameEvent {
        expected_type: GameEventValueType,
        name: &'static str,
        found_type: GameEventValueType,
    },
    #[error("Game event of type {ty} does not contain a {field} value")]
    MissingGameEventValue { ty: &'static str, field: String },
    #[error("An entity with an unknown server class({0}) was read")]
    UnknownServerClass(ClassId),
    #[error("Unknown send table: {}", _0)]
    UnknownSendTable(SendTableName),
    #[error("Property index out of bounds, got {index} but only {prop_count} props exist")]
    PropIndexOutOfBounds {
        index: i32,
        prop_count: usize,
        table: String,
    },
    #[error("An attempt was made to update an unknown entity: {0}")]
    UnknownEntity(EntityId),
    #[error("No sendprop definition found for property")]
    UnknownDefinition(SendPropIdentifier),
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum MalformedSendPropDefinitionError {
    #[error("Float property without defined size")]
    UnsizedFloat,
    #[error("Array property without defined size")]
    UnsizedArray,
    #[error("Array property without defined inner type")]
    UntypedArray,
    #[error("Property used that can't be read")]
    InvalidPropType,
    #[error("Array contents can't have the 'ChangesOften' flag")]
    ArrayChangesOften,
    #[error("SendProp value out of range")]
    OutOfRange,
    #[error("Wrong prop value type for definition")]
    WrongPropType {
        expected: &'static str,
        value: SendPropValue,
    },
}

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum GameEventError {
    #[error("Incorrect number of values")]
    IncorrectValueCount,
    #[error("Event with 'none' value")]
    NoneValue,
    #[error("Unknown type: {0}")]
    UnknownType(GameEventTypeId),
}

impl From<BitError> for ParseError {
    fn from(err: BitError) -> ParseError {
        match err {
            BitError::Utf8Error(utf8_error, _) => ParseError::MalformedUTF8(utf8_error),
            _ => ParseError::ReadError(err),
        }
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> ParseError {
        ParseError::MalformedUTF8(err.utf8_error())
    }
}

pub type Result<T> = std::result::Result<T, ParseError>;
