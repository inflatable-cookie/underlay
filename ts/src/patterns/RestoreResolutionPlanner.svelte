<script lang="ts">
  import { flip } from "svelte/animate";
  import {
    dndzone,
    SHADOW_ITEM_MARKER_PROPERTY_NAME,
    SHADOW_PLACEHOLDER_ITEM_ID,
    type DndEvent,
  } from "svelte-dnd-action";
  import ArrowRight from "lucide-svelte/icons/arrow-right";
  import GripVertical from "lucide-svelte/icons/grip-vertical";
  import ListCard from "../components/ListCard.svelte";
  import {
    normalizeRestoreResolutionOrder,
    type RestoreResolutionPlannerItem,
  } from "./restore-resolution";

  interface Props {
    items: RestoreResolutionPlannerItem[];
    value: string[];
    onChange: (next: string[]) => void;
    disabled?: boolean;
    emptyMessage?: string;
  }

  let {
    items,
    value,
    onChange,
    disabled = false,
    emptyMessage = "No items available for restore ordering.",
  }: Props = $props();

  let pendingItems = $state<RestoreResolutionPlannerItem[]>([]);
  let dragActive = $state(false);
  let syncedOrderSignature = $state("");
  let grabbedIndex = $state<number | null>(null);

  const itemMap = $derived.by(() => {
    const map = new Map<string, RestoreResolutionPlannerItem>();
    for (const item of items) {
      map.set(item.id, item);
    }
    return map;
  });

  const effectiveOrder = $derived.by(() => {
    const deduped = Array.from(new Set(value));
    const known = deduped.filter((id) => itemMap.has(id));
    const missing = items.map((item) => item.id).filter((id) => !known.includes(id));
    return [...known, ...missing];
  });

  const orderedItems = $derived(
    effectiveOrder
      .map((id) => itemMap.get(id))
      .filter((item): item is RestoreResolutionPlannerItem => Boolean(item))
  );

  const dndItems = $derived(
    orderedItems.map((item) => ({
      ...item,
      id: item.id,
    }))
  );

  const orderSignature = $derived(effectiveOrder.join("\u0000"));

  $effect(() => {
    if (!dragActive || syncedOrderSignature !== orderSignature) {
      pendingItems = dndItems;
      dragActive = false;
      syncedOrderSignature = orderSignature;
    }
  });

  function sanitizeDndItems(itemsFromDnd: RestoreResolutionPlannerItem[]) {
    return itemsFromDnd.filter((item) => {
      const shadowItem = item as RestoreResolutionPlannerItem & {
        [SHADOW_ITEM_MARKER_PROPERTY_NAME]?: boolean;
      };

      return item.id !== SHADOW_PLACEHOLDER_ITEM_ID
        && shadowItem[SHADOW_ITEM_MARKER_PROPERTY_NAME] !== true;
    });
  }

  function applyFinalizedOrder(itemsFromDnd: RestoreResolutionPlannerItem[]) {
    if (disabled) return;
    const next = normalizeRestoreResolutionOrder(
      sanitizeDndItems(itemsFromDnd)
        .map((item) => item.id)
        .filter((id): id is string => typeof id === "string"),
      effectiveOrder,
    );
    if (next.length === effectiveOrder.length) {
      onChange(next);
    }
  }

  function handleConsider(event: CustomEvent<DndEvent<RestoreResolutionPlannerItem>>) {
    if (disabled) return;
    dragActive = true;
    pendingItems = event.detail.items;
  }

  function handleFinalize(event: CustomEvent<DndEvent<RestoreResolutionPlannerItem>>) {
    if (disabled) return;
    dragActive = false;
    pendingItems = event.detail.items;
    applyFinalizedOrder(event.detail.items);
  }

  function movePendingItem(fromIndex: number, toIndex: number) {
    if (toIndex < 0 || toIndex >= pendingItems.length) return;
    const next = [...pendingItems];
    const [item] = next.splice(fromIndex, 1);
    next.splice(toIndex, 0, item);
    pendingItems = next;
  }

  function handleKeyboardReorder(event: KeyboardEvent, index: number) {
    if (disabled) return;

    if (event.key === " " || event.key === "Enter") {
      event.preventDefault();
      if (grabbedIndex === index) {
        grabbedIndex = null;
        applyFinalizedOrder(pendingItems);
      } else {
        grabbedIndex = index;
      }
      return;
    }

    if (event.key === "Escape" && grabbedIndex !== null) {
      event.preventDefault();
      grabbedIndex = null;
      pendingItems = dndItems;
      return;
    }

    if (grabbedIndex === null) {
      return;
    }

    if (event.key === "ArrowUp") {
      event.preventDefault();
      const targetIndex = grabbedIndex - 1;
      if (targetIndex >= 0) {
        movePendingItem(grabbedIndex, targetIndex);
        grabbedIndex = targetIndex;
      }
      return;
    }

    if (event.key === "ArrowDown") {
      event.preventDefault();
      const targetIndex = grabbedIndex + 1;
      if (targetIndex < pendingItems.length) {
        movePendingItem(grabbedIndex, targetIndex);
        grabbedIndex = targetIndex;
      }
    }
  }
