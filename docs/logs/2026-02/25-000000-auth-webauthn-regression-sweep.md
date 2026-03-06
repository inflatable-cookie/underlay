# Auth WebAuthn Regression Sweep (2026-02-25)

## Scope

Create and run a single repeatable non-OAuth auth regression command across consuming apps, focused on passkey/WebAuthn-related flows.

Script added:
- `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-regression.sh`

## Coverage Included

1. Songsprout Bloom server auth actions:
   - `src/lib/server-tests/auth-login-page.server.test.ts`
   - `src/lib/server-tests/security-page.server.test.ts`
2. Acowtancy Dairy auth browser-path route test:
   - `tests/auth-login-page.test.ts`

## Command Run

- `/Users/betterthanclay/Dev/projects/underlay/scripts/auth-webauthn-regression.sh`

## Result

- Songsprout: `14/14` tests passed
- Dairy: `2/2` tests passed
- Combined regression sweep: pass

## Outcome

- Passkey/WebAuthn non-OAuth regression coverage is now executable in one command from Underlay.
- This does not replace live manual authenticator validation, but it provides a stable automated baseline for `004` while OAuth remains deferred in this environment.
