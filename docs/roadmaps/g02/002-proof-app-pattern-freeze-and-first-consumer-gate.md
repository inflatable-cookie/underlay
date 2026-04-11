# 002 - Proof-App Pattern Freeze And First Consumer Gate

Status: complete
Owner: repo maintainers
Updated: 2026-04-09

## Context

`g02.001` confirmed that the live work is not broad execution across all six
consumer families. The real in-flight lane is a proof-app normalization wave
across:

- `underlay`
- `poodle`
- `acowtancy/dairy`
- `underlay-reference/acme-admin`
- `contact-patch/cp-admin`

That lane has already hardened a meaningful set of shared admin/media/detail
patterns. The next honest step is to close that proof lane cleanly before any
broader consumer rollout resumes.

## Goals

- freeze the proof-app admin/media/list/detail/dialog/recovery posture as one
  bounded wave
- reconcile any current in-flight edits inside that wave rather than letting
  them continue implicitly
- leave the wider consumer-family rollout explicit but pending

## Non-Goals

- reopening freeform family-by-family execution across all six consumers
- inventing new shared UI surfaces without proof-app evidence
- rolling the proof patterns broadly into `compli-me`, `songsprout`, or
  `loophole/composer` before the proof-app wave is closed

## Scope

### In scope

- Underlay mixed recipes that govern the proof-app admin/media families
- Poodle visible recipe guidance for those same families
- Dairy, `acme-admin`, and `cp-admin` proof routes already participating in the
  current pattern lane
- current in-flight edits that fit that bound, including media-detail recovery
  posture

### Out of scope

- non-proof consumers beyond targeted assessment for later wave planning
- new front-end/dashboard or app-specific workflow redesign
- fresh shared-surface extraction unless the proof set exposes a clear generic
  gap

## Execution Plan

### Batch 2.1 - Proof-App Surface Freeze

- [x] inventory the proof-app families already executing
- [x] classify each as Underlay-owned, Poodle-owned, or app-local
- [x] close any still-open in-flight proof-app edits inside that bound

### Batch 2.2 - Recipe And Contract Lock

- [x] verify the Underlay mixed recipe layer and Poodle guide layer match the
      frozen proof-app posture
- [x] record any remaining mismatches as explicit follow-on work rather than
      absorbing them into broad execution

### Batch 2.3 - First Consumer Gate

- [x] define which non-proof consumer family is the next honest rollout target
- [x] leave the rest of the consumer family explicit but pending
- [x] name one bounded downstream wave as the next task

## Proof-App Family Inventory

The currently executing proof-app lane is the accumulated pattern work across:

- `underlay`
- `poodle`
- `acowtancy/dairy`
- `underlay-reference/acme-admin`
- `contact-patch/cp-admin`

That lane has already hardened these families:

1. diagnostics and review queues
2. list/filter and selection/bulk-action browse posture
3. media browse/detail, related-item sections, destructive confirms, and
   recovery states
4. user-management detail/edit flows
5. auth/account signed-in pages
6. create/edit route-shell feedback via `SpaFormShell`
7. nested child create/edit flows
8. child collections under parent detail routes
9. overview and workflow-launch pages
10. ops detail and error-inspection shells

## Ownership Classification

### Underlay-owned seams

- mixed implementation-order recipes and full-stack guidance for the proof-app
  families
- retained workflow/runtime/client structures used by those families:
  - `SpaFormShell`
  - auth/runtime/navigation helpers
  - feedback/toast and authenticated-data helpers
- planning/control posture for which families are proof-app work versus pending
  broader rollout

### Poodle-owned seams

- visible shells and building blocks used repeatedly across the proof-app lane:
  - `PageHeader`
  - `MetaBar`
  - `ListContainer`
  - `FilterToolbar`
  - `FormDialog`
  - `AlertDialog`
  - `InlineListSection`
  - `DetailSection`
  - `DetailItem`
  - `EmptyState`
  - `PageLoading`
  - `Callout`
- guide-level visible recipes for list, detail, dialog, media, auth/account,
  overview, and admin delivery posture

### App-local seams

- route wiring, query mapping, and mutation sequencing
- local section headers, menus, and workflow-specific actions
- host-owned list rows, cards, and tab content
- domain-specific dashboards, workflow launch bands, assessment content, and
  rich child collections
- any page-local retry/refetch behavior after load or mutation success

## Execution State

### Already executing

- proof-app route and doc normalization inside Dairy, `acme-admin`, and
  `cp-admin`
- recipe hardening in the Underlay mixed recipe layer and the Poodle guide
  layer for the families listed above

### Closed enough for this wave

- auth/account posture
- user-management detail/edit posture
- overview/workflow-launch posture
- nested child form posture
- child collection split
- review queue and review-detail posture
- destructive confirm posture
- inline create/edit dialog posture

These families are still part of the proof-app evidence, but they do not need
more freeform execution inside `g02.002` unless the recipe/contract lock phase
finds a real mismatch.

### Still in scope for explicit closeout

- media-family recovery and empty-state posture
- recipe alignment checks that confirm the proof-app docs still match the live
  route posture rather than the older freeform execution memory

### Pending planning

- any rollout into `compli-me`, `songsprout`, or `loophole/composer`
- any broader normalization beyond the proof apps
- any new shared-surface promotion not already proven in the proof lane

## Recipe And Contract Lock

### Coverage Check

The frozen proof-app families already map cleanly onto the active recipe layer:

