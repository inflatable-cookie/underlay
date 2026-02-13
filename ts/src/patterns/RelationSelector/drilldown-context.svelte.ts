/**
 * Drill-down state management for RelationSelector.
 *
 * Extracted from context.svelte.ts to keep file lengths manageable.
 * This module creates and manages the drill-down navigation state,
 * including level loading, search, and filter management.
 */
import type { DrillDownConfig, DrillDownItem, DrillDownState, DrillDownBreadcrumb, DrillDownContext } from "./drilldown-types.js";
import type { FilterConfig, SearchResult } from "./types.js";

/** Actions exposed by the drill-down context */
export interface DrillDownActions {
  /** Select a drill-down item and advance to the next level */
  drillDownSelect: (item: DrillDownItem) => void;
  /** Navigate back one drill-down level */
  drillDownBack: () => void;
  /** Navigate to a specific drill-down depth (for breadcrumb clicks) */
  drillDownNavigateTo: (depth: number) => void;
  /** Set search query for the current drill-down level */
  setDrillDownSearch: (query: string) => void;
  /** Perform search on the current drill-down level */
  performDrillDownSearch: (query: string) => Promise<void>;
  /** Load suggestions for the current drill-down level */
  loadDrillDownSuggestions: () => Promise<void>;
  /** Set a filter value for the current drill-down level */
  setDrillDownFilter: (filterKey: string, optionId: string | undefined) => void;
  /** Whether currently in a drill-down level (not yet at final selection) */
  readonly isDrillDownActive: boolean;
  /** Current drill-down level config, or null if at final selection */
  readonly currentDrillDownLevel: DrillDownConfig["levels"][number] | null;
  /** Breadcrumb trail for current drill-down position */
  readonly drillDownBreadcrumbs: DrillDownBreadcrumb[];
  /** Build the merged filter map (drill-down selections + explicit filters) for the final level */
  getDrillDownFilters: () => Record<string, string | undefined>;
  /** Computed filters for the final selection level (from config.finalLevelFilters), or null if not configured */
  readonly finalLevelFilters: FilterConfig[] | null;
  /** Reset drill-down state (called when popover closes) */
  resetDrillDown: () => void;
}

function createInitialDrillDownState(): DrillDownState {
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
    slideDirection: "forward"
  };
}

/**
 * Creates drill-down state management.
 * Returns the reactive state and actions.
 */
