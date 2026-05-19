# Contract: Admin Template System

Status: active
Owner: repo maintainers
Depends on: `090-ts-runtime-and-client-orchestration.md`, `100-shared-patterns-and-workflow-shells.md`, `115-admin-resource-api-shapes.md`

## Purpose

Define the shared admin template system contract Underlay owns across:

- Level 1 page shells for list, detail, and form pages
- Level 1 trash workflow page shells
- retained media workflow page shells
- Level 2 section components reusable inside pages, tabs, and dialogs
- the retained `EntityListCard` composition surface
- the declarative extension model proven by the current `g03` rollout

This contract does not redefine Poodle primitives, and it does not reopen the
consumer rollout work in `g03`. It fixes the stable template boundary those
rollouts are exercising.

## Sources of Truth

Primary:

- [`ts/src/templates/index.ts`](/Users/tom/Dev/projects/underlay/ts/src/templates/index.ts)
- [`ts/src/templates/EntityListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListPage.svelte)
- [`ts/src/templates/EntityList.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityList.svelte)
- [`ts/src/templates/EntityListCard.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityListCard.svelte)
- [`ts/src/templates/entity-list-card.types.ts`](/Users/tom/Dev/projects/underlay/ts/src/templates/entity-list-card.types.ts)
- [`ts/src/templates/EntityDetailPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailPage.svelte)
- [`ts/src/templates/EntityDetail.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetail.svelte)
- [`ts/src/templates/EntityDetailModule.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityDetailModule.svelte)
- [`ts/src/templates/EntityInlineListModule.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityInlineListModule.svelte)
- [`ts/src/templates/EntityAttributeList.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityAttributeList.svelte)
- [`ts/src/templates/EntityFormPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/EntityFormPage.svelte)
- [`ts/src/templates/MediaUploadPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaUploadPage.svelte)
- [`ts/src/templates/MediaUploadWorkflowPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaUploadWorkflowPage.svelte)
- [`ts/src/templates/MediaUploadStatusPanel.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaUploadStatusPanel.svelte)
- [`ts/src/templates/MediaReplaceFileForm.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaReplaceFileForm.svelte)
- [`ts/src/templates/MediaBrowsePanel.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaBrowsePanel.svelte)
- [`ts/src/templates/MediaActionsMenu.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaActionsMenu.svelte)
- [`ts/src/templates/MediaListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaListPage.svelte)
- [`ts/src/templates/MediaListCard.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaListCard.svelte)
- [`ts/src/templates/MediaFileDetailsCard.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaFileDetailsCard.svelte)
- [`ts/src/templates/MediaEditDialog.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaEditDialog.svelte)
- [`ts/src/templates/MediaPreviewTab.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaPreviewTab.svelte)
- [`ts/src/templates/MediaRenditionsSection.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaRenditionsSection.svelte)
- [`ts/src/templates/MediaVersionActionDialogs.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaVersionActionDialogs.svelte)
- [`ts/src/templates/MediaVersionPreviewDialog.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaVersionPreviewDialog.svelte)
- [`ts/src/templates/MediaVersionsList.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaVersionsList.svelte)
- [`ts/src/templates/MediaUsageList.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaUsageList.svelte)
- [`ts/src/templates/MediaPickerWorkflow.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaPickerWorkflow.svelte)
- [`ts/src/templates/MediaDetailWorkflowPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/MediaDetailWorkflowPage.svelte)
- [`ts/src/templates/SystemIndexPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemIndexPage.svelte)
- [`ts/src/templates/SystemAuditLogListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemAuditLogListPage.svelte)
- [`ts/src/templates/SystemJobDetailPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemJobDetailPage.svelte)
- [`ts/src/templates/SystemJobListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemJobListPage.svelte)
- [`ts/src/templates/SystemMediaTrashListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemMediaTrashListPage.svelte)
- [`ts/src/templates/SystemMediaTrashListCard.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemMediaTrashListCard.svelte)
- [`ts/src/templates/SystemScheduledTaskDetailPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemScheduledTaskDetailPage.svelte)
- [`ts/src/templates/SystemScheduledTasksListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemScheduledTasksListPage.svelte)
- [`ts/src/templates/SystemScheduledTaskListCard.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/SystemScheduledTaskListCard.svelte)
- [`ts/src/templates/AdminDashboardPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/AdminDashboardPage.svelte)
- [`ts/src/templates/ErrorLogListPage.svelte`](/Users/tom/Dev/projects/underlay/ts/src/templates/ErrorLogListPage.svelte)

Primary docs:

- [`docs/usage/templates/000-template-system-overview.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/000-template-system-overview.md)
- [`docs/usage/templates/entity-list-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-page.md)
- [`docs/usage/templates/entity-detail-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-page.md)
- [`docs/usage/templates/entity-form-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-form-page.md)
- [`docs/usage/templates/media-upload-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/media-upload-page.md)
- [`docs/usage/templates/media-detail-workflow-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/media-detail-workflow-page.md)
- [`docs/usage/templates/system-index-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/system-index-page.md)
- [`docs/usage/templates/admin-dashboard-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/admin-dashboard-page.md)
- [`docs/usage/templates/template-api-reference.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/template-api-reference.md)
- [`docs/usage/templates/consumer-rollout.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/consumer-rollout.md)

Rollout evidence:

- [`docs/roadmaps/g03/001-template-system-generation-rollover.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g03/001-template-system-generation-rollover.md)
- [`docs/roadmaps/g03/006-acme-admin-list-page-proof.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g03/006-acme-admin-list-page-proof.md)
- [`docs/roadmaps/g03/009-acme-admin-detail-page-proof.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g03/009-acme-admin-detail-page-proof.md)
- [`docs/roadmaps/g03/010-dairy-complex-validation.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g03/010-dairy-complex-validation.md)
- [`docs/roadmaps/g03/013-entity-form-templates.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g03/013-entity-form-templates.md)

