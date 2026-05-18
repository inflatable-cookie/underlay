<script lang="ts">
  import { default as EntityDetailPage } from "./EntityDetailPage.svelte";
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
    dataLoader: ErrorLogDetailLoader;
    statusTone?: (statusCode: number) => "neutral" | "success" | "warning" | "danger" | "info";
  }

  let {
    id,
    title = "Error Log",
    section = "Error Log",
    backHref = "/system/errors",
    backLabel = "Back to error log",
    dataLoader,
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
          { label: "", value: statusMeta, separator: false }
        ]
      : []
  );
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
    <Code inline inlineVariant="plain" typography="inline" source={errorLog.id} showCopyButton />
  {/if}
{/snippet}

{#snippet statusMeta()}
  {#if errorLog}
    <Pill tone={statusTone(errorLog.statusCode)} appearance="badge" size="sm" typography="inherit">
      {errorLog.statusCode}
    </Pill>
  {/if}
{/snippet}

{#snippet headerActions()}
  <IconButton
    variant="secondary"
    icon="refresh-cw"
    ariaLabel="Refresh error log"
    tooltip="Refresh"
    onClick={() => {
      reloadKey += 1;
    }}
  />
{/snippet}

{#snippet content(loaded)}
  <div class="underlay-error-log-detail-page">
    <Card>
      <div class="underlay-error-log-detail-page__grid">
        <DetailSection title="Request" columns={2} separated={false}>
          <DetailItem presentation="surface" label="Method">
            {#snippet valueContent()}
              <Code inline source={loaded.method} />
            {/snippet}
          </DetailItem>
          <DetailItem presentation="surface" label="Status" value={String(loaded.statusCode)} />
          <DetailItem presentation="surface" label="Endpoint">
            {#snippet valueContent()}
              <Code inline source={loaded.endpoint} />
            {/snippet}
          </DetailItem>
          <DetailItem
            presentation="surface"
            label="Occurred"
            value={formatDisplayDateTime(loaded.occurredAt) || "-"}
          />
        </DetailSection>

        <DetailSection title="Error" columns={2} separated={false}>
          <DetailItem presentation="surface" label="Code">
            {#snippet valueContent()}
              <Code inline source={loaded.errorCode} />
            {/snippet}
          </DetailItem>
          <DetailItem presentation="surface" label="Correlation ID">
            {#snippet valueContent()}
              <Code inline source={loaded.correlationId} />
            {/snippet}
          </DetailItem>
        </DetailSection>
      </div>
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

  .underlay-error-log-detail-page__grid {
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
