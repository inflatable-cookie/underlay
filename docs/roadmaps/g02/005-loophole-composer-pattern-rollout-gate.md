# 005 - Loophole Composer Pattern Rollout Gate

Status: complete
Owner: repo maintainers
Updated: 2026-04-09

## Context

`g02.004` proved that the frozen proof-app posture and the first downstream
generalization result from `compli-me/admin` carry into a narrower
workflow-heavy consumer family without needing new shared Underlay or Poodle
surfaces.

The next honest downstream gate is `loophole/composer`. `composer-admin` is a
useful test because it mixes straightforward admin CRUD surfaces with more
domain-specific moderation, rules, and hardware workflows. This gate exists to
separate those two classes explicitly instead of treating Composer as either
"just another admin" or "too custom to normalize at all."

## Goals

- use `composer-admin` as the next bounded downstream validation of the frozen
  proof-app pattern set
- normalize the standard browse/detail/edit families that map cleanly onto the
  proof-app posture
- name the workflow-local moderation, rules-testing, and hardware surfaces
  explicitly instead of widening the shared layers by assumption

## Non-Goals

- widening immediately into the wider `loophole/composer` family beyond
  `composer-admin`
- reopening proof-app family selection
- treating all Composer differences as shared-surface gaps
- forcing workflow-local moderation or rule-testing surfaces onto generic admin
  recipes before the simple CRUD families are validated

## Scope

### In scope

- `loophole/composer/composer-admin`
- already-proven overview, browse, detail, edit, recovery, and nested
  child-collection patterns where they actually map
- Underlay/Poodle recipe surfaces only when the Composer rollout exposes a real
  shared gap

### Out of scope

- `composer-api-client`
- Rust-side Composer work
- broad rule-engine redesign
- widening immediately into deferred Composer workflow families after the first
  bounded rollout

## Execution Plan

### Batch 5.1 - Composer Family Inventory

- [x] inventory the active `composer-admin` route families against the frozen
      proof-app pattern set
- [x] classify each family as direct rollout, local exception, or deferred

### Batch 5.2 - First Bounded Composer Rollout

- [x] normalize the strongest direct-rollout Composer families onto the frozen
      proof-app posture
  - [x] first rollout slice:
        - app overview route
        - product CRUD family
        - vendor CRUD family
        - scan history browse shell
  - [x] remaining direct-rollout families:
        - variant browse/detail family
        - parameter CRUD family
        - semantic role CRUD family
- [x] keep rule-testing, moderation semantics, and grouped hardware rendering
      app-local unless a real shared gap appears

### Batch 5.3 - Generalization Report

- [x] record which proof-app patterns survive the Composer rollout
- [x] name local Composer exceptions explicitly
- [x] choose the next downstream family only after that report

## Route Family Inventory

The active `composer-admin` route families are:

- app overview route
  - `+page.svelte`
- product CRUD family
  - `products/+page.svelte`
  - `products/[id]/+page.svelte`
  - `products/[id]/edit/+page.svelte`
- vendor CRUD family
  - `vendors/+page.svelte`
  - `vendors/[id]/+page.svelte`
  - `vendors/[id]/edit/+page.svelte`
- variant browse/detail family
  - `variants/+page.svelte`
  - `variants/[id]/+page.svelte`
- parameter CRUD family
  - `parameters/+page.svelte`
  - `parameters/[id]/+page.svelte`
- semantic role CRUD family
  - `semantic-roles/+page.svelte`
  - `semantic-roles/[id]/+page.svelte`
  - `semantic-roles/new/+page.svelte`
- moderation queue family
  - `moderation/+page.svelte`
  - `moderation/[id]/+page.svelte`
- scan history family
  - `scans/+page.svelte`
- rules engine family
  - `rules/+page.svelte`
  - `rules/[id]/+page.svelte`
  - `rules/[id]/edit/+page.svelte`
  - `rules/new/+page.svelte`
  - `rules/sets/[id]/+page.svelte`
  - `rules/sets/[id]/edit/+page.svelte`
  - `rules/sets/new/+page.svelte`
  - `rules/test/+page.svelte`
