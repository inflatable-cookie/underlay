# 075 – Validation

This guide covers request validation for both backend APIs and frontend forms in your Underlay-based application.

## Overview

Validation ensures data integrity at multiple layers:
- **Frontend validation** - Immediate user feedback
- **Backend validation** - Security and data integrity
- **Database constraints** - Final enforcement

This guide covers:
- Backend request validation with structured errors
- Frontend form validation
- Error envelope integration
- Field-level error messages

## Prerequisites

- [040-rust-backend](./040-rust-backend.md) - Backend structure
- [070-api-handlers](./070-api-handlers.md) - API patterns
- [100-frontend-web](./100-frontend-web.md) - SvelteKit basics

---

## Backend Validation

### Validation Library

We recommend using [`validator`](https://crates.io/crates/validator) for Rust validation.

Add to `apps/api/Cargo.toml`:

```toml
[workspace.dependencies]
validator = { version = "0.16", features = ["derive"] }
```

Add to your API crate `apps/api/crates/api/Cargo.toml`:

```toml
[dependencies]
validator = { workspace = true }
```

### Request DTOs with Validation

Create validated request types:

```rust
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    pub password: String,

    #[validate(length(min = 1, max = 100, message = "Display name must be 1-100 characters"))]
    pub display_name: String,

    #[validate(custom = "validate_username")]
    pub username: Option<String>,
}

fn validate_username(username: &str) -> Result<(), validator::ValidationError> {
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(validator::ValidationError::new("invalid_username")
            .with_message("Username can only contain letters, numbers, and underscores".into()));
    }
    Ok(())
}
```

### Common Validation Rules

```rust
#[derive(Deserialize, Validate)]
pub struct ArticleRequest {
    // Email validation
    #[validate(email)]
    pub author_email: String,

    // Length constraints
    #[validate(length(min = 1, max = 200))]
    pub title: String,

    // Range validation
    #[validate(range(min = 1, max = 10))]
    pub rating: u32,

    // URL validation
    #[validate(url)]
    pub website: Option<String>,

    // Regex validation
    #[validate(regex = "SLUG_REGEX")]
    pub slug: String,

    // Custom validation function
    #[validate(custom = "validate_publish_date")]
    pub publish_at: Option<chrono::DateTime<chrono::Utc>>,

    // Nested validation
    #[validate]
    pub metadata: ArticleMetadata,

    // Collection validation
    #[validate(length(min = 1, max = 10))]
    #[validate]
    pub tags: Vec<Tag>,
}

lazy_static::lazy_static! {
    static ref SLUG_REGEX: regex::Regex = regex::Regex::new(r"^[a-z0-9-]+$").unwrap();
}

fn validate_publish_date(date: &chrono::DateTime<chrono::Utc>) -> Result<(), validator::ValidationError> {
    if *date < chrono::Utc::now() {
        return Err(validator::ValidationError::new("publish_date_in_past"));
    }
    Ok(())
}
```

### Validation Errors to HTTP Response

Create a helper to convert validation errors to your error envelope:

In `apps/api/crates/api/src/validation.rs`:

```rust
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use underlay_core::{AppError, ErrorEnvelope};
use validator::ValidationErrors;

/// Convert validator errors to field-level error details.
pub fn validation_errors_to_details(errors: &ValidationErrors) -> serde_json::Value {
    let field_errors: std::collections::HashMap<String, Vec<String>> = errors
        .field_errors()
        .into_iter()
        .map(|(field, errors)| {
            let messages: Vec<String> = errors
                .iter()
                .map(|e| {
                    e.message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid {}", field))
                })
                .collect();
            (field.to_string(), messages)
        })
        .collect();

    serde_json::json!({ "fields": field_errors })
}

/// Create a validation error response.
pub fn validation_error_response(errors: ValidationErrors) -> Response {
    let details = validation_errors_to_details(&errors);

    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(ErrorEnvelope {
            error: AppError {
                code: "validation_failed".into(),
                message: "Request validation failed".into(),
                details: Some(details),
            },
        }),
    )
        .into_response()
}
```

### Handler with Validation

#### Manual Validation

```rust
use axum::{Json, extract::State};
use validator::Validate;
use underlay_core::SingleResponse;

use crate::validation::validation_error_response;

pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<Json<SingleResponse<UserDto>>, Response> {
    // Validate request
    if let Err(errors) = req.validate() {
        return Err(validation_error_response(errors));
    }

    // Check for duplicate email
    if state.user_repo.find_by_email(&req.email).await?.is_some() {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorEnvelope {
                error: AppError {
                    code: "email_exists".into(),
                    message: "A user with this email already exists".into(),
                    details: Some(serde_json::json!({
                        "fields": { "email": ["Email is already registered"] }
                    })),
                },
            }),
        ).into_response());
    }

    let user = state.user_repo.create(req).await?;

    Ok(Json(SingleResponse { data: user.into() }))
}
```

#### Validated Extractor

Create a custom extractor that validates automatically:

```rust
use axum::{
    async_trait,
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
    Json,
};

/// Extractor that validates the request body.
pub struct Validated<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Validated<T>
where
    T: serde::de::DeserializeOwned + validator::Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e| e.into_response())?;

        value.validate()
            .map_err(|e| validation_error_response(e))?;

        Ok(Validated(value))
    }
}
```

Usage:

```rust
pub async fn create_user(
    State(state): State<AppState>,
    Validated(req): Validated<CreateUserRequest>,  // Auto-validated!
) -> Json<SingleResponse<UserDto>> {
    // req is already validated here
    let user = state.user_repo.create(req).await?;

    Json(SingleResponse { data: user.into() })
}
```

### Business Logic Validation

Some validation requires database queries or business logic:

```rust
pub async fn update_article(
    State(state): State<AppState>,
    Validated(req): Validated<UpdateArticleRequest>,
) -> Result<Json<SingleResponse<ArticleDto>>, Response> {
    // Check if slug is unique (excluding current article)
    if let Some(existing) = state.article_repo.find_by_slug(&req.slug).await? {
        if existing.id != req.id {
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorEnvelope {
                    error: AppError {
                        code: "slug_exists".into(),
                        message: "An article with this slug already exists".into(),
                        details: Some(serde_json::json!({
                            "fields": { "slug": ["Slug is already in use"] }
                        })),
                    },
                }),
            ).into_response());
        }
    }

    let article = state.article_repo.update(req).await?;

    Ok(Json(SingleResponse { data: article.into() }))
}
```

---

## Async Field Validation

Poodle provides the input-level async validation system. Underlay keeps the
backend and helper guidance, but form-level validity and slug behavior are now
app-owned above Poodle.

### Overview

The async validation system consists of:
- **Poodle `TextInput`** - Handles validation state, debouncing, and visual feedback
- **App-owned slug composition** - Slug generation, reserved-word checks, and uniqueness rules
- **Generic validation endpoints** - Backend validates field uniqueness and business rules
- **API client helpers** - Type-safe validation requests

### TextInput Async Validation

The `TextInput` component supports optional async validation with visual feedback:

```svelte
<script lang="ts">
  import { TextInput, type ValidationResult } from "@inflatable-cookie/poodle-svelte";

  async function validateEmail(email: string): Promise<ValidationResult> {
    // Call backend to check if email exists
    const response = await fetch("/api/validate-email", {
      method: "POST",
      body: JSON.stringify({ email })
    });

    const result = await response.json();

    return {
      valid: result.available,
      message: result.available
        ? "Email is available"
        : "Email is already registered"
    };
  }
</script>

<TextInput
  name="email"
  placeholder="you@example.com"
  validate={validateEmail}
  validationDebounce={300}
/>
```

#### ValidationResult Format

All async validators must return a `ValidationResult`:

```typescript
interface ValidationResult {
  valid: boolean;
  message?: string;    // User-friendly feedback
  suggestion?: string; // Optional alternative value
}
```

#### Validation Props

| Prop | Type | Default | Description |
|------|------|---------|-------------|
| `validate` | `(value: string, context?: unknown) => Promise<ValidationResult>` | `undefined` | Async validation function |
| `validationContext` | `unknown` | `undefined` | Context passed to validator (e.g., parent ID) |
| `validationDebounce` | `number` | `300` | Debounce delay in milliseconds |
| `showValidationStatus` | `boolean` | `true` (if validate provided) | Show status icon and message |
| `validateOnBlur` | `boolean` | `true` | Also validate on blur for immediate feedback |

#### Visual Feedback

TextInput shows real-time validation status:

- **Validating**: Gray spinner icon while checking
- **Valid**: Green checkmark icon with success message
- **Invalid**: Red alert icon with error message

Status icons appear on the right side of the input field. Messages appear below the field.

#### Validation Context

Use `validationContext` to pass additional data to the validator:

```svelte
<script lang="ts">
  let selectedModuleId = $state("module-123");

  async function validateLabel(label: string) {
    // Context is available in the validator
    return await api.validateField({
      entity: "section",
      field: "label",
      value: label,
      context: { moduleId: selectedModuleId }
    });
  }
</script>

<TextInput
  name="label"
  validate={validateLabel}
  validationContext={selectedModuleId}
/>
```

When `validationContext` changes, the field automatically revalidates.

### App-Owned Slug Composition

Underlay no longer exports a shared `SlugField` component. Build slug fields
with Poodle `Field` and `TextInput`, keep form gating app-owned, and use
shared slug helpers only where they are still useful.

```svelte
<script lang="ts">
  import {
    Field,
    TextInput,
    type InputValidationStatus,
    type ValidationResult
  } from "@inflatable-cookie/poodle-svelte";
  import { slugify, isReservedSlug, isValidSlugFormat } from "@inflatable-cookie/underlay/utils/slug";

  let titleValue = $state("");
  let slugValue = $state("");
  let lastAutoSlug = $state("");
  let slugStatus = $state<InputValidationStatus>("idle");
  let slugError = $state<string | null>(null);

  $effect(() => {
    const nextAutoSlug = slugify(titleValue);
    if (!slugValue.trim() || slugValue === lastAutoSlug) {
      slugValue = nextAutoSlug;
    }
    lastAutoSlug = nextAutoSlug;
  });

  async function validateSlug(slug: string): Promise<ValidationResult> {
    const normalized = slug.trim();

    if (normalized.length < 2) {
      return { valid: false, message: "Too short (min 2 characters)" };
    }

    if (!isValidSlugFormat(normalized, 64)) {
      return {
        valid: false,
        message: "Use lowercase letters, numbers, and hyphens only"
      };
    }

    if (isReservedSlug(normalized)) {
      return { valid: false, message: "This slug is reserved" };
    }

    const response = await fetch("/api/validate-slug", {
      method: "POST",
      body: JSON.stringify({ slug: normalized })
    });

    return await response.json();
  }

  function handleSlugBlur() {
    slugValue = slugify(slugValue);
  }
</script>

<TextInput
  id="title"
  name="title"
  value={titleValue}
  placeholder="Article Title"
  on:valueChange={(event) => {
    titleValue = event.detail.value;
  }}
/>

<Field
  id="slug"
  label="Slug"
  error={slugStatus === "invalid" ? slugError : null}
  validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
>
  <TextInput
    id="slug"
    name="slug"
    value={slugValue}
    autocomplete="off"
    required
    pattern="[a-z0-9]+(?:-[a-z0-9]+)*"
    maxLength={64}
    validate={validateSlug}
    validationDebounce={300}
    on:valueChange={(event) => {
      slugValue = event.detail.value;
    }}
    on:validationChange={(event) => {
      slugStatus = event.detail.status;
      slugError = event.detail.status === "invalid" ? event.detail.message || null : null;
    }}
    on:blur={handleSlugBlur}
  />
</Field>
```

Slug fields now follow this validation order in app code:

1. **Format check** (synchronous):
   - Minimum 2 characters
   - Maximum length
   - Pattern: `^[a-z0-9-]+$`

2. **Reserved check** (synchronous):
   - Rejects reserved slugs: `admin`, `api`, `new`, `edit`, `delete`, etc.

3. **Async validation** (if `validate` provided):
   - Uniqueness check via backend
   - Business rule validation

If any step fails, subsequent steps are skipped.

For the reusable cross-app recipe, see Poodle
[Slug Field Recipes](../../../poodle/docs/guides/007-slug-field-recipes.md).

### Backend Validation Endpoints

Create a generic validation endpoint that routes to entity/field-specific validators:

#### Payload Structure

```typescript
interface ValidateFieldPayload {
  entity: string;      // e.g., "user", "article", "module"
  field: string;       // e.g., "email", "slug", "label"
  value: string;       // The value to validate
  context?: {          // Optional context for validation
    [key: string]: any;
    excludeId?: string; // Exclude from uniqueness check
  };
}
```

#### Backend Implementation (Rust/Axum)

```rust
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateFieldPayload {
    pub entity: String,
    pub field: String,
    pub value: String,
    #[serde(default)]
    pub context: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion: Option<String>,
}

pub async fn validate_field(
    State(state): State<AppState>,
    Json(payload): Json<ValidateFieldPayload>,
) -> Result<Json<ValidationResult>, AppError> {
    let result = match (payload.entity.as_str(), payload.field.as_str()) {
        ("user", "email") => validate_user_email(&state, &payload).await?,
        ("article", "slug") => validate_article_slug(&state, &payload).await?,
        ("section", "label") => validate_section_label(&state, &payload).await?,
        _ => {
            return Err(AppError::BadRequest(
                format!("Unknown validation: {}.{}", payload.entity, payload.field)
            ));
        }
    };

    Ok(Json(result))
}

async fn validate_user_email(
    state: &AppState,
    payload: &ValidateFieldPayload,
) -> Result<ValidationResult, AppError> {
    let exclude_id = payload.context.get("excludeId")
        .and_then(|v| v.as_str());

    let exists = state.db
        .user_email_exists(&payload.value, exclude_id)
        .await?;

    Ok(ValidationResult {
        valid: !exists,
        message: Some(if exists {
            "Email is already registered".into()
        } else {
            "Email is available".into()
        }),
        suggestion: None,
    })
}

async fn validate_article_slug(
    state: &AppState,
    payload: &ValidateFieldPayload,
) -> Result<ValidationResult, AppError> {
    let exclude_id = payload.context.get("excludeId")
        .and_then(|v| v.as_str());

    let exists = state.db
        .article_slug_exists(&payload.value, exclude_id)
        .await?;

    Ok(ValidationResult {
        valid: !exists,
        message: Some(if exists {
            "Slug is already in use".into()
        } else {
            "Slug is available".into()
        }),
        suggestion: if exists {
            Some(format!("{}-{}", payload.value, rand::random::<u32>() % 1000))
        } else {
            None
        },
    })
}
```

#### Database Helpers

```rust
impl Database {
    pub async fn user_email_exists(
        &self,
        email: &str,
        exclude_id: Option<&str>,
    ) -> Result<bool, Error> {
        let query = if let Some(id) = exclude_id {
            sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1 AND user_id != $2)",
                email,
                id
            )
        } else {
            sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)",
                email
            )
        };

        Ok(query.fetch_one(&self.pool).await?.unwrap_or(false))
    }

    pub async fn article_slug_exists(
        &self,
        slug: &str,
        exclude_id: Option<&str>,
    ) -> Result<bool, Error> {
        let query = if let Some(id) = exclude_id {
            sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM articles WHERE slug = $1 AND article_id != $2)",
                slug,
                id
            )
        } else {
            sqlx::query_scalar!(
                "SELECT EXISTS(SELECT 1 FROM articles WHERE slug = $1)",
                slug
            )
        };

        Ok(query.fetch_one(&self.pool).await?.unwrap_or(false))
    }
}
```

### API Client Integration

Add validation helpers to your TypeScript API client:

```typescript
// In your API client (e.g., cattle-grid)
export interface ValidateFieldPayload {
  entity: string;
  field: string;
  value: string;
  context?: {
    excludeId?: string;
    [key: string]: any;
  };
}

export interface ValidationResult {
  valid: boolean;
  message?: string;
  suggestion?: string;
}

export async function validateField(
  payload: ValidateFieldPayload,
  fetchFn: typeof fetch,
  authToken: string
): Promise<ValidationResult> {
  const response = await fetchFn("/api/admin/validate-field", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${authToken}`
    },
    body: JSON.stringify(payload)
  });

  if (!response.ok) {
    throw new Error("Validation request failed");
  }

  return await response.json();
}
```

### Form Integration Examples

#### Basic Field Validation

```svelte
<script lang="ts">
  import { TextInput, type ValidationResult } from "@inflatable-cookie/poodle-svelte";
  import { api } from "$lib/api-client";

  export let data;  // { authToken }

  async function validateEmail(email: string): Promise<ValidationResult> {
    return await api.validateField(
      {
        entity: "user",
        field: "email",
        value: email
      },
      window.fetch.bind(window),
      data.authToken
    );
  }
