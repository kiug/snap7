#!/usr/bin/env bash
#
# Measures test coverage for the Snap7 Rust test suite.
#
# The library under test is C++ (compiled by build.rs), so the meaningful
# metric is C++ source coverage, produced here with gcov. A second, optional
# pass measures the Rust binding/test code with LLVM source-based coverage.
#
# Usage:
#   ./coverage.sh            # C++ coverage via gcov (requires gcc/gcov)
#   ./coverage.sh --rust     # Rust coverage via llvm-cov (requires llvm-tools)
#
set -euo pipefail

cd "$(dirname "$0")"

MODE="${1:---cpp}"

if [[ "$MODE" == "--cpp" ]]; then
    echo "==> Building and running tests with C++ coverage enabled"
    SNAP7_COVERAGE=1 cargo test --quiet

    # The C++ library is compiled by this crate's build script, so its objects
    # and .gcno/.gcda files live in the crate's build output directory.
    OUT_DIR="$(ls -d target/debug/build/snap7-*/out 2>/dev/null | head -n1)"
    echo "==> Build output: $OUT_DIR"

    COV_DIR="target/cov-cpp"
    rm -rf "$COV_DIR"
    mkdir -p "$COV_DIR"

    # Run gcov from OUT_DIR (so the .gcno/.gcda basenames resolve) for each
    # compiled translation unit.
    (cd "$OUT_DIR" && for obj in *.o; do
        gcov -b -c "$obj" > /dev/null 2>&1 || true
    done)

    # Collect the generated .gcov files.
    mv -f "$OUT_DIR"/*.gcov "$COV_DIR/" 2>/dev/null || true

    # Summarize line coverage across all .gcov files.
    total_lines=0
    covered_lines=0
    while IFS= read -r -d '' f; do
        # Parse gcov's per-line counts. The count field is the first ':'-
        # delimited field: a positive integer (covered), '#####' (executable
        # but never executed), or '-' (non-executable).
        l="$(awk -F: '
            {
                n = $1;
                gsub(/^[ \t]+|[ \t]+$/, "", n);
                if (n ~ /^[0-9]+$/) { t++; if (n + 0 > 0) c++; }
                else if (n == "#####") { t++; }
            }
            END { printf "%d %d", c + 0, t + 0 }
        ' "$f")"
        c="${l%% *}"; t="${l##* }"
        covered_lines=$((covered_lines + c))
        total_lines=$((total_lines + t))
    done < <(find "$COV_DIR" -name '*.gcov' -print0)

    echo ""
    echo "================ C++ COVERAGE (gcov) ================"
    echo "  covered lines: $covered_lines"
    echo "  total lines:   $total_lines"
    if [[ "$total_lines" -gt 0 ]]; then
        pct="$(awk -v c="$covered_lines" -v t="$total_lines" 'BEGIN{printf "%.2f", 100*c/t}')"
        echo "  line coverage: $pct%"
    fi
    echo "  per-file report: $COV_DIR/*.gcov"
    echo "====================================================="
elif [[ "$MODE" == "--rust" ]]; then
    # Locate the LLVM tools shipped with the active Rust toolchain.
    SYSROOT="$(rustc --print sysroot)"
    HOST="$(rustc -vV | sed -n 's/^host: //p')"
    LLVM_BIN="$SYSROOT/lib/rustlib/$HOST/bin"
    PROFDATA="$LLVM_BIN/llvm-profdata"
    LLVM_COV="$LLVM_BIN/llvm-cov"
    if [[ ! -x "$PROFDATA" || ! -x "$LLVM_COV" ]]; then
        echo "llvm-tools-preview not installed; run: rustup component add llvm-tools-preview" >&2
        exit 1
    fi

    echo "==> Building and running tests with Rust coverage enabled"
    PROFRAW_DIR="target/cov-rust"
    rm -rf "$PROFRAW_DIR"
    mkdir -p "$PROFRAW_DIR"
    RUSTFLAGS="-C instrument-coverage" \
        LLVM_PROFILE_FILE="$PROFRAW_DIR/%p-%m.profraw" cargo test --quiet

    echo "==> Merging and reporting with llvm-cov"
    PROFRAW="$(find "$PROFRAW_DIR" -name '*.profraw' -print | tr '\n' ' ')"
    "$PROFDATA" merge -sparse $PROFRAW -o "$PROFRAW_DIR/coverage.profdata"

    # Report on every instrumented test binary (unit + integration tests) so
    # that the wrapper code exercised through the public API is counted.
    OBJECTS=()
    while IFS= read -r bin; do
        OBJECTS+=(--object "$bin")
    done < <(find target/debug/deps -maxdepth 1 -type f -executable \
        ! -name 'build_script*' ! -name '*.d' | sort)
    "$LLVM_COV" report \
        --ignore-filename-regex='/.cargo/|/rustc/' \
        --instr-profile "$PROFRAW_DIR/coverage.profdata" \
        "${OBJECTS[@]}"
else
    echo "unknown mode: $MODE (use --cpp or --rust)" >&2
    exit 1
fi
