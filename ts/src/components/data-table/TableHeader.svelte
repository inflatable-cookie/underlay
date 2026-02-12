<script lang="ts" generics="T extends object">
  import FilterCell from "../data-table/FilterCell.svelte";
  import type { DataTableColumn, DataTableFilters, DataTableSort } from "../DataTable.svelte";

  interface Props {
    stickyHeader: boolean;
    selectable: boolean;
    allSelected: boolean;
    someSelected: boolean;
    visibleColumns: DataTableColumn<T>[];
    hasActions: boolean;
    sort: DataTableSort | null;
    internalFilters: DataTableFilters;
    onSort: (column: DataTableColumn<T>) => void;
    onSelectAll: () => void;
    onFilterChange: (key: string, value: string) => void;
  }

  let {
    selectable,
    stickyHeader,
    allSelected,
    someSelected,
    visibleColumns,
    hasActions,
    sort,
    internalFilters,
    onSort,
    onSelectAll,
    onFilterChange
  }: Props = $props();
</script>

<div class="underlay-table-header" role="rowgroup">
  <div class="underlay-table-row underlay-header-row" class:underlay-sticky-header-row={stickyHeader} role="row">
    {#if selectable}
      <div class="underlay-table-cell underlay-checkbox-cell" role="columnheader">
        <input
          type="checkbox"
          checked={allSelected}
          indeterminate={someSelected}
          onchange={onSelectAll}
          aria-label="Select all rows"
        />
      </div>
    {/if}

    {#each visibleColumns as column}
      <div
        class="underlay-table-cell underlay-header-cell"
        class:sortable={column.sortable}
        class:underlay-hide-mobile={column.hideOnMobile}
        class:underlay-align-center={column.align === "center"}
        class:underlay-align-right={column.align === "right"}
        role="columnheader"
        aria-sort={sort?.key === column.key ? (sort.direction === "asc" ? "ascending" : "descending") : undefined}
      >
        {#if column.sortable}
          <button type="button" class="underlay-sort-button" onclick={() => onSort(column)}>
            <span>{column.label}</span>
            <span class="underlay-sort-icon" class:underlay-active={sort?.key === column.key}>
              {#if sort?.key === column.key}
                {sort.direction === "asc" ? "↑" : "↓"}
              {:else}
                ↕
              {/if}
            </span>
          </button>
        {:else}
          {column.label}
        {/if}
      </div>
    {/each}

    {#if hasActions}
      <div class="underlay-table-cell underlay-header-cell underlay-actions-header" role="columnheader">
        <span class="underlay-sr-only">Actions</span>
      </div>
    {/if}
  </div>

  {#if visibleColumns.some((column) => column.filterable)}
    <div class="underlay-table-row underlay-filter-row" role="row">
      {#if selectable}
        <div class="underlay-table-cell" role="cell"></div>
      {/if}

      {#each visibleColumns as column}
        <div class="underlay-table-cell underlay-filter-cell" class:underlay-hide-mobile={column.hideOnMobile} role="cell">
          <FilterCell
            {column}
            value={internalFilters[column.key] ?? ""}
            onChange={(value) => onFilterChange(column.key, value)}
          />
        </div>
      {/each}

      {#if hasActions}
        <div class="underlay-table-cell" role="cell"></div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .underlay-table-header {
    display: contents;
  }

  .underlay-table-row {
    display: contents;
  }

  .underlay-header-row > .underlay-table-cell {
    background: var(--dt-header-bg);
    padding: var(--dt-gap);
    font-weight: 600;
    border-bottom: var(--dt-border);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--underlay-color-text-muted, var(--color-text-muted, #64748b));
  }

  .underlay-filter-row > .underlay-table-cell {
    background: var(--dt-header-bg);
    padding: var(--dt-gap);
    border-bottom: var(--dt-border);
  }

  :global(.underlay-compact) .underlay-header-row > .underlay-table-cell,
  :global(.underlay-compact) .underlay-filter-row > .underlay-table-cell {
    padding: var(--dt-gap-compact);
  }

  .underlay-checkbox-cell {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .underlay-header-cell {
    font-weight: 600;
  }

  .underlay-align-center {
    text-align: center;
    justify-content: center;
  }

  .underlay-align-right {
    text-align: right;
    justify-content: flex-end;
  }

  .underlay-sort-button {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    background: none;
    border: none;
    padding: 0;
    font: inherit;
    font-weight: 600;
    cursor: pointer;
    color: inherit;
  }

  .underlay-sort-button:hover {
    color: var(--color-primary, #3b82f6);
  }

  .underlay-sort-icon {
    opacity: 0.4;
    font-size: 0.75em;
  }

  .underlay-sort-icon.underlay-active {
    opacity: 1;
    color: var(--color-primary, #3b82f6);
  }

  .underlay-sticky-header-row {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .underlay-filter-cell :global(.underlay-input),
  .underlay-filter-cell :global(.underlay-input-wrapper) {
    width: 100%;
    font-size: inherit;
  }

  .underlay-filter-cell :global(.underlay-input) {
    padding: 0.25rem 0.5rem;
  }

  .underlay-filter-cell :global(.underlay-select-trigger) {
    min-width: 0;
    padding: 0.25rem 0.5rem;
    font-size: inherit;
  }

  .underlay-sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  @media (max-width: 900px) {
    .underlay-hide-mobile {
      display: none;
    }
  }
</style>
