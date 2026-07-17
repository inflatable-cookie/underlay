# 2026-07-17 - g08 Batch 3 (Lane A close): production infra + auth hardening

## Context

Final Lane A batch (`g08.008`-`g08.010`). Closes the production-readiness
infra gaps and the remaining medium-severity auth-edge findings, completing
the security lane.

## Changes

### g08.008 - Distributed rate-limit backend

- `underlay-ratelimit` gains `PostgresBackend` (feature `postgres`):
  fixed-window counters via one atomic `INSERT ... ON CONFLICT` upsert so
  replicas share a window. Migration `0001__rate_limit_counters.sql` ships
  with the crate.
- `InMemoryBackend` documented single-instance/non-prod; added a greppable
  `single_instance()` constructor.
- Guide `068` reconciled - the advertised `RedisBackend` never existed;
  documented as "implement the trait for Redis if preferred". Chose Postgres
  so app-DB consumers add no new dependency.

### g08.009 - HTTP-client SSRF and timeout defaults

- Default connect (10s) + total (30s) timeouts on every `HttpClient` profile.
- New `HttpClient::external()` + `validate_external_url`: reject
  private/loopback/link-local/unspecified hosts (incl. `169.254.169.254`),
  non-http(s) schemes; cap and re-check redirects (3 hops). IPv4-mapped IPv6
  de-mapped before classification.
- Fixed timeout-less reqwest users (`underlay-auth-oauth`,
  `underlay-devtools`).
- Embed proxy `id` validated to `[A-Za-z0-9_-]+` before URL interpolation.
- Contract `020` records the profiles and proxy-input rule.

### g08.010 - Auth hardening batch

- **Refresh replay**: reuse of a superseded refresh token revokes the whole
  session family; the lost-CAS concurrent-refresh race does not (stop
  condition honored).
- **2FA throttle**: `verify_second_factor_throttled` caps per-user attempts
  against a `RateLimitBackend` (increment-on-fail, reset-on-success).
- **Login timing**: unknown-email / no-credential paths run one KDF pass to
  remove the account-existence oracle.
- **`plain:` cipher**: rejected by default; readable only via explicit
  `with_plain_migration(true)`.
- Deferred: password-reset-initiation throttle - no in-tree initiation
  endpoint to wire; the `L` bound stays available for the consumer flow.
- Contract `030` updated.

## Validation

- `cargo test --workspace --all-features`: green (73 suites, 0 failures).
- `cargo check --workspace` (default features): clean.
- `cargo test -p underlay-ratelimit --features postgres`, `-p
  underlay-http-client -p underlay-http`, and the four auth crates: green.
- `effigy validate`: green (unit 735 + component 31).

## Consumer Upgrade Notes

- `configuration` (`g08.008`): multi-instance deployments must configure
  `PostgresBackend` (or another shared backend); `InMemoryBackend` is
  single-instance only.
- `behavioral` (`g08.009`, `g08.010`): outbound requests gain timeouts;
  `external()` callers must opt out for internal targets; refresh replay now
  forces family re-auth; 2FA guessing is throttled where the throttled
  entrypoint is adopted; `plain:` oauth secrets are rejected without the
  migration flag.
- All require six-consumer proof per `023` before release.

## Current State

Lane A (`g08.001`-`g08.010`) and Lane B (`g08.011`-`g08.014`) complete. g08
acceptance criteria 1-4 ticked. Structural/process lanes C-E remain.

## Next

Post-Batch-3 planning checkpoint (per the generation runway): is the security
posture consumer-communicable, and does any finding warrant a new contract
rather than a card? Then Lane C `g08.015` Rust error taxonomy.
