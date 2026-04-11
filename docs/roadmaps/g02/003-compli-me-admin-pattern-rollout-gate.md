# 003 - compli-me Admin Pattern Rollout Gate

Status: complete
Owner: repo maintainers
Updated: 2026-04-10

## Context

`g02.002` froze the proof-app lane around Underlay, Poodle, Dairy,
`acme-admin`, and `cp-admin`, then chose `compli-me` as the first downstream
consumer gate.

The proof-app recipe spine is already coherent enough. The next honest test is
whether those admin/media/detail/list/dialog/form/recovery patterns roll into
`compli-me/admin` cleanly without reopening broad proof-app family selection.

## Goals

- use `compli-me/admin` as the first downstream consumer validation of the
  frozen proof-app admin pattern set
- keep the rollout bounded to already-proven families
- record which patterns generalize cleanly and which remain app-local

## Non-Goals

- widening immediately into `songsprout` or `loophole/composer`
- opening new shared UI surfaces before `compli-me` proves they are needed
- treating every `compli-me` difference as a shared-surface problem

## Scope

### In scope

- `compli-me/admin`
- already-proven admin/media/list/detail/dialog/form/recovery families
- Underlay/Poodle recipe surfaces only when the rollout exposes a real shared
  gap

### Out of scope

- `compli-me` non-admin surfaces
- other non-proof consumer families
- new proof-app family selection

## Execution Plan

### Batch 3.1 - Route Family Inventory

- [x] inventory the active `compli-me/admin` route families that map to the
      frozen proof-app pattern set
- [x] classify each candidate route family as direct rollout, local exception,
      or deferred

### Batch 3.2 - Bounded Rollout

- [x] normalize the direct-rollout `compli-me/admin` families onto the frozen
      proof-app posture
- [x] keep route-local differences app-owned unless a real shared gap appears

### Batch 3.3 - Generalization Report

- [x] record which proof-app patterns survived the first downstream rollout
- [x] name any app-local exceptions explicitly
- [x] choose the next downstream family only after that report

## Route Family Inventory

The active `compli-me/admin` route families are:

- app overview shell
  - `(app)/+page.svelte`
- account and signed-in auth/security pages
  - `(app)/account/+page.svelte`
  - `(app)/account/2fa/+page.svelte`
  - `(app)/account/passkeys/+page.svelte`
  - `(app)/account/password/+page.svelte`
- users admin family
  - `(app)/users/+page.svelte`
  - `(app)/users/new/+page.svelte`
  - `(app)/users/[userId]/+page.svelte`
  - `(app)/users/[userId]/edit/+page.svelte`
- system ops family
  - `(app)/system/+page.svelte`
  - `(app)/system/jobs/+page.svelte`
  - `(app)/system/jobs/[id]/+page.svelte`
  - `(app)/system/errors/+page.svelte`
  - `(app)/system/errors/[id]/+page.svelte`
  - `(app)/system/scheduled-tasks/+page.svelte`
  - `(app)/system/scheduled-tasks/[id]/+page.svelte`
  - `(app)/system/emails/+page.svelte`
  - `(app)/system/emails/[id]/+page.svelte`
  - `(app)/system/audit/+page.svelte`
- compliments CRUD family
  - browse routes for `people`, `businesses`, and `messages`
  - detail routes for `people/[personId]`, `businesses/[businessId]`, and
    `messages/[complimentId]`
  - create/edit routes for those same entities
  - trash route at `(app)/compliments/trash/+page.svelte`
- public auth entry pages
  - `(auth)/login/+page.svelte`
  - `(auth)/forgot-password/+page.svelte`

## Classification

### Direct rollout

These families map closely to the frozen proof-app pattern set and should be
the first bounded rollout targets in Batch 3.2:

- users admin family
- system ops family except audit
- account and signed-in auth/security pages
- app overview shell
- compliments CRUD browse/detail/edit/trash family

### Local exception

These route families still participate in Batch 3.2, but some surface details
should remain app-owned even when the shell normalizes:

- compliment/person/business wording and action vocabulary
- compliment-specific empty copy, confirm copy, and filter wording
- local compliment entity cards, menus, and row rendering

### Deferred

These routes should not drive the first downstream rollout:

- `system/audit/+page.svelte`
  - adjacent to ops, but not part of the strongest frozen proof family yet
- public auth entry pages
  - belong to the retained Underlay auth workflow surface, not the main
    `compli-me/admin` rollout gate
- app/layout shell files
  - relevant context, but not primary rollout targets for this wave

## Batch 3.1 Outcome

The first downstream gate is now concrete:

- Batch 3.2 should focus on `compli-me/admin` users, system ops, account, app
  overview, and compliments CRUD/trash
- audit and public auth are explicitly deferred
- compliments remains a valid rollout target because its shell posture is
  already proven, while its domain wording stays local

## Batch 3.2 Progress

The bounded rollout completed cleanly in `compli-me/admin`.

Completed across Batch 3.2:

- app overview route
  - normalized onto the proof-app overview shell with `PageHeader`, a host-owned
    health metric band, and navigational `NavCard` sections
- users family
  - normalized badge posture and shared date formatting on list/detail surfaces
- system queue browse routes
  - normalized jobs and errors onto the current `DataTable` expansion contract
- compliments browse family
  - normalized list filters onto the current `Select` value-change contract
  - normalized card pills onto the badge-tone posture used in the proof apps
- compliments detail/edit family
  - normalized onto the current detail-shell posture with `MetaBar`,
    copyable IDs, badge-style moderation/status posture, and explicit
    timestamp formatting
- signed-in account/security and the remaining system detail routes
  - assessed against the proof-app posture and left unchanged where they
    already matched closely enough to avoid churn

Current execution rule:

- keep `compli-me` wording, card copy, filter vocabulary, and domain actions
  app-local
- only lift a change into Underlay or Poodle if this rollout exposes a real
  shared gap rather than consumer drift

## Batch 3.3 Generalization Report

The first downstream gate proved that the frozen proof-app admin pattern set
generalizes into `compli-me/admin` without reopening proof-app family
selection.

Patterns that survived cleanly:

- overview shell
- diagnostics and review-style queue browse posture
- detail-shell metadata with `MetaBar`, copyable IDs, and badge-style status
  pills
- `SpaFormShell` edit-route feedback posture
- list filter posture with current `Select` events
- compliments/media-style card browse posture with local domain cards and menus
- destructive and guarded action posture via existing Poodle dialog primitives

App-local exceptions that stayed local:

- compliment-specific wording, filter labels, and moderation vocabulary
- compliment card bodies, action menus, and row rendering
- route-specific domain copy inside overview and detail sections

Shared-surface outcome:

- no new Underlay shared workflow surface was needed
- no new Poodle primitive or composite was needed
- the frozen proof-app recipes were sufficient once consumer drift was removed

Next downstream choice:

- `songsprout` is the next honest downstream consumer family
- it should be treated as a new bounded gate rather than an automatic extension
  of `g02.003`

## Consumer Upgrade Impact

Impact class: `consumer-visible`

This wave is the first downstream rollout beyond the proof apps. It may change
consumer-visible page composition and shared recipe language where the
`compli-me/admin` family proves the frozen proof patterns generalize.

## Exit Criteria

- `compli-me/admin` has been assessed against the frozen proof-app patterns
- direct-rollout families are normalized cleanly
- local exceptions are named explicitly rather than pushed into shared layers
- the next downstream consumer decision is based on this result rather than
  assumption

Status check: met.

## Next Task

Open the next bounded downstream gate for `songsprout`, using the completed
`compli-me/admin` result as the generalization baseline instead of reopening
proof-app family selection.
