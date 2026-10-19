// Server lifecycle, parameter and error-path tests (no client required).

#include "snap7_libmain.h"
#include "test.h"

namespace {
const word TEST_PORT = 4102;

struct ServerGuard {
    S7Object h;
    ServerGuard() : h(Srv_Create()) {}
    ~ServerGuard() {
        if (h) Srv_Destroy(h);
    }
};
}  // namespace

TEST(server_create_and_destroy) {
    S7Object s = Srv_Create();
    CHECK_NE(s, (S7Object)0);
    Srv_Destroy(s);
}

TEST(server_destroy_null_is_noop) {
    S7Object s = 0;
    Srv_Destroy(s);
    CHECK_EQ(s, (S7Object)0);
}

TEST(server_null_object_errors) {
    int value = 0;
    char buf[4] = {0};
    CHECK_EQ(Srv_GetParam(0, p_u16_LocalPort, &value), errLibInvalidObject);
    CHECK_EQ(Srv_SetParam(0, p_u16_LocalPort, &value), errLibInvalidObject);
    CHECK_EQ(Srv_Start(0), errLibInvalidObject);
    CHECK_EQ(Srv_StartTo(0, "127.0.0.1"), errLibInvalidObject);
    CHECK_EQ(Srv_Stop(0), errLibInvalidObject);
    CHECK_EQ(Srv_RegisterArea(0, srvAreaDB, 1, buf, 4), errLibInvalidObject);
    CHECK_EQ(Srv_UnregisterArea(0, srvAreaDB, 1), errLibInvalidObject);
    CHECK_EQ(Srv_LockArea(0, srvAreaDB, 1), errLibInvalidObject);
    CHECK_EQ(Srv_UnlockArea(0, srvAreaDB, 1), errLibInvalidObject);
    CHECK_EQ(Srv_ClearEvents(0), errLibInvalidObject);
    CHECK_EQ(Srv_SetCpuStatus(0, S7CpuStatusRun), errLibInvalidObject);

    TSrvEvent event;
    std::memset(&event, 0, sizeof(event));
    longword mask = 0;
    int ready = 0;
    CHECK_EQ(Srv_PickEvent(0, &event, ready), errLibInvalidObject);
    CHECK_EQ(Srv_GetMask(0, mkEvent, mask), errLibInvalidObject);
    CHECK_EQ(Srv_SetMask(0, mkEvent, 0), errLibInvalidObject);
    CHECK_EQ(Srv_SetEventsCallback(0, nullptr, nullptr), errLibInvalidObject);
    CHECK_EQ(Srv_SetReadEventsCallback(0, nullptr, nullptr), errLibInvalidObject);
    CHECK_EQ(Srv_SetRWAreaCallback(0, nullptr, nullptr), errLibInvalidObject);
    CHECK_EQ(Srv_GetStatus(0, value, value, value), errLibInvalidObject);
}

TEST(server_get_param_invalid_number) {
    ServerGuard s;
    int value = 0;
    CHECK_EQ(Srv_GetParam(s.h, 9999, &value), errSrvInvalidParamNumber);
}

TEST(server_set_param_invalid_number) {
    ServerGuard s;
    int value = 0;
    CHECK_EQ(Srv_SetParam(s.h, 9999, &value), errSrvInvalidParamNumber);
}

TEST(server_local_port_round_trip) {
    ServerGuard s;
    word port = TEST_PORT;
    CHECK_EQ(Srv_SetParam(s.h, p_u16_LocalPort, &port), 0);
    word got = 0;
    CHECK_EQ(Srv_GetParam(s.h, p_u16_LocalPort, &got), 0);
    CHECK_EQ(got, port);
}

TEST(server_work_interval_and_max_clients_round_trip) {
    ServerGuard s;
    int interval = 200;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_WorkInterval, &interval), 0);
    int got = 0;
    CHECK_EQ(Srv_GetParam(s.h, p_i32_WorkInterval, &got), 0);
    CHECK_EQ(got, interval);

    int max_clients = 32;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_MaxClients, &max_clients), 0);
    CHECK_EQ(Srv_GetParam(s.h, p_i32_MaxClients, &got), 0);
    CHECK_EQ(got, max_clients);
}

TEST(server_pdu_request_param_validation) {
    ServerGuard s;
    int small = 10;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_PDURequest, &small), errSrvInvalidParams);
    int large = 0x7FFF;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_PDURequest, &large), errSrvInvalidParams);
    int valid = 480;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_PDURequest, &valid), 0);
    int zero = 0;
    CHECK_EQ(Srv_SetParam(s.h, p_i32_PDURequest, &zero), 0);
}

TEST(server_register_area_null_pointer) {
    ServerGuard s;
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaDB, 1, nullptr, 4), errSrvDBNullPointer);
}

