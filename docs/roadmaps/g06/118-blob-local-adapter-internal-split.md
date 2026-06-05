# g06.118 - Blob Local Adapter Internal Split

## Why

`g06.117` found that `underlay-blob/src/adapters/local.rs` mixes config,
adapter construction, object-key validation, path containment, file I/O,
cleanup, adapter trait methods, MIME guessing, debug formatting, and tests in
one production file.

The next split should reduce reasoning load without changing local adapter APIs
or path-safety behavior.

## Goal

Split the blob local adapter into focused internal modules while preserving
public exports and behavior.

## Scope

In scope:

- keep `adapters/local.rs` as the small module front door
- move `LocalConfig` into a focused config module
- move `LocalAdapter`, construction, debug, helper methods, and `BlobAdapter`
  impl into a focused adapter module
- move key/path-safety helpers into a focused path module
- move content-type guessing into a focused MIME module
- preserve existing local adapter tests

Out of scope:

- changing blob public APIs
- changing object-key validation or path-safety semantics
- changing local storage behavior
- changing consumer apps

## Acceptance Criteria

- `local.rs` becomes a small module front door
- responsibility groups live in focused modules
- `underlay_blob::{LocalAdapter, LocalConfig}` exports remain stable
- blob tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports or path-safety behavior must
change, stop and re-enter planning.

## Current State

`g06.118` is complete.

Artifact:

- [118 artifact](./118-blob-local-adapter-internal-split-artifact.md)

## Next Task

Execute `g06.119`: jobs runner tests modularity audit.
