# g06.076 - Media Nightfire Tests Internal Split

## Why

`g06.075` found that `underlay-media/src/tests/nightfire_tests.rs` is
test-only but mixes fixture setup, locator extraction, handler registry,
resolver, and usage-sync behavior in one high-error file.

## Goal

Split media Nightfire tests into focused test modules while preserving all
behavioral coverage and production APIs.

## Scope

In scope:

- extract shared block, matcher, handler, and repository fixtures into test
  support
- split field extractor and block-locator fallback tests into a focused module
- split registry-backed traversal and handler-map tests into a focused module
- split locator/path resolver tests into a focused module
- split structured content extraction and sync tests into a focused module
- preserve test assertions and covered behavior
- adjust imports only as needed for module-local test support

Out of scope:

- changing media public APIs
- changing Nightfire extraction semantics
- changing media usage sync behavior
- changing blob/media storage behavior
- changing consumer apps

## Acceptance Criteria

- `nightfire_tests.rs` becomes a small test front door
- behavior groups live in focused test modules
- Nightfire tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior or public APIs must change,
stop and re-enter planning.

## Current State

`g06.076` is next.

## Next Task

Execute `g06.076`: media Nightfire tests internal split.
