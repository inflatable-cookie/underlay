#!/usr/bin/env bash
set -euo pipefail

# Runs tarpaulin and enforces minimum coverage thresholds.
#
# Local prerequisites:
# - cargo-tarpaulin installed (or let this script tell you how)
#
# CI is expected to have a Docker runtime available.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if ! command -v cargo-tarpaulin >/dev/null 2>&1; then
  echo "cargo-tarpaulin not found. Install with:" >&2
  echo "  cargo install cargo-tarpaulin" >&2
  exit 1
fi

# Thresholds (adjust as the codebase matures)
MIN_OVERALL=50

echo "Running tarpaulin (workspace)…"
OUTPUT="$(cargo tarpaulin --workspace --out Xml 2>&1)" || {
  echo "$OUTPUT" >&2
  exit 1
}

# Extract overall % from the final summary line, e.g.
# "54.01% coverage, 606/1122 lines covered"
OVERALL_PCT="$(echo "$OUTPUT" | grep -Eo '^[0-9]+(\.[0-9]+)?% coverage' | tail -1 | cut -d% -f1)"

if [[ -z "$OVERALL_PCT" ]]; then
  echo "Failed to parse tarpaulin output:" >&2
  echo "$OUTPUT" >&2
  exit 1
fi

OVERALL_INT="${OVERALL_PCT%.*}"

echo "Overall coverage: ${OVERALL_PCT}% (min: ${MIN_OVERALL}%)"

if (( OVERALL_INT < MIN_OVERALL )); then
  echo "Coverage check failed: ${OVERALL_PCT}% < ${MIN_OVERALL}%" >&2
  exit 1
fi

echo "Coverage thresholds passed."