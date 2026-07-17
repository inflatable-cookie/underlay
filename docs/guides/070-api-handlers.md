# 070 - API Handlers

> **Reference Implementation**: This guide includes patterns from a production application built with Underlay. These serve as working examples of best practices.

This document covers implementing HTTP handlers and routing using Axum.

The patterns here are intentionally simple and align with Underlay's primitives:

- API routes use the `/v1/...` prefix.
- Responses use the canonical envelopes from the transport and admin resource
  contracts.
- Errors use `underlay_http::ApiError` and `ApiResult<T>` as the canonical path.
- JSON field names use snake_case (see [071-json-naming.md](./071-json-naming.md)).

## Route Shape Policy

Underlay standard is canonical resource routes with typed `profile` query params for approved projections.

- List route: `GET /v1/{scope}/{domain}/{resource}`
- Detail route: `GET /v1/{scope}/{domain}/{resource}/{id}`
- List profiles: `profile=list|filter`
- Detail enrichment profile: `profile=details` (default detail without profile is base record)

Do not encode projection/mechanics in path names.

Disallowed route tokens:

- `/paginated`
- `with-counts`, `with-joins`
- `-for-list`, `-for-filter` (migration target is canonical routes + `profile`)

See [073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md) for full contract details.

## Handler Structure

For small-to-medium services, keep things inline in `main.rs`:

```
apps/api/crates/api/src/
├── main.rs        # Router + handlers + DTOs
└── state.rs       # AppState (optional)
```

As the API grows, split by domain:

```
apps/api/crates/api/src/
├── main.rs
├── state.rs
├── error.rs
└── handlers/
    ├── mod.rs
    ├── users.rs
    ├── artists.rs
    └── admin.rs
```

## Response Envelopes

Underlay defines two main shared response shapes:

```rust
use underlay_core::{ListResponse, SingleResponse};

// bounded helper list
ListResponse { data: vec![/* ... */] }

// single
SingleResponse { data: /* ... */ }
```

For admin page-shaped root lists and detail-tab child collections, use the
paged list envelope from
[073-api-profiles-and-query-contract.md](./073-api-profiles-and-query-contract.md)
and
[../contracts/115-admin-resource-api-shapes.md](../contracts/115-admin-resource-api-shapes.md):

```json
{
  "data": [],
  "total": 0,
  "has_more": false
}
```

Rule:

- use `ListResponse<T>` for bounded helper collections
- use the paged list envelope for `EntityListPage`-class admin browsing surfaces

## Errors

Underlay’s error envelope is:

```json
{
  "error": {
    "code": "resource.not_found",
    "message": "User not found",
    "field_errors": {
      "email": "Must be a valid email"
    }
  }
}
```

In Rust, return typed handler errors with `underlay_http::ApiError`:

```rust
use underlay_http::ApiError;

fn not_found(resource: &str) -> ApiError {
    ApiError::not_found("resource.not_found", format!("{resource} not found"))
}
```

### Do / Don't

- Do: return `ApiResult<T>` from handlers and construct failures with `ApiError`.
- Do: attach structured context via `.with_context(...)` on failures.
- Don't: return raw `StatusCode::...into_response()` for handler error branches.
- Don't: use `error_response(...)` as a primary route pattern in new code.

### Database Error Diagnostics (SQLx)

When mapping SQL failures to HTTP errors, prefer `underlay_db::map_db_error_ref(...)`.

This preserves operator-useful details (SQLSTATE, location, PostgreSQL detail/hint, suggested fix) while keeping the stable code `infra.db_error`.

```rust
use underlay_http::ApiError;

let row = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|err| {
        let db_err = underlay_db::map_db_error_ref("Database error loading user", &err);

        ApiError::internal(&db_err.code, &db_err.message)
            .with_cause(&err)
            .with_context(serde_json::json!({
                "operation": "users.get",
                "user_id": id,
            }))
    })?;
```

For domain/repository layers where you already own the `sqlx::Error`, use `underlay_db::map_db_error(...)` (owned variant).

```rust
let rows = query_fn(pool)
    .await
    .map_err(|err| underlay_db::map_db_error("Database error listing users", err))?;
```

### Lintable Rule for Route Modules

Use this check to find likely non-canonical error branches in handlers:

```bash
rg -n "StatusCode::[A-Z_]+\\s*\\.into_response\\(\\)" crates/api/src/routes
```

Expected migration target: zero matches for error branches in route modules.

For a reusable migration check, run:

```bash
./scripts/check-route-error-patterns.sh crates/api/src/routes
```

To also print compatibility-helper callsites:

```bash
./scripts/check-route-error-patterns.sh crates/api/src/routes --show-compat
```

### Migration Guidance

- `ApiResult<T>` and `ApiError` are the standard handler error path.
- Route modules should avoid compatibility helper patterns and return `ApiError` directly.

## AppState

Keep `AppState` minimal and explicit:

```rust
use std::sync::Arc;
use myapp_db::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub auth_provider: Arc<dyn underlay_auth::AuthProvider>,
}

impl underlay_auth::HasAuthProvider for AppState {
    fn auth_provider(&self) -> &dyn underlay_auth::AuthProvider {
        self.auth_provider.as_ref()
    }
}
```

