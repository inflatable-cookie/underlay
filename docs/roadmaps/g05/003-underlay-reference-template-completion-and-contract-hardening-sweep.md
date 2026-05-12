# g05.003: underlay-reference template completion and contract-hardening sweep

Status: complete

## Why

`underlay-reference` should be the cleanest consumer, not just an early proof
site.

Right now `acme-admin` still has two kinds of drift:

- real browse/manage list pages declared directly in route files instead of
  reusable app-local list wrappers
- a small repeated detail-shell tail still outside `EntityDetailPage`

That leaves too much room for interpretation when the next Underlay app is
implemented.

## Consumer Upgrade Impact

This lane changes the reference implementation and the written template
contract.

Main risks:

- route-local list behavior drifting when moved behind wrappers
- tab/root reuse weakening during wrapper extraction
- docs teaching two equally-valid patterns where one should be the default

The acceptance rule is simple:

- `underlay-reference` should model the implementation style we want sibling
  apps to copy

## Current Inventory

### Already reference-grade

`acme-admin`

- `ProjectsListPage.svelte`
- `TasksListPage.svelte`
- project detail routes on `EntityDetailPage`
- task detail routes on `EntityDetailPage`
- category detail routes on `EntityDetailPage`
- project/task/category create-edit routes on `EntityFormPage`
- media detail already on `EntityDetailPage`

### Root-list wrapper sweep

Completed in `acme-admin`:

- `/media`
- `/media/trash`
- `/users`
- `/categories`
- `/system/jobs`
- `/system/scheduled-tasks`
- `/system/errors`
- `/system/audit`

Those now sit behind reusable `src/lib/lists/*` wrappers and the routes
thin-mount them.

### Detail-shell sweep

Completed in `acme-admin`:

- `/users/[userId]`
- `/system/jobs/[id]`
- `/system/scheduled-tasks/[id]`

Those now use `EntityDetailPage`.

### Shared template seam proven in this lane

One real shared gap showed up:

- `EntityDetailPage` needed a single-surface body mode for detail pages that do
  not have a real top-level tab split

That is now explicit through:

- `content={...}` for non-tab detail bodies
- `tabs={[...]}` only when the page has a real top-level section split

### Explicit workflow or non-entity exceptions

These should not be forced into the entity template lane:

- `/media/[mediaId]`
  - version manager workflow
  - activation / purge / upload flows
  - usage management
- `/media/upload`
  - upload workflow, not an entity form
- dashboard, account, auth, and system index surfaces
  - not entity list/detail/form pages
- `/system/poodle-gap-review`
  - tooling/review surface, not an admin entity page

## Contract Hardening Needed

The docs need one stronger rule:

- real admin list surfaces should be implemented as reusable app-local list
  wrappers over `EntityListPage`

That means:

- route files should normally thin-mount `src/lib/lists/*`
- detail tabs should reuse the same wrapper when the collection semantics are
  the same
- route-local `EntityListPage` composition is acceptable only for:
  - explicit workflow exceptions
  - narrow proof spikes before wrapper extraction

Parallel clarification:

- detail routes may still use `EntityDetailPage` directly because those shells
  are usually route-owned per entity
- create/edit routes may still use `EntityFormPage` directly, but shared field
  bodies belong in app-local form components when the same form serves multiple
  callers

## Done When

This lane is done when all of these are true:

- the docs say one clear thing about list wrapper ownership
- `acme-admin` root list pages no longer declare route-local `EntityListPage`
  implementations for normal browse/manage collections
- the remaining repeated detail-shell holdouts are on `EntityDetailPage`
- the only route-local exceptions left are genuine workflow or non-entity
  surfaces

## Result

This lane is complete.

`underlay-reference/acme-admin` now models the intended reference posture:

- real browse/manage collections live in reusable app-local list wrappers over
  `EntityListPage`
- routes thin-mount those wrappers
- repeated detail shells mount `EntityDetailPage` directly
- forms stay on `EntityFormPage`
- remaining route-local surfaces are explicit workflow or non-entity
  exceptions, not template drift
