# 006 – Rust Test Coverage Improvement

**Status:** In progress

This roadmap tracks test coverage improvements across Underlay’s Rust crates, with emphasis on security-critical components.

## How To Use This Roadmap

- Every actionable item is a checkbox.
- Tick items with `[x]` when complete.
- Use the “Optional” section for nice-to-haves that should not block completion.

## High-Level Checklist

- [x] Section 1 — Coverage Baseline Measurement
- [x] Section 2 — P0: Auth JWT Tests
- [x] Section 3 — P0: Auth Password Tests
- [x] Section 4 — P1: Database Tests
- [x] Section 5 — P1: Jobs Tests
- [x] Section 6 — P2: Events Tests
- [x] Section 7 — Verification and Regression Prevention

---

## Section 1 — Coverage Baseline Measurement

Goal: Establish a measurable baseline before making improvements.

- [x] Install `cargo-tarpaulin` for coverage reporting
- [x] Generate baseline coverage report
- [x] Record baseline coverage

**Baseline (workspace):** 44.65% (501/1122)

**Current (workspace):** 59.95% (690/1151)

Commands:

```bash
# Coverage (workspace)
cargo tarpaulin --workspace --out html

# Coverage (enforce minimum threshold)
rust/scripts/check-coverage.sh
```

---

## Section 2 — P0: Auth JWT Tests

**Priority:** Critical (cryptographic operations, session security)

Crate: `rust/crates/underlay-auth-jwt`

- [x] Key generation tests (`KeyPair::generate`, base64 decode, invalid base64)
- [x] Config tests (defaults, lifetimes)
- [x] `JwtConfig::from_env` tests (missing vars, overrides, invalid numbers)
- [x] Service startup rejects mismatched keypair
- [x] Access/refresh issuance tests (required claims, unique token ids)
- [x] Validation tests (expired, not-yet-valid, leeway, bad signature, issuer/audience)
- [x] Token-use enforcement returns `UnsupportedTokenType` deterministically
- [x] SessionManager refresh replay detection and revoke behavior

---

## Section 3 — P0: Auth Password Tests

**Priority:** Critical (credential security)

Crate: `rust/crates/underlay-auth-password`

### 3.1 Hashing / Rehash

- [x] Hash is non-deterministic and verifies
- [x] `needs_rehash` detects algorithm/parameter changes

### 3.2 Strength Analyzer

- [x] Enforces minimum length and rejects common passwords
- [x] Computes features (upper/lower/digits/special, unique chars)
- [x] `validate()` rejects weak passwords and accepts strong passwords

### 3.3 Account Lockout / Rate Limiting

- [x] Failed logins increment and lock out after threshold
- [x] Successful login resets failure counts
- [x] Rate limit blocks login attempts when repository returns not-allowed

### 3.4 Password Rotation

- [x] `change_password` rejects wrong current password
- [x] `change_password` rejects same password
- [x] `change_password` updates stored hash and old password stops working
- [x] `reset_password` updates hash and allows login

### 3.5 Email Normalization

- [x] `verify_login` normalizes email for lookup (trim + lowercase)

### 3.6 Compromised Password Strategy (Local)

- [x] Local blocklist rejects compromised password when enabled

### 3.7 HIBP (feature `hibp`)

- [x] Range-response parser matches suffix case-insensitively
- [x] Range-response parser ignores malformed lines
- [x] (Optional) Run `hibp_k_anonymity_check` end-to-end against a local server

---

## Section 4 — P1: Database Tests

**Priority:** High (connection management + schema safety)

Crate: `rust/crates/underlay-db`

### 4.1 Unit Tests

- [x] `validate_schema_name` allows safe identifiers
- [x] `validate_schema_name` rejects suspicious identifiers
- [x] `DestructiveGuard` allow/disallow behavior
- [x] `DbConfig` defaults and shape

### 4.2 Postgres Integration Tests (testcontainers)

File: `rust/crates/underlay-db/tests/postgres_integration.rs`

- [x] `create_pool` connects and can run `SELECT 1::BIGINT`
- [x] `drop_schemas` requires destructive guard
- [x] `drop_schemas` drops schema with CASCADE
- [x] `drop_schemas` rejects invalid schema names

Run locally (requires Docker-compatible runtime like Colima):

```bash
cargo test -p underlay-db --test postgres_integration -- --ignored
```

---

## Section 5 — P1: Jobs Tests

**Priority:** High (runner correctness)

Crate: `rust/crates/underlay-jobs`

- [x] Registry tracks job types and handler lookup
- [x] Runner dispatches jobs to handlers and marks success
- [x] Runner records failures from handler error
- [x] Runner returns false when no jobs are available
- [x] Runner ignores unknown job types
- [x] `run_forever` sleeps when no work (time-paused test)
- [x] Type tests for `Job` + `JobHandlerError`

---

## Section 6 — P2: Events Tests

**Priority:** Medium (contract correctness)

Crate: `rust/crates/underlay-events`

- [x] Schema SQL template is present
- [x] `NewDomainEvent::now` sets expected fields
- [x] `DomainEvent` serialize/deserialize round-trip
- [x] Basic type/trait checks (Send/Sync/Clone)

---

## Section 7 — Verification and Regression Prevention

Goal: Ensure improvements persist and are runnable locally.

- [x] Add Postgres integration tests (ignored by default)
- [x] Add CI workflow to run Rust tests + integration tests
- [x] Add coverage script `rust/scripts/check-coverage.sh`
- [x] Document Colima + integration test usage in `README.md`

### Optional

- [ ] Add coverage badge (e.g. Codecov) once a reporting service is chosen

---

## Completion Criteria

- [x] Coverage baseline recorded
- [x] Workspace coverage materially improved (44.65% → 59.95%)
- [x] P0 auth crates have meaningful negative-path coverage (expired/nbf/leeway, lockout/rate-limit)
- [x] DB has real Postgres integration tests via testcontainers
- [x] Jobs runner behavior is covered, including `run_forever` sleep
- [x] CI runs unit tests and integration tests

---

## Remaining Work

- [ ] Decide on coverage reporting service (optional) and add badge
- [ ] If desired: add a mockable HIBP client interface to test end-to-end without network
