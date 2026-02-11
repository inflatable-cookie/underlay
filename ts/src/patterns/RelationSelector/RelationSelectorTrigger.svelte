<script lang="ts">
  import ChevronDown from "lucide-svelte/icons/chevron-down";
  import X from "lucide-svelte/icons/x";
  import type { SelectableRelation } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";

  interface Props {
    class?: string;
  }

  let { class: className = "" }: Props = $props();

  const ctx = useRelationSelector<SelectableRelation>();

  const displayText = $derived.by(() => {
    if (ctx.isMultiSelect) {
      const count = ctx.selectedItems.length;
      if (count === 0) return null;
      if (count === 1) return ctx.selectedItems[0]?.label;
      return `${count} selected`;
    }
    return ctx.selectedItem?.label ?? null;
  });

  const hasSelection = $derived(
    ctx.isMultiSelect ? ctx.selectedItems.length > 0 : ctx.selectedItem !== null
  );

  const showClearButton = $derived(
    !ctx.props.required && hasSelection && !ctx.props.disabled
  );

  function handleClear(event: MouseEvent) {
    event.stopPropagation();
    event.preventDefault();
    ctx.clearSelection();
  }

  function handleRemovePill(itemId: string, event: MouseEvent) {
    event.stopPropagation();
    event.preventDefault();
    ctx.deselectItem(itemId);
  }
</script>

<div
  class={`relation-selector-trigger ${className}`}
  class:relation-selector-trigger--disabled={ctx.props.disabled}
  class:relation-selector-trigger--error={ctx.props.error}
  class:relation-selector-trigger--multi={ctx.isMultiSelect && ctx.selectedItems.length > 0}
  aria-expanded={ctx.state.modalOpen}
  aria-haspopup="listbox"
  aria-label={ctx.props.placeholder}
>
  {#if ctx.isMultiSelect && ctx.selectedItems.length > 0}
    <div class="relation-selector-trigger__pills">
      {#each ctx.selectedItems.slice(0, 3) as item (item.id)}
        <span class="relation-selector-trigger__pill">
          <span class="relation-selector-trigger__pill-label">{item.label}</span>
          <button
            type="button"
            class="relation-selector-trigger__pill-remove"
            aria-label={`Remove ${item.label}`}
            onclick={(e) => handleRemovePill(item.id, e)}
          >
            <X size="0.85em" strokeWidth={2.5} />
          </button>
        </span>
      {/each}
      {#if ctx.selectedItems.length > 3}
        <span class="relation-selector-trigger__more">
          +{ctx.selectedItems.length - 3} more
        </span>
      {/if}
    </div>
  {:else}
    <span class="relation-selector-trigger__text" class:placeholder={!hasSelection}>
      {displayText ?? ctx.props.placeholder ?? "Select..."}
    </span>
  {/if}

  <span class="relation-selector-trigger__controls">
    {#if showClearButton}
      <button
        type="button"
        class="relation-selector-trigger__clear"
        aria-label="Clear selection"
        onclick={handleClear}
        onpointerdown={(e) => e.stopPropagation()}
        onmousedown={(e) => e.stopPropagation()}
      >
        <X size="1em" strokeWidth={2.5} />
      </button>
    {/if}
    <ChevronDown size="1em" strokeWidth={2.5} class="relation-selector-trigger__chevron" />
  </span>
</div>

<style>
  .relation-selector-trigger {
    width: 100%;
    min-width: 0;
    max-width: 100%;
    display: inline-flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
    box-sizing: border-box;
    padding: var(--underlay-field-padding-block, 0.55em)
      var(--underlay-field-padding-inline, 0.7em);
    border-radius: 0.35rem;
    border: none;
    background: var(--underlay-color-field-bg, rgba(148, 163, 184, 0.18));
    color: var(--underlay-color-text, #e5e7eb);
    font-size: 0.85rem;
    cursor: pointer;
    text-align: left;
    min-height: 2.5em;
  }

  .relation-selector-trigger:hover:not(.relation-selector-trigger--disabled) {
    background: var(--underlay-color-field-bg-hover, rgba(148, 163, 184, 0.25));
  }

  .relation-selector-trigger--disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .relation-selector-trigger--error {
    outline: 1px solid var(--underlay-color-danger, #ef4444);
    outline-offset: -1px;
  }

  .relation-selector-trigger--multi {
    padding: 0.35em 0.7em;
  }

  .relation-selector-trigger__text {
    flex: 1;
    min-width: 0;
    display: block;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-selector-trigger__text.placeholder {
    color: var(--underlay-color-text-muted, #9ca3af);
  }

  .relation-selector-trigger__controls {
    display: flex;
    align-items: center;
    gap: 0.35rem;
    flex-shrink: 0;
  }

  .relation-selector-trigger__clear {
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

  .relation-selector-trigger__clear:hover {
    opacity: 1;
    color: var(--underlay-color-danger, #ef4444);
  }

  .relation-selector-trigger__clear:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: 1px;
  }

  :global(.relation-selector-trigger__chevron) {
    flex-shrink: 0;
    color: var(--underlay-color-text, #e5e7eb);
    opacity: 0.5;
    margin-top: 1px;
  }

  .relation-selector-trigger__pills {
    display: flex;
    flex-wrap: wrap;
    gap: 0.35rem;
    flex: 1;
    min-width: 0;
  }

  .relation-selector-trigger__pill {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    padding: 0.15em 0.45em;
    border-radius: 0.25rem;
    background: var(--underlay-color-primary, #2563eb);
    color: var(--underlay-color-on-primary, white);
    font-size: 0.8em;
    max-width: 10rem;
    min-width: 0;
  }

  .relation-selector-trigger__pill-label {
    display: block;
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .relation-selector-trigger__pill-remove {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    margin: 0;
    margin-left: 0.1em;
    border: none;
    background: transparent;
    color: inherit;
    cursor: pointer;
    border-radius: 0.15rem;
    opacity: 0.7;
    transition: opacity 0.1s ease;
  }

  .relation-selector-trigger__pill-remove:hover {
    opacity: 1;
  }

  .relation-selector-trigger__more {
    display: inline-flex;
    align-items: center;
    padding: 0.15em 0.4em;
    font-size: 0.8em;
    color: var(--underlay-color-text-muted, #9ca3af);
  }
</style>
