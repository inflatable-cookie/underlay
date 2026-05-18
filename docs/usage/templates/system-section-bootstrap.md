# System Section Bootstrap

Status: active

Use this when starting a new Underlay-based app and you want the standard
admin `/system` surface without rebuilding it route by route.

This is the shared core:

- `/system`
- `/system/errors`
- `/system/errors/[id]`
- `/system/jobs`
- `/system/jobs/[id]`
- `/system/scheduled-tasks`
- `/system/scheduled-tasks/[id]`
- `/system/audit`

Optional shared add-on:

- `/system/media/trash`

## Rule

Use thin app-local route adapters over the shared templates.

Do not fork the shared pages just to change:

- copy
- back-link targets
- accent colors
- icon choices
- a small amount of extra index content

Keep these app-local:

- command/client wiring
- route `load` functions
- app-specific labels and links
- app-specific extra system tools

## Minimal Route Set

### 1. System index

Use `SystemIndexPage`.

Bring:

- the card list
- app-specific copy
- icon snippets
- any `beforeCards` helper content

Use `extraCards` style app-local additions by simply adding more cards to the
consumer route. Shared core cards should stay present across apps.

### 2. Error log list

Use `ErrorLogListPage`.

Bring:

- the app client command used to list errors
- navigation to error detail
- any app-local page copy

### 3. Error log detail

Use `ErrorLogDetailPage`.

Bring:

- the route param lookup
- the loader that fetches one error record
- app-local back-link and any small copy differences

### 4. Job list

Use `SystemJobListPage`.

Bring:

- the list command
- retry/cancel handlers if your app exposes them
- app-local copy and navigation

### 5. Job detail

Use `SystemJobDetailPage`.

Bring:

- the job detail loader
- any app-local actions
- app-local back-link

### 6. Scheduled task list

Use `SystemScheduledTasksListPage`.

Bring:

- the list command
- any run-now or toggle handlers your app exposes
- app-local copy and navigation

### 7. Scheduled task detail

Use `SystemScheduledTaskDetailPage`.

Bring:

- the detail loader
- any task actions
- app-local back-link

### 8. Audit log

Use `SystemAuditLogListPage`.

Bring:

- the audit list command
- any app-local filter defaults
- app-local copy

### 9. Media trash when relevant

Use `SystemMediaTrashListPage` only if the app exposes media soft-delete
recovery in `/system`.

Bring:

- the trashed-media list command
- restore and purge handlers
- app-local empty-state copy if needed

## Recommended Shape

Keep each route small:

1. `+page.ts` or `+page.server.ts` loads route context and IDs
2. `+page.svelte` mounts the shared page template
3. app-local commands stay in the consumer app client layer

If the same route setup repeats inside one app, add a tiny app-local wrapper in
`src/lib/system/` or equivalent. Do not move app-specific behavior back into
Underlay unless multiple apps need the same seam.

## Index Card Baseline

Most apps should ship these cards on `/system`:

- Error log
- Job queue
- Scheduled tasks
- Audit log

Optional shared/system-adjacent cards:

- Media trash

App-local extras can sit beside them. Do not remove the shared core cards
without a real product reason.

## What Not To Rebuild

Do not hand-roll local versions of:

- the system index header and card grid
- error log browse/detail framing
- job browse/detail framing
- scheduled task browse/detail framing
- audit log browse framing

If you need a real new shared seam, add it in Underlay and update the contract
instead of cloning one app's route.

## Build Order

Recommended order for a new app:

1. `/system`
2. `/system/errors`
3. `/system/errors/[id]`
4. `/system/jobs`
5. `/system/jobs/[id]`
6. `/system/scheduled-tasks`
7. `/system/scheduled-tasks/[id]`
8. `/system/audit`
9. `/system/media/trash` if needed

## See Also

- [Template System Overview](./000-template-system-overview.md)
- [System Index Page](./system-index-page.md)
- [Template API Reference](./template-api-reference.md)
- [Admin Template System Contract](../../contracts/110-admin-template-system.md)
