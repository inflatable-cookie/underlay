<script lang="ts">
  import type { SelectableRelation } from "./types.js";
  import { useRelationSelector } from "./context.svelte.js";
  import RelationPickerDialog from "../RelationPickerDialog.svelte";
  import type { PickerSection, PickableItem } from "../RelationPickerDialog.svelte";

  const ctx = useRelationSelector<SelectableRelation>();

  let debounceTimer: ReturnType<typeof setTimeout> | null = null;

  function handleSearch(value: string) {
    ctx.setSearchQuery(value);

    // Debounce the search
    if (debounceTimer) {
      clearTimeout(debounceTimer);
    }

    if (value.trim()) {
      debounceTimer = setTimeout(() => {
        void ctx.performSearch(value);
      }, 300);
    } else {
      ctx.clearSearch();
    }
  }

  function handleSelect(item: PickableItem) {
    ctx.selectItem(item as SelectableRelation);
  }

  function handleClose() {
    ctx.closeModal();
  }

  function handleConfirm() {
    ctx.closeModal();
  }

  function handleCancel() {
    ctx.closeModal();
  }

  function handleClear() {
    ctx.clearSelection();
  }

  function handleCreate() {
    ctx.toggleCreateForm();
  }

  function handleCreateSuccess(item: PickableItem) {
    ctx.handleCreateSuccess(item as SelectableRelation);
  }

  function handleCreateCancel() {
    ctx.closeCreateForm();
  }

  function handleRetry() {
    ctx.retrySearch();
  }

  // Build sections based on current state
  const sections = $derived.by((): PickerSection[] | undefined => {
    const result: PickerSection[] = [];

    if (ctx.state.searchQuery.trim()) {
      // Show search results
      if (ctx.state.searchResults.length > 0) {
        result.push({
          label: `Results (${ctx.state.searchTotal})`,
          items: ctx.state.searchResults
        });
      }
    } else {
      // Show suggestions
      if (ctx.state.suggestionItems.length > 0) {
        result.push({
          label: ctx.props.suggestionsLabel ?? "Suggestions",
          items: ctx.state.suggestionItems
        });
      }
    }

    return result.length > 0 ? result : undefined;
  });

  // For flat items mode when no sections
  const items = $derived.by((): SelectableRelation[] => {
    if (sections) return [];
    if (ctx.state.searchQuery.trim()) {
      return ctx.state.searchResults;
    }
    return ctx.state.suggestionItems;
  });

  const selectedIds = $derived(
    ctx.isMultiSelect
      ? ctx.selectedItems.map(item => item.id)
      : ctx.selectedItem ? [ctx.selectedItem.id] : []
  );

  const hasSelection = $derived(
    ctx.isMultiSelect ? ctx.selectedItems.length > 0 : ctx.selectedItem !== null
  );

  const showClear = $derived(
    !ctx.props.required && hasSelection
  );

  const isSearching = $derived(
    ctx.state.isSearching || ctx.state.isSuggestionsLoading
  );
</script>

<RelationPickerDialog
  bind:open={ctx.state.modalOpen}
  title={ctx.props.label}
  emptyMessage={ctx.props.emptyMessage ?? "No results found"}
  {sections}
  {items}
  {selectedIds}
  searchable={true}
  searchPlaceholder={ctx.props.searchPlaceholder ?? "Search..."}
  searchQuery={ctx.state.searchQuery}
  searching={isSearching}
  onSearch={handleSearch}
  onSelect={handleSelect}
  onClose={handleClose}
  renderItem={ctx.props.renderItem}
  error={ctx.state.searchError ?? undefined}
  onRetry={handleRetry}
  {showClear}
  onClear={handleClear}
  allowCreate={ctx.props.allowCreate ?? false}
  createLabel={ctx.props.createLabel ?? "Add new"}
  onCreate={handleCreate}
  createFormOpen={ctx.state.createFormOpen}
  createForm={ctx.props.createForm}
  onCreateSuccess={handleCreateSuccess}
  onCreateCancel={handleCreateCancel}
  multiSelect={ctx.isMultiSelect}
  selectedCount={ctx.selectedItems.length}
  onConfirm={handleConfirm}
  onCancel={handleCancel}
/>
