<script lang="ts">
  import type { Snippet } from "svelte";
  import Check from "lucide-svelte/icons/check";
  import type { PickableItem, PickerSection } from "../relation-picker-types.js";

  interface Props {
    displaySections: PickerSection[] | null;
    displayItems: PickableItem[];
    sectionLabel?: string;
    selectedIds: string[];
    focusedIndex: number;
    renderItem?: Snippet<[item: PickableItem, selected: boolean]>;
    onItemClick: (item: PickableItem) => void;
    onListKeyDown: (event: KeyboardEvent) => void;
    getGlobalIndex: (sectionIndex: number, itemIndex: number) => number;
    onListRef: (node: HTMLUListElement | null) => void;
  }

  let {
    displaySections,
    displayItems,
    sectionLabel,
    selectedIds,
    focusedIndex,
    renderItem,
    onItemClick,
    onListKeyDown,
    getGlobalIndex,
    onListRef
  }: Props = $props();
  let listRef: HTMLUListElement | null = $state(null);

  $effect(() => {
    onListRef(listRef);
  });

  function isSelected(id: string): boolean {
    return selectedIds.includes(id);
  }

  function handleItemKeydown(event: KeyboardEvent, item: PickableItem) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      onItemClick(item);
    }
  }
</script>

{#if displaySections}
  {#each displaySections as section, sectionIndex (section.label)}
    {#if section.items.length > 0}
      <div class="relation-picker-dialog__section">
        <div class="relation-picker-dialog__section-label">
          {section.label}
        </div>
        <ul
          bind:this={listRef}
          class="relation-picker-dialog__list"
          role="listbox"
          id="relation-picker-list"
          onkeydown={onListKeyDown}
        >
          {#each section.items as item, itemIndex (item.id)}
            {@const selected = isSelected(item.id)}
            {@const globalIndex = getGlobalIndex(sectionIndex, itemIndex)}
            <li
              class="relation-picker-dialog__item"
              class:relation-picker-dialog__item--selected={selected}
              class:relation-picker-dialog__item--disabled={item.disabled}
              class:relation-picker-dialog__item--focused={focusedIndex === globalIndex}
              role="option"
              aria-selected={selected}
              aria-disabled={item.disabled}
              onclick={() => onItemClick(item)}
              onkeydown={(event) => handleItemKeydown(event, item)}
              tabindex={item.disabled ? -1 : 0}
            >
              {#if renderItem}
                {@render renderItem(item, selected)}
              {:else}
                <div class="relation-picker-dialog__item-content">
                  <span class="relation-picker-dialog__item-label">{item.label}</span>
                  {#if item.description}
                    <span class="relation-picker-dialog__item-description">
                      {item.description}
                    </span>
                  {/if}
                </div>
                {#if selected}
                  <Check size="1em" class="relation-picker-dialog__item-check" />
                {/if}
              {/if}
            </li>
          {/each}
        </ul>
      </div>
    {/if}
  {/each}
{:else}
  {#if sectionLabel}
    <div class="relation-picker-dialog__section-label">
      {sectionLabel}
    </div>
  {/if}
  <ul
    bind:this={listRef}
    class="relation-picker-dialog__list"
    role="listbox"
    id="relation-picker-list"
    onkeydown={onListKeyDown}
  >
    {#each displayItems as item, index (item.id)}
      {@const selected = isSelected(item.id)}
      <li
        class="relation-picker-dialog__item"
        class:relation-picker-dialog__item--selected={selected}
        class:relation-picker-dialog__item--disabled={item.disabled}
        class:relation-picker-dialog__item--focused={focusedIndex === index}
        role="option"
        aria-selected={selected}
        aria-disabled={item.disabled}
        onclick={() => onItemClick(item)}
        onkeydown={(event) => handleItemKeydown(event, item)}
        tabindex={item.disabled ? -1 : 0}
      >
        {#if renderItem}
          {@render renderItem(item, selected)}
        {:else}
          <div class="relation-picker-dialog__item-content">
            <span class="relation-picker-dialog__item-label">{item.label}</span>
            {#if item.description}
              <span class="relation-picker-dialog__item-description">
                {item.description}
              </span>
            {/if}
          </div>
          {#if selected}
            <Check size="1em" class="relation-picker-dialog__item-check" />
          {/if}
        {/if}
      </li>
    {/each}
  </ul>
{/if}

<style>
  .relation-picker-dialog__section {
    margin-bottom: 1rem;
  }

  .relation-picker-dialog__section-label {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.5rem;
  }

  .relation-picker-dialog__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.15rem;
  }

  .relation-picker-dialog__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    padding: 0.5rem 0.65rem;
    border-radius: 0.35rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.85rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .relation-picker-dialog__item:hover:not(.relation-picker-dialog__item--disabled) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-picker-dialog__item:focus-visible,
  .relation-picker-dialog__item--focused {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-picker-dialog__item--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-picker-dialog__item--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }

  .relation-picker-dialog__item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .relation-picker-dialog__item-content {
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
    min-width: 0;
    flex: 1;
  }

  .relation-picker-dialog__item-label {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-picker-dialog__item-description {
    font-size: 0.8em;
    opacity: 0.7;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.relation-picker-dialog__item-check) {
    flex-shrink: 0;
    opacity: 0.9;
  }
</style>
