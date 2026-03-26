<script lang="ts">
  import type { Snippet } from "svelte";
  import { FormActions as PoodleFormActions, IconButton, Menu } from "@poodle/svelte-primitives";
  import type { MenuItem } from "@poodle/svelte-primitives";

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
  const collapsedDangerItems = $derived<MenuItem[]>(
    (dangerItems ?? []).map((item) => ({
      value: item.label,
      label: item.label
    }))
  );

  function handleDangerAction(value: string): void {
    const item = (dangerItems ?? []).find((candidate) => candidate.label === value);
    item?.onSelect();
  }
</script>

{#if hasDangerActions}
  <PoodleFormActions
    align={align}
  >
    {@render children?.()}
    {#if danger}
      <div class="underlay-form-actions__danger underlay-form-actions__danger--full">
        {@render danger()}
      </div>
    {/if}

    {#if dangerItems?.length}
      <div class="underlay-form-actions__danger underlay-form-actions__danger--collapsed">
        <Menu
          items={collapsedDangerItems}
          placement="top-end"
          ariaLabel="More actions"
          on:action={(event) => handleDangerAction(event.detail.value)}
        >
          <div slot="trigger">
            <IconButton icon="ellipsis" ariaLabel="More actions" variant="ghost" size="sm" />
          </div>
        </Menu>
      </div>
    {/if}
  </PoodleFormActions>
{:else}
  <PoodleFormActions
    align={align}
  >
    {@render children?.()}
  </PoodleFormActions>
{/if}

<style>
  :global(.form-actions) {
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
