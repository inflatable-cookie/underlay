# Contract: Admin Template System

Status: active
Owner: repo maintainers
Depends on: `090-ts-runtime-and-client-orchestration.md`, `100-shared-patterns-and-workflow-shells.md`, `115-admin-resource-api-shapes.md`

## Purpose

Define the shared admin template system contract Underlay owns across:

- Level 1 page shells for list, detail, and form pages
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

Primary docs:

- [`docs/usage/templates/000-template-system-overview.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/000-template-system-overview.md)
- [`docs/usage/templates/entity-list-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-page.md)
- [`docs/usage/templates/entity-detail-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-page.md)
- [`docs/usage/templates/entity-form-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-form-page.md)
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
- Level 2: reusable sections
  - `EntityList`
  - `EntityDetail`
  - `EntityDetailModule`
  - `EntityInlineListModule`
  - `EntityAttributeList`
- Level 2.5: entity composition helper
  - `EntityListCard`
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
- batch action config
- reorder config
- card/table/log presentation modes
- URL-synced `query` / `onQueryChange` seam
- `beforeList`, `headerLeadingActions`, and `headerActions` extension slots

Rules:

- `EntityListPage` is the page-level shell around `EntityList`
- `EntityList` is the real list engine and may be used standalone
- data loading is caller-provided through `dataLoader(fetch, token, query)`
- that loader shape is governed by `115-admin-resource-api-shapes.md`
- presentation may be cards, table, or log
- filter, batch, and reorder behavior is declarative where it fits
- the parent may own URL query state, while the template owns list interaction
  behavior

### List-card composition seam

`EntityListCard` is the stable card composition helper for card-mode lists.

Core pieces:

- `EntityListCard`
- `EntityListCardProps`
- badge/counter/mode-display types

Rules:

- `EntityListCard` wraps Poodle `ListCard` with Underlay-specific mode and
  content conventions
- card display may adapt for reorder or selection mode through the mode-display
  overrides
- apps still own entity-specific card content choices and callbacks

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
- `EntityDetail` is the reusable section shell for detail modules
- detail modules provide the framed content grid units used inside sections
- child collections may use `EntityList` or `EntityInlineListModule`
- child collection tabs should use the canonical child-list API shape from
  `115-admin-resource-api-shapes.md`
- the detail template supports nested list/detail compositions without forcing
  the whole page into one monolith

### Form template seam

`EntityFormPage` is intentionally only a page shell.

Core piece:

- `EntityFormPage`

Rules:

- there is no declarative `EntityForm`
- forms stop at the page-shell boundary because real forms have arbitrary
  layout, custom fields, conditional logic, uploads, rich text, and custom
  validation
- `EntityFormPage` owns header, loading, error/success state, and spacing only
- apps bring the actual `<form>` markup and field logic with Poodle primitives

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
