# g06.071 - Jobs-Postgres Repository Public Model Modularity Audit

## Why

After `g06.070`, the remaining Rust production high-error god-file is
`underlay-jobs-postgres/src/postgres.rs`.

The file is an adapter crate surface, so the public repository, notifier,
scheduler, extension trait, and SQL helper boundary needs an audit before any
split changes module shape.

## Goal

Classify the jobs-postgres repository public and internal model surface and
decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-jobs-postgres/src/postgres.rs` by public type, trait impl,
  SQL helper, and runtime helper family
- classify exported repository, scheduler, notifier, error, and extension
  trait surfaces
- scan internal and consumer usage for direct imports
- decide whether the next batch should split adapter internals, target another
  Rust production warning file, or defer behind a broader jobs-postgres
  checkpoint
- update the Rust public API inventory if the jobs-postgres boundary needs
  tighter wording

Out of scope:

- changing repository trait behavior
- changing SQL semantics or migration constants
- changing LISTEN/NOTIFY behavior
- changing scheduled-task behavior
- changing consumer app job runtime behavior

## Acceptance Criteria

- jobs-postgres surface is grouped by stable contract family
- import paths and internal call sites are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking jobs-postgres contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.071` is complete.

Artifact:

- [071 artifact](./071-jobs-postgres-repository-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.072`: jobs-postgres repository internal split.
