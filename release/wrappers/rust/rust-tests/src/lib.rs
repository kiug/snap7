//! Rust unit tests for the [Snap7](https://snap7.sourceforge.net/) S7
//! communication library.
//!
//! The tests drive the library through the safe wrapper and the raw FFI
//! re-exported here from the [`snap7`] crate (`../release/wrappers/rust`).

// Re-export the whole wrapper (modules and types) so the tests can refer to
// `crate::ffi`, `crate::Client`, `crate::Server`, etc.
pub use snap7::*;

#[cfg(test)]
mod tests;
