#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

fail() {
  echo "gates self-test failed: $1" >&2
  exit 1
}

# Positive control: the working tree must pass.
bash scripts/gates.sh >/dev/null

forbidden="$ROOT/live-form.md"
trap 'rm -f "$forbidden"' EXIT
echo "forbidden" > "$forbidden"
if bash scripts/gates.sh >/dev/null 2>&1; then
  fail "expected live-form.md at repo root to fail gates"
fi
rm -f "$forbidden"
trap - EXIT

echo "gates self-test: OK"
