//! When serializing or deserializing JSON goes wrong.

use core::fmt;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::{borrow::Cow, error, io};

/// Shim for [`serde_json::Error`] which keeps source around for better error
/// messages.
///
/// Represents all possible errors that can occur when serializing or
/// deserializing JSON data.
pub struct Error<'s> {
    #[cfg(feature = "std")]
    pub(crate) src: Cow<'s, [u8]>,
    #[cfg(not(feature = "std"))]
    pub(crate) src: PhantomData<&'s ()>,
    pub(crate) inner: serde_json::Error,
}

impl fmt::Debug for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[cfg(feature = "std")]
        let src: Vec<_> = self.src.iter().copied().take(20).collect();
        #[cfg(feature = "std")]
        let src = String::from_utf8_lossy(&src);

        let mut debug_struct = f.debug_struct("Error");
        #[cfg(feature = "std")]
        debug_struct.field("src", &format!("(up to 20 bytes) {src}"));
        debug_struct.field("inner", &self.inner).finish()
    }
}

impl fmt::Display for Error<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use serde_json::error::Category;

        let err = &self.inner;

        err.fmt(f)?;

        if err.classify() == Category::Io {
            return Ok(());
        }

        #[cfg(feature = "std")]
        {
            let (line, column) = (err.line(), err.column());
            let line_num = line + 1;
            let gutter = line_num.to_string().len();
            let content = self
                .src
                .split(|&b| b == b'\n')
                .nth(line - 1)
                .expect("valid line number");
            let content = String::from_utf8_lossy(content);

            writeln!(f)?;
            //   |
            for _ in 0..=gutter {
                write!(f, " ")?;
            }
            writeln!(f, "|")?;

            // 1 | 00:32:00.a999999
            write!(f, "{line_num} | ")?;
            writeln!(f, "{content}")?;

            //   |          ^
            for _ in 0..=gutter {
                write!(f, " ")?;
            }
            write!(f, "|")?;
            for _ in 0..=column {
                write!(f, " ")?;
            }
            write!(f, "^")?;
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
impl error::Error for Error<'_> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.inner.source()
    }
}

#[cfg(feature = "std")]
impl From<Error<'_>> for io::Error {
    fn from(error: Error) -> Self {
        error.inner.into()
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error<'_> {
    fn from(value: io::Error) -> Self {
        Self {
            src: [].as_slice().into(),
            inner: serde_json::Error::io(value),
        }
    }
}
