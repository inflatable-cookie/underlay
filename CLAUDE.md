# Underlay – Claude Code Context

This file provides Claude Code-specific conventions for working on the Underlay repository.

## Quick Reference

- **Full crate inventory**: `docs/architecture/010-package-map.md` (26 Rust crates)
- **Patterns catalogue**: `docs/patterns/000-index.md`
- **Roadmap status**: `docs/roadmap/README.md`
- **Module splitting guide**: `docs/guides/041-rust-module-splitting.md`
- **Project sync checklist**: `docs/guides/200-project-sync.md`

## Test Workflow

```bash
# Test a single crate (always use --all-features)
cargo test -p underlay-http --all-features

# Check a single crate
cargo check -p underlay-http --all-features

# Test all crates
cargo test --all-features

# Run the CI file length check
bash scripts/check-file-length.sh
```

## Module Conventions

### Test file extraction
Tests are extracted to separate files using `#[path]` to keep source files under 500 lines:

```rust
// At the bottom of lib.rs or module.rs:
#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
```

The test file uses `use super::*;` and has no `mod tests` wrapper — the `#[path]` attribute handles scoping.

### Naming conventions for extracted files

| Extraction type | Source file | Extracted file |
|-----------------|------------|----------------|
| Tests | `lib.rs` | `lib_tests.rs` |
| Tests | `service.rs` | `service_tests.rs` |
| Row types | `lib.rs` | `postgres_rows.rs` |
| Scheduled jobs | `lib.rs` | `postgres_scheduled.rs` |
| Feature-gated | `lib.rs` | Named by feature (e.g., `google.rs`, `hibp.rs`, `attested.rs`) |

### Visibility patterns
- Use `pub(crate)` for types shared across modules within a crate
- Preserve `pub use` re-exports in `lib.rs` when extracting types to submodules
- Feature-gated re-exports: `#[cfg(feature = "x")] pub use submodule::Type;`

## Feature Flags

Common feature flags you'll encounter:

| Flag | Crates | Purpose |
|------|--------|---------|
| `postgres` | jobs, media, auth, http (error-logging) | PostgreSQL persistence |
| `hashing` | auth | Argon2id password hashing (used by auth-password, auth-email-totp) |
| `scheduler` | jobs | Cron-based job scheduling |
| `outbox` | jobs | Domain event outbox pattern |
| `s3` / `local` | blob | Storage backend selection |
| `smtp` / `ses` | email | Email transport selection |
| `hibp` | auth-password | Have I Been Pwned breach checking |
| `attestation` | auth-webauthn | Attested passkey verification |
| `derive` | validation | `#[derive(Validate)]` macro |
| `validator-compat` | validation | `validation_to_app_error` bridge for `validator` crate |
| `nightfire` | validation | `nightfire_validation_to_app_error` bridge |
| `field-validation` | validation | `FieldValidationResult` for live validation endpoints |
| `openapi` | http | OpenAPI response types (utoipa) |
| `db` / `server` | testing | Test infrastructure scope |

## File Length Limits

- **500 lines**: warning threshold — consider splitting
- **900 lines**: hard limit — must split before merging
- Enforced by `scripts/check-file-length.sh`

## Key Patterns

- **UUID v7** for all database-stored identifiers (`Uuid::now_v7()`)
- **`AppError`** for all error types (from `underlay-core`)
- **Snake_case** wire format for all JSON contracts
- **`ExistsCheck`** builder for flexible uniqueness validation
- **`FieldValidationResult`** (200 OK) for live field validation endpoints (from `underlay-validation`)

## Error Conventions

Each crate defines its own error enum and result alias following this pattern:

```rust
// In crate_name/src/error.rs:
#[derive(Debug, thiserror::Error)]
pub enum FooError {
    #[error("database error")]
    Db(#[from] sqlx::Error),
    // domain-specific variants...
}
pub type FooResult<T> = Result<T, FooError>;
```

Naming convention: `{Domain}Result<T>` — e.g., `AuditResult`, `MediaResult`, `EmailResult`, `BlobResult`.

## Auth Crate Family

The auth system uses an umbrella + provider pattern:

```
underlay-auth (umbrella: traits, extractors, hashing, state)
├── underlay-auth-jwt (session tokens)
├── underlay-auth-password (Argon2id)
├── underlay-auth-totp (TOTP codes)
├── underlay-auth-email-totp (email OTP)
├── underlay-auth-webauthn (passkeys)
└── underlay-auth-oauth (OAuth2 providers)
```

The `hashing` feature provides `Argon2Hasher` (used by auth-password and auth-email-totp).
The `postgres` feature provides `AuthStateStore` (flow state storage).

## Do Not

- Run TypeScript build/install in `reference/` apps (cyclic hard link issues)
- Use `Uuid::new_v4()` for database IDs (use v7)
- Create package workspaces — this is a single-package-per-language repo
- Put reports in `docs/roadmap/` or `docs/guides/` — use `docs/reports/`
