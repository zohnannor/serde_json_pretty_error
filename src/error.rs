//! When serializing or deserializing JSON goes wrong.

use core::fmt;
#[cfg(not(feature = "std"))]
use core::marker::PhantomData;
#[cfg(feature = "std")]
use std::{borrow::Cow, error, io, path::PathBuf};

#[cfg(feature = "colors")]
use owo_colors::{OwoColorize, Style};

/// Shim for [`serde_json::Error`] which keeps source around for better error
/// messages.
///
/// Represents all possible errors that can occur when serializing or
/// deserializing JSON data.
pub struct Error<'s> {
    #[cfg(feature = "std")]
    /// Bytes of the source of deserialization.
    pub(crate) src: Cow<'s, [u8]>,

    #[cfg(not(feature = "std"))]
    pub(crate) src: PhantomData<&'s ()>,

    #[cfg(feature = "std")]
    /// Name of the file.
    pub(crate) filename: Option<PathBuf>,

    /// Original error from [`serde_json`].
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
        let err = &self.inner;

        if err.is_io() {
            return err.fmt(f);
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

            let bar = "|";
            #[cfg(feature = "colors")]
            let bar = bar.style(Style::new().bold().blue());

            let arrow = "-->";
            #[cfg(feature = "colors")]
            let arrow = arrow.style(Style::new().bold().blue());

            let caret = "^";
            #[cfg(feature = "colors")]
            let caret = caret.style(Style::new().bold().red());

            #[cfg(feature = "colors")]
            let err = err.style(Style::new().bold().red());

            //     --> dir/file.json:111:222
            if let Some(filename) = &self.filename {
                let filename = filename.display();
                writeln!(f, "{: >gutter$}{arrow} {filename}", " ")?;
            }

            //      |
            writeln!(f, "{: >gutter$} {bar}", " ")?;

            // line |         "hello": 42,
            writeln!(f, "{line_num: >gutter$} {bar} {content}")?;

            //      |                  ^invalid type: i32, expected string at line 111 column 222
            writeln!(f, "{: >gutter$} {bar}{caret: >column$} {err}", " ")?;
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
            filename: None,
        }
    }
}
