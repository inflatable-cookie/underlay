<script lang="ts" generics="T = unknown">
  import {
    Callout,
    EmptyState,
    ListGrid,
    PageHeader,
    PageLoading
  } from "@inflatable-cookie/poodle-svelte";
  import { useTemplateBackInfo } from "./template-back-info.svelte";
  import type { TemplateSurface } from "./template.types";

  interface Props {
    section?: string;
    title: string;
    subtitle?: string | null;
    eyebrow?: string | null;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    backHref?: string;
    backLabel?: string;
    backIsContextual?: boolean;
    resolveBackContext?: boolean;
    loading?: boolean;
    loadingMessage?: string;
    error?: string | null;
    statusMessage?: string | null;
    statusTone?: "danger" | "info" | "success" | "neutral" | "warning";
    beforeItems?: TemplateSurface;
    items: T[];
    renderItem: TemplateSurface;
    emptyTitle: string;
    emptyMessage: string;
    emptyVisual?: TemplateSurface;
    minItemWidth?: string;
  }

  let {
    section = undefined,
    title,
    subtitle = null,
    eyebrow = null,
    headerLevel = 1,
    backHref = undefined,
    backLabel = undefined,
    backIsContextual = false,
    resolveBackContext = true,
    loading = false,
    loadingMessage = "Loading...",
    error = null,
    statusMessage = null,
    statusTone = "danger",
    beforeItems = undefined,
    items,
    renderItem,
    emptyTitle,
    emptyMessage,
    emptyVisual = undefined,
    minItemWidth = "var(--underlay-list-grid-min, 20rem)"
  }: Props = $props();

  const getResolvedBackInfo = useTemplateBackInfo(() => ({
    href: backHref,
    label: backLabel,
    contextual: backIsContextual,
    resolveContext: resolveBackContext
  }));
  const resolvedBackInfo = $derived(getResolvedBackInfo());

  function getItemKey(item: T, index: number): string | number {
    if (typeof item === "object" && item !== null && "id" in item) {
      const { id } = item;
      if (typeof id === "string" || typeof id === "number") {
        return id;
      }
    }
    return index;
  }
</script>

<PageHeader
  {section}
  {title}
  {subtitle}
  {eyebrow}
  level={headerLevel}
  backHref={resolvedBackInfo?.href ?? backHref ?? null}
  backLabel={resolvedBackInfo?.label ?? backLabel}
  backIsContextual={resolvedBackInfo?.contextual ?? false}
/>

{#if loading}
  <PageLoading message={loadingMessage} />
{:else if error}
  <Callout tone="danger" message={error} announceMode="polite" />
{:else}
  <div class="underlay-entity-trash-page">
    {#if statusMessage}
      <Callout tone={statusTone} message={statusMessage} announceMode="polite" />
    {/if}

    {#if beforeItems}
      {@render beforeItems()}
    {/if}

    {#if items.length === 0}
      <div class="underlay-entity-trash-page__empty">
        {#if emptyVisual}
          {@render emptyVisual()}
        {/if}
        <EmptyState title={emptyTitle} message={emptyMessage} />
      </div>
    {:else}
      <ListGrid minItemWidth={minItemWidth}>
        {#each items as item, index (getItemKey(item, index))}
          {@render renderItem(item)}
        {/each}
      </ListGrid>
    {/if}
  </div>
{/if}

<style>
  .underlay-entity-trash-page {
    display: grid;
    gap: 1.5rem;
  }

  .underlay-entity-trash-page__empty {
    display: grid;
    gap: 1rem;
    justify-items: center;
  }
</style>
