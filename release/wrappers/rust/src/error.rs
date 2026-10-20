//! Error type and [`Result`] alias for the safe Snap7 wrapper.

use std::fmt;

/// A Snap7 error code as returned by the C API (`0` means success).
///
/// Snap7 reports errors as plain `int` codes: `0` is success, negative values
/// are library-level failures and positive values are component-specific
/// errors. This type is a thin, copyable wrapper around that code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error(i32);

impl Error {
    /// Creates an [`Error`] from a raw C return code.
    pub const fn new(code: i32) -> Self {
        Error(code)
    }

    /// Returns the raw error code.
    pub const fn code(self) -> i32 {
        self.0
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 < 0 {
            write!(f, "Snap7 error ({})", self.0)
        } else {
            write!(f, "Snap7 error 0x{:08X}", self.0)
        }
    }
}

impl std::error::Error for Error {}

/// The [`Result`] type used by every fallible Snap7 operation.
pub type Result<T> = std::result::Result<T, Error>;

/// Converts a raw C return code into a `Result<(), Error>`.
///
/// `0` maps to `Ok(())`; any other value maps to `Err(Error(code))`.
pub(crate) fn check(code: i32) -> Result<()> {
    if code == 0 {
        Ok(())
    } else {
        Err(Error(code))
    }
}
