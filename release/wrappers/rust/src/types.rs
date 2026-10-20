//! Safe, idiomatic types used by the Snap7 wrapper.
//!
//! The enums map onto the raw `S7Area*`, `S7WL*`, `Block_*`, `CONNTYPE_*` and
//! `S7CpuStatus*` constants in [`crate::ffi`]. The structs wrap the packed C
//! records (`TS7BlockInfo`, `TS7OrderCode`, …) and convert their fixed-size
//! `u8` buffers into owned Rust `String`s.

use crate::ffi;

/// Converts a fixed-size, NUL-terminated C buffer into a Rust `String`.
pub(crate) fn c_string(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).into_owned()
}

// ---------------------------------------------------------------------------
// Areas and word lengths
// ---------------------------------------------------------------------------

/// S7 memory areas addressed by the client `read_area` / `write_area` calls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Area {
    /// Process inputs (I, `S7AreaPE`).
    ProcessInputs,
    /// Process outputs (Q, `S7AreaPA`).
    ProcessOutputs,
    /// Merkers / flags (M, `S7AreaMK`).
    Merkers,
    /// Data block (DB, `S7AreaDB`).
    DataBlock,
    /// Counters (C, `S7AreaCT`).
    Counters,
    /// Timers (T, `S7AreaTM`).
    Timers,
}

impl Area {
    pub(crate) fn code(self) -> i32 {
        match self {
            Area::ProcessInputs => ffi::S7_AREA_PE,
            Area::ProcessOutputs => ffi::S7_AREA_PA,
            Area::Merkers => ffi::S7_AREA_MK,
            Area::DataBlock => ffi::S7_AREA_DB,
            Area::Counters => ffi::S7_AREA_CT,
            Area::Timers => ffi::S7_AREA_TM,
        }
    }
}

/// Transport size / word length used when reading or writing an area.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WordLen {
    Bit,
    Byte,
    Char,
    Word,
    Int,
    DWord,
    DInt,
    Real,
    Counter,
    Timer,
}

impl WordLen {
    pub(crate) fn code(self) -> i32 {
        match self {
            WordLen::Bit => ffi::S7_WL_BIT,
            WordLen::Byte => ffi::S7_WL_BYTE,
            WordLen::Char => ffi::S7_WL_CHAR,
            WordLen::Word => ffi::S7_WL_WORD,
            WordLen::Int => ffi::S7_WL_INT,
            WordLen::DWord => ffi::S7_WL_DWORD,
            WordLen::DInt => ffi::S7_WL_DINT,
            WordLen::Real => ffi::S7_WL_REAL,
            WordLen::Counter => ffi::S7_WL_COUNTER,
            WordLen::Timer => ffi::S7_WL_TIMER,
        }
    }

    /// Number of bytes occupied by a single element of this word length.
    pub fn size(self) -> usize {
        match self {
            WordLen::Bit | WordLen::Byte | WordLen::Char => 1,
            WordLen::Word | WordLen::Int | WordLen::Counter | WordLen::Timer => 2,
            WordLen::DWord | WordLen::DInt | WordLen::Real => 4,
        }
    }
}

// ---------------------------------------------------------------------------
// Block types, connection types and CPU status
// ---------------------------------------------------------------------------

/// PLC block type used by the directory/upload/download functions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Ob,
    Db,
    Sdb,
    Fc,
    Sfc,
    Fb,
    Sfb,
}

impl BlockType {
    pub(crate) fn code(self) -> i32 {
        match self {
            BlockType::Ob => ffi::BLOCK_OB,
            BlockType::Db => ffi::BLOCK_DB,
            BlockType::Sdb => ffi::BLOCK_SDB,
            BlockType::Fc => ffi::BLOCK_FC,
            BlockType::Sfc => ffi::BLOCK_SFC,
            BlockType::Fb => ffi::BLOCK_FB,
            BlockType::Sfb => ffi::BLOCK_SFB,
        }
    }
}

/// Connection type used by [`crate::client::Client::set_connection_type`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    /// PG connection (`CONNTYPE_PG`).
    Pg,
    /// Operator connection (`CONNTYPE_OP`).
    Op,
    /// Basic connection (`CONNTYPE_BASIC`).
    Basic,
}

impl ConnectionType {
    pub(crate) fn code(self) -> u16 {
        match self {
            ConnectionType::Pg => ffi::CONNTYPE_PG,
            ConnectionType::Op => ffi::CONNTYPE_OP,
            ConnectionType::Basic => ffi::CONNTYPE_BASIC,
        }
    }
}

/// PLC CPU status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuStatus {
    Unknown,
    Run,
    Stop,
}

impl CpuStatus {
    pub(crate) fn from_code(code: i32) -> Self {
        match code {
            ffi::CPU_STATUS_RUN => CpuStatus::Run,
            ffi::CPU_STATUS_STOP => CpuStatus::Stop,
            _ => CpuStatus::Unknown,
        }
    }

