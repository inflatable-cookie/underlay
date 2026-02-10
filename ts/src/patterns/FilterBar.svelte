<script lang="ts">
  import type { Snippet } from "svelte";
  import ChevronUp from "lucide-svelte/icons/chevron-up";
  import RefreshCcw from "lucide-svelte/icons/refresh-ccw";
  import X from "lucide-svelte/icons/x";

  interface ActiveFilter {
    id: string;
    label: string;
    value: string;
    accent?: string;
  }

  interface Props {
    title?: string;
    startCollapsed?: boolean;
    /** Callback when refresh button is clicked. If provided, shows a refresh button. */
    onRefresh?: () => void;
    /** Array of currently active filters to display as pills */
    activeFilters?: ActiveFilter[];
    /** Callback to clear a specific filter by id */
    onClearFilter?: (filterId: string) => void;
    /** Callback to clear all filters */
    onClearAllFilters?: () => void;
    children?: Snippet;
  }

  let {
    title = "Filters",
    startCollapsed = true,
    onRefresh,
    activeFilters = [],
    onClearFilter,
    onClearAllFilters,
    children
  }: Props = $props();

  let collapsed = $state(true);
  let hasInteracted = $state(false);

  $effect(() => {
    if (!hasInteracted) collapsed = startCollapsed;
  });

  function toggle() {
    hasInteracted = true;
    collapsed = !collapsed;
  }

  function handleClearFilter(filterId: string) {
    onClearFilter?.(filterId);
  }

  function handleClearAll() {
    onClearAllFilters?.();
  }
</script>

