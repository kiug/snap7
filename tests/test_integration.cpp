// Integration tests: a Snap7 client talking to a Snap7 server over loopback.

#include <vector>

#include "snap7_libmain.h"
#include "test.h"

namespace {
// State captured by the async-completion callback.
int g_callback_calls = 0;
int g_callback_op_code = -1;
int g_callback_op_result = -1;
// State captured by the RW-area callback.
int g_rw_callback_calls = 0;
}

extern "C" void test_completion_callback(void* usrPtr, int opCode, int opResult) {
    (void)usrPtr;
    g_callback_calls++;
    g_callback_op_code = opCode;
    g_callback_op_result = opResult;
}

extern "C" int test_rw_callback(void* usrPtr, int Sender, int Operation, PS7Tag PTag,
                               void* pUsrData) {
    (void)usrPtr;
    (void)Sender;
    (void)PTag;
    (void)pUsrData;
    (void)Operation;
    g_rw_callback_calls++;
    return 0;  // 0 == success (the callback fulfilled the read/write)
}

namespace {
const word TEST_PORT = 4102;
const word DB1 = 1;
const word DB2 = 2;
const int DB_SIZE = 512;
const int SYS_SIZE = 256;

// Field order matters for destruction order: client is destroyed first, then
// the server, and finally the area buffers the server points at.
struct Harness {
    std::vector<char> db1, db2, mk, pe, pa, tm, ct;
    S7Object server;
    S7Object client;

    Harness() : server(0), client(0) {
        db1.assign(DB_SIZE, 0);
        db2.assign(DB_SIZE, 0);
        mk.assign(SYS_SIZE, 0);
        pe.assign(SYS_SIZE, 0);
        pa.assign(SYS_SIZE, 0);
        tm.assign(SYS_SIZE, 0);
        ct.assign(SYS_SIZE, 0);

        server = Srv_Create();
        word port = TEST_PORT;
        CHECK_EQ(Srv_SetParam(server, p_u16_LocalPort, &port), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaDB, DB1, db1.data(), DB_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaDB, DB2, db2.data(), DB_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaMK, 0, mk.data(), SYS_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaPE, 0, pe.data(), SYS_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaPA, 0, pa.data(), SYS_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaTM, 0, tm.data(), SYS_SIZE), 0);
        CHECK_EQ(Srv_RegisterArea(server, srvAreaCT, 0, ct.data(), SYS_SIZE), 0);
        CHECK_EQ(Srv_StartTo(server, "127.0.0.1"), 0);

        client = Cli_Create();
        CHECK_EQ(Cli_SetParam(client, p_u16_RemotePort, &port), 0);
        CHECK_EQ(Cli_ConnectTo(client, "127.0.0.1", 0, 2), 0);
    }

    ~Harness() {
        if (client) Cli_Destroy(client);
        if (server) Srv_Destroy(server);
    }
};
}  // namespace

