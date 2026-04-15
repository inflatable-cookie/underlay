# Auth UI Consuming-App Theming Sweep (2026-02-25)

## Scope

Validate roadmap `004` theming gate for auth UI components in consuming apps:

- Songsprout (`~/Dev/projects/songsprout/bloom`)
- Acowtancy Dairy (`~/Dev/projects/acowtancy/dairy`)

## Findings

## Songsprout (`bloom`)

Observed before fix:
- Auth route (`/routes/(auth)/login/+page.svelte`) uses Underlay auth/form primitives and Underlay token variables.
- Underlay token setup existed in app layout (`/routes/(app)/+layout.svelte`) but not in auth route group.
- Result: auth route styling relied on component fallbacks instead of app-level token contract.

Fix applied:
- Added `~/Dev/projects/songsprout/bloom/src/routes/(auth)/+layout.svelte`.
- Layout now imports:
  - `@decodelabs/underlay/styles/tokens.css`
  - `@decodelabs/underlay/styles/forms.css`
- Layout now defines the same Underlay token overrides used by Songsprout app layout for consistent theming.

Validation status:
- Static/theme-contract validation: pass
- `bun run check`: pass (`0 errors`, `0 warnings`) after workspace dependency install

## Acowtancy Dairy (`dairy`)

Observed:
- Auth routes are Underlay-auth-component based:
  - `/routes/(auth)/+layout.svelte` uses `AuthLayout`
  - `/routes/(auth)/login/+page.svelte` uses `LoginPage`
  - `/routes/(auth)/forgot-password/+page.svelte` uses `ForgotPasswordFlow`
- Global layout (`/routes/+layout.svelte`) imports Underlay token/forms CSS and defines app token overrides at `:root`.
- Auth routes inherit the same token contract.

Validation status:
- `bun run check`: pass (`0 errors`, `1 unrelated CSS warning in froyo`)

## Outcome

- Consuming-app auth theming gate is satisfied for roadmap `004`:
  - Songsprout auth routes now have explicit token/form style setup.
  - Dairy auth routes already consume Underlay auth components under a concrete token override set.
- Remaining roadmap `004` auth blockers are browser-driven WebAuthn/OAuth E2E verification, not auth UI theming.
