# Contract: AI Runtime and Suggestions

Status: active
Owner: repo maintainers
Depends on: `020-http-transport-and-server-boundary.md`, `060-jobs-events-and-operator-systems.md`

## Purpose

Define the shared AI and suggestion contract Underlay owns across:

- the provider-agnostic Rust LLM runtime boundary
- retry, circuit-breaker, route-selection, and fallback execution helpers
- the OpenAI-compatible HTTP transport client
- generic suggestion request parsing and prioritized query-building utilities
- the thin TS helper layer for suggestion request params

This contract does not define app-local prompt design, model policy, routing
config storage, spend governance, AI workflow UX, or relation-selector product
behavior. Those remain app-owned.

## Sources of Truth

Primary:

- [`rust/crates/underlay-ai-runtime/src/lib.rs`](../../rust/crates/underlay-ai-runtime/src/lib.rs)
- [`rust/crates/underlay-ai-runtime/src/chain.rs`](../../rust/crates/underlay-ai-runtime/src/chain.rs)
- [`rust/crates/underlay-ai-runtime/src/retry.rs`](../../rust/crates/underlay-ai-runtime/src/retry.rs)
- [`rust/crates/underlay-ai-runtime/src/circuit_breaker.rs`](../../rust/crates/underlay-ai-runtime/src/circuit_breaker.rs)
- [`rust/crates/underlay-suggestions/src/lib.rs`](../../rust/crates/underlay-suggestions/src/lib.rs)
- [`rust/crates/underlay-suggestions/src/params.rs`](../../rust/crates/underlay-suggestions/src/params.rs)
- [`rust/crates/underlay-suggestions/src/query.rs`](../../rust/crates/underlay-suggestions/src/query.rs)
- [`ts/src/client/suggestions.ts`](../../ts/src/client/suggestions.ts)
- [`ts/src/patterns/selection-history.ts`](../../ts/src/patterns/selection-history.ts)
- [`ts/src/runtime/ai.ts`](../../ts/src/runtime/ai.ts)
- [`ts/src/runtime/data.ts`](../../ts/src/runtime/data.ts)

Supporting:

- [`docs/guides/176-ai-runtime-routing.md`](../guides/176-ai-runtime-routing.md)
- [`docs/guides/092-selection-suggestions.md`](../guides/092-selection-suggestions.md)
- [`docs/architecture/system-inventory.md`](../architecture/system-inventory.md)

If these diverge, the shared code wins.

## Contract Goal

Underlay should provide one reusable lower AI/runtime layer with clear seams:

- apps can execute structured-output LLM calls through provider-agnostic Rust
  contracts
- fallback and resilience mechanics are shared and deterministic
- provider credentials and routing policy stay server-side
- relation-suggestion requests use one shared query-param and server-query
  vocabulary

The goal is shared infrastructure and request mechanics, not a complete AI
platform.

## Shared Boundary

### Rust AI runtime core

`underlay-ai-runtime` owns the generic LLM execution contract.

Core pieces:

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

Rules:

- the shared request model is structured-output-first
- calls resolve against explicit provider/model routes, not hidden global state
- provider clients implement `LlmClient` and stay behind the trait boundary
- apps own route discovery, alias mapping, config loading, and secret loading
- the shared runtime returns token usage and finish metadata when available but
  does not own cumulative spend policy

### Route candidate selection

Underlay owns the generic route filtering and deterministic ordering seam.

Core pieces:

- `select_route_candidates()`

Rules:

- candidate filtering is capability-based
- tie-breaking is deterministic by `priority`, then provider name, then model
  name
- route selection is a shared helper over app-produced candidates, not a DB
  policy engine

### OpenAI-compatible transport client

Underlay retains one generic HTTP client for OpenAI-style providers and
routers.

Core pieces:

- `OpenAiCompatibleClient`

Rules:

- transport targets `/chat/completions`
- auth is bearer-token based
- request mode assumes structured JSON output
- timeout and HTTP-status mapping are owned by the shared client
- provider metadata passthrough is allowlisted and sanitized
- provider response bodies are redacted by default in surfaced errors

### Resilience and fallback middleware

Underlay owns additive runtime resilience helpers.

Core pieces:

