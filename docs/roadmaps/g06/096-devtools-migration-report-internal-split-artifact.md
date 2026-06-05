# g06.096 Artifact - Devtools Migration Report Internal Split

## Summary

`underlay-devtools/src/migration_report.rs` is now a small module front door
with stable public re-exports. The former mixed report helper module was split
into focused modules under `src/migration_report/`.

New module shape:

- `migration_report.rs`: front door and public re-exports
- `migration_report/error.rs`: `MigrationReportError`
- `migration_report/json.rs`: JSON parsing helper
- `migration_report/decision.rs`: decision invalidation/governance formatting,
  top governance issues, decide-stage loading, decision index loading, and
  decision journal loading
- `migration_report/drift.rs`: drift report construction and formatting
- `migration_report/recovery.rs`: recovery advisory construction and formatting
- `migration_report/verification.rs`: verification artifact build/write/format
- `migration_report/integrity.rs`: integrity artifact build/format and labels
- `migration_report/audit.rs`: audit artifact build/write/format
- `migration_report/pipeline.rs`: pipeline-run report loading and path
  discovery
- `migration_report/policy.rs`: governance policy load/build/format

## Public API Impact

None expected.

The crate-root `underlay_devtools` re-exports, public helper function names,
`MigrationReportError`, artifact output directories, and summary strings were
preserved.

## Validation

- `cargo test -p underlay-devtools --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`

`cargo test -p underlay-devtools --all-features` passed with 23 tests passed
and 1 Docker/registry test ignored.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 40 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

The devtools migration report file no longer appears in the god-file report.
The next largest Rust production warning is
`rust/crates/underlay-ai-runtime/src/lib.rs`.

## Next Target Evidence

Queue `g06.097` as an AI runtime crate modularity audit before splitting
`underlay-ai-runtime/src/lib.rs`. AI runtime is a shared provider and routing
surface, so the next batch should classify public models, provider registry,
clients, retry/circuit behavior, and route-chain APIs before moving code.
