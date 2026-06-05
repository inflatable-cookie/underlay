# g06.097 - AI Runtime Crate Modularity Audit

## Why

After `g06.096`, the largest remaining Rust warning-level production file in
the god-file report is `underlay-ai-runtime/src/lib.rs`.

AI runtime is a shared provider, routing, client, retry, and circuit-breaker
surface. It should be split from public API and responsibility evidence, not
file size alone.

## Goal

Classify the AI runtime crate surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-ai-runtime/src/lib.rs` by responsibility family
- identify public request/response models, route selection, provider registry,
  client behavior, retry behavior, circuit-breaker behavior, and route-chain
  APIs
- identify which crate-root exports and helper methods must remain stable
- decide whether the next batch should split internal modules, extract model
  files, or defer behind a broader AI runtime contract checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing AI runtime public APIs
- changing provider routing behavior
- changing retry or circuit-breaker semantics
- changing consumer apps

## Acceptance Criteria

- AI runtime responsibilities are grouped by stable behavior family
- public exports and caller-visible helpers are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

This is a production-code structure audit. Expected impact is none unless the
audit finds public exports that must move; if so, stop and re-enter planning.

## Current State

`g06.097` is ready.

## Next Task

Execute `g06.097`: AI runtime crate modularity audit.
