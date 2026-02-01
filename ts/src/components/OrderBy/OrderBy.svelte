<script lang="ts">
  import { tick, untrack } from "svelte";
  import { flip } from "svelte/animate";
  import { dndzone, type DndEvent } from "svelte-dnd-action";
  import { Popover as BitsPopover } from "bits-ui";
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import X from "lucide-svelte/icons/x";
  import Select from "../Select.svelte";
  import TextButton from "../TextButton.svelte";
  import OrderByItem from "./OrderByItem.svelte";
  import type { OrderByFieldDefinition, OrderByField, OrderByValue } from "./types";

  interface Props {
    /** Available fields that can be sorted */
    fields: OrderByFieldDefinition[];
    /** Current ordering state (bindable) */
    value?: OrderByValue;
    /** Callback when ordering changes */
    onChange?: (value: OrderByValue) => void;
    /** Maximum number of sort fields (default: unlimited) */
    maxFields?: number;
    /** Compact mode for tight spaces */
    compact?: boolean;
    /** Additional CSS class */
    class?: string;
  }

  let {
    fields,
    value = $bindable([]),
    onChange,
    maxFields,
    compact = false,
    class: className,
  }: Props = $props();

  let open = $state(false);
  let triggerRef: HTMLElement | null = $state(null);
  let addFieldValue = $state("");

  // Create items with unique IDs for dnd-zone
  interface DndItem extends OrderByField {
    id: string;
  }

  let dndItems = $derived<DndItem[]>(
    value.map((field, index) => ({
      ...field,
      id: `${field.key}-${index}`,
    }))
  );

  // Available fields (those not already selected)
  let availableFields = $derived(
    fields.filter((f) => !value.some((v) => v.key === f.key))
  );

  let canAddMore = $derived(
    !maxFields || value.length < maxFields
  );

  // Field lookup for display
  let fieldMap = $derived(
    new Map(fields.map((f) => [f.key, f]))
  );

  // Trigger display text
  let triggerText = $derived(() => {
    if (value.length === 0) {
      return "Sort by...";
    }

    const items = value.map((v) => {
      const field = fieldMap.get(v.key);
      const label = field?.label ?? v.key;
      const arrow = v.direction === "asc" ? "↑" : "↓";
      return `${label} ${arrow}`;
    });

    if (compact && items.length > 2) {
      return `${items.slice(0, 2).join(", ")} +${items.length - 2}`;
    }

    return items.join(", ");
  });

  function updateValue(newValue: OrderByValue) {
    value = newValue;
    onChange?.(newValue);
  }

  function handleDndConsider(e: CustomEvent<DndEvent<DndItem>>) {
    const newItems = e.detail.items.map(({ key, direction }) => ({ key, direction }));
    updateValue(newItems);
  }

  function handleDndFinalize(e: CustomEvent<DndEvent<DndItem>>) {
    const newItems = e.detail.items.map(({ key, direction }) => ({ key, direction }));
    updateValue(newItems);
  }

  function toggleDirection(index: number) {
    const newValue = [...value];
    newValue[index] = {
      ...newValue[index],
      direction: newValue[index].direction === "asc" ? "desc" : "asc",
    };
    updateValue(newValue);
  }

  function removeField(index: number) {
    const newValue = value.filter((_, i) => i !== index);
    updateValue(newValue);
  }

  function addField(key: string) {
    if (!key || value.some((v) => v.key === key)) return;

    const fieldDef = fieldMap.get(key);
    const direction = fieldDef?.defaultDirection ?? "asc";

    updateValue([...value, { key, direction }]);
    addFieldValue = "";
  }

  function reverseAll() {
    const newValue = value.map((v) => ({
      ...v,
      direction: v.direction === "asc" ? "desc" as const : "asc" as const,
    }));
    updateValue(newValue);
  }

  function clearAll() {
    updateValue([]);
  }

  let showClearButton = $derived(value.length > 0);

  function handleClear(event: MouseEvent) {
    event.stopPropagation();
    event.preventDefault();
    clearAll();
  }

  // Return focus to trigger on close
  // Use untrack to capture initial value - the $effect below tracks changes
  let lastOpen = $state(untrack(() => open));
  $effect(() => {
    if (lastOpen && !open && typeof window !== "undefined") {
      void tick().then(() => triggerRef?.focus());
    }
    lastOpen = open;
  });

  // Handle select items format
  let selectItems = $derived(
    availableFields.map((f) => ({
      value: f.key,
      label: f.label,
    }))
  );
</script>

