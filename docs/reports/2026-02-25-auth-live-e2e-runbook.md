# Auth Live E2E Runbook (Songsprout + Dairy)

## Preconditions

1. Readiness script passes:
   - `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-live-e2e-readiness.sh`
   - and non-OAuth regression is green:
   - `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-regression.sh`
   - and a manual evidence dossier is generated:
   - `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-manual-dossier.sh`
2. If OAuth is in scope, Google OAuth credentials are configured for both backends:
   - Songsprout `nursery/.env`
   - Acowtancy `farmyard/.env`
   - If OAuth is out of scope, run readiness with `SKIP_GOOGLE_OAUTH_CHECKS=1`
3. Backend APIs are running:
   - Songsprout API: `http://127.0.0.1:4100`
   - Farmyard API: `http://localhost:40001`
4. Frontends are running:
   - Bloom (Songsprout)
   - Dairy (Acowtancy)

## Songsprout Live Checks

1. Login route OAuth redirect (optional when OAuth is in scope):
   - Open `/login` in Bloom.
   - Click `Continue with Google`.
   - Verify redirect to Google consent screen.
2. OAuth callback (optional when OAuth is in scope):
   - Complete consent and return to app.
   - Verify authenticated session state and redirect path.
3. Security page passkey register/login:
   - Open `/security`.
   - Start passkey registration.
   - Complete authenticator prompt.
   - Verify passkey list updates.
4. OAuth disconnect (optional when OAuth is in scope):
   - From `/security`, disconnect Google.
   - Verify status changes to disconnected.

## Dairy Live Checks

1. Login page passkey authentication:
   - Open `/login`.
   - Use passkey sign-in from `LoginPage`.
   - Verify redirect to `returnTo` or `/`.
2. Account passkey registration flow:
   - Open `/account/passkeys`.
   - Add a passkey.
   - Verify passkey appears in list.
3. Account 2FA flow sanity:
   - Open `/account/2fa`.
   - Run setup/enable cycle and verify success UI.

## Evidence To Capture

1. Timestamped screenshots for each major checkpoint.
2. Network traces (or console-logged HTTP statuses) for:
   - OAuth start + callback
   - Passkey start + finish
3. Final per-app result table:
   - `PASS` / `BLOCKED`
   - blocker reason
   - next action

## Closure Criteria For Roadmap 004

- Live WebAuthn flow verified in both consuming apps.
- Live Google OAuth flow verified in at least one consuming app when OAuth is in scope, with start/callback/disconnect validated and refresh behavior confirmed where implemented.
- `docs/roadmap/004-underlay-auth-system-roadmap.md` active remaining auth E2E item marked complete with report links.
