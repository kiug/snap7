//! Client lifecycle, parameter and error-path tests that do not require a
//! running server.

mod common;

use std::ffi::CString;

use common::{client, cstr};
use snap7::ffi;

#[test]
fn create_and_destroy_client() {
    // The guard creates and destroys the client; both are exercised here.
    let c = client();
    assert_ne!(c.raw(), 0);
}

#[test]
fn destroy_null_client_is_a_noop() {
    let mut raw = 0usize;
    unsafe { ffi::Cli_Destroy(&mut raw) };
    assert_eq!(raw, 0);
}

#[test]
fn functions_on_null_client_return_invalid_object() {
    let mut value: i32 = 0;
    let mut buffer = [0u8; 4];

    assert_eq!(unsafe { ffi::Cli_Connect(0) }, ffi::ERR_LIB_INVALID_OBJECT);
    assert_eq!(
        unsafe { ffi::Cli_Disconnect(0) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_ConnectTo(0, cstr("127.0.0.1").as_ptr(), 0, 2) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_SetConnectionType(0, ffi::CONNTYPE_PG) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_SetConnectionParams(0, cstr("127.0.0.1").as_ptr(), 0x0100, 0x0200) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_GetParam(0, ffi::P_U16_REMOTE_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_SetParam(0, ffi::P_U16_REMOTE_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_SetAsCallback(0, None, std::ptr::null_mut()) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe {
            ffi::Cli_ReadArea(
                0,
                ffi::S7_AREA_DB,
                1,
                0,
                1,
                ffi::S7_WL_BYTE,
                buffer.as_mut_ptr() as *mut _,
            )
        },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe {
            ffi::Cli_WriteArea(
                0,
                ffi::S7_AREA_DB,
                1,
                0,
                1,
                ffi::S7_WL_BYTE,
                buffer.as_mut_ptr() as *mut _,
            )
        },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_GetConnected(0, &mut value) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_GetLastError(0, &mut value) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
}

#[test]
fn get_param_invalid_number() {
    let c = client();
    let mut value: i32 = 0;
    let rc = unsafe { ffi::Cli_GetParam(c.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAM_NUMBER);
}

#[test]
fn set_param_invalid_number() {
    let c = client();
    let mut value: i32 = 42;
    let rc = unsafe { ffi::Cli_SetParam(c.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAM_NUMBER);
}

#[test]
fn remote_port_param_round_trip() {
    let c = client();
    let port: u16 = 1234;
    let rc = unsafe {
        ffi::Cli_SetParam(
            c.raw(),
            ffi::P_U16_REMOTE_PORT,
            &port as *const u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);

    let mut got: u16 = 0;
    let rc = unsafe {
        ffi::Cli_GetParam(
            c.raw(),
            ffi::P_U16_REMOTE_PORT,
            &mut got as *mut u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(got, port);
}

#[test]
fn integer_params_round_trip() {
    let c = client();

    let cases: [(i32, i32); 4] = [
        (ffi::P_I32_PING_TIMEOUT, 5000),
        (ffi::P_I32_SEND_TIMEOUT, 1234),
        (ffi::P_I32_RECV_TIMEOUT, 4321),
        (ffi::P_I32_PDU_REQUEST, 960),
    ];

    for (param, value) in cases {
        let rc = unsafe { ffi::Cli_SetParam(c.raw(), param, &value as *const i32 as *mut _) };
        assert_eq!(rc, 0, "SetParam({param}) should succeed");

        let mut got: i32 = 0;
        let rc = unsafe { ffi::Cli_GetParam(c.raw(), param, &mut got as *mut i32 as *mut _) };
        assert_eq!(rc, 0, "GetParam({param}) should succeed");
        assert_eq!(got, value, "GetParam({param}) should round-trip");
    }
}

#[test]
fn connection_type_and_connection_params_succeed() {
    let c = client();

    let rc = unsafe { ffi::Cli_SetConnectionType(c.raw(), ffi::CONNTYPE_BASIC) };
    assert_eq!(rc, 0);

    let rc = unsafe {
        ffi::Cli_SetConnectionParams(c.raw(), cstr("127.0.0.1").as_ptr(), 0x0100, 0x0200)
    };
    assert_eq!(rc, 0);
}

#[test]
fn read_area_invalid_word_len() {
    let c = client();
    let mut buffer = [0u8; 16];
    let rc = unsafe {
        ffi::Cli_ReadArea(
            c.raw(),
            ffi::S7_AREA_DB,
            1,
            0,
            1,
            0x99,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_WORD_LEN);
}

#[test]
fn read_area_invalid_params() {
    let c = client();
    let mut buffer = [0u8; 16];

    // Amount < 1
    let rc = unsafe {
        ffi::Cli_ReadArea(
            c.raw(),
            ffi::S7_AREA_DB,
            1,
            0,
            0,
            ffi::S7_WL_BYTE,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAMS);

    // Negative DB number
    let rc = unsafe {
        ffi::Cli_ReadArea(
            c.raw(),
            ffi::S7_AREA_DB,
            -1,
            0,
            1,
            ffi::S7_WL_BYTE,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAMS);
}

#[test]
fn read_area_without_connection_fails() {
    let c = client();
    let mut buffer = [0u8; 16];
    let rc = unsafe {
        ffi::Cli_ReadArea(
            c.raw(),
            ffi::S7_AREA_DB,
            1,
            0,
            1,
            ffi::S7_WL_BYTE,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    assert_ne!(rc, 0, "ReadArea without a connection must fail");
}

#[test]
fn connect_without_server_fails() {
    let c = client();
    let rc = unsafe { ffi::Cli_ConnectTo(c.raw(), cstr("127.0.0.1").as_ptr(), 0, 2) };
    assert_ne!(rc, 0, "Connect without a server must fail");
}

#[test]
fn disconnect_without_connection_succeeds() {
    let c = client();
    let rc = unsafe { ffi::Cli_Disconnect(c.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn connected_is_false_before_connect() {
    let c = client();
    let mut connected: i32 = -1;
    let rc = unsafe { ffi::Cli_GetConnected(c.raw(), &mut connected) };
    assert_eq!(rc, 0);
    assert_eq!(connected, 0);
}

#[test]
fn get_last_error_is_zero_initially() {
    let c = client();
    let mut last: i32 = -1;
    let rc = unsafe { ffi::Cli_GetLastError(c.raw(), &mut last) };
    assert_eq!(rc, 0);
    assert_eq!(last, 0);
}

#[test]
fn set_as_callback_succeeds() {
    let c = client();
    let rc = unsafe { ffi::Cli_SetAsCallback(c.raw(), None, std::ptr::null_mut()) };
    assert_eq!(rc, 0);
}

#[test]
fn check_as_completion_without_job() {
    let c = client();
    let mut op_result: i32 = -1;
    let rc = unsafe { ffi::Cli_CheckAsCompletion(c.raw(), &mut op_result) };
    // No job pending: returns JobComplete and opResult is the (zero) result.
    assert_eq!(rc, ffi::JOB_COMPLETE);
    assert_eq!(op_result, 0);
}

#[test]
fn wait_as_completion_without_job() {
    let c = client();
    let rc = unsafe { ffi::Cli_WaitAsCompletion(c.raw(), 100) };
    assert_eq!(rc, 0);
}

#[test]
fn async_write_area_invalid_word_len() {
    let c = client();
    let mut buffer = [0u8; 16];
    let rc = unsafe {
        ffi::Cli_AsWriteArea(
            c.raw(),
            ffi::S7_AREA_DB,
            1,
            0,
            1,
            0x99,
            buffer.as_mut_ptr() as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_WORD_LEN);
}

#[test]
fn async_copy_ram_to_rom_invalid_timeout() {
    let c = client();
    let rc = unsafe { ffi::Cli_AsCopyRamToRom(c.raw(), 0) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAMS);
}

#[test]
fn async_compress_invalid_timeout() {
    let c = client();
    let rc = unsafe { ffi::Cli_AsCompress(c.raw(), 0) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_PARAMS);
}

#[test]
fn async_db_get_invalid_size() {
    let c = client();
    let mut buffer = [0u8; 64];
    let mut size: i32 = 0;
    let rc = unsafe { ffi::Cli_AsDBGet(c.raw(), 1, buffer.as_mut_ptr() as *mut _, &mut size) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_BLOCK_SIZE);
}

#[test]
fn async_download_invalid_size() {
    let c = client();
    let mut buffer = [0u8; 64];
    let rc = unsafe { ffi::Cli_AsDownload(c.raw(), 1, buffer.as_mut_ptr() as *mut _, 0) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_BLOCK_SIZE);
}

#[test]
fn list_blocks_of_type_invalid_count() {
    let c = client();
    let mut items = [0u16; 4];
    let mut count: i32 = 0;
    let rc = unsafe {
        ffi::Cli_ListBlocksOfType(c.raw(), ffi::BLOCK_DB, items.as_mut_ptr(), &mut count)
    };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_BLOCK_SIZE);
}

#[test]
fn db_get_invalid_size() {
    let c = client();
    let mut buffer = [0u8; 64];
    let mut size: i32 = 0;
    let rc = unsafe { ffi::Cli_DBGet(c.raw(), 1, buffer.as_mut_ptr() as *mut _, &mut size) };
    assert_eq!(rc, ffi::ERR_CLI_INVALID_BLOCK_SIZE);
}

#[test]
fn set_session_password_on_null_client() {
    let password = CString::new("secret").unwrap();
    assert_eq!(
        unsafe { ffi::Cli_SetSessionPassword(0, password.as_ptr() as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Cli_ClearSessionPassword(0) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
}
