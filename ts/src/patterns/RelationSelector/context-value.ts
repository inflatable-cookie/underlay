import type { DrillDownActions } from "./drilldown-actions.js";
import type { RelationSelectorContext } from "./context-types.js";
import type {
  RelationSelectorProps,
  RelationSelectorState,
  SelectableRelation,
} from "./types.js";

type RelationSelectorActions<T extends SelectableRelation> = Pick<
  RelationSelectorContext<T>,
  | "openPopover"
  | "closePopover"
  | "openModal"
  | "closeModal"
  | "switchToModal"
  | "setSearchQuery"
  | "clearSearch"
  | "selectItem"
  | "deselectItem"
  | "clearSelection"
  | "toggleCreateForm"
  | "closeCreateForm"
  | "handleCreateSuccess"
  | "performSearch"
  | "retrySearch"
  | "loadSuggestions"
  | "retrySuggestions"
  | "isSelected"
  | "setFilter"
>;

export interface RelationSelectorContextValueOptions<
  T extends SelectableRelation,
> {
  props: RelationSelectorProps<T>;
  state: RelationSelectorState<T>;
  getSelectedItem: () => T | null;
  getSelectedItems: () => T[];
  getIsMultiSelect: () => boolean;
  actions: RelationSelectorActions<T>;
  drillDown: DrillDownActions;
}

export function createRelationSelectorContextValue<
  T extends SelectableRelation,
>(options: RelationSelectorContextValueOptions<T>): RelationSelectorContext<T> {
  return {
    get props() {
      return options.props;
    },
    get state() {
      return options.state;
    },
    get selectedItem() {
      return options.getSelectedItem();
    },
    get selectedItems() {
      return options.getSelectedItems();
    },
    get isMultiSelect() {
      return options.getIsMultiSelect();
    },
    ...options.actions,
    drillDown: options.drillDown,
  };
}
