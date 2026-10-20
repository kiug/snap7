//! Direct FFI round-trip tests, kept alongside the safe-wrapper tests so the
//! raw C API and its struct constructors stay covered independently.

mod common;

use std::os::raw::c_void;

use common::{lock_net, TEST_PORT};
use snap7::ffi;
use snap7::{Client, Server, ServerArea};

/// Exercises every hand-written `Default` impl on the FFI structs.
#[test]
fn ffi_struct_defaults() {
    let _ = ffi::TS7DataItem::default();
    let _ = ffi::TS7BlockInfo::default();
    let _ = ffi::TS7OrderCode::default();
    let _ = ffi::TS7CpuInfo::default();
    let _ = ffi::TS7CpInfo::default();
    let _ = ffi::TS7Protection::default();
    let _ = ffi::TS7Tag::default();
    let _ = ffi::TSrvEvent::default();
    let _ = ffi::Tm::default();
    let _ = ffi::TS7BlocksList::default();
}

#[test]
fn ffi_multi_vars_szl_and_order_code() {
    let _lock = lock_net();

    let mut db = vec![0u8; 64];
    let server = Server::create();
    server.set_local_port(TEST_PORT).unwrap();
    server
        .register_area(ServerArea::DataBlock, 1, &mut db)
        .unwrap();
    server.start_to("127.0.0.1").unwrap();

    let client = Client::create();
    client.set_remote_port(TEST_PORT).unwrap();
    client.connect_to("127.0.0.1", 0, 2).unwrap();

    // Raw multi-vars write via the C API.
    let mut data_a = [0x11u8, 0x22, 0x33, 0x44];
    let mut data_b = [0x55u8, 0x66, 0x77, 0x88];
    let mut items = [
        ffi::TS7DataItem {
            area: ffi::S7_AREA_DB,
            word_len: ffi::S7_WL_BYTE,
            result: 0,
            db_number: 1,
            start: 0,
            amount: 4,
            pdata: data_a.as_mut_ptr() as *mut c_void,
        },
        ffi::TS7DataItem {
            area: ffi::S7_AREA_DB,
            word_len: ffi::S7_WL_BYTE,
            result: 0,
            db_number: 1,
            start: 4,
            amount: 4,
            pdata: data_b.as_mut_ptr() as *mut c_void,
        },
    ];
    assert_eq!(
        unsafe { ffi::Cli_WriteMultiVars(client.raw(), items.as_mut_ptr(), 2) },
        0
    );

    // Raw SZL read via the C API.
    let mut szl = vec![0u8; 1024];
    let mut size = szl.len() as i32;
    assert_eq!(
        unsafe {
            ffi::Cli_ReadSZL(
                client.raw(),
                0x0011,
                0,
                szl.as_mut_ptr() as *mut c_void,
                &mut size,
            )
        },
        0
    );
    assert!(size > 0);

    // Raw order code via the C API.
    let mut order = ffi::TS7OrderCode::default();
    assert_eq!(
        unsafe { ffi::Cli_GetOrderCode(client.raw(), &mut order) },
        0
    );
    assert!(order.code[0] != 0);
}
