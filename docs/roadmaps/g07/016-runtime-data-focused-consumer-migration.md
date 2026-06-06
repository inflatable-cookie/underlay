# g07.016 - Runtime Data Focused Consumer Migration

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`g07.015` added focused runtime collection, reorder, and selection subpaths while
leaving `runtime/data` as the aggregate compatibility path.

This batch migrates the known six-consumer family where each import statement
cleanly maps to one focused subpath.

## Goals

- [x] migrate pagination/list imports to `runtime/collections`
- [x] migrate reorder imports to `runtime/reorder`
- [x] migrate selection-history, synced-selection, and transform-selection
  imports to `runtime/selection`
- [x] leave no live six-consumer source imports from `runtime/data`
- [x] validate the touched consumer packages

## Non-Goals

- removing `runtime/data`
- changing helper behavior
- changing app-local APIs beyond Underlay import paths
- rewriting consumer list or form code

## Consumer Changes

- `underlay-reference/acme-admin`: moved selection history, reorder conflict,
  relation-selection history type, and client pagination imports.
- `contact-patch/cp-client`: moved selection history and cursor pagination
  exports/imports.
- `contact-patch/cp-admin`: moved reorder imports.
- `compli-me/api-client`: moved cursor pagination imports/exports.
- `compli-me/admin`: moved reorder imports.
- `acowtancy/cattle-grid`: moved cursor pagination imports.
- `acowtancy/dairy`: moved reorder, selection history, synced selection, and
  transform-selection imports.
- `songsprout/stem`: moved cursor pagination type exports.
- `songsprout/greenhouse`: moved reorder imports.
- `loophole/composer/composer-admin`: moved reorder imports.

## Consumer Upgrade Impact

Impact class: `additive` for Underlay and source-compatible for consumers.

The six known consumers now use the focused subpaths. `runtime/data` remains
available as an aggregate compatibility path for unknown consumers.

## Validation

- `underlay-reference`: `effigy acme-admin/check`
- `contact-patch`: `effigy cp-client/check`, `effigy cp-admin/check`
- `compli-me`: `effigy api-client/check`, `effigy admin/check`
- `acowtancy`: `effigy cattle-grid/check`, `effigy dairy/check`
- `songsprout`: `effigy stem/check`, `effigy greenhouse/check`
- `loophole/composer`: `effigy composer-admin/check`

## Next Task

No active `g07` task remains. Keep `runtime/data` retained unless a future
bounded card proves unknown-caller risk is acceptable and the aggregate can be
retired deliberately.
