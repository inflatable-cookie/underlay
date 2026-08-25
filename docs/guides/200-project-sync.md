# 200 - Project Sync Guide

This guide provides a reproducible checklist for bringing a project up to
current Underlay patterns. Use it when:

- Setting up a new project using Underlay
- Updating an existing project to use new Underlay features
- Onboarding to an unfamiliar codebase that uses Underlay

## Prerequisites

- One Git workspace with `apps/*`, `packages/*`, root `docs/`, and one root Bun
  manifest/lockfile
- Underlay and Poodle declared as released dependencies
- Rust backend using Axum
- SvelteKit frontend(s)
- TypeScript API client
- JSON naming policy reviewed: [071-json-naming.md](./071-json-naming.md)

## Before You Upgrade

Before taking a new Underlay feature or behavior change into an app:

1. Read [190-upgrade-compatibility.md](./190-upgrade-compatibility.md).
2. Find the subsystem-specific upgrade note or linked roadmap/log batch.
3. Copy the relevant checklist into the app repo issue or rollout task.
4. Use concrete dates for any deprecation or cutover plan.

---

## Phase 1: Dependencies

### Rust Backend (`Cargo.toml`)

```toml
[dependencies]
# Core Underlay crates
underlay-core = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-db = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-http = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4", features = ["validation", "nightfire"] }
underlay-auth = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-observability = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }

# Optional crates based on features used
underlay-email = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4", features = ["templates"] }
underlay-nightfire = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-suggestions = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-ai-runtime = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-metrics = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
underlay-openapi = { git = "ssh://git@github.com/inflatable-cookie/underlay.git", tag = "v0.9.4" }
```

### TypeScript Client (`packages/client/package.json`)

```json
{
  "dependencies": {
    "@inflatable-cookie/underlay": "git+ssh://git@github.com/inflatable-cookie/underlay.git#v0.9.4"
  }
}
```

Run the JavaScript install once from the workspace root and keep the generated
lockfile there:

```bash
effigy workspace:js:prepare
```

### Sync Migrations

```bash
cargo run --bin underlay-devtools -- sync-migrations --target ./apps/api/migrations
```

Use the API package's routed `migration:*` task for normal schema work. Keep
durable migrations under `apps/api/migrations/` and dev overlays under
`apps/api/migrations_dev/`; do not create a child install or lockfile for this
operation.

**Checklist:**
- [ ] Add underlay-core dependency
- [ ] Add underlay-db dependency
- [ ] Add underlay-http with required features
- [ ] Add underlay-auth dependency
- [ ] Add underlay-observability dependency
- [ ] Add underlay-ai-runtime dependency (if app uses AI provider routing)
- [ ] Sync migrations from Underlay
- [ ] Run migrations on local database

---

## Phase 1B: Configuration Standardization (Required)

Align with [120-configuration.md](./120-configuration.md) before expanding features.

### Migration Checklist

- [ ] Inventory env keys currently read by the app
- [ ] Classify keys as `secret`, `runtime-env`, or `app-behavior`
- [ ] Move `app-behavior` keys into typed Rust config structs
- [ ] Add committed defaults in `config/default.toml`
- [ ] Support optional local overrides with `config/local.toml` (gitignored)
- [ ] Restrict env overrides to an allowlist
- [ ] Add startup validation and redacted config diagnostics
- [ ] Add legacy-key deprecation warnings where needed
- [ ] Standardize frontend public API env keys to `PUBLIC_API_BASE_URL` + `PUBLIC_API_VERSION`
- [ ] Add CI guardrails with `scripts/check-env-usage-boundary.sh` and `scripts/check-env-manifest.sh` (see `docs/guides/121-consumer-config-rollout-kit.md`)
- [ ] Remove migrated keys from `config/env-manifest.txt` and docs after transition window

---

## Phase 1A: AI Runtime Routing (Optional)

If your app executes LLM-backed jobs, standardize on `underlay-ai-runtime`.

### Migration Checklist

