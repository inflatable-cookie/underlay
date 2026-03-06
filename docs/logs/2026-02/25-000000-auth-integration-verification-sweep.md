# Auth Integration Verification Sweep (2026-02-25)

## Scope

- `songsprout/nursery` API-level auth flow verification
- `underlay-reference/acme-api` API-level auth flow verification
- Focus: roadmap `004` integration gates (register/login/sessions/2FA/passkeys/OAuth, role checks)

## Environment + Setup

1. Reset DBs:
   - `cd /Users/betterthanclay/Dev/projects/songsprout/nursery && bun run db:reset`
   - `cd /Users/betterthanclay/Dev/projects/underlay-reference/acme-api && bun run db:reset`
2. Startup fixes applied:
   - `acme-api` route syntax migrated for axum 0.8 (`:param` -> `{param}`) in:
     - `/Users/betterthanclay/Dev/projects/underlay-reference/acme-api/crates/api/src/routes/mod.rs`
3. Local auth enablement for Nursery runtime verification:
   - `LOCAL_AUTH=true` with JWT/WebAuthn env values set for process start.

## Results Matrix

## Songsprout (`nursery`)

- `POST /v1/auth/register`: pass (200)
- `POST /v1/auth/login`: pass (200)
- `GET /v1/auth/me` with bearer token: pass (200)
- `POST /v1/auth/passkeys/register/start` with bearer token: pass (200, has `state_id` + `options`)
- `POST /v1/auth/passkeys/login/start`: pass (200, has `state_id` + `options`)
- `POST /v1/auth/totp/setup` with bearer token: pass (200, has `setup_id` + `otpauth_uri`)
- `POST /v1/auth/oauth/google/start`: fails as configured-not-ready (`auth.bad_request`, message `google oauth not configured`)
- `GET /v1/auth/oauth/google/status` with bearer token: pass (200, `connected=false`)
- `POST /v1/auth/oauth/google/refresh`: fails as configured-not-ready (`auth.bad_request`)
- `POST /v1/auth/oauth/google/disconnect`: fails as configured-not-ready (`auth.bad_request`)

2FA completion attempt:
- TOTP setup + enable succeeds (`200` + `204`)
- Login without code correctly returns `auth.2fa_required`
- `login/start` correctly returns `requires_two_factor=true`
- `login/finish` returns success (`200`) when using a code from the next 30s TOTP window after setup/enable.
- Prior `auth.2fa_invalid` repros were from same-window code reuse after enable; replay protection rejects reused counters.

Inference:
- 2FA completion path is working; verification needs to account for TOTP replay-protection timing.

## Underlay Reference (`acme-api`)

- `POST /v1/auth/register`: pass (200, returns access token/session id)
- `POST /v1/auth/login`: pass (200 for non-2FA users)
- `GET /v1/auth/me` with bearer token: pass (200)
- `POST /v1/auth/passkeys/register/start`: pass (200, has `state_id` + `options`)
- `POST /v1/auth/passkeys/login/start`: pass (200, has `state_id` + `options`)
- `POST /v1/auth/totp/setup`: pass (200, has `setup_id` + `otpauth_uri`)
- `POST /v1/auth/oauth/google/start`: not implemented in reference API (404)

Role-based access verification:
- Seeded regular user token -> `GET /v1/admin/users`: `403` (`auth.forbidden`)
- Seeded admin flow via `login/start` + TOTP `login/finish` -> admin token accepted
- Seeded admin token -> `GET /v1/admin/users`: `200`

Inference:
- Role enforcement is verified for reference integration.
- OAuth verification is blocked in reference app because OAuth routes are absent.

## Roadmap 004 Impact

Evidence-backed status updates:
1. Cross-app auth API flows are mostly validated for password/session/passkey-start/TOTP-setup paths.
2. Role-based access behavior is validated in reference integration (`acme-api`).
3. Remaining open gates are now narrow and explicit:
   - Browser-driven passkey finish/login verification
   - Google OAuth configured E2E verification (Nursery config + reference route availability)
4. Auth UI theming in consuming apps is validated separately in:
   - `docs/logs/2026-02/25-000000-auth-ui-consuming-app-theming-sweep.md`
5. Browser-path auth route tests in consuming apps are validated separately in:
   - `docs/logs/2026-02/25-000000-auth-browser-path-test-sweep.md`
