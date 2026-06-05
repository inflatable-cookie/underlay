# g06.049 - Devtools Migration-Bundle Boundary Split

## Why

`g06.048` found that the runtime Rust surface is now cleaner after the object-key
lane, but `underlay-devtools` still mixes public bundle options, local OCI store
mechanics, media shard construction, replay helpers, and raw path/ref handling.

The Rust public API inventory already classifies this area as tooling-only and
candidate-type. It should be tightened before the next consumer-facing runtime
change.

## Goal

Split and tighten the migration-bundle/devtools boundary so refs, local store
paths, and media shard mapping behavior are easier to reason about and harder
to misuse.

## Scope

In scope:

- audit `underlay-devtools` migration bundle, seed bundle, local-store, and
  media-shard modules
- classify public option fields that should remain raw CLI inputs versus typed
  internal values
- add or promote typed helpers for migration bundle refs and local-store paths
  where the boundary is clear
- parse media-shard mapping object keys with the shared blob/media object-key
  rules when validating or generating shard payloads
- reduce large-file pressure where the split naturally falls out of the
  boundary work
- update the Rust public API inventory if the public devtools direction changes

Out of scope:

- changing consumer app runtime behavior
- moving app-specific migration behavior into Underlay
- changing persisted migration bundle formats unless the change is explicitly
  additive or versioned
- broad migration-core redesign
- release execution or publishing
- reverting unrelated local Rust edits

## Acceptance Criteria

- devtools local-store and bundle-ref handling has an explicit typed/raw policy
- media-shard object-key payload validation uses the same canonical key rules
  as runtime media storage
- public devtools APIs remain tooling-only and do not become runtime app
  contracts
- any module split preserves existing bundle build/publish/pull/run behavior
- known Effigy doctor backlog is updated or explicitly left as structural
  backlog

## Consumer Upgrade Impact

Expected impact: none for runtime consumers.

Potential impact: source changes only for consumers that directly call
`underlay-devtools` migration bundle APIs. Any such change must be classified
as additive, deprecation, or breaking before landing.

## Current State

`g06.049` is complete.

Artifact:

- [049 artifact](./049-devtools-migration-bundle-boundary-split-artifact.md)

## Next Task

Execute `g06.050`: migration-core public model modularity audit.
