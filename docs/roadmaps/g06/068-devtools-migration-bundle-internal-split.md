# g06.068 - Devtools Migration-Bundle Internal Split

## Why

`g06.067` found that `underlay-devtools/src/migration_bundle.rs` has a narrow
public crate-root API but still mixes public models, bundle package codec,
build orchestration, publish/pull routing, output writing, and run prep.

Devtools is tooling-only, but bundle replay and registry behavior should remain
easy to audit.

## Goal

Split migration-bundle internals into focused private modules while preserving
the existing crate-root API, package shape, digest validation, local/remote
store behavior, and CLI behavior.

## Scope

In scope:

- split public option/report/error/ref model into a focused module if it keeps
  crate-root exports intact
- split package encode/decode, payload decode, digest, and layer helpers into a
  codec/package helper module
- split pulled-output writing into a focused output module
- split build orchestration from publish/pull/run routing
- keep existing `local_store`, `remote_registry`, and `media_shards` behavior
- update tests only where private helper imports need explicit module paths

Out of scope:

- changing public option/report/error/ref type names or fields
- changing build, publish, pull, or run function signatures
- changing digest-pinned reference requirements
- changing local-store fallback order
- changing registry protocol behavior
- changing package JSON, OCI layout, sidecars, or media-shard mapping shapes
- changing seed-bundle behavior
- consumer rollout unless public crate-root exports move

## Acceptance Criteria

- `migration_bundle.rs` becomes a smaller public tooling front door or thin
  coordinator
- crate-root `underlay_devtools::*` migration-bundle exports remain
  source-compatible
- migration-bundle tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, public fields, function
signatures, package shape, digest semantics, or local/remote store behavior
must change, stop and re-enter planning.

## Current State

`g06.068` is next.

## Next Task

Execute `g06.068`: devtools migration-bundle internal split.
