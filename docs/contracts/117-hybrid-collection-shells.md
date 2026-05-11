# Contract: Hybrid Collection Shells

Status: active
Owner: repo maintainers
Depends on: `100-shared-patterns-and-workflow-shells.md`, `110-admin-template-system.md`, `115-admin-resource-api-shapes.md`, `116-canonical-collection-routes-and-query-profiles.md`

## Purpose

Define the missing shared shape between:

- plain page-shaped child collections that fit `EntityList`
- heavier workflow composites that are not really list-shell problems

This contract exists for collection surfaces that are still real browse/manage
lists, but carry enough extra behavior that forcing them onto today's
`EntityList` would either drop capability or produce opaque app-local hacks.

Typical examples:

- root-and-tab hybrid list shells
- constrained list shells with scoped filters and page-vs-tab posture
- list shells that combine browse state with reorder, batch delete, or small
  transform-launch actions

The goal is predictable shared implementation across sites without reducing
function.

## Sources of Truth

Primary shared sources:

- [`docs/contracts/100-shared-patterns-and-workflow-shells.md`](/Users/tom/Dev/projects/underlay/docs/contracts/100-shared-patterns-and-workflow-shells.md)
- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- [`docs/contracts/115-admin-resource-api-shapes.md`](/Users/tom/Dev/projects/underlay/docs/contracts/115-admin-resource-api-shapes.md)
- [`docs/contracts/116-canonical-collection-routes-and-query-profiles.md`](/Users/tom/Dev/projects/underlay/docs/contracts/116-canonical-collection-routes-and-query-profiles.md)
- [`docs/sweeps/030-child-collection-capability-convergence-sweep.md`](/Users/tom/Dev/projects/underlay/docs/sweeps/030-child-collection-capability-convergence-sweep.md)

Reference consumer evidence:

