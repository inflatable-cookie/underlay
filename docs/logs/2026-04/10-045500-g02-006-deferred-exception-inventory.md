# 2026-04-10 04:55 - g02.006 deferred exception inventory

## Context

`g02.003`, `g02.004`, and `g02.005` completed the downstream consumer gates for
`compli-me`, Songsprout, and Composer.

`g02.006` exists to stop the broad rollout line from drifting into perpetual
"one more consumer family" language and to classify the remaining deferred
exceptions honestly.

## Inventory Result

The remaining deferred set falls into three buckets.

### App-local retained surfaces

- Songsprout catalogue browse and artist detail
- Songsprout workflow-local task/program/release rendering and status language
- Songsprout ops staff-access content
- Composer moderation queue/detail semantics
- Composer rules engine family
- Composer grouped hardware-family and hardware-variant rendering
- Composer layout-shell work

These still have active UI value, but the evidence says they should remain
app-local.

### Possible future shared candidates

- Songsprout catalogue/artist relationship composition if another consumer
  proves the same workflow shape
- Composer moderation affordance patterns if another consumer develops a truly
  similar moderation workflow
- Composer grouped hardware-family browse posture if another consumer proves
  the same grouped profile/catalog pattern

These are not active implementation targets now.

### Deliberately deferred non-UI work

- Songsprout public auth entry routes
- Songsprout billing routes
- Songsprout `stem`
- Songsprout Rust route work in `nursery`
- Composer `rules/test`
- Rust-side Composer work

These belong to other lanes and should stop being discussed as unresolved UI
normalization.

## Planning Effect

Batch 6.1 is complete. The next remaining question is whether the broad
consumer-family rollout line is complete enough to close and, if so, which
narrow follow-on lanes should remain after that closure.

## Next Task

Execute `g02.006` Batch 6.2 by deciding whether the broad consumer-family
rollout line is complete enough to close, naming any narrow follow-on planning
lanes that should exist after closure, and leaving one explicit next roadmap
task instead of reopening freeform execution.
