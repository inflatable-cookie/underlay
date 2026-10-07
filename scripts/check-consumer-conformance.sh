#!/usr/bin/env bash
# Backward-compatible checkout entry point. The package-owned checker is the
# canonical implementation used by both this path and the published bin.
set -uo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
exec bash "$SCRIPT_DIR/../ts/bin/underlay-consumer-security.sh" "$@"
