# g06.150 Artifact - Template Types Modularity Audit

## Summary

`ts/src/templates/template.types.ts` is the top remaining high-severity
TypeScript source god-file after `g06.149`.

The file is type-only shared template surface. It currently groups:

- snippet/surface primitives
- filter, sort, list capability, and paged-list contracts
- batch, inline dialog, item action, and reorder contracts
- detail page and detail item contracts
- system index, dashboard, error-log, job, scheduled-task, audit-log, and media
  contracts
- Poodle table/log adapter contracts
- `EntityListSharedProps`

## Public Export Evidence

Public path:

- `ts/src/templates/index.ts` re-exports selected names from
  `./template.types`

Internal template consumers import directly from:

- `ts/src/templates/template.types.ts`

The split should keep both surfaces stable.

## Export Families

Shared primitives:

- `TemplateSurface`
- `FetchFn`
- `PagedListResult`

List and filtering:

- `TemplateFilterOption`
- `TemplateSortField`
- `FilterConfig`
- `ListVariantDefinition`
- `ListFilterDefinition`
- `ListCapabilities`
- `EntityListDataLoader`
- `EntityListCapabilitiesLoader`

Actions, dialogs, and reorder:

- `BatchDialogContext`
- `BatchDialogConfig`
- `InlineListDialogContext`
- `InlineListDialogConfig`
- `InlineListItemActionConfig`
- `InlineListItemDeleteConfig`
- `ReorderActionState`
- `BatchActionConfirm`
- `BatchActionConfig`
- `InlineReorderConfig`
- `LoadedReorderConfig`
- `CustomReorderConfig`
- `ReorderConfig`
- `ReorderErrorResult`

Detail and system index:

- `DetailMetaItemConfig`
- `DetailItemConfig`
- `DetailTabConfig`
- `DetailActionConfirm`
- `DetailActionConfig`
- `SystemIndexCardConfig`
- `AdminDashboardSectionConfig`

Error logs:

- `ErrorLogListRequest`
- `ErrorLogListItem`
- `ErrorLogDetailItem`
- `ErrorLogStatsSummary`
- `ErrorLogListLoader`
- `ErrorLogDetailLoader`
- `ErrorLogStatsLoader`

System jobs and scheduled tasks:

- `SystemJobStatus`
- `SystemJobListItem`
- `SystemJobDetailItem`
- `SystemJobStatsSummary`
- `SystemJobListRequest`
- `SystemJobListLoader`
- `SystemJobStatsLoader`
- `SystemJobAction`
- `SystemJobDetailLoader`
- `SystemScheduledTaskListItem`
- `SystemScheduledTaskDetailItem`
- `SystemScheduledTaskListRequest`
- `SystemScheduledTaskListLoader`
- `SystemScheduledTaskAction`
- `SystemScheduledTaskDetailLoader`
- `SystemScheduledTaskJobRunsLoader`

System audit and media:

- `SystemAuditActor`
- `SystemAuditLogEntry`
- `SystemAuditLogListRequest`
- `SystemAuditLogListLoader`
- `SystemMediaTrashItem`
- `SystemMediaTrashListLoader`
- `SystemMediaTrashAction`
- `MediaPickerWorkflowItem`
- `MediaPickerBrowseItem`
- `MediaActionsMenuItem`
- `MediaListPageItem`
- `MediaVersionListItem`
- `MediaUsageListItem`

Entity list adapters:

- `TableRowActionFactory`
- `LogEntryMapper`
- `LogActionTypeResolver`
- `LogActionFormatter`
- `LogResourceTypeFormatter`
- `LogActorHrefResolver`
- `LogResourceHrefResolver`
- `EntityListSharedProps`

## In-Repo Consumers

Public barrel:

- `ts/src/templates/index.ts`

Template consumers include:

- `EntityList.svelte`
- `EntityInlineListModule.svelte`
- `EntityDetailPage.svelte`
- `EntityDetail.svelte`
- `EntityAttributeList.svelte`
- `EntityActionsMenu.svelte`
- `EntityTrashPage.svelte`
- `AdminDashboardPage.svelte`
- `ErrorLogListPage.svelte`
- `ErrorLogDetailPage.svelte`
- system job, scheduled task, audit, media, and media detail workflow templates

## Type Boundaries To Preserve

The split must preserve:

- `TemplateSurface` permissive linked-workspace snippet compatibility
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

## Split Plan

Suggested module shape:

- `template.types.ts`: public barrel
- `template-types/primitives.ts`: `TemplateSurface`, `FetchFn`,
  `PagedListResult`
- `template-types/list.ts`: filters, variants, capabilities, entity list
  loaders
- `template-types/actions.ts`: batch, inline dialog, item action, and reorder
  contracts
- `template-types/detail.ts`: detail meta/item/tab/action contracts
- `template-types/system.ts`: system index, dashboard, jobs, scheduled tasks,
  audit logs, and error logs
- `template-types/media.ts`: media picker, list, versions, usage, trash types
- `template-types/entity-list.ts`: table/log adapter types and
  `EntityListSharedProps`

Keep `ts/src/templates/index.ts` unchanged unless type-only re-export paths must
be adjusted by the compiler.

## Validation Evidence

Passed:

- `effigy check:types`

Attempted but blocked by existing test-environment issue:

- `effigy test:components ts/tests/templates/entity-templates.component.test.ts`
  fails before tests run because Vite cannot resolve `$app/navigation` from
  `ts/src/client/navigation.ts`

Doctor:

- `effigy doctor` still fails on standing structural scans
- god-file findings are `16` total, `2` high
- top high source god-file is `ts/src/templates/template.types.ts`

## Public API Impact

Expected impact: none.

If the split requires changing exported names, direct `./template.types`
imports, or `templates/index.ts` public exports, stop and re-enter planning.

## Decision

Queue `g06.151` as a mechanical template types internal split.
