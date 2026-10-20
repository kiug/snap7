//! Tests for the error-text helpers (`Cli_ErrorText`, `Srv_ErrorText`,
//! `Par_ErrorText`, `Srv_EventText`).

mod common;

use common::read_c_string;
use snap7::ffi;

fn text_buf() -> Vec<u8> {
    vec![0u8; 1024]
}

#[test]
fn cli_error_text_returns_description_for_known_codes() {
    // A representative set of client error codes.
    let codes = [
        ffi::ERR_CLI_INVALID_PARAMS,
        ffi::ERR_CLI_JOB_PENDING,
        ffi::ERR_CLI_TOO_MANY_ITEMS,
        ffi::ERR_CLI_INVALID_WORD_LEN,
        ffi::ERR_CLI_INVALID_PARAM_NUMBER,
        ffi::ERR_CLI_CANNOT_CHANGE_PARAM,
        ffi::ERR_CLI_ITEM_NOT_AVAILABLE,
        ffi::ERR_LIB_INVALID_OBJECT,
        ffi::ERR_LIB_INVALID_PARAM,
    ];

    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Cli_ErrorText(code, buf.as_mut_ptr() as *mut _, buf.len() as i32) };
        assert_eq!(rc, 0, "Cli_ErrorText({:#x}) should succeed", code);
        let s = read_c_string(&buf);
        assert!(
            !s.is_empty(),
            "Cli_ErrorText({:#x}) must not be empty",
            code
        );
    }
}

#[test]
fn cli_error_text_describes_unknown_code() {
    let mut buf = text_buf();
    let rc = unsafe { ffi::Cli_ErrorText(0x7FFF_0000, buf.as_mut_ptr() as *mut _, 1024) };
    assert_eq!(rc, 0);
    assert!(read_c_string(&buf).contains("Unknown error"));
}

#[test]
fn srv_error_text_returns_description_for_known_codes() {
    let codes = [
        ffi::ERR_SRV_DB_NULL_POINTER,
        ffi::ERR_SRV_AREA_ALREADY_EXISTS,
        ffi::ERR_SRV_UNKNOWN_AREA,
        ffi::ERR_SRV_INVALID_PARAMS,
        ffi::ERR_SRV_TOO_MANY_DB,
        ffi::ERR_SRV_INVALID_PARAM_NUMBER,
        ffi::ERR_SRV_CANNOT_CHANGE_PARAM,
        ffi::ERR_LIB_INVALID_OBJECT,
    ];

    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Srv_ErrorText(code, buf.as_mut_ptr() as *mut _, buf.len() as i32) };
        assert_eq!(rc, 0, "Srv_ErrorText({:#x}) should succeed", code);
        let s = read_c_string(&buf);
        assert!(
            !s.is_empty(),
            "Srv_ErrorText({:#x}) must not be empty",
            code
        );
    }
}

#[test]
fn srv_error_text_describes_unknown_code() {
    let mut buf = text_buf();
    let rc = unsafe { ffi::Srv_ErrorText(0x7FFF_0000, buf.as_mut_ptr() as *mut _, 1024) };
    assert_eq!(rc, 0);
    assert!(read_c_string(&buf).contains("Unknown error"));
}

#[test]
fn par_error_text_returns_description_for_known_codes() {
    let codes = [
        ffi::ERR_LIB_INVALID_OBJECT,
        ffi::ERR_LIB_INVALID_PARAM,
        0x0020_0000,
        0x0030_0000,
    ];

    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Par_ErrorText(code, buf.as_mut_ptr() as *mut _, buf.len() as i32) };
        assert_eq!(rc, 0, "Par_ErrorText({:#x}) should succeed", code);
        let s = read_c_string(&buf);
        assert!(
            !s.is_empty(),
            "Par_ErrorText({:#x}) must not be empty",
            code
        );
    }
}

