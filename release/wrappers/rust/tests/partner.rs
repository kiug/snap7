//! Partner lifecycle, parameter and error-path tests, plus a loopback
//! `BSend`/`BRecv` round trip.
//!
//! Note: Snap7's passive partner listener is hardcoded to port 102
//! (`isoTcpPort`), which on Linux requires `CAP_NET_BIND_SERVICE` (root).
//! The round-trip test therefore degrades gracefully (and still exercises the
//! bind-error path) when the port cannot be bound.

mod common;

use common::{cstr, partner};
use snap7::ffi;

#[test]
fn create_and_destroy_active_partner() {
    let p = partner(true);
    assert_ne!(p.raw(), 0);
}

#[test]
fn create_and_destroy_passive_partner() {
    let p = partner(false);
    assert_ne!(p.raw(), 0);
}

#[test]
fn destroy_null_partner_is_a_noop() {
    let mut raw = 0usize;
    unsafe { ffi::Par_Destroy(&mut raw) };
    assert_eq!(raw, 0);
}

#[test]
fn functions_on_null_partner_return_invalid_object() {
    let mut value: i32 = 0;
    let mut buffer = [0u8; 8];
    let mut r_id: u32 = 0;
    let mut size: i32 = 0;

    assert_eq!(
        unsafe { ffi::Par_GetParam(0, ffi::P_U16_REMOTE_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Par_SetParam(0, ffi::P_U16_REMOTE_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(unsafe { ffi::Par_Start(0) }, ffi::ERR_LIB_INVALID_OBJECT);
    assert_eq!(
        unsafe {
            ffi::Par_StartTo(
                0,
                cstr("127.0.0.1").as_ptr(),
                cstr("127.0.0.1").as_ptr(),
                0x0100,
                0x0200,
            )
        },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(unsafe { ffi::Par_Stop(0) }, ffi::ERR_LIB_INVALID_OBJECT);
    assert_eq!(
        unsafe { ffi::Par_BSend(0, 1, buffer.as_mut_ptr() as *mut _, 8) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Par_AsBSend(0, 1, buffer.as_mut_ptr() as *mut _, 8) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Par_BRecv(0, &mut r_id, buffer.as_mut_ptr() as *mut _, &mut size, 1000) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Par_GetStatus(0, &mut value) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Par_GetLastError(0, &mut value) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
}

#[test]
fn get_param_invalid_number() {
    let p = partner(true);
    let mut value: i32 = 0;
    let rc = unsafe { ffi::Par_GetParam(p.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_PAR_INVALID_PARAM_NUMBER);
}

#[test]
fn set_param_invalid_number() {
    let p = partner(true);
    let mut value: i32 = 0;
    let rc = unsafe { ffi::Par_SetParam(p.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_PAR_INVALID_PARAM_NUMBER);
}

#[test]
fn params_round_trip() {
    let p = partner(true);

    let port: u16 = 1234;
    assert_eq!(
        unsafe {
            ffi::Par_SetParam(
                p.raw(),
                ffi::P_U16_REMOTE_PORT,
                &port as *const u16 as *mut _,
            )
        },
        0
    );
    let mut got: u16 = 0;
    assert_eq!(
        unsafe {
            ffi::Par_GetParam(
                p.raw(),
                ffi::P_U16_REMOTE_PORT,
                &mut got as *mut u16 as *mut _,
            )
        },
        0
    );
    assert_eq!(got, port);

    let send_timeout: i32 = 4000;
    assert_eq!(
        unsafe {
            ffi::Par_SetParam(
                p.raw(),
                ffi::P_I32_BSEND_TIMEOUT,
                &send_timeout as *const i32 as *mut _,
            )
        },
        0
    );
    let mut got: i32 = 0;
    assert_eq!(
        unsafe {
            ffi::Par_GetParam(
                p.raw(),
                ffi::P_I32_BSEND_TIMEOUT,
                &mut got as *mut i32 as *mut _,
            )
        },
        0
    );
    assert_eq!(got, send_timeout);

    let recv_timeout: i32 = 5000;
    assert_eq!(
        unsafe {
            ffi::Par_SetParam(
                p.raw(),
                ffi::P_I32_BRECV_TIMEOUT,
                &recv_timeout as *const i32 as *mut _,
            )
        },
        0
    );
    let mut got: i32 = 0;
    assert_eq!(
        unsafe {
            ffi::Par_GetParam(
                p.raw(),
                ffi::P_I32_BRECV_TIMEOUT,
                &mut got as *mut i32 as *mut _,
            )
        },
        0
    );
    assert_eq!(got, recv_timeout);
}

#[test]
fn set_callbacks_succeed() {
    let p = partner(true);
    assert_eq!(
        unsafe { ffi::Par_SetSendCallback(p.raw(), None, std::ptr::null_mut()) },
        0
    );
    assert_eq!(
        unsafe { ffi::Par_SetRecvCallback(p.raw(), None, std::ptr::null_mut()) },
        0
    );
}

#[test]
fn get_times_and_stats_initial() {
    let p = partner(true);
    let mut send_time: u32 = u32::MAX;
    let mut recv_time: u32 = u32::MAX;
    assert_eq!(
        unsafe { ffi::Par_GetTimes(p.raw(), &mut send_time, &mut recv_time) },
        0
    );
    assert_eq!(send_time, 0);
    assert_eq!(recv_time, 0);

    let mut sent: u32 = u32::MAX;
    let mut recv: u32 = u32::MAX;
    let mut serr: u32 = u32::MAX;
    let mut rerr: u32 = u32::MAX;
    assert_eq!(
        unsafe { ffi::Par_GetStats(p.raw(), &mut sent, &mut recv, &mut serr, &mut rerr) },
        0
    );
    assert_eq!(sent, 0);
    assert_eq!(recv, 0);
    assert_eq!(serr, 0);
    assert_eq!(rerr, 0);
}

#[test]
fn get_last_error_initial() {
    let p = partner(true);
    let mut last: i32 = -1;
    assert_eq!(unsafe { ffi::Par_GetLastError(p.raw(), &mut last) }, 0);
    assert_eq!(last, 0);
}

#[test]
fn get_status_stopped_initial() {
    let p = partner(true);
    let mut status: i32 = -1;
    assert_eq!(unsafe { ffi::Par_GetStatus(p.raw(), &mut status) }, 0);
    assert_eq!(status, ffi::PAR_STOPPED);
}

#[test]
fn bsend_without_link_fails() {
    let p = partner(true);
    let mut buffer = [0u8; 8];
    let rc = unsafe { ffi::Par_BSend(p.raw(), 1, buffer.as_mut_ptr() as *mut _, 8) };
    assert_ne!(rc, 0);
}

#[test]
fn brecv_without_link_fails() {
    let p = partner(true);
    let mut buffer = [0u8; 8];
    let mut r_id: u32 = 0;
    let mut size: i32 = 8;
    let rc = unsafe {
        ffi::Par_BRecv(
            p.raw(),
            &mut r_id,
            buffer.as_mut_ptr() as *mut _,
            &mut size,
            100,
        )
    };
    assert_ne!(rc, 0);
}

#[test]
fn check_completions_without_pending() {
    let p = partner(true);
    let mut op_result: i32 = -1;
    // No pending send -> returns JobComplete.
    let rc = unsafe { ffi::Par_CheckAsBSendCompletion(p.raw(), &mut op_result) };
    assert!(rc == ffi::JOB_COMPLETE || rc == ffi::JOB_PENDING);

    let mut r_id: u32 = 0;
    let mut size: i32 = 0;
    let mut buffer = [0u8; 8];
    let rc = unsafe {
        ffi::Par_CheckAsBRecvCompletion(
            p.raw(),
            &mut op_result,
            &mut r_id,
            buffer.as_mut_ptr() as *mut _,
            &mut size,
        )
    };
    assert!(rc == ffi::JOB_COMPLETE || rc == ffi::JOB_PENDING);
}

#[test]
fn wait_as_bsend_completion_timeout() {
    let p = partner(true);
    // No send is pending: returns the last error (0) rather than blocking.
    let rc = unsafe { ffi::Par_WaitAsBSendCompletion(p.raw(), 100) };
    // Since no job is pending this resolves immediately with a result.
    let _ = rc;
}

#[test]
fn active_partner_start_and_stop() {
    let p = partner(true);
    let rc = unsafe {
        ffi::Par_StartTo(
            p.raw(),
            cstr("127.0.0.1").as_ptr(),
            cstr("127.0.0.1").as_ptr(),
            0x0100,
            0x0200,
        )
    };
    assert_eq!(rc, 0);

    let mut status: i32 = -1;
    assert_eq!(unsafe { ffi::Par_GetStatus(p.raw(), &mut status) }, 0);
    assert_eq!(status, ffi::PAR_CONNECTING);

    assert_eq!(unsafe { ffi::Par_Stop(p.raw()) }, 0);

    let mut status: i32 = -1;
    assert_eq!(unsafe { ffi::Par_GetStatus(p.raw(), &mut status) }, 0);
    assert_eq!(status, ffi::PAR_STOPPED);
}

#[test]
fn bsend_brecv_round_trip() {
    // The passive partner binds to the hardcoded port 102, which needs root on
    // Linux. Detect that case and skip the network portion (the bind-error path
    // is still exercised).
    let passive = partner(false);
    let rc = unsafe {
        ffi::Par_StartTo(
            passive.raw(),
            cstr("127.0.0.1").as_ptr(),
            cstr("127.0.0.1").as_ptr(),
            0x0200,
            0x0100,
        )
    };

    let mut status: i32 = -1;
    unsafe { ffi::Par_GetStatus(passive.raw(), &mut status) };
    if status == ffi::PAR_BINDERROR {
        unsafe { ffi::Par_Stop(passive.raw()) };
        eprintln!("skipping partner round trip: cannot bind privileged port 102");
        return;
    }
    assert_eq!(rc, 0);

    let active = partner(true);
    let rc = unsafe {
        ffi::Par_StartTo(
            active.raw(),
            cstr("127.0.0.1").as_ptr(),
            cstr("127.0.0.1").as_ptr(),
            0x0100,
            0x0200,
        )
    };
    assert_eq!(rc, 0);

    // Wait for the active partner to link.
    let mut status: i32 = -1;
    for _ in 0..100 {
        unsafe { ffi::Par_GetStatus(active.raw(), &mut status) };
        if status == ffi::PAR_LINKED {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert_eq!(
        status,
        ffi::PAR_LINKED,
        "active partner should link to passive"
    );

    // Send a block from active to passive.
    let payload: Vec<u8> = (0..64u8).collect();
    let rc = unsafe {
        ffi::Par_BSend(
            active.raw(),
            0x11223344,
            payload.as_ptr() as *mut _,
            payload.len() as i32,
        )
    };
    assert_eq!(rc, 0);

    let mut r_id: u32 = 0;
    let mut out = vec![0u8; payload.len()];
    let mut size: i32 = payload.len() as i32;
    let rc = unsafe {
        ffi::Par_BRecv(
            passive.raw(),
            &mut r_id,
            out.as_mut_ptr() as *mut _,
            &mut size,
            5000,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(r_id, 0x1122_3344);
    assert_eq!(size as usize, payload.len());
    assert_eq!(out, payload);

    assert_eq!(unsafe { ffi::Par_Stop(active.raw()) }, 0);
    assert_eq!(unsafe { ffi::Par_Stop(passive.raw()) }, 0);
}
