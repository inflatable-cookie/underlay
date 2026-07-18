<script lang="ts">
  import { default as EntityDetailPage } from "./EntityDetailPage.svelte";
  import { default as AdminPill } from "./AdminPill.svelte";
  import { default as EntityActionsMenu } from "./EntityActionsMenu.svelte";
  import type {
    DetailMetaItemConfig,
    ErrorLogDetailItem,
    ErrorLogDetailLoader
  } from "./template.types";
  import {
    Card,
    Code,
    DetailItem,
    DetailSection,
    DetailSectionGroup,
    formatDisplayDateTime
  } from "@poodle/svelte";

  interface Props {
    id: string;
    title?: string;
    section?: string;
    backHref?: string;
    backLabel?: string;
    dataLoader: ErrorLogDetailLoader;
    statusAccent?: (statusCode: number) => string | null | undefined;
    statusTone?: (statusCode: number) => "neutral" | "success" | "warning" | "danger" | "info";
  }

  let {
    id,
    title = "Error Log",
    section = "Error Log",
    backHref = "/system/errors",
    backLabel = "Back to error log",
    dataLoader,
    statusAccent = undefined,
    statusTone = getStatusTone
  }: Props = $props();

  let reloadKey = $state(0);
  let errorLog = $state<ErrorLogDetailItem | null>(null);

  async function loadErrorLog(fetch: typeof globalThis.fetch, token: string | null) {
    if (!token) throw new Error("Not authenticated");
    const result = await dataLoader(id, fetch, token);
    errorLog = result;
    return result;
  }

  function getStatusTone(statusCode: number): "neutral" | "warning" | "danger" {
    if (statusCode >= 500) return "danger";
    if (statusCode >= 400) return "warning";
    return "neutral";
  }

  function formatRequestTitle(item: ErrorLogDetailItem): string {
    return `${item.method} ${item.endpoint}`;
  }

  const meta = $derived<DetailMetaItemConfig[]>(
    errorLog
      ? [
          { label: "ID", value: idMeta },
          { label: "Status", value: statusMeta, separator: false }
        ]
      : []
  );

  const actionItems = $derived([
    {
      label: "Refresh",
      onSelect: () => {
        reloadKey += 1;
      }
    }
  ]);
</script>

<EntityDetailPage
  title={errorLog ? formatRequestTitle(errorLog) : title}
  {section}
  {backHref}
  {backLabel}
  dataLoader={loadErrorLog}
  {reloadKey}
  {meta}
  {headerActions}
  {content}
/>

{#snippet idMeta()}
  {#if errorLog}
    <Code inline inlineVariant="plain" typography="inline" source={errorLog.id} showCopyButton size="md" />
  {/if}
{/snippet}

{#snippet statusMeta()}
  {#if errorLog}
    <AdminPill kind={statusTone(errorLog.statusCode)} label={errorLog.statusCode} accent={statusAccent?.(errorLog.statusCode) ?? null} typography="inherit" />
  {/if}
{/snippet}

{#snippet headerActions()}
  <EntityActionsMenu
    triggerAriaLabel="Error log actions"
    customActions={actionItems}
    copies={errorLog ? [{ label: "Copy error log ID", text: errorLog.id, successMessage: "Copied error log ID" }] : []}
  />
{/snippet}

{#snippet content(loaded: ErrorLogDetailItem)}
  <div class="underlay-error-log-detail-page">
    <Card>
      <DetailSectionGroup ariaLabel="Error log details">
        <DetailSection columns={2} separated={false}>
          <DetailItem presentation="surface" label="Method">
            {#snippet valueContent()}
              <Code inline inlineVariant="plain" typography="inline" size="md" source={loaded.method} />
            {/snippet}
          </DetailItem>
          <DetailItem presentation="surface" label="Status" value={String(loaded.statusCode)} />
          <DetailItem presentation="surface" label="Endpoint">
            {#snippet valueContent()}
              <Code inline inlineVariant="plain" typography="inline" size="md" source={loaded.endpoint} />
            {/snippet}
          </DetailItem>
          <DetailItem
            presentation="surface"
            label="Occurred"
            value={formatDisplayDateTime(loaded.occurredAt) || "-"}
          />
        </DetailSection>

        <DetailSection columns={2} separated={false}>
          <DetailItem presentation="surface" label="Code">
            {#snippet valueContent()}
              <Code inline inlineVariant="plain" typography="inline" size="md" source={loaded.errorCode} />
            {/snippet}
          </DetailItem>
          <DetailItem presentation="surface" label="Correlation ID">
            {#snippet valueContent()}
              <Code inline inlineVariant="plain" typography="inline" size="md" source={loaded.correlationId} />
            {/snippet}
          </DetailItem>
        </DetailSection>
      </DetailSectionGroup>
    </Card>

    {#if loaded.message}
      <Card>
        <div class="underlay-error-log-detail-page__section">
          <h3>Message</h3>
          <p class="underlay-error-log-detail-page__message">{loaded.message}</p>
        </div>
      </Card>
    {/if}

    <Card>
      <div class="underlay-error-log-detail-page__section">
        <h3>Context</h3>
        <pre>{JSON.stringify(loaded.context ?? null, null, 2)}</pre>
      </div>
    </Card>
  </div>
{/snippet}

<style>
  .underlay-error-log-detail-page {
    display: grid;
    gap: 1rem;
  }

  .underlay-error-log-detail-page__section {
    display: grid;
    gap: 0.75rem;
  }

  .underlay-error-log-detail-page__section h3 {
    margin: 0;
    font-size: 1rem;
    font-weight: 600;
  }

  .underlay-error-log-detail-page__message {
    margin: 0;
    white-space: pre-wrap;
    word-break: break-word;
  }

  .underlay-error-log-detail-page pre {
    margin: 0;
    padding: 1rem;
    overflow: auto;
    border-radius: 0.75rem;
    background: var(--poodle-color-surface-secondary, rgba(255, 255, 255, 0.04));
    font-size: 0.875rem;
  }
</style>
