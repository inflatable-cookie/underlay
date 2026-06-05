# g06.117 - Blob Local Adapter Modularity Audit

## Why

After `g06.116`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-blob/src/adapters/local.rs`.

The local blob adapter is shared storage infrastructure. It should be split
from responsibility and security-boundary evidence, not file size alone.

## Goal

Classify the blob local adapter surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-blob/src/adapters/local.rs` by responsibility family
- identify path resolution, object-key safety, file/stream I/O, metadata
  handling, delete/list behavior, and helper boundaries
- identify public adapter exports and tests that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader blob adapter checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing blob public APIs
- changing object-key validation or path-safety semantics
- changing local storage behavior
- changing consumer apps

## Acceptance Criteria

- local adapter responsibilities are grouped by stable behavior family
- public exports and security-sensitive helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds public exports or
path-safety behavior that must change, stop and re-enter planning.

## Current State

`g06.117` is complete.

Artifact:

- [117 artifact](./117-blob-local-adapter-modularity-audit-artifact.md)

## Next Task

Execute `g06.118`: blob local adapter internal split.