If these diverge, the shared code wins.

## Contract Goal

Underlay should provide one reusable higher-order admin page system with clear
seams:

- common list, detail, and form page shapes become declarative configuration
  instead of repeated 300–800 line compositions
- templates own page/section orchestration, not primitive UI
- complex real-world pages can still escape back to direct composition without
  fighting the system

The goal is stable page-shape acceleration, not total admin-page abstraction.

## Shared Boundary

### Three-level composition model

The template system is defined by a fixed three-level hierarchy.

Levels:

- Level 1: page shells
  - `EntityListPage`
  - `EntityDetailPage`
  - `EntityFormPage`
  - `EntityTrashPage`
  - `MediaUploadPage`
  - `MediaUploadWorkflowPage`
  - `MediaPickerWorkflow`
  - `MediaDetailWorkflowPage`
  - `SystemIndexPage`
  - `SystemAuditLogListPage`
  - `SystemJobDetailPage`
  - `SystemJobListPage`
  - `SystemMediaTrashListPage`
  - `SystemScheduledTaskDetailPage`
  - `SystemScheduledTasksListPage`
  - `AdminDashboardPage`
  - `ErrorLogListPage`
- Level 2: reusable sections
  - `EntityList`
  - `EntityDetail`
  - `EntityDetailModule`
  - `EntityInlineListModule`
  - `EntityAttributeList`
- Level 2.5: entity composition helper
  - `EntityListCard`
  - `MediaListCard`
- Level 3: Poodle primitives

Rules:

- page shells compose sections plus page header/action/loading state
- sections are public and reusable inside tabs, dialogs, and nested surfaces
- Poodle owns the primitive visual layer
- templates must stay higher-order composition, not become a second primitive
  kit

### List template seam

`EntityListPage` plus `EntityList` define the browse/list template contract.

Core pieces:

- `EntityListPage`
- `EntityList`
- declarative filter config
- declarative query variant config
- batch action config
- reorder config
- card/table/log presentation modes
- URL-synced `query` / `onQueryChange` seam
- `beforeList`, `headerLeadingActions`, and `headerActions` extension slots

Rules:

- `EntityListPage` is the page-level shell around `EntityList`
- `EntityListPage` is the preferred shell for real browse/manage list surfaces,
  including lists shown inside detail tabs
