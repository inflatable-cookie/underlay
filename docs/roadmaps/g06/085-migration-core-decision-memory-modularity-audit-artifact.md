# g06.085 Artifact - Migration-Core Decision-Memory Modularity Audit

## Summary

`underlay-migration-core/src/decision_memory.rs` is the largest remaining Rust
warning-level god-file after the Rust high-error backlog was cleared. Unlike
the preceding test files, this is production migration infrastructure and
exports public decision-memory helpers through the crate root.

The file currently groups:

- public decision-memory models and index structs
- public invalidation/reuse/provenance types
- decision and record fingerprinting
- decision journal NDJSON parsing
- decision index build, merge, parse, and validation
- unresolved decision validation
- decision reuse evaluation
- effective decision and provenance chain selection
- internal canonical JSON, digest validation, version compatibility, and
  provenance-rank helpers

## Behavior Evidence

The focused test suite covers these stable contracts:

- decision fingerprints are deterministic for semantically equivalent JSON
- record fingerprints change when semantic dependencies change
- decision journal NDJSON parses and validates records
- decision indexes build, merge, prefer newer entries, and parse back from JSON
- strict reuse requires exact resolver and prompt versions
- compatible reuse allows same semver-major versions and blocks plugin
  invalidation
- effective decision selection prefers human override provenance
- provenance chains sort oldest first
- unresolved decision validation rejects out-of-range thresholds

## Public Surface

The crate root re-exports these decision-memory items and must keep their paths
stable:

- `build_decision_index`
- `decision_fingerprint`
- `effective_decision_for_fingerprint`
- `evaluate_decision_reuse`
- `merge_decision_indexes`
- `parse_decision_index`
- `parse_decision_journal_ndjson`
- `provenance_chain_for_fingerprint`
- `record_fingerprint`
- `validate_decision_index`
- `validate_decision_journal_record`
- `validate_unresolved_decision_record`
- `DecisionIndex`
- `DecisionIndexEntry`
- `DecisionInvalidationReason`
- `DecisionProvenanceEvent`
- `DecisionReuseEvaluation`
- `RecordFingerprintInput`

Internal callers also use decision-memory functions from drift detection and
the migration orchestrator decide stage.

## Decision

Queue `g06.086` as a migration-core decision-memory internal split.

The split should preserve:

- all crate-root re-exports
- all function names and signatures
- serialization shapes for public structs/enums
- decision fingerprint hash inputs and ordering
- index merge/validation semantics
- reuse-policy evaluation semantics
- provenance rank and chain ordering semantics

Suggested module shape:

- `decision_memory.rs`: public front door and `pub use` surface
- `decision_memory/models.rs`: public model structs/enums
- `decision_memory/fingerprint.rs`: fingerprinting and canonical JSON helpers
- `decision_memory/index.rs`: index build, merge, parse, and validation
- `decision_memory/validation.rs`: journal/unresolved/digest validation
- `decision_memory/reuse.rs`: reuse evaluation and version compatibility
- `decision_memory/provenance.rs`: effective decision and provenance chain
  helpers

## Public API Impact

Expected impact: none.

This should be an internal production-code split with stable crate-root
exports. If preserving exports requires changing public paths or signatures,
stop and re-enter planning.

## Validation

- `cargo test -p underlay-migration-core --all-features decision_memory`

Next code batch validation:

- `cargo test -p underlay-migration-core --all-features decision_memory`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
