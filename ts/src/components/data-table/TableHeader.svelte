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

<div class="table-header" role="rowgroup">
  <div class="table-row header-row" class:sticky-header-row={stickyHeader} role="row">
    {#if selectable}
      <div class="table-cell checkbox-cell" role="columnheader">
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
        class="table-cell header-cell"
        class:sortable={column.sortable}
        class:hide-mobile={column.hideOnMobile}
        class:align-center={column.align === "center"}
        class:align-right={column.align === "right"}
        role="columnheader"
        aria-sort={sort?.key === column.key ? (sort.direction === "asc" ? "ascending" : "descending") : undefined}
      >
        {#if column.sortable}
          <button type="button" class="sort-button" onclick={() => onSort(column)}>
            <span>{column.label}</span>
            <span class="sort-icon" class:active={sort?.key === column.key}>
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
      <div class="table-cell header-cell actions-header" role="columnheader">
        <span class="sr-only">Actions</span>
      </div>
    {/if}
  </div>

  {#if visibleColumns.some((column) => column.filterable)}
    <div class="table-row filter-row" role="row">
      {#if selectable}
        <div class="table-cell" role="cell"></div>
      {/if}

      {#each visibleColumns as column}
        <div class="table-cell filter-cell" class:hide-mobile={column.hideOnMobile} role="cell">
          <FilterCell
            {column}
            value={internalFilters[column.key] ?? ""}
            onChange={(value) => onFilterChange(column.key, value)}
          />
        </div>
      {/each}

      {#if hasActions}
        <div class="table-cell" role="cell"></div>
      {/if}
    </div>
  {/if}
</div>

<style>
  .table-header {
    display: contents;
  }

  .table-row {
    display: contents;
  }

  .header-row > .table-cell {
    background: var(--dt-header-bg);
    padding: var(--dt-gap);
    font-weight: 600;
    border-bottom: var(--dt-border);
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--underlay-color-text-muted, var(--color-text-muted, #64748b));
  }

  .filter-row > .table-cell {
    background: var(--dt-header-bg);
    padding: var(--dt-gap);
    border-bottom: var(--dt-border);
  }

  :global(.compact) .header-row > .table-cell,
  :global(.compact) .filter-row > .table-cell {
    padding: var(--dt-gap-compact);
  }

  .checkbox-cell {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .header-cell {
    font-weight: 600;
  }

  .align-center {
    text-align: center;
    justify-content: center;
  }

  .align-right {
    text-align: right;
    justify-content: flex-end;
  }

  .sort-button {
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

  .sort-button:hover {
    color: var(--color-primary, #3b82f6);
  }

  .sort-icon {
    opacity: 0.4;
    font-size: 0.75em;
  }

  .sort-icon.active {
    opacity: 1;
    color: var(--color-primary, #3b82f6);
  }

  .sticky-header-row {
    position: sticky;
    top: 0;
    z-index: 10;
  }

  .filter-cell :global(.underlay-input),
  .filter-cell :global(.underlay-input-wrapper) {
    width: 100%;
    font-size: inherit;
  }

  .filter-cell :global(.underlay-input) {
    padding: 0.25rem 0.5rem;
  }

  .filter-cell :global(.underlay-select-trigger) {
    min-width: 0;
    padding: 0.25rem 0.5rem;
    font-size: inherit;
  }

  .sr-only {
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
    .hide-mobile {
      display: none;
    }
  }
</style>
