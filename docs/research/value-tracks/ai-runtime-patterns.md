# Value Track: AI Runtime Patterns

Status: Draft
Track: AI-VT-001
Owner:
Last updated: 2026-03-11
Primary project tags: ai-runtime, llm, routing, resilience

## 1) Problem statement

Underlay provides `underlay-ai-runtime` with basic primitives (`LlmClient` trait, `OpenAiCompatibleClient`, `ProviderRegistry`), but consuming apps are building significant production-grade orchestration on top:

- Circuit breaker patterns for failure handling
- Route chaining with automatic fallback
- Retry logic with exponential backoff
- Cost tracking and attribution
- Dead letter queues for failed requests

Research shows this is repeated work across AI-enabled applications.

## 2) Why this track matters

**For Underlay:**
- `underlay-ai-runtime` is currently minimal compared to market solutions
- Acowtancy has built sophisticated orchestration that could be generalized
- AI is becoming core infrastructure, needs robust runtime

**For consuming apps:**
- Every app needs circuit breakers, retries, fallbacks
- Rebuilding this is error-prone and time-consuming
- Inconsistent implementations across apps

## 3) Cross-specimen comparison

| Specimen | Approach | Strengths | Failure modes | Project signal |
| --- | --- | --- | --- | --- |
| **LiteLLM** | Open-source proxy | 100+ providers, community-driven | Python dependency, proxy latency | **Strong**: Feature breadth reference |
| **Portkey** | Managed gateway | Zero ops, rich observability | Vendor lock-in, cost at scale | **Medium**: SaaS features worth copying |
| **Acowtancy** | Custom on Underlay | Full control, DB-driven config | Maintenance burden, initial build cost | **Strong**: Internal validation of needs |

## 4) Repeated patterns

### Pattern 1: Circuit Breaker

**Finding**: Production AI systems need circuit breakers to prevent cascade failures.

**Evidence**:
- Portkey: Built-in circuit breaker per provider
- Acowtancy: Custom implementation (10 failures / 15 min threshold)
- Industry: Standard microservices pattern

**Implementation**:
```rust
const CIRCUIT_BREAKER_WINDOW_MINUTES: i32 = 15;
const CIRCUIT_BREAKER_FAILURE_THRESHOLD: i64 = 10;
```

### Pattern 2: Route Chaining with Fallback

**Finding**: Automatic failover between models/providers is essential.

**Evidence**:
- Portkey: Configurable fallback chains
- LiteLLM: Automatic fallback on error
- Acowtancy: Database-driven route chain with fallback

**Implementation approaches**:
1. Config-driven (Portkey, LiteLLM)
2. Database-driven (Acowtancy)
3. Code-driven (Underlay's current `ProviderRegistry`)

### Pattern 3: Retry with Exponential Backoff

**Finding**: Transient failures (rate limits, timeouts) need automatic retry.

**Evidence**:
- Portkey: Automatic retry with backoff
- Acowtancy: Manual retry loop with `is_retriable_error_kind()`
- LiteLLM: Configurable retry policies

**Retriable errors**:
- RateLimit
- Timeout
- Provider (5xx)

### Pattern 4: Cost Tracking per Request

**Finding**: Token usage must be tracked for attribution and budgeting.

**Evidence**:
- Portkey: Automatic cost tracking per request
- LiteLLM: Spend tracking with budget alerts
- Acowtancy: Stores `usage` in database, no budget enforcement

**Gap**: Underlay returns `TokenUsage` but doesn't track cumulative spend.

### Pattern 5: Dead Letter Queue

**Finding**: Failed requests need persistence for later analysis/retry.

**Evidence**:
- Portkey: Built-in failure tracking
- Acowtancy: Custom `insert_ai_action_dead_letter()` function
- Enterprise pattern: Essential for audit/debugging

## 5) Frontier signals

- **Streaming with fallbacks**: How to switch models mid-stream?
- **Semantic caching**: Reduce costs by caching similar prompts
- **Multi-modal routing**: Images, audio, video routing strategies
- **Fine-tuned model selection**: Automatic routing to fine-tuned variants

## 6) Project implications

### Recommended direction

**Tier 1: Add to `underlay-ai-runtime`** (high priority)

1. **Circuit breaker middleware** - Per-provider failure tracking
2. **Retry middleware** - Exponential backoff for retriable errors
3. **Route chain execution** - Automatic fallback through provider chain

**Tier 2: Consider for extraction** (medium priority)

4. **Cost tracking** - Cumulative spend tracking with budget alerts
5. **Dead letter queue traits** - Standard interface for failure persistence

**Tier 3: App-specific** (low priority)

6. **Prompt management** - Versioning, A/B testing (keep app-specific)
7. **Semantic caching** - Complex, may vary by use case

### Risks to avoid

- **Over-abstraction**: Not all apps need all features
- **Feature bloat**: Keep core runtime focused
- **Provider lock-in**: Stay OpenAI-compatible

### Evidence or prototype needed

**Prototype P-AI-001**: Circuit breaker + retry middleware
- Test with simulated provider failures
- Validate backoff timing with real rate limits
- Measure performance overhead

## 7) Source inventory

| Source | Type | Confidence | Notes |
| --- | --- | --- | --- |
| Acowtancy implementation | Production code | High | Real-world validation |
| Portkey docs | Product | High | Feature reference |
| LiteLLM docs | OSS | High | Implementation patterns |

## 8) Decision state

- `continue research` → Need P-AI-001 prototype validation
- `promote to architecture work` → After prototype validates approach

## Next Task

Draft translation memo recommending specific additions to `underlay-ai-runtime`:
1. Circuit breaker middleware trait
2. Retry middleware with exponential backoff
3. Route chain executor

## Related

- `specimen-dossiers/litellm.md` - OSS gateway patterns
- `specimen-dossiers/portkey.md` - Managed gateway patterns
- `specimen-dossiers/acme-reference-implementation.md` - Underlay usage baseline
