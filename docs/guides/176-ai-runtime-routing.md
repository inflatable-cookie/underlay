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

### Minimal Svelte integration example

Reference file: `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-page.svelte`

```svelte
<script lang="ts">
  import { auth, authLoading, currentUser } from "$lib/stores/auth";
  import { platformCommands } from "@cattle-grid";
  import {
    AiRoutingAdmin,
    type AiRoutingOpsSource,
    type AiRoutingOpsOptions,
    type AiRoutingAdminMessages
  } from "@decodelabs/underlay/patterns";

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

<AiRoutingAdmin
  {source}
  {enabled}
  {windowDefaults}
  {messages}
  section="AI Routing"
  backHref="/system"
  backLabel="Back to system"
/>
```

### Embedded variant (tabs/subsections)

Reference file: `docs/guides/code/176-ai-runtime-routing/ai-routing-admin-embedded.svelte`

Use this variant when AI routing appears inside a nested admin tab, where you want custom labels and no back link:

```svelte
<AiRoutingAdmin
  {source}
  {enabled}
  {messages}
  section="Routing ops"
  backHref={null}
/>
```

## Troubleshooting

- **`Not authenticated` errors**: Ensure your token provider returns a non-null token before invoking command calls.
- **Page stays on loading/empty**: Verify `enabled` becomes `true` only after auth state resolves and a user is present.
- **No diagnostics data**: Confirm backend endpoints for diagnostics/metrics/cost/anomalies/alerts/parity are wired and authorized for the current role.
- **Unexpected window values**: `AiRoutingAdmin` clamps window inputs to safe ranges before refresh (`metric/parity: 1-720h`, `anomaly: 2-90d`, `cost: 1-365d`).
- **Label overrides not applied**: Pass `messages` and/or `windowDefaults` directly to `AiRoutingAdmin` (not only to your source adapter).