## Example Handlers

These examples use `/v1/...` routes and the Underlay envelopes.

```rust
use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use underlay_core::{ListResponse, SingleResponse};
use underlay_http::{ApiError, ApiResult, list_ok, ok, parse_uuid_path_raw};

use crate::state::AppState;

#[derive(serde::Serialize)]
struct UserDto {
    user_id: String,
    email: String,
    created_at: String,
}

async fn list_users(State(state): State<AppState>) -> ApiResult<ListResponse<UserDto>> {
    let rows = sqlx::query!(
        r#"
        SELECT id::text as user_id, email, created_at
        FROM users
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|err| {
        ApiError::internal("db.query_failed", "Failed to list users")
            .with_cause(err.to_string())
            .with_context(serde_json::json!({"query": "users.list"}))
    })?;

    let data: Vec<UserDto> = rows
        .into_iter()
        .map(|r| UserDto {
            user_id: r.user_id,
            email: r.email,
            created_at: r.created_at.to_rfc3339(),
        })
        .collect();

    Ok(list_ok(data))
}

async fn get_user(State(state): State<AppState>, Path(user_id): Path<String>) -> ApiResult<SingleResponse<UserDto>> {
    let id = parse_uuid_path_raw(&user_id, "userId")
        .map_err(|_| ApiError::bad_request("validation.invalid_id", "Invalid user ID"))?;

    let row = sqlx::query!(
        r#"
        SELECT id::text as user_id, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|err| {
        ApiError::internal("db.query_failed", "Failed to load user")
            .with_cause(err.to_string())
            .with_context(serde_json::json!({ "user_id": id }))
    })?;

    let Some(row) = row else {
        return Err(ApiError::not_found("resource.not_found", "User not found")
            .with_context(serde_json::json!({ "user_id": id })));
    };

    let dto = UserDto {
        user_id: row.user_id,
        email: row.email,
        created_at: row.created_at.to_rfc3339(),
    };

    Ok(ok(dto))
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/v1/users", get(list_users))
        .route("/v1/users/:id", get(get_user))
        .with_state(state)
}
```

## Nightfire Persistence And Media Sync

When a route accepts Nightfire JSON and also maintains `media_usage`, keep the
server-side order explicit:

1. ensure stable block ids on the Rust side
2. persist the exact Nightfire JSON
3. run the shared media extractor and sync path against that same value

Copyable example:

- [`docs/guides/code/070-api-handlers/nightfire-persist-and-media-sync.rs`](/Users/tom/Dev/projects/underlay/docs/guides/code/070-api-handlers/nightfire-persist-and-media-sync.rs)

Important boundary rule:

- API DTO field names may still be `snake_case`
- inner Nightfire block `data` keys must already be in their final form
- do not rename keys like `imageId` during server-side mapping before
  extraction, or shared locator and media-field matching will drift from the
  stored JSON

## Production Patterns

### Error Logging

> **Quick Start**: Underlay provides production-ready error logging via the `underlay-http` crate's `error-logging` feature. For full documentation, see [rust/crates/underlay-http/ERROR_LOGGING.md](../../rust/crates/underlay-http/ERROR_LOGGING.md).

Log errors to a database for monitoring and debugging. This pattern is used in production applications.

**Benefits:**
- Centralized error tracking
- Correlation with request IDs
- Debug production issues
- Monitor error trends
- Query by time range, status code, or endpoint

#### Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
underlay-http = { path = "../../rust/crates/underlay-http", features = ["error-logging"] }
```

Sync migrations to your app:

```bash
cargo run --bin underlay-devtools -- sync-migrations --target ./migrations
```

Then run your migrations. This creates the `infra.error_log` table with indexes.

#### Basic Usage in Handlers

For handlers, prefer returning `ApiError` so context is automatically attached via headers:

```rust
use underlay_http::{ApiError, ApiResult};

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>
) -> ApiResult<SingleResponse<UserDto>> {
    let id: uuid::Uuid = match user_id.parse() {
        Ok(id) => id,
        Err(_) => {
            return Err(ApiError::bad_request("validation.invalid_id", "Invalid user ID")
                .with_context(serde_json::json!({ "userId": user_id })));
        }
    };
    
    // ... rest of handler
}
```

**Note**: Reserve direct `append_error_log()` calls for non-HTTP contexts (background jobs, workers). HTTP handlers should return `ApiError` and let middleware persist the log entry.

#### Querying Error Logs

Query recent errors for debugging:

```rust
use underlay_http::error_logging::{list_error_logs, ErrorLogFilters};
use chrono::{Utc, Duration};

// Get all 500 errors in the last hour
let filters = ErrorLogFilters {
    since: Some(Utc::now() - Duration::hours(1)),
    status_code: Some(500),
    limit: 50,
    ..Default::default()
};

let errors = list_error_logs(&pool, filters).await?;

