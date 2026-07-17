# 2026-07-17 - g08.022 export-map diet (scoped)

## Context

The package export map had ~70 subpaths with duplication: `./runtime` exposed
two renamed aliases (same symbols as `patterns/pagination-types` under
different names), and the `.` root was an empty `export {}` module.

## Decision: de-duplicate + retire the dead root; defer the broad collapse

The card's headline ask ("collapse 70 -> barrel-level") conflicts with g07's
active compatibility window: many subpaths (`runtime/collections`,
`runtime/reorder`, `runtime/media/*`, `client/envelopes`, etc.) are g07's
deliberately-added focused paths, with the aggregates kept as dated compat
(guide `190`). Collapsing now would prematurely retire retained compat paths
and break consumers for no functional gain (they compile from source; no
tree-shaking benefit). So the diet is scoped to the real defects; the broad
collapse is deferred to the g07 retirement dates.

## Changes

- **Retired the empty `.` root export** and deleted `ts/src/index.ts`. Verified
  transparent for all six consumers: the bare `@decodelabs/underlay` specifier
  appears only in non-resolving `dedupe` / `optimizeDeps.exclude` and in
  `resolve.alias` (which bypasses package exports). No consumer imports the
  bare package root.
- **Renamed/deprecated aliases put behind the compatibility-sunset registry**
  with a 2027-01-31 date (retire g09), in
  `docs/contracts/016-compatibility-adapters.csv`:
  - `./runtime` `appendPaginationParams` / `PaginatedResponse` (renamed
    exports of `appendCursorPaginationParams` / `CursorPaginatedResponse`)
  - `g08.017` deprecated `PaginationParams` type aliases (http + db pagination)
  - deprecated `WhereBuilder::add_raw` (superseded by `add_raw_indexed`)

## Validation

- `effigy check:exports`: clean (69 subpaths, was 70).
- `check-compatibility-sunset.sh`: pass (all within window).
- `bun x tsc`: clean.
- `bun x vitest run`: 739 passed, incl. `package-compatibility.test.ts` which
  locks the retained aliased surface.

## Consumer Upgrade Notes

Impact class **none** in practice. No subpath was removed, so no consumer
import breaks. The root retirement is transparent. The alias retirements are
dated (g09), not immediate - consumers migrate off the renamed names before
that date.

## Next

`g08.023` EntityList generics and split.