- [ ] Move provider-agnostic request/response/error contracts to `underlay-ai-runtime`
- [ ] Use `LlmClient` trait + `ProviderRegistry` in job orchestration
- [ ] Use `OpenAiCompatibleClient` for OpenAI-style router services
- [ ] Keep app-specific action-key alias mapping in the app infra layer
- [ ] Keep provider secrets/config loading in app config layer (not in handlers/frontends)
- [ ] Enforce outbound host allowlist for non-local environments

---

## Phase 2: Response Patterns

Ensure all API handlers use consistent response shapes.

### Imports

```rust
use underlay_core::{ListResponse, SingleResponse};
use underlay_http::{ApiError, ApiResult, ok, created, no_content, list_ok};
```

### Migration Checklist

- [ ] Replace manual `Json(...)` returns with `ok(data)` or `created(data)`
- [ ] Replace `(StatusCode::OK, Json(ListResponse { data }))` with `list_ok(data)`
- [ ] Replace `(StatusCode::NO_CONTENT, ())` with `no_content()`
- [ ] Use `ApiResult<T>` and return `ApiError` for handler failures
- [ ] Ensure API request/response payload field names are `snake_case`
- [ ] Remove internal `#[serde(rename_all = "camelCase")]` from DTOs
- [ ] Keep `camelCase` serde naming only for documented external contract exceptions

**Before:**
```rust
(StatusCode::OK, Json(SingleResponse { data: dto })).into_response()
```

**After:**
```rust
ok(dto).into_response()
```

**Error handling (canonical):**

```rust
return Err(ApiError::bad_request(
    "validation.invalid_id",
    "Invalid user ID",
));
```

---

## Phase 2A: Database Error Diagnostics (SQLx)

Replace generic database failure messages with `underlay_db` diagnostics helpers.

### Imports

```rust
use underlay_db::{map_db_error, map_db_error_ref};
```

### Migration Checklist

- [ ] Replace `AppError::new("infra.db_error", "Database error ...")` where the source error is `sqlx::Error`
- [ ] For borrowed errors in handlers, use `map_db_error_ref("Database error ...", &err)`
- [ ] For owned errors in repositories/domain, use `map_db_error("Database error ...", err)`
- [ ] Keep `.with_context(...)` on `ApiError` so operation metadata remains queryable
- [ ] Keep `.with_cause(...)` for middleware error logging and traceability

**Before:**

```rust
Err(err) => {
    let app_err = AppError::new("infra.db_error", "Database error updating user");
    return Err(ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, app_err)
        .with_context(json!({ "operation": "users.update" })));
}
```

**After:**

```rust
Err(err) => {
    let app_err = map_db_error_ref("Database error updating user", &err);
    return Err(ApiError::internal(&app_err.code, &app_err.message)
        .with_cause(&err)
        .with_context(json!({ "operation": "users.update" })));
}
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
        return Err(ApiError::bad_request(
            "validation.invalid_id",
            "Invalid user ID",
        ));
    }
};
```

**After:**
```rust
let id = parse_uuid_path_raw(&user_id, "userId")?;
```

---

## Phase 4: Database Existence Checks

Replace manual `SELECT EXISTS(...)` queries with `TypedExistsCheck`.

### Imports

```rust
use underlay_db::TypedExistsCheck;
```

### Migration Checklist

