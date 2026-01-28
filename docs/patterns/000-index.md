# Underlay Patterns Catalogue

Quick reference for implementation patterns. Use this to find the right approach for common tasks.

## How to Use This Catalogue

1. **Composite Recipes** (below) - Full system implementations. Start here for "build X" tasks.
2. **Atomic Patterns** - Individual techniques. Use when you need a specific helper.

---

## Composite Recipes

These recipes combine multiple patterns into complete, repeatable implementations.

| Recipe | Description | Guide |
|--------|-------------|-------|
| [CRUD Admin Interface](#recipe-crud-admin-interface) | Full create/read/update/delete for an entity | See below |
| [Live Validation Endpoint](#recipe-live-validation-endpoint) | Real-time field validation (e.g., slug availability) | See below |
| [Nested Entity Management](#recipe-nested-entity-management) | Child entities within a parent (e.g., Module Variants) | See below |

---

## Recipe: CRUD Admin Interface

**Use when**: You need a complete admin interface for a database table.

**Example prompt**: "Build the CRUD interface for the Bundles table"

### Checklist

#### Phase 1: Backend - Database Layer (`crates/db/src/{domain}.rs`)

- [ ] `list_{entities}(pool) -> Vec<Row>` - List all (with soft-delete filter)
- [ ] `get_{entity}_by_id(pool, id) -> Option<Row>` - Single record lookup
- [ ] `create_{entity}(pool, ..fields..) -> Row` - Insert with RETURNING
- [ ] `update_{entity}(pool, id, ..fields..) -> Option<Row>` - Update with RETURNING
- [ ] `soft_delete_{entity}(pool, id)` - Set deleted_at + delete_batch_id
- [ ] Existence checks using `ExistsCheck` for unique fields

**Existence check pattern** ([050-database.md](../guides/050-database.md#existscheck-builder)):
```rust
pub async fn {entity}_slug_exists(pool: &DbPool, slug: &str, exclude_id: Option<Uuid>) -> Result<bool, sqlx::Error> {
    let mut check = ExistsCheck::new("schema", "table").value("slug", slug);
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

#### Phase 2: Backend - DTOs (`crates/api/src/dto/{domain}.rs`)

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
```

#### Phase 3: Backend - Routes (`crates/api/src/routes/admin/{domain}.rs`)

- [ ] `GET /v1/admin/{domain}/{entities}` - List all
- [ ] `GET /v1/admin/{domain}/{entities}/:id` - Get single
- [ ] `POST /v1/admin/{domain}/{entities}` - Create
- [ ] `PUT /v1/admin/{domain}/{entities}/:id` - Update
- [ ] `DELETE /v1/admin/{domain}/{entities}/:id` - Soft delete

**Route pattern** ([070-api-handlers.md](../guides/070-api-handlers.md)):
```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/admin/domain/entities", get(list_entities).post(create_entity))
        .route("/v1/admin/domain/entities/:id", get(get_entity).put(update_entity).delete(delete_entity))
}
```

**Handler pattern**:
```rust
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
```

#### Phase 4: Client - Types (`cattle-grid/src/types/{domain}-types.ts`)

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
```

#### Phase 5: Client - Commands (`cattle-grid/src/commands/{domain}-commands.ts`)

- [ ] `get{Entities}()` - List all
- [ ] `get{Entity}(id)` - Get single
- [ ] `create{Entity}(payload)` - Create
- [ ] `update{Entity}(id, payload)` - Update
- [ ] `delete{Entity}(id)` - Soft delete

**Command pattern**:
```typescript
export function getBundles(): Promise<ApiResponse<Bundle[]>> {
  return apiClient.get('/v1/admin/domain/bundles');
}

export function getBundle(id: string): Promise<ApiResponse<Bundle>> {
  return apiClient.get(`/v1/admin/domain/bundles/${id}`);
}

export function createBundle(payload: CreateBundlePayload): Promise<ApiResponse<Bundle>> {
  return apiClient.post('/v1/admin/domain/bundles', payload);
}
```

#### Phase 6: Frontend - List Page (`dairy/src/routes/(app)/{domain}/{entities}/+page.svelte`)

- [ ] Load function fetching list via command
- [ ] DataTable with columns for key fields
- [ ] Status pills for boolean fields (is_live, etc.)
- [ ] Row click navigates to detail page
- [ ] "New" button linking to create page

**List page pattern**:
```svelte
<script lang="ts">
  import { DataTable, StatusPill } from '@underlay/components';
  import type { Bundle } from '@cattle-grid/types';

  export let data: { bundles: Bundle[] };
</script>

<DataTable
  items={data.bundles}
  columns={[
    { key: 'name', label: 'Name' },
    { key: 'slug', label: 'Slug' },
    { key: 'isLive', label: 'Status', render: (v) => StatusPill({ live: v }) },
  ]}
  onRowClick={(bundle) => goto(`/domain/bundles/${bundle.id}`)}
/>
```

#### Phase 7: Frontend - Detail Page (`dairy/src/routes/(app)/{domain}/{entities}/[id]/+page.svelte`)

- [ ] Load function fetching single entity
- [ ] Form with fields matching Update payload
- [ ] Save button calling update command
- [ ] Delete button with confirmation modal
- [ ] Breadcrumb navigation

**Form pattern**:
```svelte
<script lang="ts">
  import { FormField, TextInput, Toggle, Button } from '@underlay/components';

  export let data: { bundle: Bundle };

  let form = { ...data.bundle };

  async function handleSave() {
    await updateBundle(data.bundle.id, form);
  }
</script>

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
  <Button type="submit">Save</Button>
</form>
```

#### Phase 8: Frontend - Create Page (`dairy/src/routes/(app)/{domain}/{entities}/new/+page.svelte`)

- [ ] Form with fields matching Create payload
- [ ] Create button calling create command
- [ ] Redirect to detail page on success

---

## Recipe: Live Validation Endpoint

**Use when**: You need real-time field validation (e.g., checking slug availability as user types).

**Example prompt**: "Add live slug validation for modules"

### Checklist

#### Backend

- [ ] Add validation route: `POST /v1/admin/{domain}/validate-{field}`
- [ ] Use `ValidationResult` for response (always 200 OK)
- [ ] Use `parse_uuid_for_validation()` for UUID parameters
- [ ] Return suggestions when possible

**Handler pattern** ([070-api-handlers.md](../guides/070-api-handlers.md#live-field-validation)):
```rust
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSlugPayload {
    pub slug: String,
    pub exclude_id: Option<String>,
}

async fn validate_slug(
    State(state): State<AppState>,
    Json(payload): Json<ValidateSlugPayload>,
) -> impl IntoResponse {
    let exclude_id = match parse_optional_uuid_for_validation(
        payload.exclude_id.as_deref(), "excludeId"
    ) {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    let exists = db::slug_exists(&state.pool, &payload.slug, exclude_id).await
        .unwrap_or(false);

    if exists {
        Json(ValidationResult::invalid_with_suggestion(
            "Slug already exists",
            generate_unique_slug(&payload.slug),
        ))
    } else {
        Json(ValidationResult::valid())
    }
}
```

#### Client

- [ ] Add `validate{Field}(payload)` command
- [ ] Add `Validate{Field}Payload` type

#### Frontend

- [ ] Add debounced validation on field blur/change
- [ ] Show validation message below field
- [ ] Show suggestion with "Use this" button

---

## Recipe: Nested Entity Management

**Use when**: Managing child entities within a parent (e.g., Module Variants within a Module).

**Example prompt**: "Build the Variants tab for Modules"

### Checklist

#### Backend

- [ ] List endpoint scoped to parent: `GET /v1/admin/{parent}/:id/{children}`
- [ ] Create endpoint with parent ID: `POST /v1/admin/{parent}/:id/{children}`
- [ ] Detail/update/delete at child level: `GET/PUT/DELETE /v1/admin/{children}/:id`
- [ ] Existence checks scoped to parent

#### Client

- [ ] `get{Parent}{Children}(parentId)` - List children for parent
- [ ] Standard CRUD commands for child entity

#### Frontend

- [ ] Add tab to parent detail page
- [ ] Tab shows list of children with "New" button
- [ ] Child detail page with breadcrumb: Parent > [Name] > Children > [Child Name]

---

## Atomic Patterns

Individual techniques referenced by the recipes above.

### Database Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| ExistsCheck builder | Flexible existence checks | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| Composite uniqueness | Slug + year, slug + parent_id | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| Tables without soft-delete | `.include_deleted()` | [050-database.md#including-deleted-records](../guides/050-database.md#including-deleted-records) |
| Docs-first schema | Document before migrate | [050-database.md#docs-first-schema-development](../guides/050-database.md#docs-first-schema-development) |

### API Handler Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| UUID path parsing | `parse_uuid_path_raw()` | [070-api-handlers.md#uuid-path-parameter-parsing](../guides/070-api-handlers.md#uuid-path-parameter-parsing) |
| Live field validation | `ValidationResult` | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| Validator errors | `validation_to_app_error()` | [070-api-handlers.md#validator-crate-integration](../guides/070-api-handlers.md#validator-crate-integration) |
| Nightfire validation | `nightfire_validation_to_app_error()` | [070-api-handlers.md#nightfire-content-validation](../guides/070-api-handlers.md#nightfire-content-validation) |
| Response helpers | `ok()`, `created()`, `list_ok()` | [070-api-handlers.md#response-helpers](../guides/070-api-handlers.md#response-helpers) |
| Pagination | `PaginationParams`, `Paginated<T>` | [070-api-handlers.md#pagination](../guides/070-api-handlers.md#pagination) |
| Field mapping | `FieldMapping` for sort/filter | [070-api-handlers.md#query-field-mapping](../guides/070-api-handlers.md#query-field-mapping) |

### Authentication & Authorization

| Pattern | Description | Guide |
|---------|-------------|-------|
| Request context | `RequestContext`, `AuthenticatedContext` | [070-api-handlers.md#request-context](../guides/070-api-handlers.md#request-context) |
| Auth middleware | JWT validation, user extraction | [060-authentication.md](../guides/060-authentication.md) |
| Role-based access | Permission checks in handlers | [067-authorization.md](../guides/067-authorization.md) |

### Frontend Patterns

| Pattern | Description | Guide |
|---------|-------------|-------|
| DataTable | Sortable, filterable tables | [110-sveltekit-frontend.md](../guides/110-sveltekit-frontend.md) |
| Form handling | Form state, validation, submission | [110-sveltekit-frontend.md](../guides/110-sveltekit-frontend.md) |
| Status pills | Live/draft badges | Component library |
| Tabs | Tabbed navigation within pages | Component library |

---

## Adding New Recipes

When you discover a new repeatable pattern:

1. Add a recipe section with clear "Use when" guidance
2. Create a checklist covering all layers (backend → client → frontend)
3. Include code snippets showing the pattern
4. Link to relevant atomic patterns
5. Update this index