TEST(integration_db_read_write_round_trip) {
    Harness h;
    std::vector<char> pattern(128);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.db1.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_DBRead(h.client, DB1, 0, (int)out.size(), out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_db_read_write_with_offset) {
    Harness h;
    std::vector<char> pattern(32);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_DBWrite(h.client, DB2, 10, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.db2.data() + 10, pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_DBRead(h.client, DB2, 10, (int)out.size(), out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_mb_read_write_round_trip) {
    Harness h;
    std::vector<char> pattern(64);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)(i * 3);

    CHECK_EQ(Cli_MBWrite(h.client, 0, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.mk.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_MBRead(h.client, 0, (int)out.size(), out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_eb_read_write_round_trip) {
    Harness h;
    std::vector<char> pattern(64);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)(i + 7);

    CHECK_EQ(Cli_EBWrite(h.client, 0, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.pe.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_EBRead(h.client, 0, (int)out.size(), out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_ab_read_write_round_trip) {
    Harness h;
    std::vector<char> pattern(64);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)(i ^ 0xAA);

    CHECK_EQ(Cli_ABWrite(h.client, 0, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.pa.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_ABRead(h.client, 0, (int)out.size(), out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_tm_read_write_round_trip) {
    Harness h;
    // Timers are 2 bytes each; amount is expressed in timer units.
    const int amount = 8;
    std::vector<char> pattern(amount * 2);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_TMWrite(h.client, 0, amount, pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.tm.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_TMRead(h.client, 0, amount, out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_ct_read_write_round_trip) {
    Harness h;
    // Counters are 2 bytes each; amount is expressed in counter units.
    const int amount = 8;
    std::vector<char> pattern(amount * 2);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_CTWrite(h.client, 0, amount, pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.ct.data(), pattern.data(), pattern.size()), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_CTRead(h.client, 0, amount, out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_read_area_with_word_lengths) {
    Harness h;
    char bytes[8] = {0x01, 0x00, 0x03, 0x02, 0x05, 0x04, 0x07, 0x06};
    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, 8, bytes), 0);

    word words[4] = {0};
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 4, S7WLWord, words), 0);
    // Snap7 returns raw big-endian PLC bytes; on a little-endian host they
    // appear as little-endian words.
    CHECK_EQ(words[0], 0x0001);
    CHECK_EQ(words[1], 0x0203);
    CHECK_EQ(words[2], 0x0405);
    CHECK_EQ(words[3], 0x0607);
}

TEST(integration_write_area_with_word_lengths) {
    Harness h;
    word words[4] = {0x1234, 0x5678, 0x9ABC, 0xDEF0};
    CHECK_EQ(Cli_WriteArea(h.client, S7AreaDB, DB1, 4, 4, S7WLWord, words), 0);

    const unsigned char expected[8] = {0x34, 0x12, 0x78, 0x56, 0xBC, 0x9A, 0xF0, 0xDE};
    CHECK_EQ(std::memcmp(h.db1.data() + 4, expected, 8), 0);
}

TEST(integration_multi_vars_read_and_write) {
    Harness h;
    word write_words[3] = {0x0102, 0x0304, 0x0506};

    TS7DataItem write_item;
    std::memset(&write_item, 0, sizeof(write_item));
    write_item.Area = S7AreaDB;
    write_item.WordLen = S7WLWord;
    write_item.DBNumber = DB1;
    write_item.Start = 0;
    write_item.Amount = 3;
    write_item.pdata = write_words;

    CHECK_EQ(Cli_WriteMultiVars(h.client, &write_item, 1), 0);
    CHECK_EQ(write_item.Result, 0);
    const char expected[6] = {0x02, 0x01, 0x04, 0x03, 0x06, 0x05};
    CHECK_EQ(std::memcmp(h.db1.data(), expected, 6), 0);

    word read_words[3] = {0};
    TS7DataItem read_item;
    std::memset(&read_item, 0, sizeof(read_item));
    read_item.Area = S7AreaDB;
    read_item.WordLen = S7WLWord;
    read_item.DBNumber = DB1;
    read_item.Start = 0;
    read_item.Amount = 3;
    read_item.pdata = read_words;

    CHECK_EQ(Cli_ReadMultiVars(h.client, &read_item, 1), 0);
    CHECK_EQ(read_item.Result, 0);
    CHECK_EQ(read_words[0], write_words[0]);
    CHECK_EQ(read_words[1], write_words[1]);
    CHECK_EQ(read_words[2], write_words[2]);
}

TEST(integration_list_blocks) {
    Harness h;
    TS7BlocksList blocks;
    std::memset(&blocks, 0, sizeof(blocks));
    CHECK_EQ(Cli_ListBlocks(h.client, &blocks), 0);
    CHECK_EQ(blocks.OBCount, 0);
    CHECK_EQ(blocks.FBCount, 0);
    CHECK_EQ(blocks.FCCount, 0);
    CHECK_EQ(blocks.SFBCount, 0);
    CHECK_EQ(blocks.SFCCount, 0);
    CHECK_EQ(blocks.DBCount, 2);
    CHECK_EQ(blocks.SDBCount, 0);
}

TEST(integration_list_blocks_of_type) {
    Harness h;
    TS7BlocksOfType items;
    std::memset(items, 0, sizeof(items));
    int count = 0x2000;
    CHECK_EQ(Cli_ListBlocksOfType(h.client, Block_DB, &items, count), 0);
    CHECK(count > 0);
}

TEST(integration_get_ag_block_info) {
    Harness h;
    TS7BlockInfo info;
    std::memset(&info, 0, sizeof(info));
    CHECK_EQ(Cli_GetAgBlockInfo(h.client, Block_DB, DB1, &info), 0);
    CHECK_EQ(info.BlkType, 0x0A);  // SubBlk_DB
    CHECK_EQ(info.BlkNumber, (int)DB1);
    CHECK_EQ(info.MC7Size, DB_SIZE);
}

TEST(integration_get_pg_block_info_invalid_block) {
    Harness h;
    std::vector<char> block(128, 0);
    TS7BlockInfo info;
    std::memset(&info, 0, sizeof(info));
    CHECK_EQ(Cli_GetPgBlockInfo(h.client, block.data(), &info, (int)block.size()),
             errCliInvalidBlockSize);
}

TEST(integration_read_szl_known_id) {
    Harness h;
    TS7SZL buf;
    int size = (int)sizeof(buf);
    CHECK_EQ(Cli_ReadSZL(h.client, 0x0011, 0x0000, &buf, size), 0);
    CHECK(size > 0);
}

TEST(integration_read_szl_unknown_id) {
    Harness h;
    TS7SZL buf;
    int size = (int)sizeof(buf);
    CHECK_NE(Cli_ReadSZL(h.client, 0xFFFF, 0x0000, &buf, size), 0);
}

TEST(integration_read_szl_list) {
    Harness h;
    TS7SZLList buf;
    int count = (int)(sizeof(buf) / 2);
    CHECK_EQ(Cli_ReadSZLList(h.client, &buf, count), 0);
    CHECK(count > 0);
}

TEST(integration_get_order_code) {
    Harness h;
    TS7OrderCode order;
    std::memset(&order, 0, sizeof(order));
    CHECK_EQ(Cli_GetOrderCode(h.client, &order), 0);
    CHECK(order.Code[0] != '\0');
}

TEST(integration_get_cpu_info) {
    Harness h;
    TS7CpuInfo info;
    std::memset(&info, 0, sizeof(info));
    CHECK_EQ(Cli_GetCpuInfo(h.client, &info), 0);
    CHECK(info.ModuleTypeName[0] != '\0');
}

TEST(integration_get_cp_info) {
    Harness h;
    TS7CpInfo info;
    std::memset(&info, 0, sizeof(info));
    CHECK_EQ(Cli_GetCpInfo(h.client, &info), 0);
    CHECK(info.MaxPduLengt > 0);
    CHECK(info.MaxConnections > 0);
}

TEST(integration_plc_status_and_control) {
    Harness h;
    int status = -1;
    CHECK_EQ(Cli_GetPlcStatus(h.client, status), 0);
    CHECK_EQ(status, (int)S7CpuStatusRun);

    CHECK_EQ(Cli_PlcStop(h.client), 0);
    CHECK_EQ(Cli_GetPlcStatus(h.client, status), 0);
    CHECK_EQ(status, (int)S7CpuStatusStop);

    CHECK_EQ(Cli_PlcHotStart(h.client), 0);
    CHECK_EQ(Cli_GetPlcStatus(h.client, status), 0);
    CHECK_EQ(status, (int)S7CpuStatusRun);

    CHECK_EQ(Cli_PlcColdStart(h.client), 0);
    CHECK_EQ(Cli_GetPlcStatus(h.client, status), 0);
    CHECK_EQ(status, (int)S7CpuStatusRun);
}

TEST(integration_plc_date_time) {
    Harness h;
    // The server acknowledges SetClock but reports the current local time.
    tm set;
    std::memset(&set, 0, sizeof(set));
    set.tm_sec = 30;
    set.tm_min = 25;
    set.tm_hour = 14;
    set.tm_mday = 19;
    set.tm_mon = 9;
    set.tm_year = 126;
    CHECK_EQ(Cli_SetPlcDateTime(h.client, &set), 0);

    tm got;
    std::memset(&got, 0, sizeof(got));
    CHECK_EQ(Cli_GetPlcDateTime(h.client, got), 0);
    CHECK(got.tm_sec >= 0 && got.tm_sec <= 59);
    CHECK(got.tm_min >= 0 && got.tm_min <= 59);
    CHECK(got.tm_hour >= 0 && got.tm_hour <= 23);
    CHECK(got.tm_mday >= 1 && got.tm_mday <= 31);
    CHECK(got.tm_mon >= 0 && got.tm_mon <= 11);
    CHECK(got.tm_year >= 100);
}

TEST(integration_set_plc_system_date_time) {
    Harness h;
    CHECK_EQ(Cli_SetPlcSystemDateTime(h.client), 0);
}

TEST(integration_get_protection) {
    Harness h;
    TS7Protection protection;
    std::memset(&protection, 0, sizeof(protection));
    CHECK_EQ(Cli_GetProtection(h.client, &protection), 0);
}

TEST(integration_session_password) {
    Harness h;
    char password[] = "secret";
    CHECK_EQ(Cli_SetSessionPassword(h.client, password), 0);
    CHECK_EQ(Cli_ClearSessionPassword(h.client), 0);
}

TEST(integration_copy_ram_to_rom_and_compress) {
    Harness h;
    CHECK_EQ(Cli_CopyRamToRom(h.client, 1000), 0);
    CHECK_EQ(Cli_Compress(h.client, 1000), 0);
}

TEST(integration_db_get_and_db_fill) {
    Harness h;
    CHECK_EQ(Cli_DBFill(h.client, DB1, 0xAB), 0);
    for (int i = 0; i < DB_SIZE; ++i) CHECK_EQ((unsigned char)h.db1[i], 0xABu);

    std::vector<char> out(DB_SIZE);
    int size = DB_SIZE;
    CHECK_EQ(Cli_DBGet(h.client, DB1, out.data(), size), 0);
    CHECK_EQ(size, DB_SIZE);
    for (int i = 0; i < DB_SIZE; ++i) CHECK_EQ((unsigned char)out[i], 0xABu);

    std::vector<char> small(64);
    int small_size = 64;
    CHECK_EQ(Cli_DBGet(h.client, DB1, small.data(), small_size), errCliBufferTooSmall);
}

TEST(integration_exec_time_last_error_pdu_length) {
    Harness h;
    int time = -1;
    CHECK_EQ(Cli_GetExecTime(h.client, time), 0);
    CHECK(time >= 0);

    int last = -1;
    CHECK_EQ(Cli_GetLastError(h.client, last), 0);
    CHECK_EQ(last, 0);

    int requested = -1, negotiated = -1;
    CHECK_EQ(Cli_GetPduLength(h.client, requested, negotiated), 0);
    CHECK(requested > 0);
    CHECK(negotiated > 0);
}

TEST(integration_connected_flag_after_connect) {
    Harness h;
    int connected = 0;
    CHECK_EQ(Cli_GetConnected(h.client, connected), 0);
    CHECK_EQ(connected, 1);
}

TEST(integration_iso_exchange_buffer_rejects_malformed_pdu) {
    Harness h;
    char buf[128] = {0};
    buf[0] = 0x32;
    buf[1] = 0x01;
    int size = 2;
    CHECK_NE(Cli_IsoExchangeBuffer(h.client, buf, size), 0);
}

TEST(integration_server_events_are_generated_on_read) {
    Harness h;
    char out[8] = {0};
    CHECK_EQ(Cli_DBRead(h.client, DB1, 0, 8, out), 0);

    TSrvEvent event;
    std::memset(&event, 0, sizeof(event));
    int ready = 0;
    CHECK_EQ(Srv_PickEvent(h.server, &event, ready), 0);
    CHECK_EQ(ready, 1);
    CHECK(event.EvtCode != 0);
}

TEST(integration_async_db_read_write_round_trip) {
    Harness h;
    std::vector<char> pattern(64);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_AsDBWrite(h.client, DB1, 0, (int)pattern.size(), pattern.data()), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    std::vector<char> out(pattern.size());
    CHECK_EQ(Cli_AsDBRead(h.client, DB1, 0, (int)out.size(), out.data()), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_async_check_completion) {
    Harness h;
    std::vector<char> pattern(32);
    for (size_t i = 0; i < pattern.size(); ++i) pattern[i] = (char)i;

    CHECK_EQ(Cli_AsDBWrite(h.client, DB2, 0, (int)pattern.size(), pattern.data()), 0);
    int op_result = 0;
    int rc = Cli_CheckAsCompletion(h.client, op_result);
    CHECK(rc == JobComplete || rc == JobPending);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
}

TEST(integration_additional_word_lengths_round_trip) {
    Harness h;
    char bytes[32];
    for (int i = 0; i < 32; ++i) bytes[i] = (char)(i * 7 + 1);
    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, 32, bytes), 0);

    // Char: 1 byte per element.
    char chars[32];
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 32, S7WLChar, chars), 0);
    CHECK_EQ(std::memcmp(chars, bytes, 32), 0);

    // Int: 2 bytes per element.
    word ints[16];
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 16, S7WLInt, ints), 0);
    CHECK_EQ(std::memcmp(ints, bytes, 32), 0);

    // DWord: 4 bytes per element.
    longword dwords[8];
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 8, S7WLDWord, dwords), 0);
    CHECK_EQ(std::memcmp(dwords, bytes, 32), 0);

    // DInt: 4 bytes per element.
    int dints[8];
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 8, S7WLDInt, dints), 0);
    CHECK_EQ(std::memcmp(dints, bytes, 32), 0);

    // Real: 4 bytes per element.
    float reals[8];
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 8, S7WLReal, reals), 0);
    CHECK_EQ(std::memcmp(reals, bytes, 32), 0);
}

