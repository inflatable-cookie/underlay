import type {
  DrillDownConfig,
  DrillDownItem,
  DrillDownState,
} from "./drilldown-types.js";
import type { SearchResult } from "./types.js";
import type { DrillDownActions } from "./drilldown-actions.js";
import {
  buildDrillDownBreadcrumbs,
  buildDrillDownContext,
  createInitialDrillDownState,
  getDrillDownSelectionFilters,
  getLevelInitialFilters,
} from "./drilldown-state.js";

export type { DrillDownActions } from "./drilldown-actions.js";

/**
 * Creates drill-down state management.
 * Returns the reactive state and actions.
 */
export function createDrillDownContext(
  getConfig: () => DrillDownConfig | undefined,
): { state: DrillDownState; actions: DrillDownActions } {
  let drillDownState = $state<DrillDownState>(createInitialDrillDownState());
  let lastDrillDownSearchQuery = "";

  const buildContext = () => buildDrillDownContext(getConfig(), drillDownState);
  const buildLevelFilters = (levelIndex: number) =>
    getLevelInitialFilters(getConfig(), levelIndex);

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
    return buildDrillDownBreadcrumbs(config, drillDownState);
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
      const result: SearchResult<DrillDownItem> = await level.search(
        query,
        ctx,
      );
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
      [level.key]: item,
    };
    drillDownState.slideDirection = "forward";

    // Clear level state and advance
    clearLevelState();
    drillDownState.depth += 1;
    drillDownState.activeFilters = buildLevelFilters(drillDownState.depth);

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
    drillDownState.activeFilters = buildLevelFilters(drillDownState.depth);

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
    drillDownState.activeFilters = buildLevelFilters(depth);

    // Load suggestions for the target level
    if (depth < config.levels.length) {
      void loadDrillDownSuggestions();
    }
  }

  function setDrillDownSearch(query: string): void {
    drillDownState.searchQuery = query;
  }

  function setDrillDownFilter(
    filterKey: string,
    optionId: string | undefined,
  ): void {
    drillDownState.activeFilters = {
      ...drillDownState.activeFilters,
      [filterKey]: optionId,
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
    return getDrillDownSelectionFilters(getConfig(), drillDownState);
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
    resetDrillDown,
  };

  return { state: drillDownState, actions };
}
