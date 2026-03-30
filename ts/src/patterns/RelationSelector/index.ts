// Relation-selector helper layer retained after public UI wrapper retirement.

export {
  createRelationSelectorContext,
  useRelationSelector,
  type RelationSelectorContext
} from "./context.svelte.js";

export type {
  SelectableRelation,
  SearchResult,
  SearchOptions,
  RelationSearchFn,
  RelationSuggestionsFn,
  SuggestionOptions,
  RelationSelectorProps,
  RelationSelectorState,
  FilterOption,
  FilterConfig
} from "./types.js";

export type {
  DrillDownItem,
  DrillDownContext,
  DrillDownSearchFn,
  DrillDownSuggestionsFn,
  DrillDownLevel,
  DrillDownConfig,
  DrillDownState,
  DrillDownBreadcrumb
} from "./drilldown-types.js";
