# 2026-07-17 - g08.024 strict-type and dependency hygiene

## Context

`ts/tsconfig.json` ran `strict: true` but with `noImplicitAny: false`, silently
disabling the check across 34k LOC. Plus three dependency defects: `esm-env`
imported but undeclared (resolved transitively), `svelte-dnd-action` declared but
unused, and `server/csp.ts` importing bare `"crypto"`.

## Changes

- **Enabled `noImplicitAny`.** `tsc` was already clean; the real fallout was 16
  implicit-any snippet parameters in 8 `.svelte` templates (only svelte-check
  type-checks Svelte markup). Typed each properly:
  - `renderCell` snippets (SystemJobListPage, ErrorLogListPage) ->
    `(column: TableColumn, row: TableRow, value: TableCellValue)`.
  - detail-page `content`/`detailsTab`/`jobRunsTab` `loaded` param ->
    the page's own `SystemJobDetailItem` / `SystemScheduledTaskDetailItem` /
    `ErrorLogDetailItem`.
  - `renderItem`/`managedItem` (SystemMediaTrashListPage,
    SystemScheduledTasksListPage, EntityInlineListModule) -> the item type plus
    `EntityListItemContext`.
- **Exported `EntityListItemContext`.** EntityList's per-item render context was a
  private `interface ItemContext`; extracted it to
  `template-types/entity-list.ts` as an exported type (auto-barrelled via
  `export type *`) and pointed EntityList's local alias at it, so consuming pages
  can annotate `renderItem`/`managedItem` context params.
- **Latent bug fixed.** Once `loaded` was typed in
  `SystemScheduledTaskDetailPage`, its `createdAt`/`updatedAt` (`string | null`)
  failed against `TimeAgo`'s `string | number | Date` because the `!== undefined`
  guards passed `null` through. Tightened both to truthy checks.
- **Dependency hygiene.** Declared `esm-env` (`^1.2.2`); removed unused
  `svelte-dnd-action`; `csp.ts` -> `node:crypto` (only bare builtin in `ts/src`).
  `bun install` synced the lockfile (svelte-dnd-action pruned, esm-env promoted
  to a direct dependency).

## Validation

- `bun x tsc -p ./ts/tsconfig.json`: clean under `noImplicitAny`.
- `effigy validate`: clean — svelte-check 0 errors (2472 files), guardrails,
  component hygiene, poodle prop-name check all pass.
- 739 unit + 33 component tests pass.
- Consumer spot-check: acowtancy/dairy (7740 files) 0 errors.

## Consumer Upgrade Notes

Impact class **none**. tsconfig and dependency changes are underlay-internal; the
`EntityListItemContext` export is additive; snippet param types are internal to
underlay's own templates. No consumer-visible surface changed.

## Next

Lane D complete. `g08.025` (Lane E) front-door doc repair.
