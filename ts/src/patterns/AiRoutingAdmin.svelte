<script lang="ts">
  import { untrack } from "svelte";
  import { Button, Callout, Card, NumberEntry } from "@poodle/svelte-primitives";
  import {
    Badge,
    DataTable,
    PageLoading,
    TimeAgo,
    type DataTableColumn
  } from "../components";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";
  import ChartLine from "lucide-svelte/icons/chart-line";
  import GitCompare from "lucide-svelte/icons/git-compare";
  import RefreshCw from "lucide-svelte/icons/refresh-cw";
  import PageHeader from "./PageHeader.svelte";
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
  } from "./ai-routing-ops.svelte";

  interface Props {
    source: AiRoutingOpsSource;
    /** @deprecated Use `windowDefaults` */
    options?: AiRoutingOpsOptions;
    windowDefaults?: AiRoutingOpsOptions;
    messages?: AiRoutingAdminMessages;
    enabled?: boolean;
    section?: string;
    backHref?: string | null;
    backLabel?: string;
    loadingMessage?: string;
  }

  let {
    source,
    options,
    windowDefaults,
    messages,
    enabled = true,
    section = "AI Routing",
    backHref = null,
    backLabel = "Back",
    loadingMessage = "Loading AI routing dashboard..."
  }: Props = $props();

  const controllerSource = untrack(() => source);
  const controllerOptions = untrack(() => windowDefaults ?? options);
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

  const metricColumns: DataTableColumn<AiRoutingMetric>[] = [
    { key: "actionKey", label: "Action", width: "minmax(220px, 2fr)" },
    { key: "providerModel", label: "Provider / Model", width: "minmax(220px, 2fr)" },
    { key: "runs", label: "Runs", width: "90px" },
    { key: "successRate", label: "Success", width: "100px" },
    { key: "latency", label: "Latency", width: "120px", hideOnMobile: true },
    { key: "tokens", label: "Tokens", width: "130px", hideOnMobile: true },
    { key: "recovery", label: "Recovery", width: "minmax(190px, 1.4fr)" }
  ];

  const costColumns: DataTableColumn<AiRoutingDailyCost>[] = [
    { key: "day", label: "Day", width: "110px" },
    { key: "actionKey", label: "Action", width: "minmax(200px, 2fr)" },
    { key: "providerModel", label: "Provider / Model", width: "minmax(210px, 2fr)" },
    { key: "cost", label: "Cost (USD)", width: "120px" },
    { key: "budget", label: "Budget", width: "100px" }
  ];

  const anomalyColumns: DataTableColumn<AiRoutingCostAnomaly>[] = [
    { key: "day", label: "Day", width: "110px" },
    { key: "actionKey", label: "Action", width: "minmax(200px, 2fr)" },
    { key: "providerModel", label: "Provider / Model", width: "minmax(210px, 2fr)" },
    { key: "delta", label: "Delta", width: "100px" },
    { key: "spike", label: "Spike", width: "90px" }
  ];

  const parityColumns: DataTableColumn<AiRoutingParity>[] = [
    { key: "actionKey", label: "Action", width: "minmax(220px, 2fr)" },
    { key: "modelAlias", label: "Alias", width: "150px" },
    { key: "runs", label: "Runs", width: "90px" },
    { key: "successRate", label: "Success", width: "100px" },
    { key: "failed", label: "Failed", width: "90px" }
  ];

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
</script>

