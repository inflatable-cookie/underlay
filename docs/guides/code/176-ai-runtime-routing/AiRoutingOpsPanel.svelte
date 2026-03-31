<script lang="ts">
import {
  createAiRoutingOpsController,
  maxCostSpike,
  toPercent,
  type AiRoutingAdminMessages,
  type AiRoutingCostAnomaly,
  type AiRoutingDailyCost,
  type AiRoutingMetric,
  type AiRoutingOpsOptions,
  type AiRoutingOpsSource,
  type AiRoutingParity
} from "@decodelabs/underlay/runtime";
import {
  untrack } from "svelte";
  import { Button,
  Callout,
  Card,
  NumberInput,
  Pill,
  TimeAgo } from "@poodle/svelte-primitives";
  import { DataTable,
  PageLoading,
  type TableColumn,
  type TableRow } from "@poodle/svelte-composites";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";
  import ChartLine from "lucide-svelte/icons/chart-line";
  import GitCompare from "lucide-svelte/icons/git-compare";
  // @ts-nocheck
  
  interface Props {
    source: AiRoutingOpsSource;
    windowDefaults?: AiRoutingOpsOptions;
    messages?: AiRoutingAdminMessages;
    enabled?: boolean;
    loadingMessage?: string;
  }

  let {
    source,
    windowDefaults,
    messages,
    enabled = true,
    loadingMessage = "Loading AI routing dashboard..."
  }: Props = $props();

  const controllerSource = untrack(() => source);
  const controllerOptions = untrack(() => windowDefaults);
  const ops = createAiRoutingOpsController(controllerSource, controllerOptions);

  const labels = $derived({
    refresh: messages?.refreshLabel ?? "Refresh",
    apply: messages?.applyLabel ?? "Apply",
    metricsEmpty: messages?.metricsEmpty ?? "No routing metrics yet",
    anomaliesEmpty: messages?.anomaliesEmpty ?? "No cost anomalies",
    parityEmpty: messages?.parityEmpty ?? "No parity records",
    costEmpty: messages?.costEmpty ?? "No cost data yet",
    noSpike: messages?.noSpike ?? "No spike records in current window."
  });

  $effect(() => {
    if (enabled) {
      void ops.refreshAll();
    }
  });

  const metricColumns: TableColumn[] = [
    { id: "actionKey", label: "Action", width: "minmax(220px, 2fr)" },
    { id: "providerModel", label: "Provider / Model", width: "minmax(220px, 2fr)" },
    { id: "runs", label: "Runs", width: "90px" },
    { id: "successRate", label: "Success", width: "100px" },
    { id: "latency", label: "Latency", width: "120px", hideOnMobile: true },
    { id: "tokens", label: "Tokens", width: "130px", hideOnMobile: true },
    { id: "recovery", label: "Recovery", width: "minmax(190px, 1.4fr)" }
  ];

  const costColumns: TableColumn[] = [
    { id: "day", label: "Day", width: "110px" },
    { id: "actionKey", label: "Action", width: "minmax(200px, 2fr)" },
    { id: "providerModel", label: "Provider / Model", width: "minmax(210px, 2fr)" },
    { id: "cost", label: "Cost (USD)", width: "120px" },
    { id: "budget", label: "Budget", width: "100px" }
  ];

  const anomalyColumns: TableColumn[] = [
    { id: "day", label: "Day", width: "110px" },
    { id: "actionKey", label: "Action", width: "minmax(200px, 2fr)" },
    { id: "providerModel", label: "Provider / Model", width: "minmax(210px, 2fr)" },
    { id: "delta", label: "Delta", width: "100px" },
    { id: "spike", label: "Spike", width: "90px" }
  ];

  const parityColumns: TableColumn[] = [
    { id: "actionKey", label: "Action", width: "minmax(220px, 2fr)" },
    { id: "modelAlias", label: "Alias", width: "150px" },
    { id: "runs", label: "Runs", width: "90px" },
    { id: "successRate", label: "Success", width: "100px" },
    { id: "failed", label: "Failed", width: "90px" }
  ];

  const metricRows = $derived<TableRow<AiRoutingMetric>[]>(
    ops.metrics.map((metric) => ({
      id: `${metric.actionKey}:${metric.providerName}:${metric.modelName}`,
      cells: {
        actionKey: metric.actionKey,
        providerModel: `${metric.providerName}/${metric.modelName}`,
        runs: metric.runCount,
        successRate: toPercent(metric.runCount > 0 ? metric.successCount / metric.runCount : 0),
        latency: metric.avgLatencyMs !== null ? `${Math.round(metric.avgLatencyMs)}ms` : "—",
        tokens: metric.inputTokensSum + metric.outputTokensSum,
        recovery: ""
      },
      data: metric
    }))
  );

  const anomalyRows = $derived<TableRow<AiRoutingCostAnomaly>[]>(
    ops.anomalies.map((anomaly) => ({
      id: `${anomaly.day}:${anomaly.actionKey}:${anomaly.providerName}:${anomaly.modelName}`,
      cells: {
        day: anomaly.day,
        actionKey: anomaly.actionKey,
        providerModel: `${anomaly.providerName}/${anomaly.modelName}`,
        delta: anomaly.deltaPercent.toFixed(1),
        spike: anomaly.isSpike ? "Spike" : "Normal"
      },
      data: anomaly
    }))
  );

  const parityRows = $derived<TableRow<AiRoutingParity>[]>(
    ops.parity.map((parity) => ({
      id: `${parity.actionKey}:${parity.modelAlias}`,
      cells: {
        actionKey: parity.actionKey,
        modelAlias: parity.modelAlias,
        runs: parity.runCount,
        successRate: toPercent(parity.successRate),
        failed: parity.failedCount
      },
      data: parity
    }))
  );

  const costRows = $derived<TableRow<AiRoutingDailyCost>[]>(
    ops.cost.map((cost) => ({
      id: `${cost.day}:${cost.actionKey}:${cost.providerName}:${cost.modelName}`,
      cells: {
        day: cost.day,
        actionKey: cost.actionKey,
        providerModel: `${cost.providerName}/${cost.modelName}`,
        cost: `$${cost.estimatedCostUsd.toFixed(4)}`,
        budget: cost.overBudget ? "Over" : "OK"
      },
      data: cost
    }))
  );

  const topSpike = $derived(maxCostSpike(ops.anomalies));

  let metricHoursInput = $state<number | null>(24);
  let anomalyDaysInput = $state<number | null>(14);
  let parityHoursInput = $state<number | null>(24);
  let costDaysInput = $state<number | null>(30);

  $effect(() => {
    metricHoursInput = ops.metricHours;
    anomalyDaysInput = ops.anomalyDays;
    parityHoursInput = ops.parityHours;
    costDaysInput = ops.costDays;
  });

  function parseWindow(raw: number | null, fallback: number, min: number, max: number): number {
    const value = Number(raw);
    if (!Number.isFinite(value)) return fallback;
    return Math.max(min, Math.min(max, Math.trunc(value)));
  }

  function applyMetricWindow() {
    ops.setMetricHours(parseWindow(metricHoursInput, ops.metricHours, 1, 720));
    void ops.refreshAll();
  }

  function applyAnomalyWindow() {
    ops.setAnomalyDays(parseWindow(anomalyDaysInput, ops.anomalyDays, 2, 90));
    void ops.refreshAll();
  }

  function applyParityWindow() {
    ops.setParityHours(parseWindow(parityHoursInput, ops.parityHours, 1, 720));
    void ops.refreshAll();
  }

  function applyCostWindow() {
    ops.setCostDays(parseWindow(costDaysInput, ops.costDays, 1, 365));
    void ops.refreshAll();
  }

  function badgeTone(variant: "default" | "success" | "warning" | "danger" | "info" | "muted") {
    if (variant === "danger") return "danger" as const;
    if (variant === "warning") return "warning" as const;
    if (variant === "success") return "success" as const;
    if (variant === "info") return "info" as const;
    return "neutral" as const;
  }