</script>

<div class="underlay-restore-resolution-planner">
  {#if orderedItems.length === 0}
    <p class="underlay-restore-resolution-planner__empty">{emptyMessage}</p>
  {:else}
    <div
      class="underlay-restore-resolution-planner__list"
      role="list"
      use:dndzone={{
        items: pendingItems,
        flipDurationMs: 150,
        dropTargetStyle: {},
        dragDisabled: disabled
      }}
      onconsider={handleConsider}
      onfinalize={handleFinalize}
    >
      {#each pendingItems as item (item.id)}
        {@const MediaIcon = item.mediaIcon ?? GripVertical}
        <div
          class="underlay-restore-resolution-planner__row"
          role="listitem"
          animate:flip={{ duration: 150 }}
        >
          <button
            type="button"
            class="underlay-restore-resolution-planner__button"
            aria-label={`Reorder ${item.label}`}
            onkeydown={(event) => handleKeyboardReorder(event, pendingItems.indexOf(item))}
          >
            <ListCard
              title={item.label}
              subtitle={item.subtitle ?? undefined}
              variant="compact"
              accent={item.accent ?? "#14b8a6"}
            >
              {#snippet titleSnippet()}
                {#if item.preview}
                  {#if item.preview.prefixText}
                    <span>{item.preview.prefixText}</span>
                  {/if}
                  {#if item.preview.previousOrderText}
                    <span class="underlay-restore-resolution-planner__previous">
                      {item.preview.previousOrderText}
                    </span>
                    <ArrowRight
                      class="underlay-restore-resolution-planner__arrow"
                      size={14}
                      aria-hidden="true"
                    />
                  {/if}
                  <span>{item.preview.currentOrderText}</span>
                  {#if item.preview.title}
                    <span>: {item.preview.title}</span>
                  {/if}
                {:else}
                  {item.label}
                {/if}
              {/snippet}

              {#snippet media()}
                <MediaIcon size={16} />
              {/snippet}
            </ListCard>
          </button>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .underlay-restore-resolution-planner {
    display: grid;
    gap: 0.7rem;
  }

  .underlay-restore-resolution-planner__list {
    display: grid;
    gap: 0.55rem;
  }

  .underlay-restore-resolution-planner__row {
    cursor: grab;
  }

  .underlay-restore-resolution-planner__row:active {
    cursor: grabbing;
  }

  .underlay-restore-resolution-planner__button {
    display: block;
    width: 100%;
    padding: 0;
    border: 0;
    background: transparent;
    text-align: left;
    cursor: inherit;
  }

  .underlay-restore-resolution-planner__previous {
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  :global(.underlay-restore-resolution-planner__arrow) {
    display: inline-block;
    vertical-align: -0.12em;
    margin: 0 0.22rem;
    color: var(--underlay-color-text-muted, #94a3b8);
  }

  .underlay-restore-resolution-planner__list :global([data-is-dnd-shadow-item]) {
    opacity: 0.5;
    border: 2px dashed var(--underlay-color-primary, #14b8a6);
    border-radius: 0.5rem;
  }

  .underlay-restore-resolution-planner__empty {
    margin: 0;
    color: var(--underlay-color-text-muted, #94a3b8);
  }
</style>
