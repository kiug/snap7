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

Unit tests live in `src/` (`#[cfg(test)]`), integration tests in `tests/`:

```sh
cargo test
```

## Coverage

Setting `SNAP7_COVERAGE=1` recompiles the C++ sources with `--coverage` so that
`gcov` / `lcov` can produce a line-coverage report for the C++ implementation:

```sh
./coverage.sh --cpp
```
