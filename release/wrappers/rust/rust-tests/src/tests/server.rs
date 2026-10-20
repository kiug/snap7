//! Server lifecycle, parameter and error-path tests that do not require a
//! client.

use crate::ffi;
use crate::tests::{cstr, lock_net, server, TEST_PORT};

#[test]
fn create_and_destroy_server() {
    let s = server();
    assert_ne!(s.raw(), 0);
}

#[test]
fn destroy_null_server_is_a_noop() {
    let mut raw = 0usize;
    unsafe { ffi::Srv_Destroy(&mut raw) };
    assert_eq!(raw, 0);
}

#[test]
fn functions_on_null_server_return_invalid_object() {
    let mut value: i32 = 0;
    let mut buffer = [0u8; 4];

    assert_eq!(
        unsafe { ffi::Srv_GetParam(0, ffi::P_U16_LOCAL_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_SetParam(0, ffi::P_U16_LOCAL_PORT, &mut value as *mut _ as *mut _) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(unsafe { ffi::Srv_Start(0) }, ffi::ERR_LIB_INVALID_OBJECT);
    assert_eq!(
        unsafe { ffi::Srv_StartTo(0, cstr("127.0.0.1").as_ptr()) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(unsafe { ffi::Srv_Stop(0) }, ffi::ERR_LIB_INVALID_OBJECT);
    assert_eq!(
        unsafe { ffi::Srv_RegisterArea(0, ffi::SRV_AREA_DB, 1, buffer.as_mut_ptr() as *mut _, 4) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_UnregisterArea(0, ffi::SRV_AREA_DB, 1) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_LockArea(0, ffi::SRV_AREA_DB, 1) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_UnlockArea(0, ffi::SRV_AREA_DB, 1) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_ClearEvents(0) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
    assert_eq!(
        unsafe { ffi::Srv_SetCpuStatus(0, ffi::CPU_STATUS_RUN) },
        ffi::ERR_LIB_INVALID_OBJECT
    );
}

#[test]
fn get_param_invalid_number() {
    let s = server();
    let mut value: i32 = 0;
    let rc = unsafe { ffi::Srv_GetParam(s.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAM_NUMBER);
}

#[test]
fn set_param_invalid_number() {
    let s = server();
    let mut value: i32 = 0;
    let rc = unsafe { ffi::Srv_SetParam(s.raw(), 9999, &mut value as *mut _ as *mut _) };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAM_NUMBER);
}

#[test]
fn local_port_param_round_trip() {
    let s = server();
    let port: u16 = TEST_PORT;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_U16_LOCAL_PORT,
            &port as *const u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);

    let mut got: u16 = 0;
    let rc = unsafe {
        ffi::Srv_GetParam(
            s.raw(),
            ffi::P_U16_LOCAL_PORT,
            &mut got as *mut u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(got, port);
}

#[test]
fn work_interval_and_max_clients_round_trip() {
    let s = server();

    let interval: i32 = 200;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_WORK_INTERVAL,
            &interval as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    let mut got: i32 = 0;
    let rc = unsafe {
        ffi::Srv_GetParam(
            s.raw(),
            ffi::P_I32_WORK_INTERVAL,
            &mut got as *mut i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(got, interval);

    let max_clients: i32 = 32;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_MAX_CLIENTS,
            &max_clients as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    let mut got: i32 = 0;
    let rc = unsafe {
        ffi::Srv_GetParam(
            s.raw(),
            ffi::P_I32_MAX_CLIENTS,
            &mut got as *mut i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(got, max_clients);
}

#[test]
fn pdu_request_param_invalid_value() {
    let s = server();
    // Too small (< MinPduSize)
    let small: i32 = 10;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_PDU_REQUEST,
            &small as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAMS);

    // Too large (> IsoPayload_Size)
    let large: i32 = 0x7FFF;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_PDU_REQUEST,
            &large as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAMS);

    // Valid value.
    let valid: i32 = 480;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_PDU_REQUEST,
            &valid as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);

    // Zero disables the forced PDU.
    let zero: i32 = 0;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_I32_PDU_REQUEST,
            &zero as *const i32 as *mut _,
        )
    };
    assert_eq!(rc, 0);
}

#[test]
fn register_area_with_null_pointer() {
    let s = server();
    let rc =
        unsafe { ffi::Srv_RegisterArea(s.raw(), ffi::SRV_AREA_DB, 1, std::ptr::null_mut(), 4) };
    assert_eq!(rc, ffi::ERR_SRV_DB_NULL_POINTER);
}

#[test]
fn register_area_unknown_area_code() {
    let s = server();
    let mut buffer = [0u8; 4];
    // 99 is neither srvAreaDB nor a valid system area.
    let rc = unsafe { ffi::Srv_RegisterArea(s.raw(), 99, 0, buffer.as_mut_ptr() as *mut _, 4) };
    assert_eq!(rc, ffi::ERR_SRV_UNKNOWN_AREA);
}

#[test]
fn register_area_duplicate() {
    let s = server();
    let mut buffer = [0u8; 8];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_DB,
            1,
            buffer.as_mut_ptr() as *mut _,
            8,
        )
    };
    assert_eq!(rc, 0);

    let mut other = [0u8; 8];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_DB,
            1,
            other.as_mut_ptr() as *mut _,
            8,
        )
    };
    assert_eq!(rc, ffi::ERR_SRV_AREA_ALREADY_EXISTS);
}

#[test]
fn register_system_area_duplicate() {
    let s = server();
    let mut buffer = [0u8; 8];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_MK,
            0,
            buffer.as_mut_ptr() as *mut _,
            8,
        )
    };
    assert_eq!(rc, 0);

    let mut other = [0u8; 8];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_MK,
            0,
            other.as_mut_ptr() as *mut _,
            8,
        )
    };
    assert_eq!(rc, ffi::ERR_SRV_AREA_ALREADY_EXISTS);
}

