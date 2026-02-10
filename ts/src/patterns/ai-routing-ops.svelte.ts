export interface AiRoutingDiagnostics {
  configVersion: number;
  providerCount: number;
  enabledProviderCount: number;
  aliasCount: number;
  enabledAliasCount: number;
  bindingCount: number;
  enabledBindingCount: number;
  latestUpdatedAt: string | null;
}

export interface AiRoutingMetric {
  actionKey: string;
  providerName: string;
  modelName: string;
  runCount: number;
  successCount: number;
  failedCount: number;
  fallbackRunCount: number;
  avgLatencyMs: number | null;
  p95LatencyMs: number | null;
  inputTokensSum: number;
  outputTokensSum: number;
  terminalRuntimeFailures: number;
}

export interface AiRoutingDailyCost {
  day: string;
  actionKey: string;
  providerName: string;
  modelName: string;
  runCount: number;
  inputTokensSum: number;
  outputTokensSum: number;
  estimatedCostUsd: number;
  overBudget: boolean;
}

export interface AiRoutingCostAnomaly {
  day: string;
  actionKey: string;
  providerName: string;
  modelName: string;
  todayEstimatedCostUsd: number;
  trailingAvgCostUsd: number;
  deltaPercent: number;
  isSpike: boolean;
}

export interface AiRoutingAlertSummary {
  deadLetterCount24h: number;
  runtimeFailureCount24h: number;
  circuitOpenCount24h: number;
  fallbackRunCount24h: number;
}

export interface AiRoutingParity {
  actionKey: string;
  modelAlias: string;
  runCount: number;
  successCount: number;
  failedCount: number;
  successRate: number;
}

export interface AiRoutingOpsSource {
  fetchDiagnostics: () => Promise<AiRoutingDiagnostics>;
  fetchMetrics: (hours: number) => Promise<AiRoutingMetric[]>;
  fetchCost: (days: number) => Promise<AiRoutingDailyCost[]>;
  fetchCostAnomalies: (days: number) => Promise<AiRoutingCostAnomaly[]>;
  fetchAlerts: () => Promise<AiRoutingAlertSummary>;
  fetchParity: (hours: number) => Promise<AiRoutingParity[]>;
}

export interface AiRoutingOpsOptions {
  defaultMetricHours?: number;
  defaultCostDays?: number;
  defaultParityHours?: number;
  defaultAnomalyDays?: number;
}

export interface AiRoutingOpsController {
  diagnostics: AiRoutingDiagnostics | null;
  metrics: AiRoutingMetric[];
  cost: AiRoutingDailyCost[];
  anomalies: AiRoutingCostAnomaly[];
  alerts: AiRoutingAlertSummary | null;
  parity: AiRoutingParity[];
  loading: boolean;
  error: string | null;
  metricHours: number;
  costDays: number;
  parityHours: number;
  anomalyDays: number;
  setMetricHours: (hours: number) => void;
  setCostDays: (days: number) => void;
  setParityHours: (hours: number) => void;
  setAnomalyDays: (days: number) => void;
  refreshAll: () => Promise<void>;
}

export interface AiRoutingAdminMessages {
  refreshLabel?: string;
  applyLabel?: string;
  metricsEmpty?: string;
  anomaliesEmpty?: string;
  parityEmpty?: string;
  costEmpty?: string;
  noSpike?: string;
}

function normalizeError(error: unknown): string {
  if (error instanceof Error) {
    return error.message;
  }
  return "Failed to load AI routing data";
}

export function createAiRoutingOpsController(
  source: AiRoutingOpsSource,
  options: AiRoutingOpsOptions = {}
): AiRoutingOpsController {
  let diagnostics = $state<AiRoutingDiagnostics | null>(null);
  let metrics = $state<AiRoutingMetric[]>([]);
  let cost = $state<AiRoutingDailyCost[]>([]);
  let anomalies = $state<AiRoutingCostAnomaly[]>([]);
  let alerts = $state<AiRoutingAlertSummary | null>(null);
  let parity = $state<AiRoutingParity[]>([]);
  let loading = $state(false);
  let error = $state<string | null>(null);

  let metricHours = $state(options.defaultMetricHours ?? 24);
  let costDays = $state(options.defaultCostDays ?? 7);
  let parityHours = $state(options.defaultParityHours ?? 24);
  let anomalyDays = $state(options.defaultAnomalyDays ?? 7);

  const refreshAll = async () => {
    loading = true;
    error = null;

    const [
      diagnosticsResult,
      metricsResult,
      costResult,
      anomalyResult,
      alertsResult,
      parityResult
    ] = await Promise.allSettled([
      source.fetchDiagnostics(),
      source.fetchMetrics(metricHours),
      source.fetchCost(costDays),
      source.fetchCostAnomalies(anomalyDays),
      source.fetchAlerts(),
      source.fetchParity(parityHours)
    ]);

    if (diagnosticsResult.status === "fulfilled") diagnostics = diagnosticsResult.value;
    if (metricsResult.status === "fulfilled") metrics = metricsResult.value;
    if (costResult.status === "fulfilled") cost = costResult.value;
    if (anomalyResult.status === "fulfilled") anomalies = anomalyResult.value;
    if (alertsResult.status === "fulfilled") alerts = alertsResult.value;
    if (parityResult.status === "fulfilled") parity = parityResult.value;

    const firstRejected = [
      diagnosticsResult,
      metricsResult,
      costResult,
      anomalyResult,
      alertsResult,
      parityResult
    ].find((result) => result.status === "rejected");

    if (firstRejected && firstRejected.status === "rejected") {
      error = normalizeError(firstRejected.reason);
    }

    loading = false;
  };

  return {
    get diagnostics() {
      return diagnostics;
    },
    get metrics() {
      return metrics;
    },
    get cost() {
      return cost;
    },
    get anomalies() {
      return anomalies;
    },
    get alerts() {
      return alerts;
    },
    get parity() {
      return parity;
    },
    get loading() {
      return loading;
    },
    get error() {
      return error;
    },
    get metricHours() {
      return metricHours;
    },
    get costDays() {
      return costDays;
    },
    get parityHours() {
      return parityHours;
    },
    get anomalyDays() {
      return anomalyDays;
    },
    setMetricHours(hours: number) {
      metricHours = Math.max(1, Math.min(24 * 30, Math.floor(hours)));
    },
    setCostDays(days: number) {
      costDays = Math.max(1, Math.min(365, Math.floor(days)));
    },
    setParityHours(hours: number) {
      parityHours = Math.max(1, Math.min(24 * 30, Math.floor(hours)));
    },
    setAnomalyDays(days: number) {
      anomalyDays = Math.max(2, Math.min(90, Math.floor(days)));
    },
    refreshAll
  };
}

export function toPercent(value: number): string {
  return `${(value * 100).toFixed(1)}%`;
}

export function estimateSuccessRate(runCount: number, successCount: number): number {
  if (runCount <= 0) return 0;
  return successCount / runCount;
}

export function maxCostSpike(
  anomalies: AiRoutingCostAnomaly[]
): AiRoutingCostAnomaly | null {
  if (anomalies.length === 0) return null;
  return anomalies.reduce((max, row) =>
    row.deltaPercent > max.deltaPercent ? row : max
  );
}
