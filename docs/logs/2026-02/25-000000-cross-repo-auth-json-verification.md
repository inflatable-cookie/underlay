# Cross-Repo Auth + JSON Verification (2026-02-25)

## Scope

Validation run across:
- underlay
- underlay-reference
- acowtancy
- compli-me
- songsprout

## Commands Run

From `underlay`:

```bash
./scripts/check-json-naming.sh rust
./scripts/check-json-naming.sh /Users/betterthanclay/Dev/projects/underlay-reference/acme-api/crates
./scripts/check-json-naming.sh /Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates /Users/betterthanclay/Dev/projects/underlay/scripts/json-naming-allowlist.txt
./scripts/check-json-naming.sh /Users/betterthanclay/Dev/projects/compli-me/api/crates
./scripts/check-json-naming.sh /Users/betterthanclay/Dev/projects/songsprout/nursery/crates

./scripts/check-route-error-patterns.sh /Users/betterthanclay/Dev/projects/underlay-reference/acme-api/crates/api/src/routes
./scripts/check-route-error-patterns.sh /Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates/api/src/routes
./scripts/check-route-error-patterns.sh /Users/betterthanclay/Dev/projects/compli-me/api/crates/api/src/routes
./scripts/check-route-error-patterns.sh /Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/routes

./scripts/check-compatibility-sunset.sh

cargo test -p underlay-auth -p underlay-auth-password -p underlay-auth-jwt -p underlay-auth-totp -p underlay-auth-webauthn -p underlay-auth-oauth --all-features

# Acowtancy runtime validation
cd /Users/betterthanclay/Dev/projects/acowtancy/farmyard
bun run db:reset
API_BASE_URL=http://0.0.0.0:40001 bash scripts/validate-error-reporting.sh
curl -i http://0.0.0.0:40001/health
curl -i 'http://0.0.0.0:40001/v1/admin/assessment/sessions?include_total=true' -H 'X-Api-Version: 2026-01-01'
curl -s http://0.0.0.0:40001/openapi.json # verify include_total/includeTotal
psql "$DATABASE_URL" -c "select status_code,error_code,context from platform.error_log order by occurred_at desc limit 5;"
psql "$DATABASE_URL" -c "select id,payload from platform.job order by created_at desc limit 5;"

# Songsprout reset/validation prep
cd /Users/betterthanclay/Dev/projects/songsprout/nursery
bun run db:reset
cargo check -p nursery-api --all-features
AUTH_JWT_ISSUER=local AUTH_JWT_AUDIENCE=nursery bun run api
curl -i http://127.0.0.1:4100/health
curl -i http://127.0.0.1:4100/v1/auth/me
curl -s http://127.0.0.1:4100/openapi.json # verify include_total/includeTotal
psql "$DATABASE_URL" -c "select status_code,error_code,context from platform.error_log order by occurred_at desc limit 5;"
```

## Results

### Pass

- JSON naming guardrail passes:
  - `underlay`
  - `underlay-reference/acme-api`
  - `acowtancy/farmyard` (with explicit allowlist for `nightfire`)
  - `compli-me/api`
  - `songsprout/nursery`
- Route error pattern guardrail passes:
  - `underlay-reference/acme-api`
  - `acowtancy/farmyard`
  - `compli-me/api`
  - `songsprout/nursery`
- Compatibility sunset guardrail passes (`docs/roadmaps/supporting/016-compatibility-adapters.csv`).
- Underlay auth crates test pass completed successfully:
  - `underlay-auth` (5 tests)
  - `underlay-auth-jwt` (37 tests)
  - `underlay-auth-oauth` (10 tests)
  - `underlay-auth-password` (30 tests)
  - `underlay-auth-totp` (8 tests)
  - `underlay-auth-webauthn` (13 tests)

### Remediation Completed

1. Acowtancy route DTO drift fixed:
- removed `#[serde(rename_all = "camelCase")]` from:
  - `/Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates/api/src/routes/admin/learning/outcomes/questions.rs`

1b. Acowtancy OpenAPI query naming drift fixed:
- changed residual `includeTotal` query parameter annotations to `include_total` in:
  - `/Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates/api/src/routes/admin/assessment.rs`
  - `/Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates/api/src/routes/admin/learning/modules/core/queries.rs`
- `cargo check -p farmyard-api --all-features` passes after the update.
- OpenAPI verification now reports:
  - `include_total=3`
  - `includeTotal=0`

