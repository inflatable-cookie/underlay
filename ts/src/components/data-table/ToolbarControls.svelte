<script lang="ts">
  import type { DataTableColumn } from "../DataTable.svelte";

  interface Props {
    showColumnToggle: boolean;
    hideableColumns: DataTableColumn<any>[];
    hiddenColumns: Set<string>;
    showColumnMenu: boolean;
    showExport: boolean;
    dataLength: number;
    onToggleColumnMenu: () => void;
    onToggleColumn: (key: string) => void;
    onExport: () => void;
  }

  let {
    showColumnToggle,
    hideableColumns,
    hiddenColumns,
    showColumnMenu,
    showExport,
    dataLength,
    onToggleColumnMenu,
    onToggleColumn,
    onExport
  }: Props = $props();
</script>

{#if showColumnToggle && hideableColumns.length > 0}
  <div class="underlay-column-toggle">
    <button
      type="button"
      class="underlay-toolbar-button"
      onclick={onToggleColumnMenu}
      aria-expanded={showColumnMenu}
      aria-haspopup="true"
    >
      <span class="underlay-toolbar-icon">☰</span>
      Columns
    </button>
    {#if showColumnMenu}
      <div class="underlay-column-menu" role="menu">
        {#each hideableColumns as column}
          <label class="underlay-column-menu-item">
            <input
              type="checkbox"
              checked={!hiddenColumns.has(column.key)}
              onchange={() => onToggleColumn(column.key)}
            />
            {column.label}
          </label>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .underlay-toolbar-button {
    display: inline-flex;
    align-items: center;
    gap: var(--underlay-space-1, 0.25rem);
    padding: var(--underlay-space-1, 0.25rem) var(--underlay-space-2, 0.5rem);
    border: var(--underlay-border-width, 1px) solid var(--underlay-color-border, #d1d5db);
    border-radius: var(--underlay-radius-sm, 0.375rem);
    background: var(--underlay-color-surface, #fff);
    color: inherit;
    font: inherit;
    cursor: pointer;
    white-space: nowrap;
  }

  .underlay-toolbar-button:hover:not(:disabled) {
    background: var(--dt-row-hover);
  }

  .underlay-toolbar-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .underlay-toolbar-icon {
    font-size: inherit;
    line-height: 1;
  }

  .underlay-column-toggle {
    position: relative;
  }

  .underlay-column-menu {
    position: absolute;
    top: calc(100% + var(--underlay-space-1, 0.25rem));
    right: 0;
    min-width: 180px;
    max-height: 300px;
    overflow-y: auto;
    background: var(--underlay-color-surface, #fff);
    border: var(--underlay-border-width, 1px) solid var(--underlay-color-border, #d1d5db);
    border-radius: var(--underlay-radius-sm, 0.375rem);
    box-shadow: var(--underlay-shadow-lg, 0 10px 15px -3px rgb(0 0 0 / 0.1));
    z-index: 10;
    padding: var(--underlay-space-1, 0.25rem);
  }

  .underlay-column-menu-item {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    padding: var(--underlay-space-1, 0.25rem) var(--underlay-space-2, 0.5rem);
    border-radius: var(--underlay-radius-xs, 0.25rem);
    cursor: pointer;
    user-select: none;
    font-size: calc(1em * var(--underlay-font-scale-sm, 0.875));
  }

  .underlay-column-menu-item:hover {
    background: var(--dt-row-hover);
  }

  .underlay-column-menu-item input {
    cursor: pointer;
  }
</style>

{#if showExport}
  <button type="button" class="underlay-toolbar-button" onclick={onExport} disabled={dataLength === 0}>
    <span class="underlay-toolbar-icon">↓</span>
    Export CSV
  </button>
{/if}
