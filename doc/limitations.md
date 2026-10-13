# rustc_codegen_js — limitations

rustc_codegen_js is an out-of-tree rustc codegen backend that translates
Rust → TypeScript/JavaScript. It deliberately implements a **subset** of the
Rust standard library. Below is the complete picture of what is supported,
what is not, and the semantics that differ from native Rust.

## General

- **Single-threaded target.** The backend is designed for a single-threaded
  JavaScript runtime. `std::thread`, `parking_lot`, channel primitives,
  `Mutex`/`RwLock` (from `std::sync`), and `atomic_*` intrinsics are
  rejected.
- **No linear memory, no allocator.** Values are represented by
  [JavaScript types](#type-representation) (arrays, objects, strings,
  `BigInt`, numbers, booleans, functions). There is no heap arena managed by
  the backend; owned values are reclaimed by the JS engine (garbage
  collection).
- **No `extern "C"`/"Rust" ABI interop** beyond the small set of functions
  declared with `#[js::import]` and loaded from `globalThis.__js_imports`
  at run time.
- **`#![feature(register_tool)]` + `#![register_tool(js)]`** is required in
  every user crate that uses `#[js::import]`/`#[js::export]`, because the
  `js` tool attribute is unknown to rustc.

## Type representation

| Rust type                       | JS representation                        |
|---------------------------------|------------------------------------------|
| `bool`                          | boolean                                  |
| `char` / i8–i32 / u8–u32 / usize / isize | JS number (32-bit)               |
| i64 / u64 / i128 / u128         | `BigInt`                                 |
| f32 / f64                       | JS number (`f32` narrowed with `Math.fround` at casts/stores) |
| `&str`, `str`                   | JS string                                |
| `String`                        | 1-element array `[str]` (mutable cell)   |
| `Vec<T>`, `[T]`, `[T; N]`       | JS array                                 |
| `()` / ZSTs                     | `undefined`                              |
| tuple                           | JS array                                 |
| struct / union / `Cell`         | JS array, positional fields              |
| enum                            | `{ t: discriminant, v: [...] }`          |
| fn item / closure / fn pointer  | JS function (closures take `env` first)  |
| `&T`, `&mut T`, `Box<T>`, `Rc<T>`, `Arc<T>` | transparent (inner value)   |
| `RefCell<T>` guards             | transparent (borrow flags are **not** enforced) |

### Known gaps

- Raw pointers (`*const T`, `*mut T`) are rejected at creation/deref sites.
- `dyn Trait` (dynamic dispatch) is rejected.
- Coroutines / `async`/`await` are rejected (aggregate of coroutine state is
  not emitted).
- Unions are rejected outright (their representation cannot be mapped safely).
- Types that need a custom allocator or raw memory are rejected.

## Collections and iterators

Vector, slice, `String`, `str`, `Option`, `Result`, arrays, tuples,
`Cell`/`RefCell`, `Box`/`Rc`/`Arc`, `RangeInclusive`, `Range`, and the
iterator adaptors over them are supported **via recognized shims** — no std
MIR is ever emitted for these. The following adaptors are implemented:
`map`, `filter`, `filter_map`, `flat_map`, `flatten`, `rev`, `enumerate`,
`zip`, `chain`, `take`, `skip`, `step_by`, `cloned`, `copied`, `count`,
`last`, `nth`, `position`, `find`, `any`, `all`, `fold`, `for_each`, `sum`,
`product`, `min`, `max`, `min_by`, `max_by`, `map_or`, `map_or_else`,
 `and_then`, `ok_or`, `or_else`, `unwrap_or`, `unwrap_or_else`,
 `unwrap_or_default`, `filter_map`-like `inspect`, `collect`, `eq`.
 Conversions through `From`/`Into` are recognized by trait (not by path text,
 which erases `<...>` groups): `String::from(&str/String)`, `Vec::from([T; N])`
 (including the const-generic `[T; N]` impl, whose args carry `N` as a const),
 `Vec::from(&[T])`, and reflexive `From<T> for T` (identity) are all mapped.

