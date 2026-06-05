# g06.052 Artifact - Rust Structural Backlog Checkpoint

## Summary

Effigy doctor still reports the same structural backlog families:

- attention markers
- comment ratio
- god-files

`g06.049` and `g06.051` reduced meaningful runtime/tooling structure, but the
remaining highest-value Rust pressure is now in media and HTTP/jobs surfaces,
not migration-core front-door shape.

## Current Evidence

Doctor:

- `scan.attention-markers`: 11 findings, 2 errors
- `scan.comment-ratio`: 11 findings, 3 errors
- `scan.god-files`: 61 findings, 18 errors

Largest current Rust files:

- `underlay-auth-jwt/src/tests/service_tests.rs`: 747 lines
- `underlay-media/src/domain.rs`: 706 lines
- `underlay-jobs/src/types.rs`: 659 lines
- `underlay-media/src/renditions.rs`: 633 lines
- `underlay-http/src/query.rs`: 593 lines
- `underlay-http/src/cookies.rs`: 569 lines
- `underlay-jobs-postgres/src/postgres.rs`: 565 lines
- `underlay-devtools/src/migration_bundle.rs`: 534 lines
- `underlay-http/src/error_logging.rs`: 532 lines
- `underlay-migration-core/src/verification_rules.rs`: 492 lines
- `underlay-migration-core/src/pipeline/orchestrator.rs`: 487 lines

## Known Versus New

Known backlog:

- test-only god files remain large
- TS god files remain outside the current Rust lane
- comment-ratio findings include low-risk doc-heavy helper files
- attention-marker findings include stale or over-broad scanner matches

New after `g06.051`:

- `underlay-migration-core/src/pipeline/orchestrator.rs` appears as a new
  high-severity god-file after the pipeline split. This is expected residual
  pressure: `pipeline.rs` is now a small front door, but the stage-flow
  orchestrator still holds the full pipeline flow.

No new runtime security issue was found in this checkpoint.

## Ranking

Highest-value next code target:

- `underlay-media/src/domain.rs`

Why:

- production/shared public model surface, not only tests
- high-severity god-file finding
- strongly tied to current typed object-key and media adapter work
- clean internal split shape: identifiers, enums, entities, usage/migration
  bindings, and input/list params
- public root exports can remain unchanged

Defer for now:

- `underlay-migration-core/src/pipeline/orchestrator.rs`: still large, but the
  first pipeline split just landed and tests prove the root shape. Splitting
  stage execution helpers can wait for a later targeted batch.
- `underlay-media/src/renditions.rs`: security-relevant but recently tightened;
  defer until the domain model is easier to scan.
- `underlay-jobs/src/types.rs`: broad public model, likely needs its own audit
  before splitting.
- `underlay-http/src/query.rs`, `cookies.rs`, `error_logging.rs`: good
  candidates, but media is currently more central to the active Rust reset.
- test-only god files: lower shared-library impact.

## Decision

Queue `g06.053` as a media domain internal split.

The split should preserve:

- `underlay_media` root exports
- `underlay_media::domain::*` import compatibility where possible
- serialized shapes for media entities and usage edge types
- existing repository and Postgres adapter behavior

## Validation

- `effigy tasks`
- `effigy doctor` - expected structural backlog failure
- `effigy qa:docs`
- `effigy qa:northstar`

Next code batch validation:

- `cargo test -p underlay-media --all-features`
- `cargo test -p underlay-media-postgres --all-features`
- `effigy rust:check`
- consumer checks only if public imports move
- `effigy qa:docs`
- `effigy qa:northstar`
