# 045 - Remaining Underlay Surface Contraction

Status: Complete
Owner: Platform
Created: 2026-03-26
Depends on: 042, 044

## Overview

`g01.042` successfully moved the big primitive and generic composite wave into
Poodle, but the live Underlay public export list is still broader than the
settled retained boundary. A meaningful tail of generic card, detail, layout,
status, and list helpers still ships from `@decodelabs/underlay/components` and
`@decodelabs/underlay/patterns` even though they now have clear Poodle
successors or should be reduced to local composition.

This roadmap finishes that contraction wave. The goal is not to reopen the big
Poodle migration from scratch; it is to remove the remaining low-value public
surface that survived because the earlier wave stopped at the right time for
workflow shells, not because those generic helpers still earn long-term
Underlay ownership.

## Research Basis

- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`
- `docs/roadmaps/g01/042-poodle-adoption-and-underlay-ui-contraction.md`
- `docs/roadmaps/g01/044-shared-ui-documentation-and-demo-refresh.md`

## Decision Summary

- The remaining public Underlay UI surface should now be treated as two groups,
  not one:
  - final retained workflow shells and auth/media helpers that still earn
    Underlay ownership
  - generic export residue that should either move to direct Poodle usage or be
    internalized into callers
- The easiest and highest-value deletions were the generic detail/layout
  wrapper family and adjacent stat/list shells:
  - `DetailsCard`, `DetailItem`, `DetailsSection`, `DetailList`, `DetailItem`
  - `ContainerGrid`
  - `InlineActionGroup`
  - `StatCard`, `StatGrid`
  - `CardActions`
- Direct same-name or near-direct Poodle replacement should be preferred over
  adding any new Underlay compatibility layer.
- A surface should remain public in Underlay only when it still owns meaningful
  workflow orchestration, auth policy, media-library behavior, or structural
  shell responsibility.

## Likely Implementation Surface

- `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- `ts/src/components/index.ts`
- `ts/src/patterns/index.ts`
- active guides that still teach the leftover generic Underlay surface
- active consumer call sites in `underlay-reference/` and `contact-patch/`

## Batch 45.1 - Strict Reassessment and Execution Queue

- [x] Re-audit the live public export surface instead of relying only on the
      already-completed `g01.042` retirements.
- [x] Record the final retained Underlay surface separately from the generic
      export residue that still needs contraction.
- [x] Open a real execution roadmap for the remaining export tail instead of
      continuing to fold it into `g01.042`.
- [x] Refresh the roadmap front doors so they no longer present `g01.044` as
      the active wave.

Completed in 45.1:
- the retained-vs-replace reassessment is now recorded in
  `contracts/ui/poodle-adoption-underlay-surface-groups.json`
- the remaining easy generic wrapper family is explicitly queued instead of
  hiding inside the broader historical migration inventory
- `g01.045` is now the active wave after `g01.044` completion

## Batch 45.2 - Easy Generic Wrapper Contraction

- [x] Remove or migrate the generic detail/layout wrapper family:
  - `DetailsCard`, `DetailItem`, `DetailsSection`, `DetailList`, `DetailItem`
  - `ContainerGrid`
- [x] Remove or migrate the simplest adjacent thin composition helpers:
  - `InlineActionGroup`
  - `CardActions`
- [x] Update the heaviest live callers in shared Underlay and the reference apps
      in one grouped pass instead of one page at a time.
- [x] Remove the public exports once the callers are moved.

Completed so far in 45.2:
- `DetailsCard`, `DetailItem`, `DetailsSection`, `DetailList`, `DetailItem`,
  `ContainerGrid`, `InlineActionGroup`, and `CardActions` are now gone from the
  public Underlay surface
- the live detail/account/system callers in `underlay-reference/acme-admin` and
  `contact-patch/cp-admin` now compose over Poodle `Card`, `DetailRow`, and
  `DetailSection`
- the next remaining generic tail from this batch is the media-oriented
  `InlineListCard` / `InlineListItem` family rather than the older detail
  wrapper cluster
