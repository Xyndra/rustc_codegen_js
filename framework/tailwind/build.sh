#!/usr/bin/env bash
set -euo pipefail
DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$DIR"
if [ ! -d node_modules ]; then
  bun install
fi
bun run build
echo "wrote $DIR/styles-full.css"
