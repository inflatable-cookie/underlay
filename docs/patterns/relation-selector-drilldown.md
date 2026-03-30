# Recipe: Relation Selector with Drill-Down

**Use when**: An app-local selector shell targets items from a deep hierarchy
and a flat list of hundreds of options is impractical. The user should narrow
by level (e.g., Module → Section → Outcome) before picking.

**Example prompt**: "Add drill-down navigation for Outcome selection scoped to Module → Section"

---

## Key Principle

Layer a Finder-style columnar navigation **on top** of an app-local selector
shell built over the retained Underlay helper layer:

1. **Drill-down levels** narrow the scope one hierarchy step at a time.
2. At the **final level**, the standard search/suggestions/filters take over.
3. Drill-down selections are injected into `activeFilters` automatically, so the existing `applyFilters` callback filters results without changes.
4. The `drillDown` prop is entirely opt-in — existing selector shells are unaffected.

---

## Architecture

### Component layout

```
App-local selector shell
├── (drill-down active?)
│   └── RelationSelectorDrillDown      ← hierarchy level UI
│       ├── Breadcrumb bar (back + trail)
│       ├── Level filters (optional)
│       ├── Search input
│       └── Item list (click → next level)
└── (final level)
    ├── Header ("Add outcome")
    ├── Breadcrumbs (back + trail)
    ├── Filters (effectiveFilters)
    ├── Search input
    └── Suggestion / search result list
```

### Data flow

```
DrillDownConfig.levels[]          Consumer provides level definitions
        │
        ▼
drilldown-context.svelte.ts       Manages depth, selections, search, filters
        │
        ▼
buildContext()                     Merges prior selections + current filters
        │                          into DrillDownContext
        ▼
level.search(query, ctx)          Consumer's search fn receives context
level.suggestions(ctx)            Consumer's suggest fn receives context
        │
        ▼
(user clicks item)                drillDownSelect() advances depth
        │
        ▼
(depth === levels.length)         Final level reached → relay to local selector shell
        │                          Drill-down keys injected into activeFilters
        ▼
applyFilters(items, filters)      Consumer's existing filter callback sees
                                   { module: "...", section: "...", area: "..." }
```

### File map

| File | Purpose |
|------|---------|
| `drilldown-types.ts` | All type definitions (`DrillDownConfig`, `DrillDownLevel`, `DrillDownItem`, etc.) |
| `drilldown-context.svelte.ts` | Reactive state management (`createDrillDownContext`) |
| `drilldown-search.ts` | `createLocalDrillDownSearchFns` helper for client-side levels |
| `RelationSelectorDrillDown.svelte` | Drill-down level UI (breadcrumbs, search, item list) |

---

## Type Reference

### DrillDownConfig

The top-level configuration passed to an app-local selector shell via the
`drillDown` prop.

```typescript
interface DrillDownConfig {
  /** Hierarchy levels in order. The final selection uses existing RelationSelector props. */
  levels: DrillDownLevel[];

  /**
   * Optional callback to compute filters for the final selection level
   * based on drill-down context. Overrides the `filters` prop at the final level.
   */
  finalLevelFilters?: (context: DrillDownContext) => FilterConfig[];
}
```

### DrillDownLevel

Configuration for one hierarchy step.

```typescript
interface DrillDownLevel {
  /** Unique key — also becomes the filter key injected into activeFilters */
  key: string;
  /** Display label (breadcrumbs and header) */
  label: string;
  /** Search function for this level */
  search: DrillDownSearchFn;
  /** Optional suggestions function (called when level first shown) */
  suggestions?: DrillDownSuggestionsFn;
  /** Optional placeholder for this level's search input */
  searchPlaceholder?: string;
  /** Optional filter dropdowns shown at this level */
  filters?: FilterConfig[];
}
```

### DrillDownItem

Items displayed at each drill-down level. Extends `SelectableRelation`.

```typescript
interface DrillDownItem extends SelectableRelation {
  /** Optional child count (shown as badge) */
  count?: number;
}
```

### DrillDownContext

Passed to every search/suggestions function. Contains prior-level selections **and** current-level filter values.

```typescript
type DrillDownContext = Record<string, string>;
// Example at Section level after selecting a module with a pathway filter:
// { module: "mod-uuid-123", pathway: "pw-uuid-456" }
```

### Function signatures

```typescript
type DrillDownSearchFn = (
  query: string,
  context: DrillDownContext
) => Promise<SearchResult<DrillDownItem>>;

type DrillDownSuggestionsFn = (
  context: DrillDownContext
) => Promise<DrillDownItem[]>;
```

---

## Checklist

