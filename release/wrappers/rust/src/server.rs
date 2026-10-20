//! Safe wrapper around the Snap7 server (`S7Object` + `Srv_*` functions).
//!
//! [`Server`] owns the underlying Snap7 object and destroys it on drop. Areas
//! registered with [`Server::register_area`] are referenced (not copied) by
//! Snap7, so the caller must keep the supplied buffers alive until the area is
//! unregistered or the server is dropped.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

use crate::error::{check, Error, Result};
use crate::ffi;
use crate::types::{CpuStatus, ServerArea, SrvEvent};

/// A Snap7 server (simulated PLC).
pub struct Server {
    raw: ffi::S7Object,
}

impl Server {
    /// Creates a new server object.
    ///
    /// # Panics
    /// Panics if Snap7 fails to allocate the server (unrecoverable).
    pub fn create() -> Self {
        let raw = unsafe { ffi::Srv_Create() };
        assert_ne!(raw, 0, "Srv_Create returned a null object");
        Server { raw }
    }

    /// Returns the raw `S7Object` handle.
    pub fn raw(&self) -> ffi::S7Object {
        self.raw
    }

    // -- lifecycle -----------------------------------------------------------

    /// Starts the server listening on `address` (use `0.0.0.0` for all
    /// interfaces); the port must be set with [`set_param`](Self::set_param)
    /// or [`set_local_port`](Self::set_local_port) beforehand.
    pub fn start_to(&self, address: &str) -> Result<()> {
        let addr = CString::new(address).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        check(unsafe { ffi::Srv_StartTo(self.raw, addr.as_ptr()) })
    }

    /// Starts the server using the current parameters.
    pub fn start(&self) -> Result<()> {
        check(unsafe { ffi::Srv_Start(self.raw) })
    }

    /// Stops the server.
    pub fn stop(&self) -> Result<()> {
        check(unsafe { ffi::Srv_Stop(self.raw) })
    }

    // -- parameters ----------------------------------------------------------

    /// Sets an integer (`i32`) parameter.
    pub fn set_param_i32(&self, param_number: i32, value: i32) -> Result<()> {
        check(unsafe {
            ffi::Srv_SetParam(self.raw, param_number, &value as *const i32 as *mut c_void)
        })
    }

    /// Reads an integer (`i32`) parameter.
    pub fn get_param_i32(&self, param_number: i32) -> Result<i32> {
        let mut value: c_int = 0;
        check(unsafe {
            ffi::Srv_GetParam(
                self.raw,
                param_number,
                &mut value as *mut c_int as *mut c_void,
            )
        })?;
        Ok(value)
    }

    /// Sets a 16-bit (`u16`) parameter.
    pub fn set_param_u16(&self, param_number: i32, value: u16) -> Result<()> {
        check(unsafe {
            ffi::Srv_SetParam(self.raw, param_number, &value as *const u16 as *mut c_void)
        })
    }

    /// Reads a 16-bit (`u16`) parameter.
    pub fn get_param_u16(&self, param_number: i32) -> Result<u16> {
        let mut value: u16 = 0;
        check(unsafe {
            ffi::Srv_GetParam(
                self.raw,
                param_number,
                &mut value as *mut u16 as *mut c_void,
            )
        })?;
        Ok(value)
    }

    /// Sets the local TCP port the server listens on.
    pub fn set_local_port(&self, port: u16) -> Result<()> {
        self.set_param_u16(ffi::P_U16_LOCAL_PORT, port)
    }

    /// Sets the maximum number of simultaneous clients.
    pub fn set_max_clients(&self, max_clients: i32) -> Result<()> {
        self.set_param_i32(ffi::P_I32_MAX_CLIENTS, max_clients)
    }

    // -- areas ---------------------------------------------------------------

    /// Registers a memory area served to clients.
    ///
    /// # Safety
    /// Snap7 keeps a raw pointer to `data`; it must remain valid until the area
    /// is unregistered or the server is dropped.
    pub fn register_area(&self, area: ServerArea, index: u16, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Srv_RegisterArea(
                self.raw,
                area.code(),
                index,
                data.as_mut_ptr() as *mut c_void,
                data.len() as c_int,
            )
        })
    }

    /// Unregisters a previously registered area.
    pub fn unregister_area(&self, area: ServerArea, index: u16) -> Result<()> {
        check(unsafe { ffi::Srv_UnregisterArea(self.raw, area.code(), index) })
    }

    /// Locks an area against concurrent access from the server.
    pub fn lock_area(&self, area: ServerArea, index: u16) -> Result<()> {
        check(unsafe { ffi::Srv_LockArea(self.raw, area.code(), index) })
    }

    /// Unlocks a previously locked area.
    pub fn unlock_area(&self, area: ServerArea, index: u16) -> Result<()> {
        check(unsafe { ffi::Srv_UnlockArea(self.raw, area.code(), index) })
    }

    // -- status --------------------------------------------------------------

    /// Returns the server status, CPU status and client count.
    pub fn get_status(&self) -> Result<(i32, CpuStatus, i32)> {
        let mut server_status: c_int = 0;
        let mut cpu_status: c_int = 0;
        let mut clients: c_int = 0;
        check(unsafe {
            ffi::Srv_GetStatus(self.raw, &mut server_status, &mut cpu_status, &mut clients)
        })?;
        Ok((server_status, CpuStatus::from_code(cpu_status), clients))
    }

    /// Sets the CPU status.
    pub fn set_cpu_status(&self, status: CpuStatus) -> Result<()> {
        check(unsafe { ffi::Srv_SetCpuStatus(self.raw, status.code()) })
    }

    // -- events --------------------------------------------------------------

    /// Clears the event queue.
    pub fn clear_events(&self) -> Result<()> {
        check(unsafe { ffi::Srv_ClearEvents(self.raw) })
    }

    /// Picks the next event from the queue, returning `None` if it is empty.
    pub fn pick_event(&self) -> Result<Option<SrvEvent>> {
        let mut event = ffi::TSrvEvent::default();
        let mut ready: c_int = 0;
        check(unsafe { ffi::Srv_PickEvent(self.raw, &mut event, &mut ready) })?;
        Ok(if ready != 0 { Some(event.into()) } else { None })
    }

    // -- text ----------------------------------------------------------------

    /// Returns a human-readable description of a server error code.
    pub fn error_text(code: i32) -> String {
        let mut buffer = [0u8; 256];
        unsafe {
            ffi::Srv_ErrorText(
                code,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int,
            );
        }
        crate::types::c_string(&buffer)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        unsafe { ffi::Srv_Destroy(&mut self.raw) };
    }
}