</script>

{#if ops.loading && !ops.diagnostics}
  <PageLoading message={loadingMessage} />
{:else if ops.error && !ops.diagnostics}
  <Callout tone="danger" message={ops.error} announceMode="polite" />
{:else}
  <div class="ops-card-grid">
    <div class="ops-card">
      <Card>
        <div class="ops-card__header">
          <h2 class="ops-card__title">Routing config</h2>
        </div>
        <div class="ops-card__body">
          <p class="summary-text">
            Version <strong>{ops.diagnostics?.configVersion ?? 0}</strong>
            {#if ops.diagnostics?.latestUpdatedAt}
              · Updated <TimeAgo datetime={ops.diagnostics.latestUpdatedAt} short tooltipFormat="datetime" />
            {/if}
          </p>
          <p class="summary-text">
            Providers {ops.diagnostics?.enabledProviderCount ?? 0}/{ops.diagnostics?.providerCount ?? 0}
            · Aliases {ops.diagnostics?.enabledAliasCount ?? 0}/{ops.diagnostics?.aliasCount ?? 0}
            · Bindings {ops.diagnostics?.enabledBindingCount ?? 0}/{ops.diagnostics?.bindingCount ?? 0}
          </p>
        </div>
      </Card>
    </div>

    <div class="ops-card">
      <Card>
        <div class="ops-card__header">
          <h2 class="ops-card__title">Alert summary (24h)</h2>
        </div>
        <div class="ops-card__body">
          <div class="pill-row">
            <Pill appearance="badge" tone={badgeTone("danger")}>Dead letters: {ops.alerts?.deadLetterCount24h ?? 0}</Pill>
            <Pill appearance="badge" tone={badgeTone("danger")}>Runtime failures: {ops.alerts?.runtimeFailureCount24h ?? 0}</Pill>
            <Pill appearance="badge" tone={badgeTone("warning")}>Circuit open: {ops.alerts?.circuitOpenCount24h ?? 0}</Pill>
            <Pill appearance="badge" tone={badgeTone("info")}>Fallback runs: {ops.alerts?.fallbackRunCount24h ?? 0}</Pill>
            <Pill appearance="badge" tone={badgeTone("warning")}>Chain exhausted: {ops.alerts?.exhaustedChainRunCount24h ?? 0}</Pill>
          </div>
        </div>
      </Card>
    </div>

    <div class="ops-card">
      <Card>
        <div class="ops-card__header">
          <h2 class="ops-card__title">Cost spike</h2>
        </div>
        <div class="ops-card__body">
          {#if topSpike}
            <p class="summary-text">
              <strong>{topSpike.actionKey}</strong> on {topSpike.providerName}/{topSpike.modelName}
            </p>
            <p class="summary-text">
              Delta <strong>{topSpike.deltaPercent.toFixed(1)}%</strong> · Today ${topSpike.todayEstimatedCostUsd.toFixed(4)}
            </p>
          {:else}
            <p class="summary-text">{labels.noSpike}</p>
          {/if}
        </div>
      </Card>
    </div>
  </div>

  {#if ops.error}
    <Callout tone="danger" message={ops.error} announceMode="polite" />
  {/if}

  <section class="ops-section">
    <div class="ops-section__header">
      <h2 class="ops-section__title">
        <ChartLine size={16} aria-hidden="true" />
        Routing metrics
      </h2>
      <div class="ops-section__controls">
        <div class="controls-row">
          <NumberInput id="ai-routing-metric-hours" bind:value={metricHoursInput} min={1} max={720} />
          <Button type="button" variant="secondary" on:click={applyMetricWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
      <DataTable rows={metricRows} columns={metricColumns} emptyMessage={labels.metricsEmpty} showLimitSelector={false}>
        <svelte:fragment slot="cell" let:column let:row>
          {@const metric = row.data as AiRoutingMetric | undefined}
          {#if metric && column.id === "providerModel"}
            <code>{metric.providerName}</code>/<code>{metric.modelName}</code>
          {:else if metric && column.id === "runs"}
            {metric.runCount}
          {:else if metric && column.id === "successRate"}
            {toPercent(metric.runCount > 0 ? metric.successCount / metric.runCount : 0)}
          {:else if metric && column.id === "latency"}
            {#if metric.avgLatencyMs !== null}
              {Math.round(metric.avgLatencyMs)}ms avg / {Math.round(metric.p95LatencyMs ?? metric.avgLatencyMs)}ms p95
            {:else}
              —
            {/if}
          {:else if metric && column.id === "tokens"}
            {metric.inputTokensSum + metric.outputTokensSum}
          {:else if metric && column.id === "recovery"}
            <div class="metric-recovery-cell">
              <span class="summary-text">
                {#if metric.avgRouteAttemptCount !== null}
                  Avg {metric.avgRouteAttemptCount.toFixed(1)} routes
                {:else}
                  Avg —
                {/if}
              </span>
              <div class="pill-row">
                <Pill appearance="badge" tone={badgeTone("info")}>Fallback: {metric.fallbackRunCount}</Pill>
                <Pill appearance="badge" tone={badgeTone("warning")}>Circuit: {metric.circuitOpenRunCount}</Pill>
                <Pill appearance="badge" tone={badgeTone("danger")}>Exhausted: {metric.exhaustedChainRunCount}</Pill>
              </div>
            </div>
          {:else}
            {row.cells[column.id] ?? "—"}
          {/if}
        </svelte:fragment>
      </DataTable>
    </div>
  </section>

  <section class="ops-section">
    <div class="ops-section__header">
      <h2 class="ops-section__title">
        <AlertTriangle size={16} aria-hidden="true" />
        Cost anomalies
      </h2>
      <div class="ops-section__controls">
        <div class="controls-row">
          <NumberInput id="ai-routing-anomaly-days" bind:value={anomalyDaysInput} min={2} max={90} />
          <Button type="button" variant="secondary" on:click={applyAnomalyWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
      <DataTable rows={anomalyRows} columns={anomalyColumns} emptyMessage={labels.anomaliesEmpty} showLimitSelector={false}>
        <svelte:fragment slot="cell" let:column let:row>
          {@const anomaly = row.data as AiRoutingCostAnomaly | undefined}
          {#if anomaly && column.id === "providerModel"}
            <code>{anomaly.providerName}</code>/<code>{anomaly.modelName}</code>
          {:else if anomaly && column.id === "delta"}
            {anomaly.deltaPercent.toFixed(1)}%
          {:else if anomaly && column.id === "spike"}
            <Pill appearance="badge" tone={badgeTone(anomaly.isSpike ? "danger" : "muted")}>{anomaly.isSpike ? "Spike" : "Normal"}</Pill>
          {:else}
            {row.cells[column.id] ?? "—"}
          {/if}
        </svelte:fragment>
      </DataTable>
    </div>
  </section>

  <section class="ops-section">
    <div class="ops-section__header">
      <h2 class="ops-section__title">
        <GitCompare size={16} aria-hidden="true" />
        Alias parity
      </h2>
      <div class="ops-section__controls">
        <div class="controls-row">
          <NumberInput id="ai-routing-parity-hours" bind:value={parityHoursInput} min={1} max={720} />
          <Button type="button" variant="secondary" on:click={applyParityWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
      <DataTable rows={parityRows} columns={parityColumns} emptyMessage={labels.parityEmpty} showLimitSelector={false}>
        <svelte:fragment slot="cell" let:column let:row>
          {@const parity = row.data as AiRoutingParity | undefined}
          {#if parity && column.id === "successRate"}
            {toPercent(parity.successRate)}
          {:else}
            {row.cells[column.id] ?? "—"}
          {/if}
        </svelte:fragment>
      </DataTable>
    </div>
  </section>

  <section class="ops-section">
    <div class="ops-section__header">
      <h2 class="ops-section__title">Daily cost</h2>
      <div class="ops-section__controls">
        <div class="controls-row">
          <NumberInput id="ai-routing-cost-days" bind:value={costDaysInput} min={1} max={365} />
          <Button type="button" variant="secondary" on:click={applyCostWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
      <DataTable rows={costRows} columns={costColumns} emptyMessage={labels.costEmpty} showLimitSelector={false}>
        <svelte:fragment slot="cell" let:column let:row>
          {@const cost = row.data as AiRoutingDailyCost | undefined}
          {#if cost && column.id === "providerModel"}
            <code>{cost.providerName}</code>/<code>{cost.modelName}</code>
          {:else if cost && column.id === "cost"}
            ${cost.estimatedCostUsd.toFixed(4)}
          {:else if cost && column.id === "budget"}
            <Pill appearance="badge" tone={badgeTone(cost.overBudget ? "danger" : "success")}>{cost.overBudget ? "Over" : "OK"}</Pill>
          {:else}
            {row.cells[column.id] ?? "—"}
          {/if}
        </svelte:fragment>
      </DataTable>
    </div>
  </section>
{/if}

<style>
  .ops-card-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(18rem, 1fr));
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .ops-card {
    gap: 0.4rem;
  }

  .ops-card__header {
    display: flex;
    align-items: center;
    min-width: 0;
  }

  .ops-card__title {
    margin: 0;
    font-size: 0.9rem;
    font-weight: 600;
  }

  .ops-card__body {
    font-size: 0.86rem;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }

  .summary-text {
    margin: 0.2rem 0;
    font-size: 0.86rem;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.9));
  }

  .pill-row {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
  }

  .controls-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
    max-width: 240px;
  }

  .metric-recovery-cell {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
  }

  .ops-section {
    margin-top: 1rem;
  }

  .ops-section__header {
    display: flex;
    flex-wrap: wrap;
    align-items: flex-end;
    justify-content: space-between;
    gap: 0.5rem;
    margin-bottom: 0.5rem;
  }

  .ops-section__title {
    margin: 0;
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    font-size: 1rem;
  }

  .ops-section__controls {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    width: min(100%, 16rem);
  }

  .ops-section__content {
    min-width: 0;
  }

  @media (max-width: 640px) {
    .ops-card-grid {
      grid-template-columns: 1fr;
    }
  }
</style>
