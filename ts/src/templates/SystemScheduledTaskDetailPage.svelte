<script lang="ts">
  import { getAuthConfig } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import { default as AdminPill } from "./AdminPill.svelte";
  import { default as EntityActionsMenu } from "./EntityActionsMenu.svelte";
  import { default as EntityDetailPage } from "./EntityDetailPage.svelte";
  import type {
    DetailMetaItemConfig,
    DetailTabConfig,
    SystemJobListItem,
    SystemScheduledTaskAction,
    SystemScheduledTaskDetailItem,
    SystemScheduledTaskDetailLoader,
    SystemScheduledTaskJobRunsLoader
  } from "./template.types";
  import {
    Card,
    Code,
    DataTable,
    DetailItem,
    DetailSection,
    TimeAgo,
    formatDisplayDateTime,
    type TableColumn,
    type TableRow
  } from "@poodle/svelte";

  interface Props {
    id: string;
    title?: string;
    section?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: SystemScheduledTaskDetailLoader;
    jobRunsLoader?: SystemScheduledTaskJobRunsLoader;
    triggerAction?: SystemScheduledTaskAction<SystemScheduledTaskDetailItem>;
    toggleAction?: SystemScheduledTaskAction<SystemScheduledTaskDetailItem>;
    formatTitle?: (task: SystemScheduledTaskDetailItem) => string;
    navigateToJob?: (job: SystemJobListItem) => unknown;
    navigate?: (href: string) => unknown;
  }

  let {
    id,
    title = "Scheduled Task",
    section = "Scheduled Task",
    backHref = "/system/scheduled-tasks",
    backLabel = "Back to tasks",
    dataLoader,
    jobRunsLoader = undefined,
    triggerAction = undefined,
    toggleAction = undefined,
    formatTitle = defaultTitle,
    navigateToJob = undefined,
    navigate = defaultNavigate
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  let task = $state<SystemScheduledTaskDetailItem | null>(null);
  let activeTab = $state("details");
  let reloadKey = $state(0);
  let jobs = $state<SystemJobListItem[]>([]);
  let jobsLoading = $state(false);
  let jobsLoaded = $state(false);
  let jobsError = $state<string | null>(null);

  async function loadTask(fetch: typeof globalThis.fetch, token: string | null) {
    if (!token) throw new Error("Not authenticated");
    const result = await dataLoader(id, fetch, token);
    task = result;
    jobsLoaded = false;
    return result;
  }

  async function loadJobRuns(force = false): Promise<void> {
    if (!task || !jobRunsLoader || jobsLoading || (!force && jobsLoaded)) return;

    const token = authConfig?.getToken?.() ?? null;
    if (!token) return;

    jobsLoading = true;
    jobsError = null;
    try {
      const result = await jobRunsLoader(task, fetch, token);
      jobs = Array.isArray(result) ? result : result.data;
      jobsLoaded = true;
    } catch (error) {
      jobsError = error instanceof Error ? error.message : "Failed to load job runs";
    } finally {
      jobsLoading = false;
    }
  }

  $effect(() => {
    if (activeTab === "job-runs") {
      void loadJobRuns();
    }
  });

  function formatName(value: string): string {
    return value.replace(/_/g, " ").replace(/\b\w/g, (char) => char.toUpperCase());
  }

  function defaultTitle(nextTask: SystemScheduledTaskDetailItem): string {
    return formatName(nextTask.name);
  }

  function defaultNavigate(href: string): void {
    if (typeof globalThis.location !== "undefined") {
      globalThis.location.assign(href);
    }
  }

  function openJob(job: SystemJobListItem): void {
    if (navigateToJob) {
      navigateToJob(job);
      return;
    }

    navigate(`/system/jobs/${encodeURIComponent(job.id)}`);
  }

  function rowJob(row: TableRow): SystemJobListItem | null {
    return (row.data as SystemJobListItem | undefined) ?? null;
  }

  function openRowJob(row: TableRow): void {
    const job = rowJob(row);
    if (job) {
      openJob(job);
    }
  }

  function describeSchedule(schedule: string): string {
    if (schedule === "0 */15 * * * *") return "Every 15 minutes";
    if (schedule === "0 0 * * * *") return "Every hour at :00";
    if (schedule === "0 */5 * * * *") return "Every 5 minutes";
    if (schedule.match(/^0 \d+ \* \* \* \*$/)) return `Every hour at :${schedule.split(" ")[1]}`;
    if (schedule.match(/^0 0 \d+ \* \* \*$/)) return `Daily at ${schedule.split(" ")[2]}:00`;
    if (schedule.includes("* * 0")) return "Weekly on Sunday";
    return `Cron: ${schedule}`;
  }

  function getJobStatusTone(status: string): "neutral" | "success" | "warning" | "danger" | "info" {
    if (status === "succeeded" || status === "completed") return "success";
    if (status === "failed" || status === "cancelled") return "danger";
    if (status === "running" || status === "claimed") return "info";
    if (status === "pending" || status === "scheduled") return "warning";
    return "neutral";
  }

  function getStatusLabel(status: string): string {
    return status.replaceAll("_", " ").toLowerCase();
  }

  async function runTaskAction(
    action: SystemScheduledTaskAction<SystemScheduledTaskDetailItem>,
    successMessage: string,
    failureMessage: string,
    navigateToCreatedJob = false
  ): Promise<void> {
    const token = authConfig?.getToken?.() ?? null;
    if (!token || !task) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      const result = await action(task, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
      jobsLoaded = false;
      reloadKey += 1;

      if (navigateToCreatedJob && result && "jobId" in result && result.jobId) {
        navigate(`/system/jobs/${encodeURIComponent(result.jobId)}`);
      }
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  function refresh(): void {
    jobsLoaded = false;
    reloadKey += 1;
    if (activeTab === "job-runs") {
      void loadJobRuns(true);
    }
  }

  const actionItems = $derived.by(() => {
    const currentTask = task;
    if (!currentTask) return [];

    return [
      ...(triggerAction
        ? [{
            label: "Trigger now",
            onSelect: () => runTaskAction(triggerAction, "Job created", "Failed to trigger task", true)
          }]
        : []),
      ...(toggleAction
        ? [{
            label: currentTask.enabled ? "Disable task" : "Enable task",
            onSelect: () => runTaskAction(
              toggleAction,
              currentTask.enabled ? "Task disabled" : "Task enabled",
              "Failed to toggle task"
            )
          }]
        : []),
      {
        label: "Refresh",
        onSelect: refresh
      }
    ];
  });

  const meta = $derived<DetailMetaItemConfig[]>(
    task
      ? [
          { label: "ID", value: idMeta },
          { label: "Status", value: enabledMeta, separator: false }
        ]
      : []
  );

  const tabs = $derived<DetailTabConfig<SystemScheduledTaskDetailItem>[]>(
    [
      { id: "details", label: "Details", content: detailsTab },
      ...(jobRunsLoader ? [{ id: "job-runs", label: "Job Runs", content: jobRunsTab }] : [])
    ]
  );

  const jobColumns: TableColumn[] = [
    { id: "status", label: "Status", width: "minmax(100px, 1fr)" },
    { id: "attempts", label: "Attempts", width: "minmax(80px, 1fr)", align: "center" },
    { id: "createdAt", label: "Created", width: "minmax(120px, 1fr)" },
    { id: "finishedAt", label: "Finished", width: "minmax(120px, 1fr)" }
  ];

  const jobRows = $derived<TableRow<SystemJobListItem>[]>(
    jobs.map((job) => ({
      id: job.id,
      cells: {
        status: getStatusLabel(job.status),
        attempts: `${job.attempts}/${job.maxAttempts}`,
        createdAt: job.createdAt,
        finishedAt: job.finishedAt ?? "—"
      },
      data: job
    }))
  );

</script>

<EntityDetailPage
  title={task ? formatTitle(task) : title}
  {section}
  {backHref}
  {backLabel}
  dataLoader={loadTask}
  {reloadKey}
  meta={meta}
  tabs={tabs}
  tabsSize="sm"
  keepMountedTabs
  onTabChange={(tabId) => {
    activeTab = tabId;
  }}
  {headerActions}
/>

{#snippet idMeta()}
  {#if task}
    <Code inline inlineVariant="plain" typography="inline" source={task.id} showCopyButton size="md" />
  {/if}
{/snippet}

{#snippet enabledMeta()}
  {#if task}
    <AdminPill kind={task.enabled ? "success" : "neutral"} label={task.enabled ? "enabled" : "disabled"} typography="inherit" />
  {/if}
{/snippet}

{#snippet headerActions()}
  {#if task}
    <EntityActionsMenu
      toastStore={toastStore}
      triggerAriaLabel="Task actions"
      customActions={actionItems}
      copies={[{ label: "Copy task ID", text: task.id, successMessage: "Copied task ID" }]}
    />
  {/if}
{/snippet}

{#snippet detailsTab(loaded)}
  <div class="underlay-system-scheduled-task-detail-page">
    <div class="underlay-system-scheduled-task-detail-page__grid">
      <Card>
        <div class="underlay-system-scheduled-task-detail-page__section">
          <DetailSection columns={2} separated={false} ariaLabel="Configuration">
            <DetailItem presentation="surface" label="Name" value={formatName(loaded.name)} />
            <DetailItem presentation="surface" label="Job Type" value={formatName(loaded.jobType)} />
            <DetailItem presentation="surface" label="Schedule">
              {#snippet valueContent()}
                <Code inline inlineVariant="plain" typography="inline" size="md" source={loaded.schedule} />
              {/snippet}
            </DetailItem>
            {#if loaded.priority !== undefined && loaded.priority !== null}
              <DetailItem presentation="surface" label="Priority" value={String(loaded.priority)} />
            {/if}
            {#if loaded.maxAttempts !== undefined && loaded.maxAttempts !== null}
              <DetailItem presentation="surface" label="Max Attempts" value={String(loaded.maxAttempts)} />
            {/if}
            {#if loaded.timeoutSeconds !== undefined}
              <DetailItem presentation="surface" label="Timeout" value={loaded.timeoutSeconds ? `${loaded.timeoutSeconds}s` : "None"} />
            {/if}
            {#if loaded.allowOverlap !== undefined}
              <DetailItem presentation="surface" label="Allow Overlap" value={loaded.allowOverlap ? "Yes" : "No"} />
            {/if}
          </DetailSection>
        </div>
      </Card>

      <Card>
        <div class="underlay-system-scheduled-task-detail-page__section">
          <DetailSection columns={2} separated={false} ariaLabel="Execution history">
            <DetailItem presentation="surface" label="Last Scheduled" value={formatDisplayDateTime(loaded.lastScheduledAt) || "Never"} />
            <DetailItem presentation="surface" label="Last Completed" value={formatDisplayDateTime(loaded.lastCompletedAt) || "Never"} />
            {#if loaded.createdAt !== undefined}
              <DetailItem presentation="surface" label="Created">
                <TimeAgo datetime={loaded.createdAt} />
              </DetailItem>
            {/if}
            {#if loaded.updatedAt !== undefined}
              <DetailItem presentation="surface" label="Last Updated">
                <TimeAgo datetime={loaded.updatedAt} />
              </DetailItem>
            {/if}
          </DetailSection>
        </div>
      </Card>
    </div>

    <Card>
      <div class="underlay-system-scheduled-task-detail-page__section">
        <h3>Schedule</h3>
        <p>{describeSchedule(loaded.schedule)}</p>
      </div>
    </Card>

    <Card>
      <div class="underlay-system-scheduled-task-detail-page__section">
        <h3>Payload</h3>
        <pre>{JSON.stringify(loaded.payload ?? null, null, 2)}</pre>
      </div>
    </Card>
  </div>
{/snippet}

{#snippet jobRunsTab(_loaded)}
  {#if jobsError}
    <Card>
      <div class="underlay-system-scheduled-task-detail-page__section underlay-system-scheduled-task-detail-page__section--error">
        {jobsError}
      </div>
    </Card>
  {:else}
    <div class="underlay-system-scheduled-task-detail-page__jobs">
      <DataTable
        rows={jobRows}
        columns={jobColumns}
        loading={jobsLoading}
        emptyMessage="No job runs found for this task"
        showLimitSelector={false}
        showRowActions={false}
        onRowClick={({ row }) => openRowJob(row)}
      >
        {#snippet cell(column, row)}
          {@const job = rowJob(row)}
          {#if column.id === "status" && job}
            <AdminPill kind={getJobStatusTone(job.status)} label={getStatusLabel(job.status)} typography="label" />
          {:else if column.id === "createdAt" && job}
            <TimeAgo datetime={job.createdAt} tooltipFormat="datetime" short />
          {:else if column.id === "finishedAt" && job}
            {#if job.finishedAt}
              <TimeAgo datetime={job.finishedAt} tooltipFormat="datetime" short />
            {:else}
              —
            {/if}
          {:else}
            {row.cells[column.id] ?? "—"}
          {/if}
        {/snippet}
      </DataTable>
    </div>
  {/if}
{/snippet}

<style>
  .underlay-system-scheduled-task-detail-page {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .underlay-system-scheduled-task-detail-page__grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
    gap: 1rem;
  }

  .underlay-system-scheduled-task-detail-page__section {
    padding: 0.5rem;
  }

  .underlay-system-scheduled-task-detail-page__section h3 {
    margin: 0 0 0.75rem;
    color: var(--poodle-color-text-muted, var(--admin-color-text-muted, #94a3b8));
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  .underlay-system-scheduled-task-detail-page__section p {
    margin: 0;
  }

  .underlay-system-scheduled-task-detail-page__section--error {
    color: var(--poodle-color-danger, var(--admin-color-danger, #ef4444));
  }

  .underlay-system-scheduled-task-detail-page__jobs {
    border-radius: 0.5rem;
  }

  .underlay-system-scheduled-task-detail-page pre {
    margin: 0;
    padding: 0.75rem;
    overflow-x: auto;
    border-radius: 0.35rem;
    background: var(--poodle-color-surface-subtle, var(--admin-color-surface-subtle, #111827));
    font-size: 0.8rem;
    white-space: pre-wrap;
    word-break: break-word;
  }
</style>