</script>

<form method="post">
  <TextInput
    name="email"
    type="email"
    placeholder="you@example.com"
    required
    validate={validateEmail}
  />

  <button type="submit">Register</button>
</form>
```

#### Edit Mode with Exclusion

```svelte
<script lang="ts">
  import { TextInput } from "@inflatable-cookie/poodle-svelte";
  import { api } from "$lib/api-client";

  export let data;  // { user, authToken }

  async function validateEmail(email: string) {
    return await api.validateField(
      {
        entity: "user",
        field: "email",
        value: email,
        context: {
          excludeId: data.user.userId  // Exclude current user from check
        }
      },
      window.fetch.bind(window),
      data.authToken
    );
  }
</script>

<form method="post">
  <TextInput
    name="email"
    value={data.user.email}
    validate={validateEmail}
  />

  <button type="submit">Update</button>
</form>
```

#### Context-Dependent Validation

```svelte
<script lang="ts">
  import { TextInput } from "@inflatable-cookie/poodle-svelte";
  import { api } from "$lib/api-client";

  export let data;  // { authToken }

  let selectedModuleId = $state<string | undefined>(undefined);

  async function validateLabel(label: string) {
    if (!selectedModuleId) {
      return {
        valid: false,
        message: "Please select a module first"
      };
    }

    return await api.validateField(
      {
        entity: "section",
        field: "label",
        value: label,
        context: { moduleId: selectedModuleId }
      },
      window.fetch.bind(window),
      data.authToken
    );
  }