- hardware family
  - `hardware/+page.svelte`
  - `hardware/families/[id]/+page.svelte`
  - `hardware/families/new/+page.svelte`
  - `hardware/variants/[id]/+page.svelte`
  - `hardware/variants/new/+page.svelte`

## Classification

### Direct rollout

These families map closely enough to the frozen proof-app posture to drive
Batch 5.2:

- app overview route
- product CRUD family
- vendor CRUD family
- variant browse/detail family
- parameter CRUD family
- semantic role CRUD family
- scan history browse shell

### Local exception

These families should participate only where shared shell posture clearly maps,
while their domain semantics and richer composition stay app-local:

- moderation queue family
  - queue semantics, bulk moderation actions, and moderation vocabulary are
    workflow-local even if browse-shell posture later normalizes
- rules engine family
  - rule-card language, rule-set semantics, and the rule test bench are local
    workflow surfaces rather than generic CRUD pages
- hardware family
  - grouped vendor/product-line rendering, hardware profile stats, and family
    card posture are local domain composition

### Deferred

These families should not drive the first Composer rollout slice:

- `rules/test/+page.svelte`
  - this is a workflow-local diagnostic bench, not a proof-app pattern target
- moderation detail posture beyond simple shell alignment
  - adjacent to shared review/detail patterns, but not the strongest first
    Composer slice while queue semantics remain local
- any layout-shell work
  - relevant context, but not a primary rollout target for this gate

## Batch 5.1 Outcome

The Composer gate is now concrete:

- Batch 5.2 should start with the overview route, the standard CRUD families
  for products, vendors, parameters, semantic roles, variants, and the scan
  history browse shell
- moderation, rules, and hardware are explicitly prevented from silently
  widening the first rollout slice
- Composer now tests whether the frozen proof-app posture generalizes to a
  heavier admin CRUD consumer without conflating that with workflow-local rule
  tooling

## Consumer Upgrade Impact

Impact class: `consumer-visible`

This gate may change consumer-visible page composition in `composer-admin`
where the frozen proof-app patterns genuinely map, but it should not widen the
shared Underlay or Poodle surface without evidence from the bounded rollout.

## Exit Criteria

- `composer-admin` is assessed against the frozen proof-app pattern set
- direct-rollout families are normalized cleanly
- moderation, rules, and hardware exceptions are recorded explicitly
- the next downstream decision is based on this result instead of assumption

## Batch 5.3 Outcome

The Composer rollout proved the frozen proof-app posture survives a
workflow-heavier admin consumer without needing new Underlay or Poodle
surfaces.

### Proof-app patterns that generalized cleanly

- overview-page shell
- browse/list shell with filter toolbar and recovery posture
- detail-page shell with `PageHeader`, `MetaBar`, and summary-first card
  structure
- edit/create route shell with current Poodle form primitives
- empty/recovery posture for list and detail routes
- compact child/related collection posture where it actually maps

### Explicit local Composer exceptions

- moderation queue family
  - queue semantics, moderation vocabulary, and batch moderation actions stay
    local
- rules engine family
  - rule-set semantics, rule-editing composition, and the rule-test bench stay
    local
- hardware family
  - grouped hardware-family rendering, family metrics, and hardware-specific
    browse card posture stay local

### Downstream decision

There is no additional untouched major consumer family left to open as a broad
downstream normalization gate:

- proof-app families already cover `acowtancy`, `underlay-reference`, and
  `contact-patch`
- `g02.003` covered `compli-me/admin`
- `g02.004` covered the bounded Songsprout UI family
- `g02.005` covered the bounded Composer admin family

The next honest lane is therefore **not** another consumer-family rollout. It
is a bounded deferred-exception and closure lane for the remaining
workflow-local exceptions that were intentionally left app-local during the
consumer gates.

## Next Task

Execute `g02.006` as the deferred-exception and closure lane: inventory the
remaining workflow-local exceptions across Songsprout and Composer, confirm
which should stay app-local versus merit future shared-surface planning, and
close the broad consumer-family rollout line without reopening freeform
execution.
