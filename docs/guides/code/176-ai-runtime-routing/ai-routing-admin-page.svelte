<script lang="ts">
import {
  type AiRoutingAdminMessages,
  type AiRoutingOpsOptions,
  type AiRoutingOpsSource
} from "@inflatable-cookie/underlay/runtime/ai";
import {
  auth,
  authLoading,
  currentUser } from "$lib/stores/auth";
  import { platformCommands } from "@your-org/client";
  import { PageHeader } from "@inflatable-cookie/poodle-svelte";
  // @ts-nocheck
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
