<script lang="ts">
  import {
    Card as PoodleCard,
    Icon,
    Pill as PoodlePill,
    TimeAgo,
    formatDisplayDateTime,
    type TableColumn,
    type TableRow
  } from "@poodle/svelte";

  import { getAuthConfig } from "../patterns/auth";
  import { useAuthenticatedData } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import type { QueryParams } from "../client/query";
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import type {
    ErrorLogDetailItem,
    ErrorLogDetailLoader,
    ErrorLogListItem,
    ErrorLogListLoader,
    ErrorLogListRequest,
    ErrorLogStatsLoader,
    ErrorLogStatsSummary,
    FetchFn,
    TemplateSurface
  } from "./template.types";

  interface Props {
    title?: string;
    backHref?: string;
    backLabel?: string;
    query?: QueryParams;
    onQueryChange?: (query: QueryParams) => void;
    loadList: ErrorLogListLoader;
    loadDetail: ErrorLogDetailLoader;
    loadStats?: ErrorLogStatsLoader;
  }

  let {
    title = "Error Log",
    backHref = "/system",
    backLabel = "Back to system",
    query,
    onQueryChange,
    loadList,
    loadDetail,
    loadStats
  }: Props = $props();

  const toastStore = useToasts();

  let expandedLogId = $state<string | null>(null);
  let expandedLogDetail = $state<ErrorLogDetailItem | null>(null);
  let loadingDetail = $state(false);

  const columns: TableColumn[] = [
    { id: "expand", label: "", width: "3.5rem", align: "center", hideable: false, isRowHeader: false },
    { id: "occurredAt", label: "Time", width: "8rem" },
    { id: "request", label: "Request", width: "minmax(18rem, 1.15fr)" },
    { id: "error", label: "Error", width: "minmax(20rem, 1.85fr)" }
  ];

  const filters = [
    {
      id: "statusCode",
      type: "select" as const,
      label: "Status",
      options: [
        { value: "All", label: "All statuses" },
        { value: "400", label: "400 Bad Request" },
        { value: "401", label: "401 Unauthorized" },
        { value: "403", label: "403 Forbidden" },
        { value: "404", label: "404 Not Found" },
        { value: "500", label: "500 Server Error" },
        { value: "502", label: "502 Bad Gateway" },
        { value: "503", label: "503 Unavailable" }
      ]
    }
  ];

  async function loadErrorStats(fetch: FetchFn, token: string | null) {
    if (!loadStats || !token) {
      return null;
    }

    return await loadStats(fetch, token);
  }

  const statsData = useAuthenticatedData(
    loadErrorStats,
    {
      defaultValue: null as ErrorLogStatsSummary | null
    }
  );

  const stats = $derived(statsData.data);

  function getStatusCodeFilter(nextQuery: QueryParams): number | undefined {
    const filter = nextQuery.filters?.find((entry) => entry.field === "statusCode");
    if (!filter || filter.value === "" || filter.value === "All") {
      return undefined;
    }

    const parsed = Number(filter.value);
    return Number.isFinite(parsed) ? parsed : undefined;
  }

  async function dataLoader(
    fetchFn: typeof globalThis.fetch,
    token: string | null,
    nextQuery: QueryParams
  ) {
    if (!token) {
      throw new Error("Not authenticated");
    }

    const limit = nextQuery.limit ?? 30;
    const pageNumber = Math.max(1, nextQuery.page ?? 1);
    const offset = (pageNumber - 1) * limit;
    const request: ErrorLogListRequest = {
      statusCode: getStatusCodeFilter(nextQuery),
      limit,
      offset
    };

    return await loadList(fetchFn, token, request);
  }

  function getStatusTone(statusCode: number): "neutral" | "warning" | "danger" {
    if (statusCode >= 500) {
      return "danger";
    }
    if (statusCode >= 400) {
      return "warning";
    }
    return "neutral";
  }

  function getRowLog(row: TableRow<ErrorLogListItem>): ErrorLogListItem | null {
    return row.data ?? null;
  }

  async function toggleDetail(logId: string) {
    if (expandedLogId === logId) {
      expandedLogId = null;
      expandedLogDetail = null;
      return;
    }

    const token = getAuthConfig()?.getToken() ?? null;
    if (!token) {
      return;
    }

    expandedLogId = logId;
    expandedLogDetail = null;
    loadingDetail = true;

    try {
      expandedLogDetail = await loadDetail(logId, fetch, token);
    } catch (error) {
      const message = error instanceof Error ? error.message : "Failed to load error details";
      toastStore.push({ variant: "error", message });
      expandedLogId = null;
    } finally {
      loadingDetail = false;
    }
  }
</script>

