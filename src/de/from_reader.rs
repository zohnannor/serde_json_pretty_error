//! Deserialize an instance of type `T` from an IO stream of JSON.
use std::io::Read;

use serde::de::DeserializeOwned;
use serde_json::Deserializer;

use crate::{Error, Result};

/// Like [`serde_json::from_reader`], but with a better error message.
///
/// _Note_: This function reads whole contents of `reader` into a [`String`].
///
/// # Errors
///
/// See [`serde_json::from_reader`].
pub fn from_reader<R, T>(reader: R) -> Result<'static, T>
where
    R: Read,
    T: DeserializeOwned,
{
    let s = std::io::read_to_string(reader)?;
    let mut de = Deserializer::from_str(&s);
    T::deserialize(&mut de).map_err(|err| Error {
        src: s.into_bytes().into(),
        inner: err,
        filename: None,
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
        let s = r#"{
            "hello": {}
        }"#;
        let _: Hi = from_reader(s.as_bytes()).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    mod eof {
        use super::*;

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn empty() {
            let s = r#""#;
            let _: Hi = from_reader(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn space() {
            let s = r#" "#;
            let _: Hi = from_reader(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn line() {
            let s = r#"
            "#;
            let _: Hi = from_reader(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing an object at "]
        fn abrupt() {
            let s = r#"{"#;
            let _: Hi = from_reader(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }
    }
}
