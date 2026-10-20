//! Safe `Client` tests that don't require a running server.

use snap7::{Client, ConnectionType};

#[test]
fn create_and_destroy() {
    let _client = Client::create();
}

#[test]
fn typed_params_round_trip() {
    let client = Client::create();
    client.set_remote_port(1234).unwrap();
    assert_eq!(client.remote_port().unwrap(), 1234);

    client.set_pdu_request(960).unwrap();
    assert_eq!(client.pdu_request().unwrap(), 960);

    client.set_ping_timeout(500).unwrap();
    client.set_send_timeout(600).unwrap();
    client.set_recv_timeout(700).unwrap();
}

#[test]
fn generic_params_round_trip() {
    let client = Client::create();
    client
        .set_param_i32(snap7::ffi::P_I32_WORK_INTERVAL, 42)
        .unwrap();
    assert_eq!(
        client
            .get_param_i32(snap7::ffi::P_I32_WORK_INTERVAL)
            .unwrap(),
        42
    );

    client
        .set_param_u16(snap7::ffi::P_U16_SRC_REF, 0x1234)
        .unwrap();
    assert_eq!(
        client.get_param_u16(snap7::ffi::P_U16_SRC_REF).unwrap(),
        0x1234
    );

    client
        .set_param_u16(snap7::ffi::P_U16_DST_REF, 0x5678)
        .unwrap();
    client
        .set_param_u16(snap7::ffi::P_U16_SRC_TSAP, 0x0100)
        .unwrap();
}

#[test]
fn not_connected_initially() {
    let client = Client::create();
    assert!(!client.is_connected().unwrap());
}

#[test]
fn connection_params_and_type() {
    let client = Client::create();
    client
        .set_connection_params("192.168.0.1", 0x0100, 0x0200)
        .unwrap();
    client.set_connection_type(ConnectionType::Pg).unwrap();
    client.set_connection_type(ConnectionType::Op).unwrap();
    client.set_connection_type(ConnectionType::Basic).unwrap();
}

#[test]
fn get_pg_block_info_decodes_header() {
    let client = Client::create();
    let mut block = vec![0u8; 128];
    block[5] = 0x0A; // SubBlkType = DB
    block[8] = 0x00;
    block[9] = 0x00;
    block[10] = 0x00;
    block[11] = 0x80; // LenLoadMem = 128
    block[34] = 0x00;
    block[35] = 0x10; // MC7Len = 16

    let info = client.get_pg_block_info(&block).unwrap();
    assert_eq!(info.block_type, 0x0A);
    assert_eq!(info.mc7_size, 16);
    assert_eq!(info.load_size, 128);
}

#[test]
fn error_text_returns_description() {
    assert!(!Client::error_text(0x0020_0000).is_empty());
}