- in app consumers, those real list surfaces should normally live in reusable
  app-local wrapper components such as `src/lib/lists/*` rather than being
  declared directly in route files
- root routes should thin-mount those wrappers
- detail tabs should reuse the same wrapper when the collection semantics are
  the same and the difference is only scope, query mode, or a small feature
  toggle
- route-local `EntityListPage` composition is acceptable only for:
  - explicit workflow exceptions
  - narrow proof spikes that are expected to collapse into a wrapper
- `EntityList` is the lower-level list engine and should normally sit underneath
  `EntityListPage`, not compete with it as a peer default
- there is no separate retained `EntityTabList` surface; tab browse surfaces
  should use `EntityListPage` with tab-appropriate shell options unless a
  narrower utility/embed posture is genuinely needed
- data loading is caller-provided through `dataLoader(fetch, token, query)`
- that loader shape is governed by `115-admin-resource-api-shapes.md`
- `query.variant` represents the active named baseline query when the list
  supports variants
- query variants render above `FilterToolbar`; filters remain temporary
  refinements layered on top
- static `queryVariants` and `filters` may be replaced by
  `capabilitiesLoader(fetch, token)` when the API publishes `profile=list-config`
  capabilities
- presentation may be cards, table, or log
- filter, batch, and reorder behavior is declarative where it fits
- the parent may own URL query state, while the template owns list interaction
  behavior

### Child-tab migration acceptance

Moving a child tab onto the shared list templates is allowed only when it
preserves the real behavior of the existing surface.

Preferred target:

- `EntityListPage` for real child-collection browse/manage tabs
- `EntityList` only for narrower inline/embed utility lists where a page shell
  would be artificial

Required preservation:

- same user-facing actions still exist
- same search/filter/reload behavior still exists
- same delete, batch, or reorder behavior still exists where relevant
- same navigation/context behavior still exists
- no hidden downgrade from server-backed behavior to weaker local heuristics
  unless the route already had that posture

Allowed outcomes:

- move to `EntityListPage` when the tab is really a child-collection
  browse/manage surface
- use `EntityList` only when the surface is truly inline, subordinate, or
  utility-like rather than a real browse surface
- keep a compatibility wrapper when the behavior still fits the route contract
  but not the shared shell cleanly
- grow the shared template surface deliberately when multiple consumers need
  the same extra behavior

Disallowed posture:

- forcing a tab onto `EntityList` or `EntityListPage` by dropping meaningful
  behavior
- treating `variant="tab"` removal as success by itself

Typical reasons to keep a compatibility wrapper for now:

- the same component is still shared across root-page and detail-tab contexts
  and `EntityListPage` does not yet expose one small shell seam needed in both
  places
- it still depends on cursor-style runtime pagination rather than the page-list
  bridge in `115`
- it carries reorder, batch-transform, or other workflow behavior that the
  shared shell does not yet express cleanly

If multiple consumer wrappers share that same mixed capability set, treat that
as pressure to extract another retained shared collection shape. Do not keep
calling each wrapper "too complicated" forever.

### List-card composition seam

`EntityListCard` is the stable card composition helper for card-mode lists.

Core pieces:

- `EntityListCard`
- `MediaListCard`
- `EntityListCardProps`
- badge/counter/mode-display types

Rules:

- `EntityListCard` wraps Poodle `ListCard` with Underlay-specific mode and
  content conventions
- the default leading posture is the larger rounded-square shell used by media
  thumbs; circular leading visuals are opt-in
- card display may adapt for reorder or selection mode through the mode-display
  overrides
- reference-grade admin apps should treat `EntityListCard` as the required
  retained shell for repeated browse/manage collection cards
- real admin card actions should normally use the leading visual as the menu
  trigger through `contextMenuTrigger="leading"`
- right-click context menus remain available through
  `contextMenuTrigger="context"` when a consumer explicitly wants that posture
- apps still own entity-specific card content choices and callbacks
- media browse cards should use `MediaListCard`, not repeated app-local
  thumbnail/action/menu card implementations
