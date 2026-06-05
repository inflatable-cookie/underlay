# g06.091 - Config Crate Modularity Audit

## Why

After `g06.090`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-config/src/lib.rs`.

Config is a foundational shared crate. It likely carries public types,
environment loading, validation, and app-facing helpers in one file. It should
be split only after confirming the public surface and extension boundaries.

## Goal

Classify the config crate surface and decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-config/src/lib.rs` by responsibility family
- identify public config models, parsing/loading helpers, validation behavior,
  and environment boundaries
- identify which exports must remain stable for consuming apps
- decide whether the next batch should split internal modules, extract model
  files, or defer behind a broader config contract checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing config public APIs
- changing environment variable semantics
- changing validation behavior
- changing consumer apps

## Acceptance Criteria

- config crate responsibilities are grouped by stable behavior family
- public exports and consumer-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.091` is complete.

Artifact:

- [091 artifact](./091-config-crate-modularity-audit-artifact.md)

## Next Task

Execute `g06.092`: config crate internal split.
