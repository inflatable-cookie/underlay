<script lang="ts">
  import type { Snippet } from "svelte";
  import PageHeader from "../patterns/PageHeader.svelte";
  import PageLoading from "./PageLoading.svelte";
  import FormError from "./FormError.svelte";

  type ListVariant = "page" | "tab";

  interface Props {
    /** List title */
    title: string;
    /** Display variant: "page" for root pages, "tab" for embedded tabs */
    variant?: ListVariant;
    /** Whether data is loading */
    loading?: boolean;
    /** Error message to display */
    error?: string | null;
    /** Total item count (shown in header) */
    count?: number;
    /** Empty state message */
    emptyMessage?: string;
    /** Whether the list has items (used for empty state) */
    hasItems?: boolean;
    /** Back link href (optional) */
    backHref?: string | null;
    /** Back link label */
    backLabel?: string;
    /** Whether back link came from navigation context */
    backIsContextual?: boolean;
    /** Header action buttons (trash, add, etc.) */
    actions?: Snippet;
    /** Filter bar content */
    filters?: Snippet;
    /** Main list content */
    content?: Snippet;
    /** Batch action bar (shown when items selected) */
    batchBar?: Snippet;
    /** Pagination component */
    pagination?: Snippet;
  }

  let {
    title,
    variant = "page",
    loading = false,
    error = null,
    count,
    emptyMessage = "No items found.",
    hasItems = true,
    backHref = null,
    backLabel = "Back",
    backIsContextual = false,
    actions,
    filters,
    content,
    batchBar,
    pagination
  }: Props = $props();

  // Map variant to header level
  const headerLevel = $derived(variant === "page" ? 1 : 3);
</script>

<div class="underlay-list-container underlay-list-container--{variant}">
  <PageHeader
    {title}
    level={headerLevel}
    {count}
    {backHref}
    {backLabel}
    {backIsContextual}
  >
    {#snippet actions()}
      {#if actions}
        {@render actions()}
      {/if}
    {/snippet}
  </PageHeader>

  {#if loading}
    <PageLoading />
  {:else if error}
    <FormError message={error} />
  {:else}
    {#if hasItems && filters}
      <div class="underlay-list-container__filters">
        {@render filters()}
      </div>
    {/if}

    {#if batchBar}
      <div class="underlay-list-container__batch-bar">
        {@render batchBar()}
      </div>
    {/if}

    {#if !hasItems}
      <p class="underlay-list-container__empty">{emptyMessage}</p>
    {:else if content}
      <div class="underlay-list-container__content">
        {@render content()}
      </div>
    {/if}

    {#if hasItems && pagination}
      <div class="underlay-list-container__pagination">
        {@render pagination()}
      </div>
    {/if}
  {/if}
</div>

<style>
  .underlay-list-container {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-list-container__empty {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-style: italic;
    margin: 0;
  }

  .underlay-list-container__filters {
    margin-bottom: var(--underlay-space-2, 0.5rem);
  }

  .underlay-list-container__pagination {
    margin-top: var(--underlay-space-4, 1rem);
  }
</style>
