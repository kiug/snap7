//! Safe `Client` tests that don't require a running server.

use snap7::{Area, BlockType, Client, ConnectionType, WordLen};

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

#[test]
fn get_param_invalid_number_errors() {
    let client = Client::create();
    assert!(client.get_param_i32(9999).is_err());
    assert!(client.get_param_u16(9999).is_err());
}

#[test]
fn operations_without_connection_fail() {
    let client = Client::create();

    assert!(client.get_ag_block_info(BlockType::Db, 1).is_err());
    assert!(client.list_blocks_of_type(BlockType::Db).is_err());
    assert!(client.upload(BlockType::Db, 1).is_err());
    assert!(client.full_upload(BlockType::Db, 1).is_err());
    assert!(client.db_get(1).is_err());
    assert!(client.read_szl(0x0011, 0x0000).is_err());
    assert!(client.read_szl_list().is_err());
}

#[test]
fn get_pg_block_info_invalid_block_fails() {
    let client = Client::create();
    // A zeroed block has no valid header, which Snap7 reports as an invalid
    // block size (a too-short buffer would instead over-read and crash).
    let block = vec![0u8; 128];
    assert!(client.get_pg_block_info(&block).is_err());
}

#[test]
#[should_panic(expected = "buffer too small")]
fn read_area_buffer_too_small_panics() {
    let client = Client::create();
    let mut buf = [0u8; 2];
    // amount * word_len.size() == 4, but the buffer holds only 2 bytes.
    let _ = client.read_area(Area::DataBlock, 1, 0, 4, WordLen::Byte, &mut buf);
}

#[test]
#[should_panic(expected = "buffer too small")]
fn write_area_buffer_too_small_panics() {
    let client = Client::create();
    let buf = [0u8; 2];
    let _ = client.write_area(Area::DataBlock, 1, 0, 4, WordLen::Byte, &buf);
}
