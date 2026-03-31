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
- `RetryMiddleware`
- `CircuitBreakerMiddleware`
- `RouteChainExecutor`

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

## Opt-in resilience middleware

The runtime now includes additive resilience primitives for apps that want shared retry,
circuit-breaker, and fallback-chain behavior without changing provider configuration ownership.

### Retry and circuit breaker composition

```rust
use std::sync::Arc;

use underlay_ai_runtime::{
    CircuitBreakerConfig,
    CircuitBreakerMiddleware,
    OpenAiCompatibleClient,
    ProviderRegistry,
    RetryConfig,
    RetryMiddleware,
};

let openai = CircuitBreakerMiddleware::new(
    RetryMiddleware::new(
        OpenAiCompatibleClient::new("https://api.openai.com/v1", "openai-key")?,
        RetryConfig::default(),
    ),
    CircuitBreakerConfig::default(),
);

let anthropic = CircuitBreakerMiddleware::new(
    RetryMiddleware::new(
        OpenAiCompatibleClient::new("https://api.anthropic.com/v1", "anthropic-key")?,
        RetryConfig::default(),
    ),
    CircuitBreakerConfig::default(),
);

let mut registry = ProviderRegistry::new();
registry.register("openai", Arc::new(openai));
registry.register("anthropic", Arc::new(anthropic));
```

Defaults:

- retry middleware retries `RateLimit`, `Timeout`, `Provider`, and `Unknown` failures
- circuit breaker state is in-memory and process-local
- middleware is opt-in; existing clients keep current behavior until wrapped explicitly

### Route fallback chains

Use `RouteChainExecutor` when an app already resolved an ordered list of routes and wants
deterministic fallback across providers or models.

```rust
use underlay_ai_runtime::{RouteChainExecutor, ResolvedModelRoute};

let routes = vec![
    ResolvedModelRoute {
        alias: "authoring.primary".to_string(),
        provider_name: "openai".to_string(),
        model_name: "gpt-4.1-mini".to_string(),
        provider_metadata: None,
    },
    ResolvedModelRoute {
        alias: "authoring.fallback".to_string(),
        provider_name: "anthropic".to_string(),
        model_name: "claude-3-5-sonnet".to_string(),
        provider_metadata: None,
    },
];

let result = RouteChainExecutor::new(registry)
    .execute_with_fallback(&routes, &request)
    .await?;

let selected_route = result.route;
let attempt_history = result.attempts;
```

Behavior notes:

- fallback continues for `Auth`, `RateLimit`, `Timeout`, `Provider`, `Unknown`, and `CircuitOpen`
- validation errors stop the chain immediately because a second provider is unlikely to fix a bad request or malformed response contract
- successful results include the winning route plus the attempt history for logging or diagnostics

### First-batch non-goals

- shared cost tracking or budget enforcement
- dead-letter ownership for failed AI actions
- streaming-specific fallback semantics

Consumers that already have local retry or route-chain code can replace it incrementally:

1. Wrap one provider client with `RetryMiddleware` only.
2. Add `CircuitBreakerMiddleware` once thresholds are tuned for that action class.
3. Replace app-local fallback loops with `RouteChainExecutor` when the app already produces an ordered route list.

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

`AiRoutingAdmin` is retired from the public Underlay surface.

The stable boundary is now:

- `createAiRoutingOpsController` for state, refresh behavior, and window control
- direct Poodle composition for page framing, cards, loading/error states, and tables

That keeps the reusable operational data contract in Underlay without preserving
a second page-shaped public shell by inertia.

This is also the preferred UI posture for the broader guide set: keep Underlay
on the controller/runtime side and build the visible screen in Poodle.

Typical page composition:

- Poodle `PageHeader` for route framing
- Poodle `Card` for summary panels
- Poodle `PageLoading` and `Callout` for loading/error handling
- Poodle `DataTable` for metrics, anomalies, parity, and daily cost tables