2. Songsprout query naming normalized:
- replaced `#[serde(rename = "includeTotal")]` with `#[serde(alias = "includeTotal")]` on `include_total` fields in:
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/handlers.rs`
- OpenAPI query parameter names updated to `include_total`.

3. Songsprout migration reset blocker fixed:
- corrected FK references from `accounts.user(id)` to `artists.artist(id)` in:
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/migrations/202602091900__add_email_totp_tables.sql`
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/migrations/202602101000__sliding_window_email_totp_rate_limits.sql`
- `bun run db:reset` now succeeds in Nursery.

4. Songsprout runtime dependency/version split fixed:
- aligned workspace dependency versions in:
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/Cargo.toml`
    - `axum: 0.7 -> 0.8`
    - `prometheus: 0.13 -> 0.14`
- removed `#[async_trait]` wrappers from `FromRequestParts` impls in:
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/auth/src/extractor.rs`
- migrated legacy axum route params (`:id`) to `{id}` in:
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/routes/auth.rs`
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/routes/artist.rs`
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/routes/billing.rs`
  - `/Users/betterthanclay/Dev/projects/songsprout/nursery/crates/api/src/routes/admin.rs`
- `cargo check -p nursery-api --all-features` now passes.

5. Underlay-reference runtime route syntax migration (axum 0.8 compatibility):
- updated route path syntax in:
  - `/Users/betterthanclay/Dev/projects/underlay-reference/acme-api/crates/api/src/routes/mod.rs`
- migrated `/:param` segments to `/{param}` to avoid startup panic.

3. Acowtancy `nightfire` explicitly allowlisted as external-contract exception:
- allowlist file added:
  - `/Users/betterthanclay/Dev/projects/underlay/scripts/json-naming-allowlist.txt`
- allowlisted path:
  - `/Users/betterthanclay/Dev/projects/acowtancy/farmyard/crates/nightfire/src/lib.rs`

## Remaining Action Required

- Sample additional API responses and persisted logs/job payloads to confirm wire-level `snake_case` in real flows.
- Validate critical `acowtancy` admin pages/integrations after DB reset.
- Remove compatibility adapters after migration cutover and close roadmap 016 phase/success gates.

## Runtime Evidence Captured

- Acowtancy error-reporting runtime validation passed end-to-end:
  - route-pattern check passed
  - forced smoke failure captured with structured context
  - sample error response:
    - `{"error":{"code":"smoke.forced_db_failure","message":"Forced failure for error-log smoke testing"}}`
  - sample error-log context row includes `handler_context.operation = "smoke.error_logging_capture"`

- Current `handler_context` null-rate in Acowtancy (24h window): `71.43%` (`5/7`), still above threshold and tracked as ongoing migration metric.
- Re-run after latest migration batch: `66.67%` (`4/6`), still above threshold and tracked as ongoing migration metric.
- Acowtancy runtime samples captured (snake_case evidence):
  - `GET /health` -> `{"status":"ok","service":"farmyard-api"}`
  - `GET /v1/admin/assessment/sessions?include_total=true` (unauthenticated) -> `{"error":{"code":"auth.unauthorized","message":"Please sign in to continue."}}`
  - `platform.error_log.context` sample:
    - `{"query":"include_total=true","source":"farmyard-api","user_agent":"curl/8.7.1","handler_context":null}`
  - `platform.job.payload` samples:
    - `{"retention_days":90}`
    - `{"type":"welcome","user_id":"..."}`
    - `{"report_types":["engagement"]}`
    - `{"batch_size":100}`

- Songsprout runtime API is now bootable locally after dependency alignment:
  - `GET /health` returns `{"status":"ok","service":"nursery-api"}`.
  - `GET /v1/auth/me` (unauthenticated) returns snake_case envelope:
    - `{"error":{"code":"auth.unauthorized","message":"Authentication required"}}`
  - OpenAPI check shows query naming is canonical:
    - `include_total=10`
    - `includeTotal=0`
  - latest `platform.error_log.context` sample uses snake_case keys:
    - `{"query":null,"source":"nursery-api","user_agent":"curl/8.7.1","handler_context":null}`

## Notes

This verification confirms core Underlay auth libraries are stable via tests and runtime blockers are unblocked.

For detailed cross-app auth integration evidence and remaining auth-specific blockers, see:
- `docs/logs/2026-02/25-000000-auth-integration-verification-sweep.md`

Additional note from auth integration sweep:
- Nursery 2FA completion (`login/finish`) is verified working when using a new TOTP window after setup/enable; same-window reuse is rejected by replay protection.
