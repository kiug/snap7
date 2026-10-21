//! Safe `Partner` tests.

mod common;

use common::lock_net;
use snap7::{Partner, PartnerStatus};

#[test]
fn create_active_and_passive() {
    let _active = Partner::create(true);
    let _passive = Partner::create(false);
}

#[test]
fn params_round_trip() {
    let partner = Partner::create(true);
    partner.set_remote_port(1234).unwrap();
    partner
        .set_param_i32(snap7::ffi::P_I32_BSEND_TIMEOUT, 500)
        .unwrap();
    assert_eq!(
        partner
            .get_param_i32(snap7::ffi::P_I32_BSEND_TIMEOUT)
            .unwrap(),
        500
    );
}

#[test]
fn status_and_error_initial() {
    let partner = Partner::create(true);
    assert_eq!(partner.status().unwrap(), PartnerStatus::Stopped);
    assert_eq!(partner.last_error().unwrap(), 0);
}

#[test]
fn times_and_stats_initial() {
    let partner = Partner::create(true);
    let (send_time, recv_time) = partner.get_times().unwrap();
    assert_eq!(send_time, 0);
    assert_eq!(recv_time, 0);

    let (sent, recv, send_errors, recv_errors) = partner.get_stats().unwrap();
    assert_eq!(sent, 0);
    assert_eq!(recv, 0);
    assert_eq!(send_errors, 0);
    assert_eq!(recv_errors, 0);
}

#[test]
fn bsend_brecv_without_link_fails() {
    let partner = Partner::create(true);
    assert!(partner.b_send(1, &[0u8; 8]).is_err());

    let mut buf = [0u8; 8];
    assert!(partner.b_recv(&mut buf, 100).is_err());
}

#[test]
fn check_as_bsend_without_pending() {
    let partner = Partner::create(true);
    // No pending send: the completion check reports complete or pending.
    let _ = partner.check_as_b_send_completion().unwrap();
}

#[test]
fn error_text_returns_description() {
    assert!(!Partner::error_text(0x0020_0000).is_empty());
}

#[test]
fn bsend_brecv_round_trip() {
    let _lock = lock_net();

    let passive = Partner::create(false);
    // The passive partner binds the privileged port 102, so start_to fails
    // when the capability is missing; the failure is reported via status().
    let _ = passive.start_to("127.0.0.1", "127.0.0.1", 0x0200, 0x0100);

    if passive.status().unwrap() == PartnerStatus::BindError {
        eprintln!("skipping network round trip: cannot bind port 102");
        passive.stop().unwrap();
        return;
    }

    let active = Partner::create(true);
    active
        .start_to("127.0.0.1", "127.0.0.1", 0x0100, 0x0200)
        .unwrap();

    // Wait for the active partner to link.
    for _ in 0..200 {
        if active.status().unwrap() == PartnerStatus::Linked {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
    assert_eq!(active.status().unwrap(), PartnerStatus::Linked);

    let payload: Vec<u8> = (0..64u8).collect();

    // Synchronous send/receive.
    active.b_send(0x1122_3344, &payload).unwrap();
    let mut out = vec![0u8; payload.len()];
    let (r_id, size) = passive.b_recv(&mut out, 5000).unwrap();
    assert_eq!(r_id, 0x1122_3344);
    assert_eq!(size, payload.len());
    assert_eq!(out, payload);

    // Asynchronous send, completed via the completion API.
    active.as_b_send(0x5566_7788, &payload).unwrap();
    active.wait_as_b_send_completion(5000).unwrap();
    let mut out2 = vec![0u8; payload.len()];
    let (r_id2, _) = passive.b_recv(&mut out2, 5000).unwrap();
    assert_eq!(r_id2, 0x5566_7788);
    assert_eq!(out2, payload);

    // Transfer statistics reflect the sent/received bytes. The active partner
    // only sends in this test (b_send + as_b_send) and never receives, while
    // the passive partner only receives and never sends.
    let (sent, recv, _, _) = active.get_stats().unwrap();
    assert!(sent > 0);
    assert_eq!(recv, 0);

    let (sent_p, recv_p, _, _) = passive.get_stats().unwrap();
    assert_eq!(sent_p, 0);
    assert!(recv_p > 0);

    let (send_time, recv_time) = active.get_times().unwrap();
    let _ = (send_time, recv_time);

    active.stop().unwrap();
    passive.stop().unwrap();
}
