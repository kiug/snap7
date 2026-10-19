// White-box unit tests.
//
// These reach private/protected members by redefining the access specifiers to
// `public` before including the Snap7 headers. Access specifiers do not affect
// object layout, so the resulting objects link correctly against the normally
// compiled library (which does not see these macros).

#define private public
#define protected public

#include "snap7_libmain.h"

#undef private
#undef protected

#include "test.h"

// ---------------------------------------------------------------------------
// TSnapBase (byte swapping)
// ---------------------------------------------------------------------------
TEST(whitebox_swap_word) {
    TSnapBase b;
    CHECK(b.LittleEndian);  // x86-64
    CHECK_EQ((int)b.SwapWord(0x1234), 0x3412);
    CHECK_EQ((int)b.SwapWord(0x0000), 0x0000);
}

TEST(whitebox_swap_dword) {
    TSnapBase b;
    CHECK_EQ(b.SwapDWord(0x12345678), 0x78563412u);
    CHECK_EQ(b.SwapDWord(0x00000000), 0x00000000u);
}

// ---------------------------------------------------------------------------
// TSnap7MicroClient (BCD, sizes, blocks, CPU errors)
// ---------------------------------------------------------------------------
TEST(whitebox_bcd_to_byte) {
    TSnap7MicroClient c;
    CHECK_EQ((int)c.BCDtoByte(0x00), 0);
    CHECK_EQ((int)c.BCDtoByte(0x09), 9);
    CHECK_EQ((int)c.BCDtoByte(0x10), 10);
    CHECK_EQ((int)c.BCDtoByte(0x59), 59);
    CHECK_EQ((int)c.BCDtoByte(0x99), 99);
}

TEST(whitebox_word_to_bcd) {
    TSnap7MicroClient c;
    CHECK_EQ((int)c.WordToBCD(0), 0x00);
    CHECK_EQ((int)c.WordToBCD(9), 0x09);
    CHECK_EQ((int)c.WordToBCD(10), 0x10);
    CHECK_EQ((int)c.WordToBCD(59), 0x59);
    CHECK_EQ((int)c.WordToBCD(99), 0x99);
}

TEST(whitebox_data_size_byte) {
    TSnap7MicroClient c;
    CHECK_EQ(c.DataSizeByte(S7WLBit), 1);
    CHECK_EQ(c.DataSizeByte(S7WLByte), 1);
    CHECK_EQ(c.DataSizeByte(S7WLChar), 1);
    CHECK_EQ(c.DataSizeByte(S7WLWord), 2);
    CHECK_EQ(c.DataSizeByte(S7WLDWord), 4);
    CHECK_EQ(c.DataSizeByte(S7WLInt), 2);
    CHECK_EQ(c.DataSizeByte(S7WLDInt), 4);
    CHECK_EQ(c.DataSizeByte(S7WLReal), 4);
    CHECK_EQ(c.DataSizeByte(S7WLCounter), 2);
    CHECK_EQ(c.DataSizeByte(S7WLTimer), 2);
    CHECK_EQ(c.DataSizeByte(0x99), 0);
}

TEST(whitebox_dword_at) {
    TSnap7MicroClient c;
    longword value = 0x12345678;
    CHECK_EQ(c.DWordAt(&value), 0x78563412u);
}

TEST(whitebox_sub_block_to_block) {
    TSnap7MicroClient c;
    CHECK_EQ(c.SubBlockToBlock(SubBlk_OB), (int)Block_OB);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_DB), (int)Block_DB);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_SDB), (int)Block_SDB);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_FC), (int)Block_FC);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_SFC), (int)Block_SFC);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_FB), (int)Block_FB);
    CHECK_EQ(c.SubBlockToBlock(SubBlk_SFB), (int)Block_SFB);
    CHECK_EQ(c.SubBlockToBlock(0xFF), 0);
}

TEST(whitebox_cpu_error) {
    TSnap7MicroClient c;
    CHECK_EQ(c.CpuError(0), 0);
    CHECK_EQ(c.CpuError(Code7AddressOutOfRange), errCliAddressOutOfRange);
    CHECK_EQ(c.CpuError(Code7InvalidTransportSize), errCliInvalidTransportSize);
    CHECK_EQ(c.CpuError(Code7WriteDataSizeMismatch), errCliWriteDataSizeMismatch);
    CHECK_EQ(c.CpuError(Code7ResItemNotAvailable), errCliItemNotAvailable);
    CHECK_EQ(c.CpuError(Code7ResItemNotAvailable1), errCliItemNotAvailable);
    CHECK_EQ(c.CpuError(Code7DataOverPDU), errCliSizeOverPDU);
    CHECK_EQ(c.CpuError(Code7InvalidValue), errCliInvalidValue);
    CHECK_EQ(c.CpuError(Code7FunNotAvailable), errCliFunNotAvailable);
    CHECK_EQ(c.CpuError(Code7NeedPassword), errCliNeedPassword);
    CHECK_EQ(c.CpuError(Code7InvalidPassword), errCliInvalidPassword);
    CHECK_EQ(c.CpuError(Code7NoPasswordToSet), errCliNoPasswordToSetOrClear);
    CHECK_EQ(c.CpuError(Code7NoPasswordToClear), errCliNoPasswordToSetOrClear);
    CHECK_EQ(c.CpuError(0xFFFF), errCliFunctionRefused);
}

