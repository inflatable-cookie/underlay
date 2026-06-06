# g06.123 - Media Nightfire Walk Modularity Audit

## Why

After `g06.122`, the next Rust production warning-level file in the god-file
report is `underlay-media/src/nightfire/walk.rs`.

Nightfire walking code is traversal logic over media content. It should be
split from evidence about traversal responsibilities, mutation boundaries, and
error behavior, not from file size alone.

## Goal

Classify the media Nightfire walk surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-media/src/nightfire/walk.rs` by responsibility family
- identify traversal, collection, mutation, error, and helper boundaries
- identify any public or crate-visible behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader media/Nightfire checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing media public APIs
- changing Nightfire content behavior
- changing storage or rendition behavior
- changing consumer apps

## Acceptance Criteria

- walk responsibilities are grouped by stable behavior family
- public or crate-visible behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds traversal or
content behavior that must change, stop and re-enter planning.

## Current State

`g06.123` is ready.

## Next Task

Execute `g06.123`: media Nightfire walk modularity audit.
