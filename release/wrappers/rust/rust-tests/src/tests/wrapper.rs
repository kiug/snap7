//! Tests for the safe wrapper API, exercising the thin layer over the FFI.

use crate::tests::lock_net;
use crate::{Area, Client, Server, ServerArea, WordLen};

const TEST_PORT: u16 = 4103;

#[test]
fn client_lifecycle_and_params() {
    let client = Client::create();

    client.set_remote_port(1234).unwrap();
    assert_eq!(client.remote_port().unwrap(), 1234);

    client.set_pdu_request(960).unwrap();
    assert_eq!(client.pdu_request().unwrap(), 960);

    // Freshly created clients are not connected.
    assert!(!client.is_connected().unwrap());
}

#[test]
fn client_server_round_trip() {
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
    assert!(client.is_connected().unwrap());

    // Byte-level DB read/write.
    let pattern: Vec<u8> = (0..32u8).collect();
    client.db_write(1, 0, &pattern).unwrap();
    let mut out = vec![0u8; 32];
    client.db_read(1, 0, &mut out).unwrap();
    assert_eq!(out, pattern);

    // Word-level read/write through the area API.
    client
        .write_area(Area::DataBlock, 1, 0, 16, WordLen::Word, &pattern)
        .unwrap();
    let mut words = vec![0u8; 32];
    client
        .read_area(Area::DataBlock, 1, 0, 16, WordLen::Word, &mut words)
        .unwrap();
    assert_eq!(words, pattern);

    client.disconnect().unwrap();
    server.stop().unwrap();
}
