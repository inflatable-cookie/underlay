<script lang="ts">
  import Button from "../Button.svelte";
  import Select from "../Select.svelte";
  import TextInput from "../TextInput.svelte";
  import Download from "lucide-svelte/icons/download";
  import RefreshCw from "lucide-svelte/icons/refresh-cw";
  import X from "lucide-svelte/icons/x";
  import type { LogFilter } from "../LogList.svelte";

  interface Props {
    filters?: LogFilter[];
    filterValues?: Record<string, string>;
    loading?: boolean;
    onFilterChange?: (field: string, value: string) => void;
    onClearFilters?: () => void;
    onRefresh?: () => void;
    onExport?: () => void;
  }

  let {
    filters = [],
    filterValues = {},
    loading = false,
    onFilterChange,
    onClearFilters,
    onRefresh,
    onExport
  }: Props = $props();

  const hasFilters = $derived(filters.length > 0);
  const hasActiveFilters = $derived(
    Object.values(filterValues).some((v) => v && v.trim() !== "")
  );

  function handleFilterChange(field: string, value: string) {
    onFilterChange?.(field, value);
  }
</script>

<div class="log-list__toolbar">
  {#if hasFilters}
    <div class="log-list__filters">
      {#each filters as filter}
        <div class="log-list__filter">
          <label class="log-list__filter-label" for="filter-{filter.field}">
            {filter.label}
          </label>
          {#if filter.type === "select" && filter.options}
            <Select
              id="filter-{filter.field}"
              name={filter.field}
              items={[
                { value: "", label: filter.placeholder ?? "All" },
                ...filter.options
              ]}
              value={filterValues[filter.field] ?? ""}
              onchange={(value) => handleFilterChange(filter.field, value)}
            />
          {:else if filter.type === "date"}
            <TextInput
              id="filter-{filter.field}"
              type="date"
              value={filterValues[filter.field] ?? ""}
              onchange={(value) => handleFilterChange(filter.field, value)}
            />
          {/if}
        </div>
      {/each}
      {#if hasActiveFilters && onClearFilters}
        <Button variant="subtle" size="sm" onclick={onClearFilters}>
          <X size={14} />
          Clear
        </Button>
      {/if}
    </div>
  {/if}

  <div class="log-list__actions">
    {#if onRefresh}
      <Button
        variant="subtle"
        size="sm"
        onclick={onRefresh}
        disabled={loading}
        title="Refresh"
      >
        <RefreshCw size={14} class={loading ? "spinning" : ""} />
      </Button>
    {/if}
    {#if onExport}
      <Button variant="subtle" size="sm" onclick={onExport} disabled={loading}>
        <Download size={14} />
        Export
      </Button>
    {/if}
  </div>
</div>

<style>
  .log-list__toolbar {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-bottom: 1px solid var(--underlay-color-border-subtle, #334155);
    flex-wrap: wrap;
  }

  .log-list__filters {
    display: flex;
    align-items: flex-end;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .log-list__filter {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 120px;
  }

  .log-list__filter-label {
    font-size: 0.7rem;
    font-weight: 500;
    color: var(--underlay-color-text-muted, #94a3b8);
    text-transform: uppercase;
    letter-spacing: 0.025em;
  }

  .log-list__filter :global(.underlay-input[type="date"]) {
    min-width: 130px;
  }

  .log-list__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }
</style>
