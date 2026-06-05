# g06.087 Artifact - AI Runtime Tests Modularity Audit

## Summary

`underlay-ai-runtime/src/tests/lib_tests.rs` is the largest remaining Rust
warning-level test file after the decision-memory split. It is test-only, but
it covers route selection, provider registry behavior, OpenAI-compatible client
validation, provider metadata filtering, HTTP error mapping, stub client output,
retry/circuit middleware, and route-chain fallback behavior in one file.

The file currently groups:

- shared route fixture helper
- route selection tests
- provider registry and OpenAI-compatible client construction tests
- provider metadata and HTTP status mapping tests
- stub client structured-output test
- scripted client fixture and request/route/response helpers
- AI runtime error and retry config tests
- retry middleware tests
- circuit breaker middleware test
- route-chain fallback tests

## Behavior Evidence

The test file covers these stable contracts:

- route selection is deterministic for equal priorities
- route selection filters by required capabilities
- provider registry register/get behavior works
- OpenAI-compatible client construction rejects empty base URLs and API keys
- safe provider metadata retains only whitelisted keys
- HTTP statuses map to expected AI error kinds
- stub clients echo structured output
- AI runtime error helpers match default retry/fallback policies
- retry backoff is bounded and exponential
- retry middleware retries transient errors and stops on terminal errors
- circuit breaker opens, rejects while open, half-opens, and recovers
- route-chain execution falls back to a later route when allowed
- route-chain execution stops on validation errors

## Decision

Queue `g06.088` as an AI runtime tests internal split.

The split should preserve:

- all test names or comparably searchable behavior names
- scripted client fixture behavior
- request/route/response helper behavior
- route selection and provider registry coverage
- metadata/status mapping coverage
- retry and circuit-breaker middleware coverage
- route-chain fallback coverage
- existing production code and public APIs

Suggested test module shape:

- `lib_tests.rs`: test module front door
- `lib_tests/support.rs`: route helper, scripted client, sample request/route,
  response/error helpers
- `lib_tests/routes.rs`
- `lib_tests/providers.rs`
- `lib_tests/mapping.rs`
- `lib_tests/clients.rs`
- `lib_tests/retry.rs`
- `lib_tests/circuit_breaker.rs`
- `lib_tests/route_chain.rs`

## Public API Impact

Expected impact: none.

This should be a test-only split. If production AI runtime APIs or behavior
must change, stop and re-enter planning.

## Validation

- `cargo test -p underlay-ai-runtime --all-features`

Next code batch validation:

- `cargo test -p underlay-ai-runtime --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
