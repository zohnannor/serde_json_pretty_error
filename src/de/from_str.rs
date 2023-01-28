//! Deserialize an instance of type `T` from a string of JSON text.
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;

use serde::Deserialize;
use serde_json::Deserializer;

use crate::{Error, Result};

/// Like [`serde_json::from_str`], but with a better error message.
///
/// # Errors
///
/// See [`serde_json::from_str`].
pub fn from_str<'s, T>(s: &'s str) -> Result<T>
where
    T: Deserialize<'s>,
{
    let mut de = Deserializer::from_str(s);
    T::deserialize(&mut de).map_err(|err| Error {
        #[cfg(feature = "std")]
        src: s.as_bytes().into(),
        #[cfg(not(feature = "std"))]
        src: PhantomData,
        inner: err,
    })
}

#[cfg(test)]
mod tests {

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
        let _: Hi = from_str(s).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    mod eof {
        use super::*;

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn empty() {
            let s = r#""#;
            let _: Hi = from_str(s).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn space() {
            let s = r#" "#;
            let _: Hi = from_str(s).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing a value at "]
        fn line() {
            let s = r#"
            "#;
            let _: Hi = from_str(s).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }

        #[test]
        #[should_panic = "EOF while parsing an object at "]
        fn abrupt() {
            let s = r#"{"#;
            let _: Hi = from_str(s).unwrap_or_else(|err| {
                panic!("{}", err);
            });
        }
    }
}
