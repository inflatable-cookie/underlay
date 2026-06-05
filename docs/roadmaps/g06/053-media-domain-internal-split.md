# g06.053 - Media Domain Internal Split

## Why

`g06.052` ranked `underlay-media/src/domain.rs` as the next highest-value Rust
structural target. It is a large production/shared public model file and has a
clean internal split shape.

The split should make the media model easier to reason about without changing
the public API.

## Goal

Split `underlay-media/src/domain.rs` into focused internal modules while
preserving root exports and serialized model shapes.

## Scope

In scope:

- split identifiers into an internal identifiers module
- split rendition and usage enums into internal enum modules
- split core entities into an internal entities module
- split media usage edge and migrated attachment binding types into internal
  usage modules
- split create/update/finalize/rendition/list input types into an internal
  inputs module
- preserve `underlay_media` root re-exports
- preserve `underlay_media::domain::*` compatibility where possible
- update tests only where module paths require it

Out of scope:

- changing repository traits
- changing stored JSON or SQL row shapes
- changing media Postgres adapter behavior
- changing object-key semantics
- release execution or publishing

## Acceptance Criteria

- `domain.rs` becomes a small module front door
- public exports remain source-compatible
- `underlay-media` tests pass
- `underlay-media-postgres` tests/checks pass if affected
- consumers do not need source changes unless a hidden import path moves

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If public imports or serialized shapes must
move, stop and re-enter planning.

## Current State

`g06.053` is complete.

Artifact:

- [053 artifact](./053-media-domain-internal-split-artifact.md)

## Next Task

Execute `g06.054`: media renditions internal split.
