//! Integration tests: a Snap7 client talking to a Snap7 server over loopback.

use std::sync::MutexGuard;

use crate::ffi;
use crate::tests::{client, cstr, lock_net, server, Object, TEST_PORT};

const DB1: u16 = 1;
const DB2: u16 = 2;
const DB_SIZE: usize = 512;
const SYS_SIZE: usize = 256;

/// A running server with registered areas plus a connected client.
///
/// Field order matters: the client is dropped before the server, and the
/// server before the area buffers it serves.
pub struct Harness {
    pub client: Object,
    pub server: Object,
    db1: Vec<u8>,
    db2: Vec<u8>,
    mk: Vec<u8>,
    pe: Vec<u8>,
    pa: Vec<u8>,
    tm: Vec<u8>,
    ct: Vec<u8>,
    _guard: MutexGuard<'static, ()>,
}

impl Harness {
    pub fn new() -> Self {
        let guard = lock_net();

        let srv = server();
        let port: u16 = TEST_PORT;
        assert_eq!(
            unsafe {
                ffi::Srv_SetParam(
                    srv.raw(),
                    ffi::P_U16_LOCAL_PORT,
                    &port as *const u16 as *mut _,
                )
            },
            0
        );

        let db1 = vec![0u8; DB_SIZE];
        let db2 = vec![0u8; DB_SIZE];
        let mk = vec![0u8; SYS_SIZE];
        let pe = vec![0u8; SYS_SIZE];
        let pa = vec![0u8; SYS_SIZE];
        let tm = vec![0u8; SYS_SIZE];
        let ct = vec![0u8; SYS_SIZE];

        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_DB,
                    DB1,
                    db1.as_ptr() as *mut _,
                    db1.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_DB,
                    DB2,
                    db2.as_ptr() as *mut _,
                    db2.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_MK,
                    0,
                    mk.as_ptr() as *mut _,
                    mk.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_PE,
                    0,
                    pe.as_ptr() as *mut _,
                    pe.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_PA,
                    0,
                    pa.as_ptr() as *mut _,
                    pa.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_TM,
                    0,
                    tm.as_ptr() as *mut _,
                    tm.len() as i32,
                )
            },
            0
        );
        assert_eq!(
            unsafe {
                ffi::Srv_RegisterArea(
                    srv.raw(),
                    ffi::SRV_AREA_CT,
                    0,
                    ct.as_ptr() as *mut _,
                    ct.len() as i32,
                )
            },
            0
        );

        assert_eq!(
            unsafe { ffi::Srv_StartTo(srv.raw(), cstr("127.0.0.1").as_ptr()) },
            0
        );

        let cli = client();
        assert_eq!(
            unsafe {
                ffi::Cli_SetParam(
                    cli.raw(),
                    ffi::P_U16_REMOTE_PORT,
                    &port as *const u16 as *mut _,
                )
            },
            0
        );
        assert_eq!(
            unsafe { ffi::Cli_ConnectTo(cli.raw(), cstr("127.0.0.1").as_ptr(), 0, 2) },
            0
        );

        Harness {
            client: cli,
            server: srv,
            db1,
            db2,
            mk,
            pe,
            pa,
            tm,
            ct,
            _guard: guard,
        }
    }

    pub fn db1(&self) -> &[u8] {
        &self.db1
    }
    pub fn db2(&self) -> &[u8] {
        &self.db2
    }
    pub fn mk(&self) -> &[u8] {
        &self.mk
    }
    pub fn pe(&self) -> &[u8] {
        &self.pe
    }
    pub fn pa(&self) -> &[u8] {
        &self.pa
    }
    pub fn tm(&self) -> &[u8] {
        &self.tm
    }
    pub fn ct(&self) -> &[u8] {
        &self.ct
    }
}