    pub(crate) fn code(self) -> i32 {
        match self {
            CpuStatus::Unknown => ffi::CPU_STATUS_UNKNOWN,
            CpuStatus::Run => ffi::CPU_STATUS_RUN,
            CpuStatus::Stop => ffi::CPU_STATUS_STOP,
        }
    }
}

// ---------------------------------------------------------------------------
// Safe data records
// ---------------------------------------------------------------------------

/// Block directory summary returned by [`crate::client::Client::list_blocks`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlocksList {
    pub ob_count: i32,
    pub fb_count: i32,
    pub fc_count: i32,
    pub sfb_count: i32,
    pub sfc_count: i32,
    pub db_count: i32,
    pub sdb_count: i32,
}

impl From<ffi::TS7BlocksList> for BlocksList {
    fn from(raw: ffi::TS7BlocksList) -> Self {
        BlocksList {
            ob_count: raw.ob_count,
            fb_count: raw.fb_count,
            fc_count: raw.fc_count,
            sfb_count: raw.sfb_count,
            sfc_count: raw.sfc_count,
            db_count: raw.db_count,
            sdb_count: raw.sdb_count,
        }
    }
}

/// Detailed information about a single PLC block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockInfo {
    pub block_type: i32,
    pub block_number: i32,
    pub block_language: i32,
    pub block_flags: i32,
    pub mc7_size: i32,
    pub load_size: i32,
    pub local_data: i32,
    pub sbb_length: i32,
    pub checksum: i32,
    pub version: i32,
    pub code_date: String,
    pub interface_date: String,
    pub author: String,
    pub family: String,
    pub header: String,
}

impl From<ffi::TS7BlockInfo> for BlockInfo {
    fn from(raw: ffi::TS7BlockInfo) -> Self {
        BlockInfo {
            block_type: raw.blk_type,
            block_number: raw.blk_number,
            block_language: raw.blk_lang,
            block_flags: raw.blk_flags,
            mc7_size: raw.mc7_size,
            load_size: raw.load_size,
            local_data: raw.local_data,
            sbb_length: raw.sbb_length,
            checksum: raw.check_sum,
            version: raw.version,
            code_date: c_string(&raw.code_date),
            interface_date: c_string(&raw.intf_date),
            author: c_string(&raw.author),
            family: c_string(&raw.family),
            header: c_string(&raw.header),
        }
    }
}

/// CPU order code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderCode {
    pub code: String,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
}

impl From<ffi::TS7OrderCode> for OrderCode {
    fn from(raw: ffi::TS7OrderCode) -> Self {
        OrderCode {
            code: c_string(&raw.code),
            v1: raw.v1,
            v2: raw.v2,
            v3: raw.v3,
        }
    }
}

/// CPU identification strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuInfo {
    pub module_type_name: String,
    pub serial_number: String,
    pub as_name: String,
    pub copyright: String,
    pub module_name: String,
}

impl From<ffi::TS7CpuInfo> for CpuInfo {
    fn from(raw: ffi::TS7CpuInfo) -> Self {
        CpuInfo {
            module_type_name: c_string(&raw.module_type_name),
            serial_number: c_string(&raw.serial_number),
            as_name: c_string(&raw.as_name),
            copyright: c_string(&raw.copyright),
            module_name: c_string(&raw.module_name),
        }
    }
}

/// Communication processor information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CpInfo {
    pub max_pdu_length: i32,
    pub max_connections: i32,
    pub max_mpi_rate: i32,
    pub max_bus_rate: i32,
}

impl From<ffi::TS7CpInfo> for CpInfo {
    fn from(raw: ffi::TS7CpInfo) -> Self {
        CpInfo {
            max_pdu_length: raw.max_pdu_length,
            max_connections: raw.max_connections,
            max_mpi_rate: raw.max_mpi_rate,
            max_bus_rate: raw.max_bus_rate,
        }
    }
}

/// CPU protection levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Protection {
    pub sch_schal: u16,
    pub sch_par: u16,
    pub sch_rel: u16,
    pub bart_sch: u16,
    pub anl_sch: u16,
}

impl From<ffi::TS7Protection> for Protection {
    fn from(raw: ffi::TS7Protection) -> Self {
        Protection {
            sch_schal: raw.sch_schal,
            sch_par: raw.sch_par,
            sch_rel: raw.sch_rel,
            bart_sch: raw.bart_sch,
            anl_sch: raw.anl_sch,
        }
    }
}

/// A single item for the multi-variable read/write operations.
#[derive(Debug)]
pub struct DataItem<'a> {
    pub area: Area,
    pub word_len: WordLen,
    pub db_number: i32,
    pub start: i32,
    /// Number of elements to transfer (not bytes).
    pub amount: i32,
    /// Per-item result code, populated after the operation completes.
    pub result: i32,
    /// Data buffer; its length must be at least `amount * word_len.size()`.
    pub data: &'a mut [u8],
}

