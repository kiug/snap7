//! Hand-written FFI bindings for the Snap7 C API.
//!
//! Snap7 is a C++ library that exposes a flat C API (declared with
//! `extern "C"` in `src/lib/snap7_libmain.h`). The public structures use
//! `#pragma pack(1)`; for every structure below the field offsets happen to be
//! identical under `#[repr(C)]` (they contain no interior padding), so the
//! plain `#[repr(C)]` layout matches the C layout for the fields we touch.

use std::os::raw::{c_char, c_int, c_long, c_void};

// ---------------------------------------------------------------------------
// Primitive type aliases (see snap7/src/sys/snap_platform.h)
// ---------------------------------------------------------------------------
pub type Byte = u8;
pub type Word = u16;
pub type Longword = u32;
/// `S7Object` is `uintptr_t`, i.e. `usize` on every target Snap7 supports here.
pub type S7Object = usize;
/// `time_t` is `long` (64-bit) on Linux x86_64 / glibc.
pub type TimeT = i64;

// ---------------------------------------------------------------------------
// Areas (S7Area*)
// ---------------------------------------------------------------------------
pub const S7_AREA_PE: c_int = 0x81;
pub const S7_AREA_PA: c_int = 0x82;
pub const S7_AREA_MK: c_int = 0x83;
pub const S7_AREA_DB: c_int = 0x84;
pub const S7_AREA_CT: c_int = 0x1C;
pub const S7_AREA_TM: c_int = 0x1D;

// ---------------------------------------------------------------------------
// Word lengths (S7WL*)
// ---------------------------------------------------------------------------
pub const S7_WL_BIT: c_int = 0x01;
pub const S7_WL_BYTE: c_int = 0x02;
pub const S7_WL_CHAR: c_int = 0x03;
pub const S7_WL_WORD: c_int = 0x04;
pub const S7_WL_INT: c_int = 0x05;
pub const S7_WL_DWORD: c_int = 0x06;
pub const S7_WL_DINT: c_int = 0x07;
pub const S7_WL_REAL: c_int = 0x08;
pub const S7_WL_COUNTER: c_int = 0x1C;
pub const S7_WL_TIMER: c_int = 0x1D;

// ---------------------------------------------------------------------------
// Block types (Block_*)
// ---------------------------------------------------------------------------
pub const BLOCK_OB: c_int = 0x38;
pub const BLOCK_DB: c_int = 0x41;
pub const BLOCK_SDB: c_int = 0x42;
pub const BLOCK_FC: c_int = 0x43;
pub const BLOCK_SFC: c_int = 0x44;
pub const BLOCK_FB: c_int = 0x45;
pub const BLOCK_SFB: c_int = 0x46;

// ---------------------------------------------------------------------------
// CPU status (S7CpuStatus*)
// ---------------------------------------------------------------------------
pub const CPU_STATUS_UNKNOWN: c_int = 0x00;
pub const CPU_STATUS_RUN: c_int = 0x08;
pub const CPU_STATUS_STOP: c_int = 0x04;

// ---------------------------------------------------------------------------
// Connection type (CONNTYPE_*)
// ---------------------------------------------------------------------------
pub const CONNTYPE_PG: Word = 0x01;
pub const CONNTYPE_OP: Word = 0x02;
pub const CONNTYPE_BASIC: Word = 0x03;

// ---------------------------------------------------------------------------
// Parameter numbers (p_*)
// ---------------------------------------------------------------------------
pub const P_U16_LOCAL_PORT: c_int = 1;
pub const P_U16_REMOTE_PORT: c_int = 2;
pub const P_I32_PING_TIMEOUT: c_int = 3;
pub const P_I32_SEND_TIMEOUT: c_int = 4;
pub const P_I32_RECV_TIMEOUT: c_int = 5;
pub const P_I32_WORK_INTERVAL: c_int = 6;
pub const P_U16_SRC_REF: c_int = 7;
pub const P_U16_DST_REF: c_int = 8;
pub const P_U16_SRC_TSAP: c_int = 9;
pub const P_I32_PDU_REQUEST: c_int = 10;
pub const P_I32_MAX_CLIENTS: c_int = 11;
pub const P_I32_BSEND_TIMEOUT: c_int = 12;
pub const P_I32_BRECV_TIMEOUT: c_int = 13;
pub const P_U32_RECOVERY_TIME: c_int = 14;
pub const P_U32_KEEP_ALIVE_TIME: c_int = 15;

