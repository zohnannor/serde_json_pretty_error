#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery, missing_docs)]
#![cfg_attr(not(debug_assertions), forbid(warnings))]

pub mod de;
pub mod error;

#[cfg(feature = "std")]
#[doc(inline)]
pub use de::from_reader::from_reader;
#[doc(inline)]
pub use de::{from_slice::from_slice, from_str::from_str};
#[doc(inline)]
pub use error::Error;

pub use serde_json::{
    self, from_value, json, map, to_string, to_string_pretty, to_value, to_vec, to_vec_pretty,
    value, Deserializer, Map, Number, StreamDeserializer, Value,
};
#[cfg(feature = "std")]
pub use serde_json::{ser, to_writer, to_writer_pretty, Serializer};

/// Alias for a [`Result`] with the error type
/// [`serde_json_pretty_error::Error`].
///
/// [`Result`]: core::result::Result
/// [`serde_json_pretty_error::Error`]: Error
pub type Result<'a, T, E = Error<'a>> = core::result::Result<T, E>;