</script>

<form method="post">
  <!-- Module selector that updates selectedModuleId -->
  <select bind:value={selectedModuleId}>
    <option value="">Select module...</option>
    <option value="mod-1">Module 1</option>
    <option value="mod-2">Module 2</option>
  </select>

  <!-- Label validates within selected module -->
  <TextInput
    name="label"
    placeholder="A, B, C..."
    validate={validateLabel}
    validationContext={selectedModuleId}
  />

  <button type="submit">Create Section</button>
</form>
```

When `selectedModuleId` changes, the field automatically revalidates.

#### Reactive Validation Context

Use `$derived` to create reactive validation contexts:

```svelte
<script lang="ts">
  import { Field, TextInput, type InputValidationStatus } from "@inflatable-cookie/poodle-svelte";
  import { slugify } from "@inflatable-cookie/underlay/utils/slug";
  import { api } from "$lib/api-client";

  export let data;  // { module, authToken }
  export let form;  // Form data from +page.server.ts

  // Reactive values
  let formValues = $derived(form?.values ?? {});
  let startYearValue = $derived(
    typeof formValues?.startYear === "number"
      ? formValues.startYear
      : data.module.startYear
  );

  async function validateSlug(slug: string) {
    return await api.validateField(
      {
        entity: "module",
        field: "slug",
        value: slug,
        context: {
          pathwayId: data.module.pathwayId,
          startYear: startYearValue,  // Use reactive value
          excludeId: data.module.moduleId
        }
      },
      window.fetch.bind(window),
      data.authToken
    );
  }

  let slugStatus = $state<InputValidationStatus>("idle");
  let slugError = $state<string | null>(null);