// ---------------------------------------------------------------------------
// Server area IDs (srvArea*)
// ---------------------------------------------------------------------------
pub const SRV_AREA_PE: c_int = 0;
pub const SRV_AREA_PA: c_int = 1;
pub const SRV_AREA_MK: c_int = 2;
pub const SRV_AREA_CT: c_int = 3;
pub const SRV_AREA_TM: c_int = 4;
pub const SRV_AREA_DB: c_int = 5;

// ---------------------------------------------------------------------------
// Operation codes (OperationRead/OperationWrite)
// ---------------------------------------------------------------------------
pub const OPERATION_READ: c_int = 0;
pub const OPERATION_WRITE: c_int = 1;

// ---------------------------------------------------------------------------
// Async mode (am*)
// ---------------------------------------------------------------------------
pub const AM_POLLING: c_int = 0;
pub const AM_EVENT: c_int = 1;
pub const AM_CALLBACK: c_int = 2;

// ---------------------------------------------------------------------------
// Job status (JobComplete / JobPending)
// ---------------------------------------------------------------------------
pub const JOB_COMPLETE: c_int = 0;
pub const JOB_PENDING: c_int = 1;

// ---------------------------------------------------------------------------
// Library-level errors
// ---------------------------------------------------------------------------
pub const ERR_LIB_INVALID_PARAM: c_int = -1;
pub const ERR_LIB_INVALID_OBJECT: c_int = -2;

// ---------------------------------------------------------------------------
// Client errors (errCli*)
// ---------------------------------------------------------------------------
pub const ERR_CLI_INVALID_PARAMS: c_int = 0x0020_0000;
pub const ERR_CLI_JOB_PENDING: c_int = 0x0030_0000;
pub const ERR_CLI_TOO_MANY_ITEMS: c_int = 0x0040_0000;
pub const ERR_CLI_INVALID_WORD_LEN: c_int = 0x0050_0000;
pub const ERR_CLI_PARTIAL_DATA_WRITTEN: c_int = 0x0060_0000;
pub const ERR_CLI_SIZE_OVER_PDU: c_int = 0x0070_0000;
pub const ERR_CLI_INVALID_PLC_ANSWER: c_int = 0x0080_0000;
pub const ERR_CLI_ADDRESS_OUT_OF_RANGE: c_int = 0x0090_0000;
pub const ERR_CLI_INVALID_TRANSPORT_SIZE: c_int = 0x00A0_0000;
pub const ERR_CLI_WRITE_DATA_SIZE_MISMATCH: c_int = 0x00B0_0000;
pub const ERR_CLI_ITEM_NOT_AVAILABLE: c_int = 0x00C0_0000;
pub const ERR_CLI_INVALID_VALUE: c_int = 0x00D0_0000;
pub const ERR_CLI_CANNOT_START_PLC: c_int = 0x00E0_0000;
pub const ERR_CLI_ALREADY_RUN: c_int = 0x00F0_0000;
pub const ERR_CLI_CANNOT_STOP_PLC: c_int = 0x0100_0000;
pub const ERR_CLI_CANNOT_COPY_RAM_TO_ROM: c_int = 0x0110_0000;
pub const ERR_CLI_CANNOT_COMPRESS: c_int = 0x0120_0000;
pub const ERR_CLI_ALREADY_STOP: c_int = 0x0130_0000;
pub const ERR_CLI_FUN_NOT_AVAILABLE: c_int = 0x0140_0000;
pub const ERR_CLI_UPLOAD_SEQUENCE_FAILED: c_int = 0x0150_0000;
pub const ERR_CLI_INVALID_DATA_SIZE_RECVD: c_int = 0x0160_0000;
pub const ERR_CLI_INVALID_BLOCK_TYPE: c_int = 0x0170_0000;
pub const ERR_CLI_INVALID_BLOCK_NUMBER: c_int = 0x0180_0000;
pub const ERR_CLI_INVALID_BLOCK_SIZE: c_int = 0x0190_0000;
pub const ERR_CLI_DOWNLOAD_SEQUENCE_FAILED: c_int = 0x01A0_0000;
pub const ERR_CLI_INSERT_REFUSED: c_int = 0x01B0_0000;
pub const ERR_CLI_DELETE_REFUSED: c_int = 0x01C0_0000;
pub const ERR_CLI_NEED_PASSWORD: c_int = 0x01D0_0000;
pub const ERR_CLI_INVALID_PASSWORD: c_int = 0x01E0_0000;
pub const ERR_CLI_NO_PASSWORD_TO_SET_OR_CLEAR: c_int = 0x01F0_0000;
pub const ERR_CLI_JOB_TIMEOUT: c_int = 0x0200_0000;
pub const ERR_CLI_PARTIAL_DATA_READ: c_int = 0x0210_0000;
pub const ERR_CLI_BUFFER_TOO_SMALL: c_int = 0x0220_0000;
pub const ERR_CLI_FUNCTION_REFUSED: c_int = 0x0230_0000;
pub const ERR_CLI_DESTROYING: c_int = 0x0240_0000;
pub const ERR_CLI_INVALID_PARAM_NUMBER: c_int = 0x0250_0000;
pub const ERR_CLI_CANNOT_CHANGE_PARAM: c_int = 0x0260_0000;

