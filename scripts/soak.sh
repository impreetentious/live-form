#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
hours="${1:-}"
if [[ ! "$hours" =~ ^[1-9][0-9]*$ ]]; then
  echo "usage: soak.sh HOURS" >&2
  exit 2
fi
seconds=$((hours * 3600))
echo "E0299: soak is not implemented (requested ${hours}h / ${seconds}s)" >&2
exit 1
