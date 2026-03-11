# Source Hub: AI/LLM Provider Ecosystem

Status: Active
Hub: AI-LLM-001
Owner:
Last updated: 2026-03-11
Scope: Multi-provider LLM routing, cost optimization, and abstraction patterns

## 1) Questions this hub should answer

- How do teams route between multiple LLM providers today? (cost, capability, resilience)
- What are the common failure modes when abstracting over OpenAI, Anthropic, Google, etc.?
- How is structured output / JSON mode handled across providers?
- What patterns exist for cost tracking and attribution per request/tenant?
- How do gateway architectures (self-hosted vs managed) compare?
- What emerging standards exist for model context protocols and tool calling?

## 2) Strongest primary sources

| Source family | Authority | Version/Currency | Biases or gaps | Notes |
| --- | --- | --- | --- | --- |
| OpenAI API docs | OpenAI | v1, constantly updated | OpenAI-centric, but de facto standard | /chat/completions, structured outputs, function calling |
| Anthropic API docs | Anthropic | v1 | Claude-centric | Message format, tool use, system prompts |
| Google Gemini docs | Google | v1beta | Google-centric | Different tool format, streaming differences |
| LiteLLM docs | BerriAI (LiteLLM) | Active OSS | May overstate compatibility | Most popular unified interface |
| Portkey docs | Portkey.ai | Active SaaS | Vendor commercial interest | Gateway pattern with observability |
| MCP spec | Anthropic | Nov 2024 | New standard, limited adoption | Model Context Protocol for tool/context |
| Vercel AI SDK docs | Vercel | v4 | React/Next.js focused | Streaming patterns, React integration |

## 3) Secondary sources worth using carefully

| Source family | Why it helps | Risks or bias | Notes |
| --- | --- | --- | --- |
| LangChain docs | Patterns for chaining, agents | Heavy abstraction, version churn | Overkill for simple routing |
| OpenRouter docs | Model routing as a service | Third-party dependency | Good for capability mapping |
| LlamaIndex docs | RAG patterns | Tightly coupled to their stack | Less relevant for pure routing |
| Hacker News discussions | Real-world pain points | Anecdotal, no verification | Search "LiteLLM", "prompt proxy" |
| Cloud provider AI docs (AWS Bedrock, Azure OpenAI) | Enterprise deployment patterns | Vendor lock-in implications | Good for enterprise requirements |

## 4) Source rules

1. **API compatibility claims**: Verify with actual API calls, not just documentation
2. **Performance benchmarks**: Treat with skepticism unless methodology is clear
3. **Security practices**: Check for prompt injection, key handling, audit logging
4. **Pricing comparisons**: Use current pricing pages, not historical posts

## 5) Tracks or questions this hub should feed

- Value Track: Multi-provider routing strategies
- Specimen Dossier: LiteLLM (architecture, failure modes)
- Specimen Dossier: Portkey (gateway patterns)
- Translation Memo: When to adopt a gateway vs direct integration

## 6) Known blind spots

- OpenAI's realtime API (WebSocket) is evolving rapidly
- Anthropic's computer use (beta) - API may change
- MCP adoption is early; few real-world implementations
- Ollama/local model serving patterns not well documented

## Next Task

Create specimen dossier for LiteLLM as the most relevant comparison for Underlay's `underlay-ai-runtime` abstraction.