- the later contract review showed that `InlineListCard` / `InlineListItem`
  are not honest same-name replacement work after all; they remain deliberate
  retained holds until Poodle grows a reusable titled list-card and interactive
  row contract

## Batch 45.3 - Generic List, Stat, and Navigation Tail

- [ ] Replace or retire the remaining generic list/stat/navigation wrappers that
      still ship from Underlay:
  - `Badge`
- [ ] Prefer direct Poodle usage or caller-owned composition over minting any
      new Underlay adapter layer.
- [ ] Update the retained-surface Storybook and guides once those generic
      exports are gone.

Completed so far in 45.3:
- the `NavCard` / `NavCardGrid` landing-page family in `acowtancy/dairy` is
  now on direct Poodle `NavCard` / `NavCardGrid`, including the top-level
  assessment, content, learning, system, and exams route indexes
- the last active admin system landing pages in `underlay-reference/acme-admin`
  and `contact-patch/cp-admin` are now also on direct Poodle `NavCard` /
  `NavCardGrid`, so the old Underlay `NavCard` pattern family is retired from
  the public surface
- the last live `StatCard` / `StatGrid` dashboard caller in
  `contact-patch/cp-admin` now uses direct Poodle `MetricTile` plus local
  layout and Poodle `Pill`, so the old Underlay stats family is retired from
  the public surface
- the old Underlay `Breadcrumbs` primitive is now retired from the public
  surface; the retained `PageHeader` shell already resolves through direct
  Poodle `Breadcrumbs` internally, and no live consumer apps still depended on
  the Underlay export
- the old Underlay `Card` primitive has no remaining live consumer-app
  dependency; the retained docs, examples, and Storybook catalog now use direct
  Poodle `Card` instead, so the old Underlay `Card` export is retired from the
  public surface
- the old `FilterBar` family in `acowtancy/dairy` is now on direct Poodle
  `FilterToolbar` composition rather than either Underlay `FilterBar` or a
  long-lived local compatibility wrapper
- `FilterBar` is now retired from the public Underlay pattern surface; the
  remaining `acme-admin`, `cp-admin`, and `AutonomousList` callers compose
  directly over Poodle `FilterToolbar`
- the old Underlay `FilterToolbar` component surface is now also retired from
  the public export list; no live callers remained once the last routes and
  retained shells moved to direct Poodle `FilterToolbar`
- `CheckboxChip` is now also retired from the public Underlay component
  surface; there were no live callers anywhere in the six-app portfolio
- `Drawer` and `SegmentedControl` are now retired from the public Underlay
  surface; neither had live consumer-app callers, and the remaining residue was
  limited to dedicated tests and active guide text that now points directly to
  Poodle
- `Code` is now retired from the public Underlay surface; the full live caller
  family in `acme-admin`, `cp-admin`, and retained `DetailMetaId` now resolves
  through direct Poodle `Code` with explicit `inline` / `source` usage
- the old Underlay `Badge` and `Pill` caller families are now cleared from
  Acowtancy; Poodle `Pill` now carries the shared tone and accent capability
  those callers actually needed
- the old Underlay `Switch` caller family is now cleared from live consumer
  code in `acowtancy/dairy` and `contact-patch/cp-admin`; Poodle `Switch` now
  owns the real shared successor contract for this family, including dual
  labels and state tones
- the old Underlay `Select` caller family is now cleared from live consumer
  code across the active apps; Poodle `Select` now owns the shared successor
  contract for flat options, grouped options, lazy `loadItems` / `loadGroups`,
  filter-friendly clearable resets, and the remaining callback-style caller
  semantics that were still in broad use
- the old Underlay `TimeAgo` caller family is now cleared from the active admin
  apps and Acowtancy Dairy; Poodle `TimeAgo` now owns the shared successor
  contract for compact and longer relative text plus explicit tooltip
  formatting, and the Underlay export is retired
- `InlineListCard` and `InlineListItem` remain intentionally retained for now;
  the live caller family across media detail pages and Dairy related-item cards
  still depends on a reusable titled card-list shell, accent-dot rows,
  link-or-button row activation, hover action reveal, and badge/trailing slot
  composition that Poodle does not currently package
