# Pagination

This guide covers Underlay's lower-level cursor pagination infrastructure and
the shared controller layer that sits on top of it.

This is the lower-level pagination/runtime guide.

It is not the same thing as the higher-level admin page-shape contract in:

- [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md)
- [../contracts/115-admin-resource-api-shapes.md](../contracts/115-admin-resource-api-shapes.md)

Naming split:

- `PaginatedResponse<T>`:
  runtime/data pagination response with cursors
- `PagedListResponse<T>`:
  client-facing page-shaped admin list response
- `PagedListResult<T>`:
  template loader result for `EntityListPage` / `EntityList`

Hard rule:

- `EntityListPage` and real page-shaped child collections should use the
  page-shaped contract from `115`/`116`
- this guide is for lower-level cursor-runtime flows and explicit compatibility
  lanes, not for ordinary admin browse pages

## Overview

Underlay provides a complete pagination solution that scales from small client-side lists to server-side cursor-based pagination for millions of rows. Both modes expose the same `PaginationController` interface, allowing UI components to work seamlessly with either approach.

### Goals

- Scale to millions of rows with consistent performance
- Unified UI components that work with both pagination modes
- Simple REST-style API (not Relay/GraphQL complexity)
- Total counts available with opt-out for performance
- Seamless integration with existing patterns (`useAuthenticatedData`, `FilterToolbar`, etc.)

### Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                           SERVER SIDE                                   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ PaginationQuery │───▶│ PaginationParams │───▶│ Keyset SQL       │   │
│  │ (Axum extractor)│    │ (core types)     │    │ WHERE (w,id) > ? │   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ PaginatedResp   │◀───│ PaginationBuilder│◀───│ query_limit()+1  │   │
│  │ with cursors    │    │ (helper)         │    │ for has_more     │   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
                                    │
                                    ▼
┌─────────────────────────────────────────────────────────────────────────┐
│                           CLIENT SIDE                                   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐    ┌──────────────────┐   │
│  │ createPagination│───▶│ PaginationController│──▶│ <Pagination     │   │
│  │ Controller      │    │ (unified interface)│    │  controller={} />│   │
│  └─────────────────┘    └──────────────────┘    └──────────────────┘   │
│                                                                         │
│  ┌─────────────────┐    ┌──────────────────┐                           │
│  │ createClient    │───▶│ Same interface   │                           │
│  │ Pagination      │    │ (client-side)    │                           │
│  └─────────────────┘    └──────────────────┘                           │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
```

### Components

| Layer | Component | Package | Purpose |
|-------|-----------|---------|---------|
| Rust Core | `PaginationParams` | `underlay-db` | Core pagination parameters |
| Rust Core | `PaginatedResponse<T>` | `underlay-db` | Generic response wrapper |
| Rust Core | `Cursor` | `underlay-db` | Encode/decode cursor values |
| Rust Core | `PaginationBuilder` | `underlay-db` | Helper for building responses |
| Rust API | `PaginationQuery` | App-specific | Axum query extractor |
| Rust API | `PaginatedResponseDto<T>` | App-specific | DTO with utoipa support |
| TypeScript | `CursorPaginationParams` | `@decodelabs/underlay/runtime/data` | Cursor query parameter interface |
| TypeScript | `CursorPaginatedResponse<T>` | `@decodelabs/underlay/runtime/data` | Cursor response interface |
| TypeScript | `PaginationController<T>` | `@decodelabs/underlay/runtime/data` | Unified controller interface |
| TypeScript | `createPaginationController` | `@decodelabs/underlay/runtime/data` | Server-side controller |
| TypeScript | `createClientPagination` | `@decodelabs/underlay/runtime/data` | Client-side controller |
| TypeScript | `PageListParams` | `@decodelabs/underlay/client/page-lists` | Page-shaped `page + limit` helper |
| Svelte | `<Pagination>` | `@inflatable-cookie/poodle-svelte` | UI component |

## API Design

Pagination is a query-level contract on canonical resource list routes.

- Use `GET /v1/{scope}/{domain}/{resource}` for list endpoints.
- Use `profile` query params to select list projection (`list` vs `filter`) where relevant.
- Never use path suffixes (for example `/paginated`) to indicate pagination behavior.

### Response Format

For the runtime/data pagination layer, paginated endpoints return this
structure:

```typescript
interface PaginatedResponse<T> {
  data: T[];                    // Items for the current page
  nextCursor: string | null;    // Cursor for next page (null if no more)
  prevCursor: string | null;    // Cursor for previous page (null if at start)
  hasMore: boolean;             // Whether more items exist after this page
  total: number | null;         // Total count (null if opted out or unavailable)
}
```

Do not confuse this with the admin page-shaped list contract from `115`, which
uses:

```ts
interface PagedListResponse<T> {
  data: T[];
  total: number;
  hasMore: boolean;
}
```

Rule of thumb:

- use `PaginatedResponse<T>` for cursor-driven runtime pagination flows
- use `PagedListResponse<T>` for admin resource pages and detail-tab child lists
- use `PagedListResult<T>` at the template data-loader seam

### Query Parameters

| Param | Type | Default | Description |
|-------|------|---------|-------------|
| `limit` | number | 50 | Items per page (max 100) |
| `cursor` | string | null | Opaque cursor for position |
| `direction` | string | "forward" | "forward" or "backward" |
| `includeTotal` | boolean | true | Whether to compute total count |

Resource filters and `profile` are additional endpoint-specific query params layered on this shared pagination set.

### Cursor Encoding

Cursors are base64-encoded JSON containing sort column values and a tiebreaker ID:

```json
// For weight-sorted list:
{"w": 5, "id": "0192abc..."}

