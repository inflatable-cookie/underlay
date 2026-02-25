#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 4 ]]; then
  echo "Usage: $0 <app-root> [env-file] [example-file] [required-keys-file]" >&2
  exit 2
fi

APP_ROOT="$1"
ENV_FILE="${2:-$APP_ROOT/.env}"
EXAMPLE_FILE="${3:-$APP_ROOT/.env.example}"
REQUIRED_FILE="${4:-$APP_ROOT/config/required-secrets.txt}"

if [[ ! -f "$EXAMPLE_FILE" ]]; then
  echo "error: .env.example not found: $EXAMPLE_FILE" >&2
  exit 2
fi

extract_keys() {
  local file="$1"
  awk -F= '/^[A-Za-z_][A-Za-z0-9_]*=/{print $1}' "$file" | sort -u
}

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

extract_keys "$EXAMPLE_FILE" > "$tmp_dir/example.keys"

if [[ -f "$ENV_FILE" ]]; then
  extract_keys "$ENV_FILE" > "$tmp_dir/env.keys"
  comm -13 "$tmp_dir/example.keys" "$tmp_dir/env.keys" > "$tmp_dir/unknown.keys" || true

  if [[ -s "$tmp_dir/unknown.keys" ]]; then
    echo "Unknown keys in $ENV_FILE (not declared in $EXAMPLE_FILE):" >&2
    cat "$tmp_dir/unknown.keys" >&2
    exit 1
  fi
fi

if [[ -f "$REQUIRED_FILE" ]]; then
  missing=()
  while IFS= read -r key; do
    key="${key%%#*}"
    key="$(echo "$key" | sed 's/^[[:space:]]*//; s/[[:space:]]*$//')"
    [[ -z "$key" ]] && continue

    value=""
    if [[ -f "$ENV_FILE" ]]; then
      value="$(awk -F= -v k="$key" '$1==k{print substr($0,index($0,"=")+1)}' "$ENV_FILE" | tail -n 1)"
    fi
    if [[ -z "$value" ]]; then
      value="${!key:-}"
    fi

    if [[ -z "$value" ]]; then
      missing+=("$key")
    fi
  done < "$REQUIRED_FILE"

  if ((${#missing[@]} > 0)); then
    echo "Missing required secrets/runtime keys (from $REQUIRED_FILE):" >&2
    printf '%s\n' "${missing[@]}" >&2
    exit 1
  fi
fi

echo "OK: env manifest check passed for $APP_ROOT"
