#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

THROUGH=""
if [[ "${1:-}" == "--through" ]]; then
  THROUGH="${2:-}"
  shift 2 || true
fi

args=()
if [[ -n "$THROUGH" ]]; then
  args+=(--through "$THROUGH")
fi

node scripts/check-version-coherence.mjs
node scripts/contracts.mjs --check "${args[@]}"
bash scripts/gates.sh
bash scripts/test-gates.sh
cargo fmt --all --check
cargo clippy --locked --workspace --all-targets -- -D warnings
RUSTDOCFLAGS='-D warnings' cargo doc --locked --workspace --no-deps
cargo test --locked --workspace
cargo build --locked -p st-web --target wasm32-unknown-unknown --release
npm --prefix web run check

if [[ -n "$THROUGH" && "$THROUGH" -ge 10 ]]; then
  bash scripts/build-web.sh
  npm --prefix web run e2e
fi

if [[ -f ../live-form.md ]]; then
  node scripts/docs.mjs --check
fi
