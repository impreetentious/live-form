#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
bash scripts/build-web.sh
echo "http://localhost:8081/live-form/"
exec node scripts/serve-static.mjs --dir dist --port 8081 --base /live-form
