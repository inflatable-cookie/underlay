<script lang="ts">
  import { Button, Field, IconButton, Select, TextInput } from "@poodle/svelte-primitives";
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

<div class="underlay-log-list__toolbar">
  {#if hasFilters}
    <div class="underlay-log-list__filters">
      {#each filters as filter}
        <div class="underlay-log-list__filter">
          <label class="underlay-log-list__filter-label" for="filter-{filter.field}">
            {filter.label}
          </label>
          {#if filter.type === "select" && filter.options}
            <div class="underlay-log-list__filter-field">
              <Field id={`filter-${filter.field}`} label={filter.label} let:describedBy>
                <Select
                  id={`filter-${filter.field}`}
                  name={filter.field}
                  options={[
                    { value: "", label: filter.placeholder ?? "All" },
                    ...filter.options
                  ]}
                  value={filterValues[filter.field] ?? ""}
                  describedBy={describedBy}
                  on:valueChange={(event) => handleFilterChange(filter.field, event.detail.value)}
                />
              </Field>
            </div>
          {:else if filter.type === "date"}
            <div class="underlay-log-list__filter-field">
              <Field id={`filter-${filter.field}`} label={filter.label} let:describedBy>
                <TextInput
                  id={`filter-${filter.field}`}
                  type="date"
                  value={filterValues[filter.field] ?? ""}
                  describedBy={describedBy}
                  on:valueChange={(event) => handleFilterChange(filter.field, event.detail.value)}
                />
              </Field>
            </div>
          {/if}
        </div>
      {/each}
      {#if hasActiveFilters && onClearFilters}
        <Button variant="ghost" size="sm" leadingIcon="x" on:click={onClearFilters}>
          Clear
        </Button>
      {/if}
    </div>
  {/if}

  <div class="underlay-log-list__actions">
    {#if onRefresh}
      <IconButton
        icon="refresh-cw"
        variant="ghost"
        size="sm"
        loading={loading}
        ariaLabel="Refresh"
        tooltip="Refresh"
        on:click={onRefresh}
      />
    {/if}
    {#if onExport}
      <Button variant="ghost" size="sm" leadingIcon="download" disabled={loading} on:click={onExport}>
        Export
      </Button>
    {/if}
  </div>
</div>

<style>
  .underlay-log-list__toolbar {
    display: flex;
    align-items: flex-end;
    justify-content: space-between;
    gap: 1rem;
    padding: 0.75rem 1rem;
    background: var(--underlay-color-surface-raised, #283548);
    border-bottom: 1px solid var(--underlay-color-border-subtle, #334155);
    flex-wrap: wrap;
  }

  .underlay-log-list__filters {
    display: flex;
    align-items: flex-end;
    gap: 0.75rem;
    flex-wrap: wrap;
  }

  .underlay-log-list__filter {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    min-width: 120px;
  }

  .underlay-log-list__filter :global(.field) {
    gap: 0.25rem;
  }

  .underlay-log-list__filter :global(.field__label) {
    font-size: 0.7rem;
    font-weight: 500;
    color: var(--underlay-color-text-muted, #94a3b8);
    text-transform: uppercase;
    letter-spacing: 0.025em;
  }

  .underlay-log-list__filter :global(.text-input__control[type="date"]) {
    min-width: 130px;
  }

  .underlay-log-list__actions {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-left: auto;
  }

</style>
