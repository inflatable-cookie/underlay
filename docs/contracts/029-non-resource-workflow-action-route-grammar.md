# Contract: Non-Resource Workflow Action Route Grammar

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `026-route-families-and-access-model.md`, `027-api-canonical-path-cutovers-and-compatibility-retirement.md`, `119-helper-search-and-lookup-route-catalogue.md`

## Purpose

Define the shared route grammar for non-resource workflow actions.

This contract covers:

- action routes that are not plain CRUD reads or writes
- action naming for restore, purge, reorder, complete, skip, revoke, claim,
  release, and similar verbs
- when an action belongs on a resource path versus a dedicated action family
- when batch or collection actions are appropriate

It does not define resource list/detail routes. Those stay with `115`, `116`,
and `118`.
It does not define helper reads. Those stay with `119`.

## Sources of Truth

Primary shared sources:

- [`020-http-transport-and-server-boundary.md`](./020-http-transport-and-server-boundary.md)
- [`026-route-families-and-access-model.md`](./026-route-families-and-access-model.md)
- [`027-api-canonical-path-cutovers-and-compatibility-retirement.md`](./027-api-canonical-path-cutovers-and-compatibility-retirement.md)
- [`119-helper-search-and-lookup-route-catalogue.md`](./119-helper-search-and-lookup-route-catalogue.md)

Reference consumer evidence:

- `underlay-reference/apps/acme-api/crates/api/src/routes/mod.rs`
- `songsprout/nursery/crates/api/src/routes/artist_task_actions.rs`
- `acowtancy/farmyard/crates/api/src/routes/admin/learning/variants/reorder/variant.rs`
- `acowtancy/farmyard/crates/api/src/routes/admin/learning/variants/reorder/preseen.rs`
- `acowtancy/farmyard/crates/api/src/routes/admin/learning/levels/deletion.rs`
- `acowtancy/farmyard/crates/api/src/routes/admin/marking.rs`

If these diverge, the contract plus the cleanest current action proofs win.

## Contract Goal

Underlay should stop letting workflow actions drift into arbitrary path
grammar.

A new app should not have to guess:

- whether an action should be `POST`, `PATCH`, `PUT`, or `DELETE`
- whether the action belongs on a resource path or a separate action family
- when to use `/restore`, `/purge`, `/reorder`, `/complete`, or a batch action
- whether collection actions should use a suffix or a child route

The goal is one declared action vocabulary instead of a trail of app-local
verbs.

## Scope Boundary

In scope:

- admin and product workflow actions that mutate state
- batch actions
- reorder actions
- claim/release/complete/skip style workflow transitions
- restore/purge/soft-delete lifecycle actions

Out of scope:

- normal create/update/delete CRUD
- helper reads and status routes
- auth bootstrap and verification routes
- internal service or job bus commands

## Shared Boundary

### Action versus resource rule

Use a workflow action route when the operation is not a normal CRUD update on a
resource representation.

Typical action cases:

- restore
- purge
- soft-delete
- reorder
- complete
- skip
- claim
- release
- revoke
- bulk status changes

Rules:

- if the operation can be expressed as a normal resource update, prefer the
  resource update
- if the operation is a domain verb or workflow transition, use an action route

### Resource-scoped action rule

When the action applies to one known resource, attach the action to that
resource path.

Preferred patterns:

- `POST /v1/admin/categories/{category_id}/restore`
- `DELETE /v1/admin/media/{media_id}/purge`
- `POST /v1/admin/learning/levels/{level_id}/soft-delete`
- `POST /v1/artist-task-actions/{task_id}/complete`
- `POST /v1/artist-task-actions/{task_id}/skip`

Rules:

- use the resource path when the action target is one concrete entity
- use an action verb segment instead of inventing pseudo-fields like
  `/set-complete`
- keep the verb at the end of the route

### Dedicated action-family rule

When the action target is a workflow item rather than a stable resource family,
it may live in a dedicated action family.

Example:

- `/v1/artist-task-actions/{task_id}/complete`

Rules:

- use a dedicated action family when the route is about workflow execution more
  than entity ownership
- do not force these under a resource read family just for symmetry
- keep the family noun explicit:
  - `*-actions`
  - `*-transitions` only if the product really uses transition semantics

### Collection action rule

When the action applies to a collection or selection set, attach it to the
collection path.

