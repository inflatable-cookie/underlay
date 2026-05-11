# 030 - Child Collection Capability Convergence Sweep

This sweep verifies that child collection surfaces across Underlay consumers are
converging on a small number of predictable capability shapes rather than
accumulating app-local wrappers forever.

It is the practical audit companion to:

- [110-admin-template-system.md](../contracts/110-admin-template-system.md)
- [115-admin-resource-api-shapes.md](../contracts/115-admin-resource-api-shapes.md)
- [116-canonical-collection-routes-and-query-profiles.md](../contracts/116-canonical-collection-routes-and-query-profiles.md)

## Problem This Sweep Targets

Once route families and page envelopes are cleaned up, one harder kind of drift
remains:

- app-local tab wrappers that all look similar but each carry a slightly
  different behavior mix
- child collection tabs that should share one list shell but still bundle
  search, batch actions, reorder, transform actions, or root/tab dual-use in
  incompatible local combinations
- migration decisions that stop at "too complicated" instead of identifying the
  missing shared capability shape

The point of this sweep is to separate:

- **ready shared-shell fits**
- **valid compatibility wrappers**
- **true shared-surface extraction targets**

Do not treat `variant="tab"` or wrapper-file existence as the problem by
itself. The problem is unclear capability shape.

## Policy Baseline

See:

- [110-admin-template-system.md](../contracts/110-admin-template-system.md)
- [115-admin-resource-api-shapes.md](../contracts/115-admin-resource-api-shapes.md)

### Hard rules

- behavior preservation is the acceptance bar for child-tab migration
- route and envelope convergence do not justify dropping meaningful behavior
- page-shaped child collections should prefer:
  - canonical child route
  - `PagedListResponse<T>`
  - `toPagedListResult(...)`
  - `EntityList`

### Soft rules

- compatibility wrappers are acceptable when they still carry real behavior the
  shared shell does not express cleanly
- multiple wrappers with the same capability set are a signal to extract a
  better shared shape, not to keep classifying forever

## Scope

Run across:

- shared template surfaces in Underlay
- client commands and route shapes
- consumer tab/list wrappers
- page-local child collection tabs

```bash
export SHARED_REPO="/path/to/underlay"
export CLIENT_REPO="/path/to/app-client"
export ADMIN_REPO="/path/to/app-admin"
```

## Step 1 - Inventory child collection surfaces

Find child tabs and shared tab wrappers:

```bash
rg -n 'variant="tab"|activeTab|Tabs' "$ADMIN_REPO/src" --type svelte
rg -n 'List$|TabContent|InlineList' "$ADMIN_REPO/src/lib" --type svelte
```

Build a table for each child collection surface:

- page or wrapper name
- resource family
- route used
- response envelope
- root-only, tab-only, or shared root/tab usage
- user-facing actions
- search/filter mode
- batch or selection behavior
- reorder or transform behavior

Pass criteria:

- every surface is classified by capability set, not just by route family

## Step 2 - Class capability shape

Classify each surface into one of these capability buckets.

### A. Plain child browse surface

Usually all are true:

- child collection route is canonical
- page-shaped envelope already exists
- card/table/log presentation only
- simple search/filter
- optional add and single-item actions
- no root/tab dual-use burden
- no reorder or transform workflow

Preferred target:

- `EntityList`

### B. Child collection with shared workflow extras

Some are true:

- batch actions
- selection mode
- reorder mode
- optimistic conflict recovery
- small scoped transforms

Preferred target:

- still likely shared
- may require extending `EntityList` or adjacent template seams

Do not leave several of these in app-local wrappers if they are materially the
same shape.

### C. Root-and-tab hybrid list shell

Usually true:

- same wrapper is reused on root pages and detail tabs
- route/query posture differs across contexts
- header/back-link/filter behavior differs across contexts
- page and tab concerns are still interleaved in one surface

This is acceptable compatibility posture, but it is not a quick `EntityList`
conversion.

Needed decision:

- split into root shell + child shell
- or keep temporarily with explicit compatibility status

### D. Workflow-heavy composite surface

Usually true:

- activity or transform workflow behavior
- non-trivial local orchestration
- multiple related datasets or dialogs
- not really just a child collection browse surface

These are not list-shell cleanup targets.

## Step 3 - Check migration fitness

For each A/B surface, verify whether migration would preserve:

- user-facing actions
- search/filter behavior
- delete/batch/reorder behavior
- navigation/source-context behavior
- current route semantics

Pass criteria:

- no migration is marked "ready" unless behavior survives intact

## Step 4 - Detect shared extraction opportunities

Look for 2+ wrappers with the same capability set.

Questions:

- are they all really A surfaces and just waiting for `EntityList`
- are they all really B surfaces and pointing at one missing shared extension
- are they mixed C/D surfaces that should stay separate

Pass criteria:

- the sweep names missing shared capability shapes explicitly instead of
  stopping at "too custom"

## Step 5 - Record findings

Use this rubric:

- `shared-fit-a`
- `shared-fit-b`
- `compat-wrapper-c`
- `workflow-heavy-d`
- `extract-shared-capability`

## Reference Classification: Dairy

### Shared fits already proven

- outcome quiz/digital/written question tabs
- module syllabus-updates tab
- exam edition digital/written question tabs
- mock question tab

These prove that page-local child tabs can move to `EntityList` when the route
shape is already good and behavior is modest.

### Compatibility wrappers still acceptable

- `ModulesList`
  - shared root/tab surface
  - cursor-style runtime pagination
  - constrained pathway/level filter posture
  - reorder mode
  - batch delete
  - copy/move transform actions
- `ExamSchedulesList`
- `ExamEditionsList`
- `SectionsList`
- `AreasList`
- `OutcomesList`
- `PreSeenReleasesList`

These are not "done forever." They are named compatibility posture until a
broader shared-shape extraction or root/tab split is chosen.

### Concrete extraction target: `ModulesList`

`ModulesList` is the clearest evidence that one more retained shared shape may
be needed.

It is not a plain child browse surface because it combines:

- root and tab reuse
- cursor-style runtime pagination
- scoped constraint filters (`pathwayId`, `levelId`)
- batch delete
- reorder mode
- small transform-launch actions

That points at one of three deliberate outcomes:

1. split root shell and child shell cleanly
2. extend shared list/template capability to cover this mixed behavior set
3. retain one explicit hybrid shared shell for this class of collection

What it does **not** justify is leaving every similar surface app-local by
default. If more wrappers land in this same bucket, the next platform task is
shared extraction, not more classification.

### Workflow-heavy surfaces

- `ActivitiesList` on outcome, area, bundle, and bundle-topic tabs

These are not list-shell convergence targets in the current lane.

### Extraction target to investigate

- child collection surfaces that combine:
  - page-shaped route
  - card presentation
  - search
  - batch actions
  - maybe reorder

If several consumer wrappers land here, the next move is to extend shared
template capability deliberately rather than leave each wrapper local.
