<script lang="ts">
  import { Button } from "@poodle/svelte";
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
      <Button
        type="button"
        variant="ghost"
        className="underlay-context-action-list__item"
        disabled={busy}
        onClick={() => onActionSelect?.(action)}
      >
        <span class="underlay-context-action-list__item-copy">
          <strong>{action.name}</strong>
          <span>{action.description}</span>
        </span>
      </Button>
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

  :global(.underlay-context-action-list__item) {
    justify-content: stretch;
    width: 100%;
    min-height: auto;
    padding: 0.75rem;
    text-align: left;
  }

  .underlay-context-action-list__item-copy {
    display: grid;
    min-width: 0;
    gap: 0.25rem;
  }

  .underlay-context-action-list__item-copy strong {
    color: var(--poodle-color-text-primary);
    font-size: 0.925rem;
    line-height: 1.25;
  }

  .underlay-context-action-list__item-copy span {
    color: var(--poodle-color-text-secondary);
    font-size: 0.8125rem;
    line-height: 1.35;
  }
</style>
