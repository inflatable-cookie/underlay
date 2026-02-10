<script lang="ts">
  import { untrack } from "svelte";
  import {
    Badge,
    Button,
    DataTable,
    FormError,
    NumberInput,
    PageLoading,
    TimeAgo,
    type DataTableColumn
  } from "../components";
  import AlertTriangle from "lucide-svelte/icons/alert-triangle";
  import ChartLine from "lucide-svelte/icons/chart-line";
  import GitCompare from "lucide-svelte/icons/git-compare";
  import RefreshCw from "lucide-svelte/icons/refresh-cw";
  import PageHeader from "./PageHeader.svelte";
  import OpsCard from "./OpsCard.svelte";
  import OpsCardGrid from "./OpsCardGrid.svelte";
  import OpsSection from "./OpsSection.svelte";
  import {
    createAiRoutingOpsController,
    maxCostSpike,
    toPercent,
    type AiRoutingCostAnomaly,
    type AiRoutingDailyCost,
    type AiRoutingMetric,
    type AiRoutingOpsOptions,
    type AiRoutingOpsSource,
    type AiRoutingParity
  } from "./ai-routing-ops.svelte";

  interface Props {
    source: AiRoutingOpsSource;
    options?: AiRoutingOpsOptions;
    enabled?: boolean;
    section?: string;
    backHref?: string | null;
    backLabel?: string;
    loadingMessage?: string;
  }

  let {
    source,
    options,
    enabled = true,
    section = "AI Routing",
    backHref = null,
    backLabel = "Back",
    loadingMessage = "Loading AI routing dashboard..."
  }: Props = $props();

  const controllerSource = untrack(() => source);
  const controllerOptions = untrack(() => options);
  const ops = createAiRoutingOpsController(controllerSource, controllerOptions);

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
    { key: "fallback", label: "Fallback", width: "100px" }
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

  let metricHoursInput = $state("24");
  let anomalyDaysInput = $state("14");
  let parityHoursInput = $state("24");
  let costDaysInput = $state("30");

  $effect(() => {
    metricHoursInput = String(ops.metricHours);
    anomalyDaysInput = String(ops.anomalyDays);
    parityHoursInput = String(ops.parityHours);
    costDaysInput = String(ops.costDays);
  });

  function parseWindow(raw: string, fallback: number, min: number, max: number): number {
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
    <Button type="button" variant="subtle" onclick={() => ops.refreshAll()} disabled={ops.loading}>
      <RefreshCw size={16} />
      Refresh
    </Button>
  {/snippet}
</PageHeader>

{#if ops.loading && !ops.diagnostics}
  <PageLoading message={loadingMessage} />
{:else if ops.error && !ops.diagnostics}
  <FormError message={ops.error} />
{:else}
  <OpsCardGrid>
    <OpsCard title="Routing config">
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
    </OpsCard>

    <OpsCard title="Alert summary (24h)">
      <div class="pill-row">
        <Badge variant="danger">Dead letters: {ops.alerts?.deadLetterCount24h ?? 0}</Badge>
        <Badge variant="danger">Runtime failures: {ops.alerts?.runtimeFailureCount24h ?? 0}</Badge>
        <Badge variant="warning">Circuit open: {ops.alerts?.circuitOpenCount24h ?? 0}</Badge>
        <Badge variant="info">Fallback runs: {ops.alerts?.fallbackRunCount24h ?? 0}</Badge>
      </div>
    </OpsCard>

    <OpsCard title="Cost spike">
      {#if topSpike}
        <p class="summary-text">
          <strong>{topSpike.actionKey}</strong> on {topSpike.providerName}/{topSpike.modelName}
        </p>
        <p class="summary-text">
          Delta <strong>{topSpike.deltaPercent.toFixed(1)}%</strong> · Today ${topSpike.todayEstimatedCostUsd.toFixed(4)}
        </p>
      {:else}
        <p class="summary-text">No spike records in current window.</p>
      {/if}
    </OpsCard>
  </OpsCardGrid>

  {#if ops.error}
    <FormError message={ops.error} />
  {/if}

  <OpsSection title="Routing metrics" icon={ChartLine}>
    {#snippet controls()}
      <div class="controls-row">
        <NumberInput bind:value={metricHoursInput} min={1} max={720} />
        <Button type="button" variant="secondary" onclick={applyMetricWindow}>Apply</Button>
      </div>
    {/snippet}
    <DataTable data={ops.metrics} columns={metricColumns} emptyMessage="No routing metrics yet" showLimitSelector={false}>
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
        {:else if column.key === "fallback"}
          {row.fallbackRunCount}
        {/if}
      {/snippet}
    </DataTable>
  </OpsSection>

  <OpsSection title="Cost anomalies" icon={AlertTriangle}>
    {#snippet controls()}
      <div class="controls-row">
        <NumberInput bind:value={anomalyDaysInput} min={2} max={90} />
        <Button type="button" variant="secondary" onclick={applyAnomalyWindow}>Apply</Button>
      </div>
    {/snippet}
    <DataTable data={ops.anomalies} columns={anomalyColumns} emptyMessage="No cost anomalies" showLimitSelector={false}>
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
  </OpsSection>

  <OpsSection title="Alias parity" icon={GitCompare}>
    {#snippet controls()}
      <div class="controls-row">
        <NumberInput bind:value={parityHoursInput} min={1} max={720} />
        <Button type="button" variant="secondary" onclick={applyParityWindow}>Apply</Button>
      </div>
    {/snippet}
    <DataTable data={ops.parity} columns={parityColumns} emptyMessage="No parity records" showLimitSelector={false}>
      {#snippet cell({ column, row })}
        {#if column.key === "successRate"}
          {toPercent(row.successRate)}
        {/if}
      {/snippet}
    </DataTable>
  </OpsSection>

  <OpsSection title="Daily cost">
    {#snippet controls()}
      <div class="controls-row">
        <NumberInput bind:value={costDaysInput} min={1} max={365} />
        <Button type="button" variant="secondary" onclick={applyCostWindow}>Apply</Button>
      </div>
    {/snippet}
    <DataTable data={ops.cost} columns={costColumns} emptyMessage="No cost data yet" showLimitSelector={false}>
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
  </OpsSection>
{/if}

<style>
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
</style>