#[test]
fn par_error_text_describes_unknown_code() {
    let mut buf = text_buf();
    let rc = unsafe { ffi::Par_ErrorText(0x7FFF_0000, buf.as_mut_ptr() as *mut _, 1024) };
    assert_eq!(rc, 0);
    assert!(read_c_string(&buf).contains("Unknown error"));
}

#[test]
fn srv_event_text_returns_description() {
    let event = ffi::TSrvEvent {
        evt_time: 0,
        evt_sender: 0,
        evt_code: ffi::EVC_DATA_READ,
        evt_ret_code: 0,
        evt_param1: 0,
        evt_param2: 0,
        evt_param3: 0,
        evt_param4: 0,
    };

    let mut buf = text_buf();
    let rc = unsafe {
        ffi::Srv_EventText(
            &event as *const ffi::TSrvEvent as *mut _,
            buf.as_mut_ptr() as *mut _,
            buf.len() as i32,
        )
    };
    assert_eq!(rc, 0);
    assert!(!read_c_string(&buf).is_empty());
}

#[test]
fn text_functions_handle_zero_text_len() {
    let mut buf = text_buf();
    // A zero length must not crash and must be treated as "no room".
    let rc = unsafe { ffi::Cli_ErrorText(0, buf.as_mut_ptr() as *mut _, 0) };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_ErrorText(0, buf.as_mut_ptr() as *mut _, 0) };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Par_ErrorText(0, buf.as_mut_ptr() as *mut _, 0) };
    assert_eq!(rc, 0);
}

#[test]
fn cli_error_text_covers_all_client_error_codes() {
    let codes = [
        ffi::ERR_NEGOTIATING_PDU,
        ffi::ERR_CLI_INVALID_PARAMS,
        ffi::ERR_CLI_JOB_PENDING,
        ffi::ERR_CLI_TOO_MANY_ITEMS,
        ffi::ERR_CLI_INVALID_WORD_LEN,
        ffi::ERR_CLI_PARTIAL_DATA_WRITTEN,
        ffi::ERR_CLI_SIZE_OVER_PDU,
        ffi::ERR_CLI_INVALID_PLC_ANSWER,
        ffi::ERR_CLI_ADDRESS_OUT_OF_RANGE,
        ffi::ERR_CLI_INVALID_TRANSPORT_SIZE,
        ffi::ERR_CLI_WRITE_DATA_SIZE_MISMATCH,
        ffi::ERR_CLI_ITEM_NOT_AVAILABLE,
        ffi::ERR_CLI_INVALID_VALUE,
        ffi::ERR_CLI_CANNOT_START_PLC,
        ffi::ERR_CLI_ALREADY_RUN,
        ffi::ERR_CLI_CANNOT_STOP_PLC,
        ffi::ERR_CLI_CANNOT_COPY_RAM_TO_ROM,
        ffi::ERR_CLI_CANNOT_COMPRESS,
        ffi::ERR_CLI_ALREADY_STOP,
        ffi::ERR_CLI_FUN_NOT_AVAILABLE,
        ffi::ERR_CLI_UPLOAD_SEQUENCE_FAILED,
        ffi::ERR_CLI_INVALID_DATA_SIZE_RECVD,
        ffi::ERR_CLI_INVALID_BLOCK_TYPE,
        ffi::ERR_CLI_INVALID_BLOCK_NUMBER,
        ffi::ERR_CLI_INVALID_BLOCK_SIZE,
        ffi::ERR_CLI_DOWNLOAD_SEQUENCE_FAILED,
        ffi::ERR_CLI_INSERT_REFUSED,
        ffi::ERR_CLI_DELETE_REFUSED,
        ffi::ERR_CLI_NEED_PASSWORD,
        ffi::ERR_CLI_INVALID_PASSWORD,
        ffi::ERR_CLI_NO_PASSWORD_TO_SET_OR_CLEAR,
        ffi::ERR_CLI_JOB_TIMEOUT,
        ffi::ERR_CLI_PARTIAL_DATA_READ,
        ffi::ERR_CLI_BUFFER_TOO_SMALL,
        ffi::ERR_CLI_FUNCTION_REFUSED,
        ffi::ERR_CLI_DESTROYING,
        ffi::ERR_CLI_INVALID_PARAM_NUMBER,
        ffi::ERR_CLI_CANNOT_CHANGE_PARAM,
    ];
    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Cli_ErrorText(code, buf.as_mut_ptr() as *mut _, 1024) };
        assert_eq!(rc, 0);
        assert!(!read_c_string(&buf).is_empty(), "code {:#x}", code);
    }
}

