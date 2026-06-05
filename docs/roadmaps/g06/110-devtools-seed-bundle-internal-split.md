# g06.110 - Devtools Seed Bundle Internal Split

## Why

`g06.109` found that `underlay-devtools/src/seed_bundle.rs` mixes public
options/reports, private package models, build, publish delegation, pull
extraction, digest/layer helpers, filesystem output setup, and payload decoding
in one production file.

The next split should reduce reasoning load without changing seed-bundle APIs,
file formats, CLI behavior, or migration-bundle reuse.

## Goal

Split the devtools seed-bundle implementation into focused internal modules
while preserving crate-root exports and behavior.

## Scope

In scope:

- keep `seed_bundle.rs` as the small module front door
- move public option/report types and private manifest model into a focused
  model module
- move private bundle-package and helper functions into a focused package
  module
- move build behavior into a focused build module
- move publish delegation into a focused publish module
- move pull/extraction behavior into a focused pull module
- preserve existing devtools tests

Out of scope:

- changing devtools public APIs
- changing seed-bundle package JSON shape
- changing SQL file ordering or layer annotations
- changing publish/pull local-store behavior
- changing consumer apps

## Acceptance Criteria

- `seed_bundle.rs` becomes a small module front door
- responsibility groups live in focused modules
- crate-root seed-bundle exports remain stable
- devtools tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal tooling split. If public exports, CLI-visible behavior, or
seed-bundle formats must change, stop and re-enter planning.

## Current State

`g06.110` is ready.

## Next Task

Execute `g06.110`: devtools seed bundle internal split.