- `RetryConfig`
- `RetryMiddleware`
- `default_retriable_error_kinds()`
- `CircuitBreakerConfig`
- `CircuitBreakerMiddleware`
- `CircuitState`
- `RouteChainConfig`
- `RouteChainExecutor`
- `RouteChainAttempt`
- `RouteChainResult`
- `RouteChainFailure`
- `default_fallback_error_kinds()`

Rules:

- retry is opt-in middleware around a `LlmClient`
- default retriable kinds are `RateLimit`, `Timeout`, `Provider`, and
  `Unknown`
- circuit-breaker state is process-local and provider-keyed
- route fallback consumes an already ordered route chain; it does not discover
  routes itself
- validation failures stop retry/fallback when they indicate a bad request or
  malformed provider payload contract
- fallback history is part of the shared result/failure model for diagnostics

### Suggestion request contract

`underlay-suggestions` owns the server-side suggestion-query vocabulary.

Core pieces:

- `SuggestionParams`
- `SuggestionQuery`
- `SuggestionOrder`

Rules:

- `suggestions=true` signals a suggestion request instead of a full search/list
- `recentHints` is a comma-separated ordered hint list from the client
- server-side parsing trims whitespace and drops empty hint entries
- hint order is preserved as signal, not treated as an unordered set
- shared suggestion utilities build query inputs and ordering fragments; they
  do not generate full repository SQL or enforce app filtering rules

### TS suggestion helper shell

The only retained TS surface in this system family today is a thin helper layer
for suggestion params.

Core pieces:

- `SuggestionRequestOptions`
- `formatHintsParam()`
- `parseHintsParam()`
- `buildSuggestionParams()`
- `appendSuggestionParams()`

Rules:

- TS helpers must use the same `suggestions` and `recentHints` vocabulary as
  the Rust parser
- query merge behavior must replace existing keys rather than append duplicate
  keys
- the helper is request-shape glue only; it does not own selection-history
  persistence or relation-selector UX

## Ownership Split

Underlay owns:

- provider-agnostic Rust request/response/error contracts
- shared OpenAI-compatible transport behavior
- retry, circuit breaker, and route-chain helpers
- generic suggestion param parsing and prioritized query-building
- thin TS helper functions for the shared suggestion query vocabulary

Apps own:

- prompt design and schema content
- route catalogs, alias policy, and rollout controls
- provider credentials and infra config
- diagnostics endpoints, spend governance, and dead-letter policy
- relation-selector repository queries and filter rules
- AI feature UX and admin workflows

## Invariants

- AI provider keys and secrets stay server-side
- shared runtime contracts remain provider-agnostic above the transport client
- route ordering and fallback behavior must be deterministic
- retry and fallback decisions are based on explicit `AiErrorKind` families
- suggestion request vocabulary is stable: `suggestions` plus `recentHints`
- hint order is significant and must survive round-trips
- duplicate query keys are not allowed when merging suggestion params into an
  existing URL

## Known Drift To Assess Later

- `ts/src/runtime/ai.ts` exists now, but only as a thin compatibility barrel
  over the AI routing ops controller rather than as a broader TS AI runtime
- `underlay-suggestions` is tightly shaped around relation-selector style
  callers and may be too query-fragment oriented to count as a broader generic
  suggestion contract
- `underlay-ai-runtime` is intentionally narrow and does not yet cover spend
  accounting, dead-letter persistence, streaming, or richer provider-specific
  features
- `OpenAiCompatibleClient` is the only concrete transport implementation, so
  the provider-agnostic story above the trait is stronger than the actual
  transport coverage below it

## Assessment Questions

- is `underlay-suggestions` the right shared abstraction level, or is it too
  tied to one relation-selector usage pattern
- does `underlay-ai-runtime` still fulfil the intended “shared lower runtime”
  goal, or is it too narrow to justify a distinct retained system family
- which missing features, if any, belong in Underlay rather than staying
  app-local: spend tracking, dead-letter hooks, streaming semantics, richer
  provider coverage

## Next Task

Use [../roadmaps/g04/026-ts-ai-and-suggestion-authority-repair.md](../roadmaps/g04/026-ts-ai-and-suggestion-authority-repair.md)
to execute the current repair lane.