<PageHeader {section} {backHref} {backLabel}>
  {#snippet actions()}
    <Button type="button" variant="ghost" on:click={() => ops.refreshAll()} disabled={ops.loading}>
      <svelte:fragment slot="leading">
        <RefreshCw size={16} />
      </svelte:fragment>
      {labels.refresh}
    </Button>
  {/snippet}
</PageHeader>

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
              · Updated <TimeAgo date={ops.diagnostics.latestUpdatedAt} short tooltipFormat="datetime" />
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
            <Badge variant="danger">Dead letters: {ops.alerts?.deadLetterCount24h ?? 0}</Badge>
            <Badge variant="danger">Runtime failures: {ops.alerts?.runtimeFailureCount24h ?? 0}</Badge>
            <Badge variant="warning">Circuit open: {ops.alerts?.circuitOpenCount24h ?? 0}</Badge>
            <Badge variant="info">Fallback runs: {ops.alerts?.fallbackRunCount24h ?? 0}</Badge>
            <Badge variant="warning">Chain exhausted: {ops.alerts?.exhaustedChainRunCount24h ?? 0}</Badge>
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
          <NumberEntry id="ai-routing-metric-hours" bind:value={metricHoursInput} min={1} max={720} />
          <Button type="button" variant="secondary" on:click={applyMetricWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
    <DataTable data={ops.metrics} columns={metricColumns} emptyMessage={labels.metricsEmpty} showLimitSelector={false}>
      {#snippet cell({ column, row })}
        {#if column.key === "providerModel"}
          <code>{row.providerName}</code>/<code>{row.modelName}</code>
        {:else if column.key === "runs"}
          {row.runCount}
        {:else if column.key === "successRate"}
          {toPercent(row.runCount > 0 ? row.successCount / row.runCount : 0)}
        {:else if column.key === "latency"}
          {#if row.avgLatencyMs !== null}
            {Math.round(row.avgLatencyMs)}ms avg / {Math.round(row.p95LatencyMs ?? row.avgLatencyMs)}ms p95
          {:else}
            —
          {/if}
        {:else if column.key === "tokens"}
          {row.inputTokensSum + row.outputTokensSum}
        {:else if column.key === "recovery"}
          <div class="metric-recovery-cell">
            <span class="summary-text">
              {#if row.avgRouteAttemptCount !== null}
                Avg {row.avgRouteAttemptCount.toFixed(1)} routes
              {:else}
                Avg —
              {/if}
            </span>
            <div class="pill-row">
              <Badge variant="info">Fallback: {row.fallbackRunCount}</Badge>
              <Badge variant="warning">Circuit: {row.circuitOpenRunCount}</Badge>
              <Badge variant="danger">Exhausted: {row.exhaustedChainRunCount}</Badge>
            </div>
          </div>
        {/if}
      {/snippet}
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
          <NumberEntry id="ai-routing-anomaly-days" bind:value={anomalyDaysInput} min={2} max={90} />
          <Button type="button" variant="secondary" on:click={applyAnomalyWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
    <DataTable data={ops.anomalies} columns={anomalyColumns} emptyMessage={labels.anomaliesEmpty} showLimitSelector={false}>
      {#snippet cell({ column, row })}
        {#if column.key === "providerModel"}
          <code>{row.providerName}</code>/<code>{row.modelName}</code>
        {:else if column.key === "delta"}
          {row.deltaPercent.toFixed(1)}%
        {:else if column.key === "spike"}
          <Badge variant={row.isSpike ? "danger" : "muted"}>{row.isSpike ? "Spike" : "Normal"}</Badge>
        {/if}
      {/snippet}
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
          <NumberEntry id="ai-routing-parity-hours" bind:value={parityHoursInput} min={1} max={720} />
          <Button type="button" variant="secondary" on:click={applyParityWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
    <DataTable data={ops.parity} columns={parityColumns} emptyMessage={labels.parityEmpty} showLimitSelector={false}>
      {#snippet cell({ column, row })}
        {#if column.key === "successRate"}
          {toPercent(row.successRate)}
        {/if}
      {/snippet}
    </DataTable>
    </div>
  </section>

  <section class="ops-section">
    <div class="ops-section__header">
      <h2 class="ops-section__title">Daily cost</h2>
      <div class="ops-section__controls">
        <div class="controls-row">
          <NumberEntry id="ai-routing-cost-days" bind:value={costDaysInput} min={1} max={365} />
          <Button type="button" variant="secondary" on:click={applyCostWindow}>{labels.apply}</Button>
        </div>
      </div>
    </div>
    <div class="ops-section__content">
    <DataTable data={ops.cost} columns={costColumns} emptyMessage={labels.costEmpty} showLimitSelector={false}>
      {#snippet cell({ column, row })}
        {#if column.key === "providerModel"}
          <code>{row.providerName}</code>/<code>{row.modelName}</code>
        {:else if column.key === "cost"}
          ${row.estimatedCostUsd.toFixed(4)}
        {:else if column.key === "budget"}
          <Badge variant={row.overBudget ? "danger" : "success"}>{row.overBudget ? "Over" : "OK"}</Badge>
        {/if}
      {/snippet}
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