- diagnostics and review queues
  - Poodle: `013-admin-feature-delivery-recipes.md`
  - Underlay: `admin-ops-console.md`, `180-admin-workflow-playbook.md`
- list/filter and selection/bulk-action posture
  - Poodle: `003-list-and-filter-recipes.md`, `013-admin-feature-delivery-recipes.md`
  - Underlay: `autonomous-admin-list.md`, `097-autonomous-list-components.md`,
    `098-shared-admin-patterns.md`
- media browse/detail/related/confirm/recovery posture
  - Poodle: `004-dialog-and-detail-recipes.md`,
    `012-media-library-and-upload-recipes.md`,
    `013-admin-feature-delivery-recipes.md`
  - Underlay: `077-media-library.md`, `trash-lifecycle.md`,
    `media-upload-pipeline.md`
- user-management detail/edit posture
  - Poodle: `013-admin-feature-delivery-recipes.md`
  - Underlay: `crud-admin-interface.md`, `110-admin.md`
- auth/account signed-in pages
  - Poodle: `010-auth-ui-and-workflow-recipes.md`
  - Underlay: `062-auth-ui-components.md`
- create/edit route-shell feedback
  - Poodle: `001-form-layout-and-field-recipes.md`,
    `013-admin-feature-delivery-recipes.md`
  - Underlay: `096-form-helpers.md`, `crud-admin-interface.md`
- nested child forms and child collections
  - Poodle: `004-dialog-and-detail-recipes.md`,
    `013-admin-feature-delivery-recipes.md`
  - Underlay: `nested-entity-management.md`
- overview and workflow-launch pages
  - Poodle: `011-page-shell-and-admin-recipes.md`
  - Underlay: `110-admin.md`, `180-admin-workflow-playbook.md`
- ops detail and error inspection
  - Poodle: `004-dialog-and-detail-recipes.md`,
    `013-admin-feature-delivery-recipes.md`
  - Underlay: `admin-ops-console.md`, `180-admin-workflow-playbook.md`

### Remaining Mismatches

No new missing major guide family was found in Batch 2.2.

The real remaining mismatches are narrower:

1. **Current in-flight proof-app edits still need closeout**
   - Dairy media-detail recovery posture
   - `acme-admin` media-detail recovery posture
   - matching Poodle and Underlay guide edits for retry/recovery treatment
   These belong to the proof-app wave and should be closed there, not treated
   as authority to continue picking new families.

2. **Surgical commit discipline is still required**
   - `poodle` has large unrelated preview/specimen dirt outside the proof lane
   - `underlay` still has unrelated roadmap/log/front-door dirt outside this
     bounded wave
   The proof-app closeout batches must keep staging narrow.

3. **The next real planning decision is downstream rollout, not more proof work**
   - Batch 2.3 now needs to choose the first non-proof consumer family and keep
     the rest pending
   - there is not enough evidence to justify another fresh proof-app family by
     default

## Current Bound

`g02.002` is **not** authority to keep choosing the next proof-app family
indefinitely.

It is authority to:

- freeze the proof-app families already touched
- reconcile still-open in-flight edits that clearly belong to those families
- verify that Underlay and Poodle docs match that frozen proof posture
- choose the first downstream consumer gate after the proof lane is explicitly
  closed

## Batch 2.2 Outcome

The active recipe spine is already coherent enough for the frozen proof-app
families.

That means the next honest work is:

- close the still-open proof-app edits already in flight
- then choose the first downstream consumer family explicitly

It does **not** mean “keep normalizing one more proof-app family.”

## First Consumer Gate

### Chosen rollout target

The first non-proof consumer family is **`compli-me`**.

This is the next honest target because:

- it is still structurally closest to the proof-app admin lane
- it has the strongest overlap with the admin/media/detail/form/dialog patterns
  already proven in Dairy, `acme-admin`, and `cp-admin`
- it is more likely than `songsprout` or `loophole/composer` to expose whether
  the proof-app admin patterns really generalize across a second consumer
  family without forcing front-end or niche workflow exceptions too early

### Explicitly pending

These consumer families remain pending after the `compli-me` gate:

- `songsprout`
  - important, but more likely to mix app-specific front-end and consumer-ish
    posture with the admin pattern set
- `loophole/composer`
  - still useful later, but narrower and less valuable than `compli-me` as the
    first generalization test

### Bound for the follow-on wave

The follow-on wave should target `compli-me/admin` first and only touch:

- the already-proven admin/media/list/detail/dialog/form/recovery families
- the matching Underlay and Poodle guide surfaces only if the `compli-me`
  rollout exposes a real gap rather than route-local variation

It should not reopen proof-app family selection or widen immediately to
`songsprout` and `loophole/composer`.

## Consumer Upgrade Impact

Impact class: `assessment`

This wave freezes the proof-app posture and may still include consumer-visible
UI normalization inside the proof apps, but it is not the place for broad
downstream rollout. Any wider consumer impact belongs in the follow-on wave
opened from this gate.

## Exit Criteria

- the proof-app lane is explicitly bounded and no longer running as freeform
  execution
- Underlay and Poodle docs reflect that proof-app posture cleanly
- one downstream consumer rollout wave is named explicitly and the rest remain
  pending

## Next Task

Execute `g02.003`: take the frozen proof-app admin/media/detail/list/dialog
patterns into `compli-me/admin` as the first downstream consumer gate, then
report what still generalizes cleanly versus what proves app-local.
