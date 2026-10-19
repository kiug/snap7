# Snap7 C++ unit tests

A dependency-free C++ unit-test suite for Snap7. It tests both the public C API
and (where the symbols have external linkage) the internal helpers, using only a
standard C++ toolchain — no GoogleTest/Catch2 required.

## Layout

| Path | Purpose |
|------|---------|
| `test.h` | Minimal test framework (`TEST`, `CHECK*` macros) |
| `test_main.cpp` | Entry point; runs every registered test sequentially |
| `test_text.cpp` | White-box tests for the error-text helpers |
| `test_client.cpp` | Client lifecycle, params and error paths |
| `test_server.cpp` | Server lifecycle, params and error paths |
| `test_integration.cpp` | Client ↔ server round trips over loopback |
| `test_partner.cpp` | Partner lifecycle, params and `BSend`/`BRecv` |
| `test_whitebox.cpp` | White-box tests of private/protected methods (`#define private public`) |
| `test_socket.cpp` | Socket error paths (connect/bind failures) |
| `Makefile` | Build, run and coverage targets |

## Build & run

```sh
cd tests
make            # build build/snap7_tests
make test       # build and run
make clean      # remove build artifacts
```

Requirements: `g++` and `make`.

Before running the tests, grant the test binary permission to bind the
privileged port 102 (required by the partner `BSend`/`BRecv` round trip):

```sh
sudo setcap 'cap_net_bind_service=+ep' build/snap7_tests
```

> **Important:** `setcap` stores the capability as a file extended attribute.
> Any rebuild (`make`, `make cov`, `make clean`) deletes and recreates the
> binary, which removes the capability. Therefore build **first**, then apply
> `setcap`, then run the binary directly:
>
> ```sh
> make clean && make                       # 1. build (no capability yet)
> sudo setcap 'cap_net_bind_service=+ep' build/snap7_tests
> ./build/snap7_tests                       # 2. run with the capability
> ```
>
> For a coverage run, use the split coverage targets so the capability is
> applied **between** build and run (using `make cov` instead would rebuild
> and wipe the capability before running):
>
> ```sh
> make clean
> make cov-build                             # build with --coverage
> sudo setcap 'cap_net_bind_service=+ep' build/snap7_tests
> make cov-run                               # run the binary (keeps capability)
> make cov-report                            # generate gcov reports + summary
> ```

The tests run sequentially and use a high port (4102) for the client/server
round trips, so those run safely as an unprivileged user. The partner
`BSend`/`BRecv` round trip needs the passive partner to bind port 102
(hardcoded as `isoTcpPort`), which requires `CAP_NET_BIND_SERVICE` on Linux;
without it the test detects the bind failure and skips the network portion
gracefully while still exercising the bind-error path.

## Coverage

```sh
make cov         # build with --coverage, run, and generate gcov reports
make cov-build   # build the coverage binary (no run)
make cov-run     # run the coverage binary
make cov-report  # generate gcov reports from an already-run coverage build
make cov-html    # HTML + branch coverage (requires lcov + genhtml)
```

`make cov` rebuilds with `--coverage`, runs the suite, and generates per-file
`.gcov` reports under `build/` plus a total line-coverage summary. The summary
counts only the Snap7 library implementation (the `src/**/*.cpp` files); test
code and headers are excluded. To include the partner `BSend`/`BRecv` round
trip (which needs the privileged port 102), use the split targets described
above so the capability survives the build.

`make cov-html` produces an HTML report with branch coverage in
`build/html/index.html` (install the tools with `sudo apt install lcov` if the
message about a missing `lcov` appears).

> `make cov` and `make test` use different compile flags; run `make clean`
> between them so the object files are rebuilt with the right flags.

For a CI quality gate, enforce a line-coverage threshold with `gcovr`
(`pip install gcovr`):

```sh
gcovr --fail-under 85 --exclude 'tests/' --exclude '.*\.h'
```

### Techniques used to raise coverage

- **White-box tests** (`test_whitebox.cpp`) reach private/protected helpers via
  `#define private public` / `#define protected public`.
- **Exhaustive error-code and server-event text enumeration** (`test_text.cpp`).
- **All SZL IDs** enumerated against the simulated server
  (`test_integration.cpp`).
- **Upload and download round trips** — the simulated server implements the S7
  upload/download state machines, so `Cli_Upload`, `Cli_FullUpload` and
  `Cli_Download` (single- and multi-slice) are exercised end to end.
- **PDU fuzzing** of malformed headers (`test_integration.cpp`).
- **Async completion callback** path (`test_integration.cpp`).
- **Socket error paths** — closed port, unreachable address, bind conflict
  (`test_socket.cpp`).
- **Partner `BSend`/`BRecv` round trip** — requires `setcap` (see above).

### About coverage limits

The suite covers every exported function of the Snap7 C API, the documented
error paths, the pure helpers, and the simulated CPU's upload/download state
machines. The remaining uncovered lines are dominated by the firmware/SZL data
tables, defensive protocol branches that only a real CPU or byte-level PDU
fuzzing can trigger (PDU fragmentation, security levels, socket error
handling), and platform-specific code (Windows/Solaris). Pushing those to 100%
would be coverage theatre rather than additional value; the realistic ceiling
for Linux is ~85–90%.
