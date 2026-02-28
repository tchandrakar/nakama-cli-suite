//! Inter-tool communication for the Nakama CLI Suite.
//!
//! Implements the Nakama Message Protocol (NMP) for piping structured data
//! between tools over stdin/stdout.

pub mod message;
pub mod pipe;

pub use message::{NmpMessage, NmpSource};
