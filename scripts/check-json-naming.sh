#!/usr/bin/env bash

set -euo pipefail

TARGET_DIR="${1:-rust}"
ALLOWLIST_FILE="${2:-}"

if [[ ! -d "$TARGET_DIR" ]]; then
  echo "Target directory not found: $TARGET_DIR" >&2
  exit 2
fi

if [[ -n "$ALLOWLIST_FILE" && ! -f "$ALLOWLIST_FILE" ]]; then
  echo "Allowlist file not found: $ALLOWLIST_FILE" >&2
  exit 2
fi

echo "Checking JSON naming in: $TARGET_DIR"
echo

rename_all_matches="$(
  rg -n --glob '*.rs' '#\s*\[\s*serde\s*\(\s*rename_all\s*=\s*"camelCase"\s*\)\s*\]' "$TARGET_DIR" || true
)"

rename_field_matches="$(
  rg -n -P --glob '*.rs' '#\s*\[\s*serde\s*\(\s*rename\s*=\s*"[^"]*[A-Z][^"]*"\s*\)\s*\]' "$TARGET_DIR" || true
)"

matches="$(
  {
    [[ -n "$rename_all_matches" ]] && printf '%s\n' "$rename_all_matches"
    [[ -n "$rename_field_matches" ]] && printf '%s\n' "$rename_field_matches"
  } | rg -n . || true
)"

if [[ -n "$ALLOWLIST_FILE" && -n "$matches" ]]; then
  matches="$(printf '%s\n' "$matches" | rg -v -f "$ALLOWLIST_FILE" || true)"
fi

if [[ -n "$matches" ]]; then
  echo "Found camelCase serde naming directives (migration candidates):"
  echo "$matches"
  echo
  echo "Fail: internal DTOs should use snake_case JSON naming."
  echo "If a third-party contract requires camelCase, add path patterns to an allowlist file and pass it as argument 2."
  exit 1
fi

echo "Pass: no camelCase serde naming directives found."
