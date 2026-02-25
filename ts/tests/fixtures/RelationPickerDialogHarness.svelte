<script lang="ts">
  import RelationPickerDialog from "../../src/patterns/RelationPickerDialog.svelte";
  import type { PickableItem, PickerSection } from "../../src/patterns/relation-picker-types.js";

  interface Props {
    open?: boolean;
    title: string;
    emptyMessage?: string;
    items?: PickableItem[];
    sections?: PickerSection[];
    selectedIds?: string[];
    searchable?: boolean;
    searchPlaceholder?: string;
    searchQuery?: string;
    searching?: boolean;
    sectionLabel?: string;
    useRenderItem?: boolean;
    useHeaderExtra?: boolean;
    useFooterSnippet?: boolean;
    showClear?: boolean;
    allowCreate?: boolean;
    createLabel?: string;
    createFormOpen?: boolean;
    useCreateForm?: boolean;
    multiSelect?: boolean;
    selectedCount?: number;
    onSelect?: (item: PickableItem) => void;
    onClose?: () => void;
    onSearch?: (query: string) => void;
    onRetry?: () => void;
    onClear?: () => void;
    onCreate?: () => void;
    onCreateSuccess?: (item: PickableItem) => void;
    onCreateCancel?: () => void;
    onConfirm?: () => void;
    onCancel?: () => void;
    error?: string;
  }

  let {
    open = true,
    title,
    emptyMessage = "No items available.",
    items = [],
    sections = undefined,
    selectedIds = [],
    searchable = true,
    searchPlaceholder = "Search...",
    searchQuery = undefined,
    searching = false,
    sectionLabel = undefined,
    useRenderItem = false,
    useHeaderExtra = false,
    useFooterSnippet = false,
    showClear = false,
    allowCreate = false,
    createLabel = "Add new",
    createFormOpen = false,
    useCreateForm = false,
    multiSelect = false,
    selectedCount = 0,
    onSelect,
    onClose,
    onSearch,
    onRetry,
    onClear,
    onCreate,
    onCreateSuccess,
    onCreateCancel,
    onConfirm,
    onCancel,
    error = undefined,
  }: Props = $props();
</script>

{#snippet headerExtraSnippet()}
  <div data-testid="header-extra">Header Extra</div>
{/snippet}

{#snippet footerSnippet()}
  <div data-testid="custom-footer">Custom Footer</div>
{/snippet}

{#snippet renderItemSnippet(item: PickableItem, selected: boolean)}
  <div data-testid={"custom-item-" + item.id}>
    {item.label}::{selected ? "selected" : "idle"}
  </div>
{/snippet}

{#snippet createFormSnippet(onSuccess: (item: PickableItem) => void, onCancelLocal: () => void)}
  <div data-testid="create-form">
    <button
      type="button"
      data-testid="create-success"
      onclick={() => onSuccess({ id: "new-1", label: "Created item" })}
    >
      Success
    </button>
    <button type="button" data-testid="create-cancel" onclick={() => onCancelLocal()}>
      Cancel
    </button>
  </div>
{/snippet}

<RelationPickerDialog
  bind:open
  {title}
  {emptyMessage}
  {items}
  {sections}
  {selectedIds}
  {searchable}
  {searchPlaceholder}
  {searchQuery}
  {searching}
  {sectionLabel}
  onSelect={onSelect}
  onClose={onClose}
  onSearch={onSearch}
  renderItem={useRenderItem ? renderItemSnippet : undefined}
  headerExtra={useHeaderExtra ? headerExtraSnippet : undefined}
  footer={useFooterSnippet ? footerSnippet : undefined}
  {error}
  onRetry={onRetry}
  {showClear}
  onClear={onClear}
  {allowCreate}
  {createLabel}
  onCreate={onCreate}
  {createFormOpen}
  createForm={useCreateForm ? createFormSnippet : undefined}
  onCreateSuccess={onCreateSuccess}
  onCreateCancel={onCreateCancel}
  {multiSelect}
  {selectedCount}
  onConfirm={onConfirm}
  onCancel={onCancel}
/>
