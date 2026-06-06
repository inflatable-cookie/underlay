# g06.120 - Jobs Runner Tests Internal Split

## Why

`g06.119` found that `underlay-jobs/src/tests/runner_tests.rs` mixes runner
fixtures, event capture, dispatch/no-work behavior, failure handling, config
propagation, permanent failure behavior, runner config defaults, and batch
limiting in one test file.

The next split should reduce test reasoning load while preserving coverage and
failure readability.

## Goal

Split the jobs runner tests into focused internal test modules without changing
production code or runner behavior.

## Scope

In scope:

- replace `tests/runner_tests.rs` with a `tests/runner_tests/` module directory
- update `src/runner.rs` test path to the new module front door
- keep fixtures and shared handlers in the module front door
- move dispatch, no-work, and unknown-type tests into a focused dispatch module
- move failure recording, handler config, and permanent failure tests into a
  focused failures module
- move runner config defaults and batch limiting tests into a focused batch
  module
- preserve existing jobs tests

Out of scope:

- changing jobs public APIs
- changing runner, retry, timeout, or cancellation behavior
- changing repository semantics
- changing consumer apps

## Acceptance Criteria

- the old oversized test file is replaced by focused test modules
- existing jobs runner behavior coverage remains intact
- jobs tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior must change, stop and
re-enter planning.

## Current State

`g06.120` is complete.

Artifact:

- [120 artifact](./120-jobs-runner-tests-internal-split-artifact.md)

## Next Task

Execute `g06.121`: validation derive crate modularity audit.