</script>

<form method="post">
  <input
    type="number"
    name="startYear"
    value={startYearValue}
  />

  <Field
    id="slug"
    label="Slug"
    error={slugStatus === "invalid" ? slugError : null}
    validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
  >
    <TextInput
      id="slug"
      name="slug"
      value={data.module.slug}
      validate={validateSlug}
      validationContext={`${data.module.pathwayId}:${startYearValue}`}
      on:validationChange={(event) => {
        slugStatus = event.detail.status;
        slugError = event.detail.status === "invalid" ? event.detail.message || null : null;
      }}
      on:blur={() => {
        form.values.slug = slugify(form.values.slug);
      }}
    />
  </Field>

  <button type="submit">Update</button>
</form>
```

### Common Patterns

#### Pattern 1: Simple Uniqueness Check

```svelte
<script lang="ts">
  async function validateUsername(username: string) {
    return await api.validateField({
      entity: "user",
      field: "username",
      value: username
    }, fetch, authToken);
  }
</script>

<TextInput
  name="username"
  validate={validateUsername}
/>
```

#### Pattern 2: Scoped Uniqueness (Within Parent)

```svelte
<script lang="ts">
  let categoryId = $state("cat-123");

  async function validateSlug(slug: string) {
    return await api.validateField({
      entity: "product",
      field: "slug",
      value: slug,
      context: { categoryId }  // Unique within category
    }, fetch, authToken);
  }
</script>

<TextInput
  id="slug"
  name="slug"
  validate={validateSlug}
  validationContext={categoryId}
/>
```

#### Pattern 3: Composite Context

```svelte
<script lang="ts">
  let year = $state(2024);
  let term = $state("spring");

  async function validateCode(code: string) {
    return await api.validateField({
      entity: "course",
      field: "code",
      value: code,
      context: { year, term }  // Unique within year + term
    }, fetch, authToken);
  }
</script>

<TextInput
  name="code"
  validate={validateCode}
  validationContext={`${year}-${term}`}
