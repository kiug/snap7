//! Safe `Server` tests that don't require a client.

mod common;

use common::{lock_net, TEST_PORT};
use snap7::{CpuStatus, Server, ServerArea};

#[test]
fn create_and_destroy() {
    let _server = Server::create();
}

#[test]
fn typed_params_round_trip() {
    let server = Server::create();
    server.set_local_port(1234).unwrap();
    assert_eq!(
        server.get_param_u16(snap7::ffi::P_U16_LOCAL_PORT).unwrap(),
        1234
    );

    server.set_max_clients(32).unwrap();
    assert_eq!(
        server.get_param_i32(snap7::ffi::P_I32_MAX_CLIENTS).unwrap(),
        32
    );
}

#[test]
fn register_unregister_area() {
    let server = Server::create();
    let mut db = vec![0u8; 64];

    server
        .register_area(ServerArea::DataBlock, 7, &mut db)
        .unwrap();
    // Duplicate registration of the same DB is refused.
    assert!(server
        .register_area(ServerArea::DataBlock, 7, &mut db)
        .is_err());
    server.unregister_area(ServerArea::DataBlock, 7).unwrap();
    // Re-register after unregistering works.
    server
        .register_area(ServerArea::DataBlock, 7, &mut db)
        .unwrap();
}

#[test]
fn lock_unlock_area() {
    let server = Server::create();
    let mut mk = vec![0u8; 16];
    server
        .register_area(ServerArea::Merkers, 0, &mut mk)
        .unwrap();
    server.lock_area(ServerArea::Merkers, 0).unwrap();
    server.unlock_area(ServerArea::Merkers, 0).unwrap();
}

#[test]
fn status_and_cpu_status() {
    let server = Server::create();
    server.set_cpu_status(CpuStatus::Stop).unwrap();
    let (_, cpu, clients) = server.get_status().unwrap();
    assert_eq!(cpu, CpuStatus::Stop);
    assert_eq!(clients, 0);

    server.set_cpu_status(CpuStatus::Run).unwrap();
    let (_, cpu, _) = server.get_status().unwrap();
    assert_eq!(cpu, CpuStatus::Run);
}

#[test]
fn pick_event_empty() {
    let server = Server::create();
    assert!(server.pick_event().unwrap().is_none());
    server.clear_events().unwrap();
}

#[test]
fn start_and_stop() {
    let _lock = lock_net();
    let server = Server::create();
    server.set_local_port(TEST_PORT).unwrap();
    server.start_to("127.0.0.1").unwrap();
    server.stop().unwrap();
}

#[test]
fn error_text_returns_description() {
    assert!(!Server::error_text(0x0020_0000).is_empty());
}
