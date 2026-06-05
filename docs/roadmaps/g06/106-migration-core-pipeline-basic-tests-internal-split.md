# g06.106 - Migration-Core Pipeline Basic Tests Internal Split

## Why

`g06.105` found that `pipeline_basic_tests.rs` mixes stage-order invariants,
full-run assertions, transform failure mapping, resume behavior, resume
compatibility rejection, and verify-stage failure behavior in one test file.

The next split should reduce test reasoning load while preserving pipeline
coverage and failure readability.

## Goal

Split the migration-core pipeline basic tests into focused internal test
modules without changing production code or migration behavior.

## Scope

In scope:

- replace `pipeline_basic_tests.rs` with a `pipeline_basic_tests/` module
  directory
- keep the parent `mod pipeline_basic_tests;` path stable
- keep stage-order and reuse-policy invariants in the module front door
- move full successful run assertions into a focused full-run module
- move transform and verify failure tests into a focused failures module
- move resume success and incompatibility tests into a focused resume module
- add only small local helper constructors where they reduce repeated setup
- preserve existing shared support mocks

Out of scope:

- changing migration-core public APIs
- changing pipeline behavior
- changing stage ordering, resume, verification, or decision semantics
- changing consumer apps

## Acceptance Criteria

- the old oversized test file is replaced by focused test modules
- existing pipeline behavior coverage remains intact
- migration-core tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior must change, stop and
re-enter planning.

## Current State

`g06.106` is ready.

## Next Task

Execute `g06.106`: migration-core pipeline basic tests internal split.