export function createDrillDownContext(
  getConfig: () => DrillDownConfig | undefined
): { state: DrillDownState; actions: DrillDownActions } {
  let drillDownState = $state<DrillDownState>(createInitialDrillDownState());
  let lastDrillDownSearchQuery = "";

  // Build context from current selections (keys from levels up to current depth)
  // Also includes current-level active filters so search/suggestions can use them
  function buildContext(): DrillDownContext {
    const config = getConfig();
    if (!config) return {};
    const ctx: DrillDownContext = {};
    for (let i = 0; i < drillDownState.depth; i++) {
      const level = config.levels[i];
      if (level) {
        const selection = drillDownState.selections[level.key];
        if (selection) {
          ctx[level.key] = selection.id;
        }
      }
    }
    // Include current-level active filters
    for (const [key, value] of Object.entries(drillDownState.activeFilters)) {
      if (value !== undefined) {
        ctx[key] = value;
      }
    }
    return ctx;
  }

  // Get initial filter values for a level
  function getLevelInitialFilters(levelIndex: number): Record<string, string | undefined> {
    const config = getConfig();
    if (!config) return {};
    const level = config.levels[levelIndex];
    if (!level?.filters) return {};
    const filters: Record<string, string | undefined> = {};
    for (const filter of level.filters) {
      filters[filter.key] = filter.defaultValue;
    }
    return filters;
  }

  const isDrillDownActive = $derived.by(() => {
    const config = getConfig();
    if (!config) return false;
    return drillDownState.depth < config.levels.length;
  });

  const currentDrillDownLevel = $derived.by(() => {
    const config = getConfig();
    if (!config) return null;
    if (drillDownState.depth >= config.levels.length) return null;
    return config.levels[drillDownState.depth] ?? null;
  });

  const drillDownBreadcrumbs = $derived.by((): DrillDownBreadcrumb[] => {
    const config = getConfig();
    if (!config) return [];
    const crumbs: DrillDownBreadcrumb[] = [];
    for (let i = 0; i < drillDownState.depth; i++) {
      const level = config.levels[i];
      if (!level) break;
      const selection = drillDownState.selections[level.key];
      if (selection) {
        crumbs.push({
          key: level.key,
          label: level.label,
          itemLabel: selection.label,
          depth: i
        });
      }
    }
    return crumbs;
  });

  function clearLevelState(): void {
    drillDownState.searchQuery = "";
    drillDownState.searchResults = [];
    drillDownState.searchTotal = 0;
    drillDownState.searching = false;
    drillDownState.searchError = null;
    drillDownState.suggestionItems = [];
    drillDownState.suggestionsLoading = false;
    lastDrillDownSearchQuery = "";
  }

  async function loadDrillDownSuggestions(): Promise<void> {
    const config = getConfig();
    if (!config) return;
    const level = config.levels[drillDownState.depth];
    if (!level?.suggestions) return;

    drillDownState.suggestionsLoading = true;
    try {
      const ctx = buildContext();
      const items = await level.suggestions(ctx);
      drillDownState.suggestionItems = items;
    } catch (error) {
      console.error("Failed to load drill-down suggestions:", error);
      drillDownState.suggestionItems = [];
    } finally {
      drillDownState.suggestionsLoading = false;
    }
  }

  async function performDrillDownSearch(query: string): Promise<void> {
    const config = getConfig();
    if (!config) return;
    const level = config.levels[drillDownState.depth];
    if (!level) return;

    if (!query.trim()) {
      drillDownState.searchResults = [];
      drillDownState.searchTotal = 0;
      drillDownState.searchError = null;
      return;
    }

    lastDrillDownSearchQuery = query;
    drillDownState.searching = true;
    drillDownState.searchError = null;

    try {
      const ctx = buildContext();
      const result: SearchResult<DrillDownItem> = await level.search(query, ctx);
      drillDownState.searchResults = result.items;
      drillDownState.searchTotal = result.total;
    } catch (error) {
      drillDownState.searchError =
        error instanceof Error ? error.message : "Search failed";
      drillDownState.searchResults = [];
      drillDownState.searchTotal = 0;
    } finally {
      drillDownState.searching = false;
    }
  }

  function drillDownSelect(item: DrillDownItem): void {
    const config = getConfig();
    if (!config) return;
    const level = config.levels[drillDownState.depth];
    if (!level) return;

    // Store the selection
    drillDownState.selections = {
      ...drillDownState.selections,
      [level.key]: item
    };
    drillDownState.slideDirection = "forward";

    // Clear level state and advance
    clearLevelState();
    drillDownState.depth += 1;
    drillDownState.activeFilters = getLevelInitialFilters(drillDownState.depth);

    // Load suggestions for the new level (if it's still a drill-down level)
    if (drillDownState.depth < config.levels.length) {
      void loadDrillDownSuggestions();
    }
  }

  function drillDownBack(): void {
    if (drillDownState.depth <= 0) return;
    const config = getConfig();
    if (!config) return;

    drillDownState.slideDirection = "back";

    // Remove the selection for the level we're leaving
    const currentLevel = config.levels[drillDownState.depth];
    if (currentLevel) {
      const newSelections = { ...drillDownState.selections };
      delete newSelections[currentLevel.key];
      drillDownState.selections = newSelections;
    }

    // Also remove the selection for the level we're going back to
    // (so the user can re-choose)
    const prevLevel = config.levels[drillDownState.depth - 1];
    if (prevLevel) {
      const newSelections = { ...drillDownState.selections };
      delete newSelections[prevLevel.key];
      drillDownState.selections = newSelections;
    }

    clearLevelState();
    drillDownState.depth -= 1;
    drillDownState.activeFilters = getLevelInitialFilters(drillDownState.depth);

    // Load suggestions for the level we're returning to
    if (drillDownState.depth < config.levels.length) {
      void loadDrillDownSuggestions();
    }
  }

  function drillDownNavigateTo(depth: number): void {
    if (depth < 0) return;
    const config = getConfig();
    if (!config) return;
    if (depth >= drillDownState.depth) return;

    drillDownState.slideDirection = "back";

    // Remove selections from the target depth onwards
    const newSelections = { ...drillDownState.selections };
    for (let i = depth; i < config.levels.length; i++) {
      const level = config.levels[i];
      if (level) {
        delete newSelections[level.key];
      }
    }
    drillDownState.selections = newSelections;

    clearLevelState();
    drillDownState.depth = depth;
    drillDownState.activeFilters = getLevelInitialFilters(depth);

    // Load suggestions for the target level
    if (depth < config.levels.length) {
      void loadDrillDownSuggestions();
    }
  }

  function setDrillDownSearch(query: string): void {
    drillDownState.searchQuery = query;
  }

  function setDrillDownFilter(filterKey: string, optionId: string | undefined): void {
    drillDownState.activeFilters = {
      ...drillDownState.activeFilters,
      [filterKey]: optionId
    };
    // Reload data with new filters
    drillDownState.searchResults = [];
    drillDownState.suggestionItems = [];
    if (drillDownState.searchQuery.trim()) {
      void performDrillDownSearch(drillDownState.searchQuery);
    } else {
      void loadDrillDownSuggestions();
    }
  }

  function getDrillDownFilters(): Record<string, string | undefined> {
    const filters: Record<string, string | undefined> = {};
    const config = getConfig();
    if (!config) return filters;
    for (const level of config.levels) {
      const selection = drillDownState.selections[level.key];
      if (selection) {
        filters[level.key] = selection.id;
      }
    }
    return filters;
  }

  function resetDrillDown(): void {
    Object.assign(drillDownState, createInitialDrillDownState());
    lastDrillDownSearchQuery = "";
  }

  const actions: DrillDownActions = {
    drillDownSelect,
    drillDownBack,
    drillDownNavigateTo,
    setDrillDownSearch,
    performDrillDownSearch,
    loadDrillDownSuggestions,
    setDrillDownFilter,
    get isDrillDownActive() {
      return isDrillDownActive;
    },
    get currentDrillDownLevel() {
      return currentDrillDownLevel;
    },
    get drillDownBreadcrumbs() {
      return drillDownBreadcrumbs;
    },
    getDrillDownFilters,
    get finalLevelFilters() {
      const config = getConfig();
      if (!config?.finalLevelFilters) return null;
      const ctx = buildContext();
      return config.finalLevelFilters(ctx);
    },
    resetDrillDown
  };

  return { state: drillDownState, actions };
}
