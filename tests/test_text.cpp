// White-box unit tests for the error-text helpers in s7_text.cpp.
//
// The internal helpers (NumToString, CliTextOf, TcpTextOf, ...) have external
// linkage, so they can be exercised directly here.

#include <arpa/inet.h>
#include <ctime>

#include "s7_text.h"
#include "test.h"

// Internal helpers from s7_text.cpp (not declared in the public header).
char* NumToString(int Value, int Base, int Len, char* Result);
char* IntToString(int Value, char* Result);
char* TimeToString(time_t dt, char* Result);
char* IpAddressToString(int IP, char* Result);
char* TcpTextOf(int Error, char* Result);
char* IsoTextOf(int Error, char* Result);
char* CliTextOf(int Error, char* Result);
char* SrvTextOf(int Error, char* Result);
char* ParTextOf(int Error, char* Result);
char* SenderText(TSrvEvent& Event, char* Result);
char* TcpServerEventText(TSrvEvent& Event, char* Result);
char* PDUText(TSrvEvent& Event, char* Result);
char* TxtArea(TSrvEvent& Event, char* Result);
char* TxtStartSize(TSrvEvent& Event, char* Result);
char* TxtDataResult(TSrvEvent& Event, char* Result);
char* ControlText(word Code, char* Result);
char* ClockText(word Code, char* Result);
char* ReadSZLText(TSrvEvent& Event, char* Result);
char* UploadText(TSrvEvent& Event, char* Result);
char* DownloadText(TSrvEvent& Event, char* Result);
char* StrBlockType(word Code, char* Result);
char* BlockInfoText(TSrvEvent& Event, char* Result);
char* SecurityText(TSrvEvent& Event, char* Result);

// ---------------------------------------------------------------------------
// NumToString / IntToString
// ---------------------------------------------------------------------------
TEST(NumToString_decimal) {
    char buf[64];
    CHECK_STREQ(NumToString(123, 10, 0, buf), "123");
    CHECK_STREQ(NumToString(0, 10, 0, buf), "0");
    CHECK_STREQ(NumToString(-45, 10, 0, buf), "-45");
}

TEST(NumToString_hex) {
    char buf[64];
    CHECK_STREQ(NumToString(255, 16, 0, buf), "ff");
    CHECK_STREQ(NumToString(0x1A, 16, 0, buf), "1a");
    CHECK_STREQ(NumToString(0x1A2B3C, 16, 0, buf), "1a2b3c");
}

TEST(NumToString_padding) {
    char buf[64];
    CHECK_STREQ(NumToString(42, 10, 5, buf), "00042");
    CHECK_STREQ(NumToString(255, 16, 8, buf), "000000ff");
    // Padding shorter than the number itself has no effect.
    CHECK_STREQ(NumToString(12345, 10, 2, buf), "12345");
}

TEST(NumToString_invalid_base_returns_empty) {
    char buf[64];
    CHECK_STREQ(NumToString(42, 1, 0, buf), "");
    CHECK_STREQ(NumToString(42, 37, 0, buf), "");
}

TEST(IntToString_decimal) {
    char buf[64];
    CHECK_STREQ(IntToString(9876, buf), "9876");
    CHECK_STREQ(IntToString(-12, buf), "-12");
}

// ---------------------------------------------------------------------------
// TimeToString / IpAddressToString
// ---------------------------------------------------------------------------
TEST(TimeToString_valid) {
    char buf[64];
    time_t now = time(nullptr);
    char* r = TimeToString(now, buf);
    CHECK(r != nullptr);
    // Format is "%Y-%m-%d %H:%M:%S".
    CHECK_EQ(std::strlen(buf), size_t(19));
}

TEST(IpAddressToString_loopback) {
    char buf[64];
    IpAddressToString(inet_addr("127.0.0.1"), buf);
    CHECK_STREQ(buf, "127.0.0.1");
}

