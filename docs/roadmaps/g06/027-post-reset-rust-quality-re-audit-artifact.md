# g06.027 Artifact - Post-Reset Rust Quality Re-Audit

## Summary

The post-reset Rust architecture is materially cleaner than the pre-reset
state. The main contract/adapter problem is fixed for jobs, media, and
auth-state storage:

- `underlay-jobs` has no SQLx/Postgres dependency
- `underlay-jobs-postgres` owns concrete Postgres job runtime code
- `underlay-media-postgres` owns concrete media Postgres storage
- `underlay-auth-postgres` owns auth-state Postgres storage

No new high-severity security blocker was found in this audit.

The next best batch is to remove the remaining raw table-name API shape from
operator-facing SQL helpers, even though those paths currently validate and
quote identifiers before interpolation.

## Validation And Scan Evidence

Commands run:

- `effigy doctor`
- `effigy tasks`
- `effigy test --plan`
- `effigy rust:check`
- production Rust scan for `unsafe`
- production Rust scan for dynamic SQL construction
- dependency tree check for `underlay-jobs` and `underlay-jobs-postgres`

Results:

- `effigy rust:check` passed
- no production Rust `unsafe` usage found
- `underlay-jobs` dependency tree contains no SQLx/Postgres backend dependency
- `underlay-jobs-postgres` contains the expected SQLx/Postgres/cron backend
  dependency stack
- `effigy doctor` still reports known structural scanner backlog:
  - `scan.attention-markers`: 11 findings
  - `scan.comment-ratio`: 11 findings
  - `scan.god-files`: 61 findings

## Findings

### Medium: raw table-name APIs remain in operator SQL helpers

Files:

- `rust/crates/underlay-audit/src/query.rs`
- `rust/crates/underlay-security-alerts/src/store.rs`

Current behavior:

- callers pass table names as `&str`
- the modules validate and quote those names before interpolation
- query values still use bind parameters

Assessment:

- no immediate injection blocker found
- still weaker than the reference-grade target because safe identifier handling
  depends on every call path remembering to validate and format the table name
- the API should move to typed table config or `QualifiedTableName` inputs so
  invalid table names cannot cross the public boundary

### Medium: remaining Rust god-file findings are concentrated but not blocking

Rust production files still above scanner thresholds include:

- `rust/crates/underlay-migration-core/src/pipeline.rs`
- `rust/crates/underlay-jobs/src/types.rs`
- `rust/crates/underlay-media/src/domain.rs`
- `rust/crates/underlay-devtools/src/migration_bundle.rs`
- `rust/crates/underlay-migration-core/src/verification_rules.rs`
- `rust/crates/underlay-jobs-postgres/src/postgres.rs`

Assessment:

- the largest remaining issue is readability and future change risk
- `underlay-jobs-postgres/src/postgres.rs` is newly isolated and therefore less
  harmful than when it lived inside the contract crate
- splitting should be evidence-led and not reopen broad churn

### Low: scanner attention marker misclassifies an enum grouping comment

File:

- `rust/crates/underlay-auth/src/types.rs`

Assessment:

- the flagged `// Security` marker is an enum grouping label, not deferred
  security work
- this is scanner noise unless the scanner policy is refined

### Low: production `expect` calls are mostly invariant assertions

Examples:

- static regex construction
- poisoned mutex assertions in in-memory helpers
- HMAC construction with an API guarantee
- test utility request/response helpers

Assessment:

- no panic path was found that looks like direct untrusted-input escalation in
  app-facing runtime code
- future audit can narrow this further, but it is not the next best batch

## Decision

Open `g06.028`: typed operator table config for audit and security-alert
helpers.

Do not start another broad god-file split by default. The next change should
target the remaining safety-boundary weakness first.