### Phase 1: Derive Hierarchy Data

- [ ] Identify the hierarchy levels (e.g., Module → Section → Outcome)
- [ ] Decide which levels are drill-down steps vs. filter dropdowns
- [ ] Derive unique items for each level from the available data
- [ ] Include child counts for each derived item where possible

**Typical pattern** — derive from a flat denormalized list:

```typescript
interface DerivedModule {
  moduleId: string;
  code: string;
  title: string;
  pathwayId: string;
  pathwayName: string;
  outcomeCount: number;
}

const derivedModules = $derived.by((): DerivedModule[] => {
  const map = new Map<string, { count: number }>();
  for (const o of availableOutcomes) {
    const existing = map.get(o.moduleId);
    if (existing) existing.count++;
    else map.set(o.moduleId, { count: 1 });
  }
  // ... build result array, enrich from modules lookup, sort
});
```

### Phase 2: Build Level Search Functions

- [ ] Create search/suggest functions for each drill-down level
- [ ] Use `createLocalDrillDownSearchFns` for client-side data
- [ ] Use `applyContext` to scope items by prior-level selections
- [ ] For remote data, write custom async functions matching `DrillDownSearchFn` / `DrillDownSuggestionsFn`

**Client-side helper:**

```typescript
import { createLocalDrillDownSearchFns } from "@decodelabs/underlay/runtime";

const moduleDrillDown = $derived(
  createLocalDrillDownSearchFns(() => derivedModules, {
    toItem: (mod) => ({
      id: mod.moduleId,
      label: mod.code,
      description: mod.title,
      count: mod.outcomeCount
    }),
    getSearchText: (mod) => [mod.code, mod.title, mod.pathwayName],
    applyContext: (items, ctx) =>
      ctx.pathway ? items.filter((m) => m.pathwayId === ctx.pathway) : items
  })
);
```

**`createLocalDrillDownSearchFns` options:**

| Option | Type | Required | Description |
|--------|------|----------|-------------|
| `toItem` | `(item: T) => DrillDownItem` | Yes | Convert source item to drill-down item |
| `getSearchText` | `(item: T) => string[]` | Yes | Strings to match against (case-insensitive) |
| `applyContext` | `(items: T[], ctx) => T[]` | No | Filter by prior selections + current filters |
| `maxSuggestions` | `number` | No | Cap suggestion count (default: all) |

### Phase 3: Configure Drill-Down

- [ ] Build `DrillDownConfig` with `levels` array
- [ ] Add `finalLevelFilters` if the final level's filter dropdown needs scoping by prior selections
- [ ] Handle conditional config (e.g., skip module level when module is pre-selected)
- [ ] Return `undefined` when drill-down is unnecessary (≤1 item at first level)

**Full example:**

```typescript
const drillDownConfig = $derived.by((): DrillDownConfig | undefined => {
  // Skip drill-down when module is pre-selected
  if (isModuleConstrained) {
    const moduleSections = derivedSections.filter(
      (s) => s.moduleId === selectedModuleId
    );
    if (moduleSections.length <= 1) return undefined;
    return {
      levels: [{
        key: "section",
        label: "Section",
        search: sectionDrillDown.search,
        suggestions: sectionDrillDown.suggest,
        searchPlaceholder: "Search sections..."
      }],
      finalLevelFilters: buildAreaFilters
    };
  }

  // Full drill-down: Module → Section → Outcomes
  if (derivedModules.length <= 1) return undefined;
  return {
    levels: [
      {
        key: "module",
        label: "Module",
        search: moduleDrillDown.search,
        suggestions: moduleDrillDown.suggest,
        searchPlaceholder: "Search modules...",
        filters: pathwayFilterConfig   // ← dropdown filter at this level
      },
      {
        key: "section",
        label: "Section",
        search: sectionDrillDown.search,
        suggestions: sectionDrillDown.suggest,
        searchPlaceholder: "Search sections..."
      }
    ],
    finalLevelFilters: buildAreaFilters
  };
});
```

### Phase 4: Wire to Your Local Selector Shell

- [ ] Pass `drillDown={drillDownConfig}` to your local selector shell
- [ ] Update `applyFilters` to handle filter keys from all drill-down levels
- [ ] Set `filters` to the fallback filters (used when drill-down is not active)

```svelte
<OutcomeSelector
  label="Add outcome"
  {search}
  suggestions={suggest}
  drillDown={drillDownConfig}
  filters={moduleFilterConfig}
  ...
/>
```

**Critical: `applyFilters` must handle drill-down keys.** When the drill-down completes, selections are injected as filter keys (e.g., `filters.module`, `filters.section`). The existing `applyFilters` callback must filter by them:

