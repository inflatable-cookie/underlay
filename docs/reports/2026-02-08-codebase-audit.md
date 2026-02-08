# Underlay Rust Codebase Audit — 2026-02-08

Comprehensive audit of all 29 Rust crates covering: duplication, simplification, reorganisation, consistency, and dependency structure. Each recommendation notes the breaking-change impact on consuming apps.

---

## Executive Summary

The codebase is well-structured overall. The layered crate architecture is sound, feature flags are generally well-designed, and the separation between core/auth/data/infra/dev is clear. The main improvement areas are:

1. **Dual validation frameworks** — two parallel validation systems exist (underlay-validation + underlay-http/validation)
2. **Auth error boilerplate** — every auth crate repeats the same error conversion pattern
3. **Merge candidates** — image→media, openapi→http, and auth-state could be absorbed
4. **Inconsistent conventions** — error type naming, Uuid imports, result aliases vary across crates
5. **Dependency consolidation** — AWS SDK, reqwest, crypto crates duplicated across multiple crates

---

## 1. Unify Validation Frameworks

**Problem:** Two parallel validation systems exist:
- **Path A** (`underlay-validation`): Custom `Validate` trait, derive macro, `ValidatedJson` extractor, built-in validators
- **Path B** (`underlay-http` with `validation` feature): Wraps the `validator` crate, converts to `AppError`

Both provide field-level validation, error types, and Axum integration. Consumers must choose one.

**Additionally:** `underlay-http/field_validation.rs` provides `ValidationResult` for live validation endpoints, which overlaps with `underlay-validation`'s `ValidationError` type.

**Recommendation:** Keep `underlay-validation` as the canonical validation system. Remove the `validation` feature from `underlay-http` (which just wraps the third-party `validator` crate). Move the live `ValidationResult` type into `underlay-validation` so it owns all validation concerns.

**Breaking changes:** Apps using `underlay-http`'s `validation_to_app_error()` or `ValidateExt` trait would switch to `underlay-validation`'s equivalents.

**Files affected:**
- `underlay-http/src/validation.rs` — remove
- `underlay-http/src/field_validation.rs` — move `ValidationResult` to underlay-validation
- `underlay-http/Cargo.toml` — remove `validation` feature
- `underlay-validation/src/lib.rs` — absorb live validation types

---

## 2. Reduce Auth Error Boilerplate

**Problem:** Every auth provider crate implements nearly identical `From<CrateError> for AuthError` conversions. Six crates repeat the same match→map pattern (auth-jwt, auth-password, auth-totp, auth-email-totp, auth-oauth, auth-webauthn).

**Additionally:** Some mappings have semantic issues:
- `PasswordAuthError::AccountLocked` maps to `AuthError::RateLimited` (conflates two concepts)
- `EmailTotpError::RateLimited` hard-codes `retry_after_seconds: 300` instead of reading from config

**Recommendation:** Add a declarative macro to `underlay-auth` that generates the `From` impl:
```rust
impl_auth_error_from!(JwtError {
    Expired => SessionExpired,
    InvalidToken => TokenInvalid,
    // ...
});
```
Fix the semantic mismatches at the same time.

**Breaking changes:** None if error variants keep the same names. The `AccountLocked` → `RateLimited` fix would change error codes seen by consuming apps.

**Files affected:**
- `underlay-auth/src/lib.rs` — add macro
- All 6 auth provider crates' error modules — replace manual `From` impls

---

## 3. Merge underlay-image into underlay-media

**Problem:** `underlay-image` is used exclusively by `underlay-media`. It provides ~400 lines of thumbnail generation and format detection, but all orchestration (rendition service, config presets, storage integration) lives in media. No other crate imports it.

**Recommendation:** Absorb `underlay-image` into `underlay-media` as an internal `image` module. Remove the separate crate.

**Breaking changes:** Any app importing `underlay-image` directly would switch to `underlay-media`. Based on the dependency graph, no app does this — they all go through media.

**Additional cleanup in media after merge:**
- Deduplicate `generate_version_renditions()` vs `generate_renditions_for_version()` in `renditions.rs` (lines 380-546) — nearly identical logic
- Collapse three identical `TypedId` wrappers (`MediaId`, `MediaVersionId`, `MediaRenditionId`) into a generic `TypedId<T>` pattern

