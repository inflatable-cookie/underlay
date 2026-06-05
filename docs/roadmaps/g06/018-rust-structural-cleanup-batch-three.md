# g06.018 - Rust Structural Cleanup Batch Three

## Why

The fresh audit after `g06.016` shows the remaining Rust risk is mainly
structural. The repo is safer and better typed, but Effigy doctor still reports
high-severity god-files in runtime-adjacent Rust modules and large test files.

This card keeps that work bounded.

## Goal

Reduce the highest-value Rust structural findings without changing public
consumer contracts.

## Scope

In scope:

- split one or two high-severity Rust god-files by stable responsibility
- prefer `underlay-migration-core`, `underlay-jobs`, or `underlay-media` targets
  before test-only files
- keep public APIs stable unless an additive internal module split is needed
- preserve current supply-chain and compatibility posture

Out of scope:

- broad workspace-wide cleanup
- TypeScript doctor findings
- release execution or publishing
- consumer repo updates unless a concrete break is discovered

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- at least one high-severity Rust structural finding is removed or materially
  reduced
- touched crate checks pass
- docs QA passes if queue docs are updated
- no named consumer app update is required

## Current State

`g06.018` is superseded by `g06.019`.

The user explicitly allowed controlled breaking changes and consumer updates to
move Underlay toward a reference-grade architecture. That is a larger
sequencing reset than this local structural cleanup card.

## Next Task

Execute `g06.019`: Reference-grade architecture reset inventory.
