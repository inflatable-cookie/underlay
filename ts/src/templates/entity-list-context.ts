export const UNDERLAY_ENTITY_LIST_CONTEXT_KEY = Symbol("underlayEntityList");

export interface EntityListContext {
  readonly reorderMode: boolean;
}
