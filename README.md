Warning: this project is primarily vibe-coded

# rustc_codegen_js

Out-of-tree `rustc` codegen backend (`libcodegen_js.so`) that compiles Rust to JavaScript/TypeScript: each crate emits an `.mjs` module with an inlined runtime and a matching `.d.ts`.

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

## Site (JS playground)

Ports compile selected crates to JS via `ports/.cargo/config.toml` (release backend path).

```sh
cargo build --manifest-path ports/Cargo.toml -p dgs-site
```

Output: `ports/target/<profile>/dgs-playground/index.html` (plus `bundle.mjs`, `.d.ts`). The site build minifies the backend + playground bundles into one inlined script.

## Docs

- `doc/limitations.md` — supported subset, type representation, panics, imports/exports, decorators.
- `doc/wasm-roadmap.md` — wasm interop plan.

## Layout

- `src/` — backend implementation
- `ports/` — browser-targeted Rust crates + site
- `tests/` — smoke + differential harness
```
filePath: /run/media/xyndra/Samsung/Code/Rust/rustc_codegen_js/README.md