Optional customization stays app-owned:

- `windowDefaults` (`AiRoutingOpsOptions`) for default metric/cost/parity/anomaly windows
- `messages` (`AiRoutingAdminMessages`) for empty-state and button labels

## Role and permission expectations

- Treat AI routing operations surfaces as privileged admin tooling, not general user UI.
- Require staff-level access at minimum (`admin`), and prefer stricter gates (`superadmin`) for rollout/override actions.
- Enforce authorization on backend endpoints and only use frontend route guards for UX.
- Keep provider credentials and policy controls server-side; never expose provider keys to browser clients.
- For RBAC implementation patterns, see [067-authorization.md](./067-authorization.md).

### Minimal Svelte integration example

Reference file: `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-page.svelte`

```svelte
<script lang="ts">
  import { auth, authLoading, currentUser } from "$lib/stores/auth";
  import { platformCommands } from "@cattle-grid";
  import { PageHeader } from "@poodle/svelte-composites";
  import {
    type AiRoutingOpsSource,
    type AiRoutingOpsOptions,
    type AiRoutingAdminMessages
  } from "@decodelabs/underlay/runtime";
  import AiRoutingOpsPanel from "./AiRoutingOpsPanel.svelte";

  const getToken = auth.getTokenProvider();

  async function withToken<T>(operation: (token: string) => Promise<T>): Promise<T> {
    const token = await getToken();
    if (!token) throw new Error("Not authenticated");
    return await operation(token);
  }

  const source: AiRoutingOpsSource = {
    fetchDiagnostics: () => withToken((token) => platformCommands.getAiRoutingDiagnostics(fetch, token)),
    fetchMetrics: (hours) => withToken((token) => platformCommands.listAiRoutingMetrics(fetch, token, hours)),
    fetchCost: (days) => withToken((token) => platformCommands.listAiRoutingCost(fetch, token, { days })),
    fetchCostAnomalies: (days) =>
      withToken((token) => platformCommands.listAiRoutingCostAnomalies(fetch, token, { baselineDays: days })),
    fetchAlerts: () => withToken((token) => platformCommands.getAiRoutingAlerts(fetch, token)),
    fetchParity: (hours) => withToken((token) => platformCommands.listAiRoutingParity(fetch, token, hours))
  };

  const enabled = $derived(!$authLoading && Boolean($currentUser));

  const windowDefaults: AiRoutingOpsOptions = {
    defaultMetricHours: 24,
    defaultCostDays: 30,
    defaultParityHours: 24,
    defaultAnomalyDays: 14
  };

  const messages: AiRoutingAdminMessages = {
    noSpike: "No cost spikes detected in this baseline window."
  };
</script>

<PageHeader section="AI Routing" backHref="/system" backLabel="Back to system" />

<AiRoutingOpsPanel
  {source}
  {enabled}
  {windowDefaults}
  {messages}
/>
```

### Embedded variant (tabs/subsections)

Reference file: `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-embedded.svelte`

Use this variant when AI routing appears inside a nested admin tab, where you want custom labels and no page header:

```svelte
<AiRoutingOpsPanel
  {source}
  {enabled}
  {messages}
/>
```

## Troubleshooting

- **`Not authenticated` errors**: Ensure your token provider returns a non-null token before invoking command calls.
- **Page stays on loading/empty**: Verify `enabled` becomes `true` only after auth state resolves and a user is present.
- **No diagnostics data**: Confirm backend endpoints for diagnostics/metrics/cost/anomalies/alerts/parity are wired and authorized for the current role.
- **Unexpected window values**: `createAiRoutingOpsController` clamps window inputs to safe ranges before refresh (`metric/parity: 1-720h`, `anomaly: 2-90d`, `cost: 1-365d`).
- **Label overrides not applied**: Pass `messages` and/or `windowDefaults` directly to your composed panel/controller layer (not only to your source adapter).
