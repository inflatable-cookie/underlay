import type {
  DrillDownBreadcrumb,
  DrillDownConfig,
  DrillDownContext,
  DrillDownState,
} from "./drilldown-types.js";

export function createInitialDrillDownState(): DrillDownState {
  return {
    depth: 0,
    selections: {},
    suggestionItems: [],
    suggestionsLoading: false,
    searchQuery: "",
    searchResults: [],
    searchTotal: 0,
    searching: false,
    searchError: null,
    activeFilters: {},
    slideDirection: "forward",
  };
}

export function buildDrillDownContext(
  config: DrillDownConfig | undefined,
  state: DrillDownState,
): DrillDownContext {
  if (!config) return {};

  const ctx: DrillDownContext = {};
  for (let i = 0; i < state.depth; i++) {
    const level = config.levels[i];
    if (!level) continue;
    const selection = state.selections[level.key];
    if (selection) {
      ctx[level.key] = selection.id;
    }
  }

  for (const [key, value] of Object.entries(state.activeFilters)) {
    if (value !== undefined) {
      ctx[key] = value;
    }
  }
  return ctx;
}

export function getLevelInitialFilters(
  config: DrillDownConfig | undefined,
  levelIndex: number,
): Record<string, string | undefined> {
  const level = config?.levels[levelIndex];
  if (!level?.filters) return {};

  const filters: Record<string, string | undefined> = {};
  for (const filter of level.filters) {
    filters[filter.key] = filter.defaultValue;
  }
  return filters;
}

export function buildDrillDownBreadcrumbs(
  config: DrillDownConfig | undefined,
  state: DrillDownState,
): DrillDownBreadcrumb[] {
  if (!config) return [];

  const crumbs: DrillDownBreadcrumb[] = [];
  for (let i = 0; i < state.depth; i++) {
    const level = config.levels[i];
    if (!level) break;
    const selection = state.selections[level.key];
    if (selection) {
      crumbs.push({
        key: level.key,
        label: level.label,
        itemLabel: selection.label,
        depth: i,
      });
    }
  }
  return crumbs;
}

export function getDrillDownSelectionFilters(
  config: DrillDownConfig | undefined,
  state: DrillDownState,
): Record<string, string | undefined> {
  const filters: Record<string, string | undefined> = {};
  if (!config) return filters;

  for (const level of config.levels) {
    const selection = state.selections[level.key];
    if (selection) {
      filters[level.key] = selection.id;
    }
  }
  return filters;
}
