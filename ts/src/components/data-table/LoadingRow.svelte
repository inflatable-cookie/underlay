<script lang="ts">
  import type { DataTableColumn } from "../DataTable.svelte";
  import Skeleton from "../Skeleton.svelte";

  interface Props {
    selectable: boolean;
    visibleColumns: DataTableColumn<any>[];
    showActions: boolean;
  }

  let {
    selectable,
    visibleColumns,
    showActions
  }: Props = $props();
</script>

<div class="underlay-table-row" role="row">
  {#if selectable}
    <div class="underlay-table-cell underlay-checkbox-cell" role="cell">
      <Skeleton variant="button" width="16px" height="16px" />
    </div>
  {/if}
  {#each visibleColumns as column}
    <div class="underlay-table-cell" class:underlay-hide-mobile={column.hideOnMobile} role="cell">
      <Skeleton variant="text" />
    </div>
  {/each}
  {#if showActions}
    <div class="underlay-table-cell underlay-actions-cell" role="cell">
      <Skeleton variant="button" width="24px" height="24px" />
    </div>
  {/if}
</div>
