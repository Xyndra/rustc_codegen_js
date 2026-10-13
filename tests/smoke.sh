#!/bin/sh
# Bundle smoke test (.mjs/.d.ts exist and are well-formed) from a *user's*
# point of view: an ordinary cargo project that only sets
# -Zcodegen-backend. Artifacts land in target/smoke/target/.
#
# Usage: sh tests/smoke.sh   (run from repo root)
set -eu

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
FIX="$ROOT/target/smoke"
TARGET_DIR="$FIX/target"
BACKEND="$ROOT/target/debug/libcodegen_js.so"

# 1. build the backend
cargo build --manifest-path "$ROOT/Cargo.toml" 2>&1 | tail -n 2

# 2. a plain cargo project (no build.rs, nothing special)
rm -rf "$FIX"
mkdir -p "$FIX/src"
printf '[package]\nname = "smoke"\nversion = "0.1.0"\nedition = "2021"\n\n[lib]\ncrate-type = ["rlib"]\n' > "$FIX/Cargo.toml"
printf '#![feature(register_tool)]\n#![register_tool(js)]\n#[js::export]\npub fn add(a: i32, b: i32) -> i32 { a + b }\n' > "$FIX/src/lib.rs"

RUSTFLAGS="-Zcodegen-backend=$BACKEND" \
  cargo build --manifest-path "$FIX/Cargo.toml" --target-dir "$TARGET_DIR" 2>&1 | tail -n 3

JS_OUT=$(find "$TARGET_DIR" -name "smoke.mjs" | head -1)
DTS_OUT=$(find "$TARGET_DIR" -name "smoke.d.ts" | head -1)
RLIB_OUT=$(find "$TARGET_DIR" -name "libsmoke.rlib" | head -1)

[ -n "$JS_OUT" ] && [ -n "$DTS_OUT" ] && [ -n "$RLIB_OUT" ] || {
  echo "SMOKE FAIL: missing outputs (js=$JS_OUT dts=$DTS_OUT rlib=$RLIB_OUT)"
  exit 1
}
grep -q "@ts-nocheck" "$JS_OUT" || { echo "SMOKE FAIL: no @ts-nocheck in $JS_OUT"; exit 1; }
grep -q "_js_add as add" "$JS_OUT" || grep -q "export function add" "$JS_OUT" || { echo "SMOKE FAIL: no export in $JS_OUT"; exit 1; }
grep -q "export declare function add" "$DTS_OUT" || { echo "SMOKE FAIL: no d.ts entry in $DTS_OUT"; exit 1; }
node --check "$JS_OUT" 2>/dev/null || echo "(node unavailable, skipped syntax check)"

echo "SMOKE OK:"
echo "  $JS_OUT"
echo "  $DTS_OUT"
echo "  $RLIB_OUT"
