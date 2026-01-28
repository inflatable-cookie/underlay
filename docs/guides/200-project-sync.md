# 200 - Project Sync Guide

This guide provides a reproducible checklist for bringing a project up to current Underlay patterns. Use it when:

- Setting up a new project using Underlay
- Updating an existing project to use new Underlay features
- Onboarding to an unfamiliar codebase that uses Underlay

## Prerequisites

- Underlay repository available (usually symlinked as `underlay/`)
- Rust backend using Axum
- SvelteKit frontend(s)
- TypeScript API client

---

## Phase 1: Dependencies

### Rust Backend (`Cargo.toml`)

```toml
[dependencies]
# Core Underlay crates
underlay-core = { path = "../underlay/rust/crates/underlay-core" }
underlay-db = { path = "../underlay/rust/crates/underlay-db" }
underlay-http = { path = "../underlay/rust/crates/underlay-http", features = ["validation", "nightfire"] }
underlay-auth = { path = "../underlay/rust/crates/underlay-auth" }
underlay-observability = { path = "../underlay/rust/crates/underlay-observability" }

# Optional crates based on features used
underlay-email = { path = "../underlay/rust/crates/underlay-email", features = ["templates"] }
underlay-nightfire = { path = "../underlay/rust/crates/underlay-nightfire" }
underlay-suggestions = { path = "../underlay/rust/crates/underlay-suggestions" }
underlay-metrics = { path = "../underlay/rust/crates/underlay-metrics" }
underlay-openapi = { path = "../underlay/rust/crates/underlay-openapi" }
```

### TypeScript Client (`package.json`)

```json
{
  "dependencies": {
    "@anthropic/underlay": "link:../underlay/ts"
  }
}
```

### Sync Migrations

```bash
cargo run --bin underlay-devtools -- sync-migrations --target ./crates/db/migrations
```

**Checklist:**
- [ ] Add underlay-core dependency
- [ ] Add underlay-db dependency
- [ ] Add underlay-http with required features
- [ ] Add underlay-auth dependency
- [ ] Add underlay-observability dependency
- [ ] Sync migrations from Underlay
- [ ] Run migrations on local database

---

## Phase 2: Response Patterns

Ensure all API handlers use consistent response shapes.

### Imports

```rust
use underlay_core::{AppError, ListResponse, SingleResponse};
use underlay_http::{ok, created, no_content, list_ok, error_response};
```

### Migration Checklist

- [ ] Replace manual `Json(...)` returns with `ok(data)` or `created(data)`
- [ ] Replace `(StatusCode::OK, Json(ListResponse { data }))` with `list_ok(data)`
- [ ] Replace `(StatusCode::NO_CONTENT, ())` with `no_content()`
- [ ] Use `error_response(StatusCode::..., AppError::new(...))` for errors

**Before:**
```rust
(StatusCode::OK, Json(SingleResponse { data: dto })).into_response()
```

**After:**
```rust
ok(dto).into_response()
```

---

## Phase 3: UUID Path Parsing

Replace manual UUID parsing in path handlers.

### Imports

```rust
use underlay_http::{parse_uuid_path, parse_uuid_path_raw};
```

### Migration Checklist

- [ ] Replace `user_id.parse::<Uuid>()` with `parse_uuid_path_raw(&user_id, "userId")?`
- [ ] Remove manual error handling for invalid UUIDs
- [ ] Use `parse_uuid_path()` when you need `underlay_core::Uuid`
- [ ] Use `parse_uuid_path_raw()` when you need `uuid::Uuid` for DB queries

**Before:**
```rust
let id: uuid::Uuid = match user_id.parse() {
    Ok(id) => id,
    Err(_) => {
        return error_response(
            StatusCode::BAD_REQUEST,
            AppError::new("validation.invalid_id", "Invalid user ID"),
        );
    }
};
```

**After:**
```rust
let id = parse_uuid_path_raw(&user_id, "userId")?;
```

---

## Phase 4: Database Existence Checks

Replace manual `SELECT EXISTS(...)` queries with `ExistsCheck`.

### Imports

```rust
use underlay_db::ExistsCheck;
```

### Migration Checklist

- [ ] Identify all existence check functions in `crates/db/src/*.rs`
- [ ] Replace with `ExistsCheck::new(schema, table)`
- [ ] Use `.value(column, value)` for string equality
- [ ] Use `.value_i32(column, value)` for integer equality
- [ ] Use `.scope(column, uuid)` for UUID foreign key scoping
- [ ] Use `.nullable_value(column, Option<i32>)` for nullable columns
- [ ] Use `.excluding(id)` for update operations
- [ ] Use `.include_deleted()` for tables without soft-delete

**Before:**
```rust
pub async fn slug_exists(pool: &DbPool, slug: &str, exclude_id: Option<Uuid>) -> Result<bool, sqlx::Error> {
    let exists = if let Some(id) = exclude_id {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM schema.table WHERE slug = $1 AND id != $2 AND deleted_at IS NULL) as "exists!""#,
            slug, id
        )
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM schema.table WHERE slug = $1 AND deleted_at IS NULL) as "exists!""#,
            slug
        )
        .fetch_one(pool)
        .await?
    };
    Ok(exists)
}
```

