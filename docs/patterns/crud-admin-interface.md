# Recipe: CRUD Admin Interface

**Use when**: You need a complete admin interface for a database table.

**Example prompt**: "Build the CRUD interface for the Bundles table"

## Outcome Profile (Quickstart vs Dairy-Scale Admin)

If you stop at the base checklist in this recipe, you get a correct but **minimal CRUD surface**. That is usually enough for a simple table, but it will underdeliver for complex admin areas like Dairy's `learning`, `content`, and `system` flows.

| Area | Base recipe outcome | Dairy-scale outcome (recommended) |
|------|----------------------|-----------------------------------|
| Data loading | Single load call per page | Auth-aware loading with `useAuthenticatedData()` and retry/empty/loading states |
| List UX | Basic table/list and row click | Pagination controller, filters, search, selection mode, batch actions, reorder mode |
| Form UX | Local submit handler | `SpaFormShell`, intent-based submit (`save`, `save-close`, `delete`), field-level error mapping |
| Navigation | Static breadcrumb/back link | Context-aware back links with `consumeNavigationContext()` + `gotoWithContext()` |
| Related entities | Separate screens only | Parent detail tabs plus nested child routes and inline list actions |
| Rich fields | Plain text inputs | `MarkdownEditor`/`NightfireEditor`, content cards, schema-aware save prep |
| Operational polish | Minimal success/failure handling | Toasts, optimistic local updates, structured errors, analytics hooks |

**Rule of thumb**:
- Use this recipe alone for straightforward one-entity admin pages.
- Combine this recipe with the extension checklist below for Dairy-style admin surfaces.

---

## Checklist

### Phase 1: Backend - Database Layer

**File**: `crates/db/src/{domain}.rs`

- [ ] `list_{entities}(pool) -> Vec<Row>` - List all (with soft-delete filter)
- [ ] `get_{entity}_by_id(pool, id) -> Option<Row>` - Single record lookup
- [ ] `create_{entity}(pool, ..fields..) -> Row` - Insert with RETURNING
- [ ] `update_{entity}(pool, id, ..fields..) -> Option<Row>` - Update with RETURNING
- [ ] `soft_delete_{entity}(pool, id)` - Set deleted_at + delete_batch_id
- [ ] Existence checks using `ExistsCheck` for unique fields

