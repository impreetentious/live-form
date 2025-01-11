#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

need_node="22.12.0"
have_node="$(node -v 2>/dev/null | sed 's/^v//')"
if [[ "$have_node" != "$need_node" ]]; then
  echo "Need Node $need_node (see .nvmrc). Have: ${have_node:-none}" >&2
  echo "nvm use" >&2
  exit 1
fi

npm_major="$(npm -v | cut -d. -f1)"
if (( npm_major < 10 )); then
  echo "Need npm 10 or newer. Have: $(npm -v)" >&2
  exit 1
fi

if ! command -v rustup >/dev/null; then
  echo "Need rustup. Install from https://rustup.rs/" >&2
  exit 1
fi

rustup show
cargo fetch --locked

wasm_bindgen_ver="$(
  python3 - <<'PY'
import re
from pathlib import Path
text = Path("Cargo.lock").read_text()
blocks = text.split("[[package]]")
for block in blocks[1:]:
    name = re.search(r'^name = "([^"]+)"', block, re.M)
    version = re.search(r'^version = "([^"]+)"', block, re.M)
    if name and name.group(1) == "wasm-bindgen" and version:
        print(version.group(1))
        break
else:
    raise SystemExit("wasm-bindgen missing from Cargo.lock")
PY
)"
if ! command -v wasm-bindgen >/dev/null || [[ "$(wasm-bindgen --version | awk '{print $2}')" != "$wasm_bindgen_ver" ]]; then
  cargo install wasm-bindgen-cli --version "$wasm_bindgen_ver" --locked --force
fi

npm ci --prefix web
if [[ "${PLAYWRIGHT_SKIP:-}" == "1" ]]; then
  echo "PLAYWRIGHT_SKIP=1: browsers not installed (local only)"
else
  if [[ "$(uname -s)" == "Linux" ]]; then
    npx --prefix web playwright install --with-deps chromium firefox webkit
  else
    npx --prefix web playwright install chromium firefox webkit
  fi
fi

echo "node $(node -v)"
echo "npm $(npm -v)"
echo "rustc $(rustc --version)"
echo "setup: OK"
