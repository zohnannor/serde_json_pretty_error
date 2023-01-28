//! Deserialize JSON data to a Rust data structure.

#[cfg(feature = "std")]
pub mod from_reader;
pub mod from_slice;
pub mod from_str;