#[test]
fn unregister_area_invalid_code() {
    let s = server();
    let rc = unsafe { ffi::Srv_UnregisterArea(s.raw(), 99, 0) };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAMS);
}

#[test]
fn unregister_unregistered_db() {
    let s = server();
    let rc = unsafe { ffi::Srv_UnregisterArea(s.raw(), ffi::SRV_AREA_DB, 5) };
    assert_eq!(rc, ffi::ERR_SRV_INVALID_PARAMS);
}

#[test]
fn register_and_unregister_db_round_trip() {
    let s = server();
    let mut buffer = [0u8; 16];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_DB,
            7,
            buffer.as_mut_ptr() as *mut _,
            16,
        )
    };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_UnregisterArea(s.raw(), ffi::SRV_AREA_DB, 7) };
    assert_eq!(rc, 0);
}

#[test]
fn lock_and_unlock_area() {
    let s = server();
    let mut buffer = [0u8; 16];
    let rc = unsafe {
        ffi::Srv_RegisterArea(
            s.raw(),
            ffi::SRV_AREA_DB,
            3,
            buffer.as_mut_ptr() as *mut _,
            16,
        )
    };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_LockArea(s.raw(), ffi::SRV_AREA_DB, 3) };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_UnlockArea(s.raw(), ffi::SRV_AREA_DB, 3) };
    assert_eq!(rc, 0);
}

#[test]
fn status_and_cpu_status() {
    let s = server();
    let mut server_status: i32 = -1;
    let mut cpu_status: i32 = -1;
    let mut clients_count: i32 = -1;
    let rc = unsafe {
        ffi::Srv_GetStatus(
            s.raw(),
            &mut server_status,
            &mut cpu_status,
            &mut clients_count,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(server_status, ffi::SRV_STOPPED);

    let rc = unsafe { ffi::Srv_SetCpuStatus(s.raw(), ffi::CPU_STATUS_RUN) };
    assert_eq!(rc, 0);

    let mut cpu_status = -1;
    let rc = unsafe {
        ffi::Srv_GetStatus(
            s.raw(),
            &mut server_status,
            &mut cpu_status,
            &mut clients_count,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(cpu_status, ffi::CPU_STATUS_RUN);
}

#[test]
fn pick_event_with_no_events() {
    let s = server();
    let mut event = ffi::TSrvEvent::default();
    let mut ready: i32 = -1;
    let rc = unsafe { ffi::Srv_PickEvent(s.raw(), &mut event, &mut ready) };
    assert_eq!(rc, 0);
    assert_eq!(ready, 0);
}

#[test]
fn clear_events_succeeds() {
    let s = server();
    let rc = unsafe { ffi::Srv_ClearEvents(s.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn event_mask_round_trip() {
    let s = server();

    let mut mask: u32 = 0;
    let rc = unsafe { ffi::Srv_GetMask(s.raw(), ffi::MK_EVENT, &mut mask) };
    assert_eq!(rc, 0);

    let new_mask: u32 = ffi::EVC_DATA_READ | ffi::EVC_DATA_WRITE;
    let rc = unsafe { ffi::Srv_SetMask(s.raw(), ffi::MK_EVENT, new_mask) };
    assert_eq!(rc, 0);

    let mut got: u32 = 0;
    let rc = unsafe { ffi::Srv_GetMask(s.raw(), ffi::MK_EVENT, &mut got) };
    assert_eq!(rc, 0);
    assert_eq!(got, new_mask);
}

#[test]
fn start_stop_and_param_change_while_running() {
    let _guard = lock_net();

    let s = server();
    let port: u16 = TEST_PORT;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_U16_LOCAL_PORT,
            &port as *const u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_StartTo(s.raw(), cstr("127.0.0.1").as_ptr()) };
    assert_eq!(rc, 0);

    // Changing the port while running is refused.
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_U16_LOCAL_PORT,
            &port as *const u16 as *mut _,
        )
    };
    assert_eq!(rc, ffi::ERR_SRV_CANNOT_CHANGE_PARAM);

    let mut server_status: i32 = -1;
    let mut cpu_status: i32 = -1;
    let mut clients_count: i32 = -1;
    let rc = unsafe {
        ffi::Srv_GetStatus(
            s.raw(),
            &mut server_status,
            &mut cpu_status,
            &mut clients_count,
        )
    };
    assert_eq!(rc, 0);
    assert_eq!(server_status, ffi::SRV_RUNNING);

    let rc = unsafe { ffi::Srv_Stop(s.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn start_without_address() {
    let _guard = lock_net();

    let s = server();
    let port: u16 = TEST_PORT;
    let rc = unsafe {
        ffi::Srv_SetParam(
            s.raw(),
            ffi::P_U16_LOCAL_PORT,
            &port as *const u16 as *mut _,
        )
    };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_Start(s.raw()) };
    assert_eq!(rc, 0);

    let rc = unsafe { ffi::Srv_Stop(s.raw()) };
    assert_eq!(rc, 0);
}

#[test]
fn set_event_callbacks_succeed() {
    let s = server();
    assert_eq!(
        unsafe { ffi::Srv_SetEventsCallback(s.raw(), None, std::ptr::null_mut()) },
        0
    );
    assert_eq!(
        unsafe { ffi::Srv_SetReadEventsCallback(s.raw(), None, std::ptr::null_mut()) },
        0
    );
    assert_eq!(
        unsafe { ffi::Srv_SetRWAreaCallback(s.raw(), None, std::ptr::null_mut()) },
        0
    );
}