- [ ] Identify all existence check functions in `crates/db/src/*.rs`
- [ ] Replace with `TypedExistsCheck::from_schema_table(schema, table)?`
- [ ] Use `.value(column, value)` for string equality
- [ ] Use `.value_i32(column, value)` for integer equality
- [ ] Use `.scope(column, uuid)` for UUID foreign key scoping
- [ ] Use `.nullable_value(column, Option<i32>)` for nullable columns
- [ ] Use `.excluding(id)` for update operations
- [ ] Use `.active_only()` for tables with `deleted_at` soft-delete semantics

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
    let mut check = TypedExistsCheck::from_schema_table("schema", "table")
        .map_err(|err| sqlx::Error::Protocol(format!("invalid table name: {err}")))?
        .value("slug", slug)
        .map_err(|err| sqlx::Error::Protocol(format!("invalid column name: {err}")))?
        .active_only();
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
    let mut check = TypedExistsCheck::from_schema_table("learning", "module")
        .map_err(|err| sqlx::Error::Protocol(format!("invalid table name: {err}")))?
        .value("slug", slug)
        .map_err(|err| sqlx::Error::Protocol(format!("invalid column name: {err}")))?
        .scope("pathway_id", pathway_id)
        .map_err(|err| sqlx::Error::Protocol(format!("invalid column name: {err}")))?
        .value_i32("start_year", start_year)
        .map_err(|err| sqlx::Error::Protocol(format!("invalid column name: {err}")))?
        .active_only();
    if let Some(id) = exclude_id {
        check = check.excluding(id);
    }
    check.check(pool).await
}
```

### Tables With Soft-Delete

```rust
pub async fn module_slug_exists(pool: &DbPool, slug: &str) -> Result<bool, sqlx::Error> {
    TypedExistsCheck::from_schema_table("learning", "module")
        .map_err(|err| sqlx::Error::Protocol(format!("invalid table name: {err}")))?
        .value("slug", slug)
        .map_err(|err| sqlx::Error::Protocol(format!("invalid column name: {err}")))?
        .active_only()
        .check(pool)
        .await
}
```

---

## Phase 5: Validation Patterns

### Validator Crate Integration

For request body validation using the `validator` crate:

```rust
use underlay_http::{ApiError, validation_to_app_error, ValidateExt};
use validator::Validate;

