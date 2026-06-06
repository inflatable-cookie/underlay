# Selection History & Server-Curated Suggestions

This guide covers Underlay's infrastructure for intelligent suggestions in relation selectors. The pattern tracks user selections client-side and sends them as hints to the server, which returns a prioritized list of suggestions.

UI note:
- selector and picker-shell composition now belongs in Poodle plus app-local
  code
- use this guide for the retained suggestion/history runtime only
- use Poodle’s picker/detail/dialog guides for the visible selector posture

## Overview

When users work with relation selectors (e.g., selecting a Level for a Module), showing intelligent suggestions improves UX significantly. Rather than always showing items alphabetically or by creation date, we can prioritize items the user has recently selected.

### The Pattern

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           CLIENT SIDE                                   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ User selects    │───▶│ SelectionHistory │───▶│ localStorage     │   │
│  │ "Level A"       │    │ .track(id)       │    │ stores ID        │   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ User opens      │───▶│ SelectionHistory │───▶│ API request with │   │
│  │ selector again  │    │ .getRecentIds()  │    │ ?recentHints=... │   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           SERVER SIDE                                   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ Parse query     │───▶│ SuggestionQuery  │───▶│ SQL ORDER BY     │   │
│  │ params          │    │ builder          │    │ hint priority    │   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Components

| Layer | Component | Package | Purpose |
|-------|-----------|---------|---------|
| Client Storage | `createSelectionHistory()` | `@decodelabs/underlay/runtime/data` | Track selections in localStorage |
| UI Integration | `selectionHistory` prop | App-local selector shell | Auto-track and provide hints |
| Request Building | `appendSuggestionParams()` | `@decodelabs/underlay/client/suggestions` | Build API URLs with hints |
| Server Parsing | `SuggestionParams` | `underlay-suggestions` (Rust) | Parse query parameters |
| Query Building | `SuggestionQuery` | `underlay-suggestions` (Rust) | Build prioritized SQL |

## Client-Side: Selection History

### Creating a History Tracker

Create a history tracker for each entity type you want to track:

```typescript
// src/lib/stores/selection-history.ts
import { createSelectionHistory } from "@decodelabs/underlay/runtime/data";

// Track recently selected levels
export const levelSelectionHistory = createSelectionHistory("learning.levels", {
  maxItems: 20,        // Keep last 20 selections (default)
  storageType: "local" // Persist across sessions (default)
});

// Track recently selected pathways
export const pathwaySelectionHistory = createSelectionHistory("learning.pathways", {
  maxItems: 10,
  storageType: "local"
});

// Session-only tracking (cleared when tab closes)
export const tempHistory = createSelectionHistory("temp.items", {
  storageType: "session"
});
```

### Storage Key Format

History is stored in localStorage/sessionStorage with keys like:
```
underlay:selection-history:learning.levels
underlay:selection-history:learning.pathways
```

### API Reference

```typescript
interface SelectionHistory {
  // Track a selection (moves to front if already exists)
  track(id: string): void;

  // Track multiple selections at once
  trackMultiple(ids: string[]): void;

  // Get recent IDs (most recent first)
  getRecentIds(limit?: number): string[];

  // Check if an ID is in history
  hasRecent(id: string): boolean;

  // Remove an ID (e.g., if item was deleted)
  remove(id: string): void;

  // Clear all history for this tracker
  clear(): void;

  // Get the storage key (for debugging)
  getStorageKey(): string;
}
```

### Manual Usage

While RelationSelector handles tracking automatically, you can use the history directly:

```typescript
import { levelSelectionHistory } from "$lib/stores/selection-history";

// Track a selection
levelSelectionHistory.track("level-123");

// Get hints for an API call
const hints = levelSelectionHistory.getRecentIds(5); // Last 5 selections

// Check if something was recently used
if (levelSelectionHistory.hasRecent("level-456")) {
  // Show as "recent" in UI
}

// Clean up after deletion
levelSelectionHistory.remove("deleted-level-id");
```

## UI Integration: App-Local Selector Shell

### Automatic Tracking

Pass a `selectionHistory` prop through your app-local selector shell for
automatic tracking:

```svelte
<script lang="ts">
  import CategorySelector from "$lib/components/CategorySelector.svelte";
  import type { SuggestionOptions } from "@decodelabs/underlay/runtime/relations";
  import { levelSelectionHistory } from "$lib/stores/selection-history";
  import { api } from "$lib/api";

  let levelId = $state<string | null>(null);

  // Suggestions function receives hints automatically
  async function suggestLevels(options?: SuggestionOptions) {
    return await api.getLevels({
      suggestions: true,
      recentHints: options?.recentHints // Passed automatically by RelationSelector
    });
  }
</script>

<CategorySelector
  label="Select Level"
  bind:value={levelId}
  search={searchLevels}
  suggestions={suggestLevels}
  selectionHistory={levelSelectionHistory}
/>
```