/>
```

### Troubleshooting

#### Field Loses Focus During Validation

**Problem**: Input field loses focus when validation status changes.

**Cause**: Template condition switches branches, destroying and recreating the input element.

**Solution**: Ensure the wrapper element is always rendered when validation is enabled. TextInput handles this internally by checking `showValidationStatus` instead of `showValidationIcon`:

```svelte
<!-- ✅ CORRECT: TextInput internal implementation -->
{#if search || showValidationStatus}
  <div class="wrapper">
    <input />
    {#if showValidationIcon}
      <!-- Icon appears/disappears without destroying input -->
    {/if}
  </div>
{/if}

<!-- ❌ WRONG: Would destroy input when icon appears -->
{#if search || showValidationIcon}
  <div class="wrapper">
    <input />
  </div>
{/if}
```

If implementing custom validated inputs, always check if validation is **enabled**, not if validation is **active**.

#### Validation Doesn't Revalidate

**Problem**: Field doesn't revalidate when context changes.

**Cause**: `validationContext` prop not updated or not passed.

**Solution**: Ensure validation context is reactive:

```svelte
<!-- ❌ WRONG: Static context -->
<TextInput
  validate={validateLabel}
  validationContext="static-value"
/>

<!-- ✅ CORRECT: Reactive context -->
<script>
  let moduleId = $state("mod-1");
</script>
<TextInput
  validate={validateLabel}
  validationContext={moduleId}
/>
```

#### Backend Validation Fails

**Problem**: Backend returns error instead of ValidationResult.

**Cause**: Missing context fields or incorrect entity/field name.

**Solution**: Add proper error handling in backend:

```rust
pub async fn validate_field(
    State(state): State<AppState>,
    Json(payload): Json<ValidateFieldPayload>,
) -> Result<Json<ValidationResult>, AppError> {
    // Log unknown validations for debugging
    let result = match (payload.entity.as_str(), payload.field.as_str()) {
        ("user", "email") => validate_user_email(&state, &payload).await?,
        // ... other cases
        (entity, field) => {
            tracing::warn!("Unknown validation: {}.{}", entity, field);
            return Err(AppError::BadRequest(
                format!("Validation not implemented: {}.{}", entity, field)
            ));
        }
    };

    Ok(Json(result))
}
```

### Performance Considerations

1. **Debouncing**: Default 300ms prevents excessive API calls
2. **Caching**: Consider caching validation results for repeated values
3. **Database indexes**: Add indexes on validated columns for fast lookups
4. **Rate limiting**: Protect validation endpoints from abuse

```rust
// Add index for fast slug lookups
CREATE INDEX idx_articles_slug ON articles(slug);

// Consider composite index for scoped uniqueness
CREATE INDEX idx_products_slug_category
ON products(category_id, slug);
```

### Security Considerations

1. **Don't leak information**: Validation messages should not reveal sensitive data
2. **Rate limiting**: Limit validation requests per user/IP
3. **Authentication**: Require auth token for validation endpoints
4. **Input sanitization**: Validate and sanitize all inputs

```rust
// ❌ WRONG: Reveals if email exists
if email_exists {
    return Ok(ValidationResult {
        valid: false,
        message: Some("User with this email already exists"),
        suggestion: None,
    });
}

// ✅ CORRECT: Generic message
if email_exists {
    return Ok(ValidationResult {
        valid: false,
        message: Some("Email is not available"),
        suggestion: None,
    });
}
```

### Form-Level Validation

Underlay no longer provides `FormValidationProvider` or section-level validation
registries. Form validity should be derived in app code from real field values
plus field-level validation status callbacks.

#### Recommended Pattern

```svelte
<script lang="ts">
  import { FormLayout } from "@inflatable-cookie/poodle-svelte";
  import { Button, Field, FieldSet, TextInput } from "@inflatable-cookie/poodle-svelte";
  import { slugify } from "@inflatable-cookie/underlay/utils/slug";

  let titleValue = $state("");
  let slugValue = $state("");
  let slugStatus = $state<"idle" | "validating" | "valid" | "invalid">("idle");
  let slugError = $state<string | null>(null);

  const isFormValid = $derived(
    titleValue.trim().length > 0 &&
    slugStatus !== "validating" &&
    slugStatus !== "invalid"
  );

  async function validateSlug(slug: string) {
    return await api.validateField({
      entity: "article",
      field: "slug",
      value: slug
    });
  }
</script>

<form method="post">
  <FieldSet legend="Details">
    <FormLayout columns={1}>
      <Field label="Title" required>
        <TextInput
          id="title"
          name="title"
          required
          value={titleValue}
          on:valueChange={(event) => {
            titleValue = event.detail.value;
            if (!slugValue.trim()) {
              slugValue = slugify(event.detail.value);
            }
          }}
        />
      </Field>

      <Field
        id="slug"
        label="Slug"
        error={slugStatus === "invalid" ? slugError : null}
        validationState={slugStatus === "validating" ? "pending" : slugStatus === "invalid" ? "invalid" : slugStatus === "valid" ? "valid" : "none"}
        required
      >
        <TextInput
          id="slug"
          name="slug"
          value={slugValue}
          validate={validateSlug}
          on:valueChange={(event) => {
            slugValue = event.detail.value;
          }}
          on:validationChange={(event) => {
            slugStatus = event.detail.status;
            slugError = event.detail.status === "invalid" ? event.detail.message || null : null;
          }}
        />
      </Field>
    </FormLayout>
  </FieldSet>

  <Button type="submit" variant="primary" disabled={!isFormValid}>
    Save changes
  </Button>
</form>
```

#### Practical Rules

- Keep form-level validity app-owned.
- Use `TextInput.validate`, `validationContext`, `validationDebounce`, and
  `validationChange` for async validation-aware text fields.
- Keep slug auto-generation and availability-sensitive submit gating in the form.
- Treat `validating` as non-submittable when that field matters to the current
  submit path.
- Prefer explicit derived validity over hidden registries so conditional submit
  rules stay obvious.

---

## Frontend Validation

### SvelteKit Form Validation

Use progressive enhancement with server-side validation as the source of truth.

### Client-Side Zod Validation

Underlay keeps `useValidatedForm()` in `@inflatable-cookie/underlay/runtime/forms` for
lightweight client-side orchestration, but consuming apps should own their
actual Zod schemas.

Install `zod` in the consuming app only if you use this surface:

```bash
bun add zod
```

```ts
import { useValidatedForm } from "@inflatable-cookie/underlay/runtime/forms";
import { z } from "zod";

const registerRequestSchema = z.object({
  email: z.string().trim().email("Invalid email address"),
  password: z.string().min(12, "Password must be at least 12 characters"),
  displayName: z.string().trim().min(1, "Display name is required").max(100).optional(),
});

const form = useValidatedForm({
  schema: registerRequestSchema,
  initialValues: {
    email: "",
    password: "",
    displayName: "",
  },
  onSubmit: async (values) => {
    await api.auth.register(values);
  },
});
```

This surface is for UX only:

- server validation remains authoritative
- shared schemas should cover stable generic rules
- app-specific business rules stay in the consuming app unless they become broadly reusable
- expect a modest client bundle increase from `zod`; keep shared schemas focused on broadly reused contracts

Current scope decision for this batch:

- stop the shared schema surface at stable generic primitives and auth-adjacent request shapes already proven reusable here
- defer wider schema expansion until a consuming app shows duplicated rules that are both stable and project-agnostic

Common mapping:

| Rust validation | Zod equivalent |
|---|---|
| email validator | `z.string().trim().email()` |
| length min/max | `z.string().min(...).max(...)` |
| slug regex | `z.string().regex(/^[a-z0-9-]+$/)` |
| optional field | `.optional()` |

In `apps/web/src/routes/register/+page.server.ts`:

```typescript
import type { Actions } from "./$types";
import { fail } from "@sveltejs/kit";
import { createClient } from "$lib/api/client";

export const actions: Actions = {
  default: async ({ request, fetch }) => {
    const formData = await request.formData();

    const email = String(formData.get("email") ?? "").trim();
    const password = String(formData.get("password") ?? "").trim();
    const displayName = String(formData.get("displayName") ?? "").trim();

    // Client-side validation (optional, for better UX)
    const errors: Record<string, string[]> = {};

    if (!email) {
      errors.email = ["Email is required"];
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
      errors.email = ["Invalid email address"];
    }

    if (!password) {
      errors.password = ["Password is required"];
    } else if (password.length < 8) {
      errors.password = ["Password must be at least 8 characters"];
    }

    if (!displayName) {
      errors.displayName = ["Display name is required"];
    }

    if (Object.keys(errors).length > 0) {
      return fail(400, {
        success: false as const,
        errors,
        values: { email, displayName }
      });
    }

    // Call backend API
    const client = createClient(fetch, null);

    try {
      const response = await client.auth.register({
        email,
        password,
        displayName
      });

      // Success - cookies set, redirect
      // ... (see session management guide)
    } catch (e) {
      // Parse backend validation errors
      if (e && typeof e === "object" && "status" in e && e.status === 422) {
        const apiError = e as {
          error?: {
            code?: string;
            message?: string;
            details?: { fields?: Record<string, string[]> };
          };
        };

        return fail(422, {
          success: false as const,
          errors: apiError.error?.details?.fields ?? {},
          message: apiError.error?.message,
          values: { email, displayName }
        });
      }

      // Generic error
      return fail(400, {
        success: false as const,
        errors: {},
        message: e instanceof Error ? e.message : "Registration failed",
        values: { email, displayName }
      });
    }
  }
};
```

### Form Component with Error Display

In `apps/web/src/routes/register/+page.svelte`:

```svelte
<script lang="ts">
  import type { ActionData } from "./$types";
  import { enhance } from "$app/forms";

  export let form: ActionData;

  let loading = false;

  // Helper to get field errors
  function getFieldErrors(field: string): string[] {
    return form?.errors?.[field] ?? [];
  }

  function hasFieldError(field: string): boolean {
    return getFieldErrors(field).length > 0;
  }
</script>

<h1>Register</h1>

{#if form?.message}
  <div class="error" role="alert">
    {form.message}
  </div>
{/if}

<form method="POST" use:enhance={() => {
  loading = true;
  return async ({ update }) => {
    await update();
    loading = false;
  };
}}>
  <div class="field" class:error={hasFieldError("email")}>
    <label for="email">Email</label>
    <input
      type="email"
      id="email"
      name="email"
      value={form?.values?.email ?? ""}
      required
      disabled={loading}
      aria-invalid={hasFieldError("email")}
      aria-describedby={hasFieldError("email") ? "email-error" : undefined}
    />
    {#if hasFieldError("email")}
      <div id="email-error" class="field-error" role="alert">
        {#each getFieldErrors("email") as error}
          <p>{error}</p>
        {/each}
      </div>
    {/if}
  </div>

  <div class="field" class:error={hasFieldError("password")}>
    <label for="password">Password</label>
    <input
      type="password"
      id="password"
      name="password"
      required
      disabled={loading}
      aria-invalid={hasFieldError("password")}
      aria-describedby={hasFieldError("password") ? "password-error" : undefined}
    />
    {#if hasFieldError("password")}
      <div id="password-error" class="field-error" role="alert">
        {#each getFieldErrors("password") as error}
          <p>{error}</p>
        {/each}
      </div>
    {/if}
    <small>Must be at least 8 characters</small>
  </div>

  <div class="field" class:error={hasFieldError("displayName")}>
    <label for="displayName">Display Name</label>
    <input
      type="text"
      id="displayName"
      name="displayName"
      value={form?.values?.displayName ?? ""}
      required
      disabled={loading}
      aria-invalid={hasFieldError("displayName")}
      aria-describedby={hasFieldError("displayName") ? "displayName-error" : undefined}
    />
    {#if hasFieldError("displayName")}
      <div id="displayName-error" class="field-error" role="alert">
        {#each getFieldErrors("displayName") as error}
          <p>{error}</p>
        {/each}
      </div>
    {/if}
  </div>

  <button type="submit" disabled={loading}>
    {loading ? "Registering..." : "Register"}
  </button>
</form>

<style>
  .field.error input {
    border-color: red;
  }

  .field-error {
    color: red;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .error[role="alert"] {
    background: #fee;
    border: 1px solid red;
    padding: 1rem;
    margin-bottom: 1rem;
    border-radius: 4px;
  }
</style>
```

### Client-Side Validation (Optional)

For immediate feedback without server round-trip:

```svelte
<script lang="ts">
  let email = "";
  let emailError = "";

  function validateEmail(value: string) {
    if (!value) {
      emailError = "Email is required";
    } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value)) {
      emailError = "Invalid email address";
    } else {
      emailError = "";
    }
  }

  $: validateEmail(email);
</script>

<div class="field" class:error={emailError}>
  <label for="email">Email</label>
  <input
    type="email"
    id="email"
    bind:value={email}
    on:blur={() => validateEmail(email)}
    aria-invalid={!!emailError}
  />
  {#if emailError}
    <div class="field-error">{emailError}</div>
  {/if}
</div>
```

---

## TypeScript API Client Validation

Update your API client to handle validation errors:

In `libs/client/src/utils/http-client.ts`:

```typescript
export interface ApiError extends Error {
  status: number;
  code: string;
  details?: {
    fields?: Record<string, string[]>;
  };
  requestId?: string;
}

async request<T>(path: string, method: string, body?: unknown): Promise<T> {
  const response = await this.fetchFn(url, { method, headers, body: JSON.stringify(body) });

  if (!response.ok) {
    const errorBody = await response.json().catch(() => null);

    const apiError = Object.assign(new Error(errorBody?.error?.message ?? "Request failed"), {
      status: response.status,
      code: errorBody?.error?.code ?? "unknown_error",
      details: errorBody?.error?.details,
      requestId: errorBody?.error?.requestId,
    }) as ApiError;

    throw apiError;
  }

  return await response.json();
}
```

Usage in form action:

```typescript
try {
  await client.users.create(userData);
} catch (e) {
  if ((e as ApiError).status === 422) {
    const validationError = e as ApiError;
    return fail(422, {
      success: false,
      errors: validationError.details?.fields ?? {},
      message: validationError.message,
    });
  }

  throw e;
}
```

---

## Validation Patterns

### 1. Defensive Validation

Always validate on the backend, even if frontend validates:

```rust
// ✅ CORRECT - validate on backend
pub async fn create_user(
    Validated(req): Validated<CreateUserRequest>,
) -> Result<...> {
    // req is validated
}

// ❌ WRONG - trusting client data
pub async fn create_user(
    Json(req): Json<CreateUserRequest>,  // No validation!
) -> Result<...> {
    // req could be invalid
}
```

### 2. Early Validation

Validate as early as possible (at the boundary):

```rust
// ✅ CORRECT - validate in extractor
Validated(req): Validated<CreateUserRequest>

// ⚠️  OK - manual validation at start of handler
let req = req.validate().map_err(validation_error_response)?;

// ❌ WRONG - validating deep in business logic
async fn create_user_impl(req: CreateUserRequest) {
    req.validate()?;  // Too late!
    // ...
}
```

### 3. Consistent Error Format

Use the same error envelope for all validation errors:

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Request validation failed",
    "details": {
      "fields": {
        "email": ["Invalid email address"],
        "password": ["Password must be at least 8 characters"]
      }
    }
  }
}
```

### 4. User-Friendly Messages

Provide clear, actionable error messages:

```rust
// ❌ WRONG - technical message
#[validate(length(min = 8))]
// Error: "length"

// ✅ CORRECT - user-friendly message
#[validate(length(min = 8, message = "Password must be at least 8 characters"))]
// Error: "Password must be at least 8 characters"
```

---

## Security Considerations

### 1. Don't Leak Information

Be careful not to leak sensitive information in validation errors:

```rust
// ❌ WRONG - reveals if email exists
if user_exists {
    return Err("User with this email already exists");
}

// ✅ CORRECT - generic message for security
if user_exists {
    return Err("Unable to create account. Please try a different email.");
}
```

### 2. Rate Limiting

Apply rate limiting to validation-heavy endpoints to prevent abuse:

```rust
// In your middleware or handler
if !state.rate_limiter.check_allowed(ip_address, "register", 5, 300).await? {
    return Err(AuthError::RateLimited("Too many registration attempts".into()));
}
```

### 3. Input Sanitization

Sanitize user input to prevent injection attacks:

```rust
use ammonia::clean;

pub fn sanitize_html(input: &str) -> String {
    clean(input)
}

// In handler
let safe_bio = sanitize_html(&req.bio);
```

### 4. Validate Size Limits

Limit payload sizes to prevent DoS:

```rust
use tower_http::limit::RequestBodyLimitLayer;

let app = Router::new()
    .route("/upload", post(upload_handler))
    .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)); // 10 MB
```

---

## Testing

### Test Validation Rules

```rust
#[test]
fn test_create_user_validation() {
    let valid = CreateUserRequest {
        email: "user@example.com".into(),
        password: "password123".into(),
        display_name: "John Doe".into(),
        username: Some("john_doe".into()),
    };
    assert!(valid.validate().is_ok());

    let invalid_email = CreateUserRequest {
        email: "not-an-email".into(),
        ..valid.clone()
    };
    assert!(invalid_email.validate().is_err());

    let short_password = CreateUserRequest {
        password: "short".into(),
        ..valid.clone()
    };
    assert!(short_password.validate().is_err());
}
```

### Test Error Response Format

```rust
#[tokio::test]
async fn test_validation_error_response() {
    let app = test_app();

    let response = app
        .post("/users")
        .json(&serde_json::json!({
            "email": "invalid",
            "password": "short",
            "displayName": ""
        }))
        .send()
        .await;

    assert_eq!(response.status(), 422);

    let body: ErrorEnvelope = response.json().await;
    assert_eq!(body.error.code, "validation_failed");

    let fields = body.error.details["fields"].as_object().unwrap();
    assert!(fields.contains_key("email"));
    assert!(fields.contains_key("password"));
    assert!(fields.contains_key("displayName"));
}
```

---

## Underlay Validation (`underlay-validation`)

Underlay provides its own validation crate with a derive macro, built-in validators, and Axum integration. This is the recommended approach for new Underlay projects.

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
underlay-validation = { path = "../underlay/rust/crates/underlay-validation", features = ["derive", "axum"] }
```

#### Features

| Feature | Description |
|---------|-------------|
| `derive` | Enables `#[derive(Validate)]` macro (default) |
| `axum` | Enables `ValidatedJson` extractor for Axum |

### Basic Usage with Derive Macro

```rust
use underlay_validation::Validate;

#[derive(Validate)]
struct CreateUserRequest {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 100))]
    password: String,

    #[validate(range(min = 18, max = 120))]
    age: i32,

    #[validate(required)]
    name: String,
}
```

### Available Validators

#### Simple Validators (No Arguments)

| Validator | Description | Example |
|-----------|-------------|---------|
| `email` | Valid email address | `#[validate(email)]` |
| `url` | Valid HTTP/HTTPS URL | `#[validate(url)]` |
| `uuid` | Valid UUID (hyphenated or not) | `#[validate(uuid)]` |
| `required` | Non-empty string (trims whitespace) | `#[validate(required)]` |
| `alphanumeric` | Letters and numbers only | `#[validate(alphanumeric)]` |
| `username` | Letters, numbers, underscores, hyphens | `#[validate(username)]` |
| `slug` | Lowercase letters, numbers, hyphens | `#[validate(slug)]` |
| `positive` | Greater than zero | `#[validate(positive)]` |
| `non_negative` | Zero or greater | `#[validate(non_negative)]` |
| `not_empty` | Collection has at least one item | `#[validate(not_empty)]` |
| `nested` | Validate nested struct | `#[validate(nested)]` |
| `skip` | Skip validation for field | `#[validate(skip)]` |

#### Parameterized Validators

| Validator | Description | Example |
|-----------|-------------|---------|
| `length(min, max)` | String length bounds | `#[validate(length(min = 1, max = 100))]` |
| `range(min, max)` | Numeric range bounds | `#[validate(range(min = 0, max = 100))]` |
| `collection_length(min, max)` | Collection size bounds | `#[validate(collection_length(min = 1, max = 10))]` |
| `pattern = "regex"` | Custom regex pattern | `#[validate(pattern = r"^[A-Z]{2}\d{4}$")]` |
| `custom = "fn"` | Custom validator function | `#[validate(custom = "validate_custom")]` |

### Complete Example

```rust
use underlay_validation::Validate;

#[derive(Validate)]
struct CreateArticleRequest {
    #[validate(required, length(min = 1, max = 200))]
    title: String,

    #[validate(slug)]
    slug: String,

    #[validate(length(min = 10))]
    content: String,

    #[validate(range(min = 1, max = 5))]
    rating: i32,

    #[validate(url)]
    website: Option<String>,  // Optional fields skip validation if None

    #[validate(not_empty, collection_length(max = 10))]
    tags: Vec<String>,

    #[validate(nested)]
    author: AuthorInfo,

    #[validate(custom = "validate_publish_date")]
    publish_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Validate)]
struct AuthorInfo {
    #[validate(required)]
    name: String,

    #[validate(email)]
    email: String,
}

// Custom validator function
fn validate_publish_date(
    date: &Option<chrono::DateTime<chrono::Utc>>
) -> Result<(), underlay_validation::FieldError> {
    if let Some(d) = date {
        if *d < chrono::Utc::now() {
            return Err(underlay_validation::FieldError::with_code(
                "Publish date must be in the future",
                "publish_date.past"
            ));
        }
    }
    Ok(())
}
```

### Axum Integration with ValidatedJson

The `ValidatedJson` extractor automatically validates request bodies and returns proper HTTP error responses:

```rust
use axum::{Json, extract::State};
use underlay_validation::{Validate, ValidatedJson};

// Handler using ValidatedJson - validation is automatic!
pub async fn create_article(
    State(state): State<AppState>,
    ValidatedJson(req): ValidatedJson<CreateArticleRequest>,
) -> Json<SingleResponse<ArticleDto>> {
    // req is already validated here - no manual validation needed
    let article = state.article_repo.create(req).await?;
    Json(SingleResponse { data: article.into() })
}
```

When validation fails, `ValidatedJson` returns a `422 Unprocessable Entity` response:

```json
{
  "error": {
    "code": "validation_failed",
    "message": "Request validation failed",
    "details": {
      "fields": {
        "email": ["Invalid email address"],
        "password": ["Must be at least 8 characters"],
        "age": ["Must be at least 18"]
      }
    }
  }
}
```

### Manual Validation

You can also validate manually using the `Validate` trait:

```rust
use underlay_validation::{Validate, ValidationError, validators};

// Using derive macro
let req = CreateUserRequest { /* ... */ };
if let Err(errors) = req.validate() {
    // Handle validation errors
    for (field, messages) in errors.field_errors() {
        println!("{}: {:?}", field, messages);
    }
}

// Or implement manually for complex logic
impl Validate for CustomRequest {
    fn validate(&self) -> underlay_validation::ValidationResult<()> {
        let mut errors = ValidationError::new();

        if let Err(e) = validators::email(&self.email) {
            errors.add_field("email", e);
        }

        if let Err(e) = validators::length(&self.password, Some(8), Some(100)) {
            errors.add_field("password", e);
        }

        // Cross-field validation
        if self.password == self.email {
            errors.add_field("password", 
                underlay_validation::FieldError::new("Password cannot be the same as email"));
        }

        errors.into_result()
    }
}
```

### Using Validators Directly

All validators are available as standalone functions:

```rust
use underlay_validation::validators;

// Email validation
assert!(validators::email("user@example.com").is_ok());
assert!(validators::email("not-an-email").is_err());

// Length validation
assert!(validators::length("hello", Some(1), Some(10)).is_ok());
assert!(validators::length("hi", Some(5), None).is_err());

// Range validation
assert!(validators::range(25, Some(18), Some(120)).is_ok());
assert!(validators::range(10, Some(18), None).is_err());

// Pattern validation
assert!(validators::pattern("ABC123", r"^[A-Z]+\d+$", "Invalid format").is_ok());

// One-of validation
assert!(validators::one_of("active", &["active", "inactive"]).is_ok());
```

### Comparing with `validator` Crate

| Feature | `underlay-validation` | `validator` crate |
|---------|----------------------|-------------------|
| **Derive macro** | ✅ `#[derive(Validate)]` | ✅ `#[derive(Validate)]` |
| **Axum integration** | ✅ `ValidatedJson` built-in | ❌ Manual implementation |
| **Error format** | ✅ Matches Underlay error envelope | ❌ Custom format |
| **Built-in validators** | ✅ Common web validators | ✅ More validators |
| **Custom validators** | ✅ `#[validate(custom = "fn")]` | ✅ `#[validate(custom(...))]` |
| **Nested validation** | ✅ `#[validate(nested)]` | ✅ `#[validate]` on nested |
| **HTTP response** | ✅ Auto 422 with field errors | ❌ Manual response |

**When to use `underlay-validation`:**
- New Underlay projects
- Want automatic Axum integration
- Want error format matching Underlay error envelope

**When to use `validator` crate:**
- Existing projects already using it
- Need validators not in underlay-validation
- Non-Axum projects

### Migrating from `validator` Crate

The syntax is similar, so migration is straightforward:

```rust
// Before (validator crate)
use validator::Validate;

#[derive(Validate)]
struct Request {
    #[validate(email(message = "Invalid email"))]
    email: String,

    #[validate(length(min = 8))]
    password: String,

    #[validate]  // nested
    profile: Profile,
}

// After (underlay-validation)
use underlay_validation::Validate;

#[derive(Validate)]
struct Request {
    #[validate(email)]  // message is built-in
    email: String,

    #[validate(length(min = 8))]
    password: String,

    #[validate(nested)]  // explicit nested keyword
    profile: Profile,
}
```

Key differences:
- Use `#[validate(nested)]` instead of bare `#[validate]` for nested structs
- Error messages are built-in (can't be customized per-field yet)
- Use `ValidatedJson` instead of custom extractor

---

## Best Practices Summary

1. **Always validate on the backend** - Never trust client input
2. **Use type-safe validation** - `underlay-validation` or `validator` crate with derive macros
3. **Return structured errors** - Field-level errors with user-friendly messages
4. **Validate early** - At the request boundary (extractors)
5. **Test validation** - Unit test all validation rules
6. **Progressive enhancement** - Server validation required, client validation for UX
7. **Consistent error format** - Use error envelope everywhere
8. **Security first** - Don't leak information, rate limit, sanitize input

---

## Next Steps

- [070-api-handlers](./070-api-handlers.md) - Complete API patterns
- [100-frontend-web](./100-frontend-web.md) - Frontend integration

## See Also

**Related Guides:**
- **[065-session-management.md](./065-session-management.md)** - Login form validation, error handling
- **Poodle form guides** - Generic field, input, and validation-state UI composition
- **[100-frontend-web.md](./100-frontend-web.md)** - Complete form examples with validation
- **[070-api-handlers.md](./070-api-handlers.md)** - Backend validation in API handlers

**Key Patterns:**
- Backend: Validate early, return 422 with field errors
- Frontend: Display field errors inline, preserve form state
- Both layers: Validate - never trust client-side validation alone
- Error envelope: Use consistent `{ message, field }` structure

---

## Reference Implementation

See your project for complete examples:
- Backend validation: Check API request types in your api crates
- Frontend validation: `web/src/routes/register/+page.server.ts`
- Error handling: API error responses throughout the codebase

See Underlay source for implementation details:
- Validation crate: `underlay/rust/crates/underlay-validation/src/`
- Derive macro: `underlay/rust/crates/underlay-validation-derive/src/`
- Validators: `underlay/rust/crates/underlay-validation/src/validators.rs`
