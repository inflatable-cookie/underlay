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

<div class="table-body" role="rowgroup">
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
        class="table-row"
        class:selected={selected.includes(row)}
        class:has-extended={!!extendedRow && extendedRowWhen(row)}
        class:clickable={!!onRowClick}
        role="row"
        tabindex={onRowClick ? 0 : undefined}
        onclick={() => onRowClick?.(row)}
      >
        {#if selectable}
          <div class="table-cell checkbox-cell" role="cell">
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
            class="table-cell"
            class:hide-mobile={column.hideOnMobile}
            class:align-center={column.align === "center"}
            class:align-right={column.align === "right"}
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
          <div class="table-cell actions-cell" role="cell">
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
        <div class="table-row table-row--extended" role="row">
          <div class="table-cell table-cell--extended" role="cell">
            {@render extendedRow({ row })}
          </div>
        </div>
      {/if}
    {/each}
  {/if}
</div>

<style>
  .table-body {
    display: contents;
  }

  .table-row {
    display: contents;
  }

  .table-body > .table-row > .table-cell {
    border-bottom: var(--dt-border);
  }

  .table-body > .table-row.has-extended > .table-cell {
    border-bottom: none;
  }

  .table-body > .table-row:last-child > .table-cell,
  .table-body > .table-row:last-of-type > .table-cell,
  .table-body > .table-row--extended:last-child > .table-cell {
    border-bottom: none;
  }

  .table-body > .table-row:hover > .table-cell {
    background: var(--dt-row-hover);
  }

  .table-body > .table-row.clickable {
    cursor: pointer;
  }

  .table-body > .table-row.selected > .table-cell {
    background: var(--dt-row-selected);
  }

  :global(.striped) .table-body > .table-row:nth-child(even) > .table-cell {
    background: var(--dt-stripe);
  }

  :global(.striped) .table-body > .table-row:nth-child(even):hover > .table-cell {
    background: var(--dt-row-hover);
  }

  .table-cell {
    padding: var(--dt-gap);
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
  }

  .table-row--extended > .table-cell {
    grid-column: 1 / -1;
  }

  .table-cell--extended {
    white-space: normal;
    align-items: flex-start;
  }

  .table-cell > :global(*) {
    align-self: center;
  }

  :global(.compact) .table-cell {
    padding: var(--dt-gap-compact);
  }

  .checkbox-cell {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .align-center {
    text-align: center;
    justify-content: center;
  }

  .align-right {
    text-align: right;
    justify-content: flex-end;
  }

  .actions-cell {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: 0.5rem;
  }

  @media (max-width: 900px) {
    .hide-mobile {
      display: none;
    }
  }
</style>