### What Happens Automatically

1. **On selection**: `selectionHistory.track(item.id)` is called
2. **On open**: `selectionHistory.getRecentIds()` is called and passed to `suggestions()`
3. **On create**: New items are tracked when created via the app-local inline-create flow

### SuggestionOptions Type

```typescript
interface SuggestionOptions {
  /** Recent selection IDs as hints for server-side ordering */
  recentHints?: string[];
}
```

## Request Building

### Using appendSuggestionParams

Build API URLs with suggestion parameters:

```typescript
import {
  appendSuggestionParams,
  type SuggestionRequestOptions
} from "@decodelabs/underlay/client/suggestions";

async function getLevelsForPathway(
  pathwayId: string,
  options?: SuggestionRequestOptions
): Promise<Level[]> {
  const path = appendSuggestionParams(
    `/v1/admin/pathways/${pathwayId}/levels`,
    options
  );
  // Result: "/v1/admin/pathways/abc/levels?suggestions=true&recentHints=id1,id2,id3"

  const response = await fetch(path);
  return response.json();
}
```

### SuggestionRequestOptions Type

```typescript
interface SuggestionRequestOptions {
  /** Request server-curated suggestions instead of full list */
  suggestions?: boolean;
  /** Hint IDs (recently selected) to prioritize in results */
  recentHints?: string[];
}
```

### Alternative: buildSuggestionParams

For more control, use `buildSuggestionParams` to get URLSearchParams:

```typescript
import { buildSuggestionParams } from "@decodelabs/underlay/client/suggestions";

const params = buildSuggestionParams({
  suggestions: true,
  recentHints: ["id1", "id2"]
});

// Merge with other params
params.set("filter[status]", "active");

const url = `/api/items?${params.toString()}`;
```

### Parsing Hints (Server-Side TypeScript)

If you need to parse hints on a TypeScript server:

```typescript
import { parseHintsParam } from "@decodelabs/underlay/client/suggestions";

// In a SvelteKit server load function
export async function load({ url }) {
  const hintsParam = url.searchParams.get("recentHints");
  const hintIds = parseHintsParam(hintsParam);
  // ["id1", "id2", "id3"]
}
```

## Server-Side: Rust Implementation

### The underlay-suggestions Crate

Add to your Cargo.toml:

```toml
[dependencies]
underlay-suggestions = { path = "../underlay/rust/crates/underlay-suggestions" }
```

### Parsing Query Parameters

```rust
use underlay_suggestions::SuggestionParams;

// In an Axum handler
pub async fn list_levels(
    Query(query): Query<LevelListQuery>,
) -> Result<Json<ListResponse<Level>>, ApiError> {
    // Parse suggestion params from the query
    let suggestion_params = SuggestionParams::from_query(&query);

    if suggestion_params.wants_suggestions() {
        let hint_ids = suggestion_params.recent_hints();
        // hint_ids: Vec<&str> = ["id1", "id2", "id3"]
    }
}

#[derive(Debug, Deserialize)]
pub struct LevelListQuery {
    #[serde(default)]
    pub suggestions: bool,
    #[serde(default, rename = "recentHints")]
    pub recent_hints: String,
}
```

### Building Prioritized Queries

Use `SuggestionQuery` to build SQL that prioritizes hint IDs:

```rust
use underlay_suggestions::{SuggestionQuery, SuggestionOrder};

pub async fn list_levels_with_suggestions(
    pool: &DbPool,
    pathway_id: Uuid,
    hint_ids: &[Uuid],
) -> Result<Vec<Level>, sqlx::Error> {
    let suggestion_query = SuggestionQuery::new(hint_ids, 15)
        .with_order(SuggestionOrder::HintsFirst);

    // Get SQL fragments
    let order_sql = suggestion_query.hint_order_sql("l.id");
    // "CASE WHEN l.id = ANY($N) THEN 0 ELSE 1 END, array_position($N, l.id) NULLS LAST"

    sqlx::query_as::<_, LevelRow>(&format!(r#"
        SELECT l.id, l.title, l.description, l.weight
        FROM learning.level l
        WHERE l.pathway_id = $1 AND l.deleted_at IS NULL
        ORDER BY {}, l.weight, l.title
        LIMIT $2
    "#, order_sql))
    .bind(pathway_id)
    .bind(hint_ids)
    .bind(suggestion_query.limit() as i64)
    .fetch_all(pool)
    .await
}
```