- raw Poodle `ListCard` remains acceptable only for explicit exceptions:
  non-admin surfaces, one-off workflow cards, or subordinate embeds where the
  `EntityListCard` posture would be artificial

### Detail template seam

`EntityDetailPage` plus `EntityDetail` define the read-only detail template
contract.

Core pieces:

- `EntityDetailPage`
- `EntityDetail`
- `EntityDetailModule`
- `EntityInlineListModule`
- `EntityAttributeList`

Rules:

- `EntityDetailPage` owns header, breadcrumbs, metadata bar, top-level tabs,
  load state, and page actions
- `EntityDetailPage` supports both:
  - `tabs` for real top-level section splits
  - `content` for single-surface detail pages that still need the shared
    header/meta/action shell
- detail routes may use `EntityDetailPage` directly; unlike list pages, they do
  not need app-local wrapper components by default unless the same detail shell
  is truly reused across more than one caller
- `EntityDetailPage` may take either:
  - `dataLoader` when the template should own the fetch/load/error posture
  - `item` when the route already owns a stitched authenticated detail fetch
- `EntityDetail` is the reusable section shell for detail modules
- detail modules provide the framed content grid units used inside sections
- top-level tabs may use card or underline posture and may stay mounted when
  route-owned local tab state should persist
- fake one-tab detail layouts are not the intended posture; use `content`
  instead when there is no real top-level tab split
- child collections should normally use `EntityListPage`
- `EntityList` remains the narrower raw list engine for picker-like or utility
  embeds
- `EntityInlineListModule` is the retained compact child-collection module for
  detail-grid surfaces that still need managed behavior such as modal add,
  per-item actions, and compact pagination without becoming a full browse page
- child collection tabs should use the canonical child-list API shape from
  `115-admin-resource-api-shapes.md`
- the detail template supports nested list/detail compositions without forcing
  the whole page into one monolith

### Trash template seam

`EntityTrashPage` defines the shared page-shell contract for admin trash
surfaces.

Core pieces:

- `EntityTrashPage`
- app-owned trash item cards, usually over `EntityListCard`
- route-owned restore and purge workflow logic

Rules:

- `EntityTrashPage` owns the repeated outer shell:
  - page header
  - loading state
  - error state
  - empty state
  - grid of trash cards or delete-batch cards
- trash routes still own restore logic, purge confirmation, and any conflict
  resolution workflow
- trash routes are not normal browse/manage collections and should not be
  forced into `EntityListPage` when the main behavior is restore/purge workflow
- repeated trash cards should still prefer `EntityListCard` when they are real
  card-shaped collection items
- trash pages may use `beforeItems` for local search/sort or other trash-local
  controls without falling back to `EntityListPage`

### Form template seam

`EntityFormPage` is intentionally only a page shell.

Core piece:

- `EntityFormPage`

Rules:

- there is no declarative `EntityForm`
- forms stop at the page-shell boundary because real forms have arbitrary
  layout, custom fields, conditional logic, uploads, rich text, and custom
  validation
- `EntityFormPage` owns header, loading, error/success state, spacing, and one
  optional sidecar content region for pages that still have one clear primary
  form plus one supporting panel
- apps bring the actual `<form>` markup and field logic with Poodle primitives
- create/edit routes may use `EntityFormPage` directly, but repeated field
  bodies should live in app-local form components when a form serves more than
  one caller
- if multiple consumer routes need a broader multi-surface form shape than the
  retained sidecar seam, promote that shape deliberately instead of forcing it
  through ad hoc route composition

### Media upload workflow seam

`MediaUploadWorkflowPage` is the retained shared upload and replace workflow
for admin media-library routes. `MediaUploadPage` remains the lower-level page
framing shell.

Core pieces:

- `MediaUploadPage`
- `MediaUploadWorkflowPage`
- `MediaUploadStatusPanel`
- `MediaReplaceFileForm`
- `MediaBrowsePanel`
- `MediaActionsMenu`
- `MediaListPage`
- `MediaFileDetailsCard`
- `MediaEditDialog`
- `MediaPreviewTab`
- `MediaRenditionsSection`
- `MediaVersionActionDialogs`
- `MediaVersionPreviewDialog`
- `MediaVersionsList`
- `MediaUsageList`
- `MediaPickerWorkflow`
- `runtime/media` workflow helpers such as `runMediaUploadWorkflow`,
  `createMediaUploadPipeline`, `createMediaAndUpload`, `replaceMediaUpload`,
  `checkMediaDuplicateFile`, `loadMediaBrowsePage`, and
  `mergeMediaBrowseItems`

Rules:

- `MediaUploadPage` owns the repeated upload-page framing:
  - page header and back-link
  - optional intro region
  - loading state
  - upload-level error callout
- `MediaUploadWorkflowPage` owns the repeated workflow behavior:
  - file validation messaging
  - upload queue state
  - duplicate handling
  - retry and upload-anyway actions
  - progress display
  - replace-mode layout
- routes provide API-client callbacks for duplicate checks, create/upload,
  replace/upload, navigation, and toasts
- app-local upload pipelines should only bind generated API-client calls,
  auth/fetch inputs, and any real client-specific request difference
- when multiple consumers share the same upload wrapper shape, app-local
  pipelines should prefer `runtime/media` `createMediaUploadPipeline()` rather
  than re-wrapping `createMediaAndUpload()` and `replaceMediaUpload()`
- app-local upload routes should thin-mount `MediaUploadWorkflowPage`
- app-local media picker components should thin-mount `MediaPickerWorkflow`
- detail-page replace-file dialogs should use `MediaReplaceFileForm` instead
  of carrying app-local file validation and progress UI
- detail-page media action menus should use `MediaActionsMenu`; apps bind only
  generated delete/restore/purge commands and replace navigation
- media detail file-details cards should use `MediaFileDetailsCard`; apps bind
  only media data and file-size formatting
- media detail edit dialogs should use `MediaEditDialog`; apps bind only route
  submit wiring, field values, and visibility options
- media detail preview tabs should use `MediaPreviewTab`; apps bind only the
  resolved preview URL plus media-kind predicates
- media detail renditions blocks should use `MediaRenditionsSection`; apps bind
  renditions data, file-size formatting, and any optional generate-renditions
  action state
- media detail activate/delete dialogs should use `MediaVersionActionDialogs`;
  apps bind only open state, selected version, and confirm callbacks
- media detail version preview dialogs should use
  `MediaVersionPreviewDialog`; apps bind only selected version and shared
  preview helpers
- media-library browse routes should use `MediaListPage`; apps bind only the
  generated list/delete/batch-delete commands and query mapping differences
- media detail version lists should use `MediaVersionsList`; apps bind only
  version lifecycle predicates and commands
- media detail usage lists should use `MediaUsageList`; apps bind only data
  loading and retry/error handling
- routes should prefer retained `runtime/media` helpers for media-detail draft
  state, version-dialog state, preview URL resolution, previewability checks,
  file-size formatting, and current/activate/delete predicates before adding
  app-local helper modules
- app-local media upload queue/status/replace sections are temporary rollout
  residue and should not define parallel behavior
- this is a workflow shell, not a generic file-upload primitive
- use it only for the repeated admin media-upload family unless another
  consumer family later proves the same retained shape honestly

### Media detail workflow seam

`MediaDetailWorkflowPage` is the retained outer shell for repeated admin
media-detail routes.

Core piece:

- `MediaDetailWorkflowPage`
- `MediaFileDetailsCard`
- `MediaEditDialog`
- `MediaPreviewTab`
- `MediaRenditionsSection`
- `MediaVersionActionDialogs`
- `MediaVersionPreviewDialog`
- `MediaVersionsList`
- `MediaUsageList`

Rules:

- `MediaDetailWorkflowPage` owns the repeated media-detail framing:
  - media header and back-link
  - metadata bar
  - deleted-state banner
  - top-level tab shell
  - loading and error posture
- it supports both:
  - `dataLoader` when the template should own the media fetch shell
  - `item` plus `loading` / `error` / `onRetry` when the route already owns
    media-detail orchestration
