# g06.070 - Migration-Core Verification-Rules Internal Split

## Why

`g06.069` found that `underlay-migration-core/src/verification_rules.rs`
exposes a stable crate-root rule model but keeps evaluator internals, standard
rule constructors, and benchmarking in one high-error file.

## Goal

Split verification-rule internals into focused private modules while preserving
the public rule/result model, crate-root exports, serialized shapes, evaluator
behavior, and benchmark behavior.

## Scope

In scope:

- split public rule/result model into a focused module if exports stay intact
- split standard-rule constructors into a focused module
- split evaluator helpers for row-count, not-null, unique, and
  referential-integrity rules
- split benchmark helper if it keeps the same public signature
- preserve all crate-root exports
- update tests only where private module paths need explicit imports

Out of scope:

- changing rule semantics
- changing rule/result public fields
- changing serde names or enum variants
- changing `PipelinePolicy.verification_rules` or `VerificationInput.rules`
- changing plugin verification behavior
- changing migration pipeline verify-stage behavior
- consumer rollout unless public imports move

## Acceptance Criteria

- `verification_rules.rs` becomes a smaller public model/front door or thin
  coordinator
- root exports remain source-compatible
- verification tests pass
- full migration-core tests pass
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public exports, fields, enum variants, serde
shapes, evaluator behavior, or benchmark behavior must change, stop and
re-enter planning.

## Current State

`g06.070` is complete.

Artifact:

- [070 artifact](./070-migration-core-verification-rules-internal-split-artifact.md)

## Next Task

Execute `g06.071`: jobs-postgres repository public model modularity audit.
