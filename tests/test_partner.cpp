// Partner lifecycle, parameter and error-path tests, plus a loopback
// BSend/BRecv round trip.
//
// The passive partner listener is hardcoded to port 102 (isoTcpPort), which on
// Linux requires CAP_NET_BIND_SERVICE. The round-trip test therefore degrades
// gracefully (still exercising the bind-error path) when the port cannot be
// bound.

#include <vector>

#include "snap7_libmain.h"
#include "test.h"

namespace {
struct PartnerGuard {
    S7Object h;
    explicit PartnerGuard(bool active) : h(Par_Create(active ? 1 : 0)) {}
    ~PartnerGuard() {
        if (h) Par_Destroy(h);
    }
};
}  // namespace

TEST(partner_create_and_destroy_active) {
    S7Object p = Par_Create(1);
    CHECK_NE(p, (S7Object)0);
    Par_Destroy(p);
}

TEST(partner_create_and_destroy_passive) {
    S7Object p = Par_Create(0);
    CHECK_NE(p, (S7Object)0);
    Par_Destroy(p);
}

TEST(partner_destroy_null_is_noop) {
    S7Object p = 0;
    Par_Destroy(p);
    CHECK_EQ(p, (S7Object)0);
}

TEST(partner_null_object_errors) {
    int value = 0;
    char buf[8] = {0};
    longword r_id = 0;
    int size = 0;
    CHECK_EQ(Par_GetParam(0, p_u16_RemotePort, &value), errLibInvalidObject);
    CHECK_EQ(Par_SetParam(0, p_u16_RemotePort, &value), errLibInvalidObject);
    CHECK_EQ(Par_Start(0), errLibInvalidObject);
    CHECK_EQ(Par_StartTo(0, "127.0.0.1", "127.0.0.1", 0x0100, 0x0200), errLibInvalidObject);
    CHECK_EQ(Par_Stop(0), errLibInvalidObject);
    CHECK_EQ(Par_BSend(0, 1, buf, 8), errLibInvalidObject);
    CHECK_EQ(Par_AsBSend(0, 1, buf, 8), errLibInvalidObject);
    CHECK_EQ(Par_CheckAsBSendCompletion(0, value), errLibInvalidObject);
    CHECK_EQ(Par_WaitAsBSendCompletion(0, 100), errLibInvalidObject);
    CHECK_EQ(Par_SetSendCallback(0, nullptr, nullptr), errLibInvalidObject);
    CHECK_EQ(Par_BRecv(0, r_id, buf, size, 1000), errLibInvalidObject);
    CHECK_EQ(Par_CheckAsBRecvCompletion(0, value, r_id, buf, size), errLibInvalidObject);
    CHECK_EQ(Par_SetRecvCallback(0, nullptr, nullptr), errLibInvalidObject);

    longword a = 0, b = 0, c = 0, d = 0;
    CHECK_EQ(Par_GetTimes(0, a, b), errLibInvalidObject);
    CHECK_EQ(Par_GetStats(0, a, b, c, d), errLibInvalidObject);
    CHECK_EQ(Par_GetLastError(0, value), errLibInvalidObject);
    CHECK_EQ(Par_GetStatus(0, value), errLibInvalidObject);
}

TEST(partner_get_param_invalid_number) {
    PartnerGuard p(true);
    int value = 0;
    CHECK_EQ(Par_GetParam(p.h, 9999, &value), errParInvalidParamNumber);
}

TEST(partner_set_param_invalid_number) {
    PartnerGuard p(true);
    int value = 0;
    CHECK_EQ(Par_SetParam(p.h, 9999, &value), errParInvalidParamNumber);
}

TEST(partner_params_round_trip) {
    PartnerGuard p(true);

    word port = 1234;
    CHECK_EQ(Par_SetParam(p.h, p_u16_RemotePort, &port), 0);
    word got = 0;
    CHECK_EQ(Par_GetParam(p.h, p_u16_RemotePort, &got), 0);
    CHECK_EQ(got, port);

    int send_timeout = 4000;
    CHECK_EQ(Par_SetParam(p.h, p_i32_BSendTimeout, &send_timeout), 0);
    int igot = 0;
    CHECK_EQ(Par_GetParam(p.h, p_i32_BSendTimeout, &igot), 0);
    CHECK_EQ(igot, send_timeout);

    int recv_timeout = 5000;
    CHECK_EQ(Par_SetParam(p.h, p_i32_BRecvTimeout, &recv_timeout), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_BRecvTimeout, &igot), 0);
    CHECK_EQ(igot, recv_timeout);
}

TEST(partner_all_params_round_trip) {
    PartnerGuard p(true);
    word w = 1234;
    int i = 5000;
    longword l = 12345;

    // GetParam succeeds for every valid param.
    CHECK_EQ(Par_GetParam(p.h, p_u16_LocalPort, &w), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u16_RemotePort, &w), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_PingTimeout, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_SendTimeout, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_RecvTimeout, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_WorkInterval, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u16_SrcRef, &w), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u16_DstRef, &w), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u16_SrcTSap, &w), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_PDURequest, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_BSendTimeout, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_i32_BRecvTimeout, &i), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u32_RecoveryTime, &l), 0);
    CHECK_EQ(Par_GetParam(p.h, p_u32_KeepAliveTime, &l), 0);

    // SetParam succeeds for every settable param (LocalPort is read-only).
    CHECK_EQ(Par_SetParam(p.h, p_u16_RemotePort, &w), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_PingTimeout, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_SendTimeout, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_RecvTimeout, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_WorkInterval, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u16_SrcRef, &w), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u16_DstRef, &w), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u16_SrcTSap, &w), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_PDURequest, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_BSendTimeout, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_i32_BRecvTimeout, &i), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u32_RecoveryTime, &l), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u32_KeepAliveTime, &l), 0);
    CHECK_EQ(Par_SetParam(p.h, p_u16_LocalPort, &w), errParInvalidParamNumber);
}

