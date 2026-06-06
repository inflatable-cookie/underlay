# g06.126 - Migration-Core Verification Internal Split

## Why

`g06.125` found that `underlay-migration-core/src/verification.rs` mixes public
verification models, checksum hashing, verify-stage execution, plugin semantic
verification, artifact construction, and promotion blocker assembly in one
module.

The next split should make the verification safety boundary easier to reason
about without changing exported model names or migration verification behavior.

## Goal

Split migration-core verification into focused internal modules while
preserving the public verification API and serialized model shape.

## Scope

In scope:

- replace `verification.rs` with a `verification/` module directory
- keep `verification/mod.rs` as the public module front door
- move public verification enums and structs into `model.rs`
- move checksum generation into `checksum.rs`
- move verify-stage execution and built-in checks into `stage.rs`
- move artifact construction and promotion blocker assembly into `artifact.rs`
- preserve existing verification tests

Out of scope:

- changing migration public APIs
- changing verification check codes or blocker strings
- changing serialized verification artifact shape
- changing declarative rule engine behavior
- changing consumer apps

## Acceptance Criteria

- public root exports remain stable
- verification model serialization shape remains stable
- verification behavior coverage remains intact
- focused and full migration-core tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal module split. If exported verification APIs or serialized
artifact fields must change, stop and re-enter planning.

## Current State

`g06.126` is ready.

## Next Task

Execute `g06.126`: migration-core verification internal split.