// ---------------------------------------------------------------------------
// Partner errors (errPar*)
// ---------------------------------------------------------------------------
pub const ERR_PAR_ADDRESS_IN_USE: c_int = 0x0020_0000;
pub const ERR_PAR_NO_ROOM: c_int = 0x0030_0000;
pub const ERR_PAR_SERVER_NO_ROOM: c_int = 0x0040_0000;
pub const ERR_PAR_INVALID_PARAMS: c_int = 0x0050_0000;
pub const ERR_PAR_NOT_LINKED: c_int = 0x0060_0000;
pub const ERR_PAR_BUSY: c_int = 0x0070_0000;
pub const ERR_PAR_FRAME_TIMEOUT: c_int = 0x0080_0000;
pub const ERR_PAR_INVALID_PDU: c_int = 0x0090_0000;
pub const ERR_PAR_SEND_TIMEOUT: c_int = 0x00A0_0000;
pub const ERR_PAR_RECV_TIMEOUT: c_int = 0x00B0_0000;
pub const ERR_PAR_SEND_REFUSED: c_int = 0x00C0_0000;
pub const ERR_PAR_NEGOTIATING_PDU: c_int = 0x00D0_0000;
pub const ERR_PAR_SENDING_BLOCK: c_int = 0x00E0_0000;
pub const ERR_PAR_RECVING_BLOCK: c_int = 0x00F0_0000;
pub const ERR_PAR_BIND_ERROR: c_int = 0x0100_0000;
pub const ERR_PAR_DESTROYING: c_int = 0x0110_0000;
pub const ERR_PAR_INVALID_PARAM_NUMBER: c_int = 0x0120_0000;
pub const ERR_PAR_CANNOT_CHANGE_PARAM: c_int = 0x0130_0000;
pub const ERR_PAR_BUFFER_TOO_SMALL: c_int = 0x0140_0000;

// ---------------------------------------------------------------------------
// ISO transport errors (errIso*)
// ---------------------------------------------------------------------------
pub const ERR_ISO_CONNECT: c_int = 0x0001_0000;
pub const ERR_ISO_DISCONNECT: c_int = 0x0002_0000;
pub const ERR_ISO_INVALID_PDU: c_int = 0x0003_0000;
pub const ERR_ISO_INVALID_DATA_SIZE: c_int = 0x0004_0000;
pub const ERR_ISO_NULL_POINTER: c_int = 0x0005_0000;
pub const ERR_ISO_SHORT_PACKET: c_int = 0x0006_0000;
pub const ERR_ISO_TOO_MANY_FRAGMENTS: c_int = 0x0007_0000;
pub const ERR_ISO_PDU_OVERFLOW: c_int = 0x0008_0000;
pub const ERR_ISO_SEND_PACKET: c_int = 0x0009_0000;
pub const ERR_ISO_RECV_PACKET: c_int = 0x000A_0000;
pub const ERR_ISO_INVALID_PARAMS: c_int = 0x000B_0000;

/// Shared "negotiating PDU" / "server cannot start" code (same 0x00100000).
pub const ERR_NEGOTIATING_PDU: c_int = 0x0010_0000;

