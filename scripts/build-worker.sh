#!/usr/bin/env bash
# Build the SignalGraph Python worker as a standalone executable
# that Tauri can ship as a sidecar.
#
# Usage:
#   ./scripts/build-worker.sh
#
# Produces src-tauri/binaries/signalgraph-worker
#
# Requires: python, pip, pyinstaller (pip install pyinstaller).

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WORKER_DIR="$REPO_ROOT/worker"
OUT_DIR="$REPO_ROOT/src-tauri/binaries"

mkdir -p "$OUT_DIR"

pushd "$WORKER_DIR" >/dev/null
python -m pip install --quiet pyinstaller

pyinstaller \
    --onefile \
    --name signalgraph-worker \
    --distpath "$OUT_DIR" \
    --workpath "$WORKER_DIR/build" \
    --specpath "$WORKER_DIR/build" \
    main.py

echo "worker bundled to $OUT_DIR"
popd >/dev/null
