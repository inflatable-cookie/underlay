<script lang="ts">
  import { ListCard } from "@poodle/svelte";
  import type { ContextActionDefinition } from "./contextual-action.types";

  interface Props {
    actions?: ContextActionDefinition[];
    busy?: boolean;
    emptyMessage?: string;
    onActionSelect?: (action: ContextActionDefinition) => void;
  }

  let {
    actions = [],
    busy = false,
    emptyMessage = "No actions available here.",
    onActionSelect = undefined
  }: Props = $props();
</script>

<div class="underlay-context-action-list">
  {#if actions.length === 0}
    <p class="underlay-context-action-list__empty">{emptyMessage}</p>
  {:else}
    {#each actions as action (action.id)}
      <ListCard
        title={action.name}
        subtitle={action.description}
        layout="compact"
        size="sm"
        density="compact"
        interactive={true}
        disabled={busy}
        ariaLabel={action.name}
        onClick={() => onActionSelect?.(action)}
      />
    {/each}
  {/if}
</div>

<style>
  .underlay-context-action-list {
    display: grid;
    gap: 0.5rem;
  }

  .underlay-context-action-list__empty {
    margin: 0;
    color: var(--poodle-color-text-secondary);
    font-size: 0.875rem;
    line-height: 1.45;
  }
</style>
