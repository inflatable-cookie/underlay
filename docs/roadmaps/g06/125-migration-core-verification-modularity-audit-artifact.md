# g06.125 Artifact - Migration-Core Verification Modularity Audit

## Summary

`underlay-migration-core/src/verification.rs` is the next Rust production
warning-level file after `g06.124`. It owns the public verification model
surface plus stage verification, checksum generation, and promotion artifact
construction.

The declarative rule engine is already split under `verification_rules/`, so
this file should not absorb more rule-engine behavior.

The current file groups:

- public verification enums and structs
- transform checksum generation
- `verify_stage(...)` orchestration
- built-in decision coverage, unresolved decision, governance, and checksum
  checks
- plugin semantic verification integration
- verification artifact construction from `PipelineRunReport`
- promotion blocker assembly
- crate-local verification tests

## Boundary Evidence

The public surface is re-exported from `src/lib.rs`:

- `build_verification_artifact`
- `transform_checksum`
- `verify_stage`
- `VerificationArtifact`
- `VerificationCheckResult`
- `VerificationChecksumSection`
- `VerificationInput`
- `VerificationIntegrityGateSection`
- `VerificationIssue`
- `VerificationPromotionGate`
- `VerificationReferentialIntegritySection`
- `VerificationReport`
- `VerificationRowCountSection`
- `VerificationSeverity`

Current cross-module users include:

- `plugin.rs` for `VerificationInput` and `VerificationIssue`
- `pipeline/types.rs` for `VerificationCheckResult` and `VerificationIssue`
- `pipeline/orchestrator/stages.rs` for `transform_checksum`,
  `verify_stage`, and `VerificationInput`
- `verification_rules/*` for the public model types
- recovery, drift, audit, and tests through pipeline report fields

The split must preserve the `crate::verification::{...}` public re-export
surface and the root `underlay_migration_core::{...}` exports.

## Behavior Evidence

Existing verification tests cover:

- promotion blocker assembly from a failed pipeline report
- declarative rule failure readability
- benchmark coverage for declarative and plugin verification paths

Baseline validation:

- `cargo test -p underlay-migration-core verification --all-features`
- 3 focused verification tests passed

## Decision

Queue `g06.126` as a migration-core verification internal split.

Suggested module shape:

- `verification/mod.rs`: public verification front door, re-exports, and test
  module declaration
- `verification/model.rs`: public enums and structs
- `verification/checksum.rs`: `transform_checksum`
- `verification/stage.rs`: `verify_stage` and built-in stage check helpers
- `verification/artifact.rs`: `build_verification_artifact` and promotion
  blocker assembly

This keeps public model names stable while separating stage execution from
artifact packaging.

## Public API Impact

Expected impact: none.

If preserving the split requires changing exported verification model names,
serialized field names, check codes, promotion blocker strings, or stage
semantics, stop and re-enter planning.

## Validation

Next code batch validation:

- `cargo test -p underlay-migration-core verification --all-features`
- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
*** Add File: docs/roadmaps/g06/126-migration-core-verification-internal-split.md
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
