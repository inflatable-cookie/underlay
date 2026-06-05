# g06.116 - Migration-Core Pipeline Decision Tests Internal Split

## Why

`g06.115` found that `pipeline_decision_tests.rs` mixes cached-decision reuse,
human override precedence, plugin invalidation, invalid cached-record
governance failure, low-confidence unresolved queue behavior, and fixture setup
in one test file.

The next split should reduce test reasoning load while preserving coverage and
failure readability.

## Goal

Split the migration-core pipeline decision tests into focused internal test
modules without changing production code or decision behavior.

## Scope

In scope:

- replace `pipeline_decision_tests.rs` with a `pipeline_decision_tests/`
  module directory
- keep the parent `mod pipeline_decision_tests;` path stable
- move shared setup and seeded-decision helpers into the module front door
- move cached reuse and human override tests into a focused reuse module
- move plugin dependency invalidation tests into a focused invalidation module
- move invalid cached-record governance failure tests into a focused governance
  module
- move low-confidence unresolved queue tests into a focused unresolved module
- preserve existing migration-core tests

Out of scope:

- changing migration-core public APIs
- changing decision reuse, invalidation, governance, or unresolved queue
  behavior
- changing pipeline execution semantics
- changing consumer apps

## Acceptance Criteria

- the old oversized test file is replaced by focused test modules
- existing decision behavior coverage remains intact
- migration-core tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior must change, stop and
re-enter planning.

## Current State

`g06.116` is ready.

## Next Task

Execute `g06.116`: migration-core pipeline decision tests internal split.
