#!/usr/bin/env bash
set -euo pipefail
FW_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
PORT="${PORT:-8099}"
cargo site build --manifest-path "$FW_ROOT/examples/demo-app/Cargo.toml"
cargo site run --no-build --manifest-path "$FW_ROOT/examples/demo-app/Cargo.toml" --port "$PORT" >/tmp/demo-app.log 2>&1 &
SRV=$!
cleanup() { kill $SRV 2>/dev/null || true; }
trap cleanup EXIT
for i in $(seq 1 50); do
  if curl -s "http://127.0.0.1:$PORT/" >/dev/null 2>&1; then break; fi
  sleep 0.2
done
URL="http://127.0.0.1:$PORT" node "$FW_ROOT/examples/demo-app/e2e/test.cjs" 2>&1 | tail -5
