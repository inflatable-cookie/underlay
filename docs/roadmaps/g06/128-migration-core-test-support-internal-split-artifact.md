# g06.128 Artifact - Migration-Core Test Support Internal Split

## Summary

Migration-core test support now lives under a focused `tests/support/` module
directory instead of one large `support.rs` file.

Changed files:

- `underlay-migration-core/src/tests/support.rs` removed
- `underlay-migration-core/src/tests/support/mod.rs`
- `underlay-migration-core/src/tests/support/fingerprint.rs`
- `underlay-migration-core/src/tests/support/mocks.rs`
- `underlay-migration-core/src/tests/support/store.rs`

## Module Shape

- `mod.rs`: support front door and re-exports
- `fingerprint.rs`: deterministic decision fingerprint helper
- `mocks.rs`: mock source, plugin, decision resolver, and asset resolver
- `store.rs`: in-memory run store and `RunStore` implementation

The current `super::support::{...}` imports remain stable for migration-core
tests.

## Behavior Preserved

The split keeps existing test support behavior:

- source extract call counting
- plugin normalize call counting
- transform failure injection
- semantic verification failure injection
- decision invalidation injection
- deterministic decision fingerprinting
- resolver call counting
- in-memory checkpoint ordering
- latest resume checkpoint lookup
- decision journal append/latest/history lookup
- unresolved decision recording
- stage snapshot write/read behavior

## Validation

Passed:

- `cargo test -p underlay-migration-core --all-features`
  - 43 unit tests passed
  - 0 doc-tests
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 25 to 24 after this split.
- The next Rust warning-level target is `underlay-http/src/context.rs`.

## Public API Impact

None.

This was a test-only split. No production migration behavior, public API, or
consumer import path changed.
*** Add File: docs/roadmaps/g06/129-http-context-modularity-audit.md
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
