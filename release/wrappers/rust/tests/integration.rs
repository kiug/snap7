//! Integration tests: a safe-API client talking to a safe-API server over
//! loopback.

mod common;

use std::sync::MutexGuard;

use common::{lock_net, TEST_PORT};
use snap7::{
    Area, BlockType, Client, CpuStatus, DataItem, PlcDateTime, Server, ServerArea, WordLen,
};

const DB1: u16 = 1;
const DB2: u16 = 2;
const DB_SIZE: usize = 512;
const SYS_SIZE: usize = 256;

/// A running server with registered areas plus a connected client.
///
/// Field order matters: the client is dropped before the server, and the
/// server before the area buffers it serves.
pub struct Harness {
    pub client: Client,
    pub server: Server,
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

        let mut db1 = vec![0u8; DB_SIZE];
        let mut db2 = vec![0u8; DB_SIZE];
        let mut mk = vec![0u8; SYS_SIZE];
        let mut pe = vec![0u8; SYS_SIZE];
        let mut pa = vec![0u8; SYS_SIZE];
        let mut tm = vec![0u8; SYS_SIZE];
        let mut ct = vec![0u8; SYS_SIZE];

        let server = Server::create();
        server.set_local_port(TEST_PORT).unwrap();
        server
            .register_area(ServerArea::DataBlock, DB1, &mut db1)
            .unwrap();
        server
            .register_area(ServerArea::DataBlock, DB2, &mut db2)
            .unwrap();
        server
            .register_area(ServerArea::Merkers, 0, &mut mk)
            .unwrap();
        server
            .register_area(ServerArea::ProcessInputs, 0, &mut pe)
            .unwrap();
        server
            .register_area(ServerArea::ProcessOutputs, 0, &mut pa)
            .unwrap();
        server
            .register_area(ServerArea::Timers, 0, &mut tm)
            .unwrap();
        server
            .register_area(ServerArea::Counters, 0, &mut ct)
            .unwrap();
        server.start_to("127.0.0.1").unwrap();

        let client = Client::create();
        client.set_remote_port(TEST_PORT).unwrap();
        client.connect_to("127.0.0.1", 0, 2).unwrap();

