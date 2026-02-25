#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 2 ]]; then
  echo "Usage: $0 <app-root> [allowlist-file]" >&2
  exit 2
fi

APP_ROOT="$1"
ALLOWLIST_FILE="${2:-$APP_ROOT/config/env-usage-allowlist.txt}"

if [[ ! -d "$APP_ROOT" ]]; then
  echo "error: app root not found: $APP_ROOT" >&2
  exit 2
fi

is_allowed_path() {
  local path="$1"

  if [[ ! -f "$ALLOWLIST_FILE" ]]; then
    return 1
  fi

  while IFS= read -r rule; do
    rule="${rule%%#*}"
    rule="$(echo "$rule" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
    [[ -z "$rule" ]] && continue

    if [[ "$rule" == re:* ]]; then
      local re="${rule#re:}"
      if [[ "$path" =~ $re ]]; then
        return 0
      fi
    else
      if [[ "$path" == "$rule"* ]]; then
        return 0
      fi
    fi
  done < "$ALLOWLIST_FILE"

  return 1
}

scan_matches() {
  local pattern="$1"
  rg -n --no-heading \
    -g '!**/target/**' \
    -g '!**/node_modules/**' \
    -g '!**/.svelte-kit/**' \
    -g '!**/dist/**' \
    -g '!**/.git/**' \
    "$pattern" \
    "$APP_ROOT" || true
}

violations=()

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  file="${line%%:*}"
  rel="${file#${APP_ROOT}/}"
  if ! is_allowed_path "$rel"; then
    violations+=("$line")
  fi
done < <(scan_matches '\\bstd::env::var(_os)?\\(')

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  file="${line%%:*}"
  rel="${file#${APP_ROOT}/}"
  if ! is_allowed_path "$rel"; then
    violations+=("$line")
  fi
done < <(scan_matches '\\bprocess\\.env\\.[A-Za-z_][A-Za-z0-9_]*')

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  file="${line%%:*}"
  rel="${file#${APP_ROOT}/}"
  if ! is_allowed_path "$rel"; then
    violations+=("$line")
  fi
done < <(scan_matches 'import\\.meta\\.env\\.[A-Za-z_][A-Za-z0-9_]*')

if ((${#violations[@]} > 0)); then
  echo "Found direct env access outside allowlisted bootstrap paths:" >&2
  printf '%s\n' "${violations[@]}" >&2
  echo >&2
  echo "Allowlist file: $ALLOWLIST_FILE" >&2
  echo "Add explicit allowlist entries only for config/bootstrap files." >&2
  exit 1
fi

echo "OK: env usage boundary check passed for $APP_ROOT"