for error in errors {
    println!("{}: {} - {}", 
        error.occurred_at, 
        error.endpoint, 
        error.message
    );
}
```

#### Advanced: Middleware-Based Logging

For automatic error logging across all endpoints, implement a Tower middleware layer (planned in Phase 8.3 task 3). The current implementation provides the building blocks:

- `append_error_log()` - Direct database insertion
- `DbErrorLogSink` - Implements `ErrorLogSink` trait
- `ErrorLogFilters` - Query builder for error logs

See [rust/crates/underlay-http/ERROR_LOGGING.md](../../rust/crates/underlay-http/ERROR_LOGGING.md) for:
- Complete API documentation
- Database schema details
- Best practices for correlation IDs
- Indexing and query optimization
- Future roadmap (Tower middleware, retention policies)

### Request Tracing

Use Underlay's observability layer for structured logging:

```rust
use underlay_observability::init_tracing;

#[tokio::main]
async fn main() {
    // Initialize tracing (pretty for dev, JSON for prod)
    init_tracing();

    // Handlers automatically get tracing context
    let app = Router::new()
        .route("/v1/users", get(list_users))
        .layer(underlay_observability::trace_layer());

    // ...
}

async fn list_users() -> impl IntoResponse {
    tracing::info!("Listing users");  // Includes request ID automatically
    // ...
}
```

### Rate Limiting (Optional)

Protect endpoints from abuse:

```rust
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};

pub fn create_router() -> Router {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10)
            .burst_size(20)
            .finish()
            .unwrap(),
    );

    Router::new()
        .route("/v1/auth/login", post(login))
        .layer(GovernorLayer {
            config: Box::leak(governor_conf),
        })
}
```

Add to `Cargo.toml`:
```toml
tower-governor = "0.1"
```

## API Version Header (Optional)

If you use date-based versioning (myapp-style), send a header like:

- `X-Api-Version: 2025-01-01`

Keep the URL stable (`/v1/...`). This makes it easier to evolve behavior without changing clients' base URLs.

## See Also

**Related Guides:**
- **[060-authentication.md](./060-authentication.md)** - Authentication setup for protected endpoints
- **[067-authorization.md](./067-authorization.md)** - Role-based access control in handlers
- **[075-validation.md](./075-validation.md)** - Request validation patterns
- **[130-testing.md](./130-testing.md)** - Testing API handlers and integration tests

**Key Patterns:**
- Use `SingleResponse` and `ListResponse` for consistency
- Handle errors with `AppError` and `error_response`
- Add request ID middleware for traceability
- Log errors to database for monitoring
- Test handlers with mock repositories

---

## Underlay HTTP Utilities (`underlay-http`)

The `underlay-http` crate provides HTTP utilities for building Axum-based APIs.

### Installation

```toml
[dependencies]
underlay-http = { path = "../underlay/rust/crates/underlay-http" }

# Optional features
underlay-http = { path = "...", features = ["tracing"] }
underlay-http = { path = "...", features = ["tracing", "opentelemetry"] }
underlay-observability = { path = "...", features = ["opentelemetry"] }
```

### Request Context

`RequestContext` provides unified access to common request metadata.

#### Basic Usage

```rust
use underlay_http::context::RequestContext;
use axum::Json;

async fn my_handler(ctx: RequestContext) -> Json<String> {
    println!("Request ID: {}", ctx.request_id());
    println!("User ID: {:?}", ctx.user_id());
    println!("IP: {:?}", ctx.ip_address());
    println!("User-Agent: {:?}", ctx.user_agent());
    
    Json("ok".to_string())
}
```

#### Available Methods

| Method | Return Type | Description |
|--------|-------------|-------------|
| `request_id()` | `&str` | Request ID (from header or generated UUID v7) |
| `user_id()` | `Option<Uuid>` | Authenticated user ID (from auth middleware) |
| `ip_address()` | `Option<IpAddr>` | Client IP (from CF-Connecting-IP, X-Real-IP, or X-Forwarded-For) |
| `user_agent()` | `Option<&str>` | User-Agent header value |
| `is_authenticated()` | `bool` | Whether a user ID is present |
| `trace_context()` | `Option<&TraceContext>` | Parsed incoming W3C trace context (`opentelemetry` feature only) |
| `trace_id()` | `Option<&str>` | Incoming trace ID from `traceparent` (`opentelemetry` feature only) |
| `parent_span_id()` | `Option<&str>` | Incoming parent span ID from `traceparent` (`opentelemetry` feature only) |
| `inject_trace_context(&mut HeaderMap)` | `()` | Write `traceparent` / `tracestate` to outgoing headers (`opentelemetry` feature only) |

#### Header Priority for IP Extraction

1. `CF-Connecting-IP` (Cloudflare)
2. `X-Real-IP` (nginx)
3. `X-Forwarded-For` (first IP in list)

#### Setting User ID from Auth Middleware

```rust
use underlay_http::context::AuthenticatedUser;
use axum::Extension;

