# Recipe: Live Validation Endpoint

**Use when**: You need real-time field validation (e.g., checking slug availability as user types).

**Example prompt**: "Add live slug validation for modules"

---

## Key Principle

Live validation endpoints return **200 OK** with a `ValidationResult` body, not HTTP error codes. This allows the frontend to display validation messages without treating them as errors.

---

## Checklist

### Phase 1: Backend - Validation Route

**File**: `crates/api/src/routes/admin/{domain}.rs`

- [ ] Add route: `POST /v1/admin/{domain}/validate-{field}`
- [ ] Use `ValidationResult` for response
- [ ] Use `parse_uuid_for_validation()` for UUID parameters
- [ ] Return suggestions when possible

**Payload**:

```rust
#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSlugPayload {
    pub slug: String,
    #[serde(default)]
    pub exclude_id: Option<String>,  // For updates - exclude current record
}
```

**Handler pattern** ([070-api-handlers.md](../guides/070-api-handlers.md#live-field-validation)):

```rust
use underlay_http::{ValidationResult, parse_optional_uuid_for_validation};

async fn validate_slug(
    State(state): State<AppState>,
    Json(payload): Json<ValidateSlugPayload>,
) -> impl IntoResponse {
    // Parse optional UUID - returns ValidationResult on error, not HTTP error
    let exclude_id = match parse_optional_uuid_for_validation(
        payload.exclude_id.as_deref(),
        "excludeId"
    ) {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    // Check existence
    let exists = db::module_slug_exists(&state.pool, &payload.slug, exclude_id)
        .await
        .unwrap_or(false);

    if exists {
        // Generate a suggestion if possible
        let suggestion = generate_unique_slug(&state.pool, &payload.slug).await;

        Json(ValidationResult::invalid_with_suggestion(
            "Slug already exists",
            suggestion,
        ))
    } else {
        Json(ValidationResult::valid())
    }
}
```

**Route setup**:

```rust
pub fn routes() -> Router<AppState> {
    Router::new()
        // ... other routes
        .route("/v1/admin/learning/modules/validate-slug", post(validate_slug))
}
```

---

### Phase 2: Client - Validation Command

**File**: `cattle-grid/src/commands/{domain}-commands.ts`

- [ ] Add `validate{Field}(payload)` command
- [ ] Add `Validate{Field}Payload` type

**Types**:

```typescript
export interface ValidateSlugPayload {
  slug: string;
  excludeId?: string;
}

export interface ValidationResult {
  valid: boolean;
  message?: string;
  suggestion?: string;
}
```

**Command**:

```typescript
export function validateModuleSlug(
  payload: ValidateSlugPayload
): Promise<ApiResponse<ValidationResult>> {
  return apiClient.post('/v1/admin/learning/modules/validate-slug', payload);
}
```

---

### Phase 3: Frontend - Field Validation

**File**: Form component using the field

- [ ] Add debounced validation on field change
- [ ] Show validation message below field
- [ ] Show suggestion with "Use this" button if available

**Validation hook pattern**:

```typescript
import { debounce } from '$lib/utils';
import { validateModuleSlug } from '@cattle-grid/commands';

let slug = '';
let slugValidation: ValidationResult | null = null;
let validating = false;

const validateSlug = debounce(async (value: string, excludeId?: string) => {
  if (!value) {
    slugValidation = null;
    return;
  }

  validating = true;
  try {
    const response = await validateModuleSlug({ slug: value, excludeId });
    slugValidation = response.data;
  } finally {
    validating = false;
  }
}, 300);

// Call on input change
$: validateSlug(slug, existingId);
```

**Form field pattern**:

```svelte
<Field label="Slug" error={slugValidation?.valid === false ? slugValidation.message : undefined}>
  <TextInput
    bind:value={slug}
    on:input={() => validateSlug(slug, existingId)}
  />

  {#if validating}
    <span class="validating">Checking...</span>
  {/if}

  {#if slugValidation?.suggestion}
    <div class="suggestion">
      <span>Try: {slugValidation.suggestion}</span>
      <Button size="sm" onClick={() => slug = slugValidation.suggestion}>
        Use this
      </Button>
    </div>
  {/if}
</Field>
```

---

## Response Format

**Valid**:
```json
{
  "valid": true
}
```

**Invalid without suggestion**:
```json
{
  "valid": false,
  "message": "Slug already exists"
}
```

**Invalid with suggestion**:
```json
{
  "valid": false,
  "message": "Slug already exists",
  "suggestion": "my-slug-2"
}
```

---

## Atomic Patterns Used

| Pattern | Purpose | Guide |
|---------|---------|-------|
| ValidationResult | Response type | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| parse_uuid_for_validation | UUID parsing without HTTP errors | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| parse_optional_uuid_for_validation | Optional UUID parsing | [070-api-handlers.md#live-field-validation](../guides/070-api-handlers.md#live-field-validation) |
| ExistsCheck | Database existence check | [050-database.md#existscheck-builder](../guides/050-database.md#existscheck-builder) |

---

## Variations

### Multiple Fields

For validating multiple fields at once:

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateModulePayload {
    pub slug: Option<String>,
    pub code: Option<String>,
    pub exclude_id: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateModuleResult {
    pub slug: Option<ValidationResult>,
    pub code: Option<ValidationResult>,
}
```

### With Format Validation

Validate format before checking uniqueness:

```rust
async fn validate_slug(payload: Json<ValidateSlugPayload>) -> impl IntoResponse {
    // Format validation first
    if !is_valid_slug_format(&payload.slug) {
        return Json(ValidationResult::invalid(
            "Slug must be lowercase with hyphens only"
        ));
    }

    // Then uniqueness check
    // ...
}
```

### Composite Uniqueness

For fields unique within a scope (e.g., slug + pathway_id):

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateSlugPayload {
    pub slug: String,
    pub pathway_id: String,
    pub exclude_id: Option<String>,
}

async fn validate_slug(payload: Json<ValidateSlugPayload>) -> impl IntoResponse {
    let pathway_id = match parse_uuid_for_validation(&payload.pathway_id, "pathwayId") {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    let exclude_id = match parse_optional_uuid_for_validation(
        payload.exclude_id.as_deref(), "excludeId"
    ) {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    let exists = ExistsCheck::new("learning", "module")
        .value("slug", &payload.slug)
        .scope("pathway_id", pathway_id)
        .excluding_optional(exclude_id)
        .check(&state.pool)
        .await
        .unwrap_or(false);

    if exists {
        Json(ValidationResult::invalid("Slug already exists in this pathway"))
    } else {
        Json(ValidationResult::valid())
    }
}
```
