# 190 - Upgrade Compatibility Matrix

A compact compatibility guide for upgrades affecting Underlay-based apps.

## Version Expectations

| Area | Current expectation | Notes |
|---|---|---|
| Package manager | Bun | Use `bun`, not `pnpm`/`npm`, in app repos |
| SvelteKit admin deployment | SPA with adapter-static fallback | See `docs/guides/110-admin.md` |
| Underlay client auth flow | `configureAuth()` + token refresh handler | Required for `useAuthenticatedData()` auto-refresh |
| Admin navigation pattern | Navigation context helpers | Use `gotoWithContext` + `consumeNavigationContext` |
| Form pattern | `SpaFormShell` + intent submit | Save/save-close/delete intent model |

## Upgrade Checklist

1. Runtime/tooling
- Confirm Bun commands and lockfile integrity.
- Confirm SvelteKit adapter/static settings for admin SPA.

2. Underlay package updates
- Re-run `bun install` in consuming app repos.
- Verify no import drift for moved/renamed exports.

3. Pattern compatibility
- Validate list, form, and navigation helpers still match docs.
- Re-check recipes in `docs/patterns/000-index.md` for changed APIs.

4. Regression checks
- Run backend + frontend checks and smoke routes.
- Validate auth refresh and protected page load behavior.

## Common Breakage Signals

- Protected pages fail after token expiry -> auth runtime setup missing.
- Back/cancel navigation goes to wrong page -> navigation context not preserved.
- List pages lose filter/pagination behavior -> controller usage drift.
- Upload pipeline fails after initiate -> upload plan/header mismatch.

## Required Post-Upgrade Docs Sync

After upgrades that change conventions, update:
- `docs/guides/README.md` (reading order if needed)
- `docs/patterns/000-index.md` (recipe links/prompts)
- impacted recipe/guides with current API/component names