        Harness {
            client,
            server,
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

/// Builds a structurally valid S7 DB block image for `Cli_Download`.
fn valid_block() -> Vec<u8> {
    let mut block = vec![0u8; 128];
    block[5] = 0x0A; // SubBlkType = DB
    block[6] = 0x00;
    block[7] = 0x01; // BlkNum = 1
    block[8] = 0x00;
    block[9] = 0x00;
    block[10] = 0x00;
    block[11] = 0x80; // LenLoadMem = 128
    block[34] = 0x00;
    block[35] = 0x10; // MC7Len = 16
    block
}

#[test]
fn connected_flag() {
    let h = Harness::new();
    assert!(h.client.is_connected().unwrap());
}

#[test]
fn db_read_write_round_trip() {
    let h = Harness::new();
    let pattern: Vec<u8> = (0..128u8).collect();
    h.client.db_write(DB1 as i32, 0, &pattern).unwrap();

    let mut out = vec![0u8; 128];
    h.client.db_read(DB1 as i32, 0, &mut out).unwrap();
    assert_eq!(out, pattern);
    assert_eq!(&h.db1()[..128], &pattern[..]);
}

#[test]
fn mb_eb_ab_round_trip() {
    let h = Harness::new();
    let pattern: Vec<u8> = (0..16u8).collect();

    h.client.mb_write(0, &pattern).unwrap();
    let mut out = vec![0u8; 16];
    h.client.mb_read(0, &mut out).unwrap();
    assert_eq!(out, pattern);
    assert_eq!(&h.mk()[..16], &pattern[..]);

    h.client.eb_write(0, &pattern).unwrap();
    h.client.eb_read(0, &mut out).unwrap();
    assert_eq!(out, pattern);
    assert_eq!(&h.pe()[..16], &pattern[..]);

    h.client.ab_write(0, &pattern).unwrap();
    h.client.ab_read(0, &mut out).unwrap();
    assert_eq!(out, pattern);
    assert_eq!(&h.pa()[..16], &pattern[..]);
}

#[test]
fn tm_ct_round_trip() {
    let h = Harness::new();
    let data = [0x12u8, 0x34, 0x56, 0x78];

    h.client.tm_write(0, 2, &data).unwrap();
    let mut out = [0u8; 4];
    h.client.tm_read(0, 2, &mut out).unwrap();
    assert_eq!(out, data);
    assert_eq!(&h.tm()[..4], &data[..]);

    h.client.ct_write(0, 2, &data).unwrap();
    h.client.ct_read(0, 2, &mut out).unwrap();
    assert_eq!(out, data);
    assert_eq!(&h.ct()[..4], &data[..]);
}

#[test]
fn read_write_area_with_word_lengths() {
    let h = Harness::new();
    let bytes = [0x01u8, 0x00, 0x03, 0x02];

    // Word access swaps to big-endian on the wire.
    let words = [0x0001u16, 0x0203];
    h.client
        .write_area(Area::DataBlock, DB1 as i32, 0, 2, WordLen::Word, &bytes)
        .unwrap();

    let mut out = [0u8; 4];
    h.client
        .read_area(Area::DataBlock, DB1 as i32, 0, 2, WordLen::Word, &mut out)
        .unwrap();
    // Raw byte order matches the written bytes.
    assert_eq!(out, bytes);
    let _ = words;
}

#[test]
fn bit_read_write() {
    let h = Harness::new();
    let bit = [1u8];
    h.client
        .write_area(Area::DataBlock, DB1 as i32, 0, 1, WordLen::Bit, &bit)
        .unwrap();
    let mut out = [0u8; 1];
    h.client
        .read_area(Area::DataBlock, DB1 as i32, 0, 1, WordLen::Bit, &mut out)
        .unwrap();
    assert_eq!(out, bit);
}

#[test]
fn multi_vars_read_write() {
    let h = Harness::new();
    let mut a = [0u8; 4];
    let mut b = [0u8; 4];

    let mut write_items = [
        DataItem {
            area: Area::DataBlock,
            word_len: WordLen::Byte,
            db_number: DB1 as i32,
            start: 0,
            amount: 4,
            result: 0,
            data: &mut [0x11, 0x22, 0x33, 0x44],
        },
        DataItem {
            area: Area::DataBlock,
            word_len: WordLen::Byte,
            db_number: DB1 as i32,
            start: 4,
            amount: 4,
            result: 0,
            data: &mut [0x55, 0x66, 0x77, 0x88],
        },
    ];
    h.client.write_multi_vars(&mut write_items).unwrap();
    assert_eq!(write_items[0].result, 0);
    assert_eq!(write_items[1].result, 0);

    let mut read_items = [
        DataItem {
            area: Area::DataBlock,
            word_len: WordLen::Byte,
            db_number: DB1 as i32,
            start: 0,
            amount: 4,
            result: 0,
            data: &mut a,
        },
        DataItem {
            area: Area::DataBlock,
            word_len: WordLen::Byte,
            db_number: DB1 as i32,
            start: 4,
            amount: 4,
            result: 0,
            data: &mut b,
        },
    ];
    h.client.read_multi_vars(&mut read_items).unwrap();
    assert_eq!(a, [0x11, 0x22, 0x33, 0x44]);
    assert_eq!(b, [0x55, 0x66, 0x77, 0x88]);
}

#[test]
fn list_blocks() {
    let h = Harness::new();
    let list = h.client.list_blocks().unwrap();
    assert_eq!(list.db_count, 2);
}

#[test]
fn db2_round_trip() {
    let h = Harness::new();
    let pattern = [1u8, 2, 3, 4];
    h.client.db_write(DB2 as i32, 0, &pattern).unwrap();
    let mut out = [0u8; 4];
    h.client.db_read(DB2 as i32, 0, &mut out).unwrap();
    assert_eq!(out, pattern);
    assert_eq!(&h.db2()[..4], &pattern[..]);
}

#[test]
fn get_ag_block_info() {
    let h = Harness::new();
    let info = h
        .client
        .get_ag_block_info(BlockType::Db, DB1 as i32)
        .unwrap();
    assert_eq!(info.block_number, DB1 as i32);
}

#[test]
fn list_blocks_of_type() {
    let h = Harness::new();
    let blocks = h.client.list_blocks_of_type(BlockType::Db).unwrap();
    assert_eq!(blocks.len(), 2);
    assert_eq!(blocks[0], DB1);
    assert_eq!(blocks[1], DB2);
}

#[test]
fn db_get_and_db_fill() {
    let h = Harness::new();
    h.client.db_fill(DB1 as i32, 0xAB).unwrap();
    let data = h.client.db_get(DB1 as i32).unwrap();
    assert_eq!(data.len(), DB_SIZE);
    assert!(data.iter().all(|&b| b == 0xAB));
}

#[test]
fn upload_download_delete() {
    let h = Harness::new();
    h.client.download(DB1 as i32, &valid_block()).unwrap();
    h.client.delete(BlockType::Db, DB1 as i32).unwrap();

    // Upload a freshly-written DB.
    let pattern: Vec<u8> = (0..64u8).collect();
    h.client.db_write(DB1 as i32, 0, &pattern).unwrap();
    let data = h.client.upload(BlockType::Db, DB1 as i32).unwrap();
    assert_eq!(data.len(), DB_SIZE);
    assert_eq!(&data[..64], &pattern[..]);
}

#[test]
fn full_upload() {
    let h = Harness::new();
    let data = h.client.full_upload(BlockType::Db, DB1 as i32).unwrap();
    // Full upload returns the 36-byte block header plus the DB data.
    assert_eq!(data.len(), 36 + DB_SIZE);
}

#[test]
fn plc_date_time() {
    let h = Harness::new();
    let now = h.client.get_plc_date_time().unwrap();
    assert!(now.year >= 1984);

    // Setting to the host system time works.
    h.client.set_plc_system_date_time().unwrap();

    // The simulated server serves the current time rather than persisting an
    // explicit date, so just exercise the explicit setter without round-trip.
    let target = PlcDateTime {
        year: 2030,
        month: 6,
        day: 15,
        hour: 10,
        minute: 20,
        second: 30,
        day_of_week: 0,
    };
    h.client.set_plc_date_time(target).unwrap();
}

#[test]
fn system_info() {
    let h = Harness::new();
    let order = h.client.get_order_code().unwrap();
    assert!(!order.code.is_empty());
    let cpu = h.client.get_cpu_info().unwrap();
    assert!(!cpu.module_type_name.is_empty());
    let cp = h.client.get_cp_info().unwrap();
    assert!(cp.max_pdu_length > 0);

    let szl = h.client.read_szl(0x0011, 0x0000).unwrap();
    assert!(!szl.is_empty());
    let list = h.client.read_szl_list().unwrap();
    assert!(!list.is_empty());
}

#[test]
fn plc_status_and_control() {
    let h = Harness::new();
    assert_eq!(h.client.get_plc_status().unwrap(), CpuStatus::Run);
    h.client.plc_stop().unwrap();
    assert_eq!(h.client.get_plc_status().unwrap(), CpuStatus::Stop);
    h.client.plc_hot_start().unwrap();
    assert_eq!(h.client.get_plc_status().unwrap(), CpuStatus::Run);

    h.client.copy_ram_to_rom(1000).unwrap();
    h.client.compress(1000).unwrap();
}

#[test]
fn protection_and_password() {
    let h = Harness::new();
    let _prot = h.client.get_protection().unwrap();
    h.client.set_session_password("secret").unwrap();
    h.client.clear_session_password().unwrap();
}

#[test]
fn exec_time_and_pdu_length() {
    let h = Harness::new();
    let _time = h.client.get_exec_time().unwrap();
    let (requested, negotiated) = h.client.get_pdu_length().unwrap();
    assert!(requested > 0);
    assert!(negotiated > 0);
    assert_eq!(h.client.get_last_error().unwrap(), 0);
}

#[test]
fn async_read_write() {
    let h = Harness::new();
    let pattern = [0xAAu8, 0xBB, 0xCC, 0xDD];
    let mut out = [0u8; 4];

    h.client
        .as_write_area(Area::DataBlock, DB1 as i32, 0, 4, WordLen::Byte, &pattern)
        .unwrap();
    h.client.wait_as_completion(5000).unwrap();

    h.client
        .as_read_area(Area::DataBlock, DB1 as i32, 0, 4, WordLen::Byte, &mut out)
        .unwrap();
    h.client.wait_as_completion(5000).unwrap();
    assert_eq!(out, pattern);
}

#[test]
fn disconnect() {
    let h = Harness::new();
    assert!(h.client.is_connected().unwrap());
    h.client.disconnect().unwrap();
    assert!(!h.client.is_connected().unwrap());
}
