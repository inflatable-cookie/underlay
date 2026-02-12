<script lang="ts">
  import type { DataTableAction } from "../DataTable.svelte";
  import DropdownMenu from "../DropdownMenu.svelte";

  interface Props {
    row: any;
    rowActions: DataTableAction<any>[];
    getActionHref: (action: DataTableAction<any>, row: any) => string | undefined;
    onActionClick: (action: DataTableAction<any>, row: any) => void;
  }

  let {
    row,
    rowActions,
    getActionHref,
    onActionClick
  }: Props = $props();
</script>

{#if rowActions.length === 1}
  {#each [rowActions[0]] as action}
    {#if action.href}
      <a href={getActionHref(action, row)} class="underlay-action-link">{action.label}</a>
    {:else}
      <button type="button" class="underlay-action-button" onclick={() => onActionClick(action, row)}>
        {action.label}
      </button>
    {/if}
  {/each}
{:else if rowActions.length > 1}
  <DropdownMenu>
    {#snippet trigger()}
      <svg
        xmlns="http://www.w3.org/2000/svg"
        width="16"
        height="16"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <circle cx="12" cy="12" r="1" />
        <circle cx="12" cy="5" r="1" />
        <circle cx="12" cy="19" r="1" />
      </svg>
    {/snippet}
    {#each rowActions as action}
      {#if action.href}
        <a href={getActionHref(action, row)} class="underlay-menu-item">{action.label}</a>
      {:else}
        <button
          type="button"
          class="underlay-menu-item"
          class:underlay-danger={action.variant === "danger"}
          onclick={() => onActionClick(action, row)}
        >
          {action.label}
        </button>
      {/if}
    {/each}
  </DropdownMenu>
{/if}

<style>
  .underlay-action-link,
  .underlay-action-button {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.25rem 0.5rem;
    font-size: 0.75rem;
    color: var(--color-primary, #3b82f6);
    text-decoration: none;
    background: none;
    border: none;
    cursor: pointer;
    border-radius: var(--radius-sm, 0.25rem);
    transition: background-color 0.15s;
  }

  .underlay-action-link:hover,
  .underlay-action-button:hover {
    background: var(--dt-row-hover);
  }

  .underlay-menu-item {
    display: flex;
    align-items: center;
    width: 100%;
    padding: 0.5rem 0.75rem;
    font-size: 0.875rem;
    color: var(--color-text, #1e293b);
    text-decoration: none;
    background: none;
    border: none;
    text-align: left;
    cursor: pointer;
  }

  .underlay-menu-item:hover {
    background: var(--dt-row-hover);
  }

  .underlay-menu-item.underlay-danger {
    color: var(--color-danger, #dc2626);
  }
</style>