TEST(whitebox_check_block_invalid_type) {
    TSnap7MicroClient c;
    char block[128] = {0};
    CHECK_EQ(c.CheckBlock(0xFF, -1, block, 128), errCliInvalidBlockType);
}

TEST(whitebox_check_block_invalid_number) {
    TSnap7MicroClient c;
    char block[128] = {0};
    CHECK_EQ(c.CheckBlock(Block_DB, 0x10000, block, 128), errCliInvalidBlockNumber);
}

TEST(whitebox_check_block_size_mismatch) {
    TSnap7MicroClient c;
    char block[128] = {0};
    CHECK_EQ(c.CheckBlock(Block_DB, 1, block, 128), errCliInvalidBlockSize);
}

TEST(whitebox_check_block_valid) {
    TSnap7MicroClient c;
    char block[128];
    std::memset(block, 0, sizeof(block));
    // TS7CompactBlockInfo is packed; LenLoadMem (u_int) is at offset 8 and
    // MC7Len (word) at offset 34. Both are stored big-endian.
    block[8] = 0; block[9] = 0; block[10] = 0; block[11] = 0x80;  // LenLoadMem = 128
    block[34] = 0; block[35] = 0x10;                               // MC7Len = 16
    CHECK_EQ(c.CheckBlock(-1, -1, block, 128), 0);
}

TEST(whitebox_fill_time) {
    TSnap7MicroClient c;
    char buf[11] = {0};
    c.FillTime(0, buf);  // 0 seconds after 1984-01-01
    CHECK_STREQ(buf, "1984/01/01");
}

// ---------------------------------------------------------------------------
// TSnap7Peer (sequence counter, error handling)
// ---------------------------------------------------------------------------
TEST(whitebox_get_next_word) {
    TSnap7Peer p;
    CHECK_EQ((int)p.GetNextWord(), 0);
    CHECK_EQ((int)p.GetNextWord(), 1);
    CHECK_EQ((int)p.GetNextWord(), 2);
    // Wrap at 0xFFFF.
    p.cntword = 0xFFFF;
    CHECK_EQ((int)p.GetNextWord(), 0);
    CHECK_EQ((int)p.GetNextWord(), 1);
}

TEST(whitebox_set_and_clear_error) {
    TSnap7Peer p;
    CHECK_EQ(p.SetError(0), 0);
    CHECK_EQ(p.LastError, 0);
    CHECK_EQ(p.SetError(0x1234), 0x1234);
    CHECK_NE(p.LastError, 0);
    p.ClrError();
    CHECK_EQ(p.LastError, 0);
    CHECK_EQ(p.LastIsoError, 0);
    CHECK_EQ(p.LastTcpError, 0);
}

// ---------------------------------------------------------------------------
// TSnap7Server (DB/sys area registration)
// ---------------------------------------------------------------------------
TEST(whitebox_register_and_find_db) {
    TSnap7Server s;
    char buf[16] = {0};
    CHECK_EQ((int)s.RegisterDB(5, buf, 16), 0);
    CHECK(s.FindDB(5) != nullptr);
    CHECK(s.FindDB(6) == nullptr);
    CHECK_EQ(s.IndexOfDB(5), 0);
    CHECK_EQ(s.IndexOfDB(6), -1);
    CHECK_EQ((int)s.UnregisterDB(5), 0);
    CHECK(s.FindDB(5) == nullptr);
}

TEST(whitebox_register_and_unregister_sys) {
    TSnap7Server s;
    char buf[16] = {0};
    CHECK_EQ((int)s.RegisterSys(srvAreaMK, buf, 16), 0);
    CHECK(s.HA[srvAreaMK] != nullptr);
    CHECK_EQ((int)s.UnregisterSys(srvAreaMK), 0);
    CHECK(s.HA[srvAreaMK] == nullptr);
}

// ---------------------------------------------------------------------------
// TSnap7Partner (byte counter)
// ---------------------------------------------------------------------------
TEST(whitebox_get_next_byte) {
    TSnap7Partner p(true);
    CHECK_EQ((int)p.GetNextByte(), 1);
    CHECK_EQ((int)p.GetNextByte(), 2);
    CHECK_EQ((int)p.GetNextByte(), 3);
    // Wrap: 254 -> 255 -> reset to 1.
    p.NextByte = 254;
    CHECK_EQ((int)p.GetNextByte(), 1);
}

