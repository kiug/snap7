//! Safe wrapper around the Snap7 client (`S7Object` + `Cli_*` functions).
//!
//! [`Client`] owns the underlying Snap7 object and destroys it on drop. Every
//! fallible operation returns [`Result`](crate::error::Result); the raw object
//! handle is still available through [`Client::raw`] for anything not covered
//! here.

use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_void};

use crate::error::{check, Error, Result};
use crate::ffi;
use crate::types::{
    Area, BlockInfo, BlockType, BlocksList, ConnectionType, CpInfo, CpuInfo, CpuStatus, DataItem,
    OrderCode, PlcDateTime, Protection, WordLen,
};

/// A Snap7 client (PLC connection).
pub struct Client {
    raw: ffi::S7Object,
}

impl Client {
    /// Creates a new client object.
    ///
    /// # Panics
    /// Panics if Snap7 fails to allocate the client (unrecoverable).
    pub fn create() -> Self {
        let raw = unsafe { ffi::Cli_Create() };
        assert_ne!(raw, 0, "Cli_Create returned a null object");
        Client { raw }
    }

    /// Returns the raw `S7Object` handle.
    pub fn raw(&self) -> ffi::S7Object {
        self.raw
    }

    // -- lifecycle -----------------------------------------------------------

    /// Connects to the PLC at `address` using the given rack and slot.
    pub fn connect_to(&self, address: &str, rack: i32, slot: i32) -> Result<()> {
        let addr = CString::new(address).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        check(unsafe { ffi::Cli_ConnectTo(self.raw, addr.as_ptr(), rack, slot) })
    }

    /// Connects using the parameters previously set with
    /// [`set_connection_params`](Self::set_connection_params).
    pub fn connect(&self) -> Result<()> {
        check(unsafe { ffi::Cli_Connect(self.raw) })
    }

    /// Sets the connection parameters directly (local/remote TSAP).
    pub fn set_connection_params(
        &self,
        address: &str,
        local_tsap: u16,
        remote_tsap: u16,
    ) -> Result<()> {
        let addr = CString::new(address).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        check(unsafe {
            ffi::Cli_SetConnectionParams(self.raw, addr.as_ptr(), local_tsap, remote_tsap)
        })
    }

    /// Sets the connection type.
    pub fn set_connection_type(&self, connection_type: ConnectionType) -> Result<()> {
        check(unsafe { ffi::Cli_SetConnectionType(self.raw, connection_type.code()) })
    }

    /// Disconnects from the PLC.
    pub fn disconnect(&self) -> Result<()> {
        check(unsafe { ffi::Cli_Disconnect(self.raw) })
    }

    /// Returns `true` if the client is currently connected.
    pub fn is_connected(&self) -> Result<bool> {
        let mut connected: c_int = 0;
        check(unsafe { ffi::Cli_GetConnected(self.raw, &mut connected) })?;
        Ok(connected != 0)
    }

    // -- parameters ----------------------------------------------------------

    /// Sets an integer (`i32`) parameter.
    pub fn set_param_i32(&self, param_number: i32, value: i32) -> Result<()> {
        check(unsafe {
            ffi::Cli_SetParam(self.raw, param_number, &value as *const i32 as *mut c_void)
        })
    }

