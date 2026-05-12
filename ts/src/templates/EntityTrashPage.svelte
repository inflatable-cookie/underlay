<script lang="ts" generics="T">
  import {
    Callout,
    EmptyState,
    ListGrid,
    PageHeader,
    PageLoading
  } from "@poodle/svelte";
  import type { TemplateSurface } from "./template.types";

  interface Props {
    section?: string;
    title: string;
    subtitle?: string | null;
    eyebrow?: string | null;
    headerLevel?: 1 | 2 | 3 | 4 | 5 | 6;
    backHref?: string;
    backLabel?: string;
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
    minItemWidth = "26rem"
  }: Props = $props();
</script>

<PageHeader
  {section}
  {title}
  {subtitle}
  {eyebrow}
  level={headerLevel}
  backHref={backHref ?? null}
  {backLabel}
/>

{#if loading}
  <PageLoading message={loadingMessage} />
{:else if error}
  <Callout tone="danger" message={error} />
{:else}
  <div class="underlay-entity-trash-page">
    {#if statusMessage}
      <Callout tone={statusTone} message={statusMessage} />
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
        {#each items as item}
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