/// A PLC date/time value with human-friendly fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlcDateTime {
    /// Full year (e.g. `2026`).
    pub year: i32,
    /// Month, 1-12.
    pub month: u32,
    /// Day of month, 1-31.
    pub day: u32,
    /// Hour, 0-23.
    pub hour: u32,
    /// Minute, 0-59.
    pub minute: u32,
    /// Second, 0-60 (leap seconds are possible).
    pub second: u32,
    /// Day of week, 0-6 (0 = Sunday).
    pub day_of_week: u32,
}

impl From<ffi::Tm> for PlcDateTime {
    fn from(raw: ffi::Tm) -> Self {
        PlcDateTime {
            year: raw.tm_year + 1900,
            month: (raw.tm_mon + 1) as u32,
            day: raw.tm_mday as u32,
            hour: raw.tm_hour as u32,
            minute: raw.tm_min as u32,
            second: raw.tm_sec as u32,
            day_of_week: raw.tm_wday as u32,
        }
    }
}

impl From<PlcDateTime> for ffi::Tm {
    fn from(value: PlcDateTime) -> Self {
        ffi::Tm {
            tm_sec: value.second as i32,
            tm_min: value.minute as i32,
            tm_hour: value.hour as i32,
            tm_mday: value.day as i32,
            tm_mon: value.month as i32 - 1,
            tm_year: value.year - 1900,
            tm_wday: value.day_of_week as i32,
            tm_yday: 0,
            tm_isdst: -1,
            tm_gmtoff: 0,
            tm_zone: std::ptr::null(),
        }
    }
}

// ---------------------------------------------------------------------------
// Server types
// ---------------------------------------------------------------------------

/// Server-side area identifier used by [`crate::server::Server::register_area`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerArea {
    ProcessInputs,
    ProcessOutputs,
    Merkers,
    Counters,
    Timers,
    DataBlock,
}

impl ServerArea {
    pub(crate) fn code(self) -> i32 {
        match self {
            ServerArea::ProcessInputs => ffi::SRV_AREA_PE,
            ServerArea::ProcessOutputs => ffi::SRV_AREA_PA,
            ServerArea::Merkers => ffi::SRV_AREA_MK,
            ServerArea::Counters => ffi::SRV_AREA_CT,
            ServerArea::Timers => ffi::SRV_AREA_TM,
            ServerArea::DataBlock => ffi::SRV_AREA_DB,
        }
    }
}

/// A server event picked from the event queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SrvEvent {
    /// Unix timestamp of the event.
    pub time: i64,
    pub sender: i32,
    pub code: u32,
    pub ret_code: u16,
    pub param1: u16,
    pub param2: u16,
    pub param3: u16,
    pub param4: u16,
}

impl From<ffi::TSrvEvent> for SrvEvent {
    fn from(raw: ffi::TSrvEvent) -> Self {
        SrvEvent {
            time: raw.evt_time,
            sender: raw.evt_sender,
            code: raw.evt_code,
            ret_code: raw.evt_ret_code,
            param1: raw.evt_param1,
            param2: raw.evt_param2,
            param3: raw.evt_param3,
            param4: raw.evt_param4,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_len_sizes() {
        assert_eq!(WordLen::Bit.size(), 1);
        assert_eq!(WordLen::Byte.size(), 1);
        assert_eq!(WordLen::Char.size(), 1);
        assert_eq!(WordLen::Word.size(), 2);
        assert_eq!(WordLen::Int.size(), 2);
        assert_eq!(WordLen::DWord.size(), 4);
        assert_eq!(WordLen::DInt.size(), 4);
        assert_eq!(WordLen::Real.size(), 4);
        assert_eq!(WordLen::Counter.size(), 2);
        assert_eq!(WordLen::Timer.size(), 2);
    }

    #[test]
    fn plc_date_time_round_trip() {
        let dt = PlcDateTime {
            year: 2026,
            month: 10,
            day: 20,
            hour: 12,
            minute: 34,
            second: 56,
            day_of_week: 2,
        };
        let raw: ffi::Tm = dt.into();
        assert_eq!(raw.tm_year, 126);
        assert_eq!(raw.tm_mon, 9);
        assert_eq!(raw.tm_mday, 20);
        assert_eq!(raw.tm_hour, 12);
        assert_eq!(raw.tm_min, 34);
        assert_eq!(raw.tm_sec, 56);
        let back: PlcDateTime = raw.into();
        assert_eq!(back, dt);
    }

    #[test]
    fn c_string_stops_at_nul() {
        let buf = [b'h', b'i', 0, b'x'];
        assert_eq!(c_string(&buf), "hi");
    }

    #[test]
    fn block_info_conversion() {
        let mut raw = ffi::TS7BlockInfo::default();
        raw.blk_number = 7;
        raw.author[0] = b'a';
        raw.author[1] = b'b';
        let info: BlockInfo = raw.into();
        assert_eq!(info.block_number, 7);
        assert_eq!(info.author, "ab");
    }
}
