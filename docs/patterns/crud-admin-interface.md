# Recipe: CRUD Admin Interface

**Use when**: You need a complete admin interface for a database table.

**Example prompt**: "Build the CRUD interface for the Bundles table"

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
  import { DataTable, StatusPill, Button } from '@underlay/components';
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
      render: (value) => StatusPill({ live: value })
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
  import { FormField, TextInput, Toggle, Button, ConfirmModal } from '@underlay/components';
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
  <FormField label="Name">
    <TextInput bind:value={form.name} />
  </FormField>

  <FormField label="Slug">
    <TextInput bind:value={form.slug} />
  </FormField>

  <FormField label="Live">
    <Toggle bind:checked={form.isLive} />
  </FormField>

  <div class="form-actions">
    <Button type="submit" loading={saving}>Save</Button>
    <Button variant="danger" on:click={() => showDeleteModal = true}>Delete</Button>
  </div>
</form>

<ConfirmModal
  bind:open={showDeleteModal}
  title="Delete Bundle"
  message="Are you sure you want to delete this bundle?"
  confirmLabel="Delete"
  onConfirm={handleDelete}
/>
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
  import { FormField, TextInput, Toggle, Button } from '@underlay/components';
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
  <FormField label="Name">
    <TextInput bind:value={form.name} />
  </FormField>

  <FormField label="Slug">
    <TextInput bind:value={form.slug} />
  </FormField>

  <FormField label="Live">
    <Toggle bind:checked={form.isLive} />
  </FormField>

  <div class="form-actions">
    <Button type="submit" loading={saving}>Create</Button>
  </div>
</form>
```

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
