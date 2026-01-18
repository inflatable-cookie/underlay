<!--
  ReorderableList - Pattern component for drag-and-drop reordering

  Wraps a list of items with drag-and-drop functionality using svelte-dnd-action.
  Works with the reorder controller for state management and batch commits.

  @example
  ```svelte
  <script lang="ts">
    import { ReorderableList, createReorderController } from '@decodelabs/underlay/patterns';
    import { ListCard } from '@decodelabs/underlay/components';

    let { items, onSubmit } = $props();

    const controller = createReorderController(items, async (orderedIds) => {
      await api.reorderModules(orderedIds);
    });
  </script>

  <ReorderableList {controller} oncancel={() => exitReorderMode()}>
    {#snippet item(module)}
      <ListCard
        title={module.title}
        variant="compact"
        showDragHandle
      >
        {#snippet media()}
          <Icon name={module.icon} />
        {/snippet}
      </ListCard>
    {/snippet}
  </ReorderableList>
  ```
-->
<script lang="ts" generics="T extends { id: string }">
  import type { Snippet } from "svelte";
  import { flip } from "svelte/animate";
  import { dndzone, type DndEvent } from "svelte-dnd-action";
  import type { ReorderController } from "./reorder-controller.svelte";
  import Button from "../components/Button.svelte";

  interface Props {
    /** Reorder controller instance from createReorderController */
    controller: ReorderController<T>;
    /** Called when user cancels reorder mode */
    oncancel: () => void;
    /** Called after successful submit (controller handles the API call) */
    onsuccess?: () => void;
    /** Flip animation duration in ms */
    flipDurationMs?: number;
    /** Whether the list is disabled (e.g. during submit) */
    disabled?: boolean;
    /** Custom save button text */
    saveLabel?: string;
    /** Custom cancel button text */
    cancelLabel?: string;
    /** Item snippet - receives the item to render */
    item: Snippet<[T]>;
    /** Optional empty state snippet */
    empty?: Snippet;
  }

  let {
    controller,
    oncancel,
    onsuccess,
    flipDurationMs = 200,
    disabled = false,
    saveLabel = "Save Order",
    cancelLabel = "Cancel",
    item: itemSnippet,
    empty
  }: Props = $props();

  // Track submission state locally for error handling
  let submitError = $state<string | null>(null);

  function handleDndConsider(e: CustomEvent<DndEvent<T>>) {
    controller.updatePending(e.detail.items);
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<T>>) {
    controller.updatePending(e.detail.items);
  }

  async function handleSubmit() {
    submitError = null;
    try {
      await controller.submit();
      onsuccess?.();
    } catch (e) {
      submitError = e instanceof Error ? e.message : String(e);
    }
  }

  function handleCancel() {
    controller.reset();
    oncancel();
  }

  // Combine disabled prop with controller pending state
  let isDisabled = $derived(disabled || controller.isPending);
</script>

<div class="underlay-reorderable-list" class:underlay-reorderable-list--disabled={isDisabled}>
  <div class="underlay-reorderable-list__header">
    <Button
      variant="subtle"
      onclick={handleCancel}
      disabled={isDisabled}
    >
      {cancelLabel}
    </Button>
    <Button
      variant="primary"
      onclick={handleSubmit}
      disabled={!controller.isDirty || isDisabled}
    >
      {#if controller.isPending}
        Saving...
      {:else}
        {saveLabel}
      {/if}
    </Button>
  </div>

  {#if submitError}
    <div class="underlay-reorderable-list__error" role="alert">
      {submitError}
    </div>
  {/if}

  {#if controller.pending.length === 0 && empty}
    <div class="underlay-reorderable-list__empty">
      {@render empty()}
    </div>
  {:else}
    <div
      class="underlay-reorderable-list__items"
      use:dndzone={{
        items: controller.pending,
        flipDurationMs,
        dropTargetStyle: {},
        dragDisabled: isDisabled
      }}
      onconsider={handleDndConsider}
      onfinalize={handleDndFinalize}
    >
      {#each controller.pending as pendingItem (pendingItem.id)}
        <div class="underlay-reorderable-list__item" animate:flip={{ duration: flipDurationMs }}>
          {@render itemSnippet(pendingItem)}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .underlay-reorderable-list {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-3, 0.75rem);
  }

  .underlay-reorderable-list--disabled {
    opacity: 0.7;
    pointer-events: none;
  }

  .underlay-reorderable-list__header {
    display: flex;
    justify-content: flex-end;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    padding-bottom: var(--underlay-space-2, 0.5rem);
    border-bottom: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
  }

  .underlay-reorderable-list__error {
    padding: var(--underlay-space-3, 0.75rem);
    background: var(--underlay-color-error-bg, rgba(239, 68, 68, 0.1));
    border: 1px solid var(--underlay-color-error, #ef4444);
    border-radius: var(--underlay-radius-md, 0.5rem);
    color: var(--underlay-color-error, #ef4444);
    font-size: 0.875rem;
  }

  .underlay-reorderable-list__items {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    min-height: 100px;
  }

  .underlay-reorderable-list__item {
    /* Allow items to be grabbed */
    cursor: grab;
  }

  .underlay-reorderable-list__item:active {
    cursor: grabbing;
  }

  .underlay-reorderable-list__empty {
    padding: var(--underlay-space-6, 1.5rem);
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    border: 1px dashed var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.25));
    border-radius: var(--underlay-radius-md, 0.5rem);
  }

  /* Style the drag placeholder/shadow */
  .underlay-reorderable-list__items :global([data-is-dnd-shadow-item]) {
    opacity: 0.5;
    border: 2px dashed var(--underlay-color-primary, #2563eb);
    border-radius: var(--underlay-radius-md, 0.5rem);
  }
</style>