// ---------------------------------------------------------------------------
// Server errors (errSrv*)
// ---------------------------------------------------------------------------
pub const ERR_SRV_DB_NULL_POINTER: c_int = 0x0020_0000;
pub const ERR_SRV_AREA_ALREADY_EXISTS: c_int = 0x0030_0000;
pub const ERR_SRV_UNKNOWN_AREA: c_int = 0x0040_0000;
pub const ERR_SRV_INVALID_PARAMS: c_int = 0x0050_0000;
pub const ERR_SRV_TOO_MANY_DB: c_int = 0x0060_0000;
pub const ERR_SRV_INVALID_PARAM_NUMBER: c_int = 0x0070_0000;
pub const ERR_SRV_CANNOT_CHANGE_PARAM: c_int = 0x0080_0000;

// ---------------------------------------------------------------------------
// Server status (SrvStopped/SrvRunning/SrvError)
// ---------------------------------------------------------------------------
pub const SRV_STOPPED: c_int = 0;
pub const SRV_RUNNING: c_int = 1;
pub const SRV_ERROR: c_int = 2;

// ---------------------------------------------------------------------------
// Partner status (par_*)
// ---------------------------------------------------------------------------
pub const PAR_STOPPED: c_int = 0;
pub const PAR_CONNECTING: c_int = 1;
pub const PAR_WAITING: c_int = 2;
pub const PAR_LINKED: c_int = 3;
pub const PAR_SENDING: c_int = 4;
pub const PAR_RECEIVING: c_int = 5;
pub const PAR_BINDERROR: c_int = 6;

// ---------------------------------------------------------------------------
// Event mask kind (mkEvent / mkLog)
// ---------------------------------------------------------------------------
pub const MK_EVENT: c_int = 0;
pub const MK_LOG: c_int = 1;

// ---------------------------------------------------------------------------
// Server event codes (evc*)
// ---------------------------------------------------------------------------
pub const EVC_SERVER_STARTED: Longword = 0x0000_0001;
pub const EVC_PDU_INCOMING: Longword = 0x0001_0000;
pub const EVC_DATA_READ: Longword = 0x0002_0000;
pub const EVC_DATA_WRITE: Longword = 0x0004_0000;
pub const EVC_NEGOTIATE_PDU: Longword = 0x0008_0000;
pub const EVC_READ_SZL: Longword = 0x0010_0000;
pub const EVC_CLOCK: Longword = 0x0020_0000;
pub const EVC_UPLOAD: Longword = 0x0040_0000;
pub const EVC_DOWNLOAD: Longword = 0x0080_0000;
pub const EVC_DIRECTORY: Longword = 0x0100_0000;
pub const EVC_SECURITY: Longword = 0x0200_0000;
pub const EVC_CONTROL: Longword = 0x0400_0000;

// ---------------------------------------------------------------------------
// Public C structures (layouts match the `#pragma pack(1)` C definitions)
// ---------------------------------------------------------------------------

/// `TS7DataItem`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TS7DataItem {
    pub area: c_int,
    pub word_len: c_int,
    pub result: c_int,
    pub db_number: c_int,
    pub start: c_int,
    pub amount: c_int,
    pub pdata: *mut c_void,
}

impl Default for TS7DataItem {
    fn default() -> Self {
        TS7DataItem {
            area: 0,
            word_len: 0,
            result: 0,
            db_number: 0,
            start: 0,
            amount: 0,
            pdata: std::ptr::null_mut(),
        }
    }
}

/// `TS7BlocksList`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TS7BlocksList {
    pub ob_count: c_int,
    pub fb_count: c_int,
    pub fc_count: c_int,
    pub sfb_count: c_int,
    pub sfc_count: c_int,
    pub db_count: c_int,
    pub sdb_count: c_int,
}

/// `TS7BlockInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TS7BlockInfo {
    pub blk_type: c_int,
    pub blk_number: c_int,
    pub blk_lang: c_int,
    pub blk_flags: c_int,
    pub mc7_size: c_int,
    pub load_size: c_int,
    pub local_data: c_int,
    pub sbb_length: c_int,
    pub check_sum: c_int,
    pub version: c_int,
    pub code_date: [u8; 11],
    pub intf_date: [u8; 11],
    pub author: [u8; 9],
    pub family: [u8; 9],
    pub header: [u8; 9],
}

