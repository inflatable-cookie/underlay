<script lang="ts" generics="T extends object">
  import type { Snippet } from "svelte";
  import type { DataTableAction, DataTableColumn } from "../DataTable.svelte";
  import {
    getRenderedActionHref,
    getRenderedCellValue,
    getRenderedRowActions
  } from "./render";
  import EmptyState from "./EmptyState.svelte";
  import LoadingRow from "./LoadingRow.svelte";
  import RowActionsCell from "./RowActionsCell.svelte";

  interface Props {
    data: T[];
    actions: DataTableAction<T>[] | ((row: T) => DataTableAction<T>[]);
    loading: boolean;
    loadingRows: number;
    selectable: boolean;
    visibleColumns: DataTableColumn<T>[];
    hasActions: boolean;
    selected: T[];
    emptyMessage: string;
    onRowClick?: (row: T) => void;
    onSelectRow: (row: T) => void;
    onActionClick: (action: DataTableAction<T>, row: T) => void;
    empty?: Snippet;
    cell?: Snippet<[{ column: DataTableColumn<T>; row: T; value: string }]>;
    extendedRow?: Snippet<[{ row: T }]>;
    extendedRowWhen: (row: T) => boolean;
  }

  let {
    data,
    actions,
    loading,
    loadingRows,
    selectable,
    visibleColumns,
    hasActions,
    selected,
    emptyMessage,
    onRowClick,
    onSelectRow,
    onActionClick,
    empty,
    cell,
    extendedRow,
    extendedRowWhen
  }: Props = $props();
</script>

<div class="underlay-table-body" role="rowgroup">
  {#if loading}
    {#each Array(loadingRows) as _, i}
      <LoadingRow {selectable} {visibleColumns} showActions={hasActions} />
    {/each}
  {:else if data.length === 0}
    <EmptyState message={emptyMessage} {empty} />
  {:else}
    {#each data as row, rowIndex}
      {@const rowActions = getRenderedRowActions(row, actions)}
      <!-- svelte-ignore a11y_click_events_have_key_events -->
      <div
        class="underlay-table-row"
        class:underlay-selected={selected.includes(row)}
        class:underlay-has-extended={!!extendedRow && extendedRowWhen(row)}
        class:underlay-clickable={!!onRowClick}
        role="row"
        tabindex={onRowClick ? 0 : undefined}
        onclick={() => onRowClick?.(row)}
      >
        {#if selectable}
          <div class="underlay-table-cell underlay-checkbox-cell" role="cell">
            <input
              type="checkbox"
              checked={selected.includes(row)}
              onchange={() => onSelectRow(row)}
              aria-label={`Select row ${rowIndex + 1}`}
            />
          </div>
        {/if}

        {#each visibleColumns as column}
          <div
            class="underlay-table-cell"
            class:underlay-hide-mobile={column.hideOnMobile}
            class:underlay-align-center={column.align === "center"}
            class:underlay-align-right={column.align === "right"}
            role="cell"
          >
            {#if cell}
              {@render cell({ column, row, value: getRenderedCellValue(row, column) })}
            {:else}
              {getRenderedCellValue(row, column)}
            {/if}
          </div>
        {/each}

        {#if hasActions}
          <div class="underlay-table-cell underlay-actions-cell" role="cell">
            <RowActionsCell
              {row}
              {rowActions}
              getActionHref={getRenderedActionHref}
              onActionClick={onActionClick}
            />
          </div>
        {/if}
      </div>
      {#if extendedRow && extendedRowWhen(row)}
        <div class="underlay-table-row underlay-table-row--extended" role="row">
          <div class="underlay-table-cell underlay-table-cell--extended" role="cell">
            {@render extendedRow({ row })}
          </div>
        </div>
      {/if}
    {/each}
  {/if}
</div>

<style>
  .underlay-table-body {
    display: contents;
  }

  .underlay-table-row {
    display: contents;
  }

  .underlay-table-body > .underlay-table-row > .underlay-table-cell {
    border-bottom: var(--dt-border);
  }

  .underlay-table-body > .underlay-table-row.underlay-has-extended > .underlay-table-cell {
    border-bottom: none;
  }

  .underlay-table-body > .underlay-table-row:last-child > .underlay-table-cell,
  .underlay-table-body > .underlay-table-row:last-of-type > .underlay-table-cell,
  .underlay-table-body > .underlay-table-row--extended:last-child > .underlay-table-cell {
    border-bottom: none;
  }

  .underlay-table-body > .underlay-table-row:hover > .underlay-table-cell {
    background: var(--dt-row-hover);
  }

  .underlay-table-body > .underlay-table-row.underlay-clickable {
    cursor: pointer;
  }

  .underlay-table-body > .underlay-table-row.underlay-selected > .underlay-table-cell {
    background: var(--dt-row-selected);
  }

  :global(.underlay-striped) .underlay-table-body > .underlay-table-row:nth-child(even) > .underlay-table-cell {
    background: var(--dt-stripe);
  }

  :global(.underlay-striped) .underlay-table-body > .underlay-table-row:nth-child(even):hover > .underlay-table-cell {
    background: var(--dt-row-hover);
  }

  .underlay-table-cell {
    padding: var(--dt-gap);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
  }

  .underlay-table-row--extended > .underlay-table-cell {
    grid-column: 1 / -1;
  }

  .underlay-table-cell--extended {
    white-space: normal;
    align-items: flex-start;
  }

  .underlay-table-cell > :global(*) {
    align-self: center;
  }

  :global(.underlay-compact) .underlay-table-cell {
    padding: var(--dt-gap-compact);
  }

  .underlay-checkbox-cell {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .underlay-align-center {
    text-align: center;
    justify-content: center;
  }

  .underlay-align-right {
    text-align: right;
    justify-content: flex-end;
  }

  .underlay-actions-cell {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  @media (max-width: 900px) {
    .underlay-hide-mobile {
      display: none;
    }
  }
</style>
