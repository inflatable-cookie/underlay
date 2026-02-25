#!/usr/bin/env bash
set -euo pipefail

SONGSPROUT_NURSERY_DIR="/Users/betterthanclay/Dev/projects/songsprout/nursery"
SONGSPROUT_BLOOM_DIR="/Users/betterthanclay/Dev/projects/songsprout/bloom"
ACOWTANCY_FARMYARD_DIR="/Users/betterthanclay/Dev/projects/acowtancy/farmyard"
ACOWTANCY_DAIRY_DIR="/Users/betterthanclay/Dev/projects/acowtancy/dairy"

SONGSPROUT_API_BASE="${SONGSPROUT_API_BASE:-http://127.0.0.1:4100}"
DAIRY_API_BASE="${DAIRY_API_BASE:-http://localhost:40001}"
SKIP_GOOGLE_OAUTH_CHECKS="${SKIP_GOOGLE_OAUTH_CHECKS:-0}"

PASS=0
FAIL=0

ok() {
  PASS=$((PASS + 1))
  printf '[PASS] %s\n' "$1"
}

warn() {
  FAIL=$((FAIL + 1))
  printf '[BLOCKED] %s\n' "$1"
}

file_has_nonempty_key() {
  local file="$1"
  local key="$2"
  [[ -f "$file" ]] || return 1
  # Matches KEY=value where value is non-empty and not commented
  rg -q "^${key}=.+$" "$file"
}

check_key_group() {
  local file="$1"
  local label="$2"
  shift 2
  local missing=()

  for key in "$@"; do
    if ! file_has_nonempty_key "$file" "$key"; then
      missing+=("$key")
    fi
  done

  if [[ ${#missing[@]} -eq 0 ]]; then
    ok "$label configured in $(basename "$file")"
  else
    warn "$label missing keys in $(basename "$file"): ${missing[*]}"
  fi
}

http_check() {
  local label="$1"
  local url="$2"
  local code
  code="$(curl -sS -o /dev/null -w "%{http_code}" --max-time 3 "$url" || true)"
  if [[ "$code" =~ ^(200|401|403|404|405|422)$ ]]; then
    ok "$label reachable ($code)"
  else
    warn "$label unreachable (status=$code)"
  fi
}

printf 'Auth Live E2E Readiness\n'
printf 'Date: %s\n\n' "$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
if [[ "$SKIP_GOOGLE_OAUTH_CHECKS" == "1" ]]; then
  printf 'Mode: SKIP_GOOGLE_OAUTH_CHECKS=1 (OAuth credential checks skipped)\n\n'
fi

printf '## Songsprout\n'
check_key_group \
  "$SONGSPROUT_NURSERY_DIR/.env" \
  "JWT keys" \
  AUTH_JWT_PRIVATE_KEY AUTH_JWT_PUBLIC_KEY
check_key_group \
  "$SONGSPROUT_NURSERY_DIR/.env" \
  "WebAuthn RP config" \
  WEBAUTHN_RP_ID WEBAUTHN_RP_ORIGIN WEBAUTHN_RP_NAME
if [[ "$SKIP_GOOGLE_OAUTH_CHECKS" == "1" ]]; then
  ok "Google OAuth config check skipped for Songsprout"
else
  check_key_group \
    "$SONGSPROUT_NURSERY_DIR/.env" \
    "Google OAuth config" \
    AUTH_GOOGLE_CLIENT_ID AUTH_GOOGLE_CLIENT_SECRET AUTH_GOOGLE_REDIRECT_URI
fi

if [[ -f "$SONGSPROUT_BLOOM_DIR/src/routes/(auth)/+layout.svelte" ]]; then
  ok "Auth route layout present in Songsprout bloom"
else
  warn "Auth route layout missing in Songsprout bloom"
fi

http_check "Songsprout API auth login route" "$SONGSPROUT_API_BASE/v1/auth/login"
if [[ "$SKIP_GOOGLE_OAUTH_CHECKS" == "1" ]]; then
  ok "Songsprout API oauth route check skipped"
else
  http_check "Songsprout API oauth start route" "$SONGSPROUT_API_BASE/v1/auth/oauth/google/start"
fi

printf '\n## Acowtancy Dairy/Farmyard\n'
check_key_group \
  "$ACOWTANCY_FARMYARD_DIR/.env" \
  "JWT keys" \
  AUTH_JWT_PRIVATE_KEY AUTH_JWT_PUBLIC_KEY
check_key_group \
  "$ACOWTANCY_FARMYARD_DIR/.env" \
  "WebAuthn RP config" \
  WEBAUTHN_RP_ID WEBAUTHN_RP_ORIGIN WEBAUTHN_RP_NAME
if [[ "$SKIP_GOOGLE_OAUTH_CHECKS" == "1" ]]; then
  ok "Google OAuth config check skipped for Acowtancy"
else
  check_key_group \
    "$ACOWTANCY_FARMYARD_DIR/.env" \
    "Google OAuth config" \
    AUTH_GOOGLE_CLIENT_ID AUTH_GOOGLE_CLIENT_SECRET AUTH_GOOGLE_REDIRECT_URI
fi

if [[ -f "$ACOWTANCY_DAIRY_DIR/tests/auth-login-page.test.ts" ]]; then
  ok "Dairy browser-path auth test present"
else
  warn "Dairy browser-path auth test missing"
fi

http_check "Dairy API auth login route" "$DAIRY_API_BASE/v1/auth/login"
if [[ "$SKIP_GOOGLE_OAUTH_CHECKS" == "1" ]]; then
  ok "Dairy API oauth route check skipped"
else
  http_check "Dairy API oauth start route" "$DAIRY_API_BASE/v1/auth/oauth/google/start"
fi

printf '\nSummary: %s pass, %s blocked\n' "$PASS" "$FAIL"

if [[ "$FAIL" -gt 0 ]]; then
  exit 2
fi