<section class="underlay-filter-bar" class:underlay-filter-bar--expanded={!collapsed}>
  {#if collapsed}
    <div class="underlay-filter-bar__row">
      <button type="button" class="underlay-filter-bar__header" onclick={toggle}>
        <h2 class="underlay-filter-bar__title">{title}</h2>
        <span class="underlay-filter-bar__toggle">
          Show filters
        </span>
      </button>
      {#if onRefresh}
        <button
          type="button"
          class="underlay-filter-bar__refresh"
          onclick={onRefresh}
          aria-label="Refresh"
          title="Refresh"
        >
          <RefreshCcw size={16} />
        </button>
      {/if}
    </div>
    {#if activeFilters.length > 0}
      <div class="underlay-filter-bar__active-filters underlay-filter-bar__active-filters--collapsed">
        {#each activeFilters as filter (filter.id)}
          <span
            class="underlay-filter-bar__pill"
            style:--pill-accent={filter.accent ?? 'var(--underlay-color-primary, #0ea5e9)'}
          >
            {filter.label}: {filter.value}
          </span>
        {/each}
        {#if onClearAllFilters}
          <button
            type="button"
            class="underlay-filter-bar__clear-all"
            onclick={handleClearAll}
          >
            Clear all
          </button>
        {/if}
      </div>
    {/if}
  {:else}
    <div class="underlay-filter-bar__actions">
      {#if onRefresh}
        <button
          type="button"
          class="underlay-filter-bar__refresh"
          onclick={onRefresh}
          aria-label="Refresh"
          title="Refresh"
        >
          <RefreshCcw size={16} />
        </button>
      {/if}
      <button
        type="button"
        class="underlay-filter-bar__close"
        onclick={toggle}
        aria-label="Collapse filters"
      >
        <ChevronUp size="1em" strokeWidth={2.5} />
      </button>
    </div>
    <div class="underlay-filter-bar__body">
      {@render children?.()}
    </div>
    {#if activeFilters.length > 0}
      <div class="underlay-filter-bar__active-filters">
        <span class="underlay-filter-bar__active-label">Active filters:</span>
        <div class="underlay-filter-bar__pills">
          {#each activeFilters as filter (filter.id)}
            <span
              class="underlay-filter-bar__pill underlay-filter-bar__pill--removable"
              style:--pill-accent={filter.accent ?? 'var(--underlay-color-primary, #0ea5e9)'}
            >
              <span class="underlay-filter-bar__pill-content">
                <span class="underlay-filter-bar__pill-label">{filter.label}:</span>
                <span class="underlay-filter-bar__pill-value">{filter.value}</span>
              </span>
              {#if onClearFilter}
                <button
                  type="button"
                  class="underlay-filter-bar__pill-remove"
                  onclick={() => handleClearFilter(filter.id)}
                  aria-label="Remove {filter.label} filter"
                  title="Remove filter"
                >
                  <X size={12} />
                </button>
              {/if}
            </span>
          {/each}
        </div>
        {#if onClearAllFilters}
          <button
            type="button"
            class="underlay-filter-bar__clear-all"
            onclick={handleClearAll}
          >
            Clear all
          </button>
        {/if}
      </div>
    {/if}
  {/if}
</section>

<style>
  .underlay-filter-bar {
    position: relative;
    border-radius: var(--underlay-radius-lg, 0.75rem);
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    background: var(--underlay-color-surface-muted, rgba(15, 23, 42, 0.65));
    padding: 0.4rem 0.75rem;
    margin-bottom: var(--underlay-space-4, 1rem);
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-filter-bar--expanded {
    padding: var(--underlay-space-3, 0.75rem);
  }

  .underlay-filter-bar__row {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-filter-bar__header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--underlay-space-2, 0.5rem);
    flex: 1;
    background: none;
    border: none;
    padding: 0;
    margin: 0;
    cursor: pointer;
    text-align: left;
    font: inherit;
  }

  .underlay-filter-bar__header:hover .underlay-filter-bar__toggle {
    background: var(
      --underlay-color-button-neutral-hover,
      var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.12))
    );
  }

  .underlay-filter-bar__title {
    margin: 0;
    font-size: var(--underlay-font-size-xs, 0.75rem);
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
  }

  .underlay-filter-bar__toggle {
    border-radius: var(--underlay-radius-pill, 999px);
    border: 1px solid var(--underlay-color-border-strong, rgba(148, 163, 184, 0.5));
    background: var(--underlay-color-button-neutral-bg, rgba(15, 23, 42, 0.3));
    color: var(--underlay-color-text, #e5e7eb);
    padding: 0.2rem var(--underlay-space-3, 0.75rem);
    font-size: var(--underlay-font-size-xs, 0.75rem);
    transition: background 0.15s ease;
  }

  .underlay-filter-bar__actions {
    position: absolute;
    top: var(--underlay-space-2, 0.5rem);
    right: var(--underlay-space-2, 0.5rem);
    display: flex;
    align-items: center;
    gap: var(--underlay-space-1, 0.25rem);
  }

  .underlay-filter-bar__refresh,
  .underlay-filter-bar__close {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.5rem;
    height: 1.5rem;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    background: transparent;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    cursor: pointer;
  }

  .underlay-filter-bar__refresh:hover,
  .underlay-filter-bar__close:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.12));
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-filter-bar__body {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: var(--underlay-space-4, 1rem);
  }

  .underlay-filter-bar__body :global(.underlay-field) {
    min-width: 12rem;
  }

  /* Active filters section */
  .underlay-filter-bar__active-filters {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    padding-top: var(--underlay-space-3, 0.75rem);
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    margin-top: var(--underlay-space-2, 0.5rem);
  }

  .underlay-filter-bar__active-filters--collapsed {
    padding-top: 0;
    border-top: none;
    margin-top: 0;
    padding-left: var(--underlay-space-8, 2rem);
  }

  .underlay-filter-bar__active-label {
    font-size: var(--underlay-font-size-sm, 0.875rem);
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-weight: 500;
  }

  .underlay-filter-bar__pills {
    display: flex;
    flex-wrap: wrap;
    gap: var(--underlay-space-2, 0.5rem);
    flex: 1;
  }

  .underlay-filter-bar__pill {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-1, 0.25rem);
    padding: 0.25rem var(--underlay-space-2, 0.5rem);
    border-radius: var(--underlay-radius-pill, 999px);
    font-size: var(--underlay-font-size-sm, 0.875rem);
    background: color-mix(in srgb, var(--pill-accent) 15%, transparent);
    border: 1px solid color-mix(in srgb, var(--pill-accent) 30%, transparent);
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-filter-bar__pill--removable {
    padding-right: 0.25rem;
  }

  .underlay-filter-bar__pill-content {
    display: flex;
    align-items: center;
    gap: 0.25rem;
  }

  .underlay-filter-bar__pill-label {
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-weight: 500;
  }

  .underlay-filter-bar__pill-value {
    color: var(--pill-accent);
    font-weight: 600;
  }

  .underlay-filter-bar__pill-remove {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 1.25rem;
    height: 1.25rem;
    padding: 0;
    border: none;
    border-radius: var(--underlay-radius-pill, 999px);
    background: transparent;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .underlay-filter-bar__pill-remove:hover {
    background: var(--underlay-color-danger-bg, rgba(239, 68, 68, 0.2));
    color: var(--underlay-color-danger, #ef4444);
  }

  .underlay-filter-bar__clear-all {
    padding: 0.25rem var(--underlay-space-2, 0.5rem);
    border: none;
    border-radius: var(--underlay-radius-sm, 0.25rem);
    background: transparent;
    color: var(--underlay-color-text-muted, rgba(148, 163, 184, 0.85));
    font-size: var(--underlay-font-size-sm, 0.875rem);
    cursor: pointer;
    text-decoration: underline;
    text-underline-offset: 2px;
    transition: color 0.15s ease;
  }

  .underlay-filter-bar__clear-all:hover {
    color: var(--underlay-color-text, #e5e7eb);
  }
</style>
