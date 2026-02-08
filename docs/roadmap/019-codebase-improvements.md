# 019 – Codebase Improvements: Simplification, Deduplication & Reorganisation

## Overview

Address findings from the 2026-02-08 codebase audit (`docs/reports/2026-02-08-codebase-audit.md`). Improvements are grouped into four phases by breaking-change blast radius. Each item that changes public API is followed by updates to all consuming apps.

### Consuming Apps

| App | Location | Status |
|-----|----------|--------|
| Acowtancy | `/Users/betterthanclay/Dev/apps/acowtancy` | Active, heaviest underlay consumer |
| Compli-me | `/Users/betterthanclay/Dev/apps/compli-me` | Active |
| Songsprout | `/Users/betterthanclay/Dev/apps/songsprout` | Active |
| Loophole Composer | `/Users/betterthanclay/Dev/apps/loophole/composer` | Out of date, minimal underlay usage |
| Underlay Reference | `/Users/betterthanclay/Dev/apps/underlay-reference` | Reference implementation |

## Decision

- [x] All items from codebase audit approved for implementation
- [x] Breaking changes propagated to consuming apps as part of each item
- [x] Phased execution by blast radius

## Progress Checklist

- [x] Phase 19.1 complete (no breaking changes)
- [x] Phase 19.2 complete (import path changes)
- [x] Phase 19.3 complete (API surface changes)
- [x] Phase 19.4 complete (new additive infrastructure)

---

## Phase 19.1 – No Breaking Changes (internal cleanup)

### 19.1.1 Fix feature flag declarations

Fix Cargo.toml feature flags that don't properly declare dependencies:

- [x] `underlay-http`: Add `dep:tracing` to `error-logging` feature
- [x] `underlay-jobs`: Make `scheduler` require `postgres`
- [x] `underlay-jobs`: Make `outbox` require `postgres`
- [x] Verify all crates compile with each feature flag in isolation

### 19.1.2 Audit table validation dedup

- [x] Extract `validate_table_name()` helper in `underlay-audit`
- [x] Replace 4 copy-pasted validation blocks in `writer.rs` and `query.rs`
- [x] Run audit crate tests

### 19.1.3 Standardise internal Uuid imports

- [x] Replace all `uuid::Uuid` with `underlay_core::Uuid` inside underlay crates
- [x] Keep `uuid` as a workspace dep for re-export only
- [x] Run full test suite

### 19.1.4 Replace once_cell with std::LazyLock

- [x] Replace `once_cell::sync::Lazy` with `std::sync::LazyLock` in `underlay-core/src/slugify.rs`
- [x] Replace in `underlay-validation/src/validators.rs` if applicable
- [x] Remove `once_cell` from Cargo.toml if no longer needed
- [x] Verify minimum Rust version is 1.80+

### 19.1.5 Auth error boilerplate reduction (internal only)

- [x] Add `impl_auth_error_from!` macro to `underlay-auth`
- [x] Replace manual `From` impls in all 6 auth provider crates
- [x] Fix semantic issues: `AccountLocked` should NOT map to `RateLimited`
- [x] Fix hard-coded `retry_after_seconds: 300` in email-totp
- [x] Run full auth test suite

### Acceptance Criteria (Phase 19.1)

- [x] All feature flags properly enforce their dependencies
- [x] No duplicate table validation code in audit crate
- [x] `uuid::Uuid` not directly imported anywhere in underlay crates
- [x] `once_cell` removed from dependencies
- [x] Auth error conversions use declarative macro
- [x] All tests pass, no consuming app changes needed

---

## Phase 19.2 – Import Path Changes (crate merges)

Each merge requires updating consuming apps that import from the old crate.

### 19.2.1 Merge underlay-image into underlay-media

**Impact analysis:**
- Acowtancy: re-exports via `farmyard_infra::image` → update re-export source
- Underlay Reference: imports `underlay_image::generate_thumbnail` in jobs/media.rs → update import
- Compli-me: does not use → no change
- Songsprout: does not use → no change
- Loophole: does not use → no change

**Underlay changes:**
- [x] Move `underlay-image/src/lib.rs` contents to `underlay-media/src/image.rs`
- [x] Add `pub mod image;` to `underlay-media/src/lib.rs` (always available, not feature-gated)
- [x] Re-export key types: `pub use image::{generate_thumbnail, generate_square_thumbnail, ThumbnailConfig, ThumbnailResult, ImageError, ...}`
- [x] Remove `underlay-image` crate directory
- [x] Remove from workspace Cargo.toml
- [x] Update `underlay-media/Cargo.toml`: move `image` crate dep from optional to required
- [x] Deduplicate `generate_version_renditions()` / `generate_renditions_for_version()` in renditions.rs
- [x] Run media tests

