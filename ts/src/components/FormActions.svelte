<script lang="ts">
  import type { Snippet } from "svelte";
  import DropdownMenu from "./DropdownMenu.svelte";
  import ActionArea from "./ActionArea.svelte";

  type DangerMenuItem = {
    label: string;
    onSelect: () => void;
    destructive?: boolean;
  };

  interface Props {
    align?: "start" | "end";
    children?: Snippet;
    /** Danger slot content - shown inline on larger screens */
    danger?: Snippet;
    /** Menu items for danger actions - used in collapsed menu on small screens */
    dangerItems?: DangerMenuItem[];
  }

  let { align = "start", children, danger, dangerItems }: Props = $props();
  const hasDangerActions = $derived(Boolean(danger) || (dangerItems?.length ?? 0) > 0);
</script>

{#if hasDangerActions}
  <ActionArea
    align={align}
    class="underlay-form-actions"
  >
    {@render children?.()}
    {#snippet aside()}
      <!-- Full danger slot - hidden on small screens -->
      {#if danger}
        <div class="underlay-form-actions__danger underlay-form-actions__danger--full">
          {@render danger()}
        </div>
      {/if}

      <!-- Collapsed menu - shown on small screens -->
      {#if dangerItems?.length}
        <div class="underlay-form-actions__danger underlay-form-actions__danger--collapsed">
          <DropdownMenu
            triggerLabel="⋯"
            triggerAriaLabel="More actions"
            items={dangerItems.map(item => ({
              label: item.label,
              onSelect: item.onSelect,
              destructive: item.destructive ?? true
            }))}
          />
        </div>
      {/if}
    {/snippet}
  </ActionArea>
{:else}
  <ActionArea
    align={align}
    class="underlay-form-actions"
  >
    {@render children?.()}
  </ActionArea>
{/if}

<style>
  :global(.underlay-form-actions) {
    container-type: inline-size;
    margin-top: calc(
      var(--underlay-density-gap, 0.75rem) * 1.5
    );
  }

  .underlay-form-actions__danger {
    display: flex;
    align-items: center;
    gap: var(--underlay-form-actions-gap, 1.5rem);
  }

  /* Show full danger slot, hide collapsed by default */
  .underlay-form-actions__danger--full {
    display: flex;
  }

  .underlay-form-actions__danger--collapsed {
    display: none;
  }

  /* On small screens, swap visibility */
  @container (max-width: 500px) {
    .underlay-form-actions__danger--full {
      display: none;
    }

    .underlay-form-actions__danger--collapsed {
      display: flex;
    }
  }
</style>
