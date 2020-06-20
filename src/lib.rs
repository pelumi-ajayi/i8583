//! # ISO 8583 financial transaction message format library
//!
//! Types, associated functions and specifications for packing and unpacking
//! iso8583 messages.
//!
//! For most practical cases, iso messages are created to be sent
//! over the wire as an application level protocol on top of TCP,
//! for this reason all interfaces are in bytes([u8]) for easy use with sockets.
//! 
//! The crate relies heavily on slicing and borrowing references for performance,
//! cloning is left to user discretion. 

pub mod data;
pub mod v1987;
pub mod v1993;
pub mod nibss;
pub mod packer;
pub mod unpacker;

pub use crate::packer::Packer;
pub use crate::unpacker::Unpacker;