impl Default for TS7BlockInfo {
    fn default() -> Self {
        TS7BlockInfo {
            blk_type: 0,
            blk_number: 0,
            blk_lang: 0,
            blk_flags: 0,
            mc7_size: 0,
            load_size: 0,
            local_data: 0,
            sbb_length: 0,
            check_sum: 0,
            version: 0,
            code_date: [0; 11],
            intf_date: [0; 11],
            author: [0; 9],
            family: [0; 9],
            header: [0; 9],
        }
    }
}

/// `TS7OrderCode`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TS7OrderCode {
    pub code: [u8; 21],
    pub v1: Byte,
    pub v2: Byte,
    pub v3: Byte,
}

impl Default for TS7OrderCode {
    fn default() -> Self {
        TS7OrderCode {
            code: [0; 21],
            v1: 0,
            v2: 0,
            v3: 0,
        }
    }
}

/// `TS7CpuInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TS7CpuInfo {
    pub module_type_name: [u8; 33],
    pub serial_number: [u8; 25],
    pub as_name: [u8; 25],
    pub copyright: [u8; 27],
    pub module_name: [u8; 25],
}

impl Default for TS7CpuInfo {
    fn default() -> Self {
        TS7CpuInfo {
            module_type_name: [0; 33],
            serial_number: [0; 25],
            as_name: [0; 25],
            copyright: [0; 27],
            module_name: [0; 25],
        }
    }
}

/// `TS7CpInfo`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TS7CpInfo {
    pub max_pdu_length: c_int,
    pub max_connections: c_int,
    pub max_mpi_rate: c_int,
    pub max_bus_rate: c_int,
}

/// `TS7Protection`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TS7Protection {
    pub sch_schal: Word,
    pub sch_par: Word,
    pub sch_rel: Word,
    pub bart_sch: Word,
    pub anl_sch: Word,
}

/// `TS7Tag`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TS7Tag {
    pub area: c_int,
    pub db_number: c_int,
    pub start: c_int,
    pub size: c_int,
    pub word_len: c_int,
}

/// `TSrvEvent`
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TSrvEvent {
    pub evt_time: TimeT,
    pub evt_sender: c_int,
    pub evt_code: Longword,
    pub evt_ret_code: Word,
    pub evt_param1: Word,
    pub evt_param2: Word,
    pub evt_param3: Word,
    pub evt_param4: Word,
}

/// `tm` (glibc) used by `Cli_GetPlcDateTime` / `Cli_SetPlcDateTime`.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

// ---------------------------------------------------------------------------
// Callback types
// ---------------------------------------------------------------------------
pub type PfnCliCompletion =
    Option<unsafe extern "C" fn(usr_ptr: *mut c_void, op_code: c_int, op_result: c_int)>;
pub type PfnSrvCallBack =
    Option<unsafe extern "C" fn(usr_ptr: *mut c_void, p_event: *mut TSrvEvent, size: c_int)>;
pub type PfnRWAreaCallBack = Option<
    unsafe extern "C" fn(
        usr_ptr: *mut c_void,
        sender: c_int,
        operation: c_int,
        p_tag: *mut TS7Tag,
        p_usr_data: *mut c_void,
    ) -> c_int,
>;
pub type PfnParBSendCompletion =
    Option<unsafe extern "C" fn(usr_ptr: *mut c_void, op_result: c_int)>;
pub type PfnParBRecvCallBack = Option<
    unsafe extern "C" fn(
        usr_ptr: *mut c_void,
        op_result: c_int,
        r_id: Longword,
        p_data: *mut c_void,
        size: c_int,
    ),
>;