### SuggestionQuery API

```rust
impl SuggestionQuery {
    /// Create a new query with hint IDs and result limit
    pub fn new<S: AsRef<str>>(hints: &[S], limit: usize) -> Self;

    /// Set the ordering strategy
    pub fn with_order(self, order: SuggestionOrder) -> Self;

    /// Get SQL for ORDER BY clause that prioritizes hints
    /// Returns: "CASE WHEN {id_column} = ANY($N) THEN 0 ELSE 1 END, array_position($N, {id_column}) NULLS LAST"
    pub fn hint_order_sql(&self, id_column: &str) -> String;

    /// Get the configured limit
    pub fn limit(&self) -> usize;

    /// Calculate fill limit (how many non-hint items to fetch)
    pub fn fill_limit(&self, valid_hint_count: usize) -> usize;
}

pub enum SuggestionOrder {
    /// Hints first, then by default order
    HintsFirst,
    /// Hints first, preserving hint order
    HintsFirstPreserveOrder,
}
```

### PostgreSQL Query Pattern

The generated SQL prioritizes hint IDs while falling back to normal ordering:

```sql
SELECT l.id, l.title, l.description
FROM learning.level l
WHERE l.pathway_id = $1 AND l.deleted_at IS NULL
ORDER BY
    -- Hint items come first (0), others come second (1)
    CASE WHEN l.id = ANY($2) THEN 0 ELSE 1 END,
    -- Within hints, preserve the order they were provided
    array_position($2, l.id) NULLS LAST,
    -- Fallback ordering for non-hints
    l.weight,
    l.title
LIMIT 15
```

## Complete Example

### 1. Create Selection History Store

```typescript
// dairy/src/lib/stores/selection-history.ts
import { createSelectionHistory } from "@decodelabs/underlay/runtime/data";

export const levelSelectionHistory = createSelectionHistory("learning.levels", {
  maxItems: 20,
  storageType: "local"
});
```

### 2. API Client Function

```typescript
// cattle-grid/src/commands/learning-commands.ts
import {
  appendSuggestionParams,
  type SuggestionRequestOptions
} from "@decodelabs/underlay/runtime/relations";

export async function getLevelsForPathway(
  pathwayId: string,
  fetchFn: typeof fetch,
  accessToken: string,
  options?: SuggestionRequestOptions
): Promise<LearningLevel[]> {
  const http = getHttpClient({ fetchFn, accessToken });
  const path = appendSuggestionParams(
    `/v1/admin/learning/pathways/${encodeURIComponent(pathwayId)}/levels`,
    options
  );
  const response = await http.get<ListResponse<LearningLevel>>(path);
  return response.data;
}
```

### 3. Form Component

```svelte
<!-- dairy/src/lib/forms/learning/ModuleForm.svelte -->
<script lang="ts">
  import LevelSelector from "$lib/components/LevelSelector.svelte";
  import type {
    SelectableRelation,
    SuggestionOptions
  } from "@decodelabs/underlay/runtime/relations";
  import { levelSelectionHistory } from "$lib/stores/selection-history";
  import { learningCommands } from "@cattle-grid";

  interface Props {
    pathwayId: string;
    fetchLevelsForPathway: typeof learningCommands.getLevelsForPathway;
  }

  let { pathwayId, fetchLevelsForPathway }: Props = $props();
  let levelId = $state<string | null>(null);

  function levelToSelectable(level: LearningLevel): SelectableRelation {
    return {
      id: level.levelId,
      label: level.title,
      description: level.description
    };
  }

  async function searchLevels(query: string) {
    const levels = await fetchLevelsForPathway(pathwayId, { /* search params */ });
    const filtered = levels.filter(l =>
      l.title.toLowerCase().includes(query.toLowerCase())
    );
    // This is selector search output, not an API wire envelope.
    return { items: filtered.map(levelToSelectable), total: filtered.length };
  }

  async function suggestLevels(options?: SuggestionOptions) {
    const levels = await fetchLevelsForPathway(pathwayId, {
      suggestions: true,
      recentHints: options?.recentHints
    });
    return levels.map(levelToSelectable);
  }
</script>

<LevelSelector
  label="Select Level"
  bind:value={levelId}
  search={searchLevels}
  suggestions={suggestLevels}
  selectionHistory={levelSelectionHistory}
  placeholder="Select a level..."
/>
```

### 4. Server Endpoint (Rust)

