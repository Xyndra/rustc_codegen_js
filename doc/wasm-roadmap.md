# WASM backend compatibility roadmap

Pragmatic plan for making `rustc_codegen_js` interoperable with Rust’s
wasm backend (`wasm32-*`) without forcing a full in-house WebAssembly
codegen. JS remains the primary output; Wasm is a complementary target
reached through a shared boundary ABI plus runtime bridges.

## Current baseline

-  Host compiler is a nightly codegen backend (`libcodegen_js.so`) selected
  via `cargo` `codegen-backend`
- Output is `.mjs` + `.d.ts` bundles plus an inlined runtime; values use JS
  representations (`bool`/small ints as JS number, `i64/i128` as `BigInt`,
  strings as JS strings, ADTs as arrays/`{t,v}`).
- Boundary model: `#[js::export]` / `#[js::import]` with `boundary_ts()` in
  `src/lib.rs`; imports resolve to `globalThis.__js_imports.<name>` or a
  direct JS path.
- Targets today: `x86_64-unknown-linux-gnu` only (`rust-toolchain.toml`).
- No linear memory model, no Wasm MVP ABI, no JSPI, no Wasm GC types.

## Goals

1. Let a JS bundle produced by this backend call into (and be called from)
   a `wasm32` module built with `rustc`’s wasm backend in the same app.
2. Share primitive/string/boundary conventions where possible.
3. Keep changes incremental: do not rewrite the JS backend into a wasm backend.

## Non-goals (initially)

- Emitting raw Wasm directly from MIR here.
- Full `std`/`core` portability on wasm from this repo.
- Threaded/SharedArrayBuffer runtimes; start single-threaded.

## Milestones

### M0: Baseline + dual-target smoke (1–2 days)

- Add `wasm32-unknown-unknown` (and optionally
  `wasm32-wasip1`/`wasi`) to the toolchain components in
  `rust-toolchain.toml`.
- Add one `ports/wasm-interop` (or `dgs-wasm`) smoke crate that compiles a
  native target with this backend and imports a wasm module.
- Add a reproducible browser test harness (extend `ports/dgs-playground` or
  `tests/smoke.sh`) that loads both `.mjs` and `.wasm` in one page.

Exit: a browser page calls a function exported by each side and gets correct
numbers/strings.

### M1: Boundary ABI alignment (1 week)

Make the shared JS<->Rust boundary types compatible with wasm-bindgen-style
conventions while keeping current JS semantics.

- Document exact representations in `doc/limits.md` / new section here:
    - integers up to 32 bits as JS `number`, 64/128 as `BigInt`;
    - `&str`/`String` as JS string; slices as JS `Array`;
    - structs/tuples as JS arrays; enums as `{t,v}`.
- Define a small `boundary.js` adapter layer for wasm interop:
    - `jsString <-> wasm string` (length + pointer into linear memory or
      built-in `WebAssembly.Memory`).
    - number/BigInt conversions and checked truncation.
    - ADT array <-> linear-memory struct layouts.
- Keep `boundary_ts()` (`src/lib.rs`) as JS-facing truth; add wasm-side
  type mapping docs rather than changing all core types.
- Add opt-in `#[js::import = "..."]` paths that can reference wasm exports.

Exit: one struct and one enum round-trip across JS<->wasm in tests.

### M2: Host imports / exports (1 week)

- Standardize exported symbols:
    - JS side exports through `globalThis.__js_imports` (already).
    - Wasm side exports through a stable namespace (e.g.
      `globalThis.__wasm_imports`) or direct wasm imports.
- Add a resolver that, at runtime, wires JS exports as wasm imports:
    - function imports with the M1 adapter.
    - memory import (`env.memory` or shared `WebAssembly.Memory`).
- Support calling async host functions from wasm via JSPI (see M3), not via
  blocking.
- Keep browser and Node/Bun paths both working; detect available memory.