// In your auth middleware, after validating the JWT:
async fn auth_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    // After validating JWT and extracting user_id...
    let user_id: Uuid = validate_jwt(&token)?;
    
    // Insert into extensions for RequestContext to pick up
    req.extensions_mut().insert(AuthenticatedUser(user_id));
    
    next.run(req).await
}
```

### Authenticated Context

Use `AuthenticatedContext` when an endpoint requires authentication:

```rust
use underlay_http::context::AuthenticatedContext;

async fn protected_handler(ctx: AuthenticatedContext) -> Json<String> {
    // Guaranteed to have a user ID - returns 401 if not authenticated
    let user_id = ctx.user_id();
    
    Json(format!("Hello, user {}", user_id))
}
```

#### Error Types

```rust
use underlay_http::context::ContextError;

// ContextError::Unauthenticated - Returns 401 Unauthorized
// ContextError::MissingField(field) - Returns 400 Bad Request
```

### Tracing Integration

Enable the `tracing` feature for structured logging support:

```toml
underlay-http = { path = "...", features = ["tracing"] }
```

```rust
use underlay_http::context::{RequestContext, make_request_span};

async fn my_handler(ctx: RequestContext) -> Json<String> {
    // Create a span with request context fields
    let span = make_request_span(&ctx);
    let _guard = span.enter();
    
    tracing::info!("Processing request");
    // Logs will include: request_id, user_id, ip
    
    Json("ok".to_string())
}
```

### Trace Context Propagation

Enable the `opentelemetry` feature when you want `RequestContext` and Underlay's request spans to understand incoming W3C trace headers without forcing exporter setup into the shared crate:

```toml
underlay-http = { path = "...", features = ["tracing", "opentelemetry"] }
underlay-observability = { path = "...", features = ["opentelemetry"] }
```

With that feature enabled:

- `RequestContext` parses inbound `traceparent` and `tracestate` headers.
- `underlay_observability::trace_layer()` records `trace_id`, `parent_span_id`, `trace_flags`, and `tracestate` on the request span when the headers are present.
- `make_request_span()` and `record_to_span()` include the same correlation fields for handler-owned spans.

```rust
use axum::{http::HeaderMap, Json};
use underlay_http::RequestContext;

async fn proxy_handler(ctx: RequestContext) -> Json<String> {
    let mut outgoing_headers = HeaderMap::new();

    // Forward the current trace context to a downstream HTTP client request.
    ctx.inject_trace_context(&mut outgoing_headers);

    tracing::info!(
        request_id = %ctx.request_id(),
        trace_id = ?ctx.trace_id(),
        "forwarding request"
    );

    Json("ok".to_string())
}
```

Keep OTLP exporter configuration, sampler choice, and backend credentials in the consuming app. Underlay only provides header parsing, propagation, and span-field correlation in this batch.

### Pagination

Standardized pagination for list endpoints.

#### Basic Usage

```rust
use underlay_http::pagination::{PaginationParams, Paginated};
use axum::extract::Query;

async fn list_users(
    Query(params): Query<PaginationParams>,
    db: DbPool,
) -> Json<Paginated<User>> {
    // Get total count
    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&db)
        .await?
        .unwrap_or(0) as i64;
    
    // Fetch page of data
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        params.limit as i64,
        params.offset() as i64
    )
    .fetch_all(&db)
    .await?;
    
    // Wrap in paginated response
    Ok(Json(params.wrap(users, total)))
}
```

#### Query Parameters

| Parameter | Type | Default | Max | Description |
|-----------|------|---------|-----|-------------|
| `page` | i32 | 1 | - | Page number (1-indexed) |
| `limit` | i32 | 20 | 100 | Items per page |

#### Response Format

```json
{
  "data": [...],
  "pagination": {
    "page": 1,
    "limit": 20,
    "total": 45,
    "total_pages": 3
  }
}
```

#### PaginationParams Methods

| Method | Description |
|--------|-------------|
| `offset()` | Calculate SQL OFFSET from page and limit |
| `limit_i64()` | Get limit as `i64` for SQLx binding |
| `offset_i64()` | Get offset as `i64` for SQLx binding |
| `with_max_limit(max)` | Clamp limit to a custom maximum |
| `clamped()` | Clamp limit to `DEFAULT_MAX_LIMIT` (100) |
| `sql_clause()` | Generate `"LIMIT 20 OFFSET 0"` string |
| `sql_clause_params(l, o)` | Generate `"LIMIT $l OFFSET $o"` placeholders |
| `wrap(data, total)` | Create `Paginated<T>` response |
| `wrap_i64(data, total)` | Same as `wrap` but accepts `i64` total |

#### SQL Helpers Example

```rust
use underlay_http::pagination::PaginationParams;
use axum::extract::Query;

async fn list_users(
    Query(params): Query<PaginationParams>,
    db: DbPool,
) -> Json<Paginated<User>> {
    // Use clamped() to enforce max limit
    let params = params.clamped();
    
    // Get total count (returns i64)
    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&db)
        .await?
        .unwrap_or(0);
    
    // Use limit_i64() and offset_i64() for binding
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC LIMIT $1 OFFSET $2",
        params.limit_i64(),
        params.offset_i64()
    )
    .fetch_all(&db)
    .await?;
    
    // Use wrap_i64() to handle i64 total directly
    Ok(Json(params.wrap_i64(users, total)))
}
```

#### Using SQL Clause Helpers

For raw SQL or custom query builders:

```rust
// Inline values (for trusted contexts only)
let clause = params.sql_clause();
// => "LIMIT 20 OFFSET 40"