#[test]
fn db_read_write_round_trip() {
    let h = Harness::new();

    let pattern: Vec<u8> = (0..128u8).collect();
    let rc = unsafe {
        ffi::Cli_DBWrite(
            h.client.raw(),
            DB1 as i32,
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(&h.db1()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_DBRead(
            h.client.raw(),
            DB1 as i32,
            0,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn db_read_write_with_offset() {
    let h = Harness::new();

    let pattern: Vec<u8> = (0..32u8).collect();
    let rc = unsafe {
        ffi::Cli_DBWrite(
            h.client.raw(),
            DB2 as i32,
            10,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(&h.db2()[10..10 + pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_DBRead(
            h.client.raw(),
            DB2 as i32,
            10,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn mb_read_write_round_trip() {
    let h = Harness::new();
    let pattern: Vec<u8> = (0..64u8).map(|b| b.wrapping_mul(3)).collect();
    let rc = unsafe {
        ffi::Cli_MBWrite(
            h.client.raw(),
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(&h.mk()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_MBRead(
            h.client.raw(),
            0,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn eb_read_write_round_trip() {
    let h = Harness::new();
    let pattern: Vec<u8> = (0..64u8).map(|b| b.wrapping_add(7)).collect();
    let rc = unsafe {
        ffi::Cli_EBWrite(
            h.client.raw(),
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(&h.pe()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_EBRead(
            h.client.raw(),
            0,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn ab_read_write_round_trip() {
    let h = Harness::new();
    let pattern: Vec<u8> = (0..64u8).map(|b| b ^ 0xAA).collect();
    let rc = unsafe {
        ffi::Cli_ABWrite(
            h.client.raw(),
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(&h.pa()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_ABRead(
            h.client.raw(),
            0,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn tm_read_write_round_trip() {
    let h = Harness::new();
    // Timers are 2 bytes each; the amount is expressed in timer units.
    let amount: i32 = 8;
    let bytes = (amount * 2) as usize;
    let pattern: Vec<u8> = (0..bytes as u8).collect();
    let rc = unsafe { ffi::Cli_TMWrite(h.client.raw(), 0, amount, pattern.as_ptr() as *mut _) };
    assert_eq!(rc, 0);
    assert_eq!(&h.tm()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; bytes];
    let rc = unsafe { ffi::Cli_TMRead(h.client.raw(), 0, amount, out.as_mut_ptr() as *mut _) };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn ct_read_write_round_trip() {
    let h = Harness::new();
    // Counters are 2 bytes each; the amount is expressed in counter units.
    let amount: i32 = 8;
    let bytes = (amount * 2) as usize;
    let pattern: Vec<u8> = (0..bytes as u8).collect();
    let rc = unsafe { ffi::Cli_CTWrite(h.client.raw(), 0, amount, pattern.as_ptr() as *mut _) };
    assert_eq!(rc, 0);
    assert_eq!(&h.ct()[..pattern.len()], pattern.as_slice());

    let mut out = vec![0u8; bytes];
    let rc = unsafe { ffi::Cli_CTRead(h.client.raw(), 0, amount, out.as_mut_ptr() as *mut _) };
    assert_eq!(rc, 0);
    assert_eq!(out, pattern);
}

#[test]
fn read_area_with_word_lengths() {
    let h = Harness::new();

    // Pre-fill the DB from the client side with a known byte pattern.
    let bytes = [0x01u8, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06];
    let rc = unsafe {
        ffi::Cli_DBWrite(
            h.client.raw(),
            DB1 as i32,
            0,
            bytes.len() as i32,
            bytes.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);

    // Read 4 words (8 bytes) with S7WLWord.
    let mut words = [0u16; 4];
    let rc = unsafe {
        ffi::Cli_ReadArea(
            h.client.raw(),
            ffi::S7_AREA_DB,
            DB1 as i32,
            0,
            4,
            ffi::S7_WL_WORD,
            words.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    // Snap7 returns the raw big-endian bytes from the PLC verbatim; on a
    // little-endian host they therefore appear as little-endian words here.
    assert_eq!(words, [0x0001, 0x0203, 0x0405, 0x0607]);
}

#[test]
fn write_area_with_word_lengths() {
    let h = Harness::new();

    let words = [0x1234u16, 0x5678, 0x9ABC, 0xDEF0];
    let rc = unsafe {
        ffi::Cli_WriteArea(
            h.client.raw(),
            ffi::S7_AREA_DB,
            DB1 as i32,
            4,
            4,
            ffi::S7_WL_WORD,
            words.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    // 4 words written at byte offset 4, stored little-endian.
    assert_eq!(
        &h.db1()[4..12],
        &[0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE]
    );
}

#[test]
fn multi_vars_read_and_write() {
    let h = Harness::new();

    let write_words = [0x0102u16, 0x0304, 0x0506];
    let mut write_item = ffi::TS7DataItem {
        area: ffi::S7_AREA_DB,
        word_len: ffi::S7_WL_WORD,
        db_number: DB1 as i32,
        start: 0,
        amount: 3,
        ..Default::default()
    };
    write_item.pdata = write_words.as_ptr() as *mut _;

    let rc = unsafe { ffi::Cli_WriteMultiVars(h.client.raw(), &mut write_item, 1) };
    assert_eq!(rc, 0);
    assert_eq!(write_item.result, 0);
    assert_eq!(&h.db1()[..6], &[0x02, 0x01, 0x04, 0x03, 0x06, 0x05]);

    let mut read_words = [0u16; 3];
    let mut read_item = ffi::TS7DataItem {
        area: ffi::S7_AREA_DB,
        word_len: ffi::S7_WL_WORD,
        db_number: DB1 as i32,
        start: 0,
        amount: 3,
        ..Default::default()
    };
    read_item.pdata = read_words.as_mut_ptr() as *mut _;

    let rc = unsafe { ffi::Cli_ReadMultiVars(h.client.raw(), &mut read_item, 1) };
    assert_eq!(rc, 0);
    assert_eq!(read_item.result, 0);
    assert_eq!(read_words, write_words);
}

#[test]
fn list_blocks() {
    let h = Harness::new();
    let mut blocks = ffi::TS7BlocksList::default();
    let rc = unsafe { ffi::Cli_ListBlocks(h.client.raw(), &mut blocks) };
    assert_eq!(rc, 0);
    // The Snap7 server reports the two registered DBs and no other blocks.
    assert_eq!(blocks.ob_count, 0);
    assert_eq!(blocks.fb_count, 0);
    assert_eq!(blocks.fc_count, 0);
    assert_eq!(blocks.sfb_count, 0);
    assert_eq!(blocks.sfc_count, 0);
    assert_eq!(blocks.db_count, 2);
    assert_eq!(blocks.sdb_count, 0);
}

#[test]
fn list_blocks_of_type() {
    let h = Harness::new();
    let mut items = vec![0u16; 0x2000];
    let mut count: i32 = items.len() as i32;
    let rc = unsafe {
        ffi::Cli_ListBlocksOfType(
            h.client.raw(),
            ffi::BLOCK_DB,
            items.as_mut_ptr(),
            &mut count,
        )
    };
    assert_eq!(rc, 0);
    assert!(count > 0);
}

#[test]
fn get_ag_block_info() {
    let h = Harness::new();
    let mut info = ffi::TS7BlockInfo::default();
    let rc =
        unsafe { ffi::Cli_GetAgBlockInfo(h.client.raw(), ffi::BLOCK_DB, DB1 as i32, &mut info) };
    assert_eq!(rc, 0);
    // The client maps the server's sub-block type (0x0A = DB) into BlkType.
    assert_eq!(info.blk_type, 0x0A);
    assert_eq!(info.blk_number, DB1 as i32);
    assert_eq!(info.mc7_size, DB_SIZE as i32);
}

#[test]
fn get_pg_block_info_invalid_block() {
    let h = Harness::new();
    // A zeroed buffer is not a valid block: LenLoadMem (0) != Size (128).
    let block = vec![0u8; 128];
    let mut info = ffi::TS7BlockInfo::default();
    let rc = unsafe {
        ffi::Cli_GetPgBlockInfo(
            h.client.raw(),
            block.as_ptr() as *mut _,
            &mut info,
            block.len() as i32,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_BLOCK_SIZE);
}

#[test]
fn read_szl_known_id() {
    let h = Harness::new();
    let mut buf = vec![0u8; 0x4000];
    let mut size: i32 = buf.len() as i32;
    let rc = unsafe {
        ffi::Cli_ReadSZL(
            h.client.raw(),
            0x0011,
            0x0000,
            buf.as_mut_ptr() as *mut _,
            &mut size,
        )
    };
    assert_eq!(rc, 0);
    assert!(size > 0);
}

#[test]
fn read_szl_unknown_id() {
    let h = Harness::new();
    let mut buf = vec![0u8; 0x4000];
    let mut size: i32 = buf.len() as i32;
    let rc = unsafe {
        ffi::Cli_ReadSZL(
            h.client.raw(),
            0xFFFF,
            0x0000,
            buf.as_mut_ptr() as *mut _,
            &mut size,
        )
    };
    assert_ne!(rc, 0);
}

#[test]
fn read_szl_list() {
    let h = Harness::new();
    let mut buf = vec![0u8; 0x4000];
    // ItemsCount is an in/out parameter: input is the available room.
    let mut count: i32 = (buf.len() / 2) as i32;
    let rc =
        unsafe { ffi::Cli_ReadSZLList(h.client.raw(), buf.as_mut_ptr() as *mut _, &mut count) };
    assert_eq!(rc, 0);
    assert!(count > 0);
}

#[test]
fn get_order_code() {
    let h = Harness::new();
    let mut order = ffi::TS7OrderCode::default();
    let rc = unsafe { ffi::Cli_GetOrderCode(h.client.raw(), &mut order) };
    assert_eq!(rc, 0);
    assert!(!crate::tests::read_c_string(&order.code).is_empty());
}

#[test]
fn get_cpu_info() {
    let h = Harness::new();
    let mut info = ffi::TS7CpuInfo::default();
    let rc = unsafe { ffi::Cli_GetCpuInfo(h.client.raw(), &mut info) };
    assert_eq!(rc, 0);
    assert!(!crate::tests::read_c_string(&info.module_type_name).is_empty());
}

#[test]
fn get_cp_info() {
    let h = Harness::new();
    let mut info = ffi::TS7CpInfo::default();
    let rc = unsafe { ffi::Cli_GetCpInfo(h.client.raw(), &mut info) };
    assert_eq!(rc, 0);
    assert!(info.max_pdu_length > 0);
    assert!(info.max_connections > 0);
}

#[test]
fn plc_status_and_control() {
    let h = Harness::new();

    let mut status: i32 = -1;
    let rc = unsafe { ffi::Cli_GetPlcStatus(h.client.raw(), &mut status) };
    assert_eq!(rc, 0);
    assert_eq!(status, ffi::CPU_STATUS_RUN);

    assert_eq!(unsafe { ffi::Cli_PlcStop(h.client.raw()) }, 0);
    let rc = unsafe { ffi::Cli_GetPlcStatus(h.client.raw(), &mut status) };
    assert_eq!(rc, 0);
    assert_eq!(status, ffi::CPU_STATUS_STOP);

    assert_eq!(unsafe { ffi::Cli_PlcHotStart(h.client.raw()) }, 0);
    let rc = unsafe { ffi::Cli_GetPlcStatus(h.client.raw(), &mut status) };
    assert_eq!(rc, 0);
    assert_eq!(status, ffi::CPU_STATUS_RUN);

    assert_eq!(unsafe { ffi::Cli_PlcColdStart(h.client.raw()) }, 0);
    let rc = unsafe { ffi::Cli_GetPlcStatus(h.client.raw(), &mut status) };
    assert_eq!(rc, 0);
    assert_eq!(status, ffi::CPU_STATUS_RUN);
}

#[test]
fn plc_date_time_round_trip() {
    let h = Harness::new();

    // The Snap7 server does not persist a set clock: PerformSetClock merely
    // acknowledges the request and PerformGetClock reports the current local
    // time. Verify the API accepts a set and returns a plausible reading.
    let mut set = ffi::Tm {
        tm_sec: 30,
        tm_min: 25,
        tm_hour: 14,
        tm_mday: 19,
        tm_mon: 9,    // October
        tm_year: 126, // 2026 - 1900
        ..Default::default()
    };
    let rc = unsafe { ffi::Cli_SetPlcDateTime(h.client.raw(), &mut set) };
    assert_eq!(rc, 0);

    let mut got = ffi::Tm::default();
    let rc = unsafe { ffi::Cli_GetPlcDateTime(h.client.raw(), &mut got) };
    assert_eq!(rc, 0);
    assert!((0..=59).contains(&got.tm_sec));
    assert!((0..=59).contains(&got.tm_min));
    assert!((0..=23).contains(&got.tm_hour));
    assert!((1..=31).contains(&got.tm_mday));
    assert!((0..=11).contains(&got.tm_mon));
    assert!(got.tm_year >= 100);
}

#[test]
fn set_plc_system_date_time() {
    let h = Harness::new();
    let rc = unsafe { ffi::Cli_SetPlcSystemDateTime(h.client.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn get_protection() {
    let h = Harness::new();
    let mut protection = ffi::TS7Protection::default();
    let rc = unsafe { ffi::Cli_GetProtection(h.client.raw(), &mut protection) };
    assert_eq!(rc, 0);
}

#[test]
fn session_password() {
    let h = Harness::new();
    let password = cstr("secret");
    let rc = unsafe { ffi::Cli_SetSessionPassword(h.client.raw(), password.as_ptr() as *mut _) };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Cli_ClearSessionPassword(h.client.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn copy_ram_to_rom_and_compress() {
    let h = Harness::new();
    let rc = unsafe { ffi::Cli_CopyRamToRom(h.client.raw(), 1000) };
    assert_eq!(rc, 0);
    let rc = unsafe { ffi::Cli_Compress(h.client.raw(), 1000) };
    assert_eq!(rc, 0);
}

#[test]
fn db_get_and_db_fill() {
    let h = Harness::new();

    // Fill the entire DB with 0xAB.
    let rc = unsafe { ffi::Cli_DBFill(h.client.raw(), DB1 as i32, 0xAB) };
    assert_eq!(rc, 0);
    assert!(h.db1().iter().all(|&b| b == 0xAB));

    // DBGet reads the whole DB (MC7Size == DB_SIZE).
    let mut size: i32 = DB_SIZE as i32;
    let mut out = vec![0u8; DB_SIZE];
    let rc = unsafe {
        ffi::Cli_DBGet(
            h.client.raw(),
            DB1 as i32,
            out.as_mut_ptr() as *mut _,
            &mut size,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(size, DB_SIZE as i32);
    assert!(out.iter().all(|&b| b == 0xAB));

    // A too-small buffer is reported as errCliBufferTooSmall.
    let mut small = vec![0u8; 64];
    let mut small_size: i32 = 64;
    let rc = unsafe {
        ffi::Cli_DBGet(
            h.client.raw(),
            DB1 as i32,
            small.as_mut_ptr() as *mut _,
            &mut small_size,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_BUFFER_TOO_SMALL);
}

#[test]
fn exec_time_and_last_error_and_pdu_length() {
    let h = Harness::new();

    let mut time: i32 = -1;
    assert_eq!(
        unsafe { ffi::Cli_GetExecTime(h.client.raw(), &mut time) },
        0
    );
    assert!(time >= 0);

    let mut last: i32 = -1;
    assert_eq!(
        unsafe { ffi::Cli_GetLastError(h.client.raw(), &mut last) },
        0
    );
    assert_eq!(last, 0);

    let mut requested: i32 = -1;
    let mut negotiated: i32 = -1;
    assert_eq!(
        unsafe { ffi::Cli_GetPduLength(h.client.raw(), &mut requested, &mut negotiated) },
        0
    );
    assert!(requested > 0);
    assert!(negotiated > 0);
}

#[test]
fn connected_flag_after_connect() {
    let h = Harness::new();
    let mut connected: i32 = 0;
    let rc = unsafe { ffi::Cli_GetConnected(h.client.raw(), &mut connected) };
    assert_eq!(rc, 0);
    assert_eq!(connected, 1);
}

#[test]
fn iso_exchange_buffer_rejects_malformed_pdu() {
    let h = Harness::new();
    // A 2-byte payload is not a valid S7 PDU: the server treats it as a
    // malformed request and drops the connection, so the exchange fails.
    let mut buf = vec![0u8; 128];
    buf[0] = 0x32;
    buf[1] = 0x01;
    let mut size: i32 = 2;
    let rc = unsafe {
        ffi::Cli_IsoExchangeBuffer(h.client.raw(), buf.as_mut_ptr() as *mut _, &mut size)
    };
    assert_ne!(rc, 0);
}

#[test]
fn server_events_are_generated_on_read() {
    let h = Harness::new();

    // A read triggers a DataRead event.
    let mut out = vec![0u8; 8];
    let rc =
        unsafe { ffi::Cli_DBRead(h.client.raw(), DB1 as i32, 0, 8, out.as_mut_ptr() as *mut _) };
    assert_eq!(rc, 0);

    let mut event = ffi::TSrvEvent::default();
    let mut ready: i32 = 0;
    let rc = unsafe { ffi::Srv_PickEvent(h.server.raw(), &mut event, &mut ready) };
    assert_eq!(rc, 0);
    // At least the PDU-incoming event is queued.
    assert_eq!(ready, 1);
    assert_ne!(event.evt_code, 0);
}

#[test]
fn async_db_read_write_round_trip() {
    let h = Harness::new();

    let pattern: Vec<u8> = (0..64u8).collect();
    let rc = unsafe {
        ffi::Cli_AsDBWrite(
            h.client.raw(),
            DB1 as i32,
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(
        unsafe { ffi::Cli_WaitAsCompletion(h.client.raw(), 5000) },
        0
    );

    let mut out = vec![0u8; pattern.len()];
    let rc = unsafe {
        ffi::Cli_AsDBRead(
            h.client.raw(),
            DB1 as i32,
            0,
            out.len() as i32,
            out.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(
        unsafe { ffi::Cli_WaitAsCompletion(h.client.raw(), 5000) },
        0
    );
    assert_eq!(out, pattern);
}

#[test]
fn async_check_completion() {
    let h = Harness::new();

    let pattern: Vec<u8> = (0..32u8).collect();
    let rc = unsafe {
        ffi::Cli_AsDBWrite(
            h.client.raw(),
            DB2 as i32,
            0,
            pattern.len() as i32,
            pattern.as_ptr() as *mut _,
        )
    };
    assert_eq!(rc, 0);

    let mut op_result: i32 = 0;
    let rc = unsafe { ffi::Cli_CheckAsCompletion(h.client.raw(), &mut op_result) };
    assert!(rc == ffi::JOB_COMPLETE || rc == ffi::JOB_PENDING);

    // Drain the job.
    assert_eq!(
        unsafe { ffi::Cli_WaitAsCompletion(h.client.raw(), 5000) },
        0
    );
}
