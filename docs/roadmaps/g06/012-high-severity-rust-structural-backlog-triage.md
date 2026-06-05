# g06.012 - High-Severity Rust Structural Backlog Triage

## Why

`g06.011` removed the remaining critical Rust god-file findings.

The scanner now reports high and warning findings only. The next step is to
decide which high findings still belong in the Rust platform-contract
transition and which should remain visible backlog.

## Goal

Classify the remaining high-severity Rust structural findings and decide
whether `g06` should continue with another scoped repair batch or close this
structural lane.

## Scope

In scope:

- inspect high-severity Rust god-file findings from `effigy scan god-files`
- classify each finding as current-lane, deferred backlog, or unrelated
  historical/test mass
- decide whether to create one more scoped repair card
- keep TS findings out of the Rust platform-contract lane unless they block
  release readiness

Out of scope:

- fixing every high-severity file in one batch
- TS god-file cleanup
- changing public Rust APIs without a new compatibility card
- release execution or publishing

## Contract References

- `001`: working rules
- `023`: release and compatibility rollout
- `040`: storage, blob, and media systems
- `120`: tooling, testing, and contract artifacts
- `122`: Rust public API inventory

## Acceptance Criteria

- high-severity Rust findings are classified
- next repair card or lane closeout is explicit
- validation state from `g06.011` remains recorded
- remaining scanner backlog stays visible

## Current Scan

`effigy scan god-files` after `g06.011` reports:

- critical: 0
- high: 19
- warning: 43

High Rust findings:

| File | Classification | Decision |
| --- | --- | --- |
| `underlay-migration-core/src/pipeline.rs` | deferred current-generation backlog | Already downgraded from critical. Further split should wait until the decision stage can be separated without changing orchestration semantics. |
| `underlay-auth-jwt/src/tests/service_tests.rs` | deferred test mass | Large test file, but already test-only. Split when auth-jwt tests are reopened. |
| `underlay-media/src/tests/nightfire_tests.rs` | deferred test mass | Large test file, but `g06.011` already split production Nightfire internals. |
| `underlay-auth-webauthn/src/lib.rs` | current-lane | Security-adjacent production implementation. Good next split candidate. |
| `underlay-jobs/src/types.rs` | deferred contract model mass | Mostly shared model definitions. Splitting should follow jobs contract work, not scanner pressure alone. |
| `underlay-blob/src/adapters/s3.rs` | current-lane | Security/storage-adjacent adapter implementation. Good next split candidate. |
| `underlay-media/src/domain.rs` | deferred contract model mass | Large model file. Split only with a domain-boundary contract update. |
| `underlay-http/src/error_logging.rs` | current-lane candidate | Operational/error surface. Useful after security/storage adapter split. |
| `underlay-devtools/src/migration_bundle.rs` | deferred follow-up | Already partially split in `g06.007`; remaining high finding can wait behind security/storage code. |
| `underlay-migration-core/src/verification_rules.rs` | deferred tooling follow-up | Useful split, but less security-sensitive than auth/blob adapters. |
| `underlay-jobs/src/postgres.rs` | deferred adapter follow-up | Adapter split candidate after jobs contract/model review. |
| `underlay-auth-email-totp/src/tests/service_tests.rs` | deferred test mass | Test-only. |
| `underlay-auth-oauth/src/tests/lib_tests.rs` | deferred test mass | Test-only. |
| `underlay-auth-password/src/tests/service_tests.rs` | deferred test mass | Test-only. |

High TS findings remain out of scope for `g06`.

## Decision

Continue with one more scoped Rust repair batch focused on security/storage
adapter files:

- `rust/crates/underlay-auth-webauthn/src/lib.rs`
- `rust/crates/underlay-blob/src/adapters/s3.rs`

This keeps the lane tied to the original audit themes: security-sensitive
code, modularity, and extension seams for consuming apps.

## Current State

`g06.012` is complete.

## Next Task

Execute `g06.013`: security-adjacent Rust adapter split batch.
