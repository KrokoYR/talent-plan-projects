use std::io;
use std::net::AddrParseError;
use std::result;
use std::string::FromUtf8Error;

use thiserror::Error;

/// Error struct
#[derive(Error, Debug)]
pub enum KvError {
    /// Io
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Parsing address error
    #[error("Parsing address error")]
    AddrParseErr(#[from] AddrParseError),

    /// BsonSer
    #[error("BSON serialization error: {0}")]
    BsonSer(#[from] bson::ser::Error),

    /// BsonDeser
    #[error("BSON deserialization error: {0}")]
    BsonDeser(#[from] bson::de::Error),

    /// NotFound
    #[error("Key not found")]
    NotFound,

    /// Unexpected command
    #[error("Unexpected command type")]
    UnexpectedCommandType,

    /// Sled
    #[error("Sled error: {0}")]
    Sled(#[from] sled::Error),

    /// Utf8
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),

    /// String Error
    #[error("{0}")]
    StringError(String),
}

/// Result shortcut to default result + KvError
pub type Result<T = ()> = result::Result<T, KvError>;
