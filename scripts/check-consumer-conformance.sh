#!/usr/bin/env bash
#
# Underlay consumer security conformance checks.
#
# Statically verifies that a consumer repo follows the canonical Underlay
# security shapes (docs/architecture/070-consumer-drift-prevention.md).
# Run from anywhere:
#
#   ../underlay/scripts/check-consumer-conformance.sh .
#   ../underlay/scripts/check-consumer-conformance.sh /path/to/consumer
#
# Exit code 0 = all checks pass, 1 = at least one violation.
# Skip individual checks with CONFORMANCE_SKIP="check1,check2".

set -uo pipefail

ROOT="${1:-.}"
FAILURES=()
PASSES=()

SKIP=",${CONFORMANCE_SKIP:-},"

skip() {
  [[ "$SKIP" == *",$1,"* ]]
}

pass() {
  PASSES+=("$1")
}

fail() {
  FAILURES+=("$1: $2")
}

# --------------------------------------------------------------------------
# check-env-fail-closed: environment variables must not default to local/dev
# --------------------------------------------------------------------------
if ! skip "env-fail-closed"; then
  hits=$(rg -U --type rust \
    'env::var\([^)]*\)[^;]{0,300}unwrap_or(_else)?\([^)]*"(local|dev|development)"' \
    "$ROOT" -g '!target' -g '!*.md' 2>/dev/null \
    | rg -v "test|example|//" || true)
  if [[ -z "$hits" ]]; then
    pass "env-fail-closed"
  else
    fail "env-fail-closed" "environment defaults to local/dev (fail open):\n$hits"
  fi
fi

# --------------------------------------------------------------------------
# check-db-errors: describe_db_error only inside the canonical helper
# --------------------------------------------------------------------------
if ! skip "db-errors"; then
  hits=$(rg -l "describe_db_error" "$ROOT" --type rust -g '!target' 2>/dev/null \
    | rg -v "db_errors\.rs|/underlay/" | while read -r f; do rg -L "fn db_op_message|fn internal_db_error" "$f" >/dev/null 2>&1 || echo "$f"; done || true)
  if [[ -z "$hits" ]]; then
    pass "db-errors"
  else
    fail "db-errors" "describe_db_error used outside the canonical helper:\n$hits"
  fi
fi

# --------------------------------------------------------------------------
# check-openapi-gated: Swagger/OpenAPI must be gated to development envs
# --------------------------------------------------------------------------
if ! skip "openapi-gated"; then
  bad=""
  while IFS= read -r -d '' file; do
    if ! rg -q "include_docs|is_development|Local|build_router_with_options|Some\(axum|Option<|with_docs" "$file"; then
      bad="$bad$file\n"
    fi
  done < <(rg -l "SwaggerUi::new|/openapi\.json" "$ROOT" --type rust -g '!target' 2>/dev/null \
    | rg -v "/underlay/|test" | tr '\n' '\0')
  if [[ -z "$bad" ]]; then
    pass "openapi-gated"
  else
    fail "openapi-gated" "OpenAPI mounted without an environment gate:\n$bad"
  fi
fi

# --------------------------------------------------------------------------
# check-seeds-gated: dev seeds require env AND local-database-host gates
# --------------------------------------------------------------------------
if ! skip "seeds-gated"; then
  while IFS= read -r -d '' file; do
    if rg -q "run_dev_seeds|run_bootstrap_seed" "$file"; then
      if ! rg -q "is_local_database_url" "$file"; then
        fail "seeds-gated" "seed runner without local-database-host guard: $file"
      fi
    fi
  done < <(rg -l "run_dev_seeds|run_bootstrap_seed" "$ROOT" --type rust -g '!target' 2>/dev/null \
    | rg -v "/underlay/|test|/db/src/lib.rs|/bin/|/crates/migration/" | tr '\n' '\0')
  pass "seeds-gated"
fi

# --------------------------------------------------------------------------
# check-html-sanitized: {@html} requires a sanitizer in the same file
# --------------------------------------------------------------------------
if ! skip "html-sanitized"; then
  bad=""
  while IFS= read -r -d '' file; do
    if ! rg -q "sanitizeHtml|sanitizeSvgHtml|sanitizeEmbedHtml|DOMPurify|renderSafeMarkdownPreview|renderMarkdown\(" "$file"; then
      bad="$bad$file\n"
    fi
  done < <(rg -l "\{@html" "$ROOT" -g '*.svelte' -g '!node_modules' 2>/dev/null | tr '\n' '\0')
  if [[ -z "$bad" ]]; then
    pass "html-sanitized"
  else
    fail "html-sanitized" "{@html} without a sanitizer in the same file:\n$bad"
  fi
fi

# --------------------------------------------------------------------------
# check-no-svg-blacklist: no regex-blacklist SVG "validation"
# --------------------------------------------------------------------------
if ! skip "svg-blacklist"; then
  hits=$(rg "validateQrSvg|dangerous = /<script|dangerous = /\\\\bon" "$ROOT" -g '!node_modules' -g '!target' -g '!*.md' 2>/dev/null || true)
  if [[ -z "$hits" ]]; then
    pass "svg-blacklist"
  else
    fail "svg-blacklist" "regex-blacklist SVG validation found (use sanitizeSvgHtml):\n$hits"
  fi