Preferred patterns:

- `/v1/admin/categories:batch-delete`
- `/v1/admin/projects/{project_id}/tasks:batch-delete`
- `/v1/admin/projects/reorder`

Rules:

- collection actions may use a collection-level suffix when that posture is
  already established for the family
- keep the suffix explicit and verb-led:
  - `:batch-delete`
- do not invent multiple competing batch grammars in one app

### Reorder rule

Reorder is a collection action, not a single-resource update.

Preferred patterns:

- `PUT /v1/admin/categories/reorder`
- `PUT /v1/admin/projects/{project_id}/tasks/reorder`
- `POST /v1/admin/learning/modules/{module_id}/variants/reorder`

Rules:

- use a collection path plus `/reorder`
- `PUT` is preferred when the payload replaces ordering as a whole
- `POST` is tolerated when the existing family already uses action-posture and
  the payload is not a resource representation
- keep reorder separate from normal update endpoints

### Lifecycle action rule

Soft-delete, restore, and purge are lifecycle actions.

Preferred patterns:

- `POST /.../{id}/soft-delete`
- `POST /.../{id}/restore`
- `DELETE /.../{id}/purge`

Rules:

- soft-delete and restore are action posts
- purge is a destructive terminal delete and may use `DELETE`
- do not hide lifecycle actions behind ambiguous update payloads if the product
  already models them as explicit workflows

### Workflow transition rule

Claim, release, complete, skip, revoke, and similar transitions should use
explicit verbs.

Preferred patterns:

- `POST /.../{id}/claim`
- `POST /.../{id}/release`
- `POST /.../{id}/complete`
- `POST /.../{id}/skip`
- `POST /.../{id}/revoke`

Rules:

- use `POST` for workflow transitions unless there is a stronger established
  family rule
- keep the action verb explicit
- do not overload generic `PATCH` endpoints with hidden transition semantics

### Action naming rule

Prefer these stable verbs where they fit:

- `restore`
- `purge`
- `soft-delete`
- `reorder`
- `complete`
- `skip`
- `claim`
- `release`
- `revoke`
- `batch-delete`

Rules:

- prefer stable shared verbs over product-local synonyms
- avoid UI-oriented verbs like:
  - `open-modal`
  - `do-delete`
  - `quick-fix`
- if a domain truly needs a custom verb, keep it domain-real and explicit

## What Good Looks Like

Good outcomes:

- one action grammar across admin and product workflow families
- reorder, lifecycle, and transition routes are easy to spot
- collection actions and single-resource actions are clearly separated
- audits can tell resource shape from workflow shape quickly

Bad outcomes:

- action verbs are hidden inside generic update endpoints
- one app mixes `:batch-delete`, `/batch-delete`, and `/bulk-delete` without a
  reason
- workflow actions are named after UI affordances
- reorder is treated like a normal item patch

## Questions This Contract Should Settle

- When should an action be attached to a resource path versus a dedicated
  action family?
- Which verbs should Underlay prefer for common workflow transitions?
- When is `POST` right versus `PUT` or `DELETE` for action routes?
- How should batch and reorder actions be named?

## Assessment State

Assessed across all six consumer APIs by `g09.057` on 2026-08-27.

Verdict: `conforming` after the two bounded `g09.059` repairs.

- `g09.057` found Underlay Reference mixing `:batch-delete` with nested
  `/tasks/batch-delete`.
- `g09.057` found Compli Me mixing `/batch-delete` for its domain resources
  with `:batch-delete` for media.
- resource lifecycle, reorder, dedicated workflow, claim/release, and explicit
  transition routes conform across the fleet.
- Acowtancy's consistent `batch-soft-delete` vocabulary preserves a narrower
  app-owned lifecycle meaning and is not treated as synonym drift.

The operator chose `:batch-delete` as the canonical suffix, declared the
supported fleet caller set closed-world, and chose no compatibility window.
Underlay Reference PR9 and Compli Me PR8 completed the atomic caller and route
updates as merge commits `0109b906` and `a290d2a7`. `g09.059` is complete and
the assessed collection-action drift is repaired. See the
[`g09.057` assessment](../logs/2026-08/27-175930-g09-057-canonical-path-runtime-workflow-assessment.md).

## Next Task

No further `g09.059` work remains. Preserve `:batch-delete` as the canonical
collection-action suffix in future assessments.