// For timestamp-sorted list:
{"t": "2024-01-15T10:30:00Z", "id": "0192abc..."}
```

## Server-Side: Rust Implementation

### Core Types (underlay-db)

Add to your `Cargo.toml`:

```toml
[dependencies]
underlay-db = { path = "../underlay/rust/crates/underlay-db" }
```

The core types are in `underlay_db::pagination`:

```rust
use underlay_db::pagination::{
    PaginationParams,
    PaginatedResponse,
    PaginationBuilder,
    PaginationDirection,
    Cursor,
    WeightCursor,
    TimestampCursor,
    DEFAULT_PAGE_SIZE,
    MAX_PAGE_SIZE,
};
```

### PaginationParams

```rust
pub struct PaginationParams {
    pub limit: i64,
    pub cursor: Option<String>,
    pub direction: PaginationDirection,
    pub include_total: bool,
}

impl PaginationParams {
    /// Get the effective limit (clamped to MAX_PAGE_SIZE)
    pub fn effective_limit(&self) -> i64;
}

pub enum PaginationDirection {
    Forward,
    Backward,
}
```

### Cursor Types

```rust
/// Generic cursor with arbitrary key-value pairs
pub struct Cursor {
    values: HashMap<String, serde_json::Value>,
}

impl Cursor {
    pub fn new() -> Self;
    pub fn with_weight(self, weight: i32) -> Self;
    pub fn with_timestamp(self, timestamp: DateTime<Utc>) -> Self;
    pub fn with_id(self, id: Uuid) -> Self;
    pub fn encode(&self) -> String;  // Base64 encode
    pub fn decode(encoded: &str) -> Result<Self, CursorError>;
    pub fn get_weight(&self) -> Option<i32>;
    pub fn get_timestamp(&self) -> Option<DateTime<Utc>>;
    pub fn get_id(&self) -> Option<Uuid>;
}

/// Convenience type for weight-based cursors
pub struct WeightCursor {
    pub weight: i32,
    pub id: Uuid,
}

/// Convenience type for timestamp-based cursors
pub struct TimestampCursor {
    pub timestamp: DateTime<Utc>,
    pub id: Uuid,
}
```

### PaginationBuilder

The builder helps construct paginated responses:

```rust
pub struct PaginationBuilder {
    params: PaginationParams,
}

impl PaginationBuilder {
    pub fn new(params: PaginationParams) -> Self;

    /// Returns limit + 1 to detect if there are more items
    pub fn query_limit(&self) -> i64;

    /// Decode cursor as WeightCursor
    pub fn decode_weight_cursor(&self) -> Result<Option<WeightCursor>, CursorError>;

    /// Decode cursor as TimestampCursor
    pub fn decode_timestamp_cursor(&self) -> Result<Option<TimestampCursor>, CursorError>;