- [`dairy/src/lib/lists/ModulesList.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/ModulesList.svelte)
- [`dairy/src/lib/lists/modules-list/ModulesFilterBar.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/modules-list/ModulesFilterBar.svelte)
- [`dairy/src/lib/lists/modules-list/ModulesHeaderActions.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/modules-list/ModulesHeaderActions.svelte)
- [`dairy/src/lib/lists/modules-list/ModulesListContent.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/modules-list/ModulesListContent.svelte)
- [`dairy/src/lib/lists/modules-list/ModulesBatchActionDialog.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/modules-list/ModulesBatchActionDialog.svelte)
- [`dairy/src/lib/lists/ActivitiesList.svelte`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/ActivitiesList.svelte)
- [`dairy/src/lib/lists/activities-list/domain-fetch.ts`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/activities-list/domain-fetch.ts)
- [`dairy/src/lib/lists/activities-list/reorder-controller.ts`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/activities-list/reorder-controller.ts)
- [`dairy/src/lib/lists/activities-list/selection-controller.svelte.ts`](/Users/tom/Dev/projects/acowtancy/dairy/src/lib/lists/activities-list/selection-controller.svelte.ts)
- [`cattle-grid/src/commands/learning/modules.ts`](/Users/tom/Dev/projects/acowtancy/cattle-grid/src/commands/learning/modules.ts)

If these diverge, the shared contract wins. Consumer wrappers are evidence, not
authority.

## Contract Goal

Underlay should support one retained shared collection-shell family for list
surfaces that sit between plain `EntityList` and full app-specific workflow
pages.

This family should cover predictable combinations such as:

- root-page and detail-tab reuse over the same resource family
- constrained scoped browsing, such as pathway-only or level-only slices
- batch selection and batch actions
- reorder sessions over the same underlying collection
- small transform-launch actions that still behave like list-level actions

The goal is not another generic everything-shell. The goal is one explicit
shared answer for a recurring hybrid capability set.

Current retained shared lower-helper surface:

- `@decodelabs/underlay/runtime/data`
- `createSelectionModeController(...)`
- `buildSelectionTransformState(...)`
- `createLocalReorderSession(...)`
- `createLoadedReorderSession(...)`

Copyable usage reference:

- [`docs/guides/097-autonomous-list-components.md`](/Users/tom/Dev/projects/underlay/docs/guides/097-autonomous-list-components.md)
- [`docs/guides/code/097-autonomous-list-components/list-workflow-helpers.ts`](/Users/tom/Dev/projects/underlay/docs/guides/code/097-autonomous-list-components/list-workflow-helpers.ts)

## Scope Boundary

In scope:

- hybrid collection shells that are still fundamentally browse/manage list
  surfaces
- root-and-tab shared list components
- list shells that combine:
  - collection browsing
  - selection/batch work
  - reorder
  - small list-level transform launches

Out of scope:

- workflow-heavy composites like activity orchestration pages
- detail pages that only contain one embedded list and otherwise fit
  `EntityList`
- full page-shell concerns already owned by `EntityListPage`
- app-specific domain dialogs or composite workspaces

If a surface is really an app-owned workflow page with multiple interacting
datasets, this contract does not try to absorb it.

## Shared Boundary

### Problem class

The hybrid collection-shell problem appears when all of these are true:

- the surface is still a list, not a bespoke workflow page
- plain `EntityList` does not express the needed behavior cleanly
- more than one consumer is likely to need the same extra behavior mix

Signals:

- root and tab modes share one component
- route/query posture differs by context
- selection, reorder, or transform actions live at list-shell level
- the component owns shell logic, not just card/table rendering

### Capability set

The retained hybrid shape may include:

- shared list content presentation
- scoped constraint inputs from the parent:
  - `pathwayId`
  - `levelId`
  - similar resource-family constraints
- optional root-shell header behavior
- optional tab-shell header behavior
- batch selection and batch actions
- reorder sessions and reorder conflict recovery
- small transform-launch actions derived from current selection

It must not assume all of these are always present. The point is one predictable
family, not one mandatory kitchen sink.

### Data posture

Preferred target:

- canonical collection route family
- converged query vocabulary from `116`
- page-shaped collection response from `115`

Compatibility allowed:

- temporary cursor-style runtime pagination where the backend still carries that
  debt explicitly

Rules:

- compatibility posture must stay explicit
- do not hide route/query divergence behind convenience props
- if a hybrid shell still depends on cursor semantics, that should be a named
  input, not an invisible fallback

### Fetch-boundary typing rule

Hybrid wrappers should narrow broad transport DTOs at the fetch boundary before
those values enter shell state.

Rules:

- if an API response carries a broader domain union than the shell actually
  owns, narrow it in the fetch layer with explicit predicates
- do not carry broad API unions through shell-local filters, reorder state, and
  presentation helpers when the shell only supports a smaller domain family
- prefer one typed shell-local entity alias over repeated casts in derived state

Typical shape:

- transport/API layer:
  broad DTO union
- fetch layer:
  explicit domain narrowing
- shell layer:
  typed local entity family such as `Outcome | Bundle | PreSeen`

This keeps the shell honest after domain splits and stops stale compatibility
branches from surviving in type signatures after the behavior has already
split.

### Relationship to `EntityListPage` and `EntityList`

`EntityListPage` remains the default answer for real browse/manage list
surfaces, including tab lists.

`EntityList` remains the lower-level engine for:

- inline utility lists
- dialog/picker lists
- subordinate embeds where page-shell chrome would be artificial

The hybrid shell family exists only when `EntityList` would otherwise need
unclear or leaky extension points.

Rules:

- do not fork card/table rendering rules unnecessarily
- prefer expanding `EntityListPage` with small honest mode props before
  introducing app-local root-vs-tab divergence
- reuse `EntityList` behavior underneath `EntityListPage` where it still fits
- extract a second retained shape only for the extra shell/workflow layer, not
  for duplicated list primitives

### Root-vs-tab split rule

The first design question for a hybrid wrapper is always:

- should this become:
  - one root shell plus one child shell
- or is there still one legitimate shared shell with context-driven behavior

Rules:

- prefer split shells when root and tab concerns are only loosely related
- keep a shared hybrid shell only when the behavior set is truly the same
  collection capability family in both contexts

### Transform action rule

Transform-launch actions are allowed only when they are still list-level
workflow, for example:

- copy selected items
- move selected items
- open a scoped transform form for current selection

Rules:

- these actions must derive from current list selection or list scope
- they must not pull the shell into a broader multi-step workspace model

## Invariants

- behavior preservation is the acceptance bar
- `EntityList` stays the default for plain child collections
- hybrid collection shells must stay explicitly smaller than workflow-heavy
  composite pages
- route/query convergence still matters even when UI shell behavior is richer
- repeated compatibility wrappers with the same capability set are pressure to
  extract a shared shape, not to tolerate drift indefinitely

## Ownership Split

Templates own:

- shared list-shell composition
- page-vs-tab shell decisions above the raw controllers
- retained shared collection-shell surfaces

Patterns own:

- lower list, pagination, selection, and reorder controllers
- reusable workflow mechanics that the shell composes

Current retained lower helpers in that layer:

- `createSelectionModeController(...)`
- `buildSelectionTransformState(...)`
- `createLocalReorderSession(...)`
- `createLoadedReorderSession(...)`

Apps own:

- resource DTOs
- route wording
- permission policy
- domain-specific transform targets

## Current extraction signal

The shared lower-helper adoption proof is now explicit in Dairy across:

- `ModulesList`
- `SectionsList`
- `AreasList`
- `OutcomesList`
- `PreSeenReleasesList`
- `BundlesList`
- `PathwaysList`
- `SyllabusUpdatesList`
- `AudiosList`
- `MediaList`

What has already converged cleanly:

- selection-mode transition rules and Escape-key exit behavior across all ten
  families
- selection-derived transform-launch state across `ModulesList`,
  `SectionsList`, `AreasList`, and `OutcomesList`
- local constrained reorder session lifecycle and conflict recovery across
  `SectionsList`, `AreasList`, and `OutcomesList`
- loaded reorder session lifecycle and conflict recovery for fetch-all reorder
  flows such as `ModulesList`

What has not converged cleanly yet:

- constrained query/filter posture
- root-vs-tab shell composition
- reorder item-shaping and presentation-specific display helpers
- heavier autonomous workflow shells such as `ActivitiesList`

`ActivitiesList` is now the clearest explicit out-of-lane case for this helper
adoption wave. It is not just another wrapper with local selection glue.

What made the original shell materially different:

- one shell originally multiplexed three domains:
  - `Outcome`
  - `PreSeen`
  - `Bundle`
- data loading is domain-specific and stitched:
  - one mixed activity fetch layer
  - outcomes/releases/topics side loads
- filtering is domain-specific and local
- pagination is local client pagination over filtered in-memory data
- reorder targets vary by domain and scope:
  - outcome
  - grouped outcome-in-area
  - topic
  - pre-seen release-in-area
- transform-launch behavior depends on both domain and current selection

Current judgment:

- do not treat `ActivitiesList` as the next easy shared-helper adoption
- do not force it onto `EntityList`
- do not read its local `selection-controller` or `reorder-controller` as proof
  that Underlay needs a second retained template shell yet
- if it moves, it should be a separate decomposition lane around:
  - domain split
  - data loading split
  - reorder target resolution
  - transform-launch state

First decomposition map:

- domain-specific data loading
  - the original mixed fetch layer branched across:
    - outcome-in-module
    - area-wide outcomes plus outcome side load
    - pre-seen area plus release side load
    - bundle topic
    - bundle-wide activities plus topic side load
- domain-specific filter/query state
  - `Outcome`
    - search
    - outcome filter
  - `PreSeen`
    - search
    - release filter
  - `Bundle`
    - search
    - topic filter
- reorder target resolution
  - outcome direct
  - grouped outcome-in-area
  - topic direct
  - pre-seen release-in-area
- transform-launch state
  - `Outcome` and `Bundle` support selection-derived copy/move targets
  - `PreSeen` does not share that transform posture

Current best next sequence:

1. split the domain-specific data loaders into explicit per-domain loaders
2. split reorder target resolution so it stops living behind one mixed
   `domain` switch
3. re-check whether `Outcome`, `PreSeen`, and `Bundle` still belong in one
   shell after those seams are explicit
4. only then decide whether any shared lower helper should move up from app
   local code into Underlay

Current implementation state:

- first four steps landed
- `ActivitiesList` now dispatches across explicit per-domain loaders:
  - `fetchOutcomeActivitiesData(...)`
  - `fetchPreSeenActivitiesData(...)`
  - `fetchBundleActivitiesData(...)`
- `ActivitiesList` now dispatches across explicit per-domain reorder
  controllers:
  - `createOutcomeActivitiesReorderController(...)`
  - `createPreSeenActivitiesReorderController(...)`
  - `createBundleActivitiesReorderController(...)`
- `ActivitiesList` now dispatches across explicit per-domain filter helpers:
  - `filterOutcomeActivities(...)`
  - `filterPreSeenActivities(...)`
  - `filterBundleActivities(...)`
- the shell now builds per-domain filter options explicitly:
  - `buildOutcomeActivitiesFilterOptions(...)`
  - `buildPreSeenActivitiesFilterOptions(...)`
  - `buildBundleActivitiesFilterOptions(...)`
- `ActivitiesList` now dispatches across explicit per-domain transform state:
  - `Outcome` transform-launch state
  - `Bundle` transform-launch state
  - `PreSeen` no-transform branch
- first shell split landed
- `PreSeen` now has a dedicated consumer shell in Dairy:
  - `PreSeenActivitiesList.svelte`
- the remaining `ActivitiesList` shell now supports only:
  - `Outcome`
  - `Bundle`

That split matters. It means the first shared hybrid extraction should stay at
the lower controller/helper layer. It is still too early to freeze a retained
shared shell around the richer wrapper behavior.

## Current shell comparison result

The current Dairy proof set does **not** yet justify a second retained shared
template shell.

After the lower-controller extraction work, the remaining wrapper differences
cluster around app-owned shell concerns:

- constrained query/filter combinations
- local add-link derivation from current scoped filters
- page-vs-tab header posture and back-link wording
- resource-specific reorder item shaping and display helpers
- delete-dialog variants and other domain-owned shell glue

That is not one clean cross-site shell yet. It is a set of app-owned wrapper
differences sitting over an increasingly shared lower controller layer.

Current platform judgment:

- keep extracting shared lower controllers and helpers
- keep `EntityList` as the retained shared shell for plain child collections
- do **not** add a second retained hybrid template shell yet
- revisit only if another consumer family proves the same remaining shell
  posture after lower-controller convergence
- workflow-heavy pages outside this hybrid boundary

Current shell judgment for the activities family:

- do **not** keep treating `Outcome`, `PreSeen`, and `Bundle` as one coherent
  shell family
- `PreSeen` was the real first split
- the remaining `ActivitiesList` shell is now the authoring shell for:
  - `Outcome`
  - `Bundle`
- the fetch layer and shell-local helper types should narrow to that split
  instead of carrying the broader transport union through local state

Why:

- `PreSeen`
  - has release-scoped filtering
  - has area-plus-release reorder targets
  - has no transform-launch posture
  - has distinct empty/add-state wording
- `Outcome` and `Bundle`
  - both still carry transform-launch behavior
  - both still behave more like authoring collections than release-scoped
    operational lists

Current recommended next shape:

- split `PreSeen` into its own shell first
- then reassess whether `Outcome` and `Bundle` still earn one shared shell or
  should separate further

That is a better next move than either:

- freezing the current one-shell `ActivitiesList`
- or forcing all three domains onto `EntityList`

Current implementation state for that judgment:

- first shell split landed
- `PreSeen` now has a dedicated consumer shell in Dairy:
  - `PreSeenActivitiesList.svelte`
- the area detail tab no longer mounts the broader `ActivitiesList` for the
  `PreSeen` branch
- `ActivitiesList` is now explicitly the `Outcome` / `Bundle` shell

Current reassessment after the `PreSeen` split:

- keep `Outcome` and `Bundle` in one shell for now
- do **not** force a second split yet

Why they still cluster more honestly than `PreSeen`:

- both remain activity-authoring surfaces rather than release-scoped activity
  management
- both still support transform-launch behavior from current selection
- both still use the same visible shell pieces:
  - `ActivitiesHeaderActions`
  - `ActivitiesFilterBar`
  - `ActivitiesListContent`
  - `ActivitiesDeleteDialogs`
- both still fit the same high-level interaction model:
  - fetch activities
  - optional filter within current authoring scope
  - optional reorder within current authoring scope
  - batch delete
  - selection-derived copy/move actions

What is still different but not yet split-worthy:

- `Outcome`
  - outcome filter
  - outcome and grouped outcome-in-area reorder targets
- `Bundle`
  - topic filter
  - topic reorder target

Current judgment:

- `PreSeen` was the real shell boundary
- `Outcome` and `Bundle` still earn one shared consumer shell until another
  proof shows their remaining divergence is costly enough to split

## Assessment Questions

- is the best next shape:
  - `EntityList` with a small extension set
  - one new retained hybrid collection shell
  - or a clean split root shell plus child shell
- which current consumer wrappers really share the same capability set
- where does cursor compatibility block convergence versus where is it only a
  local implementation habit
- how much of the current hybrid behavior belongs in templates versus patterns

## Design Comparison: `ModulesList`

`ModulesList` is the reference case because it combines:

- root and tab reuse
- constrained pathway/level slices
- cursor-style runtime pagination
- batch delete
- reorder sessions
- selection-derived transform launches

### Option 1 - Extend `EntityList`

This would mean teaching `EntityList` to absorb the remaining hybrid concerns.

What already fits today:

- batch actions
- selection mode
- reorder mode
- add actions
- query ownership hooks
- reorder error recovery hook

What does **not** fit cleanly today:

- cursor-style runtime pagination as a first-class posture
- root-vs-tab shell differences in one surface
- scoped transform-launch actions derived from current selection
- constrained root/tab filter posture without pushing more app-specific shell
  logic into list props

Judgment:

- good long-term only if the backend posture is already page-shaped and the
  root/tab split has already been clarified
- weak near-term answer for `ModulesList` as it exists now

### Option 2 - New retained hybrid collection shell

This would add a second top-level shared collection-shell surface above the
same lower controllers.

Benefits:

- one explicit shared answer for repeated hybrid list behavior
- keeps `EntityList` cleaner
- could own root/tab context, constrained filters, and transform-launch actions
  more directly

Risks:

- easy to create a second everything-shell
- duplicates template-level surface area before the true common subset is
  proven across more than one family
- may freeze current consumer quirks into public API too early

Judgment:

- plausible later
- too early as the first move unless another wrapper family proves the same
  capability set

### Option 3 - Split root shell and child shell over shared lower controllers

This means:

- keep one root-page collection shell for root-specific concerns
- keep one child-shell surface for tab-constrained use
- share lower controllers and presentational primitives underneath

Benefits:

- matches the real strongest boundary in `ModulesList`
- avoids forcing root-page concerns into child tabs
- avoids inventing a broad second retained shell too early
- lets page-shaped tab consumers move toward `EntityList` separately
- keeps cursor compatibility explicit where it still exists

Risks:

- still leaves some duplicated consumer code until the lower shared shape is
  extracted cleanly
- requires discipline to avoid creating two unrelated shells

Judgment:

- best current direction

## Current Recommendation

For the first implementation pass, prefer:

1. split root shell and child shell over shared lower controllers
2. keep `EntityList` as the default answer for plain page-shaped child
   collections
3. do **not** add a new retained hybrid shell until at least one more consumer
   family proves the same capability set
4. treat cursor-style pagination as compatibility debt to reduce, not as a new
   shared default posture

This is the smallest move that preserves function and still pushes the platform
toward one predictable family of shapes.

## Implementation Decomposition: `ModulesList`

`ModulesList` should be decomposed into three layers before any broad rewrite.

### Root-shell concerns

These are root-page specific or primarily root-page specific:

- page header section and back-link posture
- unconstrained pathway filter dropdown
- root-level default source context
- root-level "all pathways" browsing posture
- root-page trash entrypoint wording

These should not be treated as child-tab requirements by default.

### Child-shell concerns

These are still legitimate in tabs, but should stay explicitly child-scoped:

- parent-provided pathway or level constraint
- child-shell title/count posture
- parent-provided source context
- add actions that remain scoped to current parent context
- selection and reorder visibility only when the scoped collection actually
  supports them

These are the likely basis for a narrower shared child shell if `EntityList`
still proves too small.

### Shared lower helper candidates

These are the strongest extraction targets because they are not really
root-vs-tab concerns:

- constrained collection query assembly:
  - scope ids
  - search
  - page/cursor params
- selection and batch-delete wiring
- reorder session orchestration and conflict recovery
- transform-launch state derived from current selection
- list content composition:
  - card grid
  - reorder list
  - empty/loading/error states

This lower layer is where most reusable value sits today.

### What should not be extracted first

Do not start by extracting:

- page header wording
- source-context labels
- route strings for add/copy/move forms
- entity-specific card details

Those are app-facing leaves, not the shared shape.

## First Extraction Sequence

Use this order for the first real implementation pass:

1. separate root-shell and child-shell concerns in the consumer wrapper
2. extract lower list-workflow helpers that both shells can reuse
3. reassess the remaining child shell:
   - can it now sit on `EntityList`
   - or does it still justify a retained hybrid child shell
4. only after that, decide whether root-shell composition also deserves a
   shared retained surface

## Next Task

Use `ModulesList` as the first implementation case for the recommended split:

1. identify what should become shared lower controller/composition helpers
2. separate root-shell concerns from child-shell concerns
3. re-check whether the remaining child shell still needs anything beyond
   `EntityList` plus shared lower helpers