// ---------------------------------------------------------------------------
// TcpTextOf
// ---------------------------------------------------------------------------
TEST(TcpTextOf_known_codes) {
    char buf[256];
    CHECK_STREQ(TcpTextOf(0, buf), "");
    TcpTextOf(EINTR, buf);
    CHECK_CONTAINS(buf, "Interrupted system call");
    TcpTextOf(ECONNREFUSED, buf);
    CHECK_CONTAINS(buf, "Connection refused");
    TcpTextOf(12001, buf);  // WSAEINVALIDADDRESS
    CHECK_CONTAINS(buf, "Invalid address");
}

TEST(TcpTextOf_default) {
    char buf[256];
    TcpTextOf(9999, buf);
    CHECK_CONTAINS(buf, "Other Socket error");
}

TEST(TcpTextOf_entire_errno_range) {
    char buf[256];
    // Exercise every specific case plus the default path.
    for (int e = 1; e <= 200; ++e) {
        TcpTextOf(e, buf);
        if (e != 0) CHECK(buf[0] != '\0');
    }
}

// ---------------------------------------------------------------------------
// IsoTextOf
// ---------------------------------------------------------------------------
TEST(IsoTextOf_known_codes) {
    char buf[256];
    CHECK_STREQ(IsoTextOf(0, buf), "");
    IsoTextOf(errIsoConnect, buf);
    CHECK_CONTAINS(buf, "Connection error");
    IsoTextOf(errIsoInvalidPDU, buf);
    CHECK_CONTAINS(buf, "Bad PDU format");
    IsoTextOf(errIsoSendPacket, buf);
    CHECK_CONTAINS(buf, "send");
}

TEST(IsoTextOf_default) {
    char buf[256];
    IsoTextOf(0x000B1234, buf);
    CHECK_CONTAINS(buf, "Unknown error");
}

// ---------------------------------------------------------------------------
// CliTextOf
// ---------------------------------------------------------------------------
TEST(CliTextOf_known_codes) {
    char buf[256];
    CHECK_STREQ(CliTextOf(0, buf), "");
    CliTextOf(errCliInvalidParams, buf);
    CHECK_CONTAINS(buf, "invalid param");
    CliTextOf(errCliJobPending, buf);
    CHECK_CONTAINS(buf, "Job pending");
    CliTextOf(errCliInvalidWordLen, buf);
    CHECK_CONTAINS(buf, "WordLength");
    CliTextOf(errNegotiatingPDU, buf);
    CHECK_CONTAINS(buf, "PDU negotiation");
}

TEST(CliTextOf_default) {
    char buf[256];
    CliTextOf(0x7FFF0000, buf);
    CHECK_CONTAINS(buf, "Unknown error");
}

TEST(CliTextOf_all_client_codes) {
    char buf[256];
    const longword codes[] = {
        errNegotiatingPDU, errCliInvalidParams, errCliJobPending,
        errCliTooManyItems, errCliInvalidWordLen, errCliPartialDataWritten,
        errCliSizeOverPDU, errCliInvalidPlcAnswer, errCliAddressOutOfRange,
        errCliInvalidTransportSize, errCliWriteDataSizeMismatch,
        errCliItemNotAvailable, errCliInvalidValue, errCliCannotStartPLC,
        errCliAlreadyRun, errCliCannotStopPLC, errCliCannotCopyRamToRom,
        errCliCannotCompress, errCliAlreadyStop, errCliFunNotAvailable,
        errCliUploadSequenceFailed, errCliInvalidDataSizeRecvd,
        errCliInvalidBlockType, errCliInvalidBlockNumber, errCliInvalidBlockSize,
        errCliDownloadSequenceFailed, errCliInsertRefused, errCliDeleteRefused,
        errCliNeedPassword, errCliInvalidPassword, errCliNoPasswordToSetOrClear,
        errCliJobTimeout, errCliPartialDataRead, errCliBufferTooSmall,
        errCliFunctionRefused, errCliDestroying, errCliInvalidParamNumber,
        errCliCannotChangeParam,
    };
    for (longword c : codes) {
        CliTextOf(c, buf);
        CHECK(buf[0] != '\0');
    }
}

