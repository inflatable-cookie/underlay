# Specimen Dossier: LiteLLM

Status: Draft
Specimen: LiteLLM
Owner:
Last updated: 2026-03-11
Scope: Unified LLM API abstraction and provider routing

## 1) Why this specimen matters

LiteLLM is the most widely adopted open-source solution for unifying LLM provider APIs. It provides both a Python SDK and a proxy server, with 100+ provider integrations. Underlay's `underlay-ai-runtime` has similar goals but Rust-native and less feature-complete.

## 2) Product and era context

- **Launched**: ~2023 by BerriAI
- **Positioning**: "Call all LLM APIs using the OpenAI format"
- **Era**: Post-ChatGPT API explosion (2023-2024)
- **Competition**: Portkey (managed), OpenRouter (routing service), Helicone (observability)
- **Adoption**: Very high in Python ecosystem, growing in other languages

## 3) Defining bets

1. **OpenAI API as the lingua franca** - Standardize all providers to OpenAI's /chat/completions format
2. **Proxy architecture** - Intercept and route requests rather than SDK-only
3. **Drop-in replacement** - Change base URL + API key, minimal code changes
4. **Enterprise features** - Rate limiting, budget management, spend tracking as premium features

## 4) Standout strengths

- **Provider coverage**: 100+ providers including OpenAI, Anthropic, Azure, Bedrock, Gemini, local models
- **Fallback routing**: Automatic failover between providers
- **Budget controls**: Per-key, per-model, per-team spending limits
- **Observability**: Logging, tracing, spend tracking out of the box
- **Caching**: Optional prompt caching layer
- **Guardrails**: Input/output filtering hooks

## 5) Chronic weaknesses and recurring costs

- **Python-centric**: Core is Python, other languages use HTTP proxy
- **Proxy latency**: Additional hop adds latency vs direct calls
- **Feature parity lag**: New provider features take time to map to OpenAI format
- **Complexity at scale**: Enterprise features add operational complexity
- **Streaming quirks**: Some streaming edge cases across providers don't map cleanly
- **Tool calling gaps**: Provider differences in tool/function calling are hard to paper over

## 6) Between-version corrections

- Added async support early for better performance
- Moved from pure SDK to proxy architecture for language-agnostic support
- Added LiteLLM Proxy (UI) for non-technical users
- Introduced "virtual keys" for multi-tenant budget management

## 7) Project-relevant lessons

### Adopt carefully

- Proxy/gateway pattern for multi-tenant scenarios
- Budget/spend tracking per request attribution
- Fallback routing with health checks

### Reject early

- Python dependency in Rust stack (Underlay is correct to stay Rust-native)
- Forcing OpenAI format on everything (some providers have unique features)
- Heavyweight proxy for simple use cases

### Prototype before deciding

- Streaming edge cases with fallback
- Tool calling standardization across providers
- Cost tracking accuracy with streaming tokens

## 8) Source inventory

| Source | Type | Version/Era | Confidence | Notes |
| --- | --- | --- | --- | --- |
| docs.litellm.ai | Official docs | 2024-2025 | High | Comprehensive, well-maintained |
| GitHub berriai/litellm | Source | main | High | Active development, MIT license |
| LiteLLM blog | Company | 2024 | Medium | Product announcements |
| HN "litellm" search | Community | 2023-2025 | Medium | Real-world usage reports |

## 9) Open questions

- How does LiteLLM handle Anthropic's computer use (beta) which doesn't fit OpenAI format?
- What's the performance overhead of the proxy in high-throughput scenarios?
- How well does their tool calling abstraction actually work in practice?

## Next Task

Compare LiteLLM's approach to Underlay's `underlay-ai-runtime` and identify specific gaps (routing, cost tracking, provider coverage) for a translation memo.
