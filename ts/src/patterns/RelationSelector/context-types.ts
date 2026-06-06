import type {
  RelationSelectorProps,
  RelationSelectorState,
  SelectableRelation,
} from "./types.js";
import type { DrillDownActions } from "./drilldown-actions.js";

export interface RelationSelectorContext<T extends SelectableRelation> {
  props: RelationSelectorProps<T>;
  state: RelationSelectorState<T>;
  selectedItem: T | null;
  selectedItems: T[];
  isMultiSelect: boolean;
  openPopover: () => void;
  closePopover: () => void;
  openModal: () => void;
  closeModal: () => void;
  switchToModal: () => void;
  setSearchQuery: (query: string) => void;
  clearSearch: () => void;
  selectItem: (item: T) => void;
  deselectItem: (itemId: string) => void;
  clearSelection: () => void;
  toggleCreateForm: () => void;
  closeCreateForm: () => void;
  handleCreateSuccess: (item: T) => void;
  performSearch: (query: string) => Promise<void>;
  retrySearch: () => Promise<void>;
  loadSuggestions: () => Promise<void>;
  retrySuggestions: () => Promise<void>;
  isSelected: (itemId: string) => boolean;
  setFilter: (filterKey: string, optionId: string | undefined) => void;
  drillDown: DrillDownActions;
}
