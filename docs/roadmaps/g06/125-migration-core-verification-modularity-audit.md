# g06.125 - Migration-Core Verification Modularity Audit

## Why

After `g06.124`, the next Rust production warning-level file in the god-file
report is `underlay-migration-core/src/verification.rs`.

Migration verification is part of the migration safety boundary. It should be
split from evidence about verification rule families, execution behavior, error
reporting, and public model impact, not from file size alone.

## Goal

Classify the migration-core verification surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/verification.rs` by responsibility family
- identify verification models, rule execution, result/error reporting, helper,
  and test boundaries
- identify public API or migration behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader migration verification checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing migration verification semantics
- changing bundle or pipeline behavior
- changing consumer apps

## Acceptance Criteria

- verification responsibilities are grouped by stable behavior family
- public API and migration behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds verification
behavior that must change, stop and re-enter planning.

## Current State

`g06.125` is complete.

Artifact:

- [125 artifact](./125-migration-core-verification-modularity-audit-artifact.md)

## Next Task

Execute `g06.126`: migration-core verification internal split.