// Option 1: Manual
if let Err(validation_err) = payload.validate() {
    let err = validation_to_app_error(&validation_err, "entity.invalid", "Validation failed.");
    return Err(ApiError::new(StatusCode::BAD_REQUEST, err));
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
    return Err(ApiError::new(StatusCode::BAD_REQUEST, err));
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

See [100-frontend-web.md](./100-frontend-web.md) and [110-admin.md](./110-admin.md) for:
- Load functions
- Form actions
- Error handling
- Component usage

---

## Phase 6.5: Structure and Contracts Guardrails

Apply these guardrails before adding new feature code.

### Refactor Triggers and Budget

Mandatory refactor in the same PR when any trigger is hit:

- A changed file crosses warning threshold in `020-project-structure.md`.
- You are adding a second distinct workflow to an existing file.
- A page/route file now mixes orchestration with business rules.
- A single PR adds more than 350 lines to one file.

Refactor budget requirement:

- Reserve at least 15% of implementation effort for structure cleanup/splitting.
- If a temporary exception is needed, add a tracked follow-up task before merge.

### Cross-Workspace Contract Checklist

When changing backend behavior, do not merge until all impacted layers are aligned:

- [ ] API request/response shape updated and documented.
- [ ] TypeScript client types updated.
- [ ] Client command(s) updated for paths/query/envelope shape.
- [ ] UI load/submit/error states updated for new behavior.
- [ ] Recipe map/docs updated where implementation pattern changed.

### Naming Conventions by Layer

| Layer | Convention |
|---|---|
| Rust routes | `snake_case` modules split by domain and action (`list.rs`, `get.rs`, `mutations/create.rs`) |
| DB/query modules | `snake_case` grouped by domain concern (`learning/module_queries.rs`) |
| TS commands | domain-first folders with `queries.ts`, `mutations.ts`, `validation.ts` |
| TS types | `<domain>-types.ts` |
| Svelte components | `PascalCase.svelte` |
| Route folders/files | SvelteKit defaults (`+page.svelte`, `+page.ts`) with feature-local `_components`, `_state`, `_api` |

### State Management Rules (Svelte)

- Use component-local state for ephemeral UI state (dialog open, hover, draft input).
- Use URL query params for shareable/filter/sort/pagination state.
- Use feature-local store modules for state shared across sibling components in one route.
- Avoid global stores unless state is truly app-wide (auth/session/theme).
- Keep data fetch and transform logic out of leaf presentational components.

### Pattern Deviation (ADR Lite)

If you intentionally diverge from an Underlay recipe, add an ADR-lite note in your docs repo using this template:

```md
# ADR-Lite: <short-title>

## Context
What recipe/pattern was expected and why this case differs.

## Decision
What was implemented instead.

## Consequences
Tradeoffs, risks, and operational impact.

## Rollback Plan
How to return to standard pattern later, and trigger for doing it.
```

---

## Phase 7: Verification

After completing the sync:

- [ ] `cargo build` succeeds with no warnings
- [ ] `cargo clippy --all-targets --all-features -- -D warnings` passes
- [ ] `cargo test` passes
- [ ] `./scripts/check-route-error-patterns.sh crates/api/src/routes` passes
- [ ] Frontend builds successfully
- [ ] API endpoints return correct response shapes
- [ ] Error responses include proper `error.code` and `error.message`
- [ ] Validation errors include `error.fieldErrors`
- [ ] Cross-workspace contract checklist (Phase 6.5) completed for any backend/client/UI change
- [ ] No changed file exceeds hard limits in `020-project-structure.md`

---

## Phase 8: Documentation Sync Definition of Done

Use this section as the required closeout checklist after implementing or updating any pattern.

### Always Update

- [ ] `docs/patterns/000-index.md` includes the new or changed recipe and prompt.
- [ ] `docs/guides/README.md` reading order includes any new guide.
- [ ] Changed APIs/components are reflected in the relevant guide and recipe snippets.
- [ ] `docs/guides/190-upgrade-compatibility.md` or a linked upgrade note covers the consumer-facing change.
- [ ] The delivery log includes consumer upgrade notes when the batch changes behavior, APIs, config, migrations, or integration patterns.

### If Work Affects Admin Flows

- [ ] `docs/guides/180-admin-workflow-playbook.md` still matches the implementation sequence.
- [ ] `docs/guides/185-recipe-map-and-testing-matrix.md` still points to valid Acowtancy references.
- [ ] Minimum testing expectations in `docs/guides/185-recipe-map-and-testing-matrix.md` are met.

### If Work Affects Tooling/Runtime/Upgrades

- [ ] `docs/guides/190-upgrade-compatibility.md` is updated for any new constraints, versions, or breakage signals.
- [ ] Command examples use current tooling (`bun` for JS/TS repositories).

### Required Validation Before Merge

- [ ] All referenced file paths in docs exist.
- [ ] All internal guide links resolve.
- [ ] Sample commands run as written from one consuming workspace root.

---

## Quick Reference

### Underlay HTTP Features

| Feature | Enables |
|---------|---------|
| `validation` | `validation_to_app_error()`, `ValidateExt` |
| `nightfire` | `nightfire_validation_to_app_error()` |
| `error-logging` | middleware + DB logging for `ApiError` and legacy context headers |
| `tracing` | Request span helpers |

### Common Imports

```rust
// Core types
use underlay_core::{ListResponse, SingleResponse};

// HTTP utilities
use underlay_http::{
    ApiError, ApiResult, ok, created, no_content, list_ok,
    parse_uuid_path_raw, parse_uuid_for_validation,
    ValidationResult, validation_to_app_error,
    PaginationParams, Paginated,
};

// Database utilities
use underlay_db::{TypedExistsCheck, map_db_error, map_db_error_ref};

// Optional: Nightfire
use underlay_http::nightfire_validation_to_app_error;
```

---

## See Also

- [Patterns Catalogue](../patterns/000-index.md) - Quick reference for all patterns
- [050-database.md](./050-database.md) - Database patterns in depth
- [070-api-handlers.md](./070-api-handlers.md) - HTTP utilities in depth
- [100-frontend-web.md](./100-frontend-web.md) - Frontend patterns
- [110-admin.md](./110-admin.md) - Admin app shell + CRUD conventions
