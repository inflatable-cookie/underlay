import type {
  DrillDownConfig,
  DrillDownBreadcrumb,
  DrillDownItem,
} from "./drilldown-types.js";
import type { FilterConfig } from "./types.js";

export interface DrillDownActions {
  drillDownSelect: (item: DrillDownItem) => void;
  drillDownBack: () => void;
  drillDownNavigateTo: (depth: number) => void;
  setDrillDownSearch: (query: string) => void;
  performDrillDownSearch: (query: string) => Promise<void>;
  loadDrillDownSuggestions: () => Promise<void>;
  setDrillDownFilter: (filterKey: string, optionId: string | undefined) => void;
  readonly isDrillDownActive: boolean;
  readonly currentDrillDownLevel: DrillDownConfig["levels"][number] | null;
  readonly drillDownBreadcrumbs: DrillDownBreadcrumb[];
  getDrillDownFilters: () => Record<string, string | undefined>;
  readonly finalLevelFilters: FilterConfig[] | null;
  resetDrillDown: () => void;
}