- `DataTable` remains intentionally retained for now; current Poodle
  `DataTable` is still materially narrower than the active Underlay caller
  contract around loading rows, built-in filtering, pagination-state
  integration, row-action menus, and custom cell/extended-row rendering
- `ReorderableList` remains intentionally retained for now; the live caller
  family still depends on batch commit/cancel workflow, dirty-state handling,
  long-list warnings, optional windowed reorder mode, and richer keyboard/live
  announcements than current Poodle `ReorderableList` owns
- `ListCard` remains intentionally retained for now; the active caller family
  still depends on route-style `href` navigation, compact reorder presentation
  with drag handles, selection mode, not-live state treatment, and the current
  action-trigger composition contract
- `Pagination` is now retired from the public Underlay surface; Poodle
  `Pagination` now carries the generic controller-driven pagination contract
  the live caller family needed, including limit selector, info text,
  simple/full variants, and scroll-target behavior used across retained
  Underlay lists and active app grids
- the old Underlay `ListGrid` family is now retired from the public surface;
  the remaining app and retained-shell callers render directly through Poodle
  `Grid` with explicit auto-fit column definitions instead of a second shared
  grid wrapper
- `PageLoading` remains intentionally retained for now; the current Poodle
  `PageLoading` is a modal full-viewport overlay, while the Underlay callers in
  the admin and front apps still rely on an inline centered page-state loader
- non-exported compatibility shims now exist for the deleted deep file-path
  imports that still had broad live app usage during the transition:
  `Dialog`, `AlertDialog`, `ActionArea`, `InlineActionGroup`, `DetailsCard`,
  `DetailsSection`, `DetailItem`, `TextInput`, `CompactGroupedBar`, and
  `CompactGroupedBarGroup`
- the safety-net layer is intentionally not a public-surface reversal; the
  deleted components remain removed from `ts/src/components/index.ts`, but the
  shims keep the six-app portfolio from breaking while the remaining deep
  imports are migrated in batches
- the first direct consumer cleanup in this wave has landed in `acowtancy/dairy`
  for the surfaced dialog/detail cluster:
  `ContextPanelContent`, `MetadataJsonDialogLink`,
  `TransformActionDialog`, `TransformIncidentBundleDialog`,
  `MediaVersionPreviewDialog`, `VimeoBrowserDialog`, `MediaEditDialog`, and the
  user detail route
- the old app-level deep-import family for deleted Underlay `TextInput`,
  `FileUpload`, and `MediaThumbnail` is now cleared from live consumer code
  across `underlay-reference`, `contact-patch`, `acowtancy`, `compli-me`, and
  `songsprout`; the remaining `dairy` deep-import debt has narrowed to the
  heavier `ContentCard`, `Tabs*`, `NumberInput`, `EntityActionsMenu`,
  restore-resolution, and `SlugField` families
- the old Underlay `ContentCard` and split `Tabs*` import family is now also
  cleared from live consumer code across all six app groups; in
  `acowtancy/dairy` those callers now resolve through app-owned local
  replacements rather than Underlay deep imports, which keeps the migration
  moving without waiting on a page-by-page direct `Poodle Tabs` rewrite
- the old Underlay `NumberInput` import family is now also cleared from live
  consumer code across all six app groups; in `acowtancy/dairy` those callers
  now resolve through an app-owned local wrapper over Poodle's numeric surface
- the old Underlay `SlugField` pattern family is now also cleared from live
  consumer code across all six app groups; in `acowtancy/dairy` those callers
  now resolve through an app-owned local slug field over Poodle `TextInput` and
  the shared slug helper utilities
- the old Underlay `EntityActionsMenu` family is now also cleared from live
  consumer code across all six app groups; in `acowtancy/dairy` the local
  cards, menus, and media list now resolve through an app-owned replacement
  over retained `CopyActionsMenu` plus Poodle `AlertDialog`
- the last live consuming-app imports of Underlay `ListCard` / `ListGrid` are
  now cleared; `underlay-reference/acme-front`, `acme-admin`, and `cp-admin`
  all use direct Poodle list/grid composition, and the internal
  `AutonomousList` shell now also renders through Poodle `Grid`