- routes still own:
  - action menus
  - app-specific media command wiring and refetch behavior
  - app-local media business logic beyond the retained helper surface
- the preferred lower-level retained detail surfaces under that shell are:
  - `MediaFileDetailsCard`
  - `MediaEditDialog`
  - `MediaPreviewTab`
  - `MediaRenditionsSection`
  - `MediaVersionActionDialogs`
  - `MediaVersionPreviewDialog`
  - `MediaVersionsList`
  - `MediaUsageList`
- routes should prefer retained `runtime/media` helpers for media-detail draft
  state, version-dialog state, preview URL resolution, previewability checks,
  file-size formatting, and current/activate/delete predicates before adding
  app-local helper modules
- app-local detail routes should thin-compose those retained surfaces before
  introducing new route-local media-detail modules
- this is a workflow shell, not a generic media viewer
- use it for the retained admin media-detail family unless another repeated
  consumer family later proves the same shape honestly

### System index seam

`SystemIndexPage` is the retained outer shell for repeated admin system index
routes.

Core piece:

- `SystemIndexPage`

Rules:

- it owns the repeated system-index framing:
  - page header
  - optional subtitle and back-link
  - nav-card grid
- routes still own:
  - destination list
  - card labels and descriptions
  - accent colors
  - icon choices
  - any extra helper content above the grid
- this is an operator index shell, not a dashboard and not a generic card-grid
  primitive

### System operator list seam

System operator lists are retained shared templates when the same platform
surface appears across normal Underlay apps.

Core pieces:

- `SystemJobListPage`
- `SystemJobDetailPage`
- `SystemMediaTrashListPage`
- `SystemMediaTrashListCard`
- `SystemScheduledTaskDetailPage`
- `SystemScheduledTasksListPage`
- `SystemScheduledTaskListCard`
- `SystemAuditLogListPage`
- `ErrorLogListPage`
- `ErrorLogDetailPage`

Rules:

- `/system` should use `SystemIndexPage` with its built-in core cards unless an
  app has a documented reason to replace the whole card set
- app-local `/system` tools should be passed as `extraCards`, not copied into a
  full repeated core card array
- `/system/jobs` should use `SystemJobListPage` unless an app has a documented
  operator workflow extension that the shared template cannot express
- `SystemJobListPage` owns job status query variants; app wrappers adapt
  `request.status` into their API client's status parameter
- `/system/jobs/[id]` should use `SystemJobDetailPage`; apps adapt API-specific
  job detail DTOs into `SystemJobDetailItem`
- `/system/scheduled-tasks` should use `SystemScheduledTasksListPage` and
  `SystemScheduledTaskListCard`
- `/system/scheduled-tasks/[id]` should use `SystemScheduledTaskDetailPage`;
  apps adapt task and job-run DTOs into the shared detail item shapes
- `/system/errors` should use `ErrorLogListPage`
- `ErrorLogListPage` owns error status query variants; app wrappers adapt
  `request.statusClass` to `status_class=4xx|5xx` and `request.statusCode` to
  exact `status_code` filters
- `/system/errors/[id]` should use `ErrorLogDetailPage`; apps adapt API-specific
  error-log DTOs into `ErrorLogDetailItem`
- `/system/audit` should use `SystemAuditLogListPage`
- media trash pages should use `SystemMediaTrashListPage`; it owns the retained
  media-trash search/sort seam, while apps bind query state only when they need
  URL-synced filters
- app API clients stay app-local; they adapt into shared loader/action callback
  types
- retry/cancel/trigger/toggle behavior belongs behind template callbacks, not
  in repeated route-local table composition
- `/system/emails` is not a retained operator list; DB-backed email capture is
  deprecated and local inspection belongs in Mailpit via Effigy
- app-local system sections may remain app-local when they are genuine domain
  tools, such as migration, AI, or review utilities

Retained cross-app `/system` inventory:

- `/system`
- `/system/errors`
- `/system/errors/[id]`
- `/system/jobs`
- `/system/jobs/[id]`
- `/system/scheduled-tasks`
- `/system/scheduled-tasks/[id]`
- `/system/audit`

