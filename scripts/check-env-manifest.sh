#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 || $# -gt 4 ]]; then
  echo "Usage: $0 <app-root> [manifest-file] [required-keys-file|env-file] [env-file required-keys-file]" >&2
  exit 2
fi

APP_ROOT="$1"
MANIFEST_FILE="${2:-$APP_ROOT/config/env-manifest.txt}"
THIRD_ARG="${3:-}"
FOURTH_ARG="${4:-}"

ENV_FILE="$APP_ROOT/.env"
REQUIRED_FILE="$APP_ROOT/config/required-secrets.txt"

if [[ -n "$FOURTH_ARG" ]]; then
  ENV_FILE="$THIRD_ARG"
  REQUIRED_FILE="$FOURTH_ARG"
elif [[ -n "$THIRD_ARG" ]]; then
  case "$(basename "$THIRD_ARG")" in
    .env|*.env|*.env.*)
      ENV_FILE="$THIRD_ARG"
      ;;
    *)
      REQUIRED_FILE="$THIRD_ARG"
      ;;
  esac
fi

if [[ ! -f "$MANIFEST_FILE" ]]; then
  echo "error: env manifest not found: $MANIFEST_FILE" >&2
  exit 2
fi

extract_env_keys() {
  local file="$1"
  awk -F= '/^[A-Za-z_][A-Za-z0-9_]*=/{print $1}' "$file" | sort -u
}

extract_manifest_keys() {
  local file="$1"
  awk '
    /^[[:space:]]*#/ { next }
    /^[[:space:]]*$/ { next }
    /^[A-Za-z_][A-Za-z0-9_]*$/ { print $1 }
  ' "$file" | sort -u
}

tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT

extract_manifest_keys "$MANIFEST_FILE" > "$tmp_dir/manifest.keys"

if [[ -f "$ENV_FILE" ]]; then
  extract_env_keys "$ENV_FILE" > "$tmp_dir/env.keys"
  comm -13 "$tmp_dir/manifest.keys" "$tmp_dir/env.keys" > "$tmp_dir/unknown.keys" || true

  if [[ -s "$tmp_dir/unknown.keys" ]]; then
    echo "Unknown keys in $ENV_FILE (not declared in $MANIFEST_FILE):" >&2
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