// ---------------------------------------------------------------------------
// C API functions
//
// Linking against the static library (and the required C/C++ runtime libs) is
// configured in build.rs via `cargo:rustc-link-*` directives.
// ---------------------------------------------------------------------------
extern "C" {
    // ---- Client: lifecycle -------------------------------------------------
    pub fn Cli_Create() -> S7Object;
    pub fn Cli_Destroy(client: *mut S7Object);
    pub fn Cli_Connect(client: S7Object) -> c_int;
    pub fn Cli_SetConnectionParams(
        client: S7Object,
        address: *const c_char,
        local_tsap: Word,
        remote_tsap: Word,
    ) -> c_int;
    pub fn Cli_SetConnectionType(client: S7Object, connection_type: Word) -> c_int;
    pub fn Cli_ConnectTo(
        client: S7Object,
        address: *const c_char,
        rack: c_int,
        slot: c_int,
    ) -> c_int;
    pub fn Cli_Disconnect(client: S7Object) -> c_int;
    pub fn Cli_GetParam(client: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Cli_SetParam(client: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Cli_SetAsCallback(
        client: S7Object,
        completion: PfnCliCompletion,
        usr_ptr: *mut c_void,
    ) -> c_int;

    // ---- Client: data I/O --------------------------------------------------
    pub fn Cli_ReadArea(
        client: S7Object,
        area: c_int,
        db_number: c_int,
        start: c_int,
        amount: c_int,
        word_len: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_WriteArea(
        client: S7Object,
        area: c_int,
        db_number: c_int,
        start: c_int,
        amount: c_int,
        word_len: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_ReadMultiVars(
        client: S7Object,
        items: *mut TS7DataItem,
        items_count: c_int,
    ) -> c_int;
    pub fn Cli_WriteMultiVars(
        client: S7Object,
        items: *mut TS7DataItem,
        items_count: c_int,
    ) -> c_int;
    pub fn Cli_DBRead(
        client: S7Object,
        db_number: c_int,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_DBWrite(
        client: S7Object,
        db_number: c_int,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_MBRead(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void) -> c_int;
    pub fn Cli_MBWrite(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void)
        -> c_int;
    pub fn Cli_EBRead(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void) -> c_int;
    pub fn Cli_EBWrite(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void)
        -> c_int;
    pub fn Cli_ABRead(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void) -> c_int;
    pub fn Cli_ABWrite(client: S7Object, start: c_int, size: c_int, usr_data: *mut c_void)
        -> c_int;
    pub fn Cli_TMRead(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_TMWrite(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_CTRead(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_CTWrite(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;

    // ---- Client: directory -------------------------------------------------
    pub fn Cli_ListBlocks(client: S7Object, usr_data: *mut TS7BlocksList) -> c_int;
    pub fn Cli_GetAgBlockInfo(
        client: S7Object,
        block_type: c_int,
        block_num: c_int,
        usr_data: *mut TS7BlockInfo,
    ) -> c_int;
    pub fn Cli_GetPgBlockInfo(
        client: S7Object,
        block: *mut c_void,
        usr_data: *mut TS7BlockInfo,
        size: c_int,
    ) -> c_int;
    pub fn Cli_ListBlocksOfType(
        client: S7Object,
        block_type: c_int,
        usr_data: *mut Word,
        items_count: *mut c_int,
    ) -> c_int;

    // ---- Client: blocks ----------------------------------------------------
    pub fn Cli_Upload(
        client: S7Object,
        block_type: c_int,
        block_num: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_FullUpload(
        client: S7Object,
        block_type: c_int,
        block_num: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_Download(
        client: S7Object,
        block_num: c_int,
        usr_data: *mut c_void,
        size: c_int,
    ) -> c_int;
    pub fn Cli_Delete(client: S7Object, block_type: c_int, block_num: c_int) -> c_int;
    pub fn Cli_DBGet(
        client: S7Object,
        db_number: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_DBFill(client: S7Object, db_number: c_int, fill_char: c_int) -> c_int;

    // ---- Client: date/time -------------------------------------------------
    pub fn Cli_GetPlcDateTime(client: S7Object, date_time: *mut Tm) -> c_int;
    pub fn Cli_SetPlcDateTime(client: S7Object, date_time: *mut Tm) -> c_int;
    pub fn Cli_SetPlcSystemDateTime(client: S7Object) -> c_int;

    // ---- Client: system info ----------------------------------------------
    pub fn Cli_GetOrderCode(client: S7Object, usr_data: *mut TS7OrderCode) -> c_int;
    pub fn Cli_GetCpuInfo(client: S7Object, usr_data: *mut TS7CpuInfo) -> c_int;
    pub fn Cli_GetCpInfo(client: S7Object, usr_data: *mut TS7CpInfo) -> c_int;
    pub fn Cli_ReadSZL(
        client: S7Object,
        id: c_int,
        index: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_ReadSZLList(
        client: S7Object,
        usr_data: *mut c_void,
        items_count: *mut c_int,
    ) -> c_int;

    // ---- Client: control ---------------------------------------------------
    pub fn Cli_PlcHotStart(client: S7Object) -> c_int;
    pub fn Cli_PlcColdStart(client: S7Object) -> c_int;
    pub fn Cli_PlcStop(client: S7Object) -> c_int;
    pub fn Cli_CopyRamToRom(client: S7Object, timeout: c_int) -> c_int;
    pub fn Cli_Compress(client: S7Object, timeout: c_int) -> c_int;
    pub fn Cli_GetPlcStatus(client: S7Object, status: *mut c_int) -> c_int;

    // ---- Client: security --------------------------------------------------
    pub fn Cli_GetProtection(client: S7Object, usr_data: *mut TS7Protection) -> c_int;
    pub fn Cli_SetSessionPassword(client: S7Object, password: *mut c_char) -> c_int;
    pub fn Cli_ClearSessionPassword(client: S7Object) -> c_int;

    // ---- Client: low level / misc -----------------------------------------
    pub fn Cli_IsoExchangeBuffer(
        client: S7Object,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_GetExecTime(client: S7Object, time: *mut c_int) -> c_int;
    pub fn Cli_GetLastError(client: S7Object, last_error: *mut c_int) -> c_int;
    pub fn Cli_GetPduLength(
        client: S7Object,
        requested: *mut c_int,
        negotiated: *mut c_int,
    ) -> c_int;
    pub fn Cli_ErrorText(error: c_int, text: *mut c_char, text_len: c_int) -> c_int;
    pub fn Cli_GetConnected(client: S7Object, connected: *mut c_int) -> c_int;

    // ---- Client: async -----------------------------------------------------
    pub fn Cli_AsReadArea(
        client: S7Object,
        area: c_int,
        db_number: c_int,
        start: c_int,
        amount: c_int,
        word_len: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsWriteArea(
        client: S7Object,
        area: c_int,
        db_number: c_int,
        start: c_int,
        amount: c_int,
        word_len: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsDBRead(
        client: S7Object,
        db_number: c_int,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsDBWrite(
        client: S7Object,
        db_number: c_int,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsMBRead(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsMBWrite(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsEBRead(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsEBWrite(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsABRead(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsABWrite(
        client: S7Object,
        start: c_int,
        size: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsTMRead(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsTMWrite(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsCTRead(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsCTWrite(
        client: S7Object,
        start: c_int,
        amount: c_int,
        usr_data: *mut c_void,
    ) -> c_int;
    pub fn Cli_AsListBlocksOfType(
        client: S7Object,
        block_type: c_int,
        usr_data: *mut Word,
        items_count: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsReadSZL(
        client: S7Object,
        id: c_int,
        index: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsReadSZLList(
        client: S7Object,
        usr_data: *mut c_void,
        items_count: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsUpload(
        client: S7Object,
        block_type: c_int,
        block_num: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsFullUpload(
        client: S7Object,
        block_type: c_int,
        block_num: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsDownload(
        client: S7Object,
        block_num: c_int,
        usr_data: *mut c_void,
        size: c_int,
    ) -> c_int;
    pub fn Cli_AsCopyRamToRom(client: S7Object, timeout: c_int) -> c_int;
    pub fn Cli_AsCompress(client: S7Object, timeout: c_int) -> c_int;
    pub fn Cli_AsDBGet(
        client: S7Object,
        db_number: c_int,
        usr_data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Cli_AsDBFill(client: S7Object, db_number: c_int, fill_char: c_int) -> c_int;
    pub fn Cli_CheckAsCompletion(client: S7Object, op_result: *mut c_int) -> c_int;
    pub fn Cli_WaitAsCompletion(client: S7Object, timeout: c_int) -> c_int;

    // ---- Server ------------------------------------------------------------
    pub fn Srv_Create() -> S7Object;
    pub fn Srv_Destroy(server: *mut S7Object);
    pub fn Srv_GetParam(server: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Srv_SetParam(server: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Srv_Start(server: S7Object) -> c_int;
    pub fn Srv_StartTo(server: S7Object, address: *const c_char) -> c_int;
    pub fn Srv_Stop(server: S7Object) -> c_int;
    pub fn Srv_RegisterArea(
        server: S7Object,
        area_code: c_int,
        index: Word,
        usr_data: *mut c_void,
        size: c_int,
    ) -> c_int;
    pub fn Srv_UnregisterArea(server: S7Object, area_code: c_int, index: Word) -> c_int;
    pub fn Srv_LockArea(server: S7Object, area_code: c_int, index: Word) -> c_int;
    pub fn Srv_UnlockArea(server: S7Object, area_code: c_int, index: Word) -> c_int;
    pub fn Srv_ClearEvents(server: S7Object) -> c_int;
    pub fn Srv_PickEvent(server: S7Object, event: *mut TSrvEvent, evt_ready: *mut c_int) -> c_int;
    pub fn Srv_GetMask(server: S7Object, mask_kind: c_int, mask: *mut Longword) -> c_int;
    pub fn Srv_SetMask(server: S7Object, mask_kind: c_int, mask: Longword) -> c_int;
    pub fn Srv_SetEventsCallback(
        server: S7Object,
        callback: PfnSrvCallBack,
        usr_ptr: *mut c_void,
    ) -> c_int;
    pub fn Srv_SetReadEventsCallback(
        server: S7Object,
        callback: PfnSrvCallBack,
        usr_ptr: *mut c_void,
    ) -> c_int;
    pub fn Srv_EventText(event: *mut TSrvEvent, text: *mut c_char, text_len: c_int) -> c_int;
    pub fn Srv_SetRWAreaCallback(
        server: S7Object,
        callback: PfnRWAreaCallBack,
        usr_ptr: *mut c_void,
    ) -> c_int;
    pub fn Srv_GetStatus(
        server: S7Object,
        server_status: *mut c_int,
        cpu_status: *mut c_int,
        clients_count: *mut c_int,
    ) -> c_int;
    pub fn Srv_SetCpuStatus(server: S7Object, cpu_status: c_int) -> c_int;
    pub fn Srv_ErrorText(error: c_int, text: *mut c_char, text_len: c_int) -> c_int;

    // ---- Partner -----------------------------------------------------------
    pub fn Par_Create(active: c_int) -> S7Object;
    pub fn Par_Destroy(partner: *mut S7Object);
    pub fn Par_GetParam(partner: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Par_SetParam(partner: S7Object, param_number: c_int, value: *mut c_void) -> c_int;
    pub fn Par_Start(partner: S7Object) -> c_int;
    pub fn Par_StartTo(
        partner: S7Object,
        local_address: *const c_char,
        remote_address: *const c_char,
        loc_tsap: Word,
        rem_tsap: Word,
    ) -> c_int;
    pub fn Par_Stop(partner: S7Object) -> c_int;
    pub fn Par_BSend(
        partner: S7Object,
        r_id: Longword,
        usr_data: *mut c_void,
        size: c_int,
    ) -> c_int;
    pub fn Par_AsBSend(
        partner: S7Object,
        r_id: Longword,
        usr_data: *mut c_void,
        size: c_int,
    ) -> c_int;
    pub fn Par_CheckAsBSendCompletion(partner: S7Object, op_result: *mut c_int) -> c_int;
    pub fn Par_WaitAsBSendCompletion(partner: S7Object, timeout: Longword) -> c_int;
    pub fn Par_SetSendCallback(
        partner: S7Object,
        completion: PfnParBSendCompletion,
        usr_ptr: *mut c_void,
    ) -> c_int;
    pub fn Par_BRecv(
        partner: S7Object,
        r_id: *mut Longword,
        data: *mut c_void,
        size: *mut c_int,
        timeout: Longword,
    ) -> c_int;
    pub fn Par_CheckAsBRecvCompletion(
        partner: S7Object,
        op_result: *mut c_int,
        r_id: *mut Longword,
        data: *mut c_void,
        size: *mut c_int,
    ) -> c_int;
    pub fn Par_SetRecvCallback(
        partner: S7Object,
        callback: PfnParBRecvCallBack,
        usr_ptr: *mut c_void,
    ) -> c_int;
    pub fn Par_GetTimes(
        partner: S7Object,
        send_time: *mut Longword,
        recv_time: *mut Longword,
    ) -> c_int;
    pub fn Par_GetStats(
        partner: S7Object,
        bytes_sent: *mut Longword,
        bytes_recv: *mut Longword,
        send_errors: *mut Longword,
        recv_errors: *mut Longword,
    ) -> c_int;
    pub fn Par_GetLastError(partner: S7Object, last_error: *mut c_int) -> c_int;
    pub fn Par_GetStatus(partner: S7Object, status: *mut c_int) -> c_int;
    pub fn Par_ErrorText(error: c_int, text: *mut c_char, text_len: c_int) -> c_int;
}
