#![deny(missing_docs)]
//! Implementation of client/server for KV engines – default/sled

pub use client::KvClient;
pub use engines::{KvEngine, KvStore, SledKvEngine};
pub use error::{KvError, Result};
pub use server::KvServer;

mod client;
mod common;
mod engines;
mod error;
mod server;
