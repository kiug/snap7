//! Build script that compiles the Snap7 C++ sources into a static library and
//! links it into the Rust crate.
//!
//! The library is compiled with the system `g++` / `ar` toolchain (no external
//! build dependencies required).
//!
//! Coverage of the C++ code can be enabled by setting `SNAP7_COVERAGE=1`, which
//! compiles the library with `--coverage` so that `gcov`/`lcov` can produce a
//! line coverage report for the C++ sources.

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

fn run(cmd: &mut Command, what: &str) {
    let status = cmd
        .status()
        .unwrap_or_else(|e| panic!("failed to run {what}: {e}"));
    if !status.success() {
        panic!("{what} failed with status {status}");
    }
}

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    // Locate the Snap7 source tree by walking up from the crate directory until
    // the `src/sys` directory is found. This keeps the wrapper independent of
    // its exact depth inside the repository.
    let root = manifest_dir
        .ancestors()
        .find(|p| p.join("src").join("sys").join("snap_msgsock.cpp").exists())
        .expect("could not locate the Snap7 source tree")
        .to_path_buf();
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let include_dirs = [
        root.join("src").join("sys"),
        root.join("src").join("core"),
        root.join("src").join("lib"),
    ];

    let sources: &[&str] = &[
        "src/sys/snap_msgsock.cpp",
        "src/sys/snap_sysutils.cpp",
        "src/sys/snap_tcpsrvr.cpp",
        "src/sys/snap_threads.cpp",
        "src/core/s7_client.cpp",
        "src/core/s7_isotcp.cpp",
        "src/core/s7_partner.cpp",
        "src/core/s7_peer.cpp",
        "src/core/s7_server.cpp",
        "src/core/s7_text.cpp",
        "src/core/s7_micro_client.cpp",
        "src/lib/snap7_libmain.cpp",
    ];

    let coverage = env::var("SNAP7_COVERAGE")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    // Re-run the build script when any source or header changes.
    for dir in &include_dirs {
        println!("cargo:rerun-if-changed={}", dir.display());
    }
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SNAP7_COVERAGE");

    let mut objects = Vec::with_capacity(sources.len());
    for src in sources {
        let path = root.join(src);
        let obj_name = src.replace('/', "_").replace('\\', "_") + ".o";
        let obj = out_dir.join(obj_name);

        let mut cmd = Command::new("g++");
        cmd.arg("-c").arg(&path).arg("-fPIC").arg("-o").arg(&obj);
        if coverage {
            cmd.arg("-O0").arg("-g").arg("--coverage");
        } else {
            cmd.arg("-O2");
        }
        for inc in &include_dirs {
            cmd.arg("-I").arg(inc);
        }
        run(&mut cmd, "g++");
        objects.push(obj);
    }

    // Archive the objects into a static library.
    let lib = out_dir.join("libsnap7_static.a");
    let mut ar = Command::new("ar");
    ar.arg("rcs").arg(&lib);
    for obj in &objects {
        ar.arg(obj);
    }
    run(&mut ar, "ar");

    println!("cargo:rustc-link-search=native={}", out_dir.display());
    println!("cargo:rustc-link-lib=static=snap7_static");
    println!("cargo:rustc-link-lib=pthread");
    println!("cargo:rustc-link-lib=rt");
    // C++ runtime support required because the library is written in C++.
    println!("cargo:rustc-link-lib=stdc++");
    println!("cargo:rustc-link-lib=gcc_s");
    if coverage {
        println!("cargo:rustc-link-lib=gcov");
    }

    // Help the linker find the standard C++ library headers when it needs them.
    let _ = Path::new("/usr/include");
}
