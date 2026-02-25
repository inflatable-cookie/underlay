#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

DRY_RUN=0
STRICT_EXTERNAL=0
RUN_RUNTIME=0

UNDERLAY_REFERENCE_ROOT="${UNDERLAY_REFERENCE_ROOT:-$HOME/Dev/projects/underlay-reference}"
ACOWTANCY_ROOT="${ACOWTANCY_ROOT:-$HOME/Dev/projects/acowtancy}"
COMPLI_ME_ROOT="${COMPLI_ME_ROOT:-$HOME/Dev/projects/compli-me}"
SONGSPROUT_ROOT="${SONGSPROUT_ROOT:-$HOME/Dev/projects/songsprout}"

FAILURES=0
SKIPS=0

usage() {
  cat <<USAGE
Usage: scripts/roadmap-015-016-closure.sh [options]

Runs the remaining roadmap closure checks for:
  - 015 Unified Error Reporting
  - 016 JSON Naming Standardization

Options:
  --dry-run          Print commands without executing them
  --strict-external  Fail when external repo paths are missing
  --run-runtime      Also run runtime evidence checks in Acowtancy (requires running API/DB env)
  -h, --help         Show this help

Optional environment variables:
  UNDERLAY_REFERENCE_ROOT (default: $HOME/Dev/projects/underlay-reference)
  ACOWTANCY_ROOT         (default: $HOME/Dev/projects/acowtancy)
  COMPLI_ME_ROOT         (default: $HOME/Dev/projects/compli-me)
  SONGSPROUT_ROOT        (default: $HOME/Dev/projects/songsprout)
  ACOWTANCY_API_BASE_URL (required with --run-runtime)
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --dry-run)
      DRY_RUN=1
      ;;
    --strict-external)
      STRICT_EXTERNAL=1
      ;;
    --run-runtime)
      RUN_RUNTIME=1
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

run_cmd() {
  local label="$1"
  shift
  echo
  echo "==> $label"
  echo "\$ $*"
  if [[ "$DRY_RUN" -eq 1 ]]; then
    return 0
  fi

  if ! "$@"; then
    echo "FAIL: $label" >&2
    FAILURES=$((FAILURES + 1))
    return 1
  fi
}

run_cmd_in() {
  local label="$1"
  local dir="$2"
  shift 2

  if [[ ! -d "$dir" ]]; then
    if [[ "$STRICT_EXTERNAL" -eq 1 ]]; then
      echo "FAIL: missing required directory for $label: $dir" >&2
      FAILURES=$((FAILURES + 1))
      return 1
    fi
    echo "SKIP: $label (directory missing: $dir)"
    SKIPS=$((SKIPS + 1))
    return 0
  fi

  echo
  echo "==> $label"
  echo "(cd $dir && $*)"
  if [[ "$DRY_RUN" -eq 1 ]]; then
    return 0
  fi

  if ! (cd "$dir" && "$@"); then
    echo "FAIL: $label" >&2
    FAILURES=$((FAILURES + 1))
    return 1
  fi
}

run_optional() {
  local label="$1"
  shift
  if ! "$@"; then
    echo "SKIP: $label"
    SKIPS=$((SKIPS + 1))
  fi
}

# 016 guardrails (underlay + cross-repo DTO scans)
run_cmd "016: underlay JSON naming guardrail" \
  "$ROOT_DIR/scripts/check-json-naming.sh" "$ROOT_DIR/rust"

run_cmd_in "016: underlay-reference JSON naming guardrail" "$UNDERLAY_REFERENCE_ROOT" \
  "$ROOT_DIR/scripts/check-json-naming.sh" "$UNDERLAY_REFERENCE_ROOT/acme-api/crates"

run_cmd_in "016: acowtancy JSON naming guardrail" "$ACOWTANCY_ROOT" \
  "$ROOT_DIR/scripts/check-json-naming.sh" "$ACOWTANCY_ROOT/farmyard/crates" "$ROOT_DIR/scripts/json-naming-allowlist.txt"

run_cmd_in "016: compli-me JSON naming guardrail" "$COMPLI_ME_ROOT" \
  "$ROOT_DIR/scripts/check-json-naming.sh" "$COMPLI_ME_ROOT/api/crates"

run_cmd_in "016: songsprout JSON naming guardrail" "$SONGSPROUT_ROOT" \
  "$ROOT_DIR/scripts/check-json-naming.sh" "$SONGSPROUT_ROOT/nursery/crates"

run_cmd "016: compatibility sunset guardrail" \
  "$ROOT_DIR/scripts/check-compatibility-sunset.sh"

# 015 guardrails (canonical API error path checks)
run_cmd_in "015: underlay-reference route error pattern check" "$UNDERLAY_REFERENCE_ROOT" \
  "$ROOT_DIR/scripts/check-route-error-patterns.sh" "$UNDERLAY_REFERENCE_ROOT/acme-api/crates/api/src/routes"

run_cmd_in "015: acowtancy route error pattern check" "$ACOWTANCY_ROOT" \
  "$ROOT_DIR/scripts/check-route-error-patterns.sh" "$ACOWTANCY_ROOT/farmyard/crates/api/src/routes"

run_cmd_in "015: compli-me route error pattern check" "$COMPLI_ME_ROOT" \
  "$ROOT_DIR/scripts/check-route-error-patterns.sh" "$COMPLI_ME_ROOT/api/crates/api/src/routes"

run_cmd_in "015: songsprout route error pattern check" "$SONGSPROUT_ROOT" \
  "$ROOT_DIR/scripts/check-route-error-patterns.sh" "$SONGSPROUT_ROOT/nursery/crates/api/src/routes"

# Underlay auth regression baseline (supports both 004/015 quality confidence)
run_cmd "auth crates regression test sweep" \
  cargo test -p underlay-auth -p underlay-auth-password -p underlay-auth-jwt -p underlay-auth-totp -p underlay-auth-webauthn -p underlay-auth-oauth --all-features

# Optional runtime evidence (015 + 016 final closure data)
if [[ "$RUN_RUNTIME" -eq 1 ]]; then
  ACOWTANCY_API_BASE_URL="${ACOWTANCY_API_BASE_URL:-}"
  if [[ -z "$ACOWTANCY_API_BASE_URL" ]]; then
    echo "FAIL: --run-runtime requires ACOWTANCY_API_BASE_URL" >&2
    FAILURES=$((FAILURES + 1))
  elif [[ -d "$ACOWTANCY_ROOT/farmyard" ]]; then
    run_cmd_in "015 runtime: acowtancy validate-error-reporting" "$ACOWTANCY_ROOT/farmyard" \
      env API_BASE_URL="$ACOWTANCY_API_BASE_URL" bash scripts/validate-error-reporting.sh
  else
    echo "SKIP: 015 runtime checks (acowtancy farmyard not found)"
    SKIPS=$((SKIPS + 1))
  fi
fi

echo
echo "==== 015/016 Closure Summary ===="
echo "Failures: $FAILURES"
echo "Skips:    $SKIPS"

if [[ "$FAILURES" -ne 0 ]]; then
  echo "Result: FAIL"
  exit 1
fi

echo "Result: PASS"
