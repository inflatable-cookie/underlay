# g06.069 - Migration-Core Verification-Rules Public Model Modularity Audit

## Why

After `g06.068`, the next Rust production high-error god-file is
`underlay-migration-core/src/verification_rules.rs`.

Verification rules are migration-core public model surface. They need an audit
before any split changes module shape.

## Goal

Classify the verification-rules public and internal model surface and decide
the safest next structural batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/verification_rules.rs` by type and
  helper family
- classify exported rules, rule sets, validation results, and helper behavior
- scan internal and consumer usage for direct imports
- decide whether the next batch should split verification-rule internals,
  target a different migration-core production file, or defer behind a broader
  migration-core checkpoint
- update the Rust public API inventory if the verification-rules boundary needs
  tighter wording

Out of scope:

- changing verification rule semantics
- changing serialized verification-rule or report shapes
- changing migration pipeline behavior
- changing consumer app migration behavior

## Acceptance Criteria

- verification-rules surface is grouped by stable contract family
- import paths and internal call sites are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is an audit and planning checkpoint. Any breaking migration-core contract
change must be promoted into an explicit follow-up card before execution.

## Current State

`g06.069` is complete.

Artifact:

- [069 artifact](./069-migration-core-verification-rules-public-model-modularity-audit-artifact.md)

## Next Task

Execute `g06.070`: migration-core verification-rules internal split.