TEST(integration_bit_read_write) {
    Harness h;
    char bit = 1;
    CHECK_EQ(Cli_WriteArea(h.client, S7AreaDB, DB1, 0, 1, S7WLBit, &bit), 0);
    char out = 0;
    CHECK_EQ(Cli_ReadArea(h.client, S7AreaDB, DB1, 0, 1, S7WLBit, &out), 0);
    CHECK_EQ(out, 1);
}

TEST(integration_full_db_transfer) {
    Harness h;
    std::vector<char> pattern(DB_SIZE);
    for (int i = 0; i < DB_SIZE; ++i) pattern[i] = (char)(i * 13);

    // A full-DB transfer spans multiple PDUs (negotiated PDU < DB_SIZE).
    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, DB_SIZE, pattern.data()), 0);
    CHECK_EQ(std::memcmp(h.db1.data(), pattern.data(), DB_SIZE), 0);

    std::vector<char> out(DB_SIZE);
    CHECK_EQ(Cli_DBRead(h.client, DB1, 0, DB_SIZE, out.data()), 0);
    CHECK_EQ(out, pattern);
}

TEST(integration_multi_vars_multiple_items) {
    Harness h;
    word w1[2] = {0x0102, 0x0304};
    word w2[2] = {0x0506, 0x0708};
    TS7DataItem items[2];
    std::memset(items, 0, sizeof(items));
    items[0].Area = S7AreaDB; items[0].WordLen = S7WLWord; items[0].DBNumber = DB1;
    items[0].Start = 0; items[0].Amount = 2; items[0].pdata = w1;
    items[1].Area = S7AreaDB; items[1].WordLen = S7WLWord; items[1].DBNumber = DB1;
    items[1].Start = 4; items[1].Amount = 2; items[1].pdata = w2;

    CHECK_EQ(Cli_WriteMultiVars(h.client, items, 2), 0);
    CHECK_EQ(items[0].Result, 0);
    CHECK_EQ(items[1].Result, 0);
    const unsigned char expected[8] = {0x02, 0x01, 0x04, 0x03, 0x06, 0x05, 0x08, 0x07};
    CHECK_EQ(std::memcmp(h.db1.data(), expected, 8), 0);
}