**App updates:**
- [x] Acowtancy: update `farmyard_infra::image` re-exports from `underlay_media::image`
- [x] Acowtancy: remove `underlay-image` from workspace Cargo.toml
- [x] Underlay Reference: change import to `underlay_media::image::{generate_thumbnail, ThumbnailConfig}`
- [x] Underlay Reference: remove `underlay-image` from workspace Cargo.toml
- [x] Songsprout: remove `underlay-image` from workspace Cargo.toml if listed

### 19.2.2 Merge underlay-openapi into underlay-http

**Impact analysis:**
- Acowtancy: heavy usage of `ApiListResponse`, `ApiSingleResponse` → update imports
- Songsprout: declared but unused → remove from deps
- Compli-me: not used → no change
- Loophole: not used → no change
- Underlay Reference: not used → no change

**Underlay changes:**
- [x] Move `underlay-openapi/src/lib.rs` to `underlay-http/src/openapi.rs`
- [x] Add `openapi` feature to `underlay-http/Cargo.toml` gated on `utoipa`
- [x] Add `#[cfg(feature = "openapi")] pub mod openapi;` to `underlay-http/src/lib.rs`
- [x] Remove `underlay-openapi` crate directory
- [x] Remove from workspace Cargo.toml
- [x] Run http crate tests

**App updates:**
- [x] Acowtancy: add `openapi` feature to `underlay-http` dependency
- [x] Acowtancy: replace all `underlay_openapi::` imports with `underlay_http::openapi::`
- [x] Acowtancy: remove `underlay-openapi` from workspace Cargo.toml
- [x] Songsprout: remove `underlay-openapi` from workspace Cargo.toml

### 19.2.3 Absorb underlay-auth-state into underlay-auth

**Impact analysis:**
- Acowtancy: imports `AuthStateError`, `AuthStateStore` → update imports
- Compli-me: imports `AuthStateError`, `AuthStateStore` → update imports
- Underlay Reference: imports `AuthStateError`, `AuthStateStore` → update imports
- Songsprout: declared but unused → remove from deps
- Loophole: not used → no change

**Underlay changes:**
- [x] Move `underlay-auth-state/src/lib.rs` to `underlay-auth/src/state.rs`
- [x] Add `postgres` feature to `underlay-auth/Cargo.toml` (gated on `sqlx`)
- [x] Add `#[cfg(feature = "postgres")] pub mod state;` to `underlay-auth/src/lib.rs`
- [x] Re-export: `#[cfg(feature = "postgres")] pub use state::{AuthStateStore, AuthStateError};`
- [x] Remove `underlay-auth-state` crate directory
- [x] Remove from workspace Cargo.toml
- [x] Run auth tests

**App updates:**
- [x] Acowtancy: replace `underlay_auth_state::` with `underlay_auth::state::`
- [x] Acowtancy: add `postgres` feature to `underlay-auth` dependency
- [x] Acowtancy: remove `underlay-auth-state` from workspace Cargo.toml
- [x] Compli-me: replace `underlay_auth_state::` with `underlay_auth::state::`
- [x] Compli-me: add `postgres` feature to `underlay-auth` dependency
- [x] Compli-me: remove `underlay-auth-state` from workspace Cargo.toml
- [x] Underlay Reference: replace `underlay_auth_state::` with `underlay_auth::state::`
- [x] Underlay Reference: add `postgres` feature to `underlay-auth` dependency
- [x] Underlay Reference: remove `underlay-auth-state` from workspace Cargo.toml
- [x] Songsprout: remove `underlay-auth-state` from workspace Cargo.toml

### 19.2.4 Extract Argon2 hasher to underlay-auth

**Impact analysis:**
- No consuming app imports `Argon2Hasher` directly from underlay crates — they all use it through the auth service types. This is an internal-only dependency change.

**Underlay changes:**
- [x] Copy `hasher.rs` (Argon2Hasher, PasswordHasherExt, PasswordVerifierExt) from `underlay-auth-password` to `underlay-auth/src/hashing.rs`
- [x] Add `hashing` feature to `underlay-auth/Cargo.toml` (gated on `argon2`)
- [x] Add `#[cfg(feature = "hashing")] pub mod hashing;` to `underlay-auth/src/lib.rs`
- [x] Update `underlay-auth-password` to import from `underlay-auth::hashing` instead of defining locally
- [x] Update `underlay-auth-email-totp` to depend on `underlay-auth` (with `hashing` feature) instead of `underlay-auth-password`
- [x] Remove `underlay-auth-email-totp` → `underlay-auth-password` dependency
- [x] Run all auth tests

### Acceptance Criteria (Phase 19.2)

- [x] `underlay-image` crate removed, all functionality in `underlay-media::image`
- [x] `underlay-openapi` crate removed, all functionality in `underlay-http::openapi`
- [x] `underlay-auth-state` crate removed, all functionality in `underlay-auth::state`
- [x] `underlay-auth-email-totp` no longer depends on `underlay-auth-password`
- [x] All 5 consuming apps compile and their tests pass
- [x] Crate count reduced from 29 to 26

---

## Phase 19.3 – API Surface Changes