// Parameterized (recommended for all queries)
let clause = params.sql_clause_params(3, 4);
// => "LIMIT $3 OFFSET $4"
// Then bind: params.limit_i64(), params.offset_i64()
```

### Response Helpers

Convenience functions for common HTTP responses:

```rust
use underlay_http::{ok, created, no_content, list_ok};

// 200 OK with JSON body
async fn get_user() -> impl IntoResponse {
    ok(user)
}

// 201 Created with JSON body
async fn create_user() -> impl IntoResponse {
    created(new_user)
}

// 204 No Content
async fn delete_user() -> impl IntoResponse {
    no_content()
}

// 200 OK with list (alias for ok)
async fn list_users() -> impl IntoResponse {
    list_ok(users)
}
```

### UUID Path Parameter Parsing

Parse and validate UUID path parameters with automatic error responses:

```rust
use underlay_http::parse_uuid_path;
use axum::extract::Path;

async fn get_user(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> impl IntoResponse {
    // Returns 400 Bad Request if invalid UUID
    let parsed = parse_uuid_path(&user_id, "userId")?;

    // parsed is underlay_core::Uuid for domain ID types
    // ...
}
```

For direct database calls that need `uuid::Uuid`:

```rust
use underlay_http::parse_uuid_path_raw;

async fn get_item(
    State(state): State<AppState>,
    Path(item_id): Path<String>,
) -> impl IntoResponse {
    // Returns uuid::Uuid for direct DB use
    let id = parse_uuid_path_raw(&item_id, "itemId")?;

    let row = sqlx::query!("SELECT * FROM items WHERE id = $1", id)
        .fetch_optional(&state.pool)
        .await?;
    // ...
}
```

Both helpers return `Result<Uuid, Response>` - on error, they return a properly formatted JSON error response:

```json
{
  "error": {
    "code": "validation.invalid_id",
    "message": "Invalid itemId; expected UUIDv7 string."
  }
}
```

### Live Field Validation

For real-time validation endpoints (e.g., checking slug availability as user types), use `ValidationResult` instead of HTTP errors:

```rust
use underlay_http::{ValidationResult, parse_uuid_for_validation};
use axum::{Json, response::IntoResponse};

async fn validate_slug(payload: Json<ValidatePayload>) -> impl IntoResponse {
    // Parse UUID with validation result (not HTTP error)
    let module_id = match parse_uuid_for_validation(&payload.module_id, "moduleId") {
        Ok(id) => id,
        Err(result) => return Json(result),
    };

    // Check business logic
    if slug_exists(&module_id, &payload.slug).await {
        return Json(ValidationResult::invalid("Slug already exists"));
    }

    // Optionally suggest alternatives
    if let Some(suggestion) = generate_unique_slug(&payload.slug).await {
        return Json(ValidationResult::invalid_with_suggestion(
            "Slug already exists",
            suggestion,
        ));
    }

    Json(ValidationResult::valid())
}
```

#### ValidationResult Methods

| Method | Description |
|--------|-------------|
| `valid()` | Create successful result |
| `invalid(message)` | Create failed result with message |
| `invalid_with_suggestion(message, suggestion)` | Failed with alternative suggestion |
| `with_suggestion(suggestion)` | Add suggestion to existing result |

#### UUID Parsing for Validation

```rust
use underlay_http::{parse_uuid_for_validation, parse_optional_uuid_for_validation};

// Required UUID - returns Err(ValidationResult) on invalid
let id = parse_uuid_for_validation(&value, "moduleId")?;

// Optional UUID - returns Ok(None) if missing
let exclude_id = parse_optional_uuid_for_validation(value.as_deref(), "excludeId")?;
```

### Validator Crate Integration

When using the `validator` crate, convert validation errors to `AppError` with field errors:

```rust
use underlay_http::{ApiError, ApiResult, validation_to_app_error, ValidateExt};
use validator::Validate;

// Option 1: Manual conversion
async fn create_user(Json(payload): Json<CreateUserPayload>) -> ApiResult<SingleResponse<UserDto>> {
    if let Err(validation_err) = payload.validate() {
        let err = validation_to_app_error(
            &validation_err,
            "user.invalid",
            "There is a problem with one or more fields."
        );
        return Err(ApiError::new(StatusCode::BAD_REQUEST, err));
    }
    // ...
}

// Option 2: Trait extension (more concise)
async fn create_user(Json(payload): Json<CreateUserPayload>) -> ApiResult<SingleResponse<UserDto>> {
    payload.validate_or_error("user.invalid")?;
    // ...
}
```

Enable with the `validation` feature:

```toml
underlay-http = { path = "...", features = ["validation"] }
```

### Nightfire Content Validation

Convert Nightfire validation errors to HTTP error responses:

```rust
use underlay_http::{ApiError, ApiResult, nightfire_validation_to_app_error};
use nightfire::{validate_nightfire_value_by_schema, NightfireValue};
use axum::http::StatusCode;

async fn create_content(body: NightfireValue) -> ApiResult<SingleResponse<ContentDto>> {
    if let Err(validation_err) = validate_nightfire_value_by_schema(&body) {
        let err = nightfire_validation_to_app_error(
            validation_err,
            "content.invalid",
            "body",
            "Content body failed schema validation.",
        );
        return Err(ApiError::new(StatusCode::BAD_REQUEST, err));
    }
    // ... rest of handler
}
```

Enable with the `nightfire` feature:

```toml
underlay-http = { path = "...", features = ["nightfire"] }
```

### Query Field Mapping

Simplify field mappings for sortable/filterable list endpoints:

```rust
use underlay_http::{FieldMapping, WhereBuilder, QueryParams};

pub async fn list_users_with_query(
    pool: &PgPool,
    query: &QueryParams,
) -> Result<Vec<UserRow>, sqlx::Error> {
    // Define field mappings (API name => DB column)
    let mapping = FieldMapping::new()
        .map("name", "name")           // Both sort and filter
        .map("email", "email")
        .map("isActive", "is_active")
        .sort_only("createdAt", "created_at")  // Sort only
        .filter_only("role", "role");          // Filter only

    // Build WHERE clause from filters
    let filters = query.filter_fields();
    let mut where_builder = WhereBuilder::new(1);
    where_builder.add_condition("deleted_at IS NULL");

    for filter in &filters {
        where_builder.add_filter(filter, &mapping.filter_map());
    }

    let (where_clause, filter_values) = where_builder.build();
    let order_by = query.sql_order_by_or(&mapping.sort_map(), "name");

    // ... execute query
}
```

Or use the macro for common cases where all fields support both sort and filter:

```rust
use underlay_http::field_mapping;

let mapping = field_mapping! {
    "name" => "name",
    "slug" => "slug",
    "isLive" => "is_live",
    "createdAt" => "created_at",
};
```

#### FieldMapping Methods

| Method | Description |
|--------|-------------|
| `map(api, db)` | Map field for both sorting and filtering |
| `sort_only(api, db)` | Map field for sorting only |
| `filter_only(api, db)` | Map field for filtering only |
| `sort_map()` | Get `HashMap<&str, &str>` for sort lookups |
| `filter_map()` | Get `HashMap<&str, &str>` for filter lookups |
| `get_sort(api)` | Look up sort column for API field |
| `get_filter(api)` | Look up filter column for API field |

### CORS Configuration

```rust
use underlay_http::{cors_layer, CorsConfig};

let cors = cors_layer(
    CorsConfig::new()
        .with_origins(["https://example.com"])
        .with_credentials(true),
);

let app = Router::new()
    .route("/api/users", get(list_users))
    .layer(cors);
```

### Graceful Shutdown (SIGINT/SIGTERM)

In production deployments (Docker, systemd, Kubernetes), your API should stop accepting new connections and allow in-flight requests to finish when it receives a shutdown signal.

Axum supports this via `with_graceful_shutdown(...)`.

Recommended pattern:

- Handle `SIGINT` (Ctrl+C) for local development.
- Handle `SIGTERM` for production (Kubernetes sends SIGTERM by default).

```rust
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ... build router, bind listener

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal as unix_signal, SignalKind};

        let mut sigint = unix_signal(SignalKind::interrupt())
            .expect("failed to install SIGINT handler");
        let mut sigterm = unix_signal(SignalKind::terminate())
            .expect("failed to install SIGTERM handler");

        tokio::select! {
            _ = sigint.recv() => {
                tracing::info!("received SIGINT; starting graceful shutdown");
            }
            _ = sigterm.recv() => {
                tracing::info!("received SIGTERM; starting graceful shutdown");
            }
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
        tracing::info!("received Ctrl+C; starting graceful shutdown");
    }
}
```

### Complete HTTP Example

```rust
use axum::{Router, routing::get, extract::Query, Json};
use underlay_http::{
    context::{RequestContext, AuthenticatedContext},
    pagination::{PaginationParams, Paginated},
    ok, created,
};

