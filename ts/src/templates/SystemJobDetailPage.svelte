<script lang="ts">
  import { getAuthConfig } from "../runtime/auth";
  import { useToasts } from "../runtime/feedback";
  import { default as EntityDetailPage } from "./EntityDetailPage.svelte";
  import type {
    DetailMetaItemConfig,
    SystemJobAction,
    SystemJobDetailItem,
    SystemJobDetailLoader,
    TemplateSurface
  } from "./template.types";
  import {
    Button,
    Card,
    Code,
    DetailItem,
    DetailSection,
    IconButton,
    Pill,
    formatDisplayDateTime
  } from "@poodle/svelte";

  interface Props {
    id: string;
    title?: string;
    section?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: SystemJobDetailLoader;
    retryAction?: SystemJobAction<SystemJobDetailItem>;
    cancelAction?: SystemJobAction<SystemJobDetailItem>;
    formatTitle?: (job: SystemJobDetailItem) => string;
    statusTone?: (status: string) => "neutral" | "success" | "warning" | "danger" | "info";
    extraDetails?: TemplateSurface;
  }

  let {
    id,
    title = "Job",
    section = "Job",
    backHref = "/system/jobs",
    backLabel = "Back to jobs",
    dataLoader,
    retryAction,
    cancelAction,
    formatTitle = defaultTitle,
    statusTone = getStatusTone,
    extraDetails = undefined
  }: Props = $props();

  const toastStore = useToasts();
  const authConfig = getAuthConfig();

  let job = $state<SystemJobDetailItem | null>(null);
  let reloadKey = $state(0);

  async function loadJob(fetch: typeof globalThis.fetch, token: string | null) {
    if (!token) throw new Error("Not authenticated");
    const result = await dataLoader(id, fetch, token);
    job = result;
    return result;
  }

  function formatJobType(jobType: string): string {
    return jobType
      .replace(/_/g, " ")
      .replace(/\b\w/g, (char) => char.toUpperCase());
  }

  function defaultTitle(nextJob: SystemJobDetailItem): string {
    return formatJobType(nextJob.jobType);
  }

  function getStatusLabel(status: string): string {
    return status.charAt(0).toUpperCase() + status.slice(1);
  }

  function getStatusTone(status: string): "neutral" | "success" | "warning" | "danger" {
    if (status === "succeeded" || status === "completed") return "success";
    if (status === "failed" || status === "cancelled") return "danger";
    if (status === "running" || status === "claimed" || status === "pending" || status === "scheduled") return "warning";
    return "neutral";
  }

  function canCancel(status: string): boolean {
    return status === "pending" || status === "claimed" || status === "running";
  }

  function canRetry(status: string): boolean {
    return status === "failed" || status === "cancelled";
  }

  async function runAction(
    action: SystemJobAction<SystemJobDetailItem>,
    successMessage: string,
    failureMessage: string
  ): Promise<void> {
    const token = authConfig?.getToken?.() ?? null;
    if (!token || !job) {
      toastStore.push({ variant: "error", message: "Not authenticated" });
      return;
    }

    try {
      await action(job, fetch, token);
      toastStore.push({ variant: "success", message: successMessage });
      reloadKey += 1;
    } catch (error) {
      const message = error instanceof Error ? error.message : failureMessage;
      toastStore.push({ variant: "error", message });
    }
  }

  const meta = $derived<DetailMetaItemConfig[]>(
    job
      ? [
          { label: "ID", value: idMeta },
          { label: "", value: statusMeta, separator: false }
        ]
      : []
  );
</script>

<EntityDetailPage
  title={job ? formatTitle(job) : title}
  {section}
  {backHref}
  {backLabel}
  dataLoader={loadJob}
  {reloadKey}
  meta={meta}
  {headerActions}
  {content}
/>

