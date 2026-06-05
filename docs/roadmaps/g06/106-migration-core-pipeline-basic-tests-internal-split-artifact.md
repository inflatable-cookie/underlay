# g06.106 Artifact - Migration-Core Pipeline Basic Tests Internal Split

## Summary

`underlay-migration-core/src/tests/pipeline_basic_tests.rs` was replaced by a
focused `pipeline_basic_tests/` module directory while preserving the parent
`mod pipeline_basic_tests;` path.

New module shape:

- `pipeline_basic_tests/mod.rs`: shared imports, small fixture helpers,
  stage-order invariant, and reuse-policy invariant
- `pipeline_basic_tests/full_run.rs`: successful full-pipeline run assertions
- `pipeline_basic_tests/failures.rs`: transform and verify failure mapping
- `pipeline_basic_tests/resume.rs`: resume success and incompatible checkpoint
  rejection

## Public API Impact

None.

This was a test-only split. Production migration APIs, pipeline behavior, stage
ordering, resume compatibility rules, decision behavior, and verification
semantics were not changed.

## Validation

- `cargo test -p underlay-migration-core --all-features`
- `effigy rust:check`

`cargo test -p underlay-migration-core --all-features` passed with 43 unit
tests passed and 0 doc-tests.

`effigy doctor` still fails on the known scanner backlog:

- `scan.god-files`: 35 findings, 5 TypeScript error-level findings
- `scan.attention-markers`: 11 findings, 2 error-level findings
- `scan.comment-ratio`: 12 findings, 3 error-level findings

`pipeline_basic_tests.rs` no longer appears in the god-file report. The next
largest Rust warning is `rust/crates/underlay-media/src/nightfire.rs`.

## Next Target Evidence

Queue `g06.107` as a media Nightfire modularity audit before splitting
`underlay-media/src/nightfire.rs`. Nightfire is a production media usage
surface, so the next batch should classify public exports, resolver/extractor
behavior, registry behavior, sync integration, and existing tests before
moving code.