// ---------------------------------------------------------------------------
// SrvTextOf
// ---------------------------------------------------------------------------
TEST(SrvTextOf_known_codes) {
    char buf[256];
    CHECK_STREQ(SrvTextOf(0, buf), "");
    SrvTextOf(errSrvDBNullPointer, buf);
    CHECK_CONTAINS(buf, "Null passed");
    SrvTextOf(errSrvCannotStart, buf);
    CHECK_CONTAINS(buf, "cannot start");
}

TEST(SrvTextOf_all_server_codes) {
    char buf[256];
    const longword codes[] = {
        errSrvCannotStart, errSrvDBNullPointer, errSrvAreaAlreadyExists,
        errSrvUnknownArea, errSrvInvalidParams, errSrvTooManyDB,
        errSrvInvalidParamNumber, errSrvCannotChangeParam,
    };
    for (longword c : codes) {
        SrvTextOf(c, buf);
        CHECK(buf[0] != '\0');
    }
}

// ---------------------------------------------------------------------------
// ParTextOf
// ---------------------------------------------------------------------------
TEST(ParTextOf_all_partner_codes) {
    char buf[256];
    const longword codes[] = {
        errParAddressInUse, errParNoRoom, errServerNoRoom, errParInvalidParams,
        errParNotLinked, errParBusy, errParFrameTimeout, errParInvalidPDU,
        errParSendTimeout, errParRecvTimeout, errParSendRefused,
        errParNegotiatingPDU, errParSendingBlock, errParRecvingBlock,
        errParBindError, errParDestroying, errParInvalidParamNumber,
        errParCannotChangeParam, errParBufferTooSmall,
    };
    for (longword c : codes) {
        ParTextOf(c, buf);
        CHECK(buf[0] != '\0');
    }
}

// ---------------------------------------------------------------------------
// ErrCliText / ErrSrvText / ErrParText
// ---------------------------------------------------------------------------
TEST(ErrCliText_ok) {
    char buf[256];
    ErrCliText(0, buf, sizeof(buf));
    CHECK_STREQ(buf, "OK");
}

TEST(ErrCliText_lib_and_unknown) {
    char buf[256];
    ErrCliText(errLibInvalidParam, buf, sizeof(buf));
    CHECK_CONTAINS(buf, "Invalid param");
    ErrCliText(errLibInvalidObject, buf, sizeof(buf));
    CHECK_CONTAINS(buf, "Invalid object");
    ErrCliText(errCliJobPending, buf, sizeof(buf));
    CHECK_CONTAINS(buf, "Job pending");
}

TEST(ErrSrvText_ok_and_errors) {
    char buf[256];
    ErrSrvText(0, buf, sizeof(buf));
    CHECK_STREQ(buf, "OK");
    ErrSrvText(errSrvUnknownArea, buf, sizeof(buf));
    CHECK_CONTAINS(buf, "Unknown Area");
}

TEST(ErrParText_ok_and_errors) {
    char buf[256];
    ErrParText(0, buf, sizeof(buf));
    CHECK_STREQ(buf, "OK");
    ErrParText(errParNotLinked, buf, sizeof(buf));
    CHECK_CONTAINS(buf, "not linked");
}

// ---------------------------------------------------------------------------
// SenderText / EvtSrvText
// ---------------------------------------------------------------------------
TEST(SenderText_server_and_remote) {
    char buf[256];
    TSrvEvent ev;
    std::memset(&ev, 0, sizeof(ev));
    ev.EvtTime = time(nullptr);

    ev.EvtSender = 0;
    SenderText(ev, buf);
    CHECK_CONTAINS(buf, "Server");

    ev.EvtSender = inet_addr("127.0.0.1");
    SenderText(ev, buf);
    CHECK_CONTAINS(buf, "127.0.0.1");
}

