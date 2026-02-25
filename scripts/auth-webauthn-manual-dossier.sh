#!/usr/bin/env bash
set -euo pipefail

UNDERLAY_DIR="/Users/betterthanclay/Dev/projects/underlay"
READINESS_SCRIPT="$UNDERLAY_DIR/scripts/auth-live-e2e-readiness.sh"
REGRESSION_SCRIPT="$UNDERLAY_DIR/scripts/auth-webauthn-regression.sh"
REPORT_DIR="$UNDERLAY_DIR/docs/reports"

SKIP_GOOGLE_OAUTH_CHECKS="${SKIP_GOOGLE_OAUTH_CHECKS:-1}"

TS="$(date -u +"%Y-%m-%dT%H:%M:%SZ")"
TS_FILE="$(date -u +"%Y-%m-%d-%H%M%S")"
REPORT_PATH="$REPORT_DIR/${TS_FILE}-auth-webauthn-manual-evidence-dossier.md"

mkdir -p "$REPORT_DIR"

readiness_tmp="$(mktemp)"
regression_tmp="$(mktemp)"

cleanup() {
  rm -f "$readiness_tmp" "$regression_tmp"
}
trap cleanup EXIT

set +e
SKIP_GOOGLE_OAUTH_CHECKS="$SKIP_GOOGLE_OAUTH_CHECKS" "$READINESS_SCRIPT" >"$readiness_tmp" 2>&1
readiness_code=$?
set -e

set +e
"$REGRESSION_SCRIPT" >"$regression_tmp" 2>&1
regression_code=$?
set -e

readiness_status="PASS"
if [[ $readiness_code -ne 0 ]]; then
  readiness_status="BLOCKED"
fi

regression_status="PASS"
if [[ $regression_code -ne 0 ]]; then
  regression_status="BLOCKED"
fi

{
  printf '# Auth WebAuthn Manual Evidence Dossier\n\n'
  printf 'Generated: %s\n\n' "$TS"
  printf '## Automated Baseline\n\n'
  printf -- '- Readiness gate (`SKIP_GOOGLE_OAUTH_CHECKS=%s`): **%s**\n' "$SKIP_GOOGLE_OAUTH_CHECKS" "$readiness_status"
  printf -- '- WebAuthn regression sweep: **%s**\n\n' "$regression_status"

  printf '## Readiness Output\n\n```text\n'
  cat "$readiness_tmp"
  printf '\n```\n\n'

  printf '## Regression Output\n\n```text\n'
  cat "$regression_tmp"
  printf '\n```\n\n'

  cat <<'EOF_REPORT'
## Manual WebAuthn Evidence Checklist

Mark each item with timestamped screenshot + outcome.

### Songsprout (Bloom + Nursery)

- [ ] Open `/login` and authenticate via passkey where applicable.
- [ ] Open `/security` and start passkey registration.
- [ ] Complete authenticator prompt and verify passkey appears in list.
- [ ] Perform passkey login confirmation after registration.

### Acowtancy (Dairy + Farmyard)

- [ ] Open `/login` and authenticate via passkey.
- [ ] Open `/account/passkeys` and add passkey.
- [ ] Verify passkey appears in list and can be renamed/deleted safely.

## Result Table (fill during manual run)

| App | Flow | Result | Evidence path | Notes |
| --- | --- | --- | --- | --- |
| Songsprout | Passkey register | PENDING |  |  |
| Songsprout | Passkey login | PENDING |  |  |
| Dairy | Passkey login | PENDING |  |  |
| Dairy | Passkey register/manage | PENDING |  |  |

## Closure Notes

- OAuth checks are intentionally out of scope when credentials are unavailable.
- Once all rows above are PASS, update roadmap `004` WebAuthn live verification item and Section 9/10 verification checkboxes accordingly.
EOF_REPORT
} >"$REPORT_PATH"

printf 'Wrote manual evidence dossier: %s\n' "$REPORT_PATH"

if [[ $readiness_code -ne 0 || $regression_code -ne 0 ]]; then
  exit 2
fi