TEST(server_register_area_unknown_area_code) {
    ServerGuard s;
    char buf[4] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, 99, 0, buf, 4), errSrvUnknownArea);
}

TEST(server_register_area_duplicate) {
    ServerGuard s;
    char buf[8] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaDB, 1, buf, 8), 0);
    char other[8] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaDB, 1, other, 8), errSrvAreaAlreadyExists);
}

TEST(server_register_system_area_duplicate) {
    ServerGuard s;
    char buf[8] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaMK, 0, buf, 8), 0);
    char other[8] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaMK, 0, other, 8), errSrvAreaAlreadyExists);
}

TEST(server_unregister_area_invalid_code) {
    ServerGuard s;
    CHECK_EQ(Srv_UnregisterArea(s.h, 99, 0), errSrvInvalidParams);
}

TEST(server_unregister_unregistered_db) {
    ServerGuard s;
    CHECK_EQ(Srv_UnregisterArea(s.h, srvAreaDB, 5), errSrvInvalidParams);
}

TEST(server_register_and_unregister_db_round_trip) {
    ServerGuard s;
    char buf[16] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaDB, 7, buf, 16), 0);
    CHECK_EQ(Srv_UnregisterArea(s.h, srvAreaDB, 7), 0);
}

TEST(server_lock_and_unlock_area) {
    ServerGuard s;
    char buf[16] = {0};
    CHECK_EQ(Srv_RegisterArea(s.h, srvAreaDB, 3, buf, 16), 0);
    CHECK_EQ(Srv_LockArea(s.h, srvAreaDB, 3), 0);
    CHECK_EQ(Srv_UnlockArea(s.h, srvAreaDB, 3), 0);
}

TEST(server_status_and_cpu_status) {
    ServerGuard s;
    int server_status = -1, cpu_status = -1, clients = -1;
    CHECK_EQ(Srv_GetStatus(s.h, server_status, cpu_status, clients), 0);
    CHECK_EQ(server_status, SrvStopped);

    CHECK_EQ(Srv_SetCpuStatus(s.h, S7CpuStatusRun), 0);
    CHECK_EQ(Srv_GetStatus(s.h, server_status, cpu_status, clients), 0);
    CHECK_EQ(cpu_status, S7CpuStatusRun);
}

TEST(server_pick_event_no_events) {
    ServerGuard s;
    TSrvEvent event;
    std::memset(&event, 0, sizeof(event));
    int ready = -1;
    CHECK_EQ(Srv_PickEvent(s.h, &event, ready), 0);
    CHECK_EQ(ready, 0);
}

TEST(server_clear_events_ok) {
    ServerGuard s;
    CHECK_EQ(Srv_ClearEvents(s.h), 0);
}

TEST(server_event_mask_round_trip) {
    ServerGuard s;
    longword mask = 0;
    CHECK_EQ(Srv_GetMask(s.h, mkEvent, mask), 0);
    longword new_mask = evcDataRead | evcDataWrite;
    CHECK_EQ(Srv_SetMask(s.h, mkEvent, new_mask), 0);
    CHECK_EQ(Srv_GetMask(s.h, mkEvent, mask), 0);
    CHECK_EQ(mask, new_mask);

    // Invalid mask kind.
    CHECK_EQ(Srv_GetMask(s.h, 99, mask), errLibInvalidParam);
    CHECK_EQ(Srv_SetMask(s.h, 99, 0), errLibInvalidParam);
}

TEST(server_start_stop_and_param_change_while_running) {
    ServerGuard s;
    word port = TEST_PORT;
    CHECK_EQ(Srv_SetParam(s.h, p_u16_LocalPort, &port), 0);
    CHECK_EQ(Srv_StartTo(s.h, "127.0.0.1"), 0);

    // Cannot change the port while running.
    CHECK_EQ(Srv_SetParam(s.h, p_u16_LocalPort, &port), errSrvCannotChangeParam);

    int server_status = -1, cpu_status = -1, clients = -1;
    CHECK_EQ(Srv_GetStatus(s.h, server_status, cpu_status, clients), 0);
    CHECK_EQ(server_status, SrvRunning);

    CHECK_EQ(Srv_Stop(s.h), 0);
}

TEST(server_start_without_address) {
    ServerGuard s;
    word port = TEST_PORT;
    CHECK_EQ(Srv_SetParam(s.h, p_u16_LocalPort, &port), 0);
    CHECK_EQ(Srv_Start(s.h), 0);
    CHECK_EQ(Srv_Stop(s.h), 0);
}

TEST(server_set_event_callbacks_ok) {
    ServerGuard s;
    CHECK_EQ(Srv_SetEventsCallback(s.h, nullptr, nullptr), 0);
    CHECK_EQ(Srv_SetReadEventsCallback(s.h, nullptr, nullptr), 0);
    CHECK_EQ(Srv_SetRWAreaCallback(s.h, nullptr, nullptr), 0);
}
