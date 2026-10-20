# Snap7 Rust test suite

Rust unit tests for the [Snap7](https://snap7.sourceforge.net/) S7
communication library. The tests drive Snap7 through the safe wrapper and the
raw FFI provided by the [`snap7`](..) crate, which is a path dependency of this
test crate.

## Layout

| Path | Purpose |
|------|---------|
| `src/lib.rs` | Crate root; re-exports the `snap7` wrapper crate |
| `src/tests/mod.rs` | Shared helpers (RAII guards, serialization lock) |
| `src/tests/client.rs` | Client lifecycle, params and error paths |
| `src/tests/server.rs` | Server lifecycle, params and error paths |
| `src/tests/integration.rs` | Client ↔ server round trips over loopback |
| `src/tests/partner.rs` | Partner lifecycle, params and `BSend`/`BRecv` |
| `src/tests/text.rs` | Error-text helpers (`Cli_ErrorText`, …) |
| `src/tests/wrapper.rs` | Smoke tests for the safe wrapper |
| `coverage.sh` | Coverage measurement (C++ via `gcov`, Rust via `llvm-cov`) |

## Requirements

- Rust toolchain (`cargo` / `rustc`)
- A C++ toolchain (`g++` and `ar`), used by the wrapper's `build.rs`

## Running the tests

```sh
cargo test
```

Network tests bind a high port (4102) and are serialized internally, so they
run safely as an unprivileged user.

> The partner `BSend`/`BRecv` round trip needs the passive partner to bind
> port 102 (hardcoded in Snap7 as `isoTcpPort`), which on Linux requires
> `CAP_NET_BIND_SERVICE`. The test detects the bind failure and skips the
> network portion gracefully while still exercising the bind-error path.

## Safe wrapper

The safe wrapper lives in the parent directory ([`..`](..)) and is exposed
here through the re-export in `src/lib.rs`:

```rust
use snap7_tests::{Area, Client, Server, ServerArea, WordLen};

let mut db = vec![0u8; 64];
let server = Server::create();
server.set_local_port(4102).unwrap();
server.register_area(ServerArea::DataBlock, 1, &mut db).unwrap();
server.start_to("0.0.0.0").unwrap();

let client = Client::create();
client.set_remote_port(4102).unwrap();
client.connect_to("127.0.0.1", 0, 2).unwrap();

client.db_write(1, 0, &[0xAA, 0xBB, 0xCC, 0xDD]).unwrap();
let mut buf = [0u8; 4];
client.db_read(1, 0, &mut buf).unwrap();
```

## Coverage

The library under test is C++, so the meaningful metric is C++ line coverage.

```sh
# C++ coverage (requires gcc/gcov)
./coverage.sh --cpp

# Rust coverage (requires the llvm-tools-preview component)
rustup component add llvm-tools-preview
./coverage.sh --rust
```

`SNAP7_COVERAGE=1` recompiles the C++ sources with `--coverage`; per-file
`.gcov` reports are written to `target/cov-cpp/`.

### About "100% coverage"

The suite exercises **every exported function** of the Snap7 C API (client,
server and partner) plus the documented error paths. However, 100% *line*
coverage of the C++ implementation is not achievable through black-box API
tests: a large share of the remaining lines are the per-error-code message
tables, the firmware/SZL data tables, and protocol branches that only a real
CPU (or byte-level PDU fuzzing) can trigger (partial transfers, PDU
fragmentation, upload/download sequences, security levels, etc.).