TEST(EvtSrvText_various_codes) {
    char buf[512];
    TSrvEvent ev;
    std::memset(&ev, 0, sizeof(ev));
    ev.EvtTime = time(nullptr);

    const longword codes[] = {
        evcPDUincoming, evcDataRead, evcDataWrite, evcNegotiatePDU, evcReadSZL,
        evcClock, evcUpload, evcDownload, evcDirectory, evcSecurity, evcControl,
    };
    for (longword code : codes) {
        ev.EvtCode = code;
        ev.EvtRetCode = 0;
        EvtSrvText(ev, buf, sizeof(buf));
        CHECK(buf[0] != '\0');
    }
}

TEST(TcpServerEventText_various_subcodes) {
    char buf[512];
    TSrvEvent ev;
    std::memset(&ev, 0, sizeof(ev));
    ev.EvtTime = time(nullptr);
    ev.EvtCode = evcDataRead;

    const word subcodes[] = {
        evsUnknown, evsStartUpload, evsGetBlockList, evsStartListBoT,
        evsListBoT, evsGetBlockInfo, evsGetClock, evsSetClock, evsSetPassword,
        evsClrPassword,
    };
    for (word sc : subcodes) {
        ev.EvtRetCode = sc;
        TcpServerEventText(ev, buf);
        CHECK(buf[0] != '\0');
    }
}

// ---------------------------------------------------------------------------
// Server-event helpers (all branches)
// ---------------------------------------------------------------------------
static void init_event(TSrvEvent& ev, longword code, word ret, word p1, word p2,
                       word p3, word p4) {
    std::memset(&ev, 0, sizeof(ev));
    ev.EvtTime = time(nullptr);
    ev.EvtCode = code;
    ev.EvtRetCode = ret;
    ev.EvtParam1 = p1;
    ev.EvtParam2 = p2;
    ev.EvtParam3 = p3;
    ev.EvtParam4 = p4;
}

TEST(PDUText_all_result_codes) {
    char buf[512];
    TSrvEvent ev;
    init_event(ev, evcPDUincoming, evrFragmentRejected, 12, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "Fragment");

    init_event(ev, evcPDUincoming, evrMalformedPDU, 12, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "Malformed");

    init_event(ev, evcPDUincoming, evrSparseBytes, 12, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "sparse");

    init_event(ev, evcPDUincoming, evrCannotHandlePDU, 0, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "Cannot handle");

    init_event(ev, evcPDUincoming, evrNotImplemented, grCyclicData, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "cyclic data");

    init_event(ev, evcPDUincoming, evrNotImplemented, grProgrammer, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "programmer");

    init_event(ev, evcPDUincoming, 0x9999, 0, 0, 0, 0);
    PDUText(ev, buf);
    CHECK_CONTAINS(buf, "Unknown Return code");
}

TEST(TxtArea_all_areas) {
    char buf[128];
    TSrvEvent ev;
    const word areas[] = {S7AreaPE, S7AreaPA, S7AreaMK, S7AreaCT, S7AreaTM};
    for (word a : areas) {
        init_event(ev, evcDataRead, 0, a, 0, 0, 0);
        TxtArea(ev, buf);
        CHECK_CONTAINS(buf, "Area");
    }
    init_event(ev, evcDataRead, 0, S7AreaDB, 7, 0, 0);
    TxtArea(ev, buf);
    CHECK_CONTAINS(buf, "DB");
    CHECK_CONTAINS(buf, "7");

    init_event(ev, evcDataRead, 0, 0xFE, 9, 0, 0);
    TxtArea(ev, buf);
    CHECK_CONTAINS(buf, "Unknown area");
}

TEST(TxtStartSize_output) {
    char buf[128];
    TSrvEvent ev;
    init_event(ev, evcDataRead, 0, 0, 0, 10, 20);
    TxtStartSize(ev, buf);
    CHECK_CONTAINS(buf, "Start : 10");
    CHECK_CONTAINS(buf, "Size : 20");
}

TEST(TxtDataResult_all_codes) {
    char buf[128];
    TSrvEvent ev;
    const word codes[] = {evrNoError, evrErrException, evrErrAreaNotFound,
                          evrErrOutOfRange, evrErrOverPDU, evrErrTransportSize,
                          evrDataSizeMismatch, 0x9999};
    for (word c : codes) {
        init_event(ev, evcDataRead, c, 0, 0, 0, 0);
        TxtDataResult(ev, buf);
        CHECK(buf[0] != '\0');
    }
}

