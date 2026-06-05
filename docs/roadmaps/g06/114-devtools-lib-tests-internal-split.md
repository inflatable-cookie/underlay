# g06.114 - Devtools Lib Tests Internal Split

## Why

`g06.113` found that `underlay-devtools/src/tests/lib_tests.rs` mixes
environment helper tests, decision report formatting, pipeline/report loading,
artifact writing, governance policy loading, and shared fixture setup in one
test file.

The next split should reduce test reasoning load while preserving coverage and
failure readability.

## Goal

Split the devtools lib tests into focused internal test modules without
changing production code or devtools behavior.

## Scope

In scope:

- replace `tests/lib_tests.rs` with a `tests/lib_tests/` module directory
- update the `src/lib.rs` test path to the new module front door
- move temp-dir fixture setup into a focused support module
- move environment helper tests into a focused env module
- move decision invalidation and governance formatting tests into a focused
  decision reports module
- move pipeline loading, drift, recovery, verification, integrity, and audit
  artifact tests into a focused pipeline reports module
- move governance policy loading and formatting tests into a focused policy
  reports module
- preserve existing devtools tests

Out of scope:

- changing devtools public APIs
- changing report formatting behavior
- changing report loading or artifact writing behavior
- changing consumer apps

## Acceptance Criteria

- the old oversized test file is replaced by focused test modules
- existing devtools behavior coverage remains intact
- devtools tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is a test-only split. If production behavior must change, stop and
re-enter planning.

## Current State

`g06.114` is ready.

## Next Task

Execute `g06.114`: devtools lib tests internal split.
