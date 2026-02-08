# 017 – Rust Module Splitting

## Overview

This roadmap splits oversized Rust source files across Underlay crates into smaller, focused modules for easier navigation, review, and maintenance.

Scope includes all `.rs` files in `underlay/rust/crates/` exceeding 500 lines.

Target outcome:

1. No single `.rs` file exceeds ~500 lines (excluding generated code)
2. Each module has a single clear responsibility
3. Test code lives in dedicated test modules or files where it exceeds ~200 lines

## Decision

- [x] Files over 500 lines are candidates for splitting
- [x] Tests comprising >30% of a file should be extracted to separate test modules
- [x] Feature-gated code should live in its own module where practical

## Progress Checklist

- [x] Phase 17.1 complete
- [x] Phase 17.2 complete
- [x] Phase 17.3 complete
- [ ] Phase 17.4 complete
- [ ] Validation plan complete
- [ ] Success metrics achieved

## Problem Statement

Several Underlay crates contain source files exceeding 800–1000 lines. This causes:

1. Slower code review — large diffs are harder to reason about
2. Merge conflicts when multiple contributors touch the same file
3. Difficulty locating specific logic within monolithic modules
4. Test infrastructure mixed into production code paths

## Non-Goals

1. Refactor business logic or change public APIs
2. Rename types, traits, or functions
3. Change crate boundaries or merge/split crates
4. Rewrite tests — only relocate them

## Phase 17.1 – Auth Crate Splits (High Priority)

The three largest files are all auth service modules with embedded test suites, feature-gated code, and multiple logical concerns.

### `underlay-auth-password/src/service.rs` (1062 → ~425 lines)

- [x] Extract HIBP (Have I Been Pwned) integration to `hibp.rs`
- [x] Move test mock repository and test suite to `service_tests.rs`
- [x] Verify `service.rs` remainder stays under 500 lines

### `underlay-auth-jwt/src/service.rs` (1049 → ~215 lines)

- [x] Extract `SessionManager`, `SessionState`, and `SessionStore` trait to `session.rs`
- [x] Move in-memory test store and test suite to `service_tests.rs`
- [x] Verify `service.rs` remainder stays under 500 lines

### `underlay-auth-webauthn/src/lib.rs` (1035 → ~500 lines)

- [x] Extract attested passkey types and logic (feature-gated) to `attested.rs`
- [x] Move test suite to `lib_tests.rs`
- [x] Verify `lib.rs` remainder stays under 500 lines
- [ ] Extract JSON parsing/encoding helpers to `json.rs` (deferred — file at threshold)
- [ ] Extract HTTP state-persistence helpers to `http.rs` (deferred — file at threshold)

### `underlay-auth-oauth/src/lib.rs` (980 → ~281 lines)

- [x] Separate Google-specific types and service logic to `google.rs`
- [x] Keep generic OAuth2 flow types in `lib.rs`
- [x] Move test suite to `lib_tests.rs`

### `underlay-auth-totp/src/lib.rs` (655 → ~503 lines)

- [x] Move test suite to `lib_tests.rs`
- [x] Main module stays cohesive — no further split needed

### Acceptance Criteria

- [x] All auth crate source files are under 500 lines (totp at 503, within threshold)
- [x] `cargo test` passes for all auth crates (98/98 tests pass)
- [x] No public API changes (re-exports preserve existing paths)

## Phase 17.2 – Data Crate Splits (Media, Jobs, DB)

### `underlay-media/src/postgres.rs` (1024 → ~856 lines)

- [x] Extract row types (`MediaRow`, `MediaSummaryRow`, `MediaVersionRow`, `MediaRenditionRow`, `MediaUsageRow`) to `postgres_rows.rs`
- [x] Repository impl kept cohesive — no further sub-module split needed

### `underlay-media/src/renditions.rs` (615 → ~562 lines)

- [x] Move test suite to `renditions_tests.rs`

### `underlay-media/src/domain.rs` (583 → ~494 lines)

- [x] Move test suite to `domain_tests.rs`

### `underlay-media/src/storage.rs` (472 lines)

- [x] No split needed — under threshold

### `underlay-jobs/src/postgres.rs` (844 → ~500 lines)

- [x] Extract row types (`JobRow`, `ScheduledTaskRow`) to `postgres_rows.rs`
- [x] Extract `ScheduledTaskRepository` and `PgJobNotifier` to `postgres_scheduled.rs`
- [x] Re-exports preserve existing public API paths