Known app-local `/system` extras:

- `underlay-reference`: `/system/poodle-gap-review`
- `acowtancy`: `/system/ai-routing`, `/system/ai-suggestions`,
  `/system/learning-transforms`

### Admin dashboard seam

`AdminDashboardPage` is the retained outer shell for repeated admin dashboard
routes.

Core piece:

- `AdminDashboardPage`

Rules:

- it owns the repeated dashboard framing:
  - page header
  - optional subtitle and back-link
  - stacked dashboard sections
- routes still own:
  - metric tiles
  - nav-card groups
  - callouts and recovery actions
  - app-specific summary widgets
- this is a dashboard shell, not a generic layout primitive
- keep the retained seam loose; do not force metric or nav-card schemas into
  Underlay unless they later prove broadly reusable

### Extension model

The stable extension model is:

- caller-provided data loaders
- caller-provided render snippets for cards, cells, expanded rows, tab content,
  and header actions
- declarative filter/batch/reorder/action config where the shared model fits
- direct fallback to section-level or primitive composition when the full page
  shell is too constraining

Rule:

- template escape hatches are part of the contract, not signs of failure
- if a page shape needs more escape hatch than template value, direct
  composition remains valid

### Valid non-template exceptions

Not every admin route should be forced onto an `Entity*` shell.

Normal exceptions:

- dashboard and overview pages
- system index pages
- media upload pages
- billing, ops, or account utility pages
- workflow-heavy planners or transform consoles
- media detail pages where the route is primarily a usage, rendition, or
  workflow surface rather than a normal entity detail shell

Also allowed:

- route-local not-found or pre-load fallback headers inside otherwise
  converged `EntityDetailPage` routes
- subordinate inner section headers inside a converged detail route

Disallowed posture:

- leaving a real entity browse/manage list, normal entity detail, or normal
  create/edit route on raw `PageHeader` plus local shell composition just
  because it existed first

## Ownership Split

Templates own:

- common admin page-shape orchestration
- page-level wiring between retained workflow controllers and Poodle primitives
- declarative configuration surfaces for common list/detail cases

Patterns/runtime own:

- the deeper workflow controllers and browser orchestration templates rely on

Poodle owns:

- visible primitives such as `PageHeader`, `DataTable`, `ListCard`, `Tabs`,
  `MetaBar`, dialogs, and loading/error states

Apps own:

- entity-specific loaders, routes, wording, permission policy, field logic,
  custom snippets, and pages that do not fit the template shape

## Invariants

- the three-level composition hierarchy stays explicit
- `EntityList` and `EntityDetail` remain public reusable sections
- list/detail templates may be nested, but forms stop at the wrapper boundary
- templates stay declarative where the common page shape is real
- rollout convenience hacks must not silently become permanent public API

## Known Drift To Assess Later

- template docs still carry stale “in development” status language even though
  the surfaces are active and consumer rollouts are well underway
- the rollout docs and some roadmap notes no longer match current reality after
  broader consumer migration work moved beyond the older `consumer-rollout.md`
  snapshot
- the stable config types are mostly embedded inside component files rather
  than exported as a cleaner dedicated public type surface
- `EntityList` carries a large amount of the real template complexity, which
  may mean the page-shell/section split is thin in practice
- `g03.010` records several complex-shape gaps from Dairy, and some of those
  enhancements may have been solved ad hoc during consumer rollout without the
  docs and contract being fully refreshed

## Assessment Questions

- does the template system now provide a stable enough declarative extension
  model, or is `EntityList` still too rollout-shaped and internally broad
- which current template enhancements are genuine shared admin abstractions
  versus consumer-specific compatibility glue
- should more of the config surface become exported first-class types instead
  of living only in component internals and docs
- does the current split between templates and retained workflow controllers
  still produce the right amount of leverage in real consumer pages

## Next Task

Use [../roadmaps/g04/032-template-docs-and-public-type-authority-repair.md](/Users/tom/Dev/projects/underlay/docs/roadmaps/g04/032-template-docs-and-public-type-authority-repair.md)
to repair the template docs and public type authority.
