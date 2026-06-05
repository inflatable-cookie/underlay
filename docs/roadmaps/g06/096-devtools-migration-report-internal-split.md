# g06.096 - Devtools Migration Report Internal Split

## Why

`g06.095` found that `underlay-devtools/src/migration_report.rs` mixes
decision, drift, recovery, verification, integrity, audit, policy, loading,
error, JSON parsing, and label-helper responsibilities in one public helper
module.

The next split should reduce report-surface reasoning load while preserving
crate-root exports and output formats.

## Goal

Split the devtools migration report helper surface into focused internal
modules while preserving all public helper names and report behavior.

## Scope

In scope:

- keep `migration_report.rs` as the small module front door
- move `MigrationReportError` into a focused error module
- move JSON parsing into a focused helper module
- move decision invalidation/governance formatting and decision file loading
  into a focused decision module
- move drift report construction and formatting into a focused drift module
- move recovery advisory construction and formatting into a focused recovery
  module
- move verification artifact build/write/format behavior into a focused
  verification module
- move integrity artifact build/format behavior and labels into a focused
  integrity module
- move audit artifact build/write/format behavior into a focused audit module
- move governance policy loading/evaluation/formatting into a focused policy
  module
- move pipeline-run report loading and path discovery into a focused pipeline
  module
- preserve existing tests and summary strings

Out of scope:

- changing migration report public APIs
- changing report output semantics
- changing migration execution behavior
- changing consumer apps

## Acceptance Criteria

- `migration_report.rs` becomes a small module front door
- responsibility groups live in focused modules
- crate-root devtools re-exports remain stable
- devtools tests pass with `--all-features`
- `effigy rust:check` passes
- docs checks pass

## Consumer Upgrade Impact

Expected impact: none.

This is an internal split. If public helper names, signatures, output
directories, or summary strings must change, stop and re-enter planning.

## Current State

`g06.096` is ready.

## Next Task

Execute `g06.096`: devtools migration report internal split.
