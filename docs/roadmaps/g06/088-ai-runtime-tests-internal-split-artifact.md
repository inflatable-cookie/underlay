# g06.088 Artifact - AI Runtime Tests Internal Split

## Summary

`underlay-ai-runtime/src/tests/lib_tests.rs` is now a small test front door.
The previous test monolith was split into focused modules under
`underlay-ai-runtime/src/tests/lib_tests/`.

The split is test-only. It does not change AI runtime production code,
provider/request/response semantics, public APIs, or consumer apps.

## Module Shape

- `lib_tests.rs`: explicit test module front door
- `lib_tests/support.rs`: route helper, scripted client, sample request/route,
  response helper, and error helper
- `lib_tests/routes.rs`: route selection tests
- `lib_tests/providers.rs`: provider registry and OpenAI-compatible client
  construction tests
- `lib_tests/mapping.rs`: safe metadata and HTTP status mapping tests
- `lib_tests/clients.rs`: stub client structured-output test
- `lib_tests/retry.rs`: error helper, retry config, and retry middleware tests
- `lib_tests/circuit_breaker.rs`: circuit breaker state transition test
- `lib_tests/route_chain.rs`: route-chain fallback and terminal-error tests

## Behavior Preserved

- all 14 AI runtime crate tests pass
- route selection deterministic ordering and capability filtering are unchanged
- provider registry behavior and client input validation remain covered
- safe provider metadata filtering and HTTP status mapping remain covered
- stub client structured-output behavior remains covered
- retry backoff and retry middleware behavior remain covered
- circuit breaker open/half-open/recovery behavior remains covered
- route-chain fallback and terminal validation-error behavior remain covered

## Public API Impact

None.

This was a Rust test-structure split only.

## Validation

- `cargo test -p underlay-ai-runtime --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
- `effigy doctor` remains expected-fail on the known backlog; `scan.god-files`
  improved from 45 findings / 40 warnings / 5 errors to 44 findings / 39
  warnings / 5 errors.
