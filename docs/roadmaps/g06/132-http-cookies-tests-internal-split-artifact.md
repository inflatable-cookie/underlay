# g06.132 Artifact - HTTP Cookies Tests Internal Split

## Summary

HTTP cookie tests now live under a focused `tests/cookies_tests/` module
directory instead of one flat `cookies_tests.rs` file.

Changed files:

- `underlay-http/src/cookies.rs`
- `underlay-http/src/tests/cookies_tests.rs` removed
- `underlay-http/src/tests/cookies_tests/mod.rs`
- `underlay-http/src/tests/cookies_tests/builders.rs`
- `underlay-http/src/tests/cookies_tests/extractors.rs`
- `underlay-http/src/tests/cookies_tests/config.rs`
- `underlay-http/src/tests/cookies_tests/headers.rs`

## Module Shape

- `mod.rs`: shared imports and child module declarations
- `builders.rs`: refresh, logged-in, CSRF, SameSite, path, and value
  validation tests
- `extractors.rs`: refresh and CSRF extraction tests
- `config.rs`: prefix, builder, typed value, domain/path, and try-builder
  validation tests
- `headers.rs`: set/clear header append tests

## Behavior Preserved

The split keeps existing security-sensitive coverage:

- refresh token cookie flags and path
- local-dev Secure omission
- logged-in and CSRF cookie HttpOnly behavior
- CSRF cookie default Secure and SameSite behavior
- refresh and CSRF extraction with default and prefixed names
- SameSite=None validation
- invalid prefix, path, domain, domain label, value, and empty value rejection
- typed cookie wrapper validation
- try-builder early validation
- CSRF set/clear `Set-Cookie` append behavior

## Validation

Passed:

- `cargo test -p underlay-http cookies --all-features`
  - 25 tests passed
- `cargo test -p underlay-http --all-features`
  - 121 unit tests passed
  - 1 Docker-backed error-logging test ignored
  - 11 doc-tests passed
  - 13 doc-tests ignored
- `effigy rust:check`

Known backlog:

- `effigy doctor` still fails on the existing structural scan backlog:
  attention markers, comment ratio, and god-files.
- God-file findings dropped from 23 to 22 after this split.
- The next Rust production warning-level target is
  `underlay-migration-core/src/drift.rs`.

## Public API Impact

None.

This was a test-only split. No cookie API, cookie behavior, default, or
consumer import path changed.
*** Add File: docs/roadmaps/g06/133-migration-core-drift-modularity-audit.md
# g06.133 - Migration-Core Drift Modularity Audit

## Why

After `g06.132`, the next Rust production warning-level file in the god-file
report is `underlay-migration-core/src/drift.rs`.

Migration drift detection is part of the promotion safety boundary. It should
be split from evidence about thresholds, issue detection, lineage checks, and
public model impact, not from file size alone.

## Goal

Classify the migration-core drift surface and decide the safest next structural
batch.

## Scope

In scope:

- inspect `underlay-migration-core/src/drift.rs` by responsibility family
- identify public models, threshold handling, run-report checks, lineage checks,
  and helper boundaries
- identify public API or migration behavior that must remain stable
- decide whether the next batch should split internal modules, extract helper
  files, or defer behind a broader migration drift checkpoint
- update roadmap evidence with the selected split shape

Out of scope:

- changing migration public APIs
- changing drift detection semantics
- changing verification or integrity behavior
- changing consumer apps

## Acceptance Criteria

- drift responsibilities are grouped by stable behavior family
- public API and migration behavior boundaries are recorded
- public API impact for a follow-up split is classified
- next card is queued from evidence rather than file size alone

## Consumer Upgrade Impact

Expected impact: none.

This is a production-code structure audit. If the audit finds drift behavior
that must change, stop and re-enter planning.

## Current State

`g06.133` is ready.

## Next Task

Execute `g06.133`: migration-core drift modularity audit.
