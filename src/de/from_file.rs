//! Deserialize an instance of type `T` from a file by given path.
use std::path::Path;

use serde::de::DeserializeOwned;
use serde_json::Deserializer;

use crate::{Error, Result};

/// Like [`serde_json::from_reader`], but with a better error message and
/// specifically for files.
///
/// _Note_: This function reads whole contents of file by a `path` into a
/// [`String`].
///
/// # Errors
///
/// See [`serde_json::from_str`].
pub fn from_file<P, T>(path: P) -> Result<'static, T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let s = std::fs::read_to_string(path)?;
    let mut de = Deserializer::from_str(&s);
    T::deserialize(&mut de).map_err(|err| Error {
        src: s.into_bytes().into(),
        inner: err,
        filename: Some(path.to_owned()),
    })
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]

    use serde::Deserialize;

    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Deserialize)]
    struct Hi {
        hello: i32,
    }

    #[test]
    #[should_panic = "invalid type: map, expected i32 at "]
    fn wrong_type() {
        let _: Hi = from_file("test/file.json").unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}
