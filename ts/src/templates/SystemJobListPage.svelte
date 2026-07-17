<script lang="ts">
  import { getAuthConfig, useAuthenticatedData } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import { default as AdminPill } from "./AdminPill.svelte";
  import { default as EntityListPage } from "./EntityListPage.svelte";
  import { TimeAgo } from "@poodle/svelte";
  import type { QueryParams } from "../client/query";
  import type {
    ListVariantDefinition,
    SystemJobAction,
    SystemJobListItem,
    SystemJobListLoader,
    SystemJobStatsLoader,
    SystemJobStatus
  } from "./template.types";
  import type {
    TableCellValue,
    TableColumn,
    TableRow,
    TableRowAction
  } from "@poodle/svelte";

  interface Props {
    title?: string;
    backHref?: string;
    backLabel?: string;
    detailHref?: (job: SystemJobListItem) => string;
    dataLoader: SystemJobListLoader;
    statsLoader?: SystemJobStatsLoader;
    retryAction?: SystemJobAction;
    cancelAction?: SystemJobAction;
    formatStatusLabel?: (status: string) => string;
    statusAccent?: (status: string) => string | null | undefined;
    statusTone?: (status: string) => "neutral" | "success" | "warning" | "danger" | "info";
    navigate?: (href: string) => unknown;
    query: QueryParams;
    onQueryChange: (query: QueryParams) => void;
  }

  let {
    title = "Job Queue",
    backHref = "/system",
    backLabel = "Back to system",
    detailHref = defaultDetailHref,
    dataLoader,
    statsLoader,
    retryAction,
    cancelAction,
    formatStatusLabel = getStatusLabel,
    statusAccent = undefined,
    statusTone = getStatusTone,
    navigate = defaultNavigate,
    query,
    onQueryChange
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  let refreshRevision = $state(0);

  function defaultDetailHref(job: SystemJobListItem): string {
    return `/system/jobs/${encodeURIComponent(job.id)}`;
  }

  const statsData = useAuthenticatedData(loadStats, {
    defaultValue: null,
    queryKey: () => `system-jobs-stats:${refreshRevision}`
  });

  const stats = $derived(statsData.data ?? null);

  async function loadStats(fetch: typeof globalThis.fetch, token: unknown) {
    void refreshRevision;
    return statsLoader && typeof token === "string" ? await statsLoader(fetch, token) : null;
  }

  function defaultNavigate(href: string): void {
    if (typeof globalThis.location !== "undefined") {
      globalThis.location.assign(href);
    }
  }

  const columns: TableColumn[] = [
    { id: "jobType", label: "Job Type", width: "minmax(18rem, 2fr)" },
    { id: "status", label: "Status", width: "8rem" },
    { id: "attempts", label: "Attempts", width: "6rem", align: "center" },
    { id: "createdAt", label: "Created", width: "8rem", hideOnMobile: true },
    { id: "finishedAt", label: "Finished", width: "8rem", hideOnMobile: true }
  ];

  const queryVariants = $derived<ListVariantDefinition[]>([
    {
      id: "pending",
      label: "Pending",
      description: "Jobs waiting to run.",
      tone: "warning",
      count: stats?.pending,
      isDefault: true
    },
    {
      id: "claimed",
      label: "Claimed",
      description: "Jobs claimed by a worker.",
      tone: "info"
    },
    {
      id: "running",
      label: "Running",
      description: "Jobs currently running.",
      tone: "info",
      count: stats?.running
    },
    {
      id: "failed",
      label: "Failed",
      description: "Jobs that failed and may need review.",
      tone: "danger",
      count: stats?.failed
    },
    {
      id: "succeeded",
      label: "Succeeded",
      description: "Completed jobs.",
      tone: "success",
      count: stats?.succeeded
    },
    {
      id: "cancelled",
      label: "Cancelled",
      description: "Jobs cancelled before completion.",
      tone: "danger"
    }
  ]);

  function getStatusVariant(nextQuery: QueryParams): SystemJobStatus | undefined {
    if (!nextQuery.variant || nextQuery.variant === "all") {
      return undefined;
    }
    return nextQuery.variant;
  }

  async function loadJobs(fetch: typeof globalThis.fetch, token: unknown, nextQuery: QueryParams) {
    if (typeof token !== "string") throw new Error("Not authenticated");

    return await dataLoader(fetch, token, {
      variant: nextQuery.variant,
      status: getStatusVariant(nextQuery),
      page: nextQuery.page ?? 1,
      limit: nextQuery.limit ?? 30
    });
  }

  function formatJobType(jobType: string): string {
    return jobType
      .replace(/_/g, " ")
      .replace(/\b\w/g, (char) => char.toUpperCase());
  }

  function getStatusTone(status: string): "neutral" | "success" | "warning" | "danger" | "info" {
    switch (status) {
      case "succeeded":
        return "success";
      case "failed":
      case "cancelled":
        return "danger";
      default:
        return "neutral";
    }
  }

  function getStatusLabel(status: string): string {
    return status.replaceAll("_", " ").toLowerCase();
  }

  function getToken() {
    return authConfig?.getToken?.() ?? null;
  }

  async function runJobAction(
    job: SystemJobListItem,
    action: SystemJobAction,
    successMessage: string,
    failureMessage: string
  ) {
    const token = getToken();
    if (!token) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      await action(job, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
      refreshRevision += 1;
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  function rowActions(row: TableRow) {
    const job = rowJob(row);
    if (!job) return [];

    const actions: { value: string; label: string }[] = [
      { value: "view", label: "View details" }
    ];

    if (retryAction && (job.status === "failed" || job.status === "cancelled")) {
      actions.push({ value: "retry", label: "Retry" });
    }

    if (cancelAction && (job.status === "pending" || job.status === "claimed" || job.status === "running")) {
      actions.push({ value: "cancel", label: "Cancel" });
    }

    return actions;
  }

  function handleRowActionSelect(row: TableRow, action: TableRowAction) {
    const job = rowJob(row);
    if (!job) return;

    if (action.value === "view") {
      void navigate(detailHref(job));
      return;
    }

    if (action.value === "retry" && retryAction) {
      runJobAction(job, retryAction, "Job queued for retry", "Failed to retry job");
      return;
    }

    if (action.value === "cancel" && cancelAction) {
      runJobAction(job, cancelAction, "Job cancelled", "Failed to cancel job");
    }
  }

  function rowJob(row: TableRow) {
    return row.data ? row.data as SystemJobListItem : null;
  }
</script>

{#snippet renderCell(column: TableColumn, row: TableRow, value: TableCellValue)}
  {@const job = rowJob(row)}
  {#if !job}
    —
  {:else if column.id === "jobType"}
    <div class="underlay-system-job-list-page__job-cell">
      <span class="underlay-system-job-list-page__job-title">{formatJobType(job.jobType)}</span>
      {#if job.errorMessage}
        <span class="underlay-system-job-list-page__job-error">{job.errorMessage}</span>
      {/if}
    </div>
  {:else if column.id === "status"}
    <AdminPill kind={statusTone(job.status)} label={formatStatusLabel(job.status)} accent={statusAccent?.(job.status) ?? null} typography="label" />
  {:else if column.id === "attempts"}
    {job.attempts}/{job.maxAttempts}
  {:else if column.id === "createdAt"}
    <TimeAgo datetime={job.createdAt} tooltipFormat="datetime" short />
  {:else if column.id === "finishedAt"}
    {#if job.finishedAt}
      <TimeAgo datetime={job.finishedAt} tooltipFormat="datetime" short />
    {:else}
      —
    {/if}
  {:else}
    {value ?? "—"}
  {/if}
{/snippet}

{#key refreshRevision}
  <EntityListPage
    {title}
    {backHref}
    {backLabel}
    dataLoader={loadJobs}
    presentation="table"
    {columns}
    {rowActions}
    {renderCell}
    onRowActionSelect={handleRowActionSelect}
    {queryVariants}
    defaultVariantId="pending"
    {query}
    {onQueryChange}
  />
{/key}

<style>
  .underlay-system-job-list-page__job-cell {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .underlay-system-job-list-page__job-title {
    font-weight: 500;
  }

  .underlay-system-job-list-page__job-error {
    color: var(--admin-color-danger, #ef4444);
    font-size: 0.875rem;
  }
</style>