    /// Build response from fetched items
    /// Items should have query_limit() count (one extra for has_more detection)
    pub fn build_response<T, F>(
        &self,
        items: Vec<T>,
        total: Option<i64>,
        cursor_fn: F,
    ) -> PaginatedResponse<T>
    where
        F: Fn(&T) -> Cursor;
}
```

### Example: Paginated Query

```rust
use underlay_db::pagination::{
    PaginationParams, PaginatedResponse, PaginationBuilder, Cursor,
};

pub async fn list_activities_paginated(
    pool: &DbPool,
    bundle_id: Uuid,
    params: PaginationParams,
) -> Result<PaginatedResponse<ActivityRow>, sqlx::Error> {
    let builder = PaginationBuilder::new(params.clone());
    let cursor = builder.decode_weight_cursor().ok().flatten();

    // Build keyset pagination WHERE clause
    let (where_clause, cursor_weight, cursor_id) = match &cursor {
        Some(c) if params.direction == PaginationDirection::Forward => {
            ("AND (ba.weight, ba.activity_id) > ($3, $4)", Some(c.weight), Some(c.id))
        }
        Some(c) => {
            ("AND (ba.weight, ba.activity_id) < ($3, $4)", Some(c.weight), Some(c.id))
        }
        None => ("", None, None),
    };

    // Query with limit + 1 for has_more detection
    let items = sqlx::query_as::<_, ActivityRow>(&format!(r#"
        SELECT a.id, a.name, a.slug, ba.weight
        FROM learning.activity a
        JOIN learning.bundle_activity ba ON ba.activity_id = a.id
        WHERE ba.bundle_id = $1 AND a.deleted_at IS NULL
        {}
        ORDER BY ba.weight {}, ba.activity_id {}
        LIMIT $2
    "#,
        where_clause,
        if params.direction == PaginationDirection::Forward { "ASC" } else { "DESC" },
        if params.direction == PaginationDirection::Forward { "ASC" } else { "DESC" },
    ))
    .bind(bundle_id)
    .bind(builder.query_limit())
    .bind(cursor_weight)
    .bind(cursor_id)
    .fetch_all(pool)
    .await?;

    // Optional: Get total count
    let total = if params.include_total {
        let count: (i64,) = sqlx::query_as(r#"
            SELECT COUNT(*)
            FROM learning.bundle_activity ba
            JOIN learning.activity a ON a.id = ba.activity_id
            WHERE ba.bundle_id = $1 AND a.deleted_at IS NULL
        "#)
        .bind(bundle_id)
        .fetch_one(pool)
        .await?;
        Some(count.0)
    } else {
        None
    };

    // Build response with cursors
    Ok(builder.build_response(items, total, |item| {
        Cursor::new()
            .with_weight(item.weight)
            .with_id(item.id)
    }))
}
```

### Axum Integration

Create an extractor in your API crate:

```rust
// api/src/pagination.rs
use serde::Deserialize;
use utoipa::IntoParams;
use underlay_db::pagination::{PaginationParams, PaginationDirection, DEFAULT_PAGE_SIZE};

/// Query parameters for paginated endpoints
#[derive(Debug, Clone, Default, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
pub struct PaginationQuery {
    #[param(minimum = 1, maximum = 100, default = 50)]
    pub limit: Option<i64>,
    pub cursor: Option<String>,
    pub direction: Option<String>,
    #[serde(default)]
    pub include_total: Option<bool>,
}

impl PaginationQuery {
    pub fn into_params(self) -> PaginationParams {
        let direction = match self.direction.as_deref() {
            Some("backward") => PaginationDirection::Backward,
            _ => PaginationDirection::Forward,
        };

        PaginationParams {
            limit: self.limit.unwrap_or(DEFAULT_PAGE_SIZE),
            cursor: self.cursor,
            direction,
            include_total: self.include_total.unwrap_or(true),
        }
    }
}
```

### DTO with utoipa

```rust
use serde::Serialize;
use utoipa::ToSchema;
use underlay_db::pagination::PaginatedResponse;

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponseDto<T: ToSchema> {
    pub data: Vec<T>,
    pub next_cursor: Option<String>,
    pub prev_cursor: Option<String>,
    pub has_more: bool,
    pub total: Option<i64>,
}

impl<T: ToSchema> From<PaginatedResponse<T>> for PaginatedResponseDto<T> {
    fn from(response: PaginatedResponse<T>) -> Self {
        Self {
            data: response.data,
            next_cursor: response.next_cursor,
            prev_cursor: response.prev_cursor,
            has_more: response.has_more,
            total: response.total,
        }
    }
}

impl<T: ToSchema> PaginatedResponseDto<T> {
    /// Create from PaginatedResponse, mapping data items
    pub fn from_mapped<U, F>(response: PaginatedResponse<U>, f: F) -> Self
    where
        F: FnMut(U) -> T,
    {
        let mapped = response.map(f);
        Self {
            data: mapped.data,
            next_cursor: mapped.next_cursor,
            prev_cursor: mapped.prev_cursor,
            has_more: mapped.has_more,
            total: mapped.total,
        }
    }
}
```

### Route Handler

```rust
use axum::{extract::{Path, Query, State}, response::IntoResponse, Json};

pub async fn get_bundle_activities(
    State(state): State<AppState>,
    AdminUser(_user): AdminUser,
    Path(bundle_id): Path<String>,
    Query(pagination): Query<PaginationQuery>,
) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&bundle_id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid bundle ID").into_response(),
    };

    let params = pagination.into_params();

    match state.repo.list_activities_paginated(uuid, params).await {
        Ok(response) => {
            let body: PaginatedResponseDto<ActivityDto> =
                PaginatedResponseDto::from_mapped(response, ActivityDto::from);
            (StatusCode::OK, Json(body)).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to list activities: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to list activities").into_response()
        }
    }
}
```

## TypeScript Types

### Imports

```typescript
import {
  // Types
  type PaginatedResponse,
  type PaginationParams,
  type PaginationController,

  // Constants
  DEFAULT_PAGE_SIZE,
  MAX_PAGE_SIZE,

  // Helpers
  buildPaginationQuery,
  appendPaginationParams,
} from "@decodelabs/underlay/runtime/data";
```

For the higher-level admin page-list surface, import the public client type
from `@decodelabs/underlay/client/types` instead:

```ts
import type { PagedListResponse } from "@decodelabs/underlay/client/types";
```

### PaginationParams

```typescript
interface PaginationParams {
  limit?: number;                         // Items per page (default: 50, max: 100)
  cursor?: string | null;                 // Opaque cursor for position
  direction?: "forward" | "backward";     // Navigation direction
  includeTotal?: boolean;                 // Whether to include total count
}
```

### PaginatedResponse

```typescript
interface PaginatedResponse<T> {
  data: T[];                   // Items for the current page
  nextCursor: string | null;   // Cursor for next page
  prevCursor: string | null;   // Cursor for previous page
  hasMore: boolean;            // Whether more items exist
  total: number | null;        // Total count (null if unavailable)
}
```

### PaginationController

The unified interface for both server-side and client-side pagination:

```typescript
interface PaginationController<T> {
  // Current state (reactive)
  readonly items: T[];              // Items for current page
  readonly currentPage: number;     // 1-indexed page number
  readonly pageSize: number;        // Items per page
  readonly hasNextPage: boolean;    // Can navigate forward
  readonly hasPrevPage: boolean;    // Can navigate backward
  readonly total: number | null;    // Total count if available
  readonly loading: boolean;        // Whether fetching data
  readonly error: string | null;    // Error message if failed

