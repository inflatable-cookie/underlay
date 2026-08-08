# g07.010 Artifact - Consumer Import Compatibility Sweep

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Scope

This artifact records the six-consumer import sweep against the g07 preferred
TypeScript public paths.

Consumer roots:

- `underlay-reference`
- `contact-patch`
- `compli-me`
- `acowtancy`
- `songsprout`
- `loophole/composer`

## Matrix

| Consumer | Result | Notes |
| --- | --- | --- |
| `underlay-reference` | preferred | Uses `client/suggestions` in `acme-client`; uses `runtime/data` for selection history; no compatibility-only Underlay import found. |
| `contact-patch` | fixed | `cp-client` re-exported suggestion helpers from `runtime/data`; changed to `client/suggestions` while keeping selection history on `runtime/data`. |
| `compli-me` | preferred source imports | Source imports use retained paths. `front` and `admin` still have config-only `@inflatable-cookie/underlay/components` optimizeDeps excludes. |
| `acowtancy` | preferred source imports | Heavy `runtime/relations`, `runtime/data`, `runtime/auth`, `templates`, `client`, `nightfire`, and one `testing` import are retained. `cattle-grid` keeps app-local suggestion query helpers for app-specific query shape. |
| `songsprout` | preferred source imports | Source imports use retained paths. `bloom` and `greenhouse` still have config-only `@inflatable-cookie/underlay/components` optimizeDeps excludes. |
| `loophole/composer` | preferred source imports | Source imports use retained paths. `composer-front` and `composer-admin` still have config-only `@inflatable-cookie/underlay/components` optimizeDeps excludes. |

## Consumer Edit

Applied in `contact-patch`:

- `cp-client/src/index.ts`
  - `createSelectionHistory` and selection history types stay on
    `@inflatable-cookie/underlay/runtime/data`
  - `formatHintsParam`, `parseHintsParam`, `buildSuggestionParams`,
    `appendSuggestionParams`, and `SuggestionRequestOptions` now come from
    `@inflatable-cookie/underlay/client/suggestions`

Validation:

- `effigy cp-client/check` passed in `contact-patch`

## Compatibility Findings

No source imports remain that require keeping suggestion request helpers
available through `runtime/data`.

Do not remove the compatibility re-export yet. First finish the config cleanup
and closeout note so the retirement decision is explicit.

## Config-Only Drift

Several consumer Vite configs still exclude
`@inflatable-cookie/underlay/components` from dependency optimization even though that
is no longer a retained Underlay package subpath.

Affected files:

- `compli-me/front/vite.config.ts`
- `compli-me/admin/vite.config.ts`
- `songsprout/bloom/vite.config.ts`
- `songsprout/greenhouse/vite.config.ts`
- `loophole/composer/composer-front/vite.config.ts`
- `loophole/composer/composer-admin/vite.config.ts`

This is config-only drift, not a source import or runtime dependency. Queue it
as `g07.011` before final closeout.
