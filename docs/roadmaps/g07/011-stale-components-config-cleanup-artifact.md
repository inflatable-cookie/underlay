# g07.011 Artifact - Stale Components Config Cleanup

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Scope

This artifact records the consumer config cleanup after `g07.010` found stale
`@inflatable-cookie/underlay/components` references in Vite optimizeDeps excludes.

## Files Updated

Removed `@inflatable-cookie/underlay/components` from:

- `compli-me/front/vite.config.ts`
- `compli-me/admin/vite.config.ts`
- `songsprout/bloom/vite.config.ts`
- `songsprout/greenhouse/vite.config.ts`
- `loophole/composer/composer-front/vite.config.ts`
- `loophole/composer/composer-admin/vite.config.ts`

No source imports were changed.

## Validation

Consumer checks:

- `compli-me`: `effigy admin/check` passed
- `compli-me`: `effigy front/check` passed
- `songsprout`: `effigy bloom/check` passed
- `songsprout`: `effigy greenhouse/check` passed
- `loophole/composer`: `effigy composer-admin/check` passed
- `loophole/composer`: `effigy composer-front/check` passed

Scan:

- targeted `rg` found no remaining `@inflatable-cookie/underlay/components` entries in
  the six edited live Vite config files

## Consumer Impact

Config-only cleanup.

No Underlay public API, source import, or runtime behavior changed.

## Follow-On

`g07.012` should publish the TS boundary hardening closeout and upgrade note.
