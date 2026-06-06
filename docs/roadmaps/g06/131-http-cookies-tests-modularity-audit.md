# g06.131 - HTTP Cookies Tests Modularity Audit

## Why

After `g06.130`, the next Rust warning-level file in the god-file report is
`underlay-http/src/tests/cookies_tests.rs`.

Cookie tests cover security-sensitive auth and CSRF cookie behavior. They
should be split from evidence about test fixture families, not from file size
alone.

## Goal

Classify the HTTP cookies test surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-http/src/tests/cookies_tests.rs` by behavior family
- identify auth cookie, CSRF cookie, extraction, validation, builder, and
  clearing test boundaries
- identify security-sensitive test behavior that must remain covered
- decide whether the next batch should split internal test modules, extract
  helper files, or defer behind a broader cookie test checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing cookie public APIs
- changing auth or CSRF cookie behavior
- changing cookie defaults
- changing consumer apps

## Acceptance Criteria

- cookie test responsibilities are grouped by stable behavior family
- security-sensitive behavior coverage is recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only structure audit. If preserving tests requires changing
cookie behavior, stop and re-enter planning.

## Current State

`g06.131` is ready.

## Next Task

Execute `g06.131`: HTTP cookies tests modularity audit.
