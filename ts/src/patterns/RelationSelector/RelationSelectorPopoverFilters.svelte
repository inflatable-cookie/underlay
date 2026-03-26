<script lang="ts">
  import { Button } from "@poodle/svelte-primitives";
  import Check from "lucide-svelte/icons/check";
  import type { FilterConfig } from "./types.js";

  interface Props {
    filters: FilterConfig[];
    activeFilters: Record<string, string | undefined>;
    openFilterKey: string | null;
    getActiveFilterLabel: (filter: FilterConfig) => string;
    onToggleFilter: (filterKey: string) => void;
    onSelectFilter: (filterKey: string, optionId: string | undefined) => void;
  }

  let {
    filters,
    activeFilters,
    openFilterKey,
    getActiveFilterLabel,
    onToggleFilter,
    onSelectFilter
  }: Props = $props();
</script>

<div class="relation-selector-popover__filters">
  {#each filters as filter (filter.key)}
    <div class="relation-selector-popover__filter">
      <span class="relation-selector-popover__filter-label">{filter.label}:</span>
      <div class="relation-selector-popover__filter-dropdown">
        <Button
          type="button"
          variant="ghost"
          size="sm"
          chevron
          className="relation-selector-popover__filter-trigger"
          on:click={() => onToggleFilter(filter.key)}
        >
          {getActiveFilterLabel(filter)}
        </Button>
        {#if openFilterKey === filter.key}
          <div class="relation-selector-popover__filter-menu">
            {#if filter.includeAll !== false}
              <button
                type="button"
                class="relation-selector-popover__filter-option"
                class:relation-selector-popover__filter-option--selected={activeFilters[filter.key] === undefined}
                onclick={() => onSelectFilter(filter.key, undefined)}
              >
                {filter.allLabel ?? "All"}
                {#if activeFilters[filter.key] === undefined}
                  <Check size="0.8em" />
                {/if}
              </button>
            {/if}
            {#each filter.options as option (option.id)}
              <button
                type="button"
                class="relation-selector-popover__filter-option"
                class:relation-selector-popover__filter-option--selected={activeFilters[filter.key] === option.id}
                onclick={() => onSelectFilter(filter.key, option.id)}
              >
                {option.label}
                {#if activeFilters[filter.key] === option.id}
                  <Check size="0.8em" />
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>
    </div>
  {/each}
</div>

<style>
  .relation-selector-popover__filters {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    padding: 0 0.75rem 0.5rem;
    flex-shrink: 0;
  }

  .relation-selector-popover__filter {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    font-size: 0.75rem;
  }

  .relation-selector-popover__filter-label {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-weight: 500;
  }

  .relation-selector-popover__filter-dropdown {
    position: relative;
  }

  :global(.relation-selector-popover__filter-trigger) {
    min-width: 0;
  }

  .relation-selector-popover__filter-menu {
    position: absolute;
    top: 100%;
    left: 0;
    z-index: 10;
    min-width: 100%;
    max-height: 12rem;
    overflow-y: auto;
    margin-top: 0.25rem;
    padding: 0.25rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.4));
    border-radius: 0.3rem;
    background: var(--underlay-color-popover-bg, var(--underlay-color-bg-surface, #020617));
    box-shadow: var(--underlay-shadow-popover, 0 4px 12px rgba(0, 0, 0, 0.3));
  }

  .relation-selector-popover__filter-option {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    width: 100%;
    padding: 0.35rem 0.5rem;
    border: none;
    border-radius: 0.2rem;
    background: transparent;
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.75rem;
    text-align: left;
    cursor: pointer;
    white-space: nowrap;
  }

  .relation-selector-popover__filter-option:hover {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-popover__filter-option--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-selector-popover__filter-option--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }
</style>
