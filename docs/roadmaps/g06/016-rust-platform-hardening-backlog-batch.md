# g06.016 - Rust Platform Hardening Backlog Batch

## Why

The post-commit Rust audit shows the platform-contract transition is sound, but
the repo still has a small hardening backlog that should be handled before
calling the Rust lane settled.

This is a bounded hardening batch, not another broad refactor.

## Goal

Close the most actionable hardening gaps left after `g06.015`.

## Scope

In scope:

- add a repo-owned `cargo-deny` policy or equivalent Effigy-backed supply-chain
  check
- classify and fix the yanked `wasm-bindgen 0.2.111` lockfile path
- normalize `underlay-http::error_logging` dynamic filter SQL onto a safer
  query-builder shape
- classify remaining production `unwrap` / `expect` paths as intentional,
  test-only, or follow-up
- preserve current consumer compatibility

Out of scope:

- release execution or publishing
- broad god-file cleanup
- TS validation drift cleanup
- changing public Rust APIs beyond additive helpers or internal refactors

## Contract References

- `001`: working rules
- `020`: HTTP transport and server boundary
- `023`: release and compatibility rollout
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- supply-chain check has repo-owned policy instead of default-policy noise
- yanked dependency warning is resolved or explicitly pinned with rationale
- `error_logging` filter SQL no longer hand-counts bind parameter positions
- touched Rust validation passes
- docs QA passes
- no named consumer app update is required

## Current State

`g06.016` is complete.

Completed work:

- Added a repo-owned `cargo-deny` policy for advisories, licenses, sources, and
  explicit bans posture.
- Resolved the yanked `wasm-bindgen 0.2.111` lockfile path by updating the
  wasm-bindgen family.
- Updated vulnerable transitive dependency paths where safe patch releases
  exist, including AWS-LC, `lettre`, `rustls-webpki`, and `astral-tokio-tar`.
- Removed the AWS SDK legacy Rustls feature edge from the optional S3 and SES
  adapters by disabling AWS SDK defaults and selecting the modern HTTPS client
  features explicitly.
- Kept `RUSTSEC-2023-0071` explicit in `deny.toml` because `jsonwebtoken
  10.3.0` still pulls `rsa 0.9.10` and upstream has no patched release.
- Reworked `underlay-http::error_logging` list/count filters onto
  `sqlx::QueryBuilder`, removing manual bind-parameter counting.
- Replaced production-facing regex/dev-capture `unwrap` calls with explicit
  invariant messages.

Remaining `unwrap` / `expect` classification:

- Test-only and integration helper paths are intentional.
- `underlay-testing` fail-fast helpers are intentional test harness behavior.
- Mutex poison paths in `underlay-http::caching` and
  `underlay-ai-runtime::circuit_breaker` remain intentional fail-fast
  invariants.
- `underlay-auth-totp` HMAC construction remains an accepted cryptographic API
  invariant: HMAC accepts arbitrary key sizes.
- Module-local `#[cfg(test)]` blocks in media and HTTP remain test-only.

Consumer impact:

- No public Rust API changed.
- No named consumer app update is required.
- Optional AWS adapter builds now avoid the AWS SDK legacy Rustls feature path,
  but the adapter features and public constructors remain unchanged.

## Next Task

Execute `g06.017`: Rust quality re-audit and fresh-start assessment.
