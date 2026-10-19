// Socket error-path tests (snap_msgsock.cpp): connection failures and
// bind conflicts.

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

struct ServerGuard {
    S7Object h;
    ServerGuard() : h(Srv_Create()) {}
    ~ServerGuard() {
        if (h) Srv_Destroy(h);
    }
};
}  // namespace

TEST(socket_connect_to_closed_port_fails) {
    ClientGuard c;
    word port = 59999;  // nothing listens here
    CHECK_EQ(Cli_SetParam(c.h, p_u16_RemotePort, &port), 0);
    CHECK_NE(Cli_ConnectTo(c.h, "127.0.0.1", 0, 2), 0);
}

TEST(socket_connect_to_unreachable_address_fails) {
    ClientGuard c;
    word port = 4102;
    CHECK_EQ(Cli_SetParam(c.h, p_u16_RemotePort, &port), 0);
    // 192.0.2.1 is TEST-NET-1 (RFC 5737): reserved and non-routable, so the
    // non-blocking connect times out (PingTimeout) instead of hanging.
    CHECK_NE(Cli_ConnectTo(c.h, "192.0.2.1", 0, 2), 0);
}

TEST(socket_server_bind_conflict) {
    ServerGuard s1;
    word port = 4102;
    CHECK_EQ(Srv_SetParam(s1.h, p_u16_LocalPort, &port), 0);
    CHECK_EQ(Srv_StartTo(s1.h, "127.0.0.1"), 0);

    ServerGuard s2;
    CHECK_EQ(Srv_SetParam(s2.h, p_u16_LocalPort, &port), 0);
    // The second server cannot bind the same port.
    CHECK_NE(Srv_StartTo(s2.h, "127.0.0.1"), 0);

    CHECK_EQ(Srv_Stop(s1.h), 0);
}

TEST(socket_write_after_disconnect_fails) {
    ServerGuard s;
    word port = 4102;
    CHECK_EQ(Srv_SetParam(s.h, p_u16_LocalPort, &port), 0);
    CHECK_EQ(Srv_StartTo(s.h, "127.0.0.1"), 0);

    ClientGuard c;
    CHECK_EQ(Cli_SetParam(c.h, p_u16_RemotePort, &port), 0);
    CHECK_EQ(Cli_ConnectTo(c.h, "127.0.0.1", 0, 2), 0);

    CHECK_EQ(Cli_Disconnect(c.h), 0);
    // Writing on a disconnected socket fails at the send layer.
    char buf[8] = {0};
    CHECK_NE(Cli_DBWrite(c.h, 1, 0, 8, buf), 0);

    CHECK_EQ(Srv_Stop(s.h), 0);
}