async fn list_items(
    ctx: RequestContext,
    Query(params): Query<PaginationParams>,
) -> Json<Paginated<Item>> {
    tracing::info!(
        request_id = %ctx.request_id(),
        "Listing items"
    );
    
    let items = fetch_items(params.limit, params.offset()).await;
    let total = count_items().await;
    
    Json(params.wrap(items, total))
}

async fn get_my_profile(ctx: AuthenticatedContext) -> impl IntoResponse {
    let user = fetch_user(ctx.user_id()).await;
    ok(user)
}

fn create_router() -> Router {
    Router::new()
        .route("/items", get(list_items))
        .route("/me", get(get_my_profile))
}
```

## Request Validation (`underlay-validation`)

Declarative validation with built-in validators and derive macro.

### Installation

```toml
[dependencies]
underlay-validation = { path = "../underlay/rust/crates/underlay-validation" }

# With Axum integration
underlay-validation = { path = "...", features = ["axum"] }
```

### Derive Macro (Recommended)

The easiest way to add validation is using the `#[derive(Validate)]` macro:

```rust
use underlay_validation::Validate;

#[derive(Validate, Deserialize)]
struct CreateUserRequest {
    #[validate(email)]
    email: String,

    #[validate(length(min = 8, max = 100))]
    password: String,

    #[validate(range(min = 18, max = 120))]
    age: i32,

    #[validate(username)]
    username: String,
}
```

