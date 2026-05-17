# Contract: Consumer Template Adoption and Exception Policy

Status: active
Owner: repo maintainers
Depends on: `100-shared-patterns-and-workflow-shells.md`, `110-admin-template-system.md`, `115-admin-resource-api-shapes.md`, `117-hybrid-collection-shells.md`

## Purpose

Define when consumer apps must adopt Underlay's shared admin templates and when
route-local composition may stay outside them.

This contract covers:

- mandatory adoption posture for normal admin list, detail, form, trash, and
  list-card surfaces
- route-wrapper versus direct-route composition rules
- allowed exception classes
- review posture for new admin interfaces and convergence work

It does not redefine template APIs. That stays in `110`.

## Sources of Truth

Primary shared contract and usage surfaces:

- [`docs/contracts/110-admin-template-system.md`](/Users/tom/Dev/projects/underlay/docs/contracts/110-admin-template-system.md)
- [`docs/usage/templates/000-template-system-overview.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/000-template-system-overview.md)
- [`docs/usage/templates/entity-list-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-page.md)
- [`docs/usage/templates/entity-detail-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-detail-page.md)
- [`docs/usage/templates/entity-form-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-form-page.md)
- [`docs/usage/templates/entity-trash-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-trash-page.md)
- [`docs/usage/templates/entity-list-card.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/entity-list-card.md)
- [`docs/usage/templates/media-upload-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/media-upload-page.md)
- [`docs/usage/templates/media-detail-workflow-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/media-detail-workflow-page.md)
- [`docs/usage/templates/system-index-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/system-index-page.md)
- [`docs/usage/templates/admin-dashboard-page.md`](/Users/tom/Dev/projects/underlay/docs/usage/templates/admin-dashboard-page.md)

Cross-app convergence evidence:

- [`docs/roadmaps/g05/001-dairy-detail-and-tab-template-convergence-sweep.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/001-dairy-detail-and-tab-template-convergence-sweep.md)
- [`docs/roadmaps/g05/002-compli-me-and-contact-patch-detail-page-convergence-sweep.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/002-compli-me-and-contact-patch-detail-page-convergence-sweep.md)
- [`docs/roadmaps/g05/003-underlay-reference-template-completion-and-contract-hardening-sweep.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/003-underlay-reference-template-completion-and-contract-hardening-sweep.md)
- [`docs/roadmaps/g05/004-cross-app-media-library-template-consolidation.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/004-cross-app-media-library-template-consolidation.md)
- [`docs/roadmaps/g05/005-system-index-page-template-proof.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/005-system-index-page-template-proof.md)
- [`docs/roadmaps/g05/006-admin-dashboard-page-template-proof.md`](/Users/tom/Dev/projects/underlay/docs/roadmaps/g05/006-admin-dashboard-page-template-proof.md)

If these diverge, the contract plus the clearest retained reference posture
(`underlay-reference`, then the grouped `g05` proofs) win.

## Contract Goal

Underlay should make template adoption boring.

A normal app team should not have to rediscover:

- when a route must use a shared page shell
- when a list should become an app-local wrapper instead of staying in the route
- when a card should use `EntityListCard`
- what counts as a real exception instead of template drift

The goal is one declared review posture for normal admin interfaces.

## Scope Boundary

In scope:

- admin browse/manage collection pages
- admin detail pages
- create/edit pages
- repeated trash workflows
- repeated admin list cards
- repeated media/system/dashboard page shells
- exception classification

Out of scope:

- raw template API details
- non-admin product surfaces
- one-off workflow internals inside otherwise converged pages
- lower Poodle primitive policy

## Shared Boundary

### Default adoption rule

Normal admin interfaces should use the shared Underlay template family by
default.

Normal retained family:

- `EntityListPage`
- `EntityDetailPage`
- `EntityFormPage`
- `EntityTrashPage`
- `EntityListCard`
- `MediaUploadPage`
- `MediaDetailWorkflowPage`
- `SystemIndexPage`
- `AdminDashboardPage`

Rules:

- treat shared template adoption as the default posture, not an optional polish
  pass
- treat repeated route-local shells in those shapes as drift unless an explicit
  exception class applies
- preserve existing behavior first; do not claim convergence by dropping
  meaningful workflow behavior

### List route rule