TEST(ControlText_all_control_codes) {
    char buf[128];
    const word codes[] = {CodeControlUnknown, CodeControlColdStart,
                          CodeControlWarmStart, CodeControlStop,
                          CodeControlCompress, CodeControlCpyRamRom,
                          CodeControlInsDel, 0x9999};
    for (word c : codes) {
        ControlText(c, buf);
        CHECK_CONTAINS(buf, "CPU Control request");
    }
}

TEST(ClockText_both_directions) {
    char buf[128];
    ClockText(evsGetClock, buf);
    CHECK_CONTAINS(buf, "read");
    ClockText(evsSetClock, buf);
    CHECK_CONTAINS(buf, "write");
}

TEST(ReadSZLText_ok_and_not_available) {
    char buf[128];
    TSrvEvent ev;
    init_event(ev, evcReadSZL, evrNoError, 0x0011, 0x0000, 0, 0);
    ReadSZLText(ev, buf);
    CHECK_CONTAINS(buf, "OK");

    init_event(ev, evcReadSZL, evrInvalidSZL, 0x0011, 0x0000, 0, 0);
    ReadSZLText(ev, buf);
    CHECK_CONTAINS(buf, "NOT AVAILABLE");
}

TEST(UploadDownloadText) {
    char buf[128];
    TSrvEvent ev;
    init_event(ev, evcUpload, 0, 0, 0, 0, 0);
    UploadText(ev, buf);
    CHECK_CONTAINS(buf, "upload");

    init_event(ev, evcDownload, 0, 0, 0, 0, 0);
    DownloadText(ev, buf);
    CHECK_CONTAINS(buf, "download");
}

TEST(StrBlockType_all_types) {
    char buf[128];
    const word types[] = {Block_OB, Block_DB, Block_SDB, Block_FC, Block_SFC,
                          Block_FB, Block_SFB, 0x99};
    for (word t : types) {
        StrBlockType(t, buf);
        CHECK(buf[0] != '\0');
    }
}

TEST(BlockInfoText_all_subfunctions) {
    char buf[128];
    TSrvEvent ev;
    const word subs[] = {evsGetBlockList, evsStartListBoT, evsListBoT,
                         evsGetBlockInfo};
    for (word s : subs) {
        init_event(ev, evcDirectory, evrNoError, s, Block_DB, 1, 0);
        BlockInfoText(ev, buf);
        CHECK(buf[0] != '\0');
    }
    init_event(ev, evcDirectory, evrResNotFound, evsGetBlockList, 0, 0, 0);
    BlockInfoText(ev, buf);
    CHECK_CONTAINS(buf, "NOT AVAILABLE");
}

TEST(SecurityText_all_subfunctions) {
    char buf[128];
    TSrvEvent ev;
    init_event(ev, evcSecurity, 0, evsSetPassword, 0, 0, 0);
    SecurityText(ev, buf);
    CHECK_CONTAINS(buf, "Set session password");

    init_event(ev, evcSecurity, 0, evsClrPassword, 0, 0, 0);
    SecurityText(ev, buf);
    CHECK_CONTAINS(buf, "Clear session password");

    init_event(ev, evcSecurity, 0, 0x99, 0, 0, 0);
    SecurityText(ev, buf);
    CHECK_CONTAINS(buf, "Unknown Subfunction");
}

TEST(TcpServerEventText_all_codes) {
    char buf[512];
    TSrvEvent ev;
    const longword codes[] = {evcServerStarted, evcServerStopped,
                              evcListenerCannotStart, evcClientAdded,
                              evcClientRejected, evcClientNoRoom,
                              evcClientException, evcClientDisconnected,
                              evcClientTerminated, evcClientsDropped, 0x99999999};
    for (longword code : codes) {
        init_event(ev, code, ECONNREFUSED, 3, 0, 0, 0);
        TcpServerEventText(ev, buf);
        CHECK(buf[0] != '\0');
    }
}
