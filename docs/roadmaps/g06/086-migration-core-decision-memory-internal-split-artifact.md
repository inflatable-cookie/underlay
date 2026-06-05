# g06.086 Artifact - Migration-Core Decision-Memory Internal Split

## Summary

`underlay-migration-core/src/decision_memory.rs` is now a small public front
door. The previous production module was split into focused modules under
`underlay-migration-core/src/decision_memory/`.

The split preserves the crate-root public API. It does not change
decision-memory serialization semantics, migration verification, drift behavior,
or consumer apps.

## Module Shape

- `decision_memory.rs`: public front door and stable `pub use` surface
- `decision_memory/models.rs`: public model structs and enums
- `decision_memory/fingerprint.rs`: decision and record fingerprinting plus
  canonical JSON normalization
- `decision_memory/index.rs`: journal NDJSON parsing and index build/merge/parse
- `decision_memory/validation.rs`: journal, unresolved decision, index, and
  digest validation
- `decision_memory/reuse.rs`: decision reuse evaluation and version
  compatibility
- `decision_memory/provenance.rs`: effective decision selection and provenance
  chains

## Behavior Preserved

- focused decision-memory tests pass
- decision and record fingerprint behavior is unchanged
- journal parsing and validation behavior is unchanged
- index build, merge, parse, and validation behavior is unchanged
- strict and compatible reuse behavior is unchanged
- plugin invalidation behavior is unchanged
- effective decision provenance ranking is unchanged
- provenance chain ordering is unchanged

## Public API Impact

None.

All crate-root decision-memory re-exports remain stable.

## Validation

- `cargo test -p underlay-migration-core --all-features decision_memory`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 46 findings / 40 warnings / 5 errors to 45 findings / 40
  warnings / 5 errors.