- `ListCard` is now an explicit deliberate hold rather than an assumed next
  delete: the remaining live caller family still depends on Underlay-owned
  behavior that Poodle `ListCard` does not yet cover cleanly, especially
  `href` navigation, compact reorder treatment with drag handles, card-level
  selection mode, and the current action-trigger composition contract
- `FormActions` is now also an explicit deliberate hold rather than a silent
  same-name migration target: the remaining app and auth callers still depend
  on the responsive inline-danger versus collapsed-danger-menu behavior that
  Poodle `FormActions` does not currently own
- the focused `FormActions` review across retained auth shells, `acme-admin`,
  `cp-admin`, and the broad `dairy` form family confirmed that the remaining
  contract is still genuinely broader than current Poodle `FormActions`:
  responsive danger-slot collapse into a menu, `dangerItems` callback wiring,
  and mixed inline-versus-collapsed destructive action treatment all remain in
  real live use
- the focused retained `ListCard` review across `acme-admin`, `cp-admin`,
  `acme-front`, `AutonomousList`, and `ReorderableList` confirmed that the
  remaining contract is still genuinely workflow-shaped rather than just a
  stale styling wrapper: route-style navigation, compact reorder affordances,
  card-level selection behavior, and action-trigger composition all remain in
  real live use
- the old generic `ActionArea`, `DetailsCard`, `DetailItem`, `DetailsSection`,
  `ListCard`, `ListGrid`, and `Tooltip` deep-import family is now also cleared
  from live consumer code across all six app groups; in `acowtancy/dairy`
  those callers now resolve through app-owned local replacements rather than
  Underlay deep imports
- the next blocker is no longer the same missing `Dialog.svelte` import family;
  it has moved on to the broader historical `dairy` tail of removed pattern
  exports and deep imports

## Batch 45.4 - Final Public Surface Tightening

- [x] Audit the remaining exported generic component tail after Batches 45.2 and
      45.3 and internalize anything that is no longer truly public.
- [x] Confirm the final retained Underlay UI surface and update the durable
      inventory and Storybook coverage accordingly.
- [x] Close the roadmap once the public export list matches the intentional
      retained boundary closely enough that there is no obvious “generic helper”
      cleanup wave left.

## Deferred

- Reopening settled retained workflow shells such as `MediaPicker`,
  `MediaActionsMenu`, `PageHeader`, `SpaFormShell`, `FormDialog`, `LogList`,
  `BatchActionBar`, and the auth shell family without new evidence of a real
  Poodle-root opportunity.
- Poodle root capability work that does not directly unblock a live Underlay
  generic export from retirement.

## Consumer Upgrade Impact

- Expected impact class: `breaking`.
- Reference apps and any remaining consumers of the generic Underlay helper tail
  should expect import replacement and light API translation onto Poodle.
- The retained workflow-shell surface should remain stable; the breakage is
  intentionally concentrated in the leftover generic wrapper family.

## Validation

```bash
effigy qa:docs
effigy qa:northstar
```

## Current Decision

- `AudioPlayer` and `AudioEmbed` are now retired from the public Underlay
  surface. Shared audio playback should use direct Poodle `AudioPlayer`, and
  the only former `AudioEmbed` callers in Dairy now compose Poodle
  `AudioPlayer` and `EmbedPreview` directly instead of keeping an extra
  Underlay wrapper alive.
- `CardRadioGroup` is now retired from the public Underlay surface. The only
  remaining live caller in Dairy now uses direct Poodle `CardRadioGroup` with
  caller-owned icon and tone composition instead of preserving a second shared
  radio-card wrapper.
- `DateRange` is now internalized to shared formatter exports only. The
  component form had no live callers, while the formatter helpers still serve
  app list-card and schedule-label text generation.
- `DurationInput` is now retired from the public Underlay surface. The last
  live callers in Dairy now use direct Poodle `DurationInput` with caller-owned
  hidden inputs and duration-string conversion instead of preserving a second
  segmented-duration wrapper.
- `Skeleton` is now internal-only in Underlay. There are no live consumer-app
  callers left, and low-level manual placeholder composition should use direct
  Poodle `Skeleton`.