{#snippet beforeList()}
  {#if stats}
    <div class="underlay-error-log-list-page__stats-grid">
      <PoodleCard>
        <div class="underlay-error-log-list-page__stat">
          <span class="underlay-error-log-list-page__stat-icon underlay-error-log-list-page__stat-icon--danger">
            <Icon icon="triangle-alert" size="lg" />
          </span>
          <div class="underlay-error-log-list-page__stat-content">
            <span class="underlay-error-log-list-page__stat-value">{stats.totalLast24h}</span>
            <span class="underlay-error-log-list-page__stat-label">Total Errors</span>
          </div>
        </div>
      </PoodleCard>
      <PoodleCard>
        <div class="underlay-error-log-list-page__stat">
          <span class="underlay-error-log-list-page__stat-icon underlay-error-log-list-page__stat-icon--danger">
            <Icon icon="circle-x" size="lg" />
          </span>
          <div class="underlay-error-log-list-page__stat-content">
            <span class="underlay-error-log-list-page__stat-value">{stats.serverErrorsLast24h}</span>
            <span class="underlay-error-log-list-page__stat-label">5xx Errors</span>
          </div>
        </div>
      </PoodleCard>
      <PoodleCard>
        <div class="underlay-error-log-list-page__stat">
          <span class="underlay-error-log-list-page__stat-icon underlay-error-log-list-page__stat-icon--warning">
            <Icon icon="alert-circle" size="lg" />
          </span>
          <div class="underlay-error-log-list-page__stat-content">
            <span class="underlay-error-log-list-page__stat-value">{stats.clientErrorsLast24h}</span>
            <span class="underlay-error-log-list-page__stat-label">4xx Errors</span>
          </div>
        </div>
      </PoodleCard>
    </div>
  {/if}
{/snippet}

{#snippet renderCell(column, row, value)}
  {@const log = getRowLog(row)}
  {#if !log}
    —
  {:else if column.id === "expand"}
    <button
      type="button"
      class="underlay-error-log-list-page__expand-btn"
      onclick={() => toggleDetail(row.id)}
      aria-label={expandedLogId === row.id ? "Collapse error details" : "Expand error details"}
      aria-expanded={expandedLogId === row.id}
    >
      <Icon icon={expandedLogId === row.id ? "chevron-up" : "chevron-down"} size="sm" />
    </button>
  {:else if column.id === "occurredAt"}
    <span class="underlay-error-log-list-page__time">
      <TimeAgo datetime={log.occurredAt} tooltipFormat="datetime" short />
    </span>
  {:else if column.id === "request"}
    <div class="underlay-error-log-list-page__request">
      <div class="underlay-error-log-list-page__request-meta">
        <PoodlePill tone={getStatusTone(log.statusCode)} appearance="badge" size="sm">
          {log.statusCode}
        </PoodlePill>
        <code class="underlay-error-log-list-page__method">{log.method}</code>
      </div>
      <code class="underlay-error-log-list-page__path">{log.endpoint}</code>
    </div>
  {:else if column.id === "error"}
    <div class="underlay-error-log-list-page__error-summary">
      <code class="underlay-error-log-list-page__error-code">{log.errorCode}</code>
      <span class="underlay-error-log-list-page__message">{log.message || "—"}</span>
    </div>
  {:else}
    {value ?? "—"}
  {/if}
{/snippet}

{#snippet renderExpandedRow(row)}
  {@const log = getRowLog(row)}
  {#if log && expandedLogId === log.id}
    {#if loadingDetail}
      <div class="underlay-error-log-list-page__detail-loading">Loading details...</div>
    {:else if expandedLogDetail}
      <div class="underlay-error-log-list-page__detail-content">
        <div class="underlay-error-log-list-page__detail-grid">
          <div class="underlay-error-log-list-page__detail-item">
            <span class="underlay-error-log-list-page__detail-label">Status</span>
            <span class="underlay-error-log-list-page__detail-value">{expandedLogDetail.statusCode}</span>
          </div>
          <div class="underlay-error-log-list-page__detail-item">
            <span class="underlay-error-log-list-page__detail-label">Full Timestamp</span>
            <span class="underlay-error-log-list-page__detail-value">{formatDisplayDateTime(expandedLogDetail.occurredAt)}</span>
          </div>
          <div class="underlay-error-log-list-page__detail-item">
            <span class="underlay-error-log-list-page__detail-label">Correlation ID</span>
            <code class="underlay-error-log-list-page__detail-value underlay-error-log-list-page__correlation-id">{expandedLogDetail.correlationId}</code>
          </div>
          <div class="underlay-error-log-list-page__detail-item">
            <span class="underlay-error-log-list-page__detail-label">Full Endpoint</span>
            <code class="underlay-error-log-list-page__detail-value">{expandedLogDetail.method} {expandedLogDetail.endpoint}</code>
          </div>
          <div class="underlay-error-log-list-page__detail-item">
            <span class="underlay-error-log-list-page__detail-label">Error Code</span>
            <code class="underlay-error-log-list-page__detail-value">{expandedLogDetail.errorCode}</code>
          </div>
        </div>
        {#if expandedLogDetail.message}
          <div class="underlay-error-log-list-page__detail-item underlay-error-log-list-page__detail-item--full">
            <span class="underlay-error-log-list-page__detail-label">Message</span>
            <span class="underlay-error-log-list-page__detail-value">{expandedLogDetail.message}</span>
          </div>
        {/if}
        {#if expandedLogDetail.context && Object.keys(expandedLogDetail.context).length > 0}
          <div class="underlay-error-log-list-page__detail-item underlay-error-log-list-page__detail-item--full">
            <span class="underlay-error-log-list-page__detail-label">Context</span>
            <pre class="underlay-error-log-list-page__detail-context">{JSON.stringify(expandedLogDetail.context, null, 2)}</pre>
          </div>
        {/if}
      </div>
    {/if}
  {/if}
{/snippet}

<EntityListPage
  {title}
  {backHref}
  {backLabel}
  {query}
  {onQueryChange}
  dataLoader={dataLoader}
  presentation="table"
  {columns}
  {filters}
  expandedRowIds={expandedLogId ? [expandedLogId] : []}
  showRowActions={false}
  beforeList={beforeList as TemplateSurface}
  renderCell={renderCell as TemplateSurface}
  renderExpandedRow={renderExpandedRow as TemplateSurface}
/>

<style>
  .underlay-error-log-list-page__stats-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(10rem, 1fr));
    gap: 1rem;
  }

  .underlay-error-log-list-page__stat {
    display: flex;
    align-items: center;
    gap: 1rem;
    padding: 0.5rem;
  }

  .underlay-error-log-list-page__stat-icon {
    display: inline-flex;
    flex-shrink: 0;
  }

  .underlay-error-log-list-page__stat-icon--danger {
    color: var(--poodle-color-status-danger);
  }

  .underlay-error-log-list-page__stat-icon--warning {
    color: var(--poodle-color-status-warning);
  }

  .underlay-error-log-list-page__stat-content {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
  }

  .underlay-error-log-list-page__stat-value {
    color: var(--poodle-color-text-primary);
    font-size: 1.5rem;
    font-weight: 600;
  }

  .underlay-error-log-list-page__stat-label {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .underlay-error-log-list-page__expand-btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 2rem;
    height: 2rem;
    border: 0;
    border-radius: 999px;
    background: transparent;
    color: var(--poodle-color-text-secondary);
    cursor: pointer;
  }

  .underlay-error-log-list-page__expand-btn:hover,
  .underlay-error-log-list-page__expand-btn:focus-visible {
    background: color-mix(in srgb, var(--poodle-color-accent-base) 12%, transparent);
    color: var(--poodle-color-text-primary);
    outline: none;
  }

  .underlay-error-log-list-page__time {
    color: var(--poodle-color-text-secondary);
  }

  .underlay-error-log-list-page__request,
  .underlay-error-log-list-page__error-summary {
    display: flex;
    flex-direction: column;
    gap: 0.35rem;
    min-width: 0;
  }

  .underlay-error-log-list-page__request-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .underlay-error-log-list-page__method,
  .underlay-error-log-list-page__path,
  .underlay-error-log-list-page__error-code,
  .underlay-error-log-list-page__correlation-id,
  .underlay-error-log-list-page__detail-context {
    font-family: var(--poodle-typography-code-family);
    font-size: 0.75rem;
  }

  .underlay-error-log-list-page__method {
    color: var(--poodle-color-text-secondary);
    font-weight: 600;
  }

  .underlay-error-log-list-page__path,
  .underlay-error-log-list-page__error-code,
  .underlay-error-log-list-page__correlation-id {
    overflow-wrap: anywhere;
  }

  .underlay-error-log-list-page__message {
    display: -webkit-box;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: normal;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    color: var(--poodle-color-text-secondary);
  }

  .underlay-error-log-list-page__detail-loading {
    padding: 0.25rem 0;
    color: var(--poodle-color-text-secondary);
  }

  .underlay-error-log-list-page__detail-content {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .underlay-error-log-list-page__detail-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(14rem, 1fr));
    gap: 0.75rem 1rem;
  }

  .underlay-error-log-list-page__detail-item {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 0;
  }

  .underlay-error-log-list-page__detail-item--full {
    width: 100%;
  }

  .underlay-error-log-list-page__detail-label {
    color: var(--poodle-color-text-secondary);
    font-size: 0.75rem;
    letter-spacing: 0.04em;
    text-transform: uppercase;
  }

  .underlay-error-log-list-page__detail-value {
    color: var(--poodle-color-text-primary);
    min-width: 0;
  }

  .underlay-error-log-list-page__detail-context {
    margin: 0;
    padding: 0.75rem;
    border-radius: var(--poodle-radius-surface);
    background: color-mix(in srgb, var(--poodle-color-background-elevated) 94%, transparent);
    color: var(--poodle-color-text-primary);
    line-height: 1.5;
    overflow-x: auto;
  }
</style>