#### Available Derive Attributes

| Attribute | Description | Example |
|-----------|-------------|---------|
| `#[validate(email)]` | Valid email format | `user@example.com` |
| `#[validate(url)]` | Valid HTTP(S) URL | `https://example.com` |
| `#[validate(uuid)]` | Valid UUID format | `550e8400-e29b-41d4-a716-446655440000` |
| `#[validate(required)]` | Non-empty string | `"hello"` |
| `#[validate(length(min = N, max = M))]` | String length bounds | 8-100 chars |
| `#[validate(range(min = N, max = M))]` | Numeric range | 18-120 |
| `#[validate(pattern = "regex")]` | Custom regex pattern | `r"^\d{3}-\d{4}$"` |
| `#[validate(custom = "fn_name")]` | Custom validator function | See below |
| `#[validate(positive)]` | Greater than zero | `1, 2, 3...` |
| `#[validate(non_negative)]` | Zero or greater | `0, 1, 2...` |
| `#[validate(alphanumeric)]` | Letters and numbers only | `ABC123` |
| `#[validate(username)]` | Letters, numbers, `_`, `-` | `john_doe` |
| `#[validate(slug)]` | Lowercase slug format | `my-article` |
| `#[validate(not_empty)]` | Non-empty collection | `vec!["a"]` |
| `#[validate(collection_length(min = N, max = M))]` | Collection size | 1-10 items |
| `#[validate(nested)]` | Validate nested struct | See below |
| `#[validate(skip)]` | Skip validation for field | - |

#### Custom Validators with Derive

```rust
use underlay_validation::{Validate, FieldError};

fn validate_starts_with_a(value: &str) -> Result<(), FieldError> {
    if value.starts_with('a') || value.starts_with('A') {
        Ok(())
    } else {
        Err(FieldError::new("Must start with 'a'"))
    }
}

#[derive(Validate)]
struct MyRequest {
    #[validate(custom = "validate_starts_with_a")]
    name: String,
}
```

#### Nested Validation with Derive

```rust
#[derive(Validate)]
struct Address {
    #[validate(required)]
    city: String,
    
    #[validate(length(min = 5, max = 10))]
    postal_code: String,
}

#[derive(Validate)]
struct CreateOrderRequest {
    #[validate(email)]
    email: String,

    #[validate(nested)]
    shipping_address: Address,
}
```

This produces field names like `shipping_address.city` in error responses.

### Manual Implementation

For complex validation logic, implement the `Validate` trait directly:

```rust
use underlay_validation::{Validate, ValidationError, validators};

#[derive(Deserialize)]
struct CreateUserRequest {
    email: String,
    password: String,
    age: i32,
    username: String,
}

impl Validate for CreateUserRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        if let Err(e) = validators::email(&self.email) {
            errors.add_field("email", e);
        }

        if let Err(e) = validators::length(&self.password, Some(8), Some(100)) {
            errors.add_field("password", e);
        }

        if let Err(e) = validators::range(self.age, Some(18), Some(120)) {
            errors.add_field("age", e);
        }

        if let Err(e) = validators::username(&self.username) {
            errors.add_field("username", e);
        }

        errors.into_result()
    }
}
```

### ValidatedJson Extractor

With the `axum` feature, use `ValidatedJson` instead of `Json`:

```rust
use underlay_validation::ValidatedJson;

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    // payload is guaranteed to be valid
    // Invalid requests return 400 with field errors automatically
}
```

Error response format:

```json
{
  "error": {
    "code": "validation.failed",
    "message": "Validation failed",
    "field_errors": {
      "email": "Invalid email address",
      "password": "Must be at least 8 characters",
      "age": "Must be at least 18"
    }
  }
}
```

### Built-in Validators

| Validator | Description | Example |
|-----------|-------------|---------|
| `email(value)` | Valid email format | `validators::email(&s)` |
| `url(value)` | Valid HTTP(S) URL | `validators::url(&s)` |
| `uuid(value)` | Valid UUID format | `validators::uuid(&s)` |
| `length(value, min, max)` | String length bounds | `validators::length(&s, Some(8), Some(100))` |
| `required(value)` | Non-empty string | `validators::required(&s)` |
| `range(value, min, max)` | Numeric range | `validators::range(age, Some(18), Some(120))` |
| `positive(value)` | Greater than zero | `validators::positive(count)` |
| `non_negative(value)` | Zero or greater | `validators::non_negative(count)` |
| `pattern(value, regex, msg)` | Custom regex | `validators::pattern(&s, r"^\d+$", "Must be digits")` |
| `one_of(value, options)` | Value in list | `validators::one_of(&status, &["active", "inactive"])` |
| `not_empty(slice)` | Non-empty collection | `validators::not_empty(&items)` |
| `collection_length(slice, min, max)` | Collection size bounds | `validators::collection_length(&items, Some(1), Some(10))` |
| `alphanumeric(value)` | Letters and numbers only | `validators::alphanumeric(&s)` |
| `username(value)` | Letters, numbers, _, - | `validators::username(&s)` |
| `slug(value)` | Lowercase slug format | `validators::slug(&s)` |

