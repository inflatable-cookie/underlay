# g06.121 - Validation Derive Crate Modularity Audit

## Why

After `g06.120`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-validation-derive/src/lib.rs`.

Derive macros are public compile-time API. They should be split from parsing
and code-generation evidence, not file size alone.

## Goal

Classify the validation derive crate surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-validation-derive/src/lib.rs` by responsibility family
- identify macro entry points, input parsing, attribute handling, validation
  rule mapping, generated code, error reporting, and test boundaries
- identify public macro behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader validation macro checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing validation public APIs
- changing derive macro syntax or generated behavior
- changing downstream consumer code
- changing consumer apps

## Acceptance Criteria

- derive crate responsibilities are grouped by stable behavior family
- macro entry points and generated behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds macro syntax or
generated behavior that must change, stop and re-enter planning.

## Current State

`g06.121` is complete.

Artifact:

- [121 artifact](./121-validation-derive-crate-modularity-audit-artifact.md)

## Next Task

Execute `g06.122`: validation derive crate internal split.
