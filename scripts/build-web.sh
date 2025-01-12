#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

cargo build --locked -p st-web --target wasm32-unknown-unknown --release

rm -rf dist
mkdir -p dist
cp web/index.html web/app.js web/heapmap.js web/histogram.js web/styles.css dist/
if [[ -d examples ]]; then
  mkdir -p dist/examples
  cp -R examples/. dist/examples/
fi
touch dist/.nojekyll

wasm_bin="target/wasm32-unknown-unknown/release/st_web.wasm"
if command -v wasm-bindgen >/dev/null && [[ -f "$wasm_bin" ]]; then
  mkdir -p web/pkg
  wasm-bindgen "$wasm_bin" --target web --out-dir web/pkg --no-typescript
  cp web/pkg/* dist/ 2>/dev/null || true
fi

(
  cd dist
  find . -type f | sort | while read -r f; do
    shasum -a 256 "$f"
  done
) > dist/SHA256SUMS

echo "artifact: $ROOT/dist"
