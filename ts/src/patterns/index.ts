// Patterns are higher-level, app-agnostic UI building blocks.

export * from "./toasts";
export * from "./clipboard";
export * from "./dom";
export * from "./auth";
export * from "./forms";
export * from "./storage";
export * from "./navigation";
export * from "./i18n";
export * from "./optimistic";
export * from "./banner";
export * from "./slugify";
export * from "./selection-history";
export {
  createLocalSearchFns,
  type LocalSearchOptions,
  type LocalSearchFns
} from "./local-search";

// Reorder utilities
export {
  createReorderController,
  type ReorderController,
  type ReorderableItem
} from "./reorder-controller.svelte";

export { default as FilterBar } from "./FilterBar.svelte";
export { default as FormShell } from "./FormShell.svelte";
export { default as PageHeader } from "./PageHeader.svelte";
export type { PageHeaderLevel, BreadcrumbItem } from "./types";
export { default as CopyActionsMenu } from "./CopyActionsMenu.svelte";
export { default as CardActions } from "./CardActions.svelte";
export { default as SubmitButton } from "./SubmitButton.svelte";
export { default as NavCard } from "./NavCard.svelte";
export { default as NavCardGrid } from "./NavCardGrid.svelte";
export { default as Banner } from "./Banner.svelte";
export { default as ReorderableList } from "./ReorderableList.svelte";
export { default as SlugField } from "./SlugField.svelte";
export { default as SpaFormShell } from "./SpaFormShell.svelte";
export type { SpaFormResult, SpaSubmitHandler, SpaNavigateFn } from "./spa-form-types";

// FormDialog pattern
export { FormDialog } from "./FormDialog/index.js";

// RelationSelector pattern
export {
  RelationSelector,
  RelationSelectorTrigger,
  RelationSelectorModal,
  createRelationSelectorContext,
  useRelationSelector,
  type RelationSelectorContext,
  type SelectableRelation,
  type SearchResult,
  type SearchOptions,
  type RelationSearchFn,
  type RelationSuggestionsFn,
  type SuggestionOptions,
  type RelationSelectorProps,
  type RelationSelectorState,
  type FilterOption,
  type FilterConfig
} from "./RelationSelector/index.js";

// RelationPickerDialog - item picker dialog (base component for relation selection)
export {
  default as RelationPickerDialog,
  type PickableItem,
  type PickerSection
} from "./RelationPickerDialog.svelte";

// Explicit export avoids dev-time prebundle staleness.
export { useToasts } from "./useToasts";

// Authenticated data fetching pattern
export {
  useAuthenticatedData,
  type AuthenticatedDataOptions,
  type AuthenticatedDataResult
} from "./authenticated-data.svelte";

// Selection state management
export {
  useSyncedSelection,
  type SyncedSelectionResult
} from "./synced-selection.svelte";