// ---------------------------------------------------------------------------
// TS7Worker (BCD, transport sizes)
// ---------------------------------------------------------------------------
TEST(whitebox_worker_bcd) {
    TS7Worker w;
    CHECK_EQ((int)w.BCD(0), 0x00);
    CHECK_EQ((int)w.BCD(9), 0x09);
    CHECK_EQ((int)w.BCD(10), 0x10);
    CHECK_EQ((int)w.BCD(59), 0x59);
    CHECK_EQ((int)w.BCD(99), 0x99);
}

TEST(whitebox_worker_data_size_byte) {
    TS7Worker w;
    CHECK_EQ(w.DataSizeByte(S7WLBit), 1);
    CHECK_EQ(w.DataSizeByte(S7WLByte), 1);
    CHECK_EQ(w.DataSizeByte(S7WLWord), 2);
    CHECK_EQ(w.DataSizeByte(S7WLDWord), 4);
    CHECK_EQ(w.DataSizeByte(S7WLReal), 4);
    CHECK_EQ(w.DataSizeByte(S7WLCounter), 2);
    CHECK_EQ(w.DataSizeByte(S7WLTimer), 2);
    CHECK_EQ(w.DataSizeByte(0x99), 0);
}

// ---------------------------------------------------------------------------
// TSnap7MicroClient (job-pending branches of every synchronous entry point)
// ---------------------------------------------------------------------------
TEST(whitebox_client_job_pending_every_op) {
    TSnap7MicroClient c;
    c.Job.Pending = true;  // simulate a busy client

    char buf[32] = {0};
    int size = (int)sizeof(buf);
    int count = 1;
    int status = 0;
    tm dt;
    std::memset(&dt, 0, sizeof(dt));
    TS7SZL szl;
    TS7SZLList szllist;
    TS7OrderCode oc;
    TS7CpuInfo cpu;
    TS7CpInfo cp;
    TS7Protection prot;
    TS7BlockInfo binfo;
    TS7BlocksList blist;
    TS7BlocksOfType bot;

    CHECK_EQ(c.ReadArea(S7AreaDB, 1, 0, 8, S7WLByte, buf), errCliJobPending);
    CHECK_EQ(c.WriteArea(S7AreaDB, 1, 0, 8, S7WLByte, buf), errCliJobPending);
    CHECK_EQ(c.ReadMultiVars(nullptr, 1), errCliJobPending);
    CHECK_EQ(c.WriteMultiVars(nullptr, 1), errCliJobPending);
    CHECK_EQ(c.Upload(Block_DB, 1, buf, size), errCliJobPending);
    CHECK_EQ(c.FullUpload(Block_DB, 1, buf, size), errCliJobPending);
    CHECK_EQ(c.Download(1, buf, (int)sizeof(buf)), errCliJobPending);
    CHECK_EQ(c.Delete(Block_DB, 1), errCliJobPending);
    CHECK_EQ(c.DBGet(1, buf, size), errCliJobPending);
    CHECK_EQ(c.DBFill(1, 0x00), errCliJobPending);
    CHECK_EQ(c.GetPlcDateTime(dt), errCliJobPending);
    CHECK_EQ(c.SetPlcDateTime(&dt), errCliJobPending);
    CHECK_EQ(c.GetOrderCode(&oc), errCliJobPending);
    CHECK_EQ(c.GetCpuInfo(&cpu), errCliJobPending);
    CHECK_EQ(c.GetCpInfo(&cp), errCliJobPending);
    CHECK_EQ(c.ReadSZL(0x0011, 0, &szl, size), errCliJobPending);
    CHECK_EQ(c.ReadSZLList(&szllist, count), errCliJobPending);
    CHECK_EQ(c.PlcHotStart(), errCliJobPending);
    CHECK_EQ(c.PlcColdStart(), errCliJobPending);
    CHECK_EQ(c.PlcStop(), errCliJobPending);
    CHECK_EQ(c.CopyRamToRom(1000), errCliJobPending);
    CHECK_EQ(c.Compress(1000), errCliJobPending);
    CHECK_EQ(c.GetPlcStatus(status), errCliJobPending);
    CHECK_EQ(c.GetProtection(&prot), errCliJobPending);
    CHECK_EQ(c.SetSessionPassword((char*)"abc"), errCliJobPending);
    CHECK_EQ(c.ClearSessionPassword(), errCliJobPending);
    CHECK_EQ(c.ListBlocks(&blist), errCliJobPending);
    CHECK_EQ(c.ListBlocksOfType(Block_DB, &bot, count), errCliJobPending);
    CHECK_EQ(c.GetAgBlockInfo(Block_DB, 1, &binfo), errCliJobPending);
}
