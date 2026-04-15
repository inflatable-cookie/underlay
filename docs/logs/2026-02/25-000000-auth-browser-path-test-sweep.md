# Auth Browser-Path Test Sweep (2026-02-25)

## Scope

Add and validate browser-path automated coverage for roadmap `004` remaining auth integration gates in consuming apps.

Targets:
- Acowtancy Dairy auth login route (`/routes/(auth)/login/+page.svelte`)
- Songsprout auth route theming/check follow-up from prior sweep

## Changes Applied

## Acowtancy Dairy

Added route-level passkey browser-path tests:
- `~/Dev/projects/acowtancy/dairy/tests/auth-login-page.test.ts`
- `~/Dev/projects/acowtancy/dairy/tests/fixtures/LoginPageStub.svelte`

What is verified:
1. Passkey login success path in route logic:
   - starts passkey challenge (`authCommands.passkeyLoginStart`)
   - converts WebAuthn options (`toPublicKeyRequestOptions`)
   - uses `navigator.credentials.get`
   - finishes login (`authCommands.passkeyLoginFinish`)
   - stores session (`auth.setSession`)
   - redirects using returnTo (`goto('/dashboard', { replaceState: true })`)
2. Unsupported-browser guard path:
   - missing WebAuthn APIs returns user-facing error
   - no passkey start call is made

Test/runtime support changes:
- Added test deps in Dairy:
  - `@testing-library/svelte`
  - `@testing-library/dom`
- Updated `~/Dev/projects/acowtancy/dairy/vite.config.ts` resolve conditions to prefer browser/module entrypoints in jsdom test runs.

## Songsprout

Follow-up validation after auth-route theming fix:
- Installed workspace dependencies and re-ran check.
- No additional code changes needed beyond prior auth route layout.

## Verification Commands

Acowtancy Dairy:
1. `bun x vitest run tests/auth-login-page.test.ts`
2. `bun x vitest run tests/auth-login-page.test.ts tests/smoke.test.ts`
3. `bun run check`

Songsprout Bloom:
1. `bun install`
2. `bun run check`

Results:
- Dairy tests/check: pass (`auth-login-page` tests green; `svelte-check` 0 errors, 1 unrelated warning in froyo)
- Songsprout check: pass (`0 errors`, `0 warnings`)

## Remaining Open Gates

Still open for roadmap `004` closure:
1. Live WebAuthn E2E with a real browser authenticator (not mocked test APIs).
2. Live Google OAuth E2E (start/callback/refresh/disconnect) against configured provider credentials in consuming apps (deferred in this environment).
3. Runtime/env readiness must be green (see `docs/logs/2026-02/25-000000-auth-live-e2e-readiness-sweep.md`).