```typescript
applyFilters: (items, filters) => {
  let filtered = items;
  if (filters?.module) {
    filtered = filtered.filter((o) => o.moduleId === filters.module);
  }
  if (filters?.section) {
    filtered = filtered.filter((o) => o.sectionId === filters.section);
  }
  if (filters?.area) {
    filtered = filtered.filter((o) => o.areaId === filters.area);
  }
  return filtered;
}
```

### Phase 5: Scoped Final-Level Filters

- [ ] If the final level needs filter dropdowns scoped by drill-down selections, use `finalLevelFilters`
- [ ] Build a function that receives `DrillDownContext` and returns `FilterConfig[]`
- [ ] Use compound labels where helpful (e.g., `${sectionLabel}${areaLabel}` → "A1")

```typescript
function buildAreaFilters(ddCtx: DrillDownContext): FilterConfig[] {
  let scoped = availableOutcomes;
  if (ddCtx.module) scoped = scoped.filter((o) => o.moduleId === ddCtx.module);
  if (ddCtx.section) scoped = scoped.filter((o) => o.sectionId === ddCtx.section);

  const seen = new Set<string>();
  const options: Array<{ id: string; label: string }> = [];
  for (const o of scoped) {
    if (!o.areaId || seen.has(o.areaId)) continue;
    seen.add(o.areaId);
    options.push({ id: o.areaId, label: `${o.sectionLabel}${o.areaLabel}` });
  }
  options.sort((a, b) => a.label.localeCompare(b.label, undefined, {
    numeric: true, sensitivity: "base"
  }));
  if (options.length <= 1) return [];

  return [{
    key: "area", label: "Area", options,
    includeAll: true, allLabel: "All areas"
  }];
}
```

### Phase 6: UX Guardrails

- [ ] Return `undefined` from the config when drill-down is unnecessary (≤1 option at first level)
- [ ] Handle constrained cases (e.g., `moduleId` prop) by skipping levels or reducing the hierarchy
- [ ] Use `count` on `DrillDownItem` to show child counts as badges
- [ ] Provide `searchPlaceholder` per level for clear affordance

---

## Imports

All drill-down types and helpers are exported from `@decodelabs/underlay/runtime`:

```typescript
import {
  RelationSelector,
  createLocalSearchFns,
  createLocalDrillDownSearchFns,
  type DrillDownConfig,
  type DrillDownContext,
  type DrillDownItem,
  type DrillDownLevel,
  type DrillDownSearchFn,
  type DrillDownSuggestionsFn,
  type FilterConfig,
  type SelectableRelation
} from "@decodelabs/underlay/runtime";
```

---

## How It Works Internally

1. **`drillDown` prop** → `context.svelte.ts` creates a `DrillDownContext` via `createDrillDownContext()`.
2. **Popover opens** → if drill-down is configured, `isDrillDownActive` is true and the drill-down UI renders the first level.
3. **User clicks an item** → `drillDownSelect(item)` stores the selection and advances `depth`.
4. **At each level** → `buildContext()` assembles a `DrillDownContext` containing all prior selections + current-level filter values. This context is passed to `search()` and `suggestions()`.
5. **Final level reached** (`depth === levels.length`) → drill-down UI hides and the final selector level takes over. Drill-down selections are merged into `activeFilters` via `getDrillDownFilters()`.
6. **`finalLevelFilters`** (if configured) → computes filter dropdown options scoped by drill-down context. Overrides `props.filters` at the final level.
7. **`applyFilters`** → receives merged filters (drill-down selections + explicit filters) and filters the final item list.
8. **Breadcrumbs** → shown at the final level so the user can navigate back. Clicking a breadcrumb resets to that depth.
9. **Popover closes** → drill-down state resets to initial.

---

## References in Acowtancy

### Detail views (single-select, + button trigger)
- `dairy/src/lib/cards/QuestionOutcomesInlineList.svelte` — shared by digital exam, written exam, and quiz question detail views

### Form views (multi-select)
- `dairy/src/lib/forms/content/QuizQuestionOutcomesSelector.svelte` — shared by digital exam, written exam, and quiz question forms

### Usage by question type

| Question type | Detail view | Form | Module constrained? |
|--------------|-------------|------|---------------------|
| Digital exam | QuestionOutcomesInlineList | QuizQuestionOutcomesSelector | Yes (`moduleId` prop) |
| Written exam | QuestionOutcomesInlineList | QuizQuestionOutcomesSelector | Yes (`moduleId` prop) |
| Quiz | QuestionOutcomesInlineList | QuizQuestionOutcomesSelector | No (full Module → Section drill-down) |
