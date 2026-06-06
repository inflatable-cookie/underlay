# g06.136 - Media Storage Internal Split

## Why

`g06.135` found that `underlay-media/src/storage.rs` mixes storage key
configuration, generator behavior, object-key validation wrappers, prefix
helpers, and MIME filename policy in one file.

The public storage surface is stable and well-tested. The next step is an
internal split that keeps the same API while making the storage contract easier
to reason about.

## Goal

Split media storage into focused internal modules without changing public
imports, generated object key formats, validation behavior, or MIME mappings.

## Scope

In scope:

- replace `underlay-media/src/storage.rs` with a `storage/` module directory
- move `StorageKeyConfig` into `storage/config.rs`
- move `StorageKeyGenerator` and key/prefix helpers into `storage/generator.rs`
- move `version_filename(...)` and `mime_to_extension(...)` into
  `storage/filename.rs`
- keep `storage/mod.rs` as the public front door and re-export surface
- keep existing crate-local storage tests attached to the storage module

Out of scope:

- changing public storage names or method signatures
- changing stored object key formats
- changing object-key validation rules
- changing MIME extension mappings
- changing consumer apps

## Acceptance Criteria

- `underlay_media::storage::{...}` imports continue to compile
- root exports and rendition service usage continue to compile
- focused storage tests pass with all features
- full `underlay-media` tests pass with all features
- `effigy rust:check` passes
- roadmap artifact records the final module shape and public API impact

## Consumer Upgrade Impact

Expected impact: none.

This should be an internal module split. If consumer import paths or generated
storage keys need to change, stop and re-enter planning.

## Current State

`g06.136` is ready.

## Next Task

Execute `g06.136`: media storage internal split.