    /// Reads an integer (`i32`) parameter.
    pub fn get_param_i32(&self, param_number: i32) -> Result<i32> {
        let mut value: c_int = 0;
        check(unsafe {
            ffi::Cli_GetParam(
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
            ffi::Cli_SetParam(self.raw, param_number, &value as *const u16 as *mut c_void)
        })
    }

    /// Reads a 16-bit (`u16`) parameter.
    pub fn get_param_u16(&self, param_number: i32) -> Result<u16> {
        let mut value: u16 = 0;
        check(unsafe {
            ffi::Cli_GetParam(
                self.raw,
                param_number,
                &mut value as *mut u16 as *mut c_void,
            )
        })?;
        Ok(value)
    }

    /// Sets the remote (PLC) TCP port.
    pub fn set_remote_port(&self, port: u16) -> Result<()> {
        self.set_param_u16(ffi::P_U16_REMOTE_PORT, port)
    }

    /// Returns the remote (PLC) TCP port.
    pub fn remote_port(&self) -> Result<u16> {
        self.get_param_u16(ffi::P_U16_REMOTE_PORT)
    }

    /// Sets the requested PDU length (bytes).
    pub fn set_pdu_request(&self, size: i32) -> Result<()> {
        self.set_param_i32(ffi::P_I32_PDU_REQUEST, size)
    }

    /// Returns the requested PDU length (bytes).
    pub fn pdu_request(&self) -> Result<i32> {
        self.get_param_i32(ffi::P_I32_PDU_REQUEST)
    }

    /// Sets the receive timeout (milliseconds).
    pub fn set_recv_timeout(&self, timeout_ms: i32) -> Result<()> {
        self.set_param_i32(ffi::P_I32_RECV_TIMEOUT, timeout_ms)
    }

    /// Sets the send timeout (milliseconds).
    pub fn set_send_timeout(&self, timeout_ms: i32) -> Result<()> {
        self.set_param_i32(ffi::P_I32_SEND_TIMEOUT, timeout_ms)
    }

    /// Sets the ping timeout (milliseconds).
    pub fn set_ping_timeout(&self, timeout_ms: i32) -> Result<()> {
        self.set_param_i32(ffi::P_I32_PING_TIMEOUT, timeout_ms)
    }

    // -- data I/O ------------------------------------------------------------

    /// Reads `amount` elements of `word_len` from `area` into `data`.
    pub fn read_area(
        &self,
        area: Area,
        db_number: i32,
        start: i32,
        amount: i32,
        word_len: WordLen,
        data: &mut [u8],
    ) -> Result<()> {
        let required = amount as usize * word_len.size();
        assert!(
            data.len() >= required,
            "buffer too small: need {required} bytes, got {}",
            data.len()
        );
        check(unsafe {
            ffi::Cli_ReadArea(
                self.raw,
                area.code(),
                db_number,
                start,
                amount,
                word_len.code(),
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Writes `amount` elements of `word_len` from `data` to `area`.
    pub fn write_area(
        &self,
        area: Area,
        db_number: i32,
        start: i32,
        amount: i32,
        word_len: WordLen,
        data: &[u8],
    ) -> Result<()> {
        let required = amount as usize * word_len.size();
        assert!(
            data.len() >= required,
            "buffer too small: need {required} bytes, got {}",
            data.len()
        );
        check(unsafe {
            ffi::Cli_WriteArea(
                self.raw,
                area.code(),
                db_number,
                start,
                amount,
                word_len.code(),
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Reads `data.len()` bytes from a data block.
    pub fn db_read(&self, db_number: i32, start: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_DBRead(
                self.raw,
                db_number,
                start,
                data.len() as c_int,
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Writes `data.len()` bytes to a data block.
    pub fn db_write(&self, db_number: i32, start: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_DBWrite(
                self.raw,
                db_number,
                start,
                data.len() as c_int,
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Reads merkers.
    pub fn mb_read(&self, start: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_MBRead(
                self.raw,
                start,
                data.len() as c_int,
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Writes merkers.
    pub fn mb_write(&self, start: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_MBWrite(
                self.raw,
                start,
                data.len() as c_int,
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Reads process inputs.
    pub fn eb_read(&self, start: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_EBRead(
                self.raw,
                start,
                data.len() as c_int,
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Writes process inputs.
    pub fn eb_write(&self, start: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_EBWrite(
                self.raw,
                start,
                data.len() as c_int,
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Reads process outputs.
    pub fn ab_read(&self, start: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_ABRead(
                self.raw,
                start,
                data.len() as c_int,
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Writes process outputs.
    pub fn ab_write(&self, start: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_ABWrite(
                self.raw,
                start,
                data.len() as c_int,
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Reads timers.
    pub fn tm_read(&self, start: i32, amount: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe { ffi::Cli_TMRead(self.raw, start, amount, data.as_mut_ptr() as *mut c_void) })
    }

    /// Writes timers.
    pub fn tm_write(&self, start: i32, amount: i32, data: &[u8]) -> Result<()> {
        check(unsafe { ffi::Cli_TMWrite(self.raw, start, amount, data.as_ptr() as *mut c_void) })
    }

    /// Reads counters.
    pub fn ct_read(&self, start: i32, amount: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe { ffi::Cli_CTRead(self.raw, start, amount, data.as_mut_ptr() as *mut c_void) })
    }

    /// Writes counters.
    pub fn ct_write(&self, start: i32, amount: i32, data: &[u8]) -> Result<()> {
        check(unsafe { ffi::Cli_CTWrite(self.raw, start, amount, data.as_ptr() as *mut c_void) })
    }

    /// Reads multiple variables in a single request.
    pub fn read_multi_vars(&self, items: &mut [DataItem<'_>]) -> Result<()> {
        let mut raw: Vec<ffi::TS7DataItem> = items
            .iter_mut()
            .map(|item| ffi::TS7DataItem {
                area: item.area.code(),
                word_len: item.word_len.code(),
                result: 0,
                db_number: item.db_number,
                start: item.start,
                amount: item.amount,
                pdata: item.data.as_mut_ptr() as *mut c_void,
            })
            .collect();
        let rc = unsafe { ffi::Cli_ReadMultiVars(self.raw, raw.as_mut_ptr(), raw.len() as c_int) };
        for (item, raw_item) in items.iter_mut().zip(&raw) {
            item.result = raw_item.result;
        }
        check(rc)
    }

    /// Writes multiple variables in a single request.
    pub fn write_multi_vars(&self, items: &mut [DataItem<'_>]) -> Result<()> {
        let mut raw: Vec<ffi::TS7DataItem> = items
            .iter_mut()
            .map(|item| ffi::TS7DataItem {
                area: item.area.code(),
                word_len: item.word_len.code(),
                result: 0,
                db_number: item.db_number,
                start: item.start,
                amount: item.amount,
                pdata: item.data.as_mut_ptr() as *mut c_void,
            })
            .collect();
        let rc = unsafe { ffi::Cli_WriteMultiVars(self.raw, raw.as_mut_ptr(), raw.len() as c_int) };
        for (item, raw_item) in items.iter_mut().zip(&raw) {
            item.result = raw_item.result;
        }
        check(rc)
    }

    // -- directory -----------------------------------------------------------

    /// Lists all blocks present on the PLC.
    pub fn list_blocks(&self) -> Result<BlocksList> {
        let mut raw = ffi::TS7BlocksList::default();
        check(unsafe { ffi::Cli_ListBlocks(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Returns detailed information about a block.
    pub fn get_ag_block_info(&self, block_type: BlockType, block_num: i32) -> Result<BlockInfo> {
        let mut raw = ffi::TS7BlockInfo::default();
        check(unsafe {
            ffi::Cli_GetAgBlockInfo(self.raw, block_type.code(), block_num, &mut raw)
        })?;
        Ok(raw.into())
    }

    /// Decodes the block header of an uploaded block (no PLC connection needed).
    pub fn get_pg_block_info(&self, block: &[u8]) -> Result<BlockInfo> {
        let mut raw = ffi::TS7BlockInfo::default();
        check(unsafe {
            ffi::Cli_GetPgBlockInfo(
                self.raw,
                block.as_ptr() as *mut c_void,
                &mut raw,
                block.len() as c_int,
            )
        })?;
        Ok(raw.into())
    }

    /// Lists the block numbers of a given type.
    pub fn list_blocks_of_type(&self, block_type: BlockType) -> Result<Vec<u16>> {
        let mut items = vec![0u16; 0x2000];
        let mut count: c_int = 0x2000;
        check(unsafe {
            ffi::Cli_ListBlocksOfType(self.raw, block_type.code(), items.as_mut_ptr(), &mut count)
        })?;
        items.truncate(count as usize);
        Ok(items)
    }

    // -- blocks --------------------------------------------------------------

    /// Uploads a block from the PLC (data only).
    pub fn upload(&self, block_type: BlockType, block_num: i32) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 0x10000];
        let mut size = buffer.len() as c_int;
        check(unsafe {
            ffi::Cli_Upload(
                self.raw,
                block_type.code(),
                block_num,
                buffer.as_mut_ptr() as *mut c_void,
                &mut size,
            )
        })?;
        buffer.truncate(size as usize);
        Ok(buffer)
    }

    /// Uploads a block from the PLC including its header and footer.
    pub fn full_upload(&self, block_type: BlockType, block_num: i32) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 0x10000];
        let mut size = buffer.len() as c_int;
        check(unsafe {
            ffi::Cli_FullUpload(
                self.raw,
                block_type.code(),
                block_num,
                buffer.as_mut_ptr() as *mut c_void,
                &mut size,
            )
        })?;
        buffer.truncate(size as usize);
        Ok(buffer)
    }

    /// Downloads a block image to the PLC.
    pub fn download(&self, block_num: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_Download(
                self.raw,
                block_num,
                data.as_ptr() as *mut c_void,
                data.len() as c_int,
            )
        })
    }

    /// Deletes a block from the PLC.
    pub fn delete(&self, block_type: BlockType, block_num: i32) -> Result<()> {
        check(unsafe { ffi::Cli_Delete(self.raw, block_type.code(), block_num) })
    }

    /// Uploads the raw contents of a data block.
    pub fn db_get(&self, db_number: i32) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 0x10000];
        let mut size = buffer.len() as c_int;
        check(unsafe {
            ffi::Cli_DBGet(
                self.raw,
                db_number,
                buffer.as_mut_ptr() as *mut c_void,
                &mut size,
            )
        })?;
        buffer.truncate(size as usize);
        Ok(buffer)
    }

    /// Fills a data block with a single byte.
    pub fn db_fill(&self, db_number: i32, fill: u8) -> Result<()> {
        check(unsafe { ffi::Cli_DBFill(self.raw, db_number, fill as c_int) })
    }

    // -- date/time -----------------------------------------------------------

    /// Reads the PLC date/time.
    pub fn get_plc_date_time(&self) -> Result<PlcDateTime> {
        let mut raw = ffi::Tm::default();
        check(unsafe { ffi::Cli_GetPlcDateTime(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Sets the PLC date/time.
    pub fn set_plc_date_time(&self, date_time: PlcDateTime) -> Result<()> {
        let mut raw: ffi::Tm = date_time.into();
        check(unsafe { ffi::Cli_SetPlcDateTime(self.raw, &mut raw) })
    }

    /// Sets the PLC date/time to the host system time.
    pub fn set_plc_system_date_time(&self) -> Result<()> {
        check(unsafe { ffi::Cli_SetPlcSystemDateTime(self.raw) })
    }

    // -- system info ---------------------------------------------------------

    /// Reads the CPU order code.
    pub fn get_order_code(&self) -> Result<OrderCode> {
        let mut raw = ffi::TS7OrderCode::default();
        check(unsafe { ffi::Cli_GetOrderCode(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Reads CPU identification data.
    pub fn get_cpu_info(&self) -> Result<CpuInfo> {
        let mut raw = ffi::TS7CpuInfo::default();
        check(unsafe { ffi::Cli_GetCpuInfo(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Reads communication processor information.
    pub fn get_cp_info(&self) -> Result<CpInfo> {
        let mut raw = ffi::TS7CpInfo::default();
        check(unsafe { ffi::Cli_GetCpInfo(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Reads a single SZL (system status list) entry.
    pub fn read_szl(&self, id: i32, index: i32) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 0x2000];
        let mut size = buffer.len() as c_int;
        check(unsafe {
            ffi::Cli_ReadSZL(
                self.raw,
                id,
                index,
                buffer.as_mut_ptr() as *mut c_void,
                &mut size,
            )
        })?;
        buffer.truncate(size as usize);
        Ok(buffer)
    }

    /// Reads a complete SZL list.
    pub fn read_szl_list(&self) -> Result<Vec<u8>> {
        let mut buffer = vec![0u8; 0x2000];
        let mut count = buffer.len() as c_int;
        check(unsafe {
            ffi::Cli_ReadSZLList(self.raw, buffer.as_mut_ptr() as *mut c_void, &mut count)
        })?;
        buffer.truncate(count as usize);
        Ok(buffer)
    }

    // -- control -------------------------------------------------------------

    /// Performs a PLC hot start.
    pub fn plc_hot_start(&self) -> Result<()> {
        check(unsafe { ffi::Cli_PlcHotStart(self.raw) })
    }

    /// Performs a PLC cold start.
    pub fn plc_cold_start(&self) -> Result<()> {
        check(unsafe { ffi::Cli_PlcColdStart(self.raw) })
    }

    /// Stops the PLC.
    pub fn plc_stop(&self) -> Result<()> {
        check(unsafe { ffi::Cli_PlcStop(self.raw) })
    }

    /// Copies RAM to ROM.
    pub fn copy_ram_to_rom(&self, timeout_ms: i32) -> Result<()> {
        check(unsafe { ffi::Cli_CopyRamToRom(self.raw, timeout_ms) })
    }

    /// Compresses the PLC memory.
    pub fn compress(&self, timeout_ms: i32) -> Result<()> {
        check(unsafe { ffi::Cli_Compress(self.raw, timeout_ms) })
    }

    /// Reads the PLC run/stop status.
    pub fn get_plc_status(&self) -> Result<CpuStatus> {
        let mut status: c_int = 0;
        check(unsafe { ffi::Cli_GetPlcStatus(self.raw, &mut status) })?;
        Ok(CpuStatus::from_code(status))
    }

    // -- security ------------------------------------------------------------

    /// Reads the CPU protection levels.
    pub fn get_protection(&self) -> Result<Protection> {
        let mut raw = ffi::TS7Protection::default();
        check(unsafe { ffi::Cli_GetProtection(self.raw, &mut raw) })?;
        Ok(raw.into())
    }

    /// Sets the session password.
    pub fn set_session_password(&self, password: &str) -> Result<()> {
        let pwd = CString::new(password).map_err(|_| Error::new(ffi::ERR_LIB_INVALID_PARAM))?;
        check(unsafe { ffi::Cli_SetSessionPassword(self.raw, pwd.as_ptr() as *mut c_char) })
    }

    /// Clears the session password.
    pub fn clear_session_password(&self) -> Result<()> {
        check(unsafe { ffi::Cli_ClearSessionPassword(self.raw) })
    }

    // -- low level / misc ----------------------------------------------------

    /// Exchanges a raw ISO-TCP/S7 buffer with the PLC (advanced).
    pub fn iso_exchange_buffer(&self, data: &mut [u8]) -> Result<usize> {
        let mut size = data.len() as c_int;
        check(unsafe {
            ffi::Cli_IsoExchangeBuffer(self.raw, data.as_mut_ptr() as *mut c_void, &mut size)
        })?;
        Ok(size as usize)
    }

    /// Returns the execution time of the last operation (milliseconds).
    pub fn get_exec_time(&self) -> Result<i32> {
        let mut time: c_int = 0;
        check(unsafe { ffi::Cli_GetExecTime(self.raw, &mut time) })?;
        Ok(time)
    }

    /// Returns the last error code.
    pub fn get_last_error(&self) -> Result<i32> {
        let mut error: c_int = 0;
        check(unsafe { ffi::Cli_GetLastError(self.raw, &mut error) })?;
        Ok(error)
    }

    /// Returns the requested and negotiated PDU lengths.
    pub fn get_pdu_length(&self) -> Result<(i32, i32)> {
        let mut requested: c_int = 0;
        let mut negotiated: c_int = 0;
        check(unsafe { ffi::Cli_GetPduLength(self.raw, &mut requested, &mut negotiated) })?;
        Ok((requested, negotiated))
    }

    // -- async ---------------------------------------------------------------

    /// Starts an asynchronous area read; complete it with
    /// [`wait_as_completion`](Self::wait_as_completion).
    pub fn as_read_area(
        &self,
        area: Area,
        db_number: i32,
        start: i32,
        amount: i32,
        word_len: WordLen,
        data: &mut [u8],
    ) -> Result<()> {
        check(unsafe {
            ffi::Cli_AsReadArea(
                self.raw,
                area.code(),
                db_number,
                start,
                amount,
                word_len.code(),
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Starts an asynchronous area write.
    pub fn as_write_area(
        &self,
        area: Area,
        db_number: i32,
        start: i32,
        amount: i32,
        word_len: WordLen,
        data: &[u8],
    ) -> Result<()> {
        check(unsafe {
            ffi::Cli_AsWriteArea(
                self.raw,
                area.code(),
                db_number,
                start,
                amount,
                word_len.code(),
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Starts an asynchronous data-block read.
    pub fn as_db_read(&self, db_number: i32, start: i32, data: &mut [u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_AsDBRead(
                self.raw,
                db_number,
                start,
                data.len() as c_int,
                data.as_mut_ptr() as *mut c_void,
            )
        })
    }

    /// Starts an asynchronous data-block write.
    pub fn as_db_write(&self, db_number: i32, start: i32, data: &[u8]) -> Result<()> {
        check(unsafe {
            ffi::Cli_AsDBWrite(
                self.raw,
                db_number,
                start,
                data.len() as c_int,
                data.as_ptr() as *mut c_void,
            )
        })
    }

    /// Returns the result of a pending asynchronous operation without blocking.
    pub fn check_as_completion(&self) -> Result<bool> {
        let mut op_result: c_int = 0;
        let rc = unsafe { ffi::Cli_CheckAsCompletion(self.raw, &mut op_result) };
        match rc {
            ffi::JOB_COMPLETE => Ok(true),
            ffi::JOB_PENDING => Ok(false),
            code => Err(Error::new(code)),
        }
    }

    /// Waits up to `timeout_ms` for a pending asynchronous operation to finish.
    pub fn wait_as_completion(&self, timeout_ms: i32) -> Result<()> {
        check(unsafe { ffi::Cli_WaitAsCompletion(self.raw, timeout_ms) })
    }

    // -- text ----------------------------------------------------------------

    /// Returns a human-readable description of a client error code.
    pub fn error_text(code: i32) -> String {
        let mut buffer = [0u8; 256];
        unsafe {
            ffi::Cli_ErrorText(
                code,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len() as c_int,
            );
        }
        crate::types::c_string(&buffer)
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        unsafe { ffi::Cli_Destroy(&mut self.raw) };
    }
}
