#!/bin/sh
# The full verification gate for this repository; see AGENTS.md "Definition of done".
#
# The gate runs on nightly: rustfmt.toml uses nightly-only options, so this
# script exits rather than run on anything else. No particular nightly is
# pinned — any recent one will do, however Rust was installed (rustup, Nix,
# distro package). Stable coverage is CI's job (.github/workflows/tests.yml).
# CI runs this script verbatim in its `gate` job, so this file is the single
# source of truth for what the gate is: extend it here, not in the workflow.
set -e

rustc --version | grep -q nightly || {
    echo "error: the gate requires a nightly toolchain" >&2
    echo "  rustup machines: rustup run nightly tests/test_all.sh" >&2
    exit 1
}

cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
# The docs.rs configuration (all features, docsrs cfg) must build too.
RUSTDOCFLAGS="-D warnings --cfg docsrs" cargo doc --no-deps --all-features

# Printed only if every step above succeeded; a truncated run can never be
# mistaken for a pass even when the exit code is swallowed by a pipeline.
echo "GATE PASSED (all steps completed)"