**Not implemented (diagnosed):** `HashMap`/`HashSet` (need a real hash-table
runtime with a `Hash` impl), `BTreeMap`/`BTreeSet` (ordered-table runtime),
`std::thread`, `std::fs`, `std::net`, `std::time`, `std::process`, `std::env`,
`std::backtrace`, SIMD intrinsics, libm, and any function that touches raw
memory (`copy_*`, `write_bytes`, volatile/unaligned intrinsics).

## Panics and unwinding

- Panics are translated into **thrown exceptions** (`throw new RustPanic(msg)`).
- `panic!()`, `panic!("{}", ...)` (format args peered at call site),
  `assert!()`, `assert_eq!()`, `assert_ne!()`, slice bounds checks,
  integer overflow checks, `Option::unwrap()`/`expect()`, `Result::unwrap()`/`expect()`
  are all supported and throw exceptions.
- `std::panic::catch_unwind()` is supported: it runs the closure inside a
  `try {...} catch {...}` block and returns `Ok(...)` / `Err(payload)`.
- The panic **message format differs** from native Rust for some forms
  (e.g. `assert!` without a custom message becomes `"assertion failed"`;
  bounds checks render their own `len`/`index` text). The payload is a string,
  not `&'static str`, so `e.to_string()`/`e.as_str()` work as expected.
- The panic payload type is `Box<dyn std::any::Any + Send>` unwrapped to a JS
  value; only the first throw value is observable.
- **Drop semantics during unwinding are not modeled**: unwind resume/terminate
  blocks are not emitted as JS, so a panic inside a destructor (or a `Drop`
  impl itself) has no faithful JS equivalent. See "Destructors" below.

## Imports / exports (JS boundary)

The boundary is opt-in per item:

- `#[js::export]` on a **free function** — generates an ES module `export`
  and a corresponding TypeScript declaration. Boundary functions are emitted
  as a type-coercing wrapper (`export function _js_<name>(...)`) plus a plain
  re-export alias (`export { _js_<name> as <name> }`), so the public name is
  the alias, not a direct `export function <name>`. Parameter/return types must be
  in the allowed set: `f64`, `i32`, `u32`, `bool`, `String`, `&str`, and
  `()` (void). Generic boundary functions are rejected.
- `#[js::import]` on a foreign function, or on its enclosing `extern "C"`
  block — declares a function drawn from `globalThis.__js_imports.<name>`
  at run time. Same type restrictions as exports.
- `String` parameters arrive from JS **wrapped in a 1-cell array**
  (`[string]`); the wrapper is transparent to Rust code. A `String` **return**
  is unwrapped at the wrapper boundary. `&str` returns are plain JS strings.

## Destructors

- **User-defined `Drop` impls are not supported.** A type with a significant
  destructor (any `impl Drop` from user code, or types whose drop requires
  layout actions) is diagnosed at the drop site with
  `"cannot drop T: user destructors are not supported"`. Types whose drops
  are purely bookkeeping (allocators, refcount ticks, fmt) are treated as
  no-ops.
- For GC-managed types (`Vec`, `String`, `Box`, our object/enum reprs) the
  drop is a **no-op**; values are reclaimed by the JS engine when no longer
  referenced. Drop *order* is LIFO in the MIR (nested scopes unwind in
  reverse construction order), but since drops don't do anything this is
  verified implicitly by correct execution after scope exit rather than by
  observing drop side effects.

## Control flow and patterns

- `if`/`else if`/`else`, `while`, `for` (over slice/vec iterators),
  `loop`/`break`/`continue`, `match` (with bindings, guards, `if let`,
  `let-else`), `Option`/`Result` combinators as shims — all supported.
- Structured emission is used when the control-flow shape is mappable
  (single-exit loops, join blocks inside the enclosing loop); otherwise a
  `bb=` state machine is emitted. The two paths are guaranteed to be
  semantically equivalent because both consume the same MIR.
