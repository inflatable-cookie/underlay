# g06.151 Artifact - Template Types Internal Split

## Summary

`ts/src/templates/template.types.ts` is now a type-only public barrel. The
implementation moved under `ts/src/templates/template-types/` by stable type
family.

Module shape:

- `template.types.ts`: public type front door
- `template-types/primitives.ts`: `TemplateSurface`, `FetchFn`,
  `PagedListResult`
- `template-types/list.ts`: filters, variants, list capabilities, and entity
  list loaders
- `template-types/actions.ts`: batch actions, inline dialogs, item actions, and
  reorder contracts
- `template-types/detail.ts`: detail meta, item, tab, and action contracts
- `template-types/system.ts`: system index, dashboard, error-log, job,
  scheduled-task, and audit-log contracts
- `template-types/media.ts`: media picker, list, version, usage, and trash
  contracts
- `template-types/entity-list.ts`: Poodle table/log adapters and
  `EntityListSharedProps`

## Public API Impact

None expected.

The retained public front doors still export the same template type names:

- `ts/src/templates/template.types.ts`
- `ts/src/templates/index.ts`

No consumer app import changes are required.

## Type Boundaries Preserved

The split preserves the audited boundaries:

- permissive `TemplateSurface` linked-workspace snippet compatibility
- `FetchFn`, `QueryParams`, and `PagedListResult` loader signatures
- filter/sort/load-options context shapes
- list capability and query-variant contracts
- batch action confirm/dialog semantics
- reorder strategy discriminants: `inline`, `loaded`, `custom`
- detail item/meta/tab/action shape
- system job and scheduled task generic loader/action defaults
- media item shapes used by media templates
- Poodle table/log adapter types
- `EntityListSharedProps` as the aggregate list component prop contract

## Validation

Passed:

- `effigy check:types`

Not rerun:

- component template validation remains blocked by the existing
  `$app/navigation` resolution issue captured in `g06.150`

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are now `15` total, `1` high
- `ts/src/templates/template.types.ts` is no longer a high-severity god-file
- no replacement template type module appears in the god-file warning list
- remaining high god-file:
  - `ts/tests/patterns/optimistic.test.ts`

## Decision

Queue `g06.152` as an optimistic tests modularity audit. It is the only
remaining high-severity god-file.
