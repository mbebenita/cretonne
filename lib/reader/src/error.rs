//! Define the `Location`, `Error`, and `Result` types.

#![macro_use]

use std::fmt;
use std::result;

/// The location of a `Token` or `Error`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Location {
    /// Line number, starting from 1.
    pub line_number: usize,
}

/// A parse error is returned when the parse failed.
#[derive(Debug)]
pub struct Error {
    /// Location of the error.
    pub location: Location,
    /// Error message.
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.location.line_number, self.message)
    }
}

/// Result of a parser operation. The `Error` variant includes a location.
pub type Result<T> = result::Result<T, Error>;

// Create an `Err` variant of `Result<X>` from a location and `format!` args.
macro_rules! err {
    ( $loc:expr, $msg:expr ) => {
        Err($crate::Error {
            location: $loc.clone(),
            message: String::from($msg),
        })
    };

    ( $loc:expr, $fmt:expr, $( $arg:expr ),+ ) => {
        Err($crate::Error {
            location: $loc.clone(),
            message: format!( $fmt, $( $arg ),+ ),
        })
    };
}
