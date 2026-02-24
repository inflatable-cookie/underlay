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
    /** Optional error hook; can transform submit errors into user-facing text */
    onsubmiterror?: (error: unknown) => void | string | Promise<void | string>;
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
    /** Optional explicit move buttons (up/down) for touch and keyboard fallback */
    showMoveButtons?: boolean;
    /** Optional highlighted IDs (for conflict recovery newly-added items) */
    highlightedIds?: string[];
    /** Optional label builder for a11y announcements */
    getItemLabel?: (item: T) => string;
    /** Threshold for long-list warning. Set to null/0 to disable. */
    longListThreshold?: number | null;
    /** Optional custom long-list warning text */
    longListWarningText?: string;
    /** Optional page/window size for large-list reorder mode. Null disables. */
    windowSize?: number | null;
  }

  let {
    controller,
    oncancel,
    onsuccess,
    onsubmiterror,
    flipDurationMs = 200,
    disabled = false,
    saveLabel = "Save Order",
    cancelLabel = "Cancel",
    item: itemSnippet,
    empty,
    showMoveButtons = false,
    highlightedIds = [],
    getItemLabel = (item: T) => item.id,
    longListThreshold = 50,
    longListWarningText,
    windowSize = null
  }: Props = $props();

  // Track submission state locally for error handling
  let submitError = $state<string | null>(null);
  let liveMessage = $state("");
  let grabbedIndex = $state<number | null>(null);

  async function handleSubmit() {
    submitError = null;
    try {
      await controller.submit();
      onsuccess?.();
    } catch (e) {
      const transformed = await onsubmiterror?.(e);
      submitError = transformed ?? (e instanceof Error ? e.message : String(e));
    }
  }

  function handleCancel() {
    controller.reset();
    grabbedIndex = null;
    oncancel();
  }

  function announce(message: string) {
    liveMessage = message;
  }

  function moveItem(fromIndex: number, toIndex: number) {
    if (toIndex < 0 || toIndex >= controller.pending.length) return;
    const item = controller.pending[fromIndex];
    controller.move(fromIndex, toIndex);
    announce(
      `Moved ${getItemLabel(item)} to position ${toIndex + 1} of ${controller.pending.length}.`
    );
  }

  function handleItemKeydown(event: KeyboardEvent, index: number) {
    const key = event.key;

    if (key === " " || key === "Enter") {
      event.preventDefault();
      if (grabbedIndex === index) {
        grabbedIndex = null;
        announce("Dropped item.");
      } else {
        grabbedIndex = index;
        announce(
          `Grabbed ${getItemLabel(controller.pending[index])}. Use arrow keys to move, Escape to cancel.`
        );
      }
      return;
    }

    if (key === "Escape" && grabbedIndex !== null) {
      event.preventDefault();
      grabbedIndex = null;
      announce("Cancelled keyboard move.");
      return;
    }

    if (key === "ArrowUp" || key === "ArrowDown") {
      event.preventDefault();
      const activeIndex = grabbedIndex ?? index;
      const targetIndex = key === "ArrowUp" ? activeIndex - 1 : activeIndex + 1;
      if (targetIndex < 0 || targetIndex >= controller.pending.length) {
        announce("Reached list boundary.");
        return;
      }
      moveItem(activeIndex, targetIndex);
      if (grabbedIndex !== null) {
        grabbedIndex = targetIndex;
      }
    }
  }

  // Combine disabled prop with controller pending state
  let isDisabled = $derived(disabled || controller.isPending);
  let highlightedSet = $derived(new Set(highlightedIds));
  let isLongList = $derived(
    longListThreshold !== null && longListThreshold > 0 && controller.pending.length > longListThreshold
  );
  let effectiveLongListWarning = $derived(
    longListWarningText ??
      `This list has ${controller.pending.length} items. Reordering large lists can be error-prone; consider chunked moves and save often.`
  );
  let windowPageIndex = $state(0);
  let effectiveWindowSize = $derived(
    windowSize !== null && windowSize > 0 ? windowSize : controller.pending.length
  );
  let isWindowed = $derived(
    windowSize !== null && windowSize > 0 && controller.pending.length > effectiveWindowSize
  );
  let windowPageCount = $derived(
    isWindowed ? Math.ceil(controller.pending.length / effectiveWindowSize) : 1
  );
  let windowStart = $derived(
    isWindowed ? windowPageIndex * effectiveWindowSize : 0
  );
  let windowEnd = $derived(Math.min(windowStart + effectiveWindowSize, controller.pending.length));
  let visibleItems = $derived(controller.pending.slice(windowStart, windowEnd));

  $effect(() => {
    if (!isWindowed) {
      windowPageIndex = 0;
      return;
    }
    if (windowPageIndex >= windowPageCount) {
      windowPageIndex = Math.max(windowPageCount - 1, 0);
    }
  });

  function previousWindowPage() {
    windowPageIndex = Math.max(windowPageIndex - 1, 0);
  }

  function nextWindowPage() {
    windowPageIndex = Math.min(windowPageIndex + 1, windowPageCount - 1);
  }

  function updatePendingFromDndItems(items: T[]) {
    if (!isWindowed) {
      controller.updatePending(items);
      return;
    }

    const merged = [...controller.pending];
    merged.splice(windowStart, items.length, ...items);
    controller.updatePending(merged);
  }