#[test]
fn srv_error_text_covers_all_server_error_codes() {
    let codes = [
        ffi::ERR_NEGOTIATING_PDU, // errSrvCannotStart
        ffi::ERR_SRV_DB_NULL_POINTER,
        ffi::ERR_SRV_AREA_ALREADY_EXISTS,
        ffi::ERR_SRV_UNKNOWN_AREA,
        ffi::ERR_SRV_INVALID_PARAMS,
        ffi::ERR_SRV_TOO_MANY_DB,
        ffi::ERR_SRV_INVALID_PARAM_NUMBER,
        ffi::ERR_SRV_CANNOT_CHANGE_PARAM,
    ];
    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Srv_ErrorText(code, buf.as_mut_ptr() as *mut _, 1024) };
        assert_eq!(rc, 0);
        assert!(!read_c_string(&buf).is_empty(), "code {:#x}", code);
    }
}

#[test]
fn par_error_text_covers_all_partner_error_codes() {
    let codes = [
        ffi::ERR_PAR_ADDRESS_IN_USE,
        ffi::ERR_PAR_NO_ROOM,
        ffi::ERR_PAR_SERVER_NO_ROOM,
        ffi::ERR_PAR_INVALID_PARAMS,
        ffi::ERR_PAR_NOT_LINKED,
        ffi::ERR_PAR_BUSY,
        ffi::ERR_PAR_FRAME_TIMEOUT,
        ffi::ERR_PAR_INVALID_PDU,
        ffi::ERR_PAR_SEND_TIMEOUT,
        ffi::ERR_PAR_RECV_TIMEOUT,
        ffi::ERR_PAR_SEND_REFUSED,
        ffi::ERR_PAR_NEGOTIATING_PDU,
        ffi::ERR_PAR_SENDING_BLOCK,
        ffi::ERR_PAR_RECVING_BLOCK,
        ffi::ERR_PAR_BIND_ERROR,
        ffi::ERR_PAR_DESTROYING,
        ffi::ERR_PAR_INVALID_PARAM_NUMBER,
        ffi::ERR_PAR_CANNOT_CHANGE_PARAM,
        ffi::ERR_PAR_BUFFER_TOO_SMALL,
    ];
    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Par_ErrorText(code, buf.as_mut_ptr() as *mut _, 1024) };
        assert_eq!(rc, 0);
        assert!(!read_c_string(&buf).is_empty(), "code {:#x}", code);
    }
}

#[test]
fn cli_error_text_covers_iso_error_codes() {
    let codes = [
        ffi::ERR_ISO_CONNECT,
        ffi::ERR_ISO_DISCONNECT,
        ffi::ERR_ISO_INVALID_PDU,
        ffi::ERR_ISO_INVALID_DATA_SIZE,
        ffi::ERR_ISO_NULL_POINTER,
        ffi::ERR_ISO_SHORT_PACKET,
        ffi::ERR_ISO_TOO_MANY_FRAGMENTS,
        ffi::ERR_ISO_PDU_OVERFLOW,
        ffi::ERR_ISO_SEND_PACKET,
        ffi::ERR_ISO_RECV_PACKET,
        ffi::ERR_ISO_INVALID_PARAMS,
    ];
    for code in codes {
        let mut buf = text_buf();
        let rc = unsafe { ffi::Cli_ErrorText(code, buf.as_mut_ptr() as *mut _, 1024) };
        assert_eq!(rc, 0);
        assert!(read_c_string(&buf).contains("ISO"), "code {:#x}", code);
    }
}
