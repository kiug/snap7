//! Rust bindings for the [Snap7](https://snap7.sourceforge.net/) S7
//! communication library.
//!
//! This crate provides two layers:
//!
//! - [`ffi`] — the raw, hand-written C API bindings (unsafe).
//! - [`client`], [`server`] and [`partner`] — safe, RAII wrappers that expose
//!   the client, server and partner components as idiomatic Rust types with
//!   `Result`-based error handling.
//!
//! The Snap7 C++ library is compiled and linked by `build.rs`.

pub mod client;
pub mod error;
pub mod ffi;
pub mod partner;
pub mod server;
pub mod types;

pub use client::Client;
pub use error::{Error, Result};
pub use partner::{Partner, PartnerStatus};
pub use server::Server;
pub use types::{
    Area, BlockInfo, BlockType, BlocksList, ConnectionType, CpInfo, CpuInfo, CpuStatus, DataItem,
    OrderCode, PlcDateTime, Protection, ServerArea, SrvEvent, WordLen,
};