  // Computed values (reactive)
  readonly showingFrom: number;     // First item index (1-indexed)
  readonly showingTo: number;       // Last item index (1-indexed)
  readonly totalPages: number | null;

  // Actions
  nextPage(): void | Promise<void>;
  prevPage(): void | Promise<void>;
  goToPage?(page: number): void;    // Only for client-side
  setPageSize(size: number): void;
  refresh(): Promise<void>;
}
```

### Helper Functions

```typescript
// Build query string parameters
function buildPaginationQuery(params: PaginationParams): Record<string, string>;

// Append pagination params to a URL path
function appendPaginationParams(path: string, params: PaginationParams): string;

// Example:
const url = appendPaginationParams("/api/items", { limit: 25, cursor: "abc123" });
// "/api/items?limit=25&cursor=abc123"
```

## Frontend Patterns

### Server-Side Pagination: createPaginationController

Use this for large datasets where you want cursor-based navigation:

```typescript
import {
  createPaginationController,
  type ServerPaginationOptions
} from "@decodelabs/underlay/runtime/data";
```

#### Options

```typescript
interface ServerPaginationOptions<T> {
  getToken?: () => string | null;   // Auth token getter (uses configureAuth if not provided)
  onRefresh?: (fetchFn: typeof fetch) => Promise<string | null>;  // Token refresh handler
  pageSize?: number;                // Initial page size (default: 50)
  includeTotal?: boolean;           // Request total counts (default: true)
  onSuccess?: (response: PaginatedResponse<T>) => void;
  onError?: (error: Error) => void;
}
```

#### Usage

```svelte
<script lang="ts">
  import {
    createPaginationController,
    type PaginationParams
  } from "@decodelabs/underlay/runtime/data";
  import { Pagination } from "@inflatable-cookie/poodle-svelte";
  import { getBundleActivitiesPaginated } from "@cattle-grid";
  import { authLoading, currentUser } from "$lib/stores/auth";

  interface Props {
    bundleId: string;
  }

  let { bundleId }: Props = $props();

  // Create pagination controller
  const pagination = createPaginationController(
    async (fetch, token, params: PaginationParams) => {
      return await getBundleActivitiesPaginated(bundleId, fetch, token, params);
    },
    { pageSize: 30 }
  );

  // Trigger initial fetch when auth is ready
  $effect(() => {
    pagination.tryFetch($authLoading, $currentUser);
  });
