//! Deserialize an instance of type `T` from bytes of JSON text.
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;

use serde::de::Deserialize;
use serde_json::Deserializer;

use crate::{Error, Result};

/// Like [`serde_json::from_slice`], but with a better error message.
///
/// # Errors
///
/// See [`serde_json::from_slice`].
pub fn from_slice<'s, T>(s: &'s [u8]) -> Result<'s, T>
where
    T: Deserialize<'s>,
{
    let mut de = Deserializer::from_slice(s);
    T::deserialize(&mut de).map_err(|err| Error {
        #[cfg(feature = "std")]
        src: s.into(),
        #[cfg(not(feature = "std"))]
        src: PhantomData,
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
        let _: Hi = from_slice(s.as_bytes()).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    mod eof {
        use super::*;

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn empty() {
            let s = r#""#;
            let _: Hi = from_slice(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn space() {
            let s = r#" "#;
            let _: Hi = from_slice(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn line() {
            let s = r#"
            "#;
            let _: Hi = from_slice(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing an object at "]
        fn abrupt() {
            let s = r#"{"#;
            let _: Hi = from_slice(s.as_bytes()).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }
    }
}
