# g06.129 - HTTP Context Modularity Audit

## Why

After `g06.128`, the next Rust production warning-level file in the god-file
report is `underlay-http/src/context.rs`.

HTTP request context code is a shared runtime boundary. It should be split from
evidence about public API, extraction behavior, extension storage, and typed
context helpers, not from file size alone.

## Goal

Classify the HTTP context surface and decide the safest next structural batch.

## Scope

In scope:

- inspect `underlay-http/src/context.rs` by responsibility family
- identify public types, request extension behavior, extractor behavior, helper
  methods, and test boundaries
- identify consumer-visible behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader HTTP context checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing HTTP public APIs
- changing request context semantics
- changing auth/session behavior
- changing consumer apps

## Acceptance Criteria

- HTTP context responsibilities are grouped by stable behavior family
- public API and runtime behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds context behavior
that must change, stop and re-enter planning.

## Current State

`g06.129` is ready.

## Next Task

Execute `g06.129`: HTTP context modularity audit.
