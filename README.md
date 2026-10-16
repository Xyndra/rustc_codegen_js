Warning: this project is primarily vibe-coded

# rustc_codegen_js

Out-of-tree `rustc` codegen backend (`libcodegen_js.so`) that compiles Rust to JavaScript/TypeScript: each crate emits an `.mjs` module with an inlined runtime and a matching `.d.ts`.

## Usage

_A full tutorial will be added once this is completed_, for now you can
look at how CI handles the examples.

## Build

Requires the pinned nightly toolchain (`rust-toolchain.toml`: `nightly-2026-09-15` with `rustc-dev`, `rust-src`, `cargo`).

```sh
cargo build          # debug backend
cargo build --release
```

The crate's `build.rs` adds the sysroot `lib/` link search needed for LLVM.

## Test

```sh
sh tests/smoke.sh   # minimal user-project build checks .mjs/.d.ts
sh tests/diff.sh    # differential corpus (native vs JS output)
```

Corpus packages live in `tests/corpus/`; each compiles with both native rustc and this backend, runs both, and diffs.
