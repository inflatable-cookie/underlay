# g07.015 - Runtime Data Subpath Split

Status: complete
Owner: repo maintainers
Updated: 2026-06-06

## Context

`runtime/data` stayed broad through `g07.006` because it represented one lower
collection workflow layer below templates. The follow-up audit showed real
consumer usage across all six roots, so removing or renaming `runtime/data`
would create churn without improving behavior.

The cleaner move is additive: expose narrower homes for new imports while
keeping `runtime/data` as the stable aggregate.

## Goals

- [x] add focused runtime subpaths for collection, reorder, and selection
  helpers
- [x] keep `runtime/data` source-compatible
- [x] update package exports
- [x] add public import coverage for the new subpaths
- [x] update contract and upgrade guidance

## Non-Goals

- migrating the six consumer apps in this batch
- removing `runtime/data`
- changing helper behavior
- moving implementation files out of `patterns/*`

## New Public Paths

| Path | Preferred for |
| --- | --- |
| `@inflatable-cookie/underlay/runtime/collections` | list controllers, cursor pagination types, cursor pagination helpers, and client/server pagination controllers |
| `@inflatable-cookie/underlay/runtime/reorder` | reorder controllers, reorder sessions, reorder conflict parsing, and reorder recovery helpers |
| `@inflatable-cookie/underlay/runtime/selection` | selection history, synced selection, batch selection/actions, selection mode, and transform-selection state |

`@inflatable-cookie/underlay/runtime/data` remains the aggregate compatibility path for
the same lower collection workflow layer.

## Consumer Upgrade Impact

Impact class: `additive`.

No consumer update is required. New code should prefer the focused paths above.
Existing `runtime/data` imports remain valid.

## Validation

- `effigy check:exports`
- `bun x vitest run -c vitest.component.config.ts ts/tests/components/package-runtime-compatibility.component.test.ts`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy validate`

## Next Task

No active `g07` task remains. Open a bounded roadmap card before migrating
consumer imports from `runtime/data` to the new focused subpaths.
