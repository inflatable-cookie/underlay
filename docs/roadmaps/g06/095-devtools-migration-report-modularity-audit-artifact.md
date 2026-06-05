# g06.095 Artifact - Devtools Migration Report Modularity Audit

## Summary

`underlay-devtools/src/migration_report.rs` is a public helper surface behind
crate-root re-exports. It combines decision invalidation/governance summaries,
pipeline report loading, drift/recovery report construction and formatting,
decision index/journal loading, verification/integrity/audit artifact build and
write helpers, governance policy loading/reporting, report errors, JSON parsing,
and enum label helpers.

The module is devtools/reporting code, not migration execution logic. The
caller-visible function names are exported from `underlay_devtools`, so the
split must preserve that public surface.

The current surface groups:

- decision invalidation formatting
- decision governance formatting and top-issue selection
- decide-stage and pipeline-run report loading
- drift report construction and formatting
- decision index and journal loading
- recovery advisory construction and formatting
- verification artifact construction, writing, and summary formatting
- integrity artifact construction and summary formatting
- audit artifact construction, writing, and summary formatting
- pipeline-run report discovery from a file or directory
- governance policy loading, evaluation, and summary formatting
- `MigrationReportError`
- JSON parsing and enum label helpers

## Behavior Evidence

The full devtools crate test set covers these stable contracts:

- decision invalidations summarize by reason
- governance report formatting counts repeated artifact/code pairs
- top governance issues are selected by caller-provided limit
- decide-stage loading supports both direct decide JSON and full pipeline-run
  JSON
- pipeline reports load from file and from a directory
- drift and recovery summaries return no-issue/no-action strings for clean runs
- verification, integrity, and audit artifacts build, summarize, and write
  output files
- governance policy reports load and format compliant policy summaries
- unrelated migration bundle and sync migration tests still pass

Validation result:

- `cargo test -p underlay-devtools --all-features`
- 23 passed, 1 ignored Docker/registry test

## Decision

Queue `g06.096` as a devtools migration report internal split.

The split should preserve:

- all crate-root `migration_report` re-exports in `underlay-devtools/src/lib.rs`
- all public helper function names and signatures
- `MigrationReportError`
- current artifact output directory names
- current summary string formats
- current report loading fallback behavior
- current devtools tests

Suggested module shape:

- `migration_report.rs`: module front door and re-exports
- `migration_report/error.rs`: `MigrationReportError`
- `migration_report/json.rs`: JSON parsing helper
- `migration_report/decision.rs`: invalidation/governance formatting, top
  issues, decide/index/journal loading
- `migration_report/drift.rs`: drift report construction and formatting
- `migration_report/recovery.rs`: recovery advisory construction and formatting
- `migration_report/verification.rs`: verification artifact build/write/format
- `migration_report/integrity.rs`: integrity artifact build/format and labels
- `migration_report/audit.rs`: audit artifact build/write/format
- `migration_report/policy.rs`: governance policy load/build/format
- `migration_report/pipeline.rs`: pipeline-run report loading and path
  discovery

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root re-exports or report
format strings forces a semantic change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-devtools --all-features`

Next code batch validation:

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
