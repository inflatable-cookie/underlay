# 004 - Songsprout Pattern Rollout Gate

Status: complete
Owner: repo maintainers
Updated: 2026-04-10

## Context

`g02.003` proved that the frozen proof-app admin pattern set generalizes into
`compli-me/admin` without reopening proof-app family selection or needing new
shared Underlay or Poodle surfaces.

The next honest downstream consumer gate is Songsprout. The live UI surface is
split:

- `greenhouse` is the richer internal operator surface
- `bloom` is the artist-facing app surface
- `stem` is primarily API/client support and is not the next UI rollout target

This gate exists to test which parts of the frozen proof posture carry into
Songsprout cleanly and which parts are better treated as app-local workflow
composition.

## Goals

- use Songsprout as the next bounded downstream validation of the proof-app
  pattern freeze after `compli-me`
- keep `greenhouse` and `bloom` distinct instead of flattening Songsprout into
  one fake app surface
- identify the strongest direct-rollout families before any new execution
  starts

## Non-Goals

- widening into `stem` as part of the UI rollout gate
- treating all Songsprout route drift as a shared-surface problem
- reopening proof-app family selection before Songsprout is classified
- forcing admin-heavy proof patterns onto artist-facing routes that are really
  workflow-local

## Scope

### In scope

- `songsprout/greenhouse`
- `songsprout/bloom`
- already-proven overview, browse, detail, recovery, auth/account, and
  workflow-launch patterns where they actually map
- Underlay/Poodle recipe surfaces only when the Songsprout rollout exposes a
  real shared gap

### Out of scope

- `songsprout/stem`
- Rust API route work in `nursery`
- reopening the proof-app lane
- widening immediately into `loophole/composer`

## Execution Plan

### Batch 4.1 - Songsprout Family Inventory

- [x] inventory the active `greenhouse` and `bloom` route families that map to
      the frozen proof-app pattern set
- [x] classify each family as direct rollout, local exception, or deferred

### Batch 4.2 - First Bounded Songsprout Rollout

- [x] normalize the strongest direct-rollout Songsprout families onto the
      frozen proof-app posture
- [x] keep workflow-local route copy, task logic, and local cards/rows app-owned

### Batch 4.3 - Generalization Report

- [x] record which proof-app patterns survived the Songsprout rollout
- [x] name app-local exceptions explicitly
- [x] choose the next downstream family only after that report

## Initial Evidence

Current route evidence suggests a meaningful split:

### `greenhouse`

- overview route at `(app)/+page.svelte`
- catalogue browse and artist detail routes
- programs and ops workflow routes
- billing and auth routes

### `bloom`

- overview route at `(app)/+page.svelte`
- programs, tracks, releases, tasks, identity, security, and billing routes
- auth routes

### Working hypothesis

- likely direct-rollout candidates:
  - overview shells
  - auth/account/security posture
  - browse and empty/recovery posture on list-like workflow pages
- likely local exceptions:
  - artist/program/task workflow content and status vocabulary
  - catalogue tab logic and workflow-local table/list rendering
- likely deferred:
  - `stem`
  - Rust-side route work
  - any route family that proves more workflow-specific than the current proof
    posture can responsibly absorb

## Route Family Inventory

### `greenhouse`

The active UI families are:

- overview route
  - `(app)/+page.svelte`
- workflow browse routes
  - `(app)/programs/+page.svelte`
  - `(app)/ops/+page.svelte`
- catalogue family
  - `(app)/catalogue/+page.svelte`
  - `(app)/catalogue/artists/[artistId]/+page.svelte`
- account/billing shell
  - `(app)/billing/+page.svelte`
- auth entry routes
  - `(auth)/login/+page.svelte`
  - `(auth)/auth/callback/+page.svelte`

### `bloom`

The active UI families are:

- overview route
  - `(app)/+page.svelte`
- artist workflow browse routes
  - `(app)/programs/+page.svelte`
  - `(app)/tracks/+page.svelte`
  - `(app)/releases/+page.svelte`
  - `(app)/tasks/+page.svelte`
- signed-in profile/security routes
  - `(app)/identity/+page.svelte`
  - `(app)/security/+page.svelte`
  - `(app)/billing/+page.svelte`