<BitsPopover.Root bind:open>
  <BitsPopover.Trigger
    bind:ref={triggerRef}
    type="button"
    class={`underlay-order-by-trigger ${className ?? ""}`}
    aria-label="Configure sort order"
  >
    <span class="underlay-order-by-trigger__text" class:placeholder={value.length === 0}>
      {triggerText()}
    </span>
    <span class="underlay-order-by-trigger__controls">
      {#if showClearButton}
        <button
          type="button"
          class="underlay-order-by-trigger__clear"
          aria-label="Clear sort"
          onclick={handleClear}
          onpointerdown={(e) => e.stopPropagation()}
          onmousedown={(e) => e.stopPropagation()}
        >
          <X size="1em" strokeWidth={2.5} />
        </button>
      {/if}
      <ChevronDown size="1em" strokeWidth={2.5} class="underlay-order-by-trigger__chevron" />
    </span>
  </BitsPopover.Trigger>

  {#if open}
    <BitsPopover.Portal>
      <BitsPopover.Content
        class="underlay-order-by-content"
        side="bottom"
        sideOffset={6}
        align="start"
        avoidCollisions={true}
        collisionPadding={8}
      >
        {#if value.length > 0}
          <div
            class="underlay-order-by-list"
            use:dndzone={{
              items: dndItems,
              flipDurationMs: 150,
              dropTargetStyle: {},
            }}
            onconsider={handleDndConsider}
            onfinalize={handleDndFinalize}
          >
            {#each dndItems as item, index (item.id)}
              {@const fieldDef = fieldMap.get(item.key)}
              <div class="underlay-order-by-list__item" animate:flip={{ duration: 150 }}>
                {#if fieldDef}
                  <OrderByItem
                    field={fieldDef}
                    direction={item.direction}
                    onToggleDirection={() => toggleDirection(index)}
                    onRemove={() => removeField(index)}
                  />
                {/if}
              </div>
            {/each}
          </div>
        {:else}
          <div class="underlay-order-by-empty">
            No sort fields selected
          </div>
        {/if}

        {#if canAddMore && availableFields.length > 0}
          <div class="underlay-order-by-add">
            <Select
              items={selectItems}
              bind:value={addFieldValue}
              placeholder="+ Add field"
              onchange={addField}
            />
          </div>
        {/if}

        {#if value.length > 0}
          <div class="underlay-order-by-actions">
            <TextButton onclick={reverseAll}>
              ↕ Reverse All
            </TextButton>
            <TextButton onclick={clearAll} variant="danger">
              Clear
            </TextButton>
          </div>
        {/if}
      </BitsPopover.Content>
    </BitsPopover.Portal>
  {/if}
</BitsPopover.Root>

<style>
  :global(.underlay-order-by-trigger) {
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    min-width: 10rem;
    padding: var(--underlay-field-padding-block, 0.55em)
      var(--underlay-field-padding-inline, 0.7em);
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
    cursor: pointer;
  }

  :global(.underlay-order-by-trigger:hover) {
    background: var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.25));
  }

  :global(.underlay-order-by-trigger:focus-visible) {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: var(--underlay-focus-ring-offset, 2px);
  }

  .underlay-order-by-trigger__text {
    flex: 1;
    text-align: left;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .underlay-order-by-trigger__text.placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  :global(.underlay-order-by-trigger__controls) {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    margin-left: 0.5rem;
  }

  :global(.underlay-order-by-trigger__clear) {
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    margin: 0;
    border: none;
    background: transparent;
    color: var(--underlay-color-text-muted, #9ca3af);
    cursor: pointer;
    border-radius: 0.2rem;
    opacity: 0.7;
    transition: opacity 0.15s ease, color 0.15s ease;
  }

  :global(.underlay-order-by-trigger__clear:hover) {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  :global(.underlay-order-by-trigger__clear:focus-visible) {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  :global(.underlay-order-by-trigger__chevron) {
    color: var(--underlay-color-text, #e5e7eb);
    opacity: 0.5;
  }

  :global(.underlay-order-by-content) {
    z-index: 150;
    min-width: 16rem;
    max-width: min(24rem, calc(100vw - 2rem));
    padding: var(--underlay-space-3, 0.75rem);
    border-radius: 0.5rem;
    border: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.5));
    background: var(--underlay-color-popover-bg, var(--underlay-color-bg-surface, #020617));
    box-shadow: var(--underlay-shadow-popover, 0 18px 48px rgba(0, 0, 0, 0.6));
    color: var(--underlay-color-text, #e5e7eb);
  }

  .underlay-order-by-list {
    display: flex;
    flex-direction: column;
    gap: var(--underlay-space-2, 0.5rem);
    min-height: 2.5rem;
  }

  .underlay-order-by-empty {
    padding: var(--underlay-space-3, 0.75rem);
    text-align: center;
    color: var(--underlay-color-text-muted, #9ca3af);
    font-size: 0.85rem;
  }

  .underlay-order-by-add {
    margin-top: var(--underlay-space-3, 0.75rem);
    padding-top: var(--underlay-space-3, 0.75rem);
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
  }

  .underlay-order-by-actions {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-top: var(--underlay-space-3, 0.75rem);
    padding-top: var(--underlay-space-3, 0.75rem);
    border-top: 1px solid var(--underlay-color-border-subtle, rgba(148, 163, 184, 0.2));
  }

  /* Style the drag placeholder */
  .underlay-order-by-list :global([data-is-dnd-shadow-item]) {
    opacity: 0.5;
    border: 2px dashed var(--underlay-color-primary, #2563eb);
    border-radius: var(--underlay-radius-md, 0.375rem);
  }
</style>