---

## 4. Merge underlay-openapi into underlay-http

**Problem:** `underlay-openapi` is 56 lines — four type wrappers around `utoipa` (`ApiUuid`, `ApiSingleResponse`, `ApiListResponse`, `ApiErrorEnvelope`). It exists solely to provide OpenAPI schema annotations for types that already live in `underlay-core` and `underlay-http`.

**Recommendation:** Move these into `underlay-http` as a feature-gated `openapi` module (gated on `utoipa`). This removes a crate without losing modularity.

**Breaking changes:** Apps importing `underlay_openapi::*` would change to `underlay_http::openapi::*`.

---

## 5. Absorb underlay-auth-state into underlay-auth

**Problem:** `underlay-auth-state` is a single 238-line file containing `AuthStateStore` — a thin wrapper around `PgPool` for storing authentication flow state. It's too thin to justify a separate crate.

**Recommendation:** Move into `underlay-auth` behind a `postgres` feature flag.

**Breaking changes:** Import path changes from `underlay_auth_state::*` to `underlay_auth::state::*`.

---

## 6. Extract Argon2 Hasher from underlay-auth-password

**Problem:** `underlay-auth-email-totp` depends on `underlay-auth-password` just to reuse the `Argon2Hasher`. This creates tight coupling between two crates that should be independent providers.

**Recommendation:** Move `Argon2Hasher` (and the `PasswordHasherExt` / `PasswordVerifierExt` traits) into `underlay-auth` as an optional `hashing` module. Both password and email-totp crates then depend on the umbrella crate for hashing, not on each other.

**Breaking changes:** Import path for `Argon2Hasher` changes. The dependency from email-totp → auth-password is removed.

---

## 7. Standardise Error Conventions

**Problem:** Error types, result aliases, and error patterns vary across crates:

| Inconsistency | Examples |
|---------------|----------|
| Result alias naming | `Result<T>` vs `EmailResult<T>` vs `BlobResult<T>` vs none |
| Error abstraction | RateLimitError (2 variants) vs EmailError (11 variants) vs audit (raw `sqlx::Error`) |
| Uuid imports | Some use `uuid::Uuid`, others use `underlay_core::Uuid` |
| Error context | Some errors include details, others are bare enums |

**Recommendation:** Define a standard pattern in CLAUDE.md / AGENTS.md:
- Result alias: `pub type Result<T> = std::result::Result<T, CrateError>;` (short form, consistent)
- Always use `underlay_core::Uuid` not `uuid::Uuid` directly
- Always use `thiserror::Error` derive
- 3-6 variants per error enum (not 2, not 11)

Audit crate should wrap `sqlx::Error` in its own `AuditError` type.

**Breaking changes:** Result type name changes in some crates. Adding `AuditError` wrapper changes audit crate return types.

---

## 8. Consolidate AWS SDK Setup

**Problem:** Two crates independently pull in AWS SDK + config:
- `underlay-blob` (S3): `aws-sdk-s3`, `aws-config`
- `underlay-email` (SES): `aws-sdk-sesv2`, `aws-config`

Both need identical credential/region configuration.

**Recommendation:** Create `underlay-aws` (or a shared module in `underlay-core`) that provides:
- Unified `aws-config` setup with credential chain
- Re-exports for service clients
- Common region/endpoint configuration

**Breaking changes:** None if done as a new dependency. Consuming apps don't interact with AWS config directly.

---

## 9. Consolidate HTTP Client (reqwest)

**Problem:** `reqwest` appears as a dependency in 4 crates with different feature sets:
- `underlay-auth-oauth` (OAuth2 token exchange)
- `underlay-auth-password` (optional HIBP check)
- `underlay-http` (optional embed proxy)
- `underlay-testing` (optional test server client)

**Recommendation:** Create `underlay-http-client` that provides a configured `reqwest::Client` with sensible defaults (timeouts, TLS, user-agent). Individual crates depend on this wrapper instead of reqwest directly.

**Breaking changes:** None — internal dependency change only.