</script>

<div class="underlay-reorderable-list" class:underlay-reorderable-list--disabled={isDisabled}>
  <div class="underlay-reorderable-list__sr" aria-live="polite" aria-atomic="true">{liveMessage}</div>
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

  {#if isLongList}
    <div class="underlay-reorderable-list__info" role="status">
      {effectiveLongListWarning}
    </div>
  {/if}

  {#if isWindowed}
    <div class="underlay-reorderable-list__window-nav">
      <Button variant="subtle" onclick={previousWindowPage} disabled={isDisabled || windowPageIndex === 0}>
        Previous
      </Button>
      <span class="underlay-reorderable-list__window-label">
        Page {windowPageIndex + 1} of {windowPageCount}
      </span>
      <Button
        variant="subtle"
        onclick={nextWindowPage}
        disabled={isDisabled || windowPageIndex >= windowPageCount - 1}
      >
        Next
      </Button>
    </div>
  {/if}

  {#if controller.pending.length === 0 && empty}
    <div class="underlay-reorderable-list__empty">
      {@render empty()}
    </div>
  {:else}
    <div
      class="underlay-reorderable-list__items"
      role="list"
      use:dndzone={{
        items: visibleItems,
        flipDurationMs,
        dropTargetStyle: {},
        dragDisabled: isDisabled
      }}
      onconsider={(event: CustomEvent<DndEvent<T>>) => updatePendingFromDndItems(event.detail.items)}
      onfinalize={(event: CustomEvent<DndEvent<T>>) => updatePendingFromDndItems(event.detail.items)}
    >
      {#each visibleItems as pendingItem, localIndex (pendingItem.id)}
        {@const index = windowStart + localIndex}
        <div
          class="underlay-reorderable-list__item"
          class:underlay-reorderable-list__item--highlighted={highlightedSet.has(pendingItem.id)}
          animate:flip={{ duration: flipDurationMs }}
          role="listitem"
        >
          <button
            type="button"
            class="underlay-reorderable-list__kbd-handle"
            disabled={isDisabled}
            aria-label={`Reorder ${getItemLabel(pendingItem)}. Position ${index + 1} of ${controller.pending.length}. Press space to grab, then arrow keys to move.`}
            onkeydown={(event) => handleItemKeydown(event, index)}
          >
            Reorder
          </button>
          {#if showMoveButtons}
            <div class="underlay-reorderable-list__move-buttons">
              <button
                type="button"
                class="underlay-reorderable-list__move-btn"
                onclick={() => moveItem(index, index - 1)}
                disabled={isDisabled || index === 0}
                aria-label={`Move ${getItemLabel(pendingItem)} up`}
              >
                ↑
              </button>
              <button
                type="button"
                class="underlay-reorderable-list__move-btn"
                onclick={() => moveItem(index, index + 1)}
                disabled={isDisabled || index === controller.pending.length - 1}
                aria-label={`Move ${getItemLabel(pendingItem)} down`}
              >
                ↓
              </button>
            </div>
          {/if}
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

  .underlay-reorderable-list__sr {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
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

  .underlay-reorderable-list__info {
    padding: var(--underlay-space-3, 0.75rem);
    background: rgba(37, 99, 235, 0.08);
    border: 1px solid rgba(37, 99, 235, 0.2);
    border-radius: var(--underlay-radius-md, 0.5rem);
    color: var(--underlay-color-text, #1e293b);
    font-size: 0.875rem;
  }

  .underlay-reorderable-list__items {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    min-height: 100px;
  }

  .underlay-reorderable-list__window-nav {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    gap: var(--underlay-space-2, 0.5rem);
  }

  .underlay-reorderable-list__window-label {
    font-size: 0.875rem;
    color: var(--underlay-color-text-muted, #64748b);
    min-width: 110px;
    text-align: center;
  }

  .underlay-reorderable-list__item {
    cursor: grab;
    display: flex;
    align-items: center;
    gap: var(--underlay-space-2, 0.5rem);
    border-radius: var(--underlay-radius-md, 0.5rem);
  }

  .underlay-reorderable-list__item--highlighted {
    background: rgba(37, 99, 235, 0.08);
    box-shadow: inset 0 0 0 1px rgba(37, 99, 235, 0.25);
  }

  .underlay-reorderable-list__kbd-handle {
    min-width: 56px;
    height: 28px;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    border-radius: var(--underlay-radius-sm, 0.375rem);
    background: var(--underlay-color-surface, #fff);
    color: var(--underlay-color-text-muted, #475569);
    font-size: 0.75rem;
    cursor: pointer;
    flex-shrink: 0;
  }

  .underlay-reorderable-list__kbd-handle:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 2px;
  }

  .underlay-reorderable-list__move-buttons {
    display: flex;
    flex-direction: column;
    gap: 4px;
    flex-shrink: 0;
  }

  .underlay-reorderable-list__move-btn {
    width: 28px;
    height: 24px;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.3));
    background: var(--underlay-color-surface, #fff);
    border-radius: var(--underlay-radius-sm, 0.375rem);
    color: var(--underlay-color-text-muted, #475569);
    font-size: 12px;
    line-height: 1;
    cursor: pointer;
  }

  .underlay-reorderable-list__move-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
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
