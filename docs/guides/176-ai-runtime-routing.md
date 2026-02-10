# 176 — AI Runtime Routing (Backend)

This guide defines the reusable backend boundary for LLM execution in Underlay-based apps.

## Goals

- Keep frontend clients talking only to app backends.
- Keep provider credentials and routing policy server-side.
- Allow model/provider swaps without changing UI contracts.

## Underlay crate

Use `underlay-ai-runtime` for app-agnostic runtime pieces:

- `LlmClient` trait
- `OpenAiCompatibleClient`
- `ProviderRegistry`
- route candidate and capability types
- request/response/error contracts

## Recommended architecture in consuming apps

1. App config crate owns environment and secret loading.
2. App infra crate wires `underlay-ai-runtime` client instances from config.
3. Jobs/workers orchestrate route selection and fallback policy.
4. App DB/API layers own app-specific routing config tables and diagnostics endpoints.

## Security defaults

- Do not log provider response bodies by default.
- Keep runtime logs metadata-oriented (provider/model/alias/status/fallback counts).
- Enforce explicit host allowlisting for non-local environments.

## OpenAI-compatible transport

`OpenAiCompatibleClient` targets `/chat/completions` with:

- bearer auth
- request timeout
- structured JSON response mode
- sanitized provider metadata passthrough (allowlisted keys only)

## What remains app-specific

- Action-key to alias mapping strategy
- DB schema for routing config and versioning
- Runtime rollout flags (canary %, action-prefix rollout, force-primary)
- Admin diagnostics and governance endpoints

## TypeScript admin helper

Underlay exports `createAiRoutingOpsController` from `ts/src/patterns/ai-routing-ops.svelte.ts`.

Use it to compose app-specific fetchers for diagnostics/metrics/cost/anomalies/alerts/parity,
while keeping state and refresh behavior reusable across admin apps.

## Svelte admin dashboard pattern

Underlay also exports `AiRoutingAdmin` from `ts/src/patterns/AiRoutingAdmin.svelte`.

Use it when you want a ready-made AI routing operations page with:

- summary cards (routing config, alert counts, top spike)
- window controls for metrics/anomalies/parity/cost
- preconfigured tables for metrics, anomalies, parity, and daily cost

Apps only need to provide an `AiRoutingOpsSource` implementation and auth gating.

Optional customization:

- `windowDefaults` (`AiRoutingOpsOptions`) for default metric/cost/parity/anomaly windows
- `messages` (`AiRoutingAdminMessages`) for empty-state and button labels