### 19.3.1 Unify validation frameworks

**Impact analysis:**
- Acowtancy: 50+ uses of `validation_to_app_error` from `underlay-http` → migrate to `underlay-validation`
- Compli-me: does not use → no change
- Songsprout: does not use → no change
- Loophole: does not use → no change
- Underlay Reference: does not use → no change

**Underlay changes:**
- [x] Move `ValidationResult` and `parse_uuid_for_validation` from `underlay-http/src/field_validation.rs` to `underlay-validation`
- [x] Move `validation_to_app_error` and `nightfire_validation_to_app_error` logic into `underlay-validation` (with optional `axum` feature)
- [x] Remove `validation` feature from `underlay-http/Cargo.toml`
- [x] Remove `underlay-http/src/validation.rs`
- [x] Remove `underlay-http/src/field_validation.rs` (after moving types)
- [x] Run validation crate tests

**App updates:**
- [x] Acowtancy: add `underlay-validation` with `axum` feature to workspace deps
- [x] Acowtancy: replace all `underlay_http::validation_to_app_error` with `underlay_validation::validation_to_app_error`
- [x] Acowtancy: replace `underlay_http::parse_uuid_for_validation` with `underlay_validation::parse_uuid_for_validation`
- [x] Acowtancy: remove `validation` feature from `underlay-http` dependency
- [x] Acowtancy: verify all 50+ usage sites compile

### 19.3.2 Standardise error conventions

**Underlay changes:**
- [x] Add `AuditError` enum to `underlay-audit` wrapping `sqlx::Error`
- [x] Rename inconsistent result aliases to `Result<T>` pattern
- [x] Standardise Uuid imports across all crates (ensure `underlay_core::Uuid` only)
- [x] Document error convention in CLAUDE.md and AGENTS.md

**App updates (if result alias names change):**
- [x] Check each app for direct references to old result type aliases
- [x] Update any affected imports

### 19.3.3 Clean up underlay-core exports

**Impact analysis:**
- Slug functions: no app uses them → safe to remove
- `RawUuid`: Acowtancy re-exports and uses it in 2 places. Loophole re-exports it.

**Underlay changes:**
- [x] Remove slug module from `underlay-core` (move to `underlay-validation` or a utility crate if needed later)
- [x] Keep `RawUuid` (Acowtancy uses it)
- [x] Run core tests

**App updates:**
- [x] Verify no app imports slug functions (confirmed: none do)

### Acceptance Criteria (Phase 19.3)

- [x] Single validation framework (`underlay-validation`)
- [x] No `validation` feature on `underlay-http`
- [x] Consistent error conventions documented and applied
- [x] Slug module removed from core
- [x] All 5 consuming apps compile and tests pass

---

## Phase 19.4 – New Infrastructure (additive, no breaking changes)

### 19.4.1 Create underlay-aws shared config

**Underlay changes:**
- [x] Create `underlay-aws` crate with unified `aws-config` setup
- [x] Provide `AwsConfig` builder with credential chain, region, endpoint
- [x] Update `underlay-blob` (S3 feature) to use `underlay-aws`
- [x] Update `underlay-email` (SES feature) to use `underlay-aws`
- [x] Run blob and email tests

### 19.4.2 Create underlay-http-client wrapper

**Underlay changes:**
- [x] Create `underlay-http-client` crate wrapping `reqwest::Client`
- [x] Sensible defaults: timeouts, rustls-tls, user-agent
- [x] Update `underlay-auth-oauth`, `underlay-auth-password` (hibp), `underlay-http` (embed) to use it
- [x] Run affected crate tests

### Acceptance Criteria (Phase 19.4)

- [x] AWS crates share configuration through `underlay-aws`
- [x] HTTP client crates share configuration through `underlay-http-client`
- [x] No consuming app changes required

---

## Validation Plan

- [x] All underlay crates build with `cargo check --all-features`
- [x] All underlay tests pass with `cargo test --all-features`
- [x] File length check passes: `bash scripts/check-file-length.sh`
- [x] Each consuming app builds after each phase
- [x] Package map updated to reflect crate count changes
- [x] AGENTS.md and CLAUDE.md updated with any new conventions

## Success Metrics

- [x] Crate count reduced from 29 to 26 (image, openapi, auth-state removed)
- [x] Single validation framework in use
- [x] Auth error boilerplate eliminated via macro
- [x] Feature flags properly enforce dependencies
- [x] Error conventions documented and consistent
- [x] All consuming apps updated and compiling

## Execution Notes

1. Commit after each numbered item (19.1.1, 19.1.2, etc.) — not at phase boundaries.
2. For Phase 19.2 merges: update underlay first, then update each consuming app, then commit everything together.
3. For Loophole Composer: only update if changes affect its minimal deps (core, metrics, db). Don't fix unrelated issues.
4. Run `cargo check` on each consuming app after changes — don't run full test suites unless the app has a simple `cargo test` path.
