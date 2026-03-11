# Implementation Decision Record: AI Runtime Resilience Middleware

## Feature

Name: AI Runtime Circuit Breaker and Retry Middleware
Author: Research Thread
Date: 2026-03-11
Status: `proposed`

## Summary

Add production-grade resilience patterns to `underlay-ai-runtime`: circuit breakers, retry with exponential backoff, and route chain execution. Based on analysis of Acowtancy's custom implementation and comparison with LiteLLM/Portkey patterns.

## Research Discovery

### Architecture Target

- Primary doc: `rust/crates/underlay-ai-runtime/src/lib.rs`
- Related docs: `docs/guides/176-ai-runtime-routing.md`

### Research Consulted

| Type | Document | Key finding | Relevance |
| --- | --- | --- | --- |
| Specimen Dossier | `specimen-dossiers/portkey.md` | Managed gateway with built-in circuit breakers | Feature reference |
| Specimen Dossier | `specimen-dossiers/litellm.md` | OSS proxy with retry and fallback | Implementation patterns |
| Value Track | `value-tracks/ai-runtime-patterns.md` | 5 repeated patterns across specimens | Validates abstraction need |
| Production Code | Acowtancy `ai_actions.rs` | Custom circuit breaker (10 fails/15 min), retry, dead letter | Internal validation |

### Prototypes or Validation Work

| Item | Status | Finding | Impact |
| --- | --- | --- | --- |
| Acowtancy code analysis | `complete` | 200+ lines of custom orchestration | Significant reinvention |
| LiteLLM comparison | `complete` | Similar patterns in proxy layer | Pattern validated |
| Portkey comparison | `complete` | Circuit breaker, retry, route chain as core features | Market expectation |

## Decisions

### Decision 1: Add Circuit Breaker Middleware

**Decision:** Create `CircuitBreakerMiddleware` that wraps `LlmClient` and tracks failures per provider.

**Research basis:**
- Acowtancy implements custom circuit breaker (lines 174-224 in `ai_actions.rs`)
- Portkey provides this as built-in feature
- Standard microservices pattern for resilience

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| App-level implementation (status quo) | Reproduces Acowtancy's 50+ lines of custom code per app |
| External service (like Portkey) | Adds infrastructure dependency, latency |
| No circuit breaker | Unacceptable for production reliability |

**Confidence:** `high`

**Risks**
- State storage requires decision (in-memory vs pluggable)
- Configuration thresholds vary by use case

**Proposed API**

```rust
pub struct CircuitBreakerConfig {
    pub failure_threshold: u32,
    pub window_duration: Duration,
    pub reset_timeout: Duration,
}

pub struct CircuitBreakerMiddleware<C> {
    inner: C,
    config: CircuitBreakerConfig,
    state: Arc<RwLock<HashMap<String, CircuitState>>>,
}

#[async_trait]
impl<C: LlmClient> LlmClient for CircuitBreakerMiddleware<C> {
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        // Check circuit state
        // Call inner if closed
        // Track failure/success
    }
}
```

### Decision 2: Add Retry Middleware with Exponential Backoff

**Decision:** Create `RetryMiddleware` that automatically retries retriable errors.

**Research basis:**
- Acowtancy implements custom retry logic with `is_retriable_error_kind()`
- LiteLLM provides configurable retry policies
- Portkey handles retry automatically
- Transient failures (rate limits, timeouts) are common

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| No retry | Leaves apps to handle all transient failures |
| Infinite retry | Could cause runaway resource consumption |
| Fixed delay | Exponential backoff is standard best practice |

**Confidence:** `high`

**Risks**
- Retry storms if many requests fail simultaneously
- Idempotency concerns (though LLM calls are typically idempotent)

**Proposed API**

```rust
pub struct RetryConfig {
    pub max_attempts: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub retriable_errors: Vec<AiErrorKind>,
}

pub struct RetryMiddleware<C> {
    inner: C,
    config: RetryConfig,
}

#[async_trait]
impl<C: LlmClient> LlmClient for RetryMiddleware<C> {
    async fn generate_structured(
        &self,
        route: &ResolvedModelRoute,
        request: &LlmRequest,
    ) -> Result<LlmResponse, AiRuntimeError> {
        // Attempt with retry loop
        // Exponential backoff between attempts
        // Return last error if all fail
    }
}
```

### Decision 3: Add Route Chain Executor

