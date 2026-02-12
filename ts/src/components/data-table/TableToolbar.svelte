<script lang="ts" generics="T extends object">
  import type { Snippet } from "svelte";
  import type { DataTableColumn } from "../DataTable.svelte";
  import ToolbarControls from "./ToolbarControls.svelte";

  interface Props {
    showColumnToggle: boolean;
    hideableColumns: DataTableColumn<T>[];
    hiddenColumns: Set<string>;
    showColumnMenu: boolean;
    showExport: boolean;
    dataLength: number;
    toolbarLeft?: Snippet;
    toolbarRight?: Snippet;
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
    toolbarLeft,
    toolbarRight,
    onToggleColumnMenu,
    onToggleColumn,
    onExport
  }: Props = $props();
</script>

<div class="underlay-table-toolbar">
  <div class="underlay-toolbar-left">
    {@render toolbarLeft?.()}
  </div>
  <div class="underlay-toolbar-right">
    <ToolbarControls
      {showColumnToggle}
      {hideableColumns}
      {hiddenColumns}
      {showColumnMenu}
      {showExport}
      {dataLength}
      onToggleColumnMenu={onToggleColumnMenu}
      onToggleColumn={onToggleColumn}
      onExport={onExport}
    />
    {@render toolbarRight?.()}
  </div>
</div>

<style>
  .underlay-table-toolbar {
    grid-column: 1 / -1;
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5rem 0.75rem;
    background: var(--dt-header-bg);
    border-bottom: var(--dt-border);
    gap: 0.5rem;
  }

  .underlay-toolbar-left,
  .underlay-toolbar-right {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }
</style>