</script>

{#if pagination.loading && pagination.items.length === 0}
  <p>Loading...</p>
{:else if pagination.error}
  <p class="error">{pagination.error}</p>
{:else}
  <ul>
    {#each pagination.items as item}
      <li>{item.name}</li>
    {/each}
  </ul>

  <Pagination controller={pagination} variant="simple" />
{/if}
```

### Client-Side Pagination: createClientPagination

Use this for pre-loaded data where you want instant page switching:

```typescript
import { createClientPagination } from "@decodelabs/underlay/runtime/data";
```

#### Options

```typescript
interface ClientPaginationOptions {
  pageSize?: number;      // Initial page size (default: 50)
  initialPage?: number;   // Starting page (default: 1)
}
```

#### Usage

```svelte
<script lang="ts">
  import { createClientPagination } from "@decodelabs/underlay/runtime/data";
  import { Pagination } from "@inflatable-cookie/poodle-svelte";

  interface Props {
    activities: Activity[];
  }

  let { activities }: Props = $props();

  // Filter state
  let searchFilter = $state("");

  // Filtered list (reactive)
  const filteredActivities = $derived(() => {
    if (!searchFilter) return activities;
    return activities.filter(a =>
      a.name.toLowerCase().includes(searchFilter.toLowerCase())
    );
  });

  // Paginate the filtered list
  const pagination = createClientPagination(
    () => filteredActivities(),
    { pageSize: 25 }
  );

  // Client-side supports random access
  function jumpToPage(page: number) {
    pagination.goToPage?.(page);
  }
</script>

<input bind:value={searchFilter} placeholder="Search..." />

<ul>
  {#each pagination.items as item}
    <li>{item.name}</li>
  {/each}
</ul>

{#if (pagination.totalPages ?? 1) > 1}
  <Pagination
    controller={pagination}
    showLimitSelector
    limitOptions={[10, 25, 50, 100]}
  />
{/if}
```

#### Key Differences from Server-Side

| Feature | Server-Side | Client-Side |
|---------|-------------|-------------|
| `goToPage()` | Not available | Available |
| `loading` | Can be true | Always false |
| `error` | Can have errors | Always null |
| Data source | Fetched per page | Pre-loaded array |
| Page switching | Network request | Instant |

## UI Component: Pagination

### Import

```svelte
<script>
  import { Pagination } from "@inflatable-cookie/poodle-svelte";
</script>
```

### Props

```typescript
interface Props {
  // Controller mode (recommended)
  controller?: PaginationController<unknown>;

  // Props mode
  page?: number;
  limit?: number;
  total?: number;
  onPageChange?: (page: number) => void;
  onLimitChange?: (limit: number) => void;

  // Appearance
  variant?: "full" | "simple";     // "full" shows page numbers, "simple" shows prev/next
  compact?: boolean;               // Smaller padding
  showLimitSelector?: boolean;     // Show page size dropdown
  limitOptions?: number[];         // Options for dropdown (default: [10, 20, 50, 100])
  showInfo?: boolean;              // Show "Showing X to Y of Z" (default: true)
  className?: string;
}
```

### Variants

**Full variant** - shows first/prev, page summary, and next/last buttons:
```
[««] [«]  Page 2 of 10  [»] [»»]   |  Show: [50 ▼]
```

**Simple variant** - shows prev/next with item range:
```
[« Prev]  51–100 of 1,234  [Next »]   |  Show: [50 ▼]
```

### Examples

#### With Controller (Recommended)

```svelte
<Pagination controller={pagination} />
```

#### With Options

```svelte
<Pagination
  controller={pagination}
  variant="simple"
  showLimitSelector
  limitOptions={[25, 50, 100]}
/>
```

#### Props Mode

```svelte
<Pagination
  page={currentPage}
  limit={itemsPerPage}
  total={totalItems}
  onPageChange={(page) => currentPage = page}
  onLimitChange={(limit) => itemsPerPage = limit}
  showLimitSelector
/>
```

### States

- **Loading**: Reduced opacity, buttons disabled
- **First page**: Prev button disabled
- **Last page**: Next button disabled (or hidden if `hasMore=false`)
- **Empty**: Component not rendered
- **Single page**: Component not rendered (unless `showAlways` prop)

### Design Tokens

The component uses these CSS custom properties:

| Token | Purpose | Default |
|-------|---------|---------|
| `--underlay-color-surface-muted` | Container background | transparent |
| `--underlay-color-border-subtle` | Borders | rgba(148, 163, 184, 0.2) |
| `--underlay-color-button-neutral-bg` | Button background | rgba(255, 255, 255, 0.05) |
| `--underlay-color-button-neutral-hover` | Button hover | rgba(255, 255, 255, 0.1) |
| `--underlay-color-text` | Text color | inherit |
| `--underlay-color-text-muted` | Info text | #9ca3af |
| `--underlay-color-primary` | Focus rings | #14b8a6 |

## Complete Example

### 1. API Client (cattle-grid)

```typescript
// cattle-grid/src/commands/my-commands.ts
import {
  appendPaginationParams,
  type PaginatedResponse,
  type PaginationParams
} from "@decodelabs/underlay/runtime/data";

export async function getItemsPaginated(
  fetchFn: typeof fetch,
  accessToken: string,
  pagination?: PaginationParams
): Promise<PaginatedResponse<Item>> {
  const http = getHttpClient({ fetchFn, accessToken });
  const path = appendPaginationParams("/v1/admin/items", pagination ?? {});
  return await http.get<PaginatedResponse<Item>>(path);
}
```

This example is for a lower-level cursor-aware command surface. If the command
is feeding `EntityListPage` or another page-shaped admin list, prefer the
`PagedListResponse<T>` pattern from:

- [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md)
- [code/073-api-profiles-and-query-contract/entity-list-page-paged-loader.ts](./code/073-api-profiles-and-query-contract/entity-list-page-paged-loader.ts)

### 2. Page Component

```svelte
<!-- +page.svelte -->
<script lang="ts">
  import {
    createPaginationController,
    createClientPagination,
    PageHeader
  } from "@decodelabs/underlay/runtime/data";
  import { FilterToolbar } from "@inflatable-cookie/poodle-svelte";
  import {
    Field,
    Pagination,
    Select,
    TextInput
  } from "@inflatable-cookie/poodle-svelte";
  import { page } from "$app/stores";
  import { authLoading, currentUser } from "$lib/stores/auth";
  import { getItemsPaginated } from "@cattle-grid";

  // Support ?limit=N for testing
  const initialPageSize = $derived(() => {
    const limitParam = $page.url.searchParams.get("limit");
    if (limitParam) {
      const parsed = parseInt(limitParam, 10);
      if (!isNaN(parsed) && parsed > 0) return parsed;
    }
    return 30;
  });

  // Create server-side pagination controller
  const pagination = createPaginationController(
    async (fetch, token, params) => {
      return await getItemsPaginated(fetch, token, params);
    },
    { pageSize: initialPageSize() }
  );

  // Filter state
  let searchFilter = $state("");
  let categoryFilter = $state("all");

  // Trigger fetch on auth ready
  $effect(() => {
    pagination.tryFetch($authLoading, $currentUser);
  });

  // Reset pagination when filters change
  $effect(() => {
    // Access filter values to create dependency
    const _ = [searchFilter, categoryFilter];
    pagination.refresh();
  });
</script>

<PageHeader title="Items" />

<FilterToolbar ariaLabel="Item filters" summaryText="Filters">
  <Field label="Category">
    <Select bind:value={categoryFilter} items={categoryOptions} />
  </Field>
  <Field label="Search">
    <TextInput bind:value={searchFilter} placeholder="Search..." debounce={300} />
  </Field>
</FilterToolbar>

{#if pagination.loading && pagination.items.length === 0}
  <p>Loading items...</p>
{:else if pagination.error}
  <p class="error">{pagination.error}</p>
{:else if pagination.items.length === 0}
  <p>No items found.</p>
{:else}
  <Grid columns="repeat(auto-fit, minmax(min(22.5rem, 100%), 1fr))" gap="lg">
    {#each pagination.items as item}
      <ItemCard {item} />
    {/each}
  </Grid>

  <Pagination
    controller={pagination}
    variant="simple"
    showLimitSelector
    limitOptions={[25, 50, 100]}
  />
{/if}
```

## When to Use Each Approach

### Use Server-Side Pagination When:

- Dataset can be large (100+ items possible)
- Items are fetched from an API
- You need consistent performance regardless of total count
- Memory usage is a concern

### Use Client-Side Pagination When:

- Dataset is small (typically < 100 items)
- Items are already loaded (e.g., passed as props)
- You need instant page switching
- You need random page access (`goToPage`)
- List is filtered client-side before pagination

### Don't Paginate When:

- List is always small (< 20 items)
- Items need to be reorderable (requires full list)
- Users need to see all items at once

## Best Practices

### Page Sizes

- **30 items**: Good default for card grids
- **50 items**: Good for compact lists
- **100 items**: Maximum for most use cases

### Total Counts

- Enable for small-medium datasets (< 100k rows)
- Disable for very large datasets where COUNT is expensive
- UI gracefully handles missing totals

### Cursor Stability

- Always include a unique tiebreaker (usually `id`) in cursors
- Use `(weight, id)` or `(created_at, id)` patterns
- Never use offset-based pagination for large datasets

### Filter Integration

Reset pagination when filters change:

```typescript
$effect(() => {
  // Access filter values to track dependencies
  const _ = [searchFilter, categoryFilter];
  pagination.refresh();
});
```

### Testing

Use `?limit=N` query param to test with small page sizes:

```
/items?limit=2        # 2 items per page
/items?limit=1        # 1 item per page
```

## Troubleshooting

### Pagination Not Showing

1. Check that `totalPages > 1` or there are prev/next pages available
2. Ensure the controller is passed correctly: `controller={pagination}`
3. Check that items are being loaded: `console.log(pagination.items)`

### Cursor Decode Errors

1. Ensure cursor encoding/decoding uses the same format on client and server
2. Check that IDs in cursors are valid UUIDs
3. Log the raw cursor value to debug encoding issues

### Total Count is Null

This is expected when:
- `includeTotal: false` was passed
- The server opted out of counting for performance
- The UI handles this gracefully by hiding "of X" text

### Styles Not Matching Theme

Ensure your app defines the required CSS custom properties:
- `--underlay-color-button-neutral-bg`
- `--underlay-color-border-subtle`
- `--underlay-color-text`
- `--underlay-color-text-muted`

See the [Design Tokens](#design-tokens) section for the full list.

## Related Documentation

- [097 - Autonomous List Components](./097-autonomous-list-components.md) - Retained pagination and list-controller runtime guidance
- [Rust Backend](./040-rust-backend.md) - Axum handler patterns
- [TypeScript Client](./080-typescript-client.md) - HTTP client utilities
- [Database](./050-database.md) - PostgreSQL query patterns
- [API Profiles and Unified Query Contract](./073-api-profiles-and-query-contract.md) - Canonical route and profile policy