```rust
// farmyard/crates/api/src/routes/admin/learning.rs
use axum::{extract::{Path, Query}, Json};
use underlay_suggestions::SuggestionParams;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct LevelListQuery {
    #[serde(default)]
    pub suggestions: bool,
    #[serde(default, rename = "recentHints")]
    pub recent_hints: String,
}

pub async fn list_levels_for_pathway(
    State(state): State<AppState>,
    Path(pathway_id): Path<Uuid>,
    Query(query): Query<LevelListQuery>,
    claims: Claims,
) -> Result<Json<ListResponse<LevelDto>>, ApiError> {
    let repo = state.learning_repo();

    let levels = if query.suggestions {
        // Parse hint IDs
        let hint_ids: Vec<Uuid> = query.recent_hints
            .split(',')
            .filter(|s| !s.is_empty())
            .filter_map(|s| Uuid::parse_str(s).ok())
            .collect();

        repo.list_levels_for_pathway_with_suggestions(pathway_id, &hint_ids).await?
    } else {
        repo.list_levels_for_pathway(pathway_id).await?
    };

    Ok(Json(ListResponse::new(levels.into_iter().map(Into::into).collect())))
}
```

### 5. Repository Implementation (Rust)

```rust
// farmyard/crates/db/src/learning.rs
pub async fn list_levels_for_pathway_with_suggestions(
    pool: &DbPool,
    pathway_id: Uuid,
    hint_ids: &[Uuid],
) -> Result<Vec<LevelRow>, sqlx::Error> {
    sqlx::query_as::<_, LevelRow>(r#"
        SELECT
            l.id,
            l.pathway_id,
            l.title,
            l.description,
            l.weight,
            l.created_at,
            l.updated_at
        FROM learning.level l
        WHERE l.pathway_id = $1 AND l.deleted_at IS NULL
        ORDER BY
            CASE WHEN l.id = ANY($2) THEN 0 ELSE 1 END,
            array_position($2, l.id) NULLS LAST,
            l.weight,
            l.title
        LIMIT 15
    "#)
    .bind(pathway_id)
    .bind(hint_ids)
    .fetch_all(pool)
    .await
}
```

## Best Practices

### Storage Keys

Use namespaced, descriptive keys:
- `learning.levels` - Learning domain levels
- `content.topics` - Content domain topics
- `users.assignees` - User selection for assignments

### History Size

- **20 items** (default): Good for frequently used selectors
- **10 items**: For less frequently used relations
- **5 items**: For very specific contexts

### When to Use Session vs Local Storage

| Use Case | Storage Type |
|----------|--------------|
| General entity selection | `local` |
| Workflow-specific context | `session` |
| Sensitive data | `session` |
| Cross-tab consistency needed | `local` |

### Cleaning Up Deleted Items

When an item is deleted, remove it from relevant histories:

```typescript
async function deleteLevel(levelId: string) {
  await api.deleteLevel(levelId);
  levelSelectionHistory.remove(levelId);
}
```

### Server-Side Limits

Always limit suggestion queries to prevent large result sets:
- **15 items**: Good default for dropdown suggestions
- **25 items**: For modal views with more space
- Never return unlimited results in suggestion mode

## Troubleshooting

### Hints Not Appearing in Server Logs

1. **Check client-side**: Add console.log in your suggestions function
   ```typescript
   async function suggestLevels(options?: SuggestionOptions) {
     console.log("Suggestion options:", options);
     // ...
   }
   ```

2. **Check history tracking**: Verify selections are being tracked
   ```typescript
   console.log("History:", levelSelectionHistory.getRecentIds());
   ```

3. **Check URL building**: Log the final URL
   ```typescript
   const path = appendSuggestionParams("/api/levels", options);
   console.log("Request path:", path);
   ```

### Suggestions Not Updating After Selection

Selector shells built over the retained helper layer usually only load
suggestions once per mount. To see updated hints:
- Remount the component (e.g., navigate away and back)
- Use `{#key}` blocks for dependent fields that should remount

```svelte
{#key pathwayId}
  <LevelSelector
    selectionHistory={levelSelectionHistory}
    suggestions={suggestLevels}
    ...
  />
{/key}
```

### TypeScript Type Errors

Ensure you're importing from the correct packages:

```typescript
// For component integration
import { type SuggestionOptions } from "@decodelabs/underlay/runtime/relations";

// For API request building
import { type SuggestionRequestOptions } from "@decodelabs/underlay/client/suggestions";
```

## Related Documentation

- [098 - Shared Admin Patterns](./098-shared-admin-patterns.md) - Current retained relation-selector helper-layer boundary
- [TypeScript Client](./080-typescript-client.md) - HTTP client utilities
- [Rust Backend](./040-rust-backend.md) - Axum handler patterns
- [Database](./050-database.md) - PostgreSQL query patterns