---

## 10. Simplify underlay-http Cookie Helpers

**Problem:** Four near-identical functions build cookies with the same domain/secure/same-site settings:
- `refresh_token_cookie()`
- `clear_refresh_token_cookie()`
- `logged_in_cookie()`
- `clear_logged_in_cookie()`

Each is ~20 lines with identical cookie-building logic.

**Recommendation:** Extract shared cookie builder, reduce to two generic functions: `set_cookie()` and `clear_cookie()` that take a cookie name and value.

**Breaking changes:** Function signatures change. Consuming apps calling these helpers would update.

---

## 11. Clean Up underlay-core

**Problem:** The slug module (`slugify.rs`, 163 lines) exports 6 public items but grep shows no usage in any dependent crate. It contains domain-specific logic (reserved slugs, URL conflict detection) that doesn't belong in a core primitives crate.

Additionally, `RawUuid` is exported but unused.

**Recommendation:**
- Confirm whether slug functions are used by consuming apps (not just underlay crates). If not, remove or move to a utility crate.
- Remove `RawUuid` if unused.
- Replace `once_cell::sync::Lazy` with `std::sync::LazyLock` (stable since Rust 1.80).

**Breaking changes:** If apps use slug functions, they'd need to import from a different crate.

---

## 12. Fix Feature Flag Issues

**Problem:** Several feature flag declarations are incomplete or inconsistent:

| Crate | Issue |
|-------|-------|
| `underlay-http` | `error-logging` uses `tracing` without declaring `dep:tracing` |
| `underlay-jobs` | `scheduler` and `outbox` require `postgres` but don't declare it |

**Recommendation:** Fix Cargo.toml declarations so invalid feature combinations fail at compile time with clear messages rather than cryptic errors.

**Breaking changes:** None — apps already enable the required features; this just enforces what was already needed.

---

## 13. Audit Table Validation Duplication

**Problem:** In `underlay-audit`, the same 8-line table name validation block is copy-pasted 4 times across `writer.rs` and `query.rs`.

**Recommendation:** Extract to a `fn validate_table_name(table: &str) -> Result<(), sqlx::Error>` helper.

**Breaking changes:** None — internal refactor.

---

## Items Evaluated and Left As-Is

These were investigated but don't warrant changes:

| Item | Reason to keep |
|------|----------------|
| **observability + metrics as separate crates** | Different dependencies, can be used independently |
| **underlay-events as separate crate** | Valid contract-only crate with meaningful tests |
| **underlay-suggestions as separate crate** | Clean, focused utility; used by multiple UI patterns |
| **underlay-nightfire standalone** | No internal underlay deps, clean protocol crate |
| **underlay-ratelimit standalone** | No internal deps, used by multiple auth crates |
| **Split underlay-jobs further** | Current feature-flag approach works; splitting would create 3-4 tiny crates |
| **Split underlay-auth-password** | Complexity is warranted; strength analysis + hashing are tightly coupled |

---

## Suggested Execution Order

Grouped by breaking-change blast radius:

### Phase A — No Breaking Changes (internal cleanup)
1. Fix feature flag declarations (item 12)
2. Audit table validation dedup (item 13)
3. Media rendition dedup after image merge (item 3, internal part)
4. Standardise Uuid imports, result aliases (item 7, non-public parts)
5. Replace `once_cell` with `std::LazyLock` in core (item 11, partial)

### Phase B — Minor Breaking Changes (import path changes only)
6. Merge underlay-image into underlay-media (item 3)
7. Merge underlay-openapi into underlay-http (item 4)
8. Absorb underlay-auth-state into underlay-auth (item 5)
9. Extract Argon2 hasher to underlay-auth (item 6)

### Phase C — Moderate Breaking Changes (API surface changes)
10. Unify validation frameworks (item 1)
11. Auth error macro + semantic fixes (item 2)
12. Standardise error conventions across all crates (item 7)
13. Simplify cookie helpers (item 10)
14. Clean up underlay-core exports (item 11)

### Phase D — New Infrastructure (no breaking changes, additive)
15. Create underlay-aws shared config (item 8)
16. Create underlay-http-client wrapper (item 9)
