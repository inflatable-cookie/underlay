<script lang="ts" generics="T extends SelectableRelation">
  import type { SelectableRelation, RelationSelectorProps } from "./types.js";
  import { createRelationSelectorContext } from "./context.svelte.js";
  import { Popover as BitsPopover } from "bits-ui";
  import RelationSelectorTrigger from "./RelationSelectorTrigger.svelte";
  import RelationSelectorPopover from "./RelationSelectorPopover.svelte";
  import RelationSelectorModal from "./RelationSelectorModal.svelte";
  import Plus from "lucide-svelte/icons/plus";

  type Props = RelationSelectorProps<T> & {
    class?: string;
  };

  let {
    // Selection (single-select)
    value = $bindable(null),
    onchange,
    initialSelection = undefined,
    // Selection (multi-select)
    values = $bindable([]),
    onchangeMulti,
    initialSelections = undefined,
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

  // Create the props object for context - use getters for reactive access
  const contextProps: RelationSelectorProps<T> = {
    get value() {
      return value;
    },
    onchange: (v) => {
      value = v;
      onchange?.(v);
    },
    get initialSelection() {
      return initialSelection;
    },
    get values() {
      return values;
    },
    onchangeMulti: (v) => {
      values = v;
      onchangeMulti?.(v);
    },
    get initialSelections() {
      return initialSelections;
    },
    get mode() {
      return mode;
    },
    get search() {
      return search;
    },
    get suggestions() {
      return suggestions;
    },
    get filters() {
      return filters;
    },
    get label() {
      return label;
    },
    get placeholder() {
      return placeholder;
    },
    get searchPlaceholder() {
      return searchPlaceholder;
    },
    get emptyMessage() {
      return emptyMessage;
    },
    get suggestionsLabel() {
      return suggestionsLabel;
    },
    get disabled() {
      return disabled;
    },
    get required() {
      return required;
    },
    get error() {
      return error;
    },
    get allowCreate() {
      return allowCreate;
    },
    get createLabel() {
      return createLabel;
    },
    get onCreate() {
      return onCreate;
    },
    get selectionHistory() {
      return selectionHistory;
    },
    get renderItem() {
      return renderItem;
    },
    get renderTrigger() {
      return renderTrigger;
    },
    get renderSelectedPill() {
      return renderSelectedPill;
    },
    get createForm() {
      return createForm;
    }
  };

  // Create the context - this sets up all the state and actions
  const ctx = createRelationSelectorContext(contextProps);

  function handleTriggerClick() {
    if (disabled) return;
    ctx.openPopover();
  }

  function handleCreateClick() {
    if (disabled) return;
    ctx.switchToModal();
  }

  const showInlineCreate = $derived(allowCreate && !!createForm);
</script>

<div
  class={`relation-selector ${className}`}
  class:relation-selector--disabled={disabled}
  class:relation-selector--error={error}
>
  <div class="relation-selector__row" class:relation-selector__row--with-create={showInlineCreate}>
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

    {#if showInlineCreate}
      <button
        type="button"
        class="relation-selector__create-button"
        aria-label={createLabel ?? "Create new"}
        title={createLabel ?? "Create new"}
        disabled={disabled}
        onclick={handleCreateClick}
      >
        <Plus size="1.1em" strokeWidth={2.5} />
      </button>
    {/if}
  </div>

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

  .relation-selector__row {
    display: flex;
    align-items: stretch;
    gap: 0;
    width: 100%;
  }

  .relation-selector__row > :global(:first-child) {
    flex: 1;
    min-width: 0;
    max-width: 100%;
  }

  .relation-selector__row > :global(*) {
    min-width: 0;
  }

  .relation-selector__trigger-wrapper {
    display: block;
    width: 100%;
    min-width: 0;
    max-width: 100%;
    overflow: hidden;
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

  .relation-selector__row--with-create .relation-selector__trigger-wrapper :global(.relation-selector-trigger) {
    border-top-right-radius: 0;
    border-bottom-right-radius: 0;
  }

  .relation-selector__trigger-wrapper:disabled {
    cursor: not-allowed;
  }

  .relation-selector__create-button {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 2.5em;
    flex-shrink: 0;
    padding: 0;
    margin: 0;
    border: none;
    border-radius: 0 0.35rem 0.35rem 0;
    background: var(--underlay-color-primary, var(--underlay-color-primary, #2563eb));
    color: #fff;
    cursor: pointer;
    font-size: 0.85rem;
    transition: background-color 0.12s ease;
  }

  .relation-selector__create-button:hover:not(:disabled) {
    background: var(--underlay-color-primary-strong, var(--underlay-color-primary-strong, #1d4ed8));
  }

  .relation-selector__create-button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .relation-selector__create-button:focus-visible {
    outline: 2px solid var(--underlay-color-primary, #2563eb);
    outline-offset: -1px;
    z-index: 1;
  }

  .relation-selector__error {
    margin-top: 0.35rem;
    font-size: 0.8rem;
    color: var(--underlay-color-danger, #ef4444);
  }
</style>