**Decision:** Create `RouteChainExecutor` that attempts multiple routes in sequence.

**Research basis:**
- Acowtancy implements custom route chain logic (lines 142-153, 287-300)
- Portkey provides automatic fallback chains
- LiteLLM supports model fallback
- Essential for high availability

**Alternatives considered**

| Alternative | Why rejected |
| --- | --- |
| Single route only | No fallback on provider failure |
| App-level chaining | Reproduces Acowtancy's complex loop per app |
| Random selection | Deterministic fallback is more predictable |

**Confidence:** `medium` (needs validation for streaming scenarios)

**Risks**
- Streaming responses complicate fallback
- Different models may have incompatible outputs

**Proposed API**

```rust
pub struct RouteChainExecutor {
    clients: ProviderRegistry,
    chain: Vec<ResolvedModelRoute>,
}

impl RouteChainExecutor {
    pub async fn execute_with_fallback(
        &self,
        request: &LlmRequest,
    ) -> Result<(LlmResponse, ResolvedModelRoute, usize), AiRuntimeError> {
        // Try each route in chain
        // Return first success with route metadata
        // Return last error if all fail
    }
}
```

### Decision 4: DO NOT Add Cost Tracking (Yet)

**Decision:** Defer cumulative cost tracking to app layer for now.

**Research basis:**
- Portkey and LiteLLM both provide cost tracking
- Acowtancy stores per-request usage but not cumulative budgets
- Different apps have different attribution needs (per user, per tenant, per action)

**Rationale:**
- Underlay already returns `TokenUsage` per response
- Cumulative tracking requires storage decisions best left to apps
- Budget enforcement policies vary significantly

**Future consideration:** Provide traits for cost tracking hooks.

## Deviations From Research

| Research recommendation | Our approach | Justification |
| --- | --- | --- |
| Include dead letter queue | Defer to app layer | Storage choices too varied |
| Include cost tracking | Return per-request only | Cumulative tracking app-specific |
| Semantic caching | Not included | Too complex for initial scope |

## Implementation Notes

### Key locations

- New file: `rust/crates/underlay-ai-runtime/src/circuit_breaker.rs`
- New file: `rust/crates/underlay-ai-runtime/src/retry.rs`
- New file: `rust/crates/underlay-ai-runtime/src/chain.rs`
- Update: `rust/crates/underlay-ai-runtime/src/lib.rs` (exports)

### Middleware composition

```rust
// Example: Composed client with all resilience features
let client = CircuitBreakerMiddleware::new(
    RetryMiddleware::new(
        OpenAiCompatibleClient::new(base_url, api_key)?,
        RetryConfig::default(),
    ),
    CircuitBreakerConfig::default(),
);
```

### Research references in code

```rust
// Research: value-tracks/ai-runtime-patterns.md
// Based on: specimen-dossiers/portkey.md, specimen-dossiers/litellm.md
// Decision: IDR-AI-001
```

## Research Gaps Found

| Gap | Impact | Action |
| --- | --- | --- |
| Streaming fallback behavior | Medium | Prototype must test with streaming responses |
| Circuit breaker state persistence | Low | Start with in-memory, add pluggable storage if needed |

## Validation

- [ ] Middleware tested in Acowtancy with simulated failures
- [ ] Retry backoff timing validated with real rate limits
- [ ] Circuit breaker thresholds configurable per provider
- [ ] Documentation includes composition examples

## Related Documents

- Architecture: `docs/architecture/000-overview.md` (crate structure)
- Guide: `docs/guides/176-ai-runtime-routing.md`
- Value track: `docs/research/value-tracks/ai-runtime-patterns.md`
- Dossier: `docs/research/specimen-dossiers/portkey.md`
- Dossier: `docs/research/specimen-dossiers/litellm.md`

## Next Task

Create implementation roadmap:
1. Circuit breaker middleware (2-3 days)
2. Retry middleware (1-2 days)
3. Route chain executor (2-3 days)
4. Integration tests with mock providers (2 days)
5. Documentation and examples (1-2 days)

## Handoff Notes for Implementation Thread

**Priority:** High
**Estimated effort:** 8-12 days
**Dependencies:** None (extends existing crate)
**Breaking changes:** None (additive)
**Test strategy:** Unit tests + integration with Acowtancy for validation

**Success criteria:**
- Acowtancy can remove custom circuit breaker implementation
- Retry logic handles rate limits gracefully
- Route chain falls back on provider failure