### Nested Validation

Validate nested objects with prefixed field names:

```rust
impl Validate for CreateOrderRequest {
    fn validate(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();

        // Validate nested address
        if let Err(address_errors) = self.address.validate() {
            errors.merge_nested("address", address_errors);
        }

        // Validate nested items
        for (i, item) in self.items.iter().enumerate() {
            if let Err(item_errors) = item.validate() {
                errors.merge_nested(&format!("items[{}]", i), item_errors);
            }
        }

        errors.into_result()
    }
}
```

Results in field names like `address.city` and `items[0].quantity`.

### Custom Validators

Create reusable validators:

```rust
fn validate_phone(value: &str) -> Result<(), FieldError> {
    static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^\+?[1-9]\d{1,14}$").unwrap()
    });

    if PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(FieldError::with_code("Invalid phone number", "phone.invalid"))
    }
}

// Use in validation
if let Err(e) = validate_phone(&self.phone) {
    errors.add_field("phone", e);
}
```

### Error Codes for i18n

Field errors include codes for internationalization:

```rust
FieldError::with_code("Invalid email address", "email.invalid")
```

Built-in validator codes:
- `email.invalid`, `url.invalid`, `uuid.invalid`
- `length.min`, `length.max`
- `range.min`, `range.max`
- `required`, `positive`, `non_negative`
- `pattern.invalid`, `one_of.invalid`
- `not_empty`, `collection.min`, `collection.max`
- `alphanumeric`, `username.invalid`, `slug.invalid`

### Migration Guide

#### Migrating from Manual Validation

If you have existing manual validation code, you can gradually migrate to the derive macro:

**Before (manual validation scattered in handlers):**

```rust
async fn create_user(Json(payload): Json<CreateUserRequest>) -> ApiResult<SingleResponse<UserDto>> {
    // Manual validation in handler
    if !payload.email.contains('@') {
        return Err(ApiError::bad_request(
            "validation.failed",
            "Invalid email",
        ));
    }
    if payload.password.len() < 8 {
        return Err(ApiError::bad_request(
            "validation.failed",
            "Password too short",
        ));
    }
    // ... more validation
    
    // Business logic
}
```

**After (derive macro + ValidatedJson):**

```rust
#[derive(Validate, Deserialize)]
struct CreateUserRequest {
    #[validate(email)]
    email: String,
    
    #[validate(length(min = 8))]
    password: String,
}

async fn create_user(
    ValidatedJson(payload): ValidatedJson<CreateUserRequest>,
) -> impl IntoResponse {
    // Validation already done - payload is guaranteed valid
    // Business logic only
}
```

#### Step-by-Step Migration

1. **Add the dependency:**
   ```toml
   underlay-validation = { path = "...", features = ["axum"] }
   ```

2. **Add `#[derive(Validate)]` to request structs:**
   ```rust
   #[derive(Validate, Deserialize)]
   struct MyRequest { ... }
   ```

3. **Add validation attributes to fields:**
   ```rust
   #[validate(email)]
   email: String,
   ```

4. **Replace `Json<T>` with `ValidatedJson<T>`:**
   ```rust
   // Before
   async fn handler(Json(payload): Json<MyRequest>)
   
   // After
   async fn handler(ValidatedJson(payload): ValidatedJson<MyRequest>)
   ```

5. **Remove manual validation code from handlers.**

#### Mixing Manual and Derive Validation

For complex cross-field validation, combine both approaches:

```rust
#[derive(Validate, Deserialize)]
struct CreateOrderRequest {
    #[validate(positive)]
    quantity: i32,
    
    #[validate(positive)]  
    max_quantity: i32,
    
    #[validate(email)]
    email: String,
}

// Add cross-field validation manually
impl CreateOrderRequest {
    fn validate_business_rules(&self) -> Result<(), ValidationError> {
        let mut errors = ValidationError::new();
        
        // Cross-field validation
        if self.quantity > self.max_quantity {
            errors.add_field("quantity", 
                FieldError::new("Cannot exceed max quantity"));
        }
        
        errors.into_result()
    }
}

async fn create_order(
    ValidatedJson(payload): ValidatedJson<CreateOrderRequest>,
) -> impl IntoResponse {
    // Basic validation done by derive macro
    // Now do cross-field validation
    payload.validate_business_rules()?;
    
    // Business logic
}
```

## Next Steps

Proceed to [080-typescript-client](./080-typescript-client.md) to build a typed client that matches these envelopes and errors.