**After:**
```rust
pub async fn slug_exists(pool: &DbPool, slug: &str, exclude_id: Option<Uuid>) -> Result<bool, sqlx::Error> {
    let mut check = ExistsCheck::new("schema", "table").value("slug", slug);
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

### Composite Uniqueness

For fields unique within a scope (e.g., slug + pathway_id + year):

```rust
pub async fn module_slug_exists(
    pool: &DbPool,
    slug: &str,
    pathway_id: Uuid,
    start_year: i32,
    exclude_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    let mut check = ExistsCheck::new("learning", "module")
        .value("slug", slug)
        .scope("pathway_id", pathway_id)
        .value_i32("start_year", start_year);
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

### Tables Without Soft-Delete

```rust
pub async fn area_slug_exists(pool: &DbPool, slug: &str) -> Result<bool, sqlx::Error> {
    ExistsCheck::new("learning", "area")
        .value("slug", slug)
        .include_deleted()  // Skip deleted_at IS NULL filter
        .check(pool)
        .await
}
```

---

## Phase 5: Validation Patterns

### Validator Crate Integration

For request body validation using the `validator` crate:

```rust
use underlay_http::{validation_to_app_error, ValidateExt};
use validator::Validate;

// Option 1: Manual
if let Err(validation_err) = payload.validate() {
    let err = validation_to_app_error(&validation_err, "entity.invalid", "Validation failed.");
    return error_response(StatusCode::BAD_REQUEST, err).into_response();
}

// Option 2: Trait extension
payload.validate_or_error("entity.invalid")?;
```

### Live Field Validation

For endpoints that return validation results (always 200 OK):

```rust
use underlay_http::{ValidationResult, parse_uuid_for_validation, parse_optional_uuid_for_validation};

async fn validate_slug(Json(payload): Json<ValidatePayload>) -> impl IntoResponse {
    let exclude_id = match parse_optional_uuid_for_validation(payload.exclude_id.as_deref(), "excludeId") {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    if slug_exists(&payload.slug, exclude_id).await {
        return Json(ValidationResult::invalid("Slug already exists"));
    }

    Json(ValidationResult::valid())
}
```

### Nightfire Content Validation

For Nightfire structured content validation:

```rust
use underlay_http::nightfire_validation_to_app_error;
use nightfire::validate_nightfire_value_by_schema;

if let Err(validation_err) = validate_nightfire_value_by_schema(&body) {
    let err = nightfire_validation_to_app_error(
        validation_err,
        "content.invalid",
        "body",
        "Content body failed schema validation.",
    );
    return error_response(StatusCode::BAD_REQUEST, err).into_response();
}
```

**Checklist:**
- [ ] Migrate validator crate errors to `validation_to_app_error()`
- [ ] Migrate live validation endpoints to `ValidationResult`
- [ ] Migrate Nightfire validation to `nightfire_validation_to_app_error()`

---

## Phase 6: Frontend Patterns

### API Client Commands

Ensure commands follow the pattern:

```typescript
import { apiClient, type ApiResponse } from '@cattle-grid/client';
import type { Entity, CreateEntityPayload } from '@cattle-grid/types';

export function getEntities(): Promise<ApiResponse<Entity[]>> {
  return apiClient.get('/v1/admin/domain/entities');
}

export function getEntity(id: string): Promise<ApiResponse<Entity>> {
  return apiClient.get(`/v1/admin/domain/entities/${id}`);
}

export function createEntity(payload: CreateEntityPayload): Promise<ApiResponse<Entity>> {
  return apiClient.post('/v1/admin/domain/entities', payload);
}

export function updateEntity(id: string, payload: UpdateEntityPayload): Promise<ApiResponse<Entity>> {
  return apiClient.put(`/v1/admin/domain/entities/${id}`, payload);
}

export function deleteEntity(id: string): Promise<ApiResponse<void>> {
  return apiClient.delete(`/v1/admin/domain/entities/${id}`);
}
```

### SvelteKit Page Patterns

See [110-sveltekit-frontend.md](./110-sveltekit-frontend.md) for:
- Load functions
- Form actions
- Error handling
- Component usage

---

## Phase 7: Verification

After completing the sync:

- [ ] `cargo build` succeeds with no warnings
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] Frontend builds successfully
- [ ] API endpoints return correct response shapes
- [ ] Error responses include proper `error.code` and `error.message`
- [ ] Validation errors include `error.fieldErrors`

---

## Quick Reference

### Underlay HTTP Features

| Feature | Enables |
|---------|---------|
| `validation` | `validation_to_app_error()`, `ValidateExt` |
| `nightfire` | `nightfire_validation_to_app_error()` |
| `error-logging` | `append_error_log()`, `DbErrorLogSink` |
| `tracing` | Request span helpers |

### Common Imports

```rust
// Core types
use underlay_core::{AppError, ListResponse, SingleResponse};

// HTTP utilities
use underlay_http::{
    ok, created, no_content, list_ok, error_response,
    parse_uuid_path_raw, parse_uuid_for_validation,
    ValidationResult, validation_to_app_error,
    PaginationParams, Paginated,
};

// Database utilities
use underlay_db::ExistsCheck;

// Optional: Nightfire
use underlay_http::nightfire_validation_to_app_error;
```

---

## See Also

- [Patterns Catalogue](../patterns/000-index.md) - Quick reference for all patterns
- [050-database.md](./050-database.md) - Database patterns in depth
- [070-api-handlers.md](./070-api-handlers.md) - HTTP utilities in depth
- [110-sveltekit-frontend.md](./110-sveltekit-frontend.md) - Frontend patterns
