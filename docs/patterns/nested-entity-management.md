# Recipe: Nested Entity Management

**Use when**: Managing child entities within a parent (e.g., Module Variants within a Module, Sections within a Module).

**Example prompt**: "Build the Variants tab for Modules"

---

## Key Principle

Child entities are managed through:
1. **Scoped list endpoint** on parent: `GET /parents/:id/children`
2. **Independent CRUD** on child: `GET/PUT/DELETE /children/:id`
3. **Frontend tab** on parent detail page

---

## Checklist

### Phase 1: Backend - Database Layer

**File**: `crates/db/src/{domain}.rs`

- [ ] `list_{children}_for_{parent}(pool, parent_id) -> Vec<Row>` - Scoped list
- [ ] `get_{child}_by_id(pool, id) -> Option<Row>` - Single lookup
- [ ] `create_{child}(pool, parent_id, ..fields..) -> Row` - Create with parent FK
- [ ] `update_{child}(pool, id, ..fields..) -> Option<Row>` - Update
- [ ] `soft_delete_{child}(pool, id)` - Soft delete
- [ ] Existence checks scoped to parent

**Scoped list pattern**:

```rust
pub async fn list_variants_for_module(
    pool: &DbPool,
    module_id: Uuid,
) -> Result<Vec<ModuleVariantRow>, sqlx::Error> {
    sqlx::query_as!(
        ModuleVariantRow,
        r#"
        SELECT id, module_id, key, name, weight, is_live, is_default, created_at, updated_at
        FROM learning.module_variant
        WHERE module_id = $1 AND deleted_at IS NULL
        ORDER BY name
        "#,
        module_id
    )
    .fetch_all(pool)
    .await
}
```

**Scoped existence check**:

```rust
pub async fn variant_key_exists(
    pool: &DbPool,
    module_id: Uuid,
    key: &str,
    exclude_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let mut check = ExistsCheck::new("learning", "module_variant")
        .scope("module_id", module_id)
        .value("key", key);
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

---

### Phase 2: Backend - DTOs

**File**: `crates/api/src/dto/{domain}.rs`

- [ ] `{Child}Dto` - Response shape
- [ ] `Create{Child}Payload` - With parent_id field
- [ ] `Update{Child}Payload` - Without parent_id (can't change parent)

```rust
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ModuleVariantDto {
    pub id: String,
    pub module_id: String,
    pub key: String,
    pub name: String,
    pub weight: f64,
    pub is_live: bool,
    pub is_default: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateModuleVariantPayload {
    pub module_id: String,  // Parent reference
    #[validate(length(min = 1, max = 50))]
    pub key: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub weight: f64,
    pub is_live: bool,
    pub is_default: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateModuleVariantPayload {
    // No module_id - can't change parent
    #[validate(length(min = 1, max = 50))]
    pub key: String,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub weight: f64,
    pub is_live: bool,
    pub is_default: bool,
}
```

---

### Phase 3: Backend - Routes

**File**: `crates/api/src/routes/admin/{domain}.rs`

- [ ] `GET /v1/admin/{domain}/{parents}/:id/{children}` - List children for parent
- [ ] `POST /v1/admin/{domain}/{children}` - Create child (parent_id in body)
- [ ] `GET /v1/admin/{domain}/{children}/:id` - Get single child
- [ ] `PUT /v1/admin/{domain}/{children}/:id` - Update child
- [ ] `DELETE /v1/admin/{domain}/{children}/:id` - Delete child

**Route setup**:

```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        // Parent-scoped list
        .route("/v1/admin/learning/modules/:id/variants", get(list_module_variants))
        // Child CRUD (independent of parent in URL)
        .route("/v1/admin/learning/variants", post(create_variant))
        .route("/v1/admin/learning/variants/:id", get(get_variant).put(update_variant).delete(delete_variant))
}
```

**List handler** (scoped to parent):

```rust
async fn list_module_variants(
    State(state): State<AppState>,
    Path(module_id): Path<String>,
) -> impl IntoResponse {
    let module_id = parse_uuid_path_raw(&module_id, "moduleId")?;

    let rows = db::list_variants_for_module(&state.pool, module_id).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    let dtos: Vec<ModuleVariantDto> = rows.into_iter().map(Into::into).collect();
    list_ok(dtos).into_response()
}
```

**Create handler** (parent_id from body):

```rust
async fn create_variant(
    State(state): State<AppState>,
    Json(payload): Json<CreateModuleVariantPayload>,
) -> impl IntoResponse {
    let module_id = parse_uuid_path_raw(&payload.module_id, "moduleId")?;

    if let Err(e) = payload.validate() {
        let err = validation_to_app_error(&e, "variant.invalid", "Validation failed.");
        return error_response(StatusCode::BAD_REQUEST, err).into_response();
    }

    // Check uniqueness scoped to parent
    if db::variant_key_exists(&state.pool, module_id, &payload.key, None).await.unwrap_or(false) {
        return error_response(StatusCode::CONFLICT,
            AppError::new("variant.key_exists", "Key already exists for this module")).into_response();
    }

    let row = db::create_variant(&state.pool, module_id, &payload.key, &payload.name, payload.weight, payload.is_live, payload.is_default).await
        .map_err(|e| error_response(StatusCode::INTERNAL_SERVER_ERROR,
            AppError::new("db.error", e.to_string())))?;

    created(ModuleVariantDto::from(row)).into_response()
}
```

---

### Phase 4: Client - Types and Commands

**File**: `cattle-grid/src/types/{domain}-types.ts`

```typescript
export interface ModuleVariant {
  id: string;
  moduleId: string;
  key: string;
  name: string;
  weight: number;
  isLive: boolean;
  isDefault: boolean;
  createdAt: string;
  updatedAt: string;
}

export interface CreateModuleVariantPayload {
  moduleId: string;
  key: string;
  name: string;
  weight: number;
  isLive: boolean;
  isDefault: boolean;
}

export interface UpdateModuleVariantPayload {
  key: string;
  name: string;
  weight: number;
  isLive: boolean;
  isDefault: boolean;
}
```

**File**: `cattle-grid/src/commands/{domain}-commands.ts`

```typescript
// Scoped list
export function getModuleVariants(moduleId: string): Promise<ApiResponse<ModuleVariant[]>> {
  return apiClient.get(`/v1/admin/learning/modules/${moduleId}/variants`);
}

// Independent CRUD
export function getModuleVariant(id: string): Promise<ApiResponse<ModuleVariant>> {
  return apiClient.get(`/v1/admin/learning/variants/${id}`);
}

export function createModuleVariant(payload: CreateModuleVariantPayload): Promise<ApiResponse<ModuleVariant>> {
  return apiClient.post('/v1/admin/learning/variants', payload);
}

export function updateModuleVariant(id: string, payload: UpdateModuleVariantPayload): Promise<ApiResponse<ModuleVariant>> {
  return apiClient.put(`/v1/admin/learning/variants/${id}`, payload);
}

export function deleteModuleVariant(id: string): Promise<ApiResponse<void>> {
  return apiClient.delete(`/v1/admin/learning/variants/${id}`);
}
```

---

### Phase 5: Frontend - Tab on Parent Page

**File**: `dairy/src/routes/(app)/{domain}/{parents}/[id]/+page.svelte`

- [ ] Add tab component to parent detail page
- [ ] Tab loads children list
- [ ] "New" button in tab header
- [ ] Each row links to child detail page

**Tab pattern**:

```svelte
<script lang="ts">
  import { Tabs, TabList, Tab, TabPanel } from '@underlay/components';
  import { getModuleVariants } from '@cattle-grid/commands';
  import type { Module, ModuleVariant } from '@cattle-grid/types';

  export let data: { module: Module };

  let variants: ModuleVariant[] = [];
  let loadingVariants = false;

  async function loadVariants() {
    loadingVariants = true;
    try {
      const response = await getModuleVariants(data.module.id);
      variants = response.data;
    } finally {
      loadingVariants = false;
    }
  }
</script>

<Tabs>
  <TabList>
    <Tab>Details</Tab>
    <Tab on:select={loadVariants}>Variants ({variants.length})</Tab>
  </TabList>

  <TabPanel>
    <!-- Main module form -->
  </TabPanel>

  <TabPanel>
    <div class="tab-header">
      <h2>Variants</h2>
      <Button href="/learning/modules/{data.module.id}/variants/new">
        New Variant
      </Button>
    </div>

    {#if loadingVariants}
      <Loading />
    {:else}
      <DataTable
        items={variants}
        columns={[
          { key: 'name', label: 'Name' },
          { key: 'key', label: 'Key' },
          { key: 'weight', label: 'Weight' },
          { key: 'isLive', label: 'Status', render: (v) => StatusPill({ live: v }) },
          { key: 'isDefault', label: 'Default', render: (v) => v ? 'Yes' : '' },
        ]}
        onRowClick={(variant) => goto(`/learning/variants/${variant.id}`)}
      />
    {/if}
  </TabPanel>
</Tabs>
```

---

### Phase 6: Frontend - Child Detail Page

**File**: `dairy/src/routes/(app)/{domain}/{children}/[id]/+page.svelte`

- [ ] Load function fetching child (and optionally parent for breadcrumb)
- [ ] Form with update fields
- [ ] Breadcrumb: Parent > [Parent Name] > Children > [Child Name]
- [ ] Delete with redirect to parent

**Load function**:

```typescript
import { getModuleVariant, getModule } from '@cattle-grid/commands';

export const load: PageLoad = async ({ params }) => {
  const variantResponse = await getModuleVariant(params.id);
  const variant = variantResponse.data;

  // Load parent for breadcrumb
  const moduleResponse = await getModule(variant.moduleId);

  return {
    variant,
    module: moduleResponse.data,
  };
};
```

**Detail page**:

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { updateModuleVariant, deleteModuleVariant } from '@cattle-grid/commands';

  export let data: { variant: ModuleVariant; module: Module };

  let form = { ...data.variant };

  async function handleSave() {
    await updateModuleVariant(data.variant.id, {
      key: form.key,
      name: form.name,
      weight: form.weight,
      isLive: form.isLive,
      isDefault: form.isDefault,
    });
  }

  async function handleDelete() {
    await deleteModuleVariant(data.variant.id);
    goto(`/learning/modules/${data.module.id}`);  // Back to parent
  }
</script>

<nav class="breadcrumb">
  <a href="/learning/modules">Modules</a>
  <a href="/learning/modules/{data.module.id}">{data.module.name}</a>
  <span>Variants</span>
  <span>{data.variant.name}</span>
</nav>

<!-- Form fields... -->
```

---

### Phase 7: Frontend - Child Create Page

**File**: `dairy/src/routes/(app)/{domain}/{parents}/[id]/{children}/new/+page.svelte`

- [ ] Load parent for context
- [ ] Form with create fields (parent_id pre-filled)
- [ ] Redirect to child detail on success

**Load function**:

```typescript
export const load: PageLoad = async ({ params }) => {
  const moduleResponse = await getModule(params.id);
  return { module: moduleResponse.data };
};
```

**Create page**:

```svelte
<script lang="ts">
  import { goto } from '$app/navigation';
  import { createModuleVariant } from '@cattle-grid/commands';

  export let data: { module: Module };

  let form = {
    key: '',
    name: '',
    weight: 1.0,
    isLive: false,
    isDefault: false,
  };

  async function handleCreate() {
    const response = await createModuleVariant({
      moduleId: data.module.id,  // Parent ID from route
      ...form,
    });
    goto(`/learning/variants/${response.data.id}`);
  }
</script>

<nav class="breadcrumb">
  <a href="/learning/modules">Modules</a>
  <a href="/learning/modules/{data.module.id}">{data.module.name}</a>
  <span>Variants</span>
  <span>New</span>
</nav>

<!-- Form fields... -->
```

---

## URL Structure

| URL | Purpose |
|-----|---------|
| `/learning/modules/:id` | Parent detail with tabs |
| `/learning/modules/:id/variants/new` | Create child (parent context) |
| `/learning/variants/:id` | Child detail/edit |

---

## Atomic Patterns Used

| Pattern | Phase | Guide |
|---------|-------|-------|
| ExistsCheck with .scope() | 1 | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |
| parse_uuid_path_raw | 3 | [070-api-handlers.md#uuid-path-parameter-parsing](../guides/070-api-handlers.md#uuid-path-parameter-parsing) |
| Tabs component | 5 | Component library |

---

## Variations

### Inline Editing

For simple children, edit in the tab without separate pages:

```svelte
<TabPanel>
  {#each variants as variant}
    <VariantInlineEditor
      {variant}
      on:save={(e) => updateModuleVariant(variant.id, e.detail)}
      on:delete={() => deleteModuleVariant(variant.id)}
    />
  {/each}
</TabPanel>
```

### Drag-and-Drop Ordering

For ordered children (sections, areas):

```svelte
<DragDropList
  items={sections}
  on:reorder={(e) => reorderSections(moduleId, e.detail.orderedIds)}
/>
```

### Multiple Nesting Levels

For deeper hierarchies (Module → Section → Area):

- Each level follows this same pattern
- Breadcrumbs extend: Module > [Name] > Sections > [Name] > Areas > [Name]
- URL structure: `/modules/:id/sections/:id/areas/:id`
