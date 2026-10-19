/*=============================================================================|
|  Snap7 C++ unit-test harness                                                |
|==============================================================================|
|  A small, dependency-free test framework: tests are registered with TEST()   |
|  and run sequentially by test_main.cpp. This keeps the suite buildable with  |
|  only a standard C++ toolchain (no GoogleTest/Catch2 required).              |
|=============================================================================*/
#ifndef SNAP7_TEST_H
#define SNAP7_TEST_H

#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <vector>

namespace snap7test {

struct TestCase {
    const char* name;
    void (*fn)();
};

inline std::vector<TestCase>& registry() {
    static std::vector<TestCase> r;
    return r;
}

struct Registrar {
    Registrar(const char* name, void (*fn)()) { registry().push_back({name, fn}); }
};

inline int run_all() {
    int passed = 0;
    for (const TestCase& t : registry()) {
        std::printf("[ RUN      ] %s\n", t.name);
        std::fflush(stdout);
        t.fn();  // CHECK failures abort the process
        std::printf("[       OK ] %s\n", t.name);
        ++passed;
    }
    std::printf("\nAll %d tests passed.\n", passed);
    return 0;
}

inline void fail(const char* file, int line, const char* expr) {
    std::fprintf(stderr, "  FAIL %s:%d: %s\n", file, line, expr);
    std::fflush(stderr);
    std::abort();
}

}  // namespace snap7test

#define TEST_CONCAT_(a, b) a##b
#define TEST_CONCAT(a, b) TEST_CONCAT_(a, b)

#define TEST(name)                                                             \
    static void TEST_CONCAT(snap7_test_fn_, name)();                           \
    static ::snap7test::Registrar TEST_CONCAT(snap7_reg_, name)(               \
        #name, TEST_CONCAT(snap7_test_fn_, name));                             \
    static void TEST_CONCAT(snap7_test_fn_, name)()

#define CHECK(cond)                                                            \
    do {                                                                       \
        if (!(cond)) ::snap7test::fail(__FILE__, __LINE__, #cond);             \
    } while (0)

#define CHECK_EQ(a, b)                                                         \
    do {                                                                       \
        if (!((a) == (b))) ::snap7test::fail(__FILE__, __LINE__, #a " == " #b); \
    } while (0)

#define CHECK_NE(a, b)                                                         \
    do {                                                                       \
        if (!((a) != (b))) ::snap7test::fail(__FILE__, __LINE__, #a " != " #b); \
    } while (0)

#define CHECK_STREQ(a, b)                                                      \
    do {                                                                       \
        if (std::strcmp((a), (b)) != 0)                                        \
            ::snap7test::fail(__FILE__, __LINE__, #a " == " #b);               \
    } while (0)

#define CHECK_CONTAINS(haystack, needle)                                       \
    do {                                                                       \
        if (std::strstr((haystack), (needle)) == nullptr)                       \
            ::snap7test::fail(__FILE__, __LINE__, #haystack " contains " #needle); \
    } while (0)

#endif  // SNAP7_TEST_H
