<script lang="ts">
  import type { Snippet } from "svelte";
  import Check from "lucide-svelte/icons/check";
  import type { SelectableRelation } from "./types.js";

  interface Props {
    label: string;
    items: SelectableRelation[];
    focusedIndex: number;
    isSelected: (id: string) => boolean;
    onItemClick: (item: SelectableRelation) => void;
    onListKeyDown: (event: KeyboardEvent, items: SelectableRelation[]) => void;
    renderItem?: Snippet<[item: SelectableRelation, selected: boolean]>;
    onListRef: (node: HTMLUListElement | null) => void;
  }

  let {
    label,
    items,
    focusedIndex,
    isSelected,
    onItemClick,
    onListKeyDown,
    renderItem,
    onListRef
  }: Props = $props();

  let listRef: HTMLUListElement | null = $state(null);

  $effect(() => {
    onListRef(listRef);
  });
</script>

<div class="relation-selector-popover__section">
  <div class="relation-selector-popover__section-label">
    {label}
  </div>
  <ul
    bind:this={listRef}
    class="relation-selector-popover__list"
    role="listbox"
    id="relation-selector-popover-list"
    onkeydown={(event) => onListKeyDown(event, items)}
  >
    {#each items as item, index (item.id)}
      {@const selected = isSelected(item.id)}
      <li
        class="relation-selector-popover__item"
        class:relation-selector-popover__item--selected={selected}
        class:relation-selector-popover__item--disabled={item.disabled}
        class:relation-selector-popover__item--focused={focusedIndex === index}
        role="option"
        aria-selected={selected}
        onclick={() => onItemClick(item)}
        onkeydown={(event) => {
          if (event.key === "Enter" || event.key === " ") {
            event.preventDefault();
            onItemClick(item);
          }
        }}
        tabindex={item.disabled ? -1 : 0}
      >
        {#if renderItem}
          {@render renderItem(item, selected)}
        {:else}
          <div class="relation-selector-popover__item-content">
            <span class="relation-selector-popover__item-label">{item.label}</span>
            {#if item.description}
              <span class="relation-selector-popover__item-description">
                {item.description}
              </span>
            {/if}
          </div>
          {#if selected}
            <Check size="0.9em" class="relation-selector-popover__item-check" />
          {/if}
        {/if}
      </li>
    {/each}
  </ul>
</div>

<style>
  .relation-selector-popover__section {
    margin-bottom: 0.5rem;
  }

  .relation-selector-popover__section-label {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.04em;
    color: var(--underlay-color-text-muted, #9ca3af);
    margin-bottom: 0.35rem;
  }

  .relation-selector-popover__list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 0.1rem;
  }

  .relation-selector-popover__item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.4rem;
    padding: 0.4rem 0.5rem;
    border-radius: 0.3rem;
    cursor: pointer;
    user-select: none;
    font-size: 0.8rem;
    color: var(--underlay-color-text, #e5e7eb);
  }

  .relation-selector-popover__item:hover:not(.relation-selector-popover__item--disabled) {
    background: var(--underlay-color-hover-bg, rgba(148, 163, 184, 0.2));
  }

  .relation-selector-popover__item:focus-visible,
  .relation-selector-popover__item--focused {
    outline: var(--underlay-focus-ring-width, 2px) solid
      var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
  }

  .relation-selector-popover__item--selected {
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
  }

  .relation-selector-popover__item--selected:hover {
    background: var(--underlay-color-primary-strong, #1d4ed8);
  }

  .relation-selector-popover__item--disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .relation-selector-popover__item-content {
    display: flex;
    flex-direction: column;
    gap: 0.05rem;
    min-width: 0;
    flex: 1;
  }

  .relation-selector-popover__item-label {
    font-weight: 500;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-selector-popover__item-description {
    font-size: 0.75em;
    opacity: 0.7;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  :global(.relation-selector-popover__item-check) {
    flex-shrink: 0;
    opacity: 0.9;
  }
</style>