fi

# --------------------------------------------------------------------------
# check-csp-served: CSP must exist at the real serving layer
# --------------------------------------------------------------------------
if ! skip "csp-served"; then
  found=""
  # static-host _headers files
  if rg -q "Content-Security-Policy" "$ROOT" -g '_headers' -g '!node_modules' 2>/dev/null; then
    found="static _headers"
  fi
  # server-side header emission (node adapter, gateway, Rust middleware)
  if rg -q "content-security-policy|Content-Security-Policy|applyCspHeaders" "$ROOT" \
    -g '!node_modules' -g '!target' -g '!*.md' -g '!_headers' 2>/dev/null; then
    found="server headers"
  fi
  if [[ -n "$found" ]]; then
    pass "csp-served"
  else
    fail "csp-served" "no CSP at any serving layer (static _headers, server headers, or hooks)"
  fi
fi

# --------------------------------------------------------------------------
# check-no-tracked-secrets: .env and local.toml must not be tracked
# --------------------------------------------------------------------------
if ! skip "tracked-secrets"; then
  bad=""
  if git -C "$ROOT" rev-parse --is-inside-work-tree &>/dev/null; then
    tracked=$(git -C "$ROOT" ls-files | rg '(^|/)\.env($|\.)|(^|/)config/local\.toml$' | rg -v '\.example$' || true)
    if [[ -n "$tracked" ]]; then
      bad="$tracked"
    fi
  fi
  if [[ -z "$bad" ]]; then
    pass "tracked-secrets"
  else
    fail "tracked-secrets" "secret-bearing files tracked in git:\n$bad"
  fi
fi

# --------------------------------------------------------------------------
# check-totp-cipher: TOTP secrets must go through SecretCipher
# --------------------------------------------------------------------------
if ! skip "totp-cipher"; then
  totp_files=$(rg -l "secret_base32" "$ROOT" --type rust -g '!target' 2>/dev/null \
    | rg -v "/underlay/" || true)
  if [[ -z "$totp_files" ]]; then
    pass "totp-cipher (no TOTP)"
  else
    if rg -l "SecretCipher|totp_cipher" "$ROOT" --type rust -g '!target' 2>/dev/null | rg -q "."; then
      pass "totp-cipher"
    else
      fail "totp-cipher" "TOTP secrets present but no SecretCipher usage:\n$totp_files"
    fi
  fi
fi

# --------------------------------------------------------------------------
# check-canonical-sessions: session rotation via underlay-auth-session only
# --------------------------------------------------------------------------
if ! skip "canonical-sessions"; then
  hits=$(rg -l "fn rotate_session_if_current|\.rotate_session_if_current\(" "$ROOT" --type rust -g '!target' 2>/dev/null \
    | rg -v "session_repo\.rs|/underlay/" || true)
  if [[ -z "$hits" ]]; then
    pass "canonical-sessions"
  else
    fail "canonical-sessions" "local session rotation logic outside the session_repo adapter:\n$hits"
  fi
fi

# --------------------------------------------------------------------------
# check-role-hierarchy: admin user mutations require the hierarchy guard
# --------------------------------------------------------------------------
if ! skip "role-hierarchy"; then
  mutation_files=$(rg -l "pub async fn (update_user|create_user|suspend_user|update_user_role)" "$ROOT" \
    --type rust -g '!target' 2>/dev/null | rg "routes/" | rg -v "/underlay/" || true)
  if [[ -z "$mutation_files" ]]; then
    pass "role-hierarchy (no admin user mutations)"
  else
    if rg -l "RoleHierarchy|can_manage_user|guard_can_manage|can_assign_role" "$ROOT" \
      --type rust -g '!target' 2>/dev/null | rg -q "."; then
      pass "role-hierarchy"
    else
      fail "role-hierarchy" "admin user mutations without hierarchy guard:\n$mutation_files"
    fi
  fi
fi

# --------------------------------------------------------------------------
# check-refresh-recheck: refresh must re-check account status
# (implied by canonical-sessions when using the crate; explicit for stragglers)
# --------------------------------------------------------------------------
if ! skip "refresh-recheck"; then
  if rg -l "underlay_auth_session|underlay-auth-session" "$ROOT" -g 'Cargo.toml' 2>/dev/null | rg -q "."; then
    pass "refresh-recheck"
  else
    refresh_files=$(rg -l "verify_refresh_token" "$ROOT" --type rust -g '!target' 2>/dev/null \
      | rg -v "/underlay/" || true)
    if [[ -z "$refresh_files" ]]; then
      pass "refresh-recheck (no refresh path)"
    else
      fail "refresh-recheck" "refresh token handling without underlay-auth-session:\n$refresh_files"
    fi
  fi
fi

# --------------------------------------------------------------------------
# Report
# --------------------------------------------------------------------------
echo "Conformance report for: $ROOT"
echo

for p in "${PASSES[@]}"; do
  echo "  PASS  $p"
done

if [[ ${#FAILURES[@]} -gt 0 ]]; then
  echo
  for f in "${FAILURES[@]}"; do
    echo -e "  FAIL  $f"
  done
  echo
  echo "${#FAILURES[@]} conformance violation(s) found."
  exit 1
fi

echo
echo "All conformance checks passed."
exit 0
