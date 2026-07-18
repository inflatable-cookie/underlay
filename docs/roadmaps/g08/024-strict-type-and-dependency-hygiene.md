# g08.024 - Strict-Type And Dependency Hygiene

Status: done
Owner: repo maintainers
Started: 2026-07-17
Completed: 2026-07-17

## Purpose

Close TS-level correctness and dependency gaps. `noImplicitAny` is `false` under
`strict: true`, silently disabling one of the most valuable checks across 34k
LOC (only 7 explicit `as any` exist, so the fallout is likely small). Plus:
`esm-env` is imported but undeclared (resolves transitively today),
`svelte-dnd-action` is declared but unused (dead dep), and `server/csp.ts`
imports bare `"crypto"` instead of `"node:crypto"`.

## Evidence

- `ts/tsconfig.json` (`noImplicitAny: false`)
- `ts/src/patterns/storage-wrapper.ts`, `storage-availability.ts`,
  `timezone.svelte.ts` (undeclared `esm-env`)
- `package.json` (`svelte-dnd-action` unused)
- `ts/src/server/csp.ts:2` (bare `crypto`)

## Governing References

- [090 TS runtime and client orchestration](../../contracts/090-ts-runtime-and-client-orchestration.md)
- [120 Tooling, testing, and contract artifacts](../../contracts/120-tooling-testing-and-contract-artifacts.md)

## Planned Changes

- [x] Removed the `noImplicitAny: false` override. `tsc` was already clean; the
  fallout was 16 implicit-any snippet params across 8 `.svelte` templates (only
  svelte-check checks those). All typed properly: `renderCell` trios ->
  `(TableColumn, TableRow, TableCellValue)`; detail-page `content`/`detailsTab`/
  `jobRunsTab` `loaded` -> the page's own `*DetailItem` type; `renderItem`/
  `managedItem` -> the item type plus a new exported `EntityListItemContext`
  (extracted from EntityList's previously-private `ItemContext` so consuming
  pages can annotate). Fallout was small enough to fix in-card, not split.
- [x] Typing `loaded` surfaced a real latent gap in
  `SystemScheduledTaskDetailPage`: `createdAt`/`updatedAt` are `string | null`
  but the `!== undefined` guards let `null` reach `TimeAgo` (`string | number |
  Date`). Tightened both guards to a truthy check.
- [x] Declared `esm-env` (`^1.2.2`, was resolved transitively); dropped unused
  `svelte-dnd-action`; `server/csp.ts` now imports `node:crypto` (the only bare
  node builtin in `ts/src`). Lockfile synced.

## Consumer Upgrade Impact

Impact class: `none`.

## Validation

- [x] `bun x tsc -p ./ts/tsconfig.json` clean under `noImplicitAny`.
- [x] `effigy validate` clean (svelte-check 0 errors across 2472 files,
  guardrails, component hygiene, poodle prop-name check).
- [x] 739 unit + 33 component tests pass.
- [x] Consumer spot-check: acowtancy/dairy (7740 files) 0 errors — the changes
  are internal (tsconfig, pruned dep, additive `EntityListItemContext` export,
  snippet param types), so consumer impact is none.

## Stop Conditions

Stop if enabling `noImplicitAny` surfaces a large latent fallout that warrants
its own card; if so, split it out rather than blocking the dep fixes.

## Next Task

Lane D complete. `g08.025` (Lane E) front-door doc repair.
