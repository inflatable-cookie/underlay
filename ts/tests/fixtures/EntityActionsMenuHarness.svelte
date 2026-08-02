<script lang="ts">
  import EntityActionsMenu from "../../src/templates/EntityActionsMenu.svelte";
  import type { ToastStore } from "../../src/patterns/toasts";

  interface Props {
    onEdit?: () => void;
    execute?: () => void | Promise<void>;
    onDeleteSuccess?: () => void;
    toastStore?: ToastStore;
  }

  let {
    onEdit = undefined,
    execute = async () => {},
    onDeleteSuccess = undefined,
    toastStore = undefined
  }: Props = $props();
</script>

<EntityActionsMenu
  {onEdit}
  {toastStore}
  deleteConfig={{
    entityLabel: "G2019 Machines in motion",
    title: "Purge chapter?",
    description: "This permanently deletes the chapter.",
    confirmLabel: "Purge",
    execute
  }}
  {onDeleteSuccess}
>
  {#snippet content({ items, onAction })}
    {#each items as item}
      {#if item.kind !== "separator"}
        <button
          type="button"
          data-testid={`action-${item.value}`}
          disabled={item.disabled ?? false}
          onclick={() => onAction(item.value)}
        >
          {item.label}
        </button>
      {/if}
    {/each}
  {/snippet}
</EntityActionsMenu>