Real browse/manage collection surfaces should normally become reusable app-local
wrapper components over `EntityListPage`.

Rules:

- root list routes should thin-mount `src/lib/lists/*` wrappers or the local
  app equivalent
- detail tabs should reuse that same wrapper when the collection semantics are
  the same and only scope, query mode, or a small feature toggle changes
- route files should not keep full `EntityListPage` compositions in place once
  the surface is a stable app feature
- use `EntityList` directly only when the surface is genuinely subordinate,
  inline, or utility-like rather than a real browse/manage page or tab

### Detail route rule

Normal repeated entity detail shells should use `EntityDetailPage`.

Rules:

- detail routes may mount `EntityDetailPage` directly
- do not require an app-local detail wrapper unless the same detail shell is
  genuinely reused across more than one caller
- route-owned workflow sections may stay local inside the shared shell
- the retained default top-level tab posture is underline; treat `tabsVariant="card"`
  as an explicit exception, not the normal detail default

### Form route rule

Normal create/edit pages should use `EntityFormPage`.

Rules:

- routes may mount `EntityFormPage` directly
- repeated field bodies or supporting subforms should become app-local form
  components when the same form logic serves more than one caller
- do not force declarative form config where direct form composition is the
  intended contract

### Trash route rule

Repeated admin trash workflows should use `EntityTrashPage`.

Rules:

- the shared shell should own header, loading, error, empty, and card-grid
  posture
- route or wrapper ownership may still keep restore, purge, search, sort,
  conflict handling, and dialog workflow
- do not push trash pages through `EntityListPage` just to make them fit the
  list lane

### List-card rule

Repeated admin collection cards should use `EntityListCard`.

Rules:

- app-local card components should normally compose `EntityListCard` instead of
  raw Poodle `ListCard`
- real admin card actions should normally use the leading visual as the menu
  trigger
- keep raw `ListCard` only for explicit exception classes

### Shared page-shell rule

When a repeated non-entity page shell has already been promoted into Underlay,
consumers should use it.

Current retained shells:

- `MediaUploadPage`
- `MediaDetailWorkflowPage`
- `SystemIndexPage`
- `AdminDashboardPage`

Rules:

- do not keep app-local copies of those outer shells once the retained template
  exists and fits the route
- route-local workflow internals may stay local when that is the actual seam

## Allowed Exceptions

### Valid exception classes

A route or card may stay outside the shared template family when it is clearly
one of these:

- non-admin surface
- workflow-heavy surface where the repeated page shell has not been promoted
  yet
- subordinate embed or inline utility surface where a page shell would be
  artificial
- narrow proof spike that is expected to collapse into a retained wrapper or
  template after the proof
- route-local not-found, error, or gate layer inside an otherwise converged
  shared shell

### Invalid exception claims

These are not good enough reasons on their own:

- "this route is slightly different"
- "the tab version has a different parent scope"
- "the page already works"
- "the card only needs a few extra badges"
- "it would be faster to keep the shell here"

If the surface is still a normal admin list, detail, form, trash, or repeated
card shape, it should stay on-contract unless a real workflow boundary proves
otherwise.

## Review Posture

### New admin interface rule

When building or reviewing a new admin interface:

- start from the shared template family first
- justify exceptions explicitly
- prefer app-local wrappers over route-local shells for stable list surfaces
- prefer route-local direct `EntityDetailPage` and `EntityFormPage` use over
  bespoke outer shells

### Convergence review rule

When auditing an existing app:

- classify each candidate as:
  - already converged
  - true drift
  - explicit exception
- do not mix repeated admin CRUD drift with dashboards, system indexes, media
  workflows, or other shells that already have different retained templates
- do not keep reopening settled template families without new evidence

## What Good Looks Like

Good outcomes:

- root list routes thin-mount app-local wrappers
- detail tabs reuse the same list wrapper when the semantics match
- repeated details and forms use shared outer shells
- trash pages use `EntityTrashPage`
- repeated admin collection cards use `EntityListCard`
- exceptions are explicit and defensible

Bad outcomes:

- route-local `EntityListPage` definitions scattered across stable app features
- repeated raw `ListCard` admin cards with no exception rationale
- forcing every workflow fragment into a template just to increase template
  counts
- calling a surface "special" when it is still plainly a normal admin list or
  detail page