- auth entry routes
  - `(auth)/login/+page.svelte`
  - `(auth)/auth/callback/+page.svelte`

## Classification

### Direct rollout

These families map cleanly enough to the frozen proof-app posture to be the
first Batch 4.2 targets:

- `greenhouse` overview route
- `bloom` overview route
- `bloom` signed-in security route
- `bloom` artist workflow browse routes for programs, tracks, releases, and
  tasks
- `greenhouse` ops browse posture where it overlaps with the proven ops/review
  queue, empty-state, and recovery shells

### Local exception

These families should remain app-local even when some shell posture
normalization happens around them:

- `greenhouse` catalogue browse and artist detail routes
  - tab logic, artist/program/task relationship rendering, and custom table/list
    posture are workflow-local
- `greenhouse` ops staff-access workflow and health-card content
- `bloom` workflow status language, task/program semantics, and local card/list
  rendering
- `bloom` identity route content and workflow copy

### Deferred

These families should not drive the first Songsprout rollout wave:

- `greenhouse` and `bloom` auth entry routes
  - they belong to the retained auth workflow lane and should not lead the
    first Songsprout consumer gate
- `greenhouse` and `bloom` billing routes
  - adjacent to shared shells, but not part of the strongest proved family set
    for this gate
- `stem`
- Rust route work in `nursery`

## Batch 4.1 Outcome

The Songsprout gate is now concrete:

- Batch 4.2 should start with overview shells, `bloom` signed-in security, the
  artist workflow browse routes in `bloom`, and the ops browse shell in
  `greenhouse`
- the catalogue family is explicitly treated as a local exception, not a hidden
  shared-surface gap
- public auth, billing, `stem`, and Rust route work are explicitly deferred

## Batch 4.2 Outcome

The bounded rollout completed cleanly across the strongest direct-rollout
families:

- `greenhouse`
  - overview route normalized onto the proof-app overview shell
  - ops route normalized onto the shared route-shell posture while keeping
    workflow-local ops content app-owned
- `bloom`
  - overview route normalized onto the proof-app overview shell
  - security route normalized onto the shared signed-in security shell
  - workflow browse routes for programs, tracks, releases, and tasks
    normalized onto the shared browse-shell posture

What stayed local on purpose:

- `greenhouse` catalogue browse and artist detail
- Songsprout-specific workflow wording, status vocabulary, and list rendering
- local task/program semantics and action flows

Execution note:

- `greenhouse` still has pre-existing Svelte 5 state-capture warnings in the
  catalogue and programs routes
- those warnings are outside the normalized direct-rollout slice and did not
  block the bounded gate result

## Batch 4.3 Generalization Report

The Songsprout gate proved a narrower but still useful generalization outcome
than `compli-me`.

Patterns that survived cleanly:

- overview shells
- signed-in security/account shell posture
- browse-page shell posture for list-like workflow routes
- empty and recovery posture using `Callout`, `EmptyState`, and local filter
  controls

Patterns that remained app-local:

- catalogue tab systems and artist-detail composition
- richer workflow-local status rendering and task/program semantics
- workflow-specific action rows and domain copy

Shared-surface outcome:

- no new Underlay shared workflow surface was needed
- no new Poodle primitive or composite was needed
- the existing proof-app recipe set remained sufficient once Songsprout route
  shells were aligned

Next downstream choice:

- `loophole/composer` is the next honest downstream consumer gate
- it should be opened as its own bounded rollout lane rather than treated as an
  automatic continuation of Songsprout

## Consumer Upgrade Impact

Impact class: `consumer-visible`

This gate may change Songsprout page composition and route-shell posture where
the frozen proof-app patterns generalize cleanly, but it should avoid pushing
Songsprout-specific workflow logic into shared layers.

## Exit Criteria

- Songsprout route families are classified from evidence rather than assumption
- the strongest rollout families are normalized cleanly
- local exceptions are recorded explicitly
- the next consumer decision is based on the Songsprout result rather than
  queue momentum

Status check: met.

## Next Task

Open the next bounded downstream consumer gate for `loophole/composer`, using
the completed Songsprout result as the next generalization baseline instead of
reopening proof-app family selection.
