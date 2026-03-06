# Auth Live E2E Readiness Sweep (2026-02-25)

## Scope

Establish a repeatable readiness gate before running live browser WebAuthn/OAuth E2E for roadmap `004`.

Script added:
- `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-live-e2e-readiness.sh`

## What The Script Checks

- Songsprout (`nursery` + `bloom`):
  - JWT keys present
  - WebAuthn RP settings present
  - Google OAuth settings present
  - auth route layout presence in Bloom
  - auth/oauth endpoint reachability
- Acowtancy (`farmyard` + `dairy`):
  - JWT keys present
  - WebAuthn RP settings present
  - Google OAuth settings present
  - browser-path test presence in Dairy
  - auth/oauth endpoint reachability

## Run Results

### Initial run (2026-02-25T10:31:27Z)

Command:
- `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-live-e2e-readiness.sh`

Summary:
- `5 pass, 7 blocked`

Key blockers identified:
1. Songsprout `nursery/.env` missing local JWT, WebAuthn RP, and Google OAuth values.
2. Songsprout API at `http://127.0.0.1:4100` was not running during readiness check.
3. Acowtancy `farmyard/.env` has JWT keys, but WebAuthn RP and Google OAuth config are still unset.

Key passes:
1. Songsprout auth route theming/layout fix is present.
2. Dairy browser-path auth test is present.
3. Dairy auth/oauth routes are reachable on current API base (HTTP `405` on GET confirms route presence for POST handlers).

### Follow-up run after env/runtime fixes (2026-02-25T10:36:12Z)

Applied fixes before rerun:
1. Songsprout `nursery/.env` updated with local auth mode, JWT keys, OAuth secret key, and WebAuthn RP config.
2. Farmyard `.env` updated with WebAuthn RP config.
3. Songsprout API started on `:4100` for reachability checks.

Summary:
- `10 pass, 2 blocked`

Remaining blockers:
1. Songsprout: Google OAuth credentials still unset (`AUTH_GOOGLE_CLIENT_ID`, `AUTH_GOOGLE_CLIENT_SECRET`, `AUTH_GOOGLE_REDIRECT_URI`).
2. Acowtancy Farmyard: Google OAuth credentials still unset (`AUTH_GOOGLE_CLIENT_ID`, `AUTH_GOOGLE_CLIENT_SECRET`, `AUTH_GOOGLE_REDIRECT_URI`).

### OAuth-deferred run (2026-02-25T10:43:59Z)

Command:
- `SKIP_GOOGLE_OAUTH_CHECKS=1 /Users/betterthanclay/Dev/projects/underlay/scripts/auth-live-e2e-readiness.sh`

Summary:
- `12 pass, 0 blocked`

Notes:
1. This mode explicitly skips Google OAuth credential and OAuth route checks.
2. It provides a green readiness gate for WebAuthn/live-auth flows in environments where OAuth credentials are unavailable.
3. Confirmed again at `2026-02-25T10:49:19Z` with Songsprout API running: still `12 pass, 0 blocked`.

## Practical Next Step

1. If OAuth is in scope, set real Google OAuth credentials in Songsprout `nursery/.env` and Acowtancy `farmyard/.env`.
2. Re-run readiness script (normal mode or skip mode as appropriate).
3. Execute the live browser E2E runbook for in-scope checks (WebAuthn required; OAuth optional/deferred in this environment).
