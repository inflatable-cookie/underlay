<script lang="ts">
  import { IconButton } from "@poodle/svelte-primitives";
  import type { OrderByFieldDefinition, OrderByField } from "./types";

  interface Props {
    /** The field definition */
    field: OrderByFieldDefinition;
    /** Current sort direction */
    direction: "asc" | "desc";
    /** Called when direction is toggled */
    onToggleDirection: () => void;
    /** Called when field is removed */
    onRemove: () => void;
  }

  let {
    field,
    direction,
    onToggleDirection,
    onRemove,
  }: Props = $props();

  let directionLabel = $derived(direction === "asc" ? "Ascending" : "Descending");
  let directionIcon = $derived(direction === "asc" ? "arrow-up" : "arrow-down");
</script>

<div class="underlay-order-by-item">
  <span class="underlay-order-by-item__handle" aria-hidden="true">⠿</span>
  <span class="underlay-order-by-item__label">{field.label}</span>
  <span class="underlay-order-by-item__action underlay-order-by-item__direction">
    <IconButton
      icon={directionIcon}
      ariaLabel={`Toggle direction to ${direction === "asc" ? "descending" : "ascending"}`}
      tooltip={directionLabel}
      size="sm"
      variant="ghost"
      on:click={onToggleDirection}
    />
  </span>
  <span class="underlay-order-by-item__action underlay-order-by-item__remove">
    <IconButton
      icon="x"
      ariaLabel={`Remove ${field.label} from sort`}
      tooltip={`Remove ${field.label}`}
      size="sm"
      variant="ghost"
      tone="danger"
      on:click={onRemove}
    />
  </span>
</div>

<style>
  .underlay-order-by-item {
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    padding: var(--underlay-space-2, 0.5rem);
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.1));
    border-radius: var(--underlay-radius-md, 0.375rem);
    cursor: grab;
  }

  .underlay-order-by-item:active {
    cursor: grabbing;
  }

  .underlay-order-by-item__handle {
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 1rem;
    line-height: 1;
    user-select: none;
  }

  .underlay-order-by-item__label {
    flex: 1;
    font-size: 0.875rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-order-by-item__action {
    display: inline-flex;
  }

  :global(.underlay-order-by-item__direction .icon-button) {
    border: none;
    background: transparent;
  }

  :global(.underlay-order-by-item__direction .icon-button:hover) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.15));
  }

  :global(.underlay-order-by-item__remove .icon-button) {
    border: none;
    background: transparent;
  }

  :global(.underlay-order-by-item__remove .icon-button:hover) {
    background: rgba(239, 68, 68, 0.15);
  }
</style>
