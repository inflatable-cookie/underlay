# Contract: Shared Patterns and Workflow Shells

Status: active
Owner: repo maintainers
Depends on: `030-auth-and-session-systems.md`, `040-storage-blob-and-media-systems.md`, `080-ai-runtime-and-suggestions.md`, `090-ts-runtime-and-client-orchestration.md`

## Purpose

Define the shared workflow and pattern contract Underlay owns across:

- retained page/form workflow shells
- auth-aware data-loading and list/pagination/reorder controllers
- relation selector and drill-down selection systems
- media upload flow and optimistic state helpers
- navigation-context core and related workflow glue
- retained auth workflow components

This contract does not define visible admin page composition, design-system UI,
or app-specific route wording and permission policy. Those remain app-owned or
Poodle-owned.

Page-shaped child-tab list composition is owned by the template layer, not the
pattern layer. If a tab is really a child collection browse surface, the
preferred target is `EntityList` plus the `115` paged-list seam, not a new
pattern-owned tab-list shell.

## Sources of Truth

Primary:

- [`ts/src/patterns/index.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/index.ts)
- [`ts/src/patterns/SpaFormShell.svelte`](/Users/tom/Dev/projects/underlay/ts/src/patterns/SpaFormShell.svelte)
- [`ts/src/patterns/FormShell.svelte`](/Users/tom/Dev/projects/underlay/ts/src/patterns/FormShell.svelte)
- [`ts/src/patterns/auth.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth.ts)
- [`ts/src/patterns/authenticated-data.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/authenticated-data.svelte.ts)
- [`ts/src/patterns/auth-workflows.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/auth-workflows.ts)
- [`ts/src/patterns/list-controller.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/list-controller.svelte.ts)
- [`ts/src/patterns/pagination.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/pagination.svelte.ts)
- [`ts/src/patterns/reorder-controller.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/reorder-controller.svelte.ts)
- [`ts/src/patterns/reorder-conflict.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/reorder-conflict.ts)
- [`ts/src/patterns/reorder-session.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/reorder-session.svelte.ts)
- [`ts/src/patterns/batch-selection.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/batch-selection.svelte.ts)
- [`ts/src/patterns/batch-actions.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/batch-actions.svelte.ts)
- [`ts/src/patterns/selection-mode-controller.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/selection-mode-controller.svelte.ts)
- [`ts/src/patterns/selection-transform-state.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/selection-transform-state.ts)
- [`ts/src/patterns/selection-history.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/selection-history.ts)
- [`ts/src/patterns/RelationSelector/types.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/RelationSelector/types.ts)
- [`ts/src/patterns/RelationSelector/context.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/RelationSelector/context.svelte.ts)
- [`ts/src/patterns/RelationSelector/drilldown-context.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/RelationSelector/drilldown-context.svelte.ts)
- [`ts/src/patterns/local-search.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/local-search.ts)
- [`ts/src/patterns/drilldown-search.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/drilldown-search.ts)
- [`ts/src/patterns/media-upload-flow.svelte.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/media-upload-flow.svelte.ts)
- [`ts/src/patterns/optimistic.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/optimistic.ts)
- [`ts/src/patterns/navigation.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/navigation.ts)
- [`ts/src/patterns/forms.ts`](/Users/tom/Dev/projects/underlay/ts/src/patterns/forms.ts)

Supporting:

- [`docs/guides/096-form-helpers.md`](/Users/tom/Dev/projects/underlay/docs/guides/096-form-helpers.md)
- [`docs/guides/098-shared-admin-patterns.md`](/Users/tom/Dev/projects/underlay/docs/guides/098-shared-admin-patterns.md)
- [`docs/guides/095-navigation-context.md`](/Users/tom/Dev/projects/underlay/docs/guides/095-navigation-context.md)
- [`docs/guides/092-selection-suggestions.md`](/Users/tom/Dev/projects/underlay/docs/guides/092-selection-suggestions.md)

If these diverge, the shared code wins.

## Contract Goal

Underlay should provide one retained workflow-shell layer with clear seams:

- shared route and form workflows can be reused without preserving whole app
  pages in Underlay
- stateful list, selection, reorder, and upload behavior can be reused across
  apps without re-implementing controller logic
- richer selector, navigation, and auth-aware loading workflows can stay
  generic where the behavior is actually common

The goal is reusable workflow mechanics, not generic app UI.

## Shared Boundary

### Public pattern surface

The root `@decodelabs/underlay/patterns` barrel is intentionally small.

Current root exports:

- `ForgotPasswordFlow`
- `LoginPage`
- `PasswordRequirements`
- `SpaFormShell`
- SPA form result/submit/navigate types
- contextual action controller/types

Rule:

- root-barrel minimalism does not mean the broader pattern layer is not
  retained
- many pattern-owned implementations are intentionally consumed through
  `runtime/*` subpaths rather than the root `patterns` barrel

### SPA form workflow shells

Underlay retains a page-level SPA form orchestration shell.

Core pieces:

- `SpaFormShell`
- `FormShell`
- `SpaFormResult`
- `SpaSubmitHandler`
- `SpaNavigateFn`

Rules:

- Poodle owns fields, visible action chrome, and page composition
- `SpaFormShell` owns submit/result state, prepare hook, redirect handoff, and
  consistent success/error framing for SPA create/edit flows
- `FormShell` is the lower framework-agnostic form host underneath
- use this shell only when the page genuinely needs shared intent/result
  orchestration rather than just a styled form container

### Auth-aware workflow helpers

Underlay retains generic auth-aware workflow state above the lower auth store.

Core pieces:

- `configureAuth()`
- `getAuthConfig()`
- `useAuthenticatedData()`
- auth state guards like `isAuthenticated()` and `requireAuth()`

Rules:

- `useAuthenticatedData()` solves the auth-initialization race for protected
  browser fetch flows
- global auth wiring is app-provided through `configureAuth()`
- this layer owns auth-aware data loading as a workflow concern, not auth
  transport or session issuance

### List, pagination, and selection controllers

Underlay retains reusable list-state workflow controllers.

Core pieces:

- `createListController()`
- `createPaginationController()`
- `useBatchSelection()`
- `useBatchActions()`
- `useSyncedSelection()`
- pagination types and result interfaces under the pattern layer

Rules:

- these controllers own loading/error/filter/selection state and optimistic
  local mutation hooks
- apps provide fetchers, item DTOs, action implementations, and page
  composition
- list and pagination controllers are retained because the workflow behavior is
  reused across many admin surfaces

### Reorder workflow seam

Underlay retains the shared reorder session/controller model.

Core pieces:

- `createReorderController()`
- `ReorderController`
- `ReorderableItem`
- `reorder-conflict` helpers
- `createLocalReorderSession()`
- `createLoadedReorderSession()`
- `createSelectionModeController()`
- `buildSelectionTransformState()`

Rules:

- reorder state is local and batch-committed
- apps provide submit semantics and conflict policy
- shared logic covers pending order, dirtiness, reset, merge, item removal,
  local constrained reorder sessions, loaded fetch-all reorder sessions,
  selection-mode transitions, and selection-derived transform-launch state
- the preferred public import path for this lower helper set is
  `@decodelabs/underlay/runtime/data`
- use `createLocalReorderSession()` when the visible constrained list already
  contains the full reorder set
- use `createLoadedReorderSession()` when normal browsing is paged or
  cursor-backed and reorder needs a separate full-set load

Reference guide:

- [`docs/guides/097-autonomous-list-components.md`](/Users/tom/Dev/projects/underlay/docs/guides/097-autonomous-list-components.md)
- [`docs/guides/code/097-autonomous-list-components/list-workflow-helpers.ts`](/Users/tom/Dev/projects/underlay/docs/guides/code/097-autonomous-list-components/list-workflow-helpers.ts)

### Relation selector workflow family

Underlay retains the relation-selector system as a major shared workflow
surface.

Core pieces:

- `RelationSelector` types
- `createRelationSelectorContext()`
- `useRelationSelector()`
- drill-down context/types
- `createLocalSearchFns()`
- `createLocalDrillDownSearchFns()`
- `SelectionHistory`

Rules:

- relation selector owns search, suggestions, filter state, selection state,
  optional create-form handoff, and optional drill-down hierarchy flow
- apps provide actual search/suggestion functions, item DTOs, and rendered UI
- selection history is a shared workflow helper for recents-based suggestion
  flows
- local search helpers are retained because they adapt app-local in-memory
  datasets to the shared selector contracts
- when a selector is backed by a real resource collection route, the preferred
  posture is to call the same canonical resource list command with
  `profile=filter` rather than inventing a selector-only route or selector-only
  command family

### Media upload workflow seam

Underlay retains the reusable media upload state machine.

Core pieces:

- `createMediaUploadFlow()`
- `MediaUploadFlowController`
- `MediaUploadStep`

Rules:

- the shared flow covers duplicate check, create/initiate/upload/finalise,
  progress, replace-file, and error states
- apps provide API calls, completion policy, and visible UI composition
- lower blob/media storage semantics stay governed by `040`

### Optimistic state helpers

Underlay retains focused optimistic workflow primitives.

Core pieces:

- `createOptimisticList()`
- `createOptimisticCounter()`
- `createOptimisticToggle()`
- `createOptimisticValue()`

Rules:

- optimistic helpers own local transient mutation state and rollback hooks
- they do not own network transport or page composition

### Retained auth workflow components

Underlay still retains a narrow auth workflow UI family.

Core pieces:

- `LoginPage`
- `ForgotPasswordFlow`
- `PasswordRequirements`
- supporting auth-workflow components and types under `auth-workflows/`

Rules:

- these components are retained because they encode shared auth flow mechanics
  beyond one app’s page wording
- they are the exception in a mostly non-visual pattern layer

## Ownership Split

Underlay patterns own:

- workflow state machines and controllers
- retained SPA form shell behavior
- relation selector behavior and drill-down selection mechanics
- shared media upload flow
- optimistic state primitives
- a narrow retained auth workflow component set

Runtime owns:

- the curated public subpath entrypoints many apps import from

Client owns:

- framework-specific SvelteKit glue and lower browser integration

Poodle owns:

- general visual component primitives and most visible admin composition

Apps own:

- fetchers, DTO mapping, route wiring, wording, permission policy, and page UI

## Invariants

- pattern-owned controllers stay generic and app-agnostic
- visible page composition should not creep back into Underlay unless it is a
  genuine retained workflow shell
- shared workflow helpers may depend on app-provided callbacks, but the state
  model and lifecycle rules remain stable
- auth-aware workflow helpers rely on configured global auth hooks rather than
  app-local hidden imports
- relation selector search/suggestion contracts stay async even when backed by
  local in-memory helpers

## Retained Drift To Assess Later

- the pattern layer mixes true workflow shells with some helpers that may not
  still earn pattern ownership, especially `i18n`, compatibility-only
  suggestion-param re-exports in `selection-history.ts`, and some low-level
  form helpers
- root-barrel exports are tiny while most real pattern-owned behavior is
  consumed through `runtime/*`, so public authority and implementation location
  are split and easy to misread
- list, pagination, reorder, and batch helpers are retained under
  `runtime/data`; this is broad but intentional because they form one lower
  collection workflow layer below templates
- templates own page/list shell composition; lower data helpers should not grow
  visible page-shell behavior just because templates consume them
- the auth workflow component family may be broader or narrower than what
  still earns shared ownership after the UI translation wave

## Assessment Questions

- which helpers in `patterns/*` are real retained workflow shells versus
  compatibility residue that should move to `runtime`, `client`, or `utils`
- does the relation-selector system still represent one coherent shared
  contract, or has it become too broad
- should the duplicated auth-refresh list/fetch patterns collapse into fewer
  shared controllers
- does `SpaFormShell` still justify retained Underlay ownership now that most
  visible admin composition is Poodle-owned

## Next Task

Execute `g07.009`: TS public-surface test and guardrail reinforcement.