- `DataSkeleton` is now retired from the public Underlay surface. It had no
  live consumer-app or internal runtime callers left, so keeping a second
  shared loading-layout wrapper and preset registry public was just stale
  surface area. Repeated loading layouts should now compose directly from
  Poodle `Skeleton` presets in the consuming app.
- `ToastHost` remains a deliberate retained Underlay surface. Live apps still
  depend on the store-driven toast host contract, including shared subscription
  and auto-dismiss behavior for non-error toasts, while current Poodle
  `ToastStack` expects caller-owned item orchestration.
- `PageLoading` remains a deliberate retained Underlay surface. The live admin
  and front apps still rely on an inline centered page-state loader, while
  current Poodle `PageLoading` is a full-viewport modal overlay with backdrop,
  progress, and optional cancel semantics.
- `Pagination` is now retired from the public Underlay surface. The active
  caller family, including retained `AutonomousList` and Dairy list shells,
  now uses direct Poodle `Pagination`, and the generic controller-oriented
  contract remains shared through the Underlay pagination controller helpers.
- the focused `DataTable` review across `acme-admin`, `cp-admin`, retained
  `AiRoutingAdmin`, and Dairy operational views confirmed that the remaining
  contract is still genuinely broader than current Poodle `DataTable`:
  host-owned pagination state, built-in filter callbacks, richer row-action
  menus, custom cell snippets, extended-row rendering, loading rows, and the
  local column/filter model all remain in real live use
- `RangeSlider` is now retired from the public Underlay surface. The live
  Dairy caller family turned out to be a discrete labeled single-value
  difficulty scale, so those forms now use direct Poodle `Slider` with
  caller-owned labels instead of preserving a second shared Underlay wrapper
  or forcing a false `RangeSlider` parity story.
- `ColorPicker` and the old standalone `TextArea` surface are now retired from
  the public Underlay surface. Neither retained a live consumer-app caller
  family once the active forms moved to direct Poodle primitives, and multiline
  text entry now lives on Poodle `TextInput`, so keeping those wrappers
  exported was only stale surface area.
- `VideoPlayer` is now retired from the public Underlay surface. It no longer
  had live consumer-app callers, and direct shared video playback belongs to
  Poodle `VideoPlayer`.
- `MarkdownEditor` is now internal-only in Underlay. There are no live
  consumer-app callers left, but retained Nightfire internals still depend on
  the editor-context contract exposed through `markdown-editor-events` and the
  local EasyMDE-backed implementation.
- `Badge`, `Pill`, and `Switch` are now retired from the public Underlay
  surface. Their live consumer-app caller families are gone, and direct
  Poodle `Pill` and `Switch` now carry the real shared successor behavior
  without a second Underlay wrapper layer.
- `Select` is now internal-only in Underlay. There are no live consumer-app
  callers left, but retained internals such as `OrderBy`, `DataTable`, and
  Nightfire editor helpers still depend on the local richer select contract.
- `OrderBy` remains a deliberate retained Underlay surface for now. The active
  admin caller family still depends on ordered multi-field sorting with
  add/remove, per-field direction, drag reordering, compact trigger text, and
  URL round-tripping for sort arrays, while current Poodle `OrderBy` is a
  single-field toggle control.

## Completion

`g01.045` is complete. The remaining public Underlay surface now matches the
true retained boundary closely enough that there is no further honest generic
cleanup wave left inside this roadmap.

What remains is either:

- deliberate retained generic holds that still exceed current Poodle capability
  (`OrderBy`, `DropdownMenu`, `ToastHost`, `PageLoading`, `FormActions`,
  `ListCard`, `InlineListCard`, `InlineListItem`, `DataTable`)
- retained workflow, auth, media, and structural shells that still clearly
  earn Underlay ownership

Any further contraction should now open as a new focused capability project,
not another tail-cleanup sweep under `045`.

## Next Task

Open `g01.046` only for a real focused capability project. The strongest next
candidate is one of the deliberate retained generic holds such as
`FormActions`, `OrderBy`, or `DataTable`, not another tail-cleanup sweep.