Files: runtime preamble (`src/runtime.js`), assembly in `assemble_js()`
(`src/lib.rs`), plus new adapter in `ports/*`.

### M3: JSPI (async host calls) (1–2 weeks)

JSPI lets wasm suspend while calling asynchronous JS functions. JS backend
already has sync host calls; wasm interop can use JSPI for Rust `async fn`
or blocking host calls only when needed.

- Add JS-side async export/import shims:
    - Mark imported funcs as suspending in wasm imports
      (`WebAssembly.Suspending` or JSPI transform pipeline).
    - Ensure wrappers return promises and unwrap values/panics properly.
- Define panic propagation across the boundary (JS `RustPanic` / wasm trap).
- Document required runtime support (Chrome/Node versions; polyfill limits).

Exit: one wasm module awaits a JS async function via JSPI and resumes.

### M4: Memory bridge (1 week)

Current JS backend has no explicit linear memory; values are JS objects.
Wasm uses linear memory. Add a bridge instead of forcing one model:

- Optional shared `WebAssembly.Memory` imported by both sides.
- Helpers to allocate/free JS-visible views into wasm memory
  (TextEncoder/decoder already exist in `src/runtime.js`).
- Decide ownership: wasm owns allocations passed to JS; JS may allocate
  transient buffers.
- Do not require JS backend to switch to linear memory; keep JS values native.

Exit: wasm returns a string and JS reads it through the adapter correctly.

### M5: GC integration (later, 2–3 weeks)

Wasm GC gives structs/arrays/functions as GC types instead of linear memory.
Evaluate against current JS representations (which already use JS objects).

- Option A: keep JS-native representations and only adapt at boundary.
- Option B: experiment with Wasm GC for wasm-only data, via wasm backend
  features (`reference-types`), not in this backend.
- Add docs if/when using `externref`/`anyref` in interop shims.
- Avoid mixing GC and linear-memory assumptions in one ABI without a clear owner.

Exit: a decision record in this file (update as results arrive).

### M6: Sysroot / std support for wasm targets (2+ weeks)

`std` on wasm requires sysroot components and target support. Scope:

- Toolchain: install `rust-src`, wasm targets; build `core`/`alloc`/`std`
  as needed per target.
- Runtime integration: time, randomness, env/args, panic hooks.
- Keep host crates that use unsupported features (`fs`, `net`, `thread`) as
  JS-hosted modules, imported via `#[js::import]`.
- Provide documented profiles (example: `wasm32-unknown-unknown + js host`).

Exit: a small `std`-using wasm crate runs in the browser with this repo’s
JS side as host.

## Browser/runtime considerations

- Security: no eval of untrusted wasm; validate imports.
- CSP: avoid `unsafe-eval`; bundles are static modules.
- Loading order: wasm + JS bundles; resolve imports after both instantiated.
- Memory limits and SAB availability; default to non-shared.
- Debugging: keep sourcemaps/panic messages; maintain `RustPanic`.
- Size: minify pipeline (already in playground write-up) applies to JS;
  compress wasm separately.

## Risks

- Boundary ABI drift between JS values and wasm memory.
- JSPI/async behavior differences across browsers.
- Sysroot churn with nightly toolchain.
- Maintaining two (JS object vs wasm) mental models.

## Implementation starting points

- `rust-toolchain.toml`, `ports/.cargo/config.toml`, `tests/smoke.sh`
- `src/lib.rs` (`boundary_ts`, `check_boundary_sig`, `is_js_import`)
- `src/runtime.js` (helpers, enc/dec)
- `src/emit.rs` (call/import lowering if direct wasm calls are needed)
- `ports/dgs-playground` and `docs/limitations.md`

## Acceptance criteria (first usable milestone)

- `cargo build` includes wasm target without breaking JS builds.
- A browser demo calls Rust (wasm) from Rust-generated JS and back.
- Documented adapter supports `i32`, `f64`, `BigInt`, `string`.
- No regression in existing JS corpus tests.
