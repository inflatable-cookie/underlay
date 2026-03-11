# Specimen Dossier: Portkey

Status: Draft
Specimen: Portkey.ai
Owner:
Last updated: 2026-03-11
Scope: Managed LLM gateway with observability, routing, and governance

## 1) Why this specimen matters

Portkey is the leading managed LLM gateway, providing a unified interface to 200+ providers with enterprise features like cost tracking, guardrails, and observability. Unlike LiteLLM's open-source approach, Portkey is fully managed but offers similar routing capabilities.

## 2) Product and era context

- **Launched**: 2023 by Portkey.ai
- **Positioning**: "AI Gateway for production LLM apps"
- **Era**: Post-ChatGPT enterprise adoption wave (2023-2024)
- **Competition**: LiteLLM (open source), OpenRouter (routing service), Helicone (observability-focused)
- **Adoption**: Strong in enterprise/Growth-stage startups

## 3) Defining bets

1. **Managed over self-hosted** - Enterprise customers prefer SaaS for infrastructure
2. **Unified observability** - Cost, latency, quality tracking across all providers
3. **Governance layer** - Rate limits, budget controls, PII guardrails as core features
4. **Developer experience** - Drop-in SDKs, simple config, good dashboards

## 4) Standout strengths

- **Provider coverage**: 200+ providers/models via unified interface
- **Smart routing**: Load balancing, fallback chains, model upgrades
- **Cost optimization**: Automatic model downgrading, caching
- **Observability**: Detailed traces, cost attribution, quality metrics
- **Governance**: Budget limits, rate limiting, content moderation
- **Prompt management**: Versioning, A/B testing, prompt templates
- **Caching**: Semantic and exact match caching layers

## 5) Chronic weaknesses and recurring costs

- **Vendor lock-in** - Harder to migrate away than self-hosted LiteLLM
- **Pricing** - Per-request costs add up at scale
- **Latency** - Additional network hop (though they have edge deployment)
- **Customization limits** - Less flexible than self-hosted for exotic use cases
- **Data residency** - SaaS requires trust for sensitive data

## 6) Between-version corrections

- Added semantic caching after customer demand
- Introduced prompt management as standalone feature
- Expanded from pure proxy to include SDK-based routing
- Added BYOK (Bring Your Own Keys) for enterprise

## 7) Project-relevant lessons

### Adopt carefully

- **Circuit breaker pattern** - Essential for production resilience
- **Route chaining with fallback** - Automatic failover between models
- **Cost attribution per request** - Track spend by user/tenant/action
- **Dead letter queue** - Handle failures gracefully with retry

### Reject early

- **Full SaaS dependency** - Underlay's library approach is correct
- **Prompt management as core** - Keep Underlay focused on routing/runtime

### Prototype before deciding

- **Automatic fallback thresholds** - When to switch models based on latency/errors
- **Caching strategies** - Semantic vs exact match, cache hit rates

## 8) Comparison with Acowtancy's Implementation

| Feature | Portkey | Acowtancy (built on Underlay) |
|--------|---------|------------------------------|
| Circuit breaker | ✅ Built-in | ✅ Custom implementation |
| Route chaining | ✅ Config-driven | ✅ Database-driven |
| Cost tracking | ✅ Automatic | ❌ Manual (not implemented) |
| Dead letter queue | ✅ Built-in | ✅ Custom implementation |
| Retry with backoff | ✅ Automatic | ✅ Custom implementation |
| Provider registry | ✅ SaaS | ✅ Custom `ProviderRegistry` |
| Config API | ✅ REST/CLI | ❌ Database + code |
| Observability dashboard | ✅ Built-in | ❌ App must build |

**Finding**: Acowtancy has built production-grade orchestration (circuit breaker, retry, dead letter) that could potentially be extracted to Underlay.

## 9) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| portkey.ai/docs | Official docs | 2024-2025 | High | Comprehensive feature docs |
| Portkey blog | Company | 2024 | Medium | Architecture decisions |
| HN "portkey" search | Community | 2023-2025 | Medium | Real-world usage |

## 10) Open questions

- How does Portkey handle streaming with fallbacks?
- What are their semantic caching hit rates in practice?
- How do they manage provider API version drift?

## Next Task

Create value track synthesis comparing LiteLLM, Portkey, and Acowtancy's custom implementation to identify gaps in Underlay's `underlay-ai-runtime`.
