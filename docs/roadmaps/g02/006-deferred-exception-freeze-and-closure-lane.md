# 006 - Deferred Exception Freeze And Closure Lane

Status: complete
Owner: repo maintainers
Updated: 2026-04-10

## Context

`g02.003`, `g02.004`, and `g02.005` completed the bounded downstream consumer
gates for:

- `compli-me/admin`
- bounded Songsprout UI families
- bounded `loophole/composer/composer-admin`

That means the broad six-family rollout question is now answered. What remains
is not another untouched consumer family. The remaining work is the set of
deferred workflow-local exceptions that were intentionally left app-local while
the generic proof-app posture was normalized.

## Goals

- inventory the deferred workflow-local exceptions that remain after the
  consumer gates
- confirm which should stay app-local and which merit future shared-surface
  planning
- close the broad consumer-family rollout line cleanly without drifting back
  into open-ended execution

## Non-Goals

- reopening broad consumer-family normalization
- treating all remaining local exceptions as shared-surface candidates
- forcing moderation, rules, hardware, billing, or Rust-side work into the
  current UI rollout lane
- widening into a new execution wave before the deferred exception posture is
  frozen

## Scope

### In scope

- deferred UI/workflow exceptions left open by:
  - `g02.004` Songsprout gate
  - `g02.005` Composer gate
- the planning/control surfaces needed to classify and close those exceptions

### Out of scope

- new shared primitive or composite implementation work
- broad code changes across the consumers
- reopening the proof-app freeze or the completed downstream consumer gates

## Execution Plan

### Batch 6.1 - Deferred Exception Inventory

- [x] inventory the deferred exceptions remaining after the consumer gates
- [x] classify each as app-local retained surface, possible future shared
      candidate, or deliberately deferred non-UI work

### Batch 6.2 - Closure Recommendation

- [x] state whether the broad consumer-family rollout line is complete enough
      to close
- [x] identify any narrow follow-on planning lanes that should exist after this
      closure
- [x] leave one explicit next roadmap task instead of reopening freeform churn

## Initial Working Set

The currently known deferred/local-exception families include:

- Songsprout
  - public auth entry routes
  - billing routes
  - `stem`
  - workflow-local catalogue, artist, program, and task rendering that was
    intentionally left app-owned
- Loophole Composer
  - moderation queue/detail semantics
  - rules engine and rule-test bench
  - grouped hardware-family and hardware-variant rendering
  - layout-shell work that was intentionally left out of the bounded gate

## Batch 6.1 Inventory

### App-local retained surfaces

These are still active UI/workflow surfaces, but the current evidence says they
should remain app-owned rather than being treated as shared-surface gaps:

- Songsprout
  - `greenhouse` catalogue browse
  - `greenhouse` artist detail
  - `greenhouse` ops staff-access workflow content
  - `bloom` workflow-local task/program/release rendering and status language
- Loophole Composer
  - moderation queue family
  - moderation detail family
  - rules engine family
  - grouped hardware-family and hardware-variant rendering
  - `composer-admin` layout-shell work

Why these stay local:

- they are workflow-heavy enough that the shared value is route-shell posture,
  which was already normalized in the bounded gates
- the remaining differences are domain semantics, custom vocabulary,
  workflow-specific action logic, or app-specific grouped rendering

### Possible future shared candidates

These are not active implementation targets now, but they are the only
remaining exceptions that might justify future shared-surface planning if they
recur outside their current app:

- Songsprout
  - richer catalogue tab and artist relationship composition if another app
    develops the same browse/detail posture
- Loophole Composer
  - moderation queue affordance patterns if a second consumer grows a truly
    similar review/moderation workflow
  - hardware grouped-family browse posture if another consumer proves a second
    grouped profile/catalog family with the same needs

Current judgment:

- none of these are strong enough yet to open a new shared-surface execution
  lane
- they should remain evidence-only until a second consumer proves the pattern

### Deliberately deferred non-UI work

These items remain outside the current consumer UI rollout line and should not
be treated as pending shared UI normalization:

- Songsprout
  - public auth entry routes
  - billing routes
  - `stem`
  - Rust route work in `nursery`
- Loophole Composer
  - `rules/test`
  - Rust-side Composer work

Why these stay deferred:

- they are either retained auth workflow territory, billing/product-workflow
  work, non-UI support surfaces, or Rust-side implementation that belongs to a
  different planning lane

## Batch 6.1 Outcome

The deferred-exception set is now concrete:

- most remaining exceptions are honest app-local retained surfaces
- a much smaller tail is only a possible future shared candidate if another
  consumer proves the same workflow shape
- the rest is deliberately deferred non-UI or Rust-side work and should stop
  being discussed as though it were unresolved UI normalization

## Batch 6.2 Closure Recommendation

### Broad rollout decision

The broad consumer-family rollout line is complete enough to close.

Reason:

- the proof-app freeze was completed in `g02.002`
- the downstream consumer gates were completed in:
  - `g02.003` for `compli-me/admin`
  - `g02.004` for the bounded Songsprout UI family
  - `g02.005` for the bounded Composer admin family
- the remaining deferred set is now explicitly classified, and it does not
  justify another broad consumer-family normalization lane

### Narrow follow-on lanes that may exist later

If future work is opened, it should use narrow domain-specific lanes rather
than another consumer-family rollout gate.

The only credible follow-on lane types are:

- a future shared-candidate evidence lane
  - only if a second consumer proves one of the current evidence-only patterns
    such as moderation affordances, grouped hardware/catalog browse posture, or
    richer catalogue/artist relationship composition
- a deferred auth/billing lane
  - only if retained auth entry routes or billing surfaces become an explicit
    priority across consumers
- a Rust/non-UI domain lane
  - for `stem`, `nursery`, Composer Rust work, or `rules/test` when those
    become active priorities

### Closure conclusion

There is no honest active follow-on execution wave inside the old broad
consumer-family rollout line.

That line should now be treated as closed. Future work must open a new narrow
lane explicitly instead of treating any deferred exception as an implied
continuation of the completed overhaul.

## Consumer Upgrade Impact

Impact class: `assessment`

This lane should primarily harden the planning posture. It should not itself
require broad consumer implementation work unless the inventory exposes a
misclassified live exception that must be corrected before closure.

## Exit Criteria

- the remaining deferred exceptions are explicitly inventoried
- each exception is classified as app-local, future shared candidate, or
  deliberately deferred non-UI work
- the broad consumer-family rollout line is either closed or narrowed into one
  explicit follow-on lane

## Next Task

Completion: the broad consumer-family rollout line is closed.

If work resumes, open one new narrow roadmap lane explicitly for the prioritized
domain instead of continuing `g02` as a freeform consumer-normalization queue.
