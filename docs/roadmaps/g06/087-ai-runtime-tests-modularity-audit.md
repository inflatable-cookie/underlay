# g06.087 - AI Runtime Tests Modularity Audit

## Why

After `g06.086`, `underlay-ai-runtime/src/tests/lib_tests.rs` is the largest
remaining Rust warning-level test file in the god-file report.

AI runtime tests cover shared AI request/runtime behavior. They should be split
from behavior evidence rather than file size alone.

## Goal

Classify the AI runtime test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-ai-runtime/src/tests/lib_tests.rs` by behavior family
- identify shared fixtures, provider/runtime setup, request/response behavior,
  and error edge-case groups
- decide whether the next batch should split test modules, extract helper
  fixtures, or defer behind a broader AI runtime checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing AI runtime public APIs
- changing provider/request/response semantics
- changing consumer apps

## Acceptance Criteria

- AI runtime tests are grouped by stable behavior family
- helper/fixture extraction opportunities are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a test-structure audit. Expected impact is none.

## Current State

`g06.087` is complete.

Artifact:

- [087 artifact](./087-ai-runtime-tests-modularity-audit-artifact.md)

## Next Task

Execute `g06.088`: AI runtime tests internal split.
