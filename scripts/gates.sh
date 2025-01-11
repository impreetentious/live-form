#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

fail() {
  echo "Invariant violation: $1" >&2
  exit 1
}

if [[ -f "$ROOT/live-form.md" ]]; then
  fail "live-form.md is not part of this repository."
fi

lib=(crates/st-syntax crates/st-bytecode crates/st-heap crates/st-vm)
for crate in "${lib[@]}"; do
  if rg -n --glob '!**/tests/**' -e '\bstd::fs\b' -e '\bstd::env\b' -e '\bstd::thread\b' -e '\bstd::net\b' \
    -e '\bInstant::now\b' -e '\bSystemTime::now\b' -e '\bprintln!\b' "$crate/src"; then
    fail "$crate must not use filesystem, env, threads, clocks, or println"
  fi
done

if rg -n --glob '!**/tests/**' -e '\bstd::collections::HashMap\b' -e '\bstd::collections::HashSet\b' crates; then
  fail "use BTreeMap/BTreeSet/Vec, not HashMap/HashSet"
fi

if [[ -f crates/st-vm/src/numeric.rs ]] && rg -n -e '\bf64::sin\b' -e '\bf64::cos\b' -e '\bf64::sqrt\b' -e '\.mul_add\b' crates/st-vm/src/numeric.rs; then
  fail "VM numeric code must call libm, not platform math"
fi

python3 - <<'PY'
import json, subprocess, sys
meta = json.loads(subprocess.check_output(["cargo", "metadata", "--format-version", "1", "--no-deps"]))
wanted = {
    "st-syntax": set(),
    "st-bytecode": {"st-syntax"},
    "st-heap": set(),
    "st-vm": {"st-syntax", "st-bytecode", "st-heap"},
    "st-cli": {"st-syntax", "st-bytecode", "st-heap", "st-vm"},
    "st-web": {"st-vm", "st-heap"},
}
by_id = {p["id"]: p for p in meta["packages"] if p["name"] in wanted}
name_of = {p["id"]: p["name"] for p in by_id.values()}
for pkg in by_id.values():
    deps = {name_of[d] for d in (dep["pkg"] if False else [])}
    deps = set()
    for dep in pkg["dependencies"]:
        if dep["name"] in wanted:
            deps.add(dep["name"])
    expected = wanted[pkg["name"]]
    extra = deps - expected
    missing = expected - deps
    if extra or missing:
        print(f"{pkg['name']}: extra={sorted(extra)} missing={sorted(missing)}", file=sys.stderr)
        sys.exit(1)
print("dag: OK")
PY

echo "gates: OK"