- Coroutines/async are not supported (see above).

## Constants and compile-time evaluation

- MIR constants are rendered directly into JS expressions (BigInt literals
  for 64/128-bit integers, string literals, array/tuple/enum aggregates).
- `core::fmt::Arguments`/`fmt::rt::Argument` constructors are **peered at
  call sites** (`write!`, `format!`, `panic!`, `print!`); only the
  `from_str` family (used to build format templates) and their runtime
  helpers are emitted.

## Number model

- 64/128-bit integers use `BigInt` (not the wasm-style split lo/hi pairs).
- Wrapping operations (`wrapping_add`, `wrapping_sub`, `wrapping_mul`,
  `wrapping_neg`, `wrapping_shl`, `wrapping_shr`), checked operations
  (`checked_add`, `checked_sub`, `checked_mul`, `checked_div`, `checked_rem`,
  `checked_neg`), saturating operations (`saturating_add`, `saturating_sub`,
  `saturating_mul`), and overflowing operations (`overflowing_add`,
  `overflowing_sub`, `overflowing_mul`, `overflowing_neg`) are implemented using
  `BigInt` arithmetic when the operand width exceeds 32 bits, so they match
  Rust's semantics (including wrap on overflow) through 128 bits.
- Float-to-int casts use Rust-style saturation semantics (`NaN` → 0, infinities clamp to the range).

## Toolchain

- Pinned nightly `2026-09-15` (features required: `rustc_private`,
  `register_tool`). Out-of-tree backend linkage needs `libLLVM-*-rust-*.so`
  from the toolchain sysroot (`cargo:rustc-link-search` is set in `build.rs`).
- The pinned toolchain note (§1 of `src/lib.rs`) also documents that "Hook A"
  (`CodegenBackend::mir_built`) does not exist on this toolchain, so library
  recognition (§6.1) is applied at emission time, and `mir_drops_elaborated_and_const_checked`
  is provided manually so bodies are read pre-optimization.

## Corpus tests

The `tests/corpus` suite is a cargo package with one crate per topic. Each
build:

1. compiles the crate with `-Zcodegen-backend=libcodegen_js.so` (the
   generated `.mjs` + `.d.ts` go under `target/<pkg>/`);
2. compiles the same source natively (`native.rs` sidecar) and runs the
   resulting binary;
3. runs the `.mjs` under Node (default driver calls `run()`, or a custom
   `run.js`);
4. **diffs** the two outputs and fails the build on any difference.

Packages:

- `ints` — 64/128-bit ints, cross-width casts, f32.
- `arith` — basic operators, wrapping, shifts, comparisons, checked/saturating, bool logic.
- `collections` — Vec/slice, Option/Result, iterator chains, Box, Cell/RefCell, `write!`.
- `enums` — enum match/if let/let-else, derived Debug, char predicates, `matches`.
- `traits` — trait monomorphization, iterator adapters, closures, panics via `catch_unwind`,
  drop ordering (scope-exit, no-op destructors).
- `boundary` — `#[js::import]`/`#[js::export]` type boundary (imports resolved
  from `globalThis.__js_imports`, exports re-exported to JS).

## Open items (not yet implemented)

- Multi-threading, OS services, file/network/time, custom allocators.
- Raw-pointer arithmetic.
- f32 bit-exactness (f32 is narrowed with `Math.fround` at casts/stores;
  intermediate operations run in f64).
- `dyn Trait` dynamic dispatch.
- Cross-crate generic calls: these resolve by `DefId` (identity fallback), so
  a generic function from another crate is emitted once per local use rather
  than truly monomorphized per upstream instance.
- `async`/`await` / coroutines.
- `derive` beyond what lowers to recognized shims (no `PartialEq`/`Eq` trait
  vtable path; equality is structural via `__deep_eq`).
- Full `std::collections::hash::` / `btree::` families.
- Foreign imports other than the tiny accepted signature set.
- `cargo test` harness (only the custom differential harness exists).
