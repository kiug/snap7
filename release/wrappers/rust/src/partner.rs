//! Safe wrapper around the Snap7 partner (`S7Object` + `Par_*` functions).
//!
//! [`Partner`] owns the underlying Snap7 object and destroys it on drop. An
//! *active* partner initiates the connection; a *passive* partner listens (on
//! the privileged port 102 by default, which requires `CAP_NET_BIND_SERVICE`).

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

use crate::error::{check, Error, Result};
use crate::ffi;

/// Partner link status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartnerStatus {
    Stopped,
    Connecting,
    Waiting,
    Linked,
    Sending,
    Receiving,
    BindError,
}

impl PartnerStatus {
    fn from_code(code: i32) -> Self {
        match code {
            ffi::PAR_STOPPED => PartnerStatus::Stopped,
            ffi::PAR_CONNECTING => PartnerStatus::Connecting,
            ffi::PAR_WAITING => PartnerStatus::Waiting,
            ffi::PAR_LINKED => PartnerStatus::Linked,
            ffi::PAR_SENDING => PartnerStatus::Sending,
            ffi::PAR_RECEIVING => PartnerStatus::Receiving,
            ffi::PAR_BINDERROR => PartnerStatus::BindError,
            _ => PartnerStatus::Stopped,
        }
    }
}

/// A Snap7 partner.
pub struct Partner {
    raw: ffi::S7Object,
}

impl Partner {
    /// Creates a new partner. Pass `true` for an active partner (connects out)
    /// or `false` for a passive partner (listens).
    ///
    /// # Panics
    /// Panics if Snap7 fails to allocate the partner (unrecoverable).
    pub fn create(active: bool) -> Self {
        let raw = unsafe { ffi::Par_Create(if active { 1 } else { 0 }) };
        assert_ne!(raw, 0, "Par_Create returned a null object");
        Partner { raw }
    }

    /// Returns the raw `S7Object` handle.
    pub fn raw(&self) -> ffi::S7Object {
        self.raw
    }

    // -- lifecycle -----------------------------------------------------------

    /// Starts the partner with explicit local/remote addresses and TSAPs.
    pub fn start_to(
        &self,
        local_address: &str,
        remote_address: &str,
        local_tsap: u16,
        remote_tsap: u16,
    ) -> Result<()> {
        let local =
            CString::new(local_address).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        let remote =
            CString::new(remote_address).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        check(unsafe {
            ffi::Par_StartTo(
                self.raw,
                local.as_ptr(),
                remote.as_ptr(),
                local_tsap,
                remote_tsap,
            )
        })
    }

    /// Starts the partner using the current parameters.
    pub fn start(&self) -> Result<()> {
        check(unsafe { ffi::Par_Start(self.raw) })
    }

    /// Stops the partner.
    pub fn stop(&self) -> Result<()> {
        check(unsafe { ffi::Par_Stop(self.raw) })
    }

    // -- parameters ----------------------------------------------------------

    /// Sets an integer (`i32`) parameter.
    pub fn set_param_i32(&self, param_number: i32, value: i32) -> Result<()> {
        check(unsafe {
            ffi::Par_SetParam(self.raw, param_number, &value as *const i32 as *mut c_void)
        })
    }

    /// Reads an integer (`i32`) parameter.
    pub fn get_param_i32(&self, param_number: i32) -> Result<i32> {
        let mut value: c_int = 0;
        check(unsafe {
            ffi::Par_GetParam(
                self.raw,
                param_number,
                &mut value as *mut c_int as *mut c_void,
            )
        })?;
        Ok(value)
    }

    /// Sets the remote TCP port (active partner).
    pub fn set_remote_port(&self, port: u16) -> Result<()> {
        check(unsafe {
            ffi::Par_SetParam(
                self.raw,
                ffi::P_U16_REMOTE_PORT,
                &port as *const u16 as *mut c_void,
            )
        })
    }

    // -- data transfer -------------------------------------------------------

