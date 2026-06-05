# g06.107 - Media Nightfire Modularity Audit

## Why

After `g06.106`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-media/src/nightfire.rs`.

Nightfire is shared media usage extraction and sync code. It should be split
from responsibility and public-surface evidence, not file size alone.

## Goal

Classify the media Nightfire surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-media/src/nightfire.rs` by responsibility family
- identify public exports, extractor behavior, registry behavior, resolver
  behavior, sync integration, and helper boundaries
- identify which types, functions, and tests must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media usage checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing Nightfire extraction or sync semantics
- changing media repository behavior
- changing consumer apps

## Acceptance Criteria

- Nightfire responsibilities are grouped by stable behavior family
- public exports and service-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds public exports
that must move or behavior that must change, stop and re-enter planning.

## Current State

`g06.107` is complete.

Artifact:

- [107 artifact](./107-media-nightfire-modularity-audit-artifact.md)

## Next Task

Execute `g06.108`: media Nightfire internal split.
