//! Shared test helpers and module declarations.

mod client;
mod integration;
mod partner;
mod server;
mod text;
mod wrapper;

use std::ffi::CString;
use std::sync::Mutex;

use crate::ffi;

/// Serializes tests that bind network ports. Snap7 uses a fixed local/remote
/// port (default 102, overridden in tests to a high port), so only one
/// network test may run at a time.
pub static NET_LOCK: Mutex<()> = Mutex::new(());

/// Acquires the network lock, recovering from a poisoned mutex so that a panic
/// in one network test does not cascade into unrelated tests.
pub fn lock_net() -> std::sync::MutexGuard<'static, ()> {
    NET_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

/// High, unprivileged port used by the client/server integration tests.
pub const TEST_PORT: u16 = 4102;

/// Returns a `CString` for a Rust string literal (panics on interior NUL).
pub fn cstr(s: &str) -> CString {
    CString::new(s).expect("string must not contain NUL bytes")
}

/// RAII guard that destroys a Snap7 object when dropped.
pub struct Object {
    raw: ffi::S7Object,
    /// Function used to destroy the object.
    destroy: unsafe extern "C" fn(*mut ffi::S7Object),
}

impl Object {
    pub fn new(raw: ffi::S7Object, destroy: unsafe extern "C" fn(*mut ffi::S7Object)) -> Self {
        Object { raw, destroy }
    }

    pub fn raw(&self) -> ffi::S7Object {
        self.raw
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        let mut raw = self.raw;
        if raw != 0 {
            unsafe { (self.destroy)(&mut raw) };
        }
    }
}

/// Creates a client and returns a guard that destroys it on drop.
pub fn client() -> Object {
    let raw = unsafe { ffi::Cli_Create() };
    assert_ne!(raw, 0, "Cli_Create must return a valid object");
    Object::new(raw, ffi::Cli_Destroy)
}

/// Creates a server and returns a guard that destroys it on drop.
pub fn server() -> Object {
    let raw = unsafe { ffi::Srv_Create() };
    assert_ne!(raw, 0, "Srv_Create must return a valid object");
    Object::new(raw, ffi::Srv_Destroy)
}

/// Creates a partner and returns a guard that destroys it on drop.
pub fn partner(active: bool) -> Object {
    let raw = unsafe { ffi::Par_Create(if active { 1 } else { 0 }) };
    assert_ne!(raw, 0, "Par_Create must return a valid object");
    Object::new(raw, ffi::Par_Destroy)
}

/// Reads a NUL-terminated C string into a Rust `String`.
pub fn read_c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}
