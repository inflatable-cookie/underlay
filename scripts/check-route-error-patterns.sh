#!/usr/bin/env bash

set -euo pipefail

TARGET_DIR="${1:-crates/api/src/routes}"
SHOW_COMPAT="${2:-}"

if [[ ! -d "$TARGET_DIR" ]]; then
  echo "Target directory not found: $TARGET_DIR" >&2
  exit 2
fi

echo "Checking route handlers in: $TARGET_DIR"
echo

raw_status_matches="$(
  rg -n --glob '*.rs' \
    'StatusCode::(BAD_REQUEST|UNAUTHORIZED|FORBIDDEN|NOT_FOUND|METHOD_NOT_ALLOWED|CONFLICT|UNPROCESSABLE_ENTITY|TOO_MANY_REQUESTS|INTERNAL_SERVER_ERROR|NOT_IMPLEMENTED|BAD_GATEWAY|SERVICE_UNAVAILABLE|GATEWAY_TIMEOUT)\s*\.into_response\(\)' \
    "$TARGET_DIR" || true
)"
compat_matches="$(rg -n --glob '*.rs' 'error_response\(' "$TARGET_DIR" || true)"

has_failures=0

if [[ -n "$raw_status_matches" ]]; then
  has_failures=1
  echo "Non-canonical raw status error responses found:"
  echo "$raw_status_matches"
  echo
fi

if [[ -n "$compat_matches" ]]; then
  compat_count="$(printf '%s\n' "$compat_matches" | rg -c .)"
  echo "Compatibility helper usage found (migration candidates): $compat_count"
  if [[ "$SHOW_COMPAT" == "--show-compat" ]]; then
    echo "$compat_matches"
  fi
  echo
fi

if [[ $has_failures -eq 1 ]]; then
  echo "Fail: migrate raw status error responses to ApiError/ApiResult."
  exit 1
fi

echo "Pass: no raw status error responses found."
