# Snap7 Rust wrapper

A safe, idiomatic Rust wrapper for the [Snap7](https://snap7.sourceforge.net/)
S7 communication library.

Snap7 is a C++ library that exposes a flat C API. This crate provides:

- **`ffi`** — hand-written, low-level bindings for the C API (unsafe).
- **`client` / `server` / `partner`** — safe, RAII wrappers with `Result`-based
  error handling.

The Snap7 C++ sources are compiled and linked by `build.rs` (using the system
`g++` / `ar` toolchain), so no pre-built library or external dependency is
required.

## Layout

| Path | Purpose |
|------|---------|
| `build.rs` | Compiles the Snap7 C++ sources into a static library and links it |
| `src/ffi.rs` | Hand-written FFI bindings (types, constants, C functions) |
| `src/error.rs` | `Error` type and `Result` alias |
| `src/types.rs` | Safe enums and records (`Area`, `WordLen`, `BlockInfo`, …) |
| `src/client.rs` | Safe `Client` wrapper (`Cli_*`) |
| `src/server.rs` | Safe `Server` wrapper (`Srv_*`) |
| `src/partner.rs` | Safe `Partner` wrapper (`Par_*`) |
| `tests/` | Integration tests (public API + raw FFI) and shared `common/` helpers |

## Usage

Add this crate as a path dependency:

```toml
[dependencies]
snap7 = { path = "release/wrappers/rust" }
```

Then:

```rust
use snap7::{Client, Server, ServerArea};

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
assert_eq!(buf, [0xAA, 0xBB, 0xCC, 0xDD]);
```

Every fallible operation returns [`Result`](crate::error::Result); objects are
destroyed automatically on drop.

> **Note:** buffers passed to
> [`Server::register_area`](crate::server::Server::register_area) are referenced
> (not copied) by Snap7 and must remain alive until the area is unregistered or
> the server is dropped.

## Building

```sh
cargo build
```

Requirements: Rust toolchain (`cargo` / `rustc`) and a C++ toolchain (`g++`
and `ar`).

## Tests

Unit tests live in `src/` (`#[cfg(test)]`); integration tests live in
`tests/` (each file is a separate test binary using the public API).

```sh
cargo test                        # run everything (unit + integration)
cargo test --lib                  # unit tests only
cargo test --test client          # a single integration test binary
cargo test --test integration     # the client↔server round-trips
cargo test <name>                 # a single test by name
```

Network tests bind a high port (4102) and are serialized internally, so they
run safely as an unprivileged user.

> The partner `BSend`/`BRecv` round trip needs the passive partner to bind the
> privileged port 102 (hardcoded in Snap7), which on Linux requires
> `CAP_NET_BIND_SERVICE`. Without it the test detects the bind failure and
> skips the network portion gracefully. To exercise it fully, grant the
> capability to the test binary and re-run:
>
> ```sh
> cargo test --test wrapper_partner --no-run
> sudo setcap 'cap_net_bind_service=+ep' \
>     "$(find target/debug/deps -name 'wrapper_partner-*' -type f -executable)"
> cargo test --test wrapper_partner -- --nocapture
> ```
>
> The capability is stored on the binary file, so it must be re-applied after
> every rebuild (or run the suite as root).

## Coverage

The crate reports two coverage metrics:

- **C++ coverage** — the Snap7 library itself (the meaningful metric).
- **Rust coverage** — the wrapper code in `src/`.

```sh
./coverage.sh --cpp    # C++ line coverage via gcov (requires gcc/gcov)
./coverage.sh --rust   # Rust coverage via llvm-cov (requires llvm-tools-preview)
```

`--cpp` recompiles the C++ sources with `--coverage` (via `SNAP7_COVERAGE=1`),
runs the suite, and writes per-file `.gcov` reports to `target/cov-cpp/` plus a
total line-coverage summary.

`--rust` instruments the Rust code with `-C instrument-coverage` and prints an
`llvm-cov` report (per-file and total) for the `src/` wrapper code.

To install the optional Rust coverage tools:

```sh
rustup component add llvm-tools-preview
```
