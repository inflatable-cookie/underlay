import type { DrillDownActions } from "./drilldown-actions.js";
import type { RelationSelectorState, SelectableRelation } from "./types.js";

export interface RelationSelectorUiActionOptions<T extends SelectableRelation> {
  state: RelationSelectorState<T>;
  drillDown: DrillDownActions;
  hasDrillDown: () => boolean;
  hasSuggestions: () => boolean;
  loadSuggestions: () => void;
}

export function createRelationSelectorUiActions<T extends SelectableRelation>(
  options: RelationSelectorUiActionOptions<T>,
) {
  const { state, drillDown, hasDrillDown, hasSuggestions, loadSuggestions } =
    options;

  function openPopover() {
    state.popoverOpen = true;
    if (drillDown.isDrillDownActive) {
      void drillDown.loadDrillDownSuggestions();
    } else if (hasSuggestions() && state.suggestionItems.length === 0) {
      loadSuggestions();
    }
  }

  function closePopover() {
    state.popoverOpen = false;
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
    if (hasDrillDown()) {
      drillDown.resetDrillDown();
    }
  }

  function openModal() {
    state.modalOpen = true;
    if (hasSuggestions() && state.suggestionItems.length === 0) {
      loadSuggestions();
    }
  }

  function closeModal() {
    state.modalOpen = false;
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
    state.createFormOpen = false;
  }

  function switchToModal() {
    state.popoverOpen = false;
    state.modalOpen = true;
    state.createFormOpen = true;
  }

  function setSearchQuery(query: string) {
    state.searchQuery = query;
  }

  function clearSearch() {
    state.searchQuery = "";
    state.searchResults = [];
    state.searchError = null;
  }

  function toggleCreateForm() {
    if (state.popoverOpen) {
      switchToModal();
    } else {
      state.createFormOpen = !state.createFormOpen;
    }
  }

  function closeCreateForm() {
    state.createFormOpen = false;
  }

  return {
    openPopover,
    closePopover,
    openModal,
    closeModal,
    switchToModal,
    setSearchQuery,
    clearSearch,
    toggleCreateForm,
    closeCreateForm,
  };
}
