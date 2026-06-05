# g06.109 - Devtools Seed Bundle Modularity Audit

## Why

After `g06.108`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-devtools/src/seed_bundle.rs`.

Seed bundles are shared development and bootstrap tooling. They should be split
from responsibility and public-surface evidence, not file size alone.

## Goal

Classify the devtools seed-bundle surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-devtools/src/seed_bundle.rs` by responsibility family
- identify public models, filesystem I/O, validation, serialization,
  path-safety behavior, and helper boundaries
- identify tests and docs that depend on the current surface
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader devtools bundle checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing devtools public APIs
- changing seed-bundle file formats
- changing path-safety or validation semantics
- changing consumer apps

## Acceptance Criteria

- seed-bundle responsibilities are grouped by stable behavior family
- public exports and helper boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds public exports,
formats, or behavior that must change, stop and re-enter planning.

## Current State

`g06.109` is ready.

## Next Task

Execute `g06.109`: devtools seed bundle modularity audit.