TEST(partner_set_callbacks_ok) {
    PartnerGuard p(true);
    CHECK_EQ(Par_SetSendCallback(p.h, nullptr, nullptr), 0);
    CHECK_EQ(Par_SetRecvCallback(p.h, nullptr, nullptr), 0);
}

TEST(partner_get_times_and_stats_initial) {
    PartnerGuard p(true);
    longword send_time = ~0u, recv_time = ~0u;
    CHECK_EQ(Par_GetTimes(p.h, send_time, recv_time), 0);
    CHECK_EQ(send_time, 0u);
    CHECK_EQ(recv_time, 0u);

    longword sent = ~0u, recv = ~0u, serr = ~0u, rerr = ~0u;
    CHECK_EQ(Par_GetStats(p.h, sent, recv, serr, rerr), 0);
    CHECK_EQ(sent, 0u);
    CHECK_EQ(recv, 0u);
    CHECK_EQ(serr, 0u);
    CHECK_EQ(rerr, 0u);
}

TEST(partner_get_last_error_initial) {
    PartnerGuard p(true);
    int last = -1;
    CHECK_EQ(Par_GetLastError(p.h, last), 0);
    CHECK_EQ(last, 0);
}

TEST(partner_get_status_stopped_initial) {
    PartnerGuard p(true);
    int status = -1;
    CHECK_EQ(Par_GetStatus(p.h, status), 0);
    CHECK_EQ(status, par_stopped);
}

TEST(partner_bsend_without_link_fails) {
    PartnerGuard p(true);
    char buf[8] = {0};
    CHECK_NE(Par_BSend(p.h, 1, buf, 8), 0);
}

TEST(partner_brecv_without_link_fails) {
    PartnerGuard p(true);
    char buf[8] = {0};
    longword r_id = 0;
    int size = 8;
    CHECK_NE(Par_BRecv(p.h, r_id, buf, size, 100), 0);
}

TEST(partner_check_completions_without_pending) {
    PartnerGuard p(true);
    int op_result = -1;
    int rc = Par_CheckAsBSendCompletion(p.h, op_result);
    CHECK(rc == JobComplete || rc == JobPending);

    longword r_id = 0;
    int size = 0;
    char buf[8] = {0};
    rc = Par_CheckAsBRecvCompletion(p.h, op_result, r_id, buf, size);
    CHECK(rc == JobComplete || rc == JobPending);
}

TEST(partner_active_start_and_stop) {
    PartnerGuard p(true);
    CHECK_EQ(Par_StartTo(p.h, "127.0.0.1", "127.0.0.1", 0x0100, 0x0200), 0);

    int status = -1;
    CHECK_EQ(Par_GetStatus(p.h, status), 0);
    CHECK_EQ(status, par_connecting);

    CHECK_EQ(Par_Stop(p.h), 0);
    CHECK_EQ(Par_GetStatus(p.h, status), 0);
    CHECK_EQ(status, par_stopped);
}

TEST(partner_bsend_brecv_round_trip) {
    PartnerGuard passive(false);
    int rc = Par_StartTo(passive.h, "127.0.0.1", "127.0.0.1", 0x0200, 0x0100);

    int status = -1;
    Par_GetStatus(passive.h, status);
    if (status == par_binderror) {
        // Cannot bind privileged port 102 as an unprivileged user.
        Par_Stop(passive.h);
        std::printf("    (skipping network round trip: cannot bind port 102)\n");
        return;
    }
    CHECK_EQ(rc, 0);

    PartnerGuard active(true);
    CHECK_EQ(Par_StartTo(active.h, "127.0.0.1", "127.0.0.1", 0x0100, 0x0200), 0);

    // Wait for the active partner to link.
    for (int i = 0; i < 200; ++i) {
        Par_GetStatus(active.h, status);
        if (status == par_linked) break;
        SysSleep(20);
    }
    CHECK_EQ(status, par_linked);

    std::vector<char> payload(64);
    for (size_t i = 0; i < payload.size(); ++i) payload[i] = (char)i;
    CHECK_EQ(Par_BSend(active.h, 0x11223344, payload.data(), (int)payload.size()), 0);

    longword r_id = 0;
    std::vector<char> out(payload.size());
    int size = (int)out.size();
    CHECK_EQ(Par_BRecv(passive.h, r_id, out.data(), size, 5000), 0);
    CHECK_EQ(r_id, 0x11223344u);
    CHECK_EQ(size, (int)payload.size());
    CHECK_EQ(out, payload);

    CHECK_EQ(Par_Stop(active.h), 0);
    CHECK_EQ(Par_Stop(passive.h), 0);
}
