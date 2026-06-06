import { getContext } from "svelte";
import { RELATION_SELECTOR_CONTEXT_KEY } from "./context-key.js";
import type { RelationSelectorContext } from "./context-types.js";
import type { SelectableRelation } from "./types.js";

export function useRelationSelector<
  T extends SelectableRelation,
>(): RelationSelectorContext<T> {
  const context = getContext<RelationSelectorContext<T>>(
    RELATION_SELECTOR_CONTEXT_KEY,
  );
  if (!context) {
    throw new Error(
      "useRelationSelector must be called within a RelationSelector component",
    );
  }
  return context;
}
