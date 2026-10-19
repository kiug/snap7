// Client lifecycle, parameter and error-path tests (no server required).

#include "snap7_libmain.h"
#include "test.h"

namespace {
struct ClientGuard {
    S7Object h;
    ClientGuard() : h(Cli_Create()) {}
    ~ClientGuard() {
        if (h) Cli_Destroy(h);
    }
};
}  // namespace

TEST(client_create_and_destroy) {
    S7Object c = Cli_Create();
    CHECK_NE(c, (S7Object)0);
    Cli_Destroy(c);
}

TEST(client_destroy_null_is_noop) {
    S7Object c = 0;
    Cli_Destroy(c);
    CHECK_EQ(c, (S7Object)0);
}

TEST(client_null_object_errors) {
    int value = 0;
    char buf[16] = {0};
    CHECK_EQ(Cli_Connect(0), errLibInvalidObject);
    CHECK_EQ(Cli_Disconnect(0), errLibInvalidObject);
    CHECK_EQ(Cli_ConnectTo(0, "127.0.0.1", 0, 2), errLibInvalidObject);
    CHECK_EQ(Cli_SetConnectionType(0, CONNTYPE_PG), errLibInvalidObject);
    CHECK_EQ(Cli_SetConnectionParams(0, "127.0.0.1", 0x0100, 0x0200), errLibInvalidObject);
    CHECK_EQ(Cli_GetParam(0, p_u16_RemotePort, &value), errLibInvalidObject);
    CHECK_EQ(Cli_SetParam(0, p_u16_RemotePort, &value), errLibInvalidObject);
    CHECK_EQ(Cli_SetAsCallback(0, nullptr, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_ReadArea(0, S7AreaDB, 1, 0, 1, S7WLByte, buf), errLibInvalidObject);
    CHECK_EQ(Cli_WriteArea(0, S7AreaDB, 1, 0, 1, S7WLByte, buf), errLibInvalidObject);
    CHECK_EQ(Cli_GetConnected(0, value), errLibInvalidObject);
    CHECK_EQ(Cli_GetLastError(0, value), errLibInvalidObject);
}

TEST(client_all_functions_null_object) {
    int value = 0;
    int size = 8;
    int count = 1;
    longword mask = 0;
    char buf[16] = {0};
    tm dt;
    std::memset(&dt, 0, sizeof(dt));

    // Data I/O.
    CHECK_EQ(Cli_ReadMultiVars(0, nullptr, 0), errLibInvalidObject);
    CHECK_EQ(Cli_WriteMultiVars(0, nullptr, 0), errLibInvalidObject);
    CHECK_EQ(Cli_DBRead(0, 1, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_DBWrite(0, 1, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_MBRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_MBWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_EBRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_EBWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_ABRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_ABWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_TMRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_TMWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_CTRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_CTWrite(0, 0, 8, buf), errLibInvalidObject);

    // Directory / blocks.
    CHECK_EQ(Cli_ListBlocks(0, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_GetAgBlockInfo(0, Block_DB, 1, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_GetPgBlockInfo(0, buf, nullptr, 8), errLibInvalidObject);
    CHECK_EQ(Cli_ListBlocksOfType(0, Block_DB, nullptr, count), errLibInvalidObject);
    CHECK_EQ(Cli_Upload(0, Block_DB, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_FullUpload(0, Block_DB, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_Download(0, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_Delete(0, Block_DB, 1), errLibInvalidObject);
    CHECK_EQ(Cli_DBGet(0, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_DBFill(0, 1, 0), errLibInvalidObject);

    // Date/time.
    CHECK_EQ(Cli_GetPlcDateTime(0, dt), errLibInvalidObject);
    CHECK_EQ(Cli_SetPlcDateTime(0, &dt), errLibInvalidObject);
    CHECK_EQ(Cli_SetPlcSystemDateTime(0), errLibInvalidObject);

    // System info.
    CHECK_EQ(Cli_GetOrderCode(0, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_GetCpuInfo(0, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_GetCpInfo(0, nullptr), errLibInvalidObject);
    CHECK_EQ(Cli_ReadSZL(0, 0x0011, 0, nullptr, size), errLibInvalidObject);
    CHECK_EQ(Cli_ReadSZLList(0, nullptr, count), errLibInvalidObject);

    // Control.
    CHECK_EQ(Cli_PlcHotStart(0), errLibInvalidObject);
    CHECK_EQ(Cli_PlcColdStart(0), errLibInvalidObject);
    CHECK_EQ(Cli_PlcStop(0), errLibInvalidObject);
    CHECK_EQ(Cli_CopyRamToRom(0, 1000), errLibInvalidObject);
    CHECK_EQ(Cli_Compress(0, 1000), errLibInvalidObject);
    CHECK_EQ(Cli_GetPlcStatus(0, value), errLibInvalidObject);

    // Security.
    CHECK_EQ(Cli_GetProtection(0, nullptr), errLibInvalidObject);

    // Low level / misc.
    CHECK_EQ(Cli_IsoExchangeBuffer(0, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_GetExecTime(0, value), errLibInvalidObject);
    CHECK_EQ(Cli_GetPduLength(0, value, value), errLibInvalidObject);

    // Async.
    // (Note: Cli_AsReadArea returns errLibInvalidParam, not InvalidObject.)
    CHECK_EQ(Cli_AsReadArea(0, S7AreaDB, 1, 0, 1, S7WLByte, buf), errLibInvalidParam);
    CHECK_EQ(Cli_AsWriteArea(0, S7AreaDB, 1, 0, 1, S7WLByte, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsDBRead(0, 1, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsDBWrite(0, 1, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsMBRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsMBWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsEBRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsEBWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsABRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsABWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsTMRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsTMWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsCTRead(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsCTWrite(0, 0, 8, buf), errLibInvalidObject);
    CHECK_EQ(Cli_AsListBlocksOfType(0, Block_DB, nullptr, count), errLibInvalidObject);
    CHECK_EQ(Cli_AsReadSZL(0, 0x0011, 0, nullptr, size), errLibInvalidObject);
    CHECK_EQ(Cli_AsReadSZLList(0, nullptr, count), errLibInvalidObject);
    CHECK_EQ(Cli_AsUpload(0, Block_DB, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_AsFullUpload(0, Block_DB, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_AsDownload(0, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_AsCopyRamToRom(0, 1000), errLibInvalidObject);
    CHECK_EQ(Cli_AsCompress(0, 1000), errLibInvalidObject);
    CHECK_EQ(Cli_AsDBGet(0, 1, buf, size), errLibInvalidObject);
    CHECK_EQ(Cli_AsDBFill(0, 1, 0), errLibInvalidObject);
    CHECK_EQ(Cli_CheckAsCompletion(0, value), errLibInvalidObject);
    CHECK_EQ(Cli_WaitAsCompletion(0, 100), errLibInvalidObject);
}

TEST(client_get_param_invalid_number) {
    ClientGuard c;
    int value = 0;
    CHECK_EQ(Cli_GetParam(c.h, 9999, &value), errCliInvalidParamNumber);
}

TEST(client_set_param_invalid_number) {
    ClientGuard c;
    int value = 42;
    CHECK_EQ(Cli_SetParam(c.h, 9999, &value), errCliInvalidParamNumber);
}

TEST(client_remote_port_round_trip) {
    ClientGuard c;
    word port = 1234;
    CHECK_EQ(Cli_SetParam(c.h, p_u16_RemotePort, &port), 0);
    word got = 0;
    CHECK_EQ(Cli_GetParam(c.h, p_u16_RemotePort, &got), 0);
    CHECK_EQ(got, port);
}

TEST(client_integer_params_round_trip) {
    ClientGuard c;
    int value = 5000;
    CHECK_EQ(Cli_SetParam(c.h, p_i32_PingTimeout, &value), 0);
    int got = 0;
    CHECK_EQ(Cli_GetParam(c.h, p_i32_PingTimeout, &got), 0);
    CHECK_EQ(got, value);

    value = 960;
    CHECK_EQ(Cli_SetParam(c.h, p_i32_PDURequest, &value), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_PDURequest, &got), 0);
    CHECK_EQ(got, value);
}

TEST(client_all_params_round_trip) {
    ClientGuard c;
    word w = 1234;
    int i = 5000;

    // GetParam succeeds for every valid param number.
    CHECK_EQ(Cli_GetParam(c.h, p_u16_RemotePort, &w), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_PingTimeout, &i), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_SendTimeout, &i), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_RecvTimeout, &i), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_WorkInterval, &i), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_u16_SrcRef, &w), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_u16_DstRef, &w), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_u16_SrcTSap, &w), 0);
    CHECK_EQ(Cli_GetParam(c.h, p_i32_PDURequest, &i), 0);

    // SetParam succeeds for every settable param number.
    CHECK_EQ(Cli_SetParam(c.h, p_u16_RemotePort, &w), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_i32_PingTimeout, &i), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_i32_SendTimeout, &i), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_i32_RecvTimeout, &i), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_i32_WorkInterval, &i), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_u16_SrcRef, &w), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_u16_DstRef, &w), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_u16_SrcTSap, &w), 0);
    CHECK_EQ(Cli_SetParam(c.h, p_i32_PDURequest, &i), 0);
}

TEST(client_connection_type_and_params) {
    ClientGuard c;
    CHECK_EQ(Cli_SetConnectionType(c.h, CONNTYPE_BASIC), 0);
    CHECK_EQ(Cli_SetConnectionParams(c.h, "127.0.0.1", 0x0100, 0x0200), 0);
}

TEST(client_read_area_invalid_word_len) {
    ClientGuard c;
    char buf[16] = {0};
    CHECK_EQ(Cli_ReadArea(c.h, S7AreaDB, 1, 0, 1, 0x99, buf), errCliInvalidWordLen);
}

TEST(client_read_area_invalid_params) {
    ClientGuard c;
    char buf[16] = {0};
    CHECK_EQ(Cli_ReadArea(c.h, S7AreaDB, 1, 0, 0, S7WLByte, buf), errCliInvalidParams);
    CHECK_EQ(Cli_ReadArea(c.h, S7AreaDB, -1, 0, 1, S7WLByte, buf), errCliInvalidParams);
}

TEST(client_read_area_without_connection_fails) {
    ClientGuard c;
    char buf[16] = {0};
    CHECK_NE(Cli_ReadArea(c.h, S7AreaDB, 1, 0, 1, S7WLByte, buf), 0);
}

TEST(client_connect_without_server_fails) {
    ClientGuard c;
    CHECK_NE(Cli_ConnectTo(c.h, "127.0.0.1", 0, 2), 0);
}

TEST(client_disconnect_without_connection_ok) {
    ClientGuard c;
    CHECK_EQ(Cli_Disconnect(c.h), 0);
}

TEST(client_connected_flag_false_before_connect) {
    ClientGuard c;
    int connected = -1;
    CHECK_EQ(Cli_GetConnected(c.h, connected), 0);
    CHECK_EQ(connected, 0);
}

TEST(client_last_error_zero_initially) {
    ClientGuard c;
    int last = -1;
    CHECK_EQ(Cli_GetLastError(c.h, last), 0);
    CHECK_EQ(last, 0);
}

TEST(client_set_as_callback_ok) {
    ClientGuard c;
    CHECK_EQ(Cli_SetAsCallback(c.h, nullptr, nullptr), 0);
}

TEST(client_check_as_completion_without_job) {
    ClientGuard c;
    int op_result = -1;
    CHECK_EQ(Cli_CheckAsCompletion(c.h, op_result), JobComplete);
    CHECK_EQ(op_result, 0);
}

TEST(client_wait_as_completion_without_job) {
    ClientGuard c;
    CHECK_EQ(Cli_WaitAsCompletion(c.h, 100), 0);
}

TEST(client_async_write_area_invalid_word_len) {
    ClientGuard c;
    char buf[16] = {0};
    CHECK_EQ(Cli_AsWriteArea(c.h, S7AreaDB, 1, 0, 1, 0x99, buf), errCliInvalidWordLen);
}

TEST(client_async_copy_ram_to_rom_invalid_timeout) {
    ClientGuard c;
    CHECK_EQ(Cli_AsCopyRamToRom(c.h, 0), errCliInvalidParams);
}

TEST(client_async_compress_invalid_timeout) {
    ClientGuard c;
    CHECK_EQ(Cli_AsCompress(c.h, 0), errCliInvalidParams);
}

TEST(client_async_db_get_invalid_size) {
    ClientGuard c;
    char buf[64] = {0};
    int size = 0;
    CHECK_EQ(Cli_AsDBGet(c.h, 1, buf, size), errCliInvalidBlockSize);
}

TEST(client_async_download_invalid_size) {
    ClientGuard c;
    char buf[64] = {0};
    CHECK_EQ(Cli_AsDownload(c.h, 1, buf, 0), errCliInvalidBlockSize);
}

TEST(client_list_blocks_of_type_invalid_count) {
    ClientGuard c;
    TS7BlocksOfType items;
    std::memset(items, 0, sizeof(items));
    int count = 0;
    CHECK_EQ(Cli_ListBlocksOfType(c.h, Block_DB, &items, count), errCliInvalidBlockSize);
}

TEST(client_db_get_invalid_size) {
    ClientGuard c;
    char buf[64] = {0};
    int size = 0;
    CHECK_EQ(Cli_DBGet(c.h, 1, buf, size), errCliInvalidBlockSize);
}