    /// Sends a block of data to the linked partner.
    pub fn b_send(&self, r_id: u32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Par_BSend(
                self.raw,
                r_id,
                data.as_ptr() as *mut c_void,
                data.len() as c_int,
            )
        })
    }

    /// Starts an asynchronous send.
    pub fn as_b_send(&self, r_id: u32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Par_AsBSend(
                self.raw,
                r_id,
                data.as_ptr() as *mut c_void,
                data.len() as c_int,
            )
        })
    }

    /// Returns `true` when a pending asynchronous send has completed.
    pub fn check_as_b_send_completion(&self) -> Result<bool> {
        let mut op_result: c_int = 0;
        let rc = unsafe { ffi::Par_CheckAsBSendCompletion(self.raw, &mut op_result) };
        match rc {
            ffi::JOB_COMPLETE => Ok(true),
            ffi::JOB_PENDING => Ok(false),
            code => Err(Error::new(code)),
        }
    }

    /// Waits up to `timeout_ms` for a pending asynchronous send.
    pub fn wait_as_b_send_completion(&self, timeout_ms: u32) -> Result<()> {
        check(unsafe { ffi::Par_WaitAsBSendCompletion(self.raw, timeout_ms) })
    }

    /// Receives a block of data from the linked partner.
    ///
    /// `data` is a pre-allocated buffer; on success its first `n` bytes (the
    /// returned length) contain the received payload.
    pub fn b_recv(&self, data: &mut [u8], timeout_ms: u32) -> Result<(u32, usize)> {
        let mut r_id: u32 = 0;
        let mut size = data.len() as c_int;
        check(unsafe {
            ffi::Par_BRecv(
                self.raw,
                &mut r_id,
                data.as_mut_ptr() as *mut c_void,
                &mut size,
                timeout_ms,
            )
        })?;
        Ok((r_id, size as usize))
    }

    // -- status / stats ------------------------------------------------------

    /// Returns the partner link status.
    pub fn status(&self) -> Result<PartnerStatus> {
        let mut status: c_int = 0;
        check(unsafe { ffi::Par_GetStatus(self.raw, &mut status) })?;
        Ok(PartnerStatus::from_code(status))
    }

    /// Returns the last partner error code.
    pub fn last_error(&self) -> Result<i32> {
        let mut error: c_int = 0;
        check(unsafe { ffi::Par_GetLastError(self.raw, &mut error) })?;
        Ok(error)
    }

    /// Returns the send/receive times of the last block transfer (µs).
    pub fn get_times(&self) -> Result<(u32, u32)> {
        let mut send_time: u32 = 0;
        let mut recv_time: u32 = 0;
        check(unsafe { ffi::Par_GetTimes(self.raw, &mut send_time, &mut recv_time) })?;
        Ok((send_time, recv_time))
    }

    /// Returns cumulative transfer statistics.
    pub fn get_stats(&self) -> Result<(u32, u32, u32, u32)> {
        let mut sent: u32 = 0;
        let mut recv: u32 = 0;
        let mut send_errors: u32 = 0;
        let mut recv_errors: u32 = 0;
        check(unsafe {
            ffi::Par_GetStats(
                self.raw,
                &mut sent,
                &mut recv,
                &mut send_errors,
                &mut recv_errors,
            )
        })?;
        Ok((sent, recv, send_errors, recv_errors))
    }

    // -- text ----------------------------------------------------------------

    /// Returns a human-readable description of a partner error code.
    pub fn error_text(code: i32) -> String {
        let mut buffer = [0u8; 256];
        unsafe {
            ffi::Par_ErrorText(
                code,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int,
            );
        }
        crate::types::c_string(&buffer)
    }
}

impl Drop for Partner {
    fn drop(&mut self) {
        unsafe { ffi::Par_Destroy(&mut self.raw) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_from_code() {
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_STOPPED),
            PartnerStatus::Stopped
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_CONNECTING),
            PartnerStatus::Connecting
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_WAITING),
            PartnerStatus::Waiting
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_LINKED),
            PartnerStatus::Linked
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_SENDING),
            PartnerStatus::Sending
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_RECEIVING),
            PartnerStatus::Receiving
        );
        assert_eq!(
            PartnerStatus::from_code(ffi::PAR_BINDERROR),
            PartnerStatus::BindError
        );
        assert_eq!(PartnerStatus::from_code(0x1234), PartnerStatus::Stopped);
    }
}
