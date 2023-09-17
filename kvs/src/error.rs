use std::{io, result};

use thiserror::Error;

/// Error struct
#[derive(Error, Debug)]
pub enum KvError {
    /// Io
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

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
}

/// Result shortcut to default result + KvError
pub type Result<T = ()> = result::Result<T, KvError>;
