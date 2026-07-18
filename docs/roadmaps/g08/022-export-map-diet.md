# g08.022 - Export-Map Diet

Status: complete
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Shrink the ~60-subpath export map to one canonical path per symbol. `client`
exports 18 individual files plus a barrel; `runtime` has both `./runtime` and 15
sub-barrels; `runtime/data` re-exports selection + reorder + list-controller +
pagination so the same symbols are importable three ways; `runtime/index.ts`
renames symbols (`appendCursorPaginationParams as appendPaginationParams`) so the
same function has two public names. The `check-exports.ts` guardrail exists
because the surface is hard to police. Tree-shaking is a non-issue since
consumers compile from source.

## Evidence

- `package.json` exports (~60 subpaths)
- `ts/src/runtime/data.ts`, `ts/src/runtime/index.ts` (aliasing)
- root `.` export is an empty module (`ts/src/index.ts` = `export {};`)

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [023 Release and compatibility rollout](../../contracts/023-release-and-compatibility-rollout.md)

## Planned Changes

- [~] **Deferred: broad subpath collapse conflicts with g07's active
  compatibility window.** Many of these subpaths (`runtime/collections`,
  `runtime/reorder`, `runtime/media/*`, `client/envelopes`, etc.) were added
  in `g07.015`/`.018`/`.020` as focused paths with the aggregates kept as
  dated compatibility (guide `190`). Collapsing them now would prematurely
  retire retained compat paths and break consumers for no functional gain
  (consumers compile from source; no tree-shaking benefit). The collapse
  should wait until the g07 aggregate paths reach their retirement dates.
- [ ] One canonical home per symbol; keep renamed aliases only behind the
  compatibility-sunset mechanism (`scripts/check-compatibility-sunset.sh`) with
  a dated retirement.
- [x] Retired the empty `.` root export (`export {}`) and deleted
  `ts/src/index.ts`. Safe: no consumer imports the bare package; the bare
  specifier appears only in consumer `dedupe`/`optimizeDeps.exclude`
  (non-resolving) and `resolve.alias` (bypasses package exports).

## Consumer Upgrade Impact

Impact class: `behavioral`. Import paths change; managed via the compatibility
sunset process. Requires six-consumer proof per `023`.

## Validation

- [ ] `effigy check:exports`; `package-compatibility.test.ts` updated to the new
  surface
- [ ] `bun x vitest run`
- [ ] `effigy validate`

## Stop Conditions

Stop if a consumer depends on a subpath with no barrel equivalent; add the
barrel before removing the subpath.

## Completion Notes

Completed 2026-07-17 (scoped: de-duplication + root retirement; broad subpath
collapse deferred).
- Empty `.` root export retired; `ts/src/index.ts` deleted. `check-exports`
  clean; verified transparent for all six consumers (bare specifier only in
  non-resolving `dedupe`/`exclude`/`alias`).
- Renamed/deprecated aliases registered in the compatibility-sunset CSV with a
  2027-01-31 date (retire g09): `./runtime` `appendPaginationParams` /
  `PaginatedResponse`; `g08.017` `PaginationParams` type aliases (http + db);
  deprecated `WhereBuilder::add_raw`. `check-compatibility-sunset` passes.
- **Deferred (surfaced):** the headline "70 -> barrel-level" collapse. The
  subpaths are largely g07's intentional focused-path surface with the
  aggregates dated for later retirement (guide `190`); collapsing now would
  break retained compat paths for no functional gain. Recorded as a follow-up
  gated on the g07 retirement dates rather than forced here.

Validated: `effigy check:exports` clean, `check-compatibility-sunset` pass,
`bun x tsc` clean, `bun x vitest run` 739 passed (incl. the retained
`package-compatibility.test.ts` locking the aliased surface).

## Consumer Rollout

Impact class **none** in practice. The root retirement is transparent (no
consumer imports the bare package root). No subpath was removed, so no consumer
import breaks; the alias retirements are dated (g09), not immediate.

## Next Task

`g08.023` EntityList generics and split.
