//! An easy way to print cause chains in errors.
//! Simply wrap your error type in ErrorChain and debug format it.
//!
//! Works great with thiserror's `#[from]`!
//!
//! ```rust
//! use sausage::ErrorChain;
//! use std::io;
//!
//! fn main_() -> Result<(), ErrorChain> {
//!     might_fail()?;
//!
//!     Ok(())
//! }
//!
//! fn might_fail() -> Result<(), io::Error> {
//!     Err(io::Error::new(io::ErrorKind::Other, "oh noes!"))
//! }
//! ```

#![warn(missing_docs, unreachable_pub)]

use std::{fmt::{self, Write}, error};

use indenter::indented;

/// Wraps an error, prints the error with all it's `sources`s.
pub struct ErrorChain(pub Box<dyn error::Error>);

impl<E: error::Error + 'static> From<E> for ErrorChain {
    fn from(err: E) -> Self {
        ErrorChain(Box::new(err))
    }
}

impl fmt::Debug for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;

        if let Some(source) = self.0.source() {
            write!(f, "\n\nCaused by:")?;

            let multi = source.source().is_some();

            let mut source = Some(source);
            let mut i = 0;
            while let Some(err) = source {
                let writer = indented(f).with_str("    ");
                let mut writer = if multi {
                    writer.ind(i)
                } else {
                    writer
                };
                write!(writer, "\n{}", err)?;
                i += 1;
                source = err.source();
            }
        }

        Ok(())
    }
}