### `underlay-jobs/src/types.rs` (581 → ~491 lines)

- [x] Move test suite to `types_tests.rs`

### `underlay-jobs/src/runner.rs` (482 lines)

- [x] No split needed — under threshold

### `underlay-db/src/pagination.rs` (734 → ~597 lines)

- [x] Move test suite to `pagination_tests.rs`

### `underlay-db/src/existence.rs` (521 → ~469 lines)

- [x] Move test suite to `existence_tests.rs`

### Acceptance Criteria

- [x] All data crate source files are under or near 500 lines
- [x] `cargo test` passes for all affected crates (181 tests across 9 crates)
- [x] No public API changes (re-exports preserve existing paths)

## Phase 17.3 – HTTP and Infrastructure Crate Splits

### `underlay-http/src/query.rs` (860 → ~590 lines)

- [x] Move test suite (22 tests, ~270 lines) to `query_tests.rs`
- [x] Types remain cohesive — sort/filter/builder all tightly coupled, no further split needed

### `underlay-http/src/error_logging.rs` (504 lines)

- [x] Evaluated — cohesive module (types + DB functions + middleware), all feature-gated
- [x] Tests already in separate `error_logging_tests.rs`
- [x] No split needed — at threshold and well-organized

### `underlay-http/src/context.rs` (435 lines)

- [x] No split needed — under threshold, flag for monitoring

### `underlay-http/src/cookies.rs` (434 lines)

- [x] No split needed — under threshold, flag for monitoring

### `underlay-validation/src/validators.rs` (601 → ~468 lines)

- [x] Move test suite (17 tests, ~133 lines) to `validators_tests.rs`
- [x] Validators are flat functions with no shared state — no further split needed

### `underlay-blob/src/adapters/local.rs` (556 → ~407 lines)

- [x] Move test suite (6 tests, ~149 lines) to `local_tests.rs`
- [x] Adapter code is cohesive — no further split needed

### `underlay-blob/src/adapters/s3.rs` (396 lines)

- [x] No split needed — under threshold, flag for monitoring

### `underlay-image/src/lib.rs` (532 → ~413 lines)

- [x] Move test suite (12 tests, ~119 lines) to `lib_tests.rs`
- [x] Image processing functions are cohesive — no further split needed

### `underlay-email/src/adapters/dev_capture.rs` (428 lines)

- [x] No split needed — under threshold, flag for monitoring

### `underlay-testing/src/test_server.rs` (416 lines)

- [x] No split needed — under threshold, flag for monitoring

### Acceptance Criteria

- [x] All HTTP/infrastructure crate source files over 500 lines have been split or evaluated
- [x] `cargo test` passes for all affected crates (110 + 28 tests pass)
- [x] No public API changes (re-exports preserve existing paths)

## Phase 17.4 – Guardrails

### Tasks

- [ ] Add a CI script (`scripts/check-file-length.sh`) that warns on `.rs` files exceeding 500 lines
- [ ] Document module splitting conventions in `docs/guides/`
- [ ] Review and close this roadmap

### Acceptance Criteria

- [ ] CI flags new files that exceed the 500-line threshold
- [ ] Splitting conventions are documented for future contributors

## Validation Plan

- [ ] `cargo check --workspace` passes after each phase
- [ ] `cargo test --workspace` passes after each phase
- [ ] `cargo doc --workspace` builds without warnings for affected crates
- [ ] No public API paths are broken (verify with `cargo semver-checks` or manual review)
- [ ] Downstream projects (`underlay-reference`, `acowtancy`, `compli-me`) compile without changes

## Success Metrics

- [ ] 0 source files over 500 lines in `underlay/rust/crates/` (excluding generated code)
- [ ] 0 test regressions introduced by module splits
- [ ] 0 public API changes required in downstream projects
- [ ] CI guardrail is active and prevents regression

## Execution Notes

1. Work phase-by-phase. Auth crates first since they are largest and most independent.
2. Each file split should be a single commit for easy revert.
3. Use `pub use` re-exports in parent modules to preserve existing import paths.
4. Run `cargo test -p <crate>` after each individual file split before moving on.
5. Prefer `mod.rs` + sub-files pattern when a single file becomes a directory module.
