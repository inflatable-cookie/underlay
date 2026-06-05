# g06.097 Artifact - AI Runtime Crate Modularity Audit

## Summary

`underlay-ai-runtime/src/lib.rs` is now the largest remaining Rust
warning-level production file. Retry middleware, circuit breaker middleware,
and route-chain execution already live in focused modules, but `lib.rs` still
combines public core models, error helpers, provider registry behavior, route
candidate selection, OpenAI-compatible transport wiring, private transport wire
types, HTTP status mapping, provider metadata filtering, and the stub client.

Contract `080-ai-runtime-and-suggestions` names the stable public surface:

- `LlmClient`
- `LlmRequest`
- `StructuredOutputSpec`
- `LlmResponse`
- `TokenUsage`
- `AiRuntimeError`
- `AiErrorKind`
- `ResolvedModelRoute`
- `ResolvedModelRouteCandidate`
- `ModelCapability`
- `ProviderRegistry`
- `select_route_candidates()`
- `OpenAiCompatibleClient`
- retry, circuit-breaker, and route-chain exports

## Behavior Evidence

The focused crate validation covers these stable contracts:

- route selection is deterministic for equal priorities
- route selection filters by required capabilities
- provider registry register/get behavior works
- OpenAI-compatible client construction rejects empty base URLs and API keys
- safe provider metadata retains only allowlisted keys
- HTTP statuses map to expected AI error kinds
- stub clients echo structured output
- AI runtime error helpers match default retry/fallback policies
- retry backoff is bounded and exponential
- retry middleware retries transient errors and stops on terminal errors
- circuit breaker opens, rejects while open, half-opens, and recovers
- route-chain execution falls back to later routes when allowed
- route-chain execution stops on validation errors

Validation result:

- `cargo test -p underlay-ai-runtime --all-features`
- 14 unit tests passed

## Decision

Queue `g06.098` as an AI runtime crate internal split.

The split should preserve:

- all contract-listed crate-root public exports
- retry, circuit-breaker, and route-chain re-exports
- `AiRuntimeError::is_retriable()` and `AiRuntimeError::allows_fallback()`
- `select_route_candidates()` ordering and filtering semantics
- OpenAI-compatible client validation, timeout, status mapping, response-body
  redaction, and structured-output parsing behavior
- provider metadata allowlist semantics
- stub client behavior
- private test access to `map_http_status_to_error_kind()` and
  `safe_provider_metadata()` through the crate test module, or an equivalent
  `pub(crate)` helper surface

Suggested module shape:

- `lib.rs`: crate docs, module declarations, public re-exports, and test module
- `error.rs`: `AiErrorKind`, `AiRuntimeError`, and helper methods
- `types.rs`: request, response, token usage, route, and capability public
  types
- `client.rs`: `LlmClient`
- `registry.rs`: `ProviderRegistry`
- `routing.rs`: `select_route_candidates()`
- `openai.rs`: `OpenAiCompatibleClient`, wire structs, metadata filtering, and
  HTTP status mapping
- `stub.rs`: `StubLlmClient`

## Public API Impact

Expected impact: none.

This should be an internal split. If preserving crate-root exports or
contract-listed behavior forces a public API change, stop and re-enter
planning.

## Validation

- `cargo test -p underlay-ai-runtime --all-features`

Next code batch validation:

- `cargo test -p underlay-ai-runtime --all-features`
- `effigy rust:check`
- `effigy qa:docs`
- `effigy qa:northstar`
