// RelationSelector - A popover/modal relation picker component
export { default as RelationSelector } from "./RelationSelector.svelte";
export { default as RelationSelectorTrigger } from "./RelationSelectorTrigger.svelte";
export { default as RelationSelectorPopover } from "./RelationSelectorPopover.svelte";
export { default as RelationSelectorModal } from "./RelationSelectorModal.svelte";

export {
  createRelationSelectorContext,
  useRelationSelector,
  type RelationSelectorContext
} from "./context.svelte.js";

export type {
  SelectableRelation,
  SearchResult,
  RelationSearchFn,
  RelationSuggestionsFn,
  RelationSelectorProps,
  RelationSelectorState
} from "./types.js";