TEST(integration_upload_download_delete) {
    Harness h;
    // Download a structurally valid load-memory block for DB1, then delete it.
    char block[128];
    for (int i = 0; i < (int)sizeof(block); ++i) block[i] = (char)i;
    block[5] = SubBlk_DB;                       // SubBlkType = DB
    block[6] = 0x00; block[7] = 0x01;           // BlkNum = 1 (big-endian)
    block[8] = 0x00; block[9] = 0x00; block[10] = 0x00; block[11] = 0x80;  // LenLoadMem = 128
    block[34] = 0x00; block[35] = 0x10;         // MC7Len = 16

    CHECK_EQ(Cli_Download(h.client, DB1, block, (int)sizeof(block)), 0);
    // The downloaded load-memory image is stored into DB1. Spot-check bytes
    // outside the fields the client rewrites (block number and footer
    // checksum).
    CHECK_EQ((unsigned char)h.db1[36], (unsigned char)block[36]);
    CHECK_EQ((unsigned char)h.db1[100], (unsigned char)block[100]);
    CHECK_EQ(Cli_Delete(h.client, Block_DB, DB1), 0);
}

TEST(integration_upload_round_trip) {
    Harness h;
    char pattern[DB_SIZE];
    for (int i = 0; i < DB_SIZE; ++i) pattern[i] = (char)i;
    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, DB_SIZE, pattern), 0);

    char buf[DB_SIZE + 16];
    int size = DB_SIZE;
    CHECK_EQ(Cli_Upload(h.client, Block_DB, DB1, buf, size), 0);
    CHECK_EQ(size, DB_SIZE);
    CHECK_EQ(std::memcmp(buf, pattern, DB_SIZE), 0);
}