{#snippet idMeta()}
  {#if job}
    <Code inline inlineVariant="plain" typography="inline" source={job.id} showCopyButton />
  {/if}
{/snippet}

{#snippet statusMeta()}
  {#if job}
    <Pill tone={statusTone(job.status)} appearance="badge" size="sm" typography="inherit">
      {getStatusLabel(job.status)}
    </Pill>
  {/if}
{/snippet}

{#snippet headerActions()}
  {#if job}
    <div class="underlay-system-job-detail-page__actions">
      {#if cancelAction && canCancel(job.status)}
        <Button
          variant="secondary"
          onClick={() => runAction(cancelAction, "Job cancelled", "Failed to cancel job")}
        >
          Cancel
        </Button>
      {/if}
      {#if retryAction && canRetry(job.status)}
        <Button
          variant="primary"
          onClick={() => runAction(retryAction, "Job queued for retry", "Failed to retry job")}
        >
          Retry
        </Button>
      {/if}
      <IconButton
        variant="secondary"
        icon="refresh-cw"
        ariaLabel="Refresh job"
        tooltip="Refresh"
        onClick={() => {
          reloadKey += 1;
        }}
      />
    </div>
  {/if}
{/snippet}

{#snippet content(loaded)}
  <div class="underlay-system-job-detail-page">
    <Card>
      <div class="underlay-system-job-detail-page__grid">
        <DetailSection title="Details" columns={2} separated={false}>
          <DetailItem presentation="surface" label="Type">
            {#snippet valueContent()}
              <Code inline source={loaded.jobType} />
            {/snippet}
          </DetailItem>
          <DetailItem presentation="surface" label="Attempts" value={`${loaded.attempts} / ${loaded.maxAttempts}`} />
          {#if loaded.priority !== undefined && loaded.priority !== null}
            <DetailItem presentation="surface" label="Priority" value={String(loaded.priority)} />
          {/if}
        </DetailSection>

        <DetailSection title="Timestamps" columns={2} separated={false}>
          <DetailItem presentation="surface" label="Created" value={formatDisplayDateTime(loaded.createdAt) || "-"} />
          {#if loaded.scheduledFor !== undefined}
            <DetailItem presentation="surface" label="Scheduled for" value={formatDisplayDateTime(loaded.scheduledFor) || "-"} />
          {/if}
          {#if loaded.claimedAt !== undefined}
            <DetailItem presentation="surface" label="Claimed at" value={formatDisplayDateTime(loaded.claimedAt) || "-"} />
          {/if}
          {#if loaded.startedAt !== undefined}
            <DetailItem presentation="surface" label="Started at" value={formatDisplayDateTime(loaded.startedAt) || "-"} />
          {/if}
          {#if loaded.finishedAt !== undefined}
            <DetailItem presentation="surface" label="Finished at" value={formatDisplayDateTime(loaded.finishedAt) || "-"} />
          {/if}
          {#if loaded.heartbeatAt !== undefined}
            <DetailItem presentation="surface" label="Last heartbeat" value={formatDisplayDateTime(loaded.heartbeatAt) || "-"} />
          {/if}
        </DetailSection>
      </div>
    </Card>

    {#if loaded.claimedBy}
      <Card>
        <div class="underlay-system-job-detail-page__section">
          <h3>Worker</h3>
          <p>{loaded.claimedBy}</p>
        </div>
      </Card>
    {/if}

    {#if loaded.errorMessage || loaded.lastError}
      <Card>
        <div class="underlay-system-job-detail-page__section underlay-system-job-detail-page__section--error">
          <h3>{loaded.lastError ? "Last Error" : "Error"}</h3>
          <p class="underlay-system-job-detail-page__error">{loaded.lastError ?? loaded.errorMessage}</p>
        </div>
      </Card>
    {/if}

    <Card>
      <div class="underlay-system-job-detail-page__section">
        <h3>Payload</h3>
        <pre>{JSON.stringify(loaded.payload ?? null, null, 2)}</pre>
      </div>
    </Card>

    {#if loaded.progress}
      <Card>
        <div class="underlay-system-job-detail-page__section">
          <h3>Progress</h3>
          <pre>{JSON.stringify(loaded.progress, null, 2)}</pre>
        </div>
      </Card>
    {/if}

    {#if loaded.errorHistory}
      <Card>
        <div class="underlay-system-job-detail-page__section">
          <h3>Error History</h3>
          <pre>{JSON.stringify(loaded.errorHistory, null, 2)}</pre>
        </div>
      </Card>
    {/if}

    {#if extraDetails}
      {@render extraDetails(loaded)}
    {/if}
  </div>
{/snippet}

<style>
  .underlay-system-job-detail-page,
  .underlay-system-job-detail-page__grid {
    display: grid;
    gap: 1rem;
  }

  .underlay-system-job-detail-page__actions {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .underlay-system-job-detail-page__section {
    padding: 0.5rem;
  }

  .underlay-system-job-detail-page__section h3 {
    margin: 0 0 0.75rem;
    color: var(--poodle-color-text-muted, var(--admin-color-text-muted, #94a3b8));
    font-size: 0.75rem;
    font-weight: 600;
    letter-spacing: 0.05em;
    text-transform: uppercase;
  }

  .underlay-system-job-detail-page__section p {
    margin: 0;
  }

  .underlay-system-job-detail-page__section--error {
    border-left: 3px solid var(--poodle-color-danger, var(--admin-color-danger, #ef4444));
    padding-left: 1rem;
  }

  .underlay-system-job-detail-page__error {
    color: var(--poodle-color-danger, var(--admin-color-danger, #ef4444));
    font-family: monospace;
    font-size: 0.875rem;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .underlay-system-job-detail-page pre {
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
