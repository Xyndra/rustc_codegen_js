#!/bin/sh
# Differential corpus (§12): every tests/corpus/<name>/ is a cargo package
# whose build.rs compiles the corpus with the JS backend *and* with stock
# rustc, runs both, and fails the build when their output differs. The
# emitted .mjs/.d.ts (plus both outputs) are left in
# tests/corpus/target/<name>/.
#
# Usage: sh tests/diff.sh [name...]
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# 1. build the backend itself (the corpus build scripts link against it)
cargo build --manifest-path "$ROOT/Cargo.toml" 2>&1 | tail -n 1

# 2. build the corpus workspace; each build script does its own diffing
exec cargo build --manifest-path "$ROOT/tests/corpus/Cargo.toml" "$@"
