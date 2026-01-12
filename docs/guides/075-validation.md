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
- [100-frontend-bloom](./100-frontend-bloom.md) - SvelteKit basics

---

## Backend Validation

### Validation Library

We recommend using [`validator`](https://crates.io/crates/validator) for Rust validation.

Add to `apps/nursery/Cargo.toml`:

```toml
[workspace.dependencies]
validator = { version = "0.16", features = ["derive"] }
```

Add to your API crate `apps/nursery/crates/api/Cargo.toml`:

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

In `apps/nursery/crates/api/src/validation.rs`:

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

## Frontend Validation

### SvelteKit Form Validation

Use progressive enhancement with server-side validation as the source of truth.

In `apps/bloom/src/routes/register/+page.server.ts`:

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

In `apps/bloom/src/routes/register/+page.svelte`:

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

In `cattle-grid/src/utils/http-client.ts`:

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
- [100-frontend-bloom](./100-frontend-bloom.md) - Frontend integration

## See Also

**Related Guides:**
- **[065-session-management.md](./065-session-management.md)** - Login form validation, error handling
- **[090-ui-kit.md](./090-ui-kit.md)** - Form components (Field, TextInput, validation props)
- **[100-frontend-bloom.md](./100-frontend-bloom.md)** - Complete form examples with validation
- **[070-api-handlers.md](./070-api-handlers.md)** - Backend validation in API handlers

**Key Patterns:**
- Backend: Validate early, return 422 with field errors
- Frontend: Display field errors inline, preserve form state
- Both layers: Validate - never trust client-side validation alone
- Error envelope: Use consistent `{ message, field }` structure

---

## Reference Implementation

See Acowtancy for complete examples:
- Backend validation: Check API request types in farmyard crates
- Frontend validation: `cream/src/routes/register/+page.server.ts`
- Error handling: API error responses throughout the codebase

See Underlay source for implementation details:
- Validation crate: `underlay/rust/crates/underlay-validation/src/`
- Derive macro: `underlay/rust/crates/underlay-validation-derive/src/`
- Validators: `underlay/rust/crates/underlay-validation/src/validators.rs`
