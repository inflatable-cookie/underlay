# g06.067 - Devtools Migration-Bundle Public Model Modularity Audit

## Why

After `g06.066`, the largest remaining Rust production god-file is
`underlay-devtools/src/migration_bundle.rs`.

Devtools is tooling-only, but migration bundle behavior touches replay,
registry, and local store workflows. The next move should classify the public
model before changing file shape.

## Goal

Classify the devtools migration-bundle public and internal model surface and
decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-devtools/src/migration_bundle.rs` by helper family
- classify exported bundle types, option structs, store helpers, registry
  helpers, and replay behavior
- scan internal and consumer usage for direct imports or tooling-only reliance
- decide whether the next batch should split migration-bundle internals, target
  `verification_rules.rs`, or defer behind a broader devtools checkpoint
- update the Rust public API inventory if the devtools migration-bundle
  boundary needs tighter wording

Out of scope:

- changing migration bundle archive or replay behavior
- changing digest validation behavior
- changing registry or local store semantics
- changing app runtime contracts
- changing migration-core behavior

## Acceptance Criteria

- migration-bundle surface is grouped by stable contract family
- import paths and internal call sites are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking devtools contract change
must be promoted into an explicit follow-up card before execution.

## Current State

`g06.067` is complete.

Artifact:

- [067 artifact](./067-devtools-migration-bundle-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.068`: devtools migration-bundle internal split.
