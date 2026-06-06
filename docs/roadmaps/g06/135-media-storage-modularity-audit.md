# g06.135 - Media Storage Modularity Audit

## Why

After `g06.134`, the next Rust production warning-level file in the god-file
report is `underlay-media/src/storage.rs`.

Media storage key generation is a shared storage boundary. It should be split
from evidence about public API, typed object-key behavior, path safety, and
test coverage, not from file size alone.

## Goal

Classify the media storage surface and decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-media/src/storage.rs` by responsibility family
- identify public types, key generation, object-key helpers, path validation,
  MIME mapping, and test boundaries
- identify public API or storage behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media storage checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing stored object key formats
- changing blob adapter behavior
- changing consumer apps

## Acceptance Criteria

- storage responsibilities are grouped by stable behavior family
- public API and object-key behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds storage key
behavior that must change, stop and re-enter planning.

## Current State

`g06.135` is complete.

Artifact:

- [135 artifact](./135-media-storage-modularity-audit-artifact.md)

## Next Task

Execute `g06.136`: media storage internal split.