**Existence check pattern** ([050-database.md](../guides/050-database.md#existscheck-builder)):

```rust
pub async fn {entity}_slug_exists(
    pool: &DbPool,
    slug: &str,
    exclude_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let mut check = ExistsCheck::new("schema", "table").value("slug", slug);
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

---

### Phase 2: Backend - DTOs

**File**: `crates/api/src/dto/{domain}.rs`

- [ ] `{Entity}Dto` - Response shape (camelCase via serde rename_all)
- [ ] `Create{Entity}Payload` - Create request body with validation
- [ ] `Update{Entity}Payload` - Update request body with validation

**DTO pattern**:

```rust
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BundleDto {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub is_live: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBundlePayload {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
    pub is_live: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBundlePayload {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(length(min = 1, max = 100))]
    pub slug: String,
    pub is_live: bool,
}
```

---

### Phase 3: Backend - Routes

**File**: `crates/api/src/routes/admin/{domain}.rs`

- [ ] `GET /v1/admin/{domain}/{entities}` - List all
- [ ] `GET /v1/admin/{domain}/{entities}/:id` - Get single
- [ ] `POST /v1/admin/{domain}/{entities}` - Create
- [ ] `PUT /v1/admin/{domain}/{entities}/:id` - Update
- [ ] `DELETE /v1/admin/{domain}/{entities}/:id` - Soft delete

**Route setup pattern**:

```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/admin/domain/entities", get(list_entities).post(create_entity))
        .route("/v1/admin/domain/entities/:id", get(get_entity).put(update_entity).delete(delete_entity))
}
```

**Handler patterns** ([070-api-handlers.md](../guides/070-api-handlers.md)):

```rust
// GET single
async fn get_entity(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let id = parse_uuid_path_raw(&id, "id")?;

    let row = db::get_entity_by_id(&state.pool, id).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    let Some(row) = row else {
        return error_response(StatusCode::NOT_FOUND,
            AppError::new("entity.not_found", "Entity not found")).into_response();
    };

    ok(EntityDto::from(row)).into_response()
}

// POST create
async fn create_entity(
    State(state): State<AppState>,
    Json(payload): Json<CreateEntityPayload>,
) -> impl IntoResponse {
    if let Err(e) = payload.validate() {
        let err = validation_to_app_error(&e, "entity.invalid", "Validation failed.");
        return error_response(StatusCode::BAD_REQUEST, err).into_response();
    }

    // Check uniqueness
    if db::entity_slug_exists(&state.pool, &payload.slug, None).await.unwrap_or(false) {
        return error_response(StatusCode::CONFLICT,
            AppError::new("entity.slug_exists", "Slug already exists")).into_response();
    }

    let row = db::create_entity(&state.pool, &payload.name, &payload.slug, payload.is_live).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    created(EntityDto::from(row)).into_response()
}

// PUT update
async fn update_entity(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateEntityPayload>,
) -> impl IntoResponse {
    let id = parse_uuid_path_raw(&id, "id")?;

    if let Err(e) = payload.validate() {
        let err = validation_to_app_error(&e, "entity.invalid", "Validation failed.");
        return error_response(StatusCode::BAD_REQUEST, err).into_response();
    }

    // Check uniqueness excluding current record
    if db::entity_slug_exists(&state.pool, &payload.slug, Some(id)).await.unwrap_or(false) {
        return error_response(StatusCode::CONFLICT,
            AppError::new("entity.slug_exists", "Slug already exists")).into_response();
    }

    let row = db::update_entity(&state.pool, id, &payload.name, &payload.slug, payload.is_live).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    let Some(row) = row else {
        return error_response(StatusCode::NOT_FOUND,
            AppError::new("entity.not_found", "Entity not found")).into_response();
    };

    ok(EntityDto::from(row)).into_response()
}

// DELETE soft delete
async fn delete_entity(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let id = parse_uuid_path_raw(&id, "id")?;

    db::soft_delete_entity(&state.pool, id).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    no_content().into_response()
}
```

---

### Phase 4: Client - Types

**File**: `cattle-grid/src/types/{domain}-types.ts`

- [ ] `{Entity}` interface matching `{Entity}Dto`
- [ ] `Create{Entity}Payload` interface
- [ ] `Update{Entity}Payload` interface

**Type pattern**:

```typescript
export interface Bundle {
  id: string;
  name: string;
  slug: string;
  isLive: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateBundlePayload {
  name: string;
  slug: string;
  isLive: boolean;
}

export interface UpdateBundlePayload {
  name: string;
  slug: string;
  isLive: boolean;
}
```

---

### Phase 5: Client - Commands

**File**: `cattle-grid/src/commands/{domain}-commands.ts`

- [ ] `get{Entities}()` - List all
- [ ] `get{Entity}(id)` - Get single
- [ ] `create{Entity}(payload)` - Create
- [ ] `update{Entity}(id, payload)` - Update
- [ ] `delete{Entity}(id)` - Soft delete

**Command pattern**:

```typescript
import { apiClient, type ApiResponse } from '../client';
import type { Bundle, CreateBundlePayload, UpdateBundlePayload } from '../types';

export function getBundles(): Promise<ApiResponse<Bundle[]>> {
  return apiClient.get('/v1/admin/domain/bundles');
}

export function getBundle(id: string): Promise<ApiResponse<Bundle>> {
  return apiClient.get(`/v1/admin/domain/bundles/${id}`);
}

export function createBundle(payload: CreateBundlePayload): Promise<ApiResponse<Bundle>> {
  return apiClient.post('/v1/admin/domain/bundles', payload);
}

export function updateBundle(id: string, payload: UpdateBundlePayload): Promise<ApiResponse<Bundle>> {
  return apiClient.put(`/v1/admin/domain/bundles/${id}`, payload);
}

export function deleteBundle(id: string): Promise<ApiResponse<void>> {
  return apiClient.delete(`/v1/admin/domain/bundles/${id}`);
}
```

---

### Phase 6: Frontend - List Page

**File**: `dairy/src/routes/(app)/{domain}/{entities}/+page.svelte`

- [ ] Load function fetching list via command
- [ ] DataTable with columns for key fields
- [ ] Status pills for boolean fields (is_live, etc.)
- [ ] Row click navigates to detail page
- [ ] "New" button linking to create page

**Load function** (`+page.ts`):

```typescript
import { getBundles } from '@cattle-grid/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async () => {
  const response = await getBundles();
  return { bundles: response.data };
};
```

**List page pattern**:

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { DataTable, Button } from '@underlay/components';
  import type { Bundle } from '@cattle-grid/types';

  export let data: { bundles: Bundle[] };
</script>

<div class="page-header">
  <h1>Bundles</h1>
  <Button href="/domain/bundles/new">New Bundle</Button>
</div>

<DataTable
  items={data.bundles}
  columns={[
    { key: 'name', label: 'Name', sortable: true },
    { key: 'slug', label: 'Slug' },
    {
      key: 'isLive',
      label: 'Status',
      render: (value) => value ? 'Live' : 'Draft'
    },
  ]}
  onRowClick={(bundle) => goto(`/domain/bundles/${bundle.id}`)}
/>
```

---

### Phase 7: Frontend - Detail Page

**File**: `dairy/src/routes/(app)/{domain}/{entities}/[id]/+page.svelte`

- [ ] Load function fetching single entity
- [ ] Form with fields matching Update payload
- [ ] Save button calling update command
- [ ] Delete button with confirmation modal
- [ ] Breadcrumb navigation

**Load function** (`+page.ts`):

```typescript
import { getBundle } from '@cattle-grid/commands';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
  const response = await getBundle(params.id);
  return { bundle: response.data };
};
```

**Detail page pattern**:

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { AlertDialog, Field, TextInput, Switch, Button } from '@underlay/components';
  import { updateBundle, deleteBundle } from '@cattle-grid/commands';
  import type { Bundle } from '@cattle-grid/types';

  export let data: { bundle: Bundle };

  let form = { ...data.bundle };
  let showDeleteModal = false;
  let saving = false;

  async function handleSave() {
    saving = true;
    try {
      await updateBundle(data.bundle.id, {
        name: form.name,
        slug: form.slug,
        isLive: form.isLive,
      });
      // Show success toast or refresh
    } finally {
      saving = false;
    }
  }

  async function handleDelete() {
    await deleteBundle(data.bundle.id);
    goto('/domain/bundles');
  }
</script>

<div class="page-header">
  <nav class="breadcrumb">
    <a href="/domain/bundles">Bundles</a> / {data.bundle.name}
  </nav>
</div>

<form on:submit|preventDefault={handleSave}>
  <Field label="Name">
    <TextInput bind:value={form.name} />
  </Field>

  <Field label="Slug">
    <TextInput bind:value={form.slug} />
  </Field>

  <Field label="Live">
    <Switch bind:checked={form.isLive} />
  </Field>

  <div class="form-actions">
    <Button type="submit" loading={saving}>Save</Button>
    <Button variant="danger" on:click={() => showDeleteModal = true}>Delete</Button>
  </div>
</form>

{#if showDeleteModal}
  <AlertDialog
    title="Delete Bundle"
    description="Are you sure you want to delete this bundle?"
    onConfirm={handleDelete}
    onCancel={() => showDeleteModal = false}
  />
{/if}
```

---

### Phase 8: Frontend - Create Page

**File**: `dairy/src/routes/(app)/{domain}/{entities}/new/+page.svelte`

- [ ] Form with fields matching Create payload
- [ ] Create button calling create command
- [ ] Redirect to detail page on success

**Create page pattern**:

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { Field, TextInput, Switch, Button } from '@underlay/components';
  import { createBundle } from '@cattle-grid/commands';

  let form = {
    name: '',
    slug: '',
    isLive: false,
  };
  let saving = false;

  async function handleCreate() {
    saving = true;
    try {
      const response = await createBundle(form);
      goto(`/domain/bundles/${response.data.id}`);
    } finally {
      saving = false;
    }
  }
</script>

<div class="page-header">
  <nav class="breadcrumb">
    <a href="/domain/bundles">Bundles</a> / New
  </nav>
</div>

<form on:submit|preventDefault={handleCreate}>
  <Field label="Name">
    <TextInput bind:value={form.name} />
  </Field>

  <Field label="Slug">
    <TextInput bind:value={form.slug} />
  </Field>

  <Field label="Live">
    <Switch bind:checked={form.isLive} />
  </Field>

  <div class="form-actions">
    <Button type="submit" loading={saving}>Create</Button>
  </div>
</form>
```

---

## Dairy-Scale Extension Checklist (Required for Complex Admin Areas)

Use these extension phases whenever the admin surface has nested entities, richer forms, or operational workflows.

### Phase 9: Authenticated SPA Data Flow

- [ ] Use `useAuthenticatedData()` instead of plain `+page.ts` data-only loading for protected routes.
- [ ] Trigger fetch using auth readiness (`authLoading` + current user state).
- [ ] Expose clear loading, empty, and error states (`PageLoading`, `FormError`).
- [ ] Keep token access in auth store (do not pass tokens through page data).

Primary references:
- [110-admin.md#complete-crud-admin-pattern](../guides/110-admin.md#complete-crud-admin-pattern)
- [110-admin.md#createedit-page-pattern](../guides/110-admin.md#createedit-page-pattern)

### Phase 10: Form Shell + Intent Pattern

- [ ] Wrap create/edit pages in `SpaFormShell`.
- [ ] Support intents: `save`, `save-close`, and `delete` (edit mode).
- [ ] Return `SpaFormResult` with `fieldErrors` mapped to form fields.
- [ ] Use a reusable form component that renders fields only (no page-level routing logic inside form component).
- [ ] Keep destructive actions explicit using `ConfirmAction`/`AlertDialog`.

Primary references:
- [110-admin.md#form-component-pattern](../guides/110-admin.md#form-component-pattern)
- [110-admin.md#createedit-page-pattern](../guides/110-admin.md#createedit-page-pattern)

### Phase 11: Navigation Context and Tab-Aware Back Links

- [ ] Use `consumeNavigationContext()` on create/edit/detail routes.
- [ ] Use `gotoWithContext()` from list cards/tab lists.
- [ ] Include active tab in context href for detail pages with tabs.
- [ ] Compute back links dynamically with `computeBackInfo()`.

Primary references:
- [110-admin.md#navigation-context-with-tabs](../guides/110-admin.md#navigation-context-with-tabs)
- [110-admin.md#detail-page-header-pattern](../guides/110-admin.md#detail-page-header-pattern)

### Phase 12: List Controllers (Pagination, Batch Actions, Reorder)

- [ ] Use `createPaginationController()` for server pagination.
- [ ] Add `FilterBar` + search/filter inputs where datasets are non-trivial.
- [ ] Add `useBatchActions()` + `BatchActionBar` for multi-select delete/archive workflows.
- [ ] Add `createReorderController()` + `ReorderableList` when sequence/order matters.
- [ ] Keep list components autonomous and reusable between page and tab contexts.

Primary references:
- [110-admin.md#tab-content-pattern-list-view](../guides/110-admin.md#tab-content-pattern-list-view)
- [nested-entity-management.md#phase-5-frontend---tab-on-parent-page](./nested-entity-management.md#phase-5-frontend---tab-on-parent-page)

### Phase 13: Nested Entity and Relation Workflows

- [ ] For child entities under a parent, apply the nested recipe (`/parents/:id/children` list + independent child CRUD).
- [ ] Use tab content for child lists and dedicated child create/edit/detail routes.
- [ ] Add relation picking/search where entities reference other resources (`RelationSelector` + local/remote search functions).
- [ ] Support inline-create relations only when it clearly reduces authoring friction.

Primary references:
- [nested-entity-management.md](./nested-entity-management.md)
- [live-validation-endpoint.md](./live-validation-endpoint.md)

### Phase 14: Rich Content and Validation

- [ ] Use `MarkdownEditor` for `TEXT` fields and `NightfireEditor` for `JSONB`.
- [ ] Run field validation endpoints for uniqueness and scoped constraints.
- [ ] Show rich content on detail pages with `ContentCard` and structured details sections.
- [ ] Keep server-side validation as source of truth (`validation_to_app_error`, Nightfire validation mapping).

Primary references:
- [050-database.md#rich-text-field-conventions](../guides/050-database.md#rich-text-field-conventions)
- [070-api-handlers.md#nightfire-content-validation](../guides/070-api-handlers.md#nightfire-content-validation)
- [live-validation-endpoint.md](./live-validation-endpoint.md)

---

## Atomic Patterns Used

This recipe uses these atomic patterns:

| Pattern | Phase | Guide |
|---------|-------|-------|
| ExistsCheck | 1 | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| parse_uuid_path_raw | 3 | [070-api-handlers.md#uuid-path-parameter-parsing](../guides/070-api-handlers.md#uuid-path-parameter-parsing) |
| validation_to_app_error | 3 | [070-api-handlers.md#validator-crate-integration](../guides/070-api-handlers.md#validator-crate-integration) |
| ok/created/no_content | 3 | [070-api-handlers.md#response-helpers](../guides/070-api-handlers.md#response-helpers) |
| error_response | 3 | [070-api-handlers.md#errors](../guides/070-api-handlers.md#errors) |

---

### Detail Page View Structure

Detail pages should use `DetailsGrid` with `DetailsSection` to organize information. Always include a **Timestamps** section at the end showing creation and update times.

**Components**:
- `DetailsGrid` - Container for all detail sections
- `DetailsSection` - Groups related fields with a legend
- `DetailsItem` - Individual field display (label + value)
- `TimeAgo` - Renders relative time with tooltip for exact time

**Standard pattern**:

```svelte
<script lang="ts">
  import {
    DetailsGrid,
    DetailsItem,
    DetailsSection,
    TimeAgo
  } from "@decodelabs/underlay/components";
</script>

<div class="underlay-details-content">
  <DetailsGrid>
    <!-- Domain-specific sections -->
    <DetailsSection legend="Details">
      <DetailsItem label="Name" value={entity.name} />
      <DetailsItem label="Slug" value={entity.slug} code />
    </DetailsSection>

    <!-- Timestamps section - always last -->
    <DetailsSection legend="Timestamps">
      <DetailsItem label="Created">
        <TimeAgo date={entity.createdAt} />
      </DetailsItem>
      <DetailsItem label="Last Updated">
        <TimeAgo date={entity.updatedAt} />
      </DetailsItem>
    </DetailsSection>
  </DetailsGrid>
</div>
```

**DetailsItem props**:
- `label` - Field label (uppercase styling applied)
- `value` - Plain text/number value
- `code` - Display value as monospace code
- `span` - Column span (`number` or `"full"` for entire row)
- `muted` - Show as secondary/less important
- `children` - Custom content snippet instead of plain value

**Full-width grid**: For simpler detail views, make the grid single-column:

```svelte
<DetailsGrid class="my-details-grid">
  ...
</DetailsGrid>

<style>
  :global(.my-details-grid) {
    grid-template-columns: 1fr !important;
  }
</style>
```

**Markdown content**: Use `ContentCard` with `markdown` prop for rich text fields:

```svelte
<ContentCard
  title="Description"
  value={entity.description}
  markdown
  emptyMessage="No description set."
  maxHeight={0}
/>
```

---

## Variations

### With Pagination

Add to list endpoint:
- Use `PaginationParams` in handler
- Return `Paginated<EntityDto>`
- See [070-api-handlers.md#pagination](../guides/070-api-handlers.md#pagination)

### With Sorting/Filtering

Add to list endpoint:
- Use `QueryParams` and `FieldMapping`
- See [070-api-handlers.md#query-field-mapping](../guides/070-api-handlers.md#query-field-mapping)

### With Nightfire Content

Add to DTOs and forms:
- Use `serde_json::Value` for JSONB fields
- Use `NightfireEditor` component
- Validate with `nightfire_validation_to_app_error()`
- See [070-api-handlers.md#nightfire-content-validation](../guides/070-api-handlers.md#nightfire-content-validation)