TEST(integration_full_upload) {
    Harness h;
    char buf[1024];
    int size = (int)sizeof(buf);
    CHECK_EQ(Cli_FullUpload(h.client, Block_DB, DB1, buf, size), 0);
    // Full upload returns the 36-byte block info header plus the DB data.
    CHECK_EQ(size, 36 + DB_SIZE);
}

TEST(integration_upload_buffer_too_small) {
    Harness h;
    char buf[64];
    int size = 64;
    CHECK_EQ(Cli_Upload(h.client, Block_DB, DB1, buf, size), errCliPartialDataRead);
}

TEST(integration_read_szl_all_ids) {
    Harness h;

    // SZL IDs with a direct data bank (independent of the index).
    const int direct_ids[] = {
        0x0000, 0x0F00, 0x0002, 0x0011, 0x0012, 0x0013, 0x0014, 0x0015,
        0x0F14, 0x0019, 0x0F19, 0x001C, 0x0F1C, 0x0036, 0x0F36, 0x0025,
        0x0F25, 0x0037, 0x0F37, 0x0074, 0x0F74, 0x0591, 0x0A91, 0x0F92,
        0x0294, 0x0794, 0x0F94, 0x0095, 0x0F95, 0x00A0, 0x0FA0, 0x0017,
        0x0F17, 0x0018, 0x0F18, 0x001A, 0x0F1A, 0x001B, 0x0F1B, 0x0021,
        0x0A21, 0x0F21, 0x0023, 0x0F23, 0x0024, 0x0124, 0x0424, 0x0038,
        0x0F38, 0x003A, 0x0F3A, 0x0F9A,
    };
    for (int id : direct_ids) {
        TS7SZL buf;
        int size = (int)sizeof(buf);
        CHECK_EQ(Cli_ReadSZL(h.client, id, 0x0000, &buf, size), 0);
    }

    // SZL IDs whose availability depends on the requested index. Every valid
    // (ID, index) combination is enumerated.
    struct Pair { int id; int index; };
    const Pair indexed[] = {
        {0x0D91, 0x0000}, {0x0092, 0x0000}, {0x0292, 0x0000},
        {0x0692, 0x0000}, {0x0094, 0x0000}, {0x0D97, 0x0000},
        {0x0111, 0x0001}, {0x0111, 0x0006}, {0x0111, 0x0007},
        {0x0F11, 0x0001}, {0x0F11, 0x0006}, {0x0F11, 0x0007},
        {0x0112, 0x0000}, {0x0112, 0x0100}, {0x0112, 0x0200}, {0x0112, 0x0400},
        {0x0F12, 0x0000}, {0x0F12, 0x0100}, {0x0F12, 0x0200}, {0x0F12, 0x0400},
        {0x0113, 0x0001}, {0x0115, 0x0800},
        {0x011C, 0x0001}, {0x011C, 0x0002}, {0x011C, 0x0003}, {0x011C, 0x0004},
        {0x011C, 0x0005}, {0x011C, 0x0007}, {0x011C, 0x0008}, {0x011C, 0x0009},
        {0x011C, 0x000A}, {0x011C, 0x000B},
        {0x0222, 0x0001}, {0x0222, 0x000A}, {0x0222, 0x0014},
        {0x0222, 0x0028}, {0x0222, 0x0050}, {0x0222, 0x0064},
        {0x0125, 0x0000}, {0x0125, 0x0001}, {0x0225, 0x0001},
        {0x0137, 0x07FE}, {0x0174, 0x0001}, {0x0174, 0x0004},
        {0x0174, 0x0005}, {0x0174, 0x0006}, {0x0174, 0x000B}, {0x0174, 0x000C},
        {0x0194, 0x0064}, {0x0694, 0x0064}, {0x0232, 0x0001}, {0x0232, 0x0004},
        {0x0C91, 0x07FE},
    };
    for (const Pair& p : indexed) {
        TS7SZL buf;
        int size = (int)sizeof(buf);
        CHECK_EQ(Cli_ReadSZL(h.client, p.id, p.index, &buf, size), 0);
    }

    // Ranges of valid indices.
    const int id_ranges[][3] = {
        {0x0131, 0x0001, 0x0009},
        {0x0117, 0x0000, 0x0004},
        {0x0118, 0x0000, 0x0003},
        {0x0132, 0x0001, 0x000C},
        {0x01A0, 0x0000, 0x0015},
    };
    for (const auto& r : id_ranges) {
        for (int index = r[1]; index <= r[2]; ++index) {
            TS7SZL buf;
            int size = (int)sizeof(buf);
            CHECK_EQ(Cli_ReadSZL(h.client, r[0], index, &buf, size), 0);
        }
    }

    // An index not covered by an index-dependent ID is reported unavailable.
    TS7SZL buf;
    int size = (int)sizeof(buf);
    CHECK_NE(Cli_ReadSZL(h.client, 0x0111, 0x0000, &buf, size), 0);
}

