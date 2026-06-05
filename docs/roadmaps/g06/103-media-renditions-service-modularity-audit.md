# g06.103 - Media Renditions Service Modularity Audit

## Why

After `g06.102`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-media/src/renditions/service.rs`.

Media rendition generation is shared processing code. It should be split from
responsibility and public-surface evidence, not file size alone.

## Goal

Classify the media renditions service surface and decide the safest next
structural batch.

## Scope

In scope:

- inspect `underlay-media/src/renditions/service.rs` by responsibility family
- identify public service methods, rendition planning, repository/storage
  behavior, processor calls, and helper boundaries
- identify which exports and service method contracts must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media rendition checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media rendition public APIs
- changing storage or repository semantics
- changing generated rendition behavior
- changing consumer apps

## Acceptance Criteria

- media rendition service responsibilities are grouped by stable behavior
  family
- public exports and service-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.103` is complete.

Artifact:

- [103 artifact](./103-media-renditions-service-modularity-audit-artifact.md)

## Next Task

Execute `g06.104`: media renditions service internal split.
