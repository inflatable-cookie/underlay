<script lang="ts" generics="T extends SelectableRelation">
  import type { SelectableRelation, RelationSelectorProps } from "./types.js";
  import { createRelationSelectorContext } from "./context.svelte.js";
  import { Popover as BitsPopover } from "bits-ui";
  import RelationSelectorTrigger from "./RelationSelectorTrigger.svelte";
  import RelationSelectorPopover from "./RelationSelectorPopover.svelte";
  import RelationSelectorModal from "./RelationSelectorModal.svelte";

  type Props = RelationSelectorProps<T> & {
    class?: string;
  };

  let {
    // Selection (single-select)
    value = $bindable(null),
    onchange,
    // Selection (multi-select)
    values = $bindable([]),
    onchangeMulti,
    // Mode
    mode = "single",
    // Data fetching
    search,
    suggestions,
    // Filters
    filters,
    // Labels
    label,
    placeholder,
    searchPlaceholder,
    emptyMessage,
    suggestionsLabel,
    // State
    disabled = false,
    required = false,
    error,
    // Create form
    allowCreate = false,
    createLabel,
    onCreate,
    // Selection history
    selectionHistory,
    // Snippets
    renderItem,
    renderTrigger,
    renderSelectedPill,
    createForm,
    // Extra
    class: className = ""
  }: Props = $props();

  // Create the props object for context
  const contextProps: RelationSelectorProps<T> = {
    get value() {
      return value;
    },
    onchange: (v) => {
      value = v;
      onchange?.(v);
    },
    get values() {
      return values;
    },
    onchangeMulti: (v) => {
      values = v;
      onchangeMulti?.(v);
    },
    mode,
    search,
    suggestions,
    filters,
    label,
    placeholder,
    searchPlaceholder,
    emptyMessage,
    suggestionsLabel,
    get disabled() {
      return disabled;
    },
    required,
    get error() {
      return error;
    },
    allowCreate,
    createLabel,
    onCreate,
    selectionHistory,
    renderItem,
    renderTrigger,
    renderSelectedPill,
    createForm
  };

  // Create the context - this sets up all the state and actions
  const ctx = createRelationSelectorContext(contextProps);

  function handleTriggerClick() {
    if (disabled) return;
    ctx.openPopover();
  }
</script>

<div
  class={`relation-selector ${className}`}
  class:relation-selector--disabled={disabled}
  class:relation-selector--error={error}
>
  <BitsPopover.Root
    bind:open={ctx.state.popoverOpen}
    onOpenChange={(open) => {
      if (!open) ctx.closePopover();
    }}
  >
    <BitsPopover.Trigger>
      {#snippet child({ props })}
        <button
          {...props}
          type="button"
          class="relation-selector__trigger-wrapper"
          disabled={disabled}
          onclick={handleTriggerClick}
        >
          {#if renderTrigger}
            {@render renderTrigger(
              mode === "multi" ? ctx.selectedItems : ctx.selectedItem,
              handleTriggerClick
            )}
          {:else}
            <RelationSelectorTrigger />
          {/if}
        </button>
      {/snippet}
    </BitsPopover.Trigger>

    <RelationSelectorPopover />
  </BitsPopover.Root>

  <!-- Modal for create form (opens when user clicks "Add new" in popover) -->
  <RelationSelectorModal />

  {#if error}
    <div class="relation-selector__error">{error}</div>
  {/if}
</div>

<style>
  .relation-selector {
    position: relative;
    width: 100%;
  }

  .relation-selector__trigger-wrapper {
    display: block;
    width: 100%;
    padding: 0;
    margin: 0;
    border: none;
    background: transparent;
    font: inherit;
    color: inherit;
    text-align: inherit;
    cursor: pointer;
    appearance: none;
    -webkit-appearance: none;
  }

  .relation-selector__trigger-wrapper:disabled {
    cursor: not-allowed;
  }

  .relation-selector__error {
    margin-top: 0.35rem;
    font-size: 0.8rem;
    color: var(--underlay-color-danger, #ef4444);
  }
</style>