TEST(integration_pdu_fuzzing_invalid_type) {
    Harness h;
    int timeout = 300;
    CHECK_EQ(Cli_SetParam(h.client, p_i32_RecvTimeout, &timeout), 0);

    // Well-sized header (ParLen=0, DataLen=0) but an invalid PDUType hits the
    // PDUType branch of CheckPDU_in.
    char buf[10] = {0};
    buf[0] = 0x32;
    buf[1] = 0x09;  // neither request (1) nor userdata (7)
    int size = (int)sizeof(buf);
    CHECK_NE(Cli_IsoExchangeBuffer(h.client, buf, size), 0);
}

TEST(integration_async_callback) {
    Harness h;
    g_callback_calls = 0;
    g_callback_op_code = -1;
    g_callback_op_result = -1;
    CHECK_EQ(Cli_SetAsCallback(h.client, test_completion_callback, nullptr), 0);

    char pattern[32] = {0};
    CHECK_EQ(Cli_AsDBWrite(h.client, DB1, 0, (int)sizeof(pattern), pattern), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    // The completion callback runs on the client thread right after the event
    // is signalled, so poll briefly.
    for (int i = 0; i < 200 && g_callback_calls == 0; ++i) SysSleep(10);
    CHECK_EQ(g_callback_calls, 1);
    CHECK_EQ(g_callback_op_code, s7opWriteArea);
    CHECK_EQ(g_callback_op_result, 0);
}

// ---------------------------------------------------------------------------
// All asynchronous lean/block/control functions, each drained with
// WaitAsCompletion.
// ---------------------------------------------------------------------------
TEST(integration_async_read_write_areas) {
    Harness h;
    char buf[32] = {0};

    CHECK_EQ(Cli_AsWriteArea(h.client, S7AreaDB, DB1, 0, 32, S7WLByte, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsReadArea(h.client, S7AreaDB, DB1, 0, 32, S7WLByte, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    CHECK_EQ(Cli_AsMBWrite(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsMBRead(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    CHECK_EQ(Cli_AsEBWrite(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsEBRead(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    CHECK_EQ(Cli_AsABWrite(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsABRead(h.client, 0, 16, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
}

TEST(integration_async_timer_counter) {
    Harness h;
    char buf[16] = {0};

    CHECK_EQ(Cli_AsTMWrite(h.client, 0, 8, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsTMRead(h.client, 0, 8, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    CHECK_EQ(Cli_AsCTWrite(h.client, 0, 8, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsCTRead(h.client, 0, 8, buf), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
}

TEST(integration_async_block_operations) {
    Harness h;

    TS7BlocksOfType items;
    std::memset(items, 0, sizeof(items));
    int count = 0x2000;
    CHECK_EQ(Cli_AsListBlocksOfType(h.client, Block_DB, &items, count), 0);
    // AsListBlocksOfType does not copy ItemsCount into Job.Amount (a library
    // quirk), so the async listing reports a partial read instead of success.
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), errCliPartialDataRead);

    TS7SZL szl;
    int size = (int)sizeof(szl);
    CHECK_EQ(Cli_AsReadSZL(h.client, 0x0011, 0x0000, &szl, size), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    TS7SZLList list;
    count = (int)(sizeof(list) / 2);
    CHECK_EQ(Cli_AsReadSZLList(h.client, &list, count), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    // Upload/download are refused by the simulated CPU.
    char buf[256] = {0};
    size = (int)sizeof(buf);
    CHECK_EQ(Cli_AsUpload(h.client, Block_DB, DB1, buf, size), 0);
    CHECK_NE(Cli_WaitAsCompletion(h.client, 5000), 0);
    size = (int)sizeof(buf);
    CHECK_EQ(Cli_AsFullUpload(h.client, Block_DB, DB1, buf, size), 0);
    CHECK_NE(Cli_WaitAsCompletion(h.client, 5000), 0);
    size = (int)sizeof(buf);
    CHECK_EQ(Cli_AsDownload(h.client, DB1, buf, size), 0);
    CHECK_NE(Cli_WaitAsCompletion(h.client, 5000), 0);
}

TEST(integration_async_control_and_db) {
    Harness h;

    CHECK_EQ(Cli_AsCopyRamToRom(h.client, 1000), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
    CHECK_EQ(Cli_AsCompress(h.client, 1000), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    char buf[DB_SIZE];
    int size = DB_SIZE;
    CHECK_EQ(Cli_AsDBGet(h.client, DB1, buf, size), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);

    CHECK_EQ(Cli_AsDBFill(h.client, DB1, 0x5A), 0);
    CHECK_EQ(Cli_WaitAsCompletion(h.client, 5000), 0);
}

// ---------------------------------------------------------------------------
// Server read/write error paths (area not found, out of range, etc.)
// ---------------------------------------------------------------------------
TEST(integration_read_unregistered_area) {
    Harness h;
    char buf[8] = {0};
    CHECK_NE(Cli_DBRead(h.client, 99, 0, 8, buf), 0);
}

TEST(integration_read_out_of_range) {
    Harness h;
    char buf[8] = {0};
    // DB1 is DB_SIZE bytes; an 8-byte read at offset DB_SIZE-4 overruns.
    CHECK_NE(Cli_DBRead(h.client, DB1, DB_SIZE - 4, 8, buf), 0);
}

TEST(integration_write_unregistered_area) {
    Harness h;
    char buf[8] = {0};
    CHECK_NE(Cli_DBWrite(h.client, 99, 0, 8, buf), 0);
}

TEST(integration_write_out_of_range) {
    Harness h;
    char buf[8] = {0};
    CHECK_NE(Cli_DBWrite(h.client, DB1, DB_SIZE - 4, 8, buf), 0);
}

TEST(integration_read_unregistered_sys_area) {
    Harness h;
    char buf[8] = {0};
    // PE is registered, but reading from an unregistered DB-based area fails.
    CHECK_NE(Cli_MBRead(h.client, SYS_SIZE + 1, 4, buf), 0);
}

TEST(integration_download_multiple_slices) {
    Harness h;
    // A block larger than the negotiated PDU slice forces the client's
    // download loop to fragment the transfer over several request/response
    // cycles.
    char block[DB_SIZE];
    for (int i = 0; i < (int)sizeof(block); ++i) block[i] = (char)(i & 0xFF);
    block[5] = SubBlk_DB;                       // SubBlkType = DB
    block[6] = 0x00; block[7] = 0x01;           // BlkNum = 1 (big-endian)
    block[8] = 0x00; block[9] = 0x00; block[10] = 0x02; block[11] = 0x00;  // LenLoadMem = 512
    block[34] = 0x00; block[35] = 0x64;         // MC7Len = 100

    CHECK_EQ(Cli_Download(h.client, DB1, block, (int)sizeof(block)), 0);
    // Spot-check the stored image (outside the client-rewritten fields).
    CHECK_EQ((unsigned char)h.db1[36], (unsigned char)block[36]);
    CHECK_EQ((unsigned char)h.db1[400], (unsigned char)block[400]);
    CHECK_EQ(Cli_Delete(h.client, Block_DB, DB1), 0);
}

TEST(integration_list_blocks_of_type_no_blocks) {
    Harness h;
    TS7BlocksOfType items;
    std::memset(items, 0, sizeof(items));
    int count = 0x2000;
    // The simulated CPU has no OBs.
    CHECK_NE(Cli_ListBlocksOfType(h.client, Block_OB, &items, count), 0);
}

TEST(integration_get_ag_block_info_nonexistent) {
    Harness h;
    TS7BlockInfo info;
    std::memset(&info, 0, sizeof(info));
    // DB 99 is not registered.
    CHECK_NE(Cli_GetAgBlockInfo(h.client, Block_DB, 99, &info), 0);
}

TEST(integration_rw_area_callback) {
    Harness h;
    g_rw_callback_calls = 0;
    CHECK_EQ(Srv_SetRWAreaCallback(h.server, test_rw_callback, nullptr), 0);

    char buf[8] = {0};
    CHECK_EQ(Cli_DBRead(h.client, DB1, 0, 8, buf), 0);
    CHECK_EQ(Cli_DBWrite(h.client, DB1, 0, 8, buf), 0);
    CHECK(g_rw_callback_calls > 0);
}
