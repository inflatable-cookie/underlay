# Error Logging

Underlay provides a comprehensive error logging system that captures HTTP error responses and stores them in a database for later inspection. This is invaluable for debugging production issues, monitoring error trends, and auditing system behaviour.

## Overview

The error logging feature includes:

- **Middleware** - Automatically captures 4xx and 5xx responses
- **Database storage** - Persists errors to `platform.error_log` table
- **Query functions** - List, filter, and retrieve error logs
- **Context enrichment** - Capture structured handler context from `ApiError`

## Feature Flag

Error logging is an optional feature that must be enabled in your `Cargo.toml`:

```toml
[dependencies]
underlay-http = { path = "../underlay/rust/crates/underlay-http", features = ["error-logging"] }
```

This feature flag enables the following additional dependencies:
- `underlay-db` for database access
- `sqlx` for queries
- `chrono` for timestamps
- `tokio` for async spawning
- `urlencoding` for header encoding
- `tracing` for error logging

## Database Migration

Add the `platform.error_log` table to your database. The migration is included in Underlay's baseline migrations when using `underlay-db`:

```sql
CREATE TABLE platform.error_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    endpoint TEXT NOT NULL,
    method VARCHAR(10) NOT NULL,
    status_code INTEGER NOT NULL,
    error_code VARCHAR(100) NOT NULL,
    message TEXT NOT NULL DEFAULT '',
    correlation_id VARCHAR(100) NOT NULL DEFAULT '',
    context JSONB NOT NULL DEFAULT '{}'::jsonb
);

CREATE INDEX idx_error_log_occurred_at ON platform.error_log(occurred_at DESC);
CREATE INDEX idx_error_log_status_code ON platform.error_log(status_code);
CREATE INDEX idx_error_log_error_code ON platform.error_log(error_code);
```

## Basic Usage

### Setting Up the Middleware

Configure and add the error logging middleware to your Axum router:

```rust
use underlay_http::{error_logging_middleware, ErrorLoggingConfig};
use axum::{Router, middleware};

// Create configuration
let error_logging_config = ErrorLoggingConfig::new(pool.clone())
    .with_source("my-api")           // Optional: identifies the app in logs
    .with_client_errors(true)         // Log 4xx errors (default: true)
    .with_server_errors(true);        // Log 5xx errors (default: true)

// Add to router
let app = Router::new()
    .route("/", get(handler))
    .with_state(app_state)
    .layer(middleware::from_fn_with_state(
        error_logging_config,
        error_logging_middleware,
    ));
```

### Middleware Placement

The error logging middleware should be placed after the request ID and tracing layers but before CORS:

```rust
let app = routes::build_router()
    .with_state(state)
    .layer(underlay_observability::trace_layer())      // First: tracing
    .layer(underlay_observability::request_id_layer()) // Second: request ID
    .layer(middleware::from_fn_with_state(             // Third: error logging
        error_logging_config,
        error_logging_middleware,
    ))
    .layer(underlay_http::cors_layer(cors));           // Last: CORS
```

This ensures:
1. Request IDs are available for correlation
2. Tracing spans are established
3. Errors are captured before CORS headers are added

## Canonical Handler Path

Use `ApiError` and `ApiResult<T>` in handlers. This is the canonical Underlay path for rich error logging:

```rust
use axum::extract::{Json, Path};
use underlay_core::SingleResponse;
use underlay_http::{ApiError, ApiResult, ok};

async fn update_record(
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateRequest>,
) -> ApiResult<SingleResponse<RecordDto>> {
    let record = db::update_record(&pool, id, &payload).await.map_err(|e| {
        ApiError::internal("db.update_failed", "Failed to update record")
            .with_cause(e.to_string())
            .with_context(serde_json::json!({
                "record_id": id,
                "payload": payload
            }))
    })?;

    Ok(ok(record))
}
```

`ApiError` writes `x-error-code`, `x-error-message`, and `x-error-context` headers; the middleware extracts these and persists them into `platform.error_log.context.handler_context`.

## Compatibility Path

`error_response_with_context()` remains available for legacy handlers returning `impl IntoResponse`:

```rust
use axum::http::StatusCode;
use underlay_core::AppError;
use underlay_http::error_response_with_context;

return error_response_with_context(
    StatusCode::BAD_REQUEST,
    AppError::new("validation.invalid", "Validation failed"),
    serde_json::json!({"field": "email"}),
)
.into_response();
```

Soft deprecation policy:

1. Keep compatibility helpers available during migration windows.
2. Prefer `ApiError` in all new and touched handlers.
3. Treat new `error_response(...)` usage in routes as a migration regression.

## Downstream Upgrade Playbook

Use this sequence to migrate an existing Underlay-based app with minimal local glue code:

1. Update Underlay dependencies and ensure `underlay-http` has `error-logging` enabled.
2. Confirm middleware order (`trace` -> `request_id` -> `error_logging` -> `cors`).
3. Convert route handlers to `ApiResult<T>` where practical.
4. Replace raw `StatusCode::...into_response()` error branches with `ApiError`.
5. Replace legacy `error_response(...)` / `error_response_with_context(...)` route callsites with `ApiError`.
6. Add structured context to high-value failures:
   - operation name
   - primary entity IDs
   - safe query/filter metadata
7. Run a migration sweep:
   - `rg -n "\\berror_response\\(" crates/api/src/routes`
   - `rg -n "StatusCode::(BAD_REQUEST|UNAUTHORIZED|FORBIDDEN|NOT_FOUND|CONFLICT|INTERNAL_SERVER_ERROR|TOO_MANY_REQUESTS).*into_response\\(" crates/api/src/routes`
8. Validate via `cargo check` and trigger known failure paths; confirm admin error logs now include non-null `handler_context`.

### Copy-Paste Migration Patterns

#### Pattern A: Raw status branch -> `ApiError`

```rust
// before
if invalid {
    return StatusCode::BAD_REQUEST.into_response();
}

// after
if invalid {
    return ApiError::bad_request("validation.invalid_input", "Invalid input")
        .with_context(serde_json::json!({
            "operation": "items.create",
            "item_id": item_id,
        }))
        .into_response();
}
```

#### Pattern B: Legacy `error_response_with_context` -> `ApiError`

```rust
// before
return error_response_with_context(
    StatusCode::BAD_REQUEST,
    AppError::new("validation.invalid", "Validation failed"),
    serde_json::json!({ "field": "email" }),
)
.into_response();

// after
return ApiError::bad_request("validation.invalid", "Validation failed")
    .with_context(serde_json::json!({ "field": "email" }))
    .into_response();
```

#### Pattern C: DB failure mapping with context

```rust
let row = repo::get_item(&pool, id).await.map_err(|e| {
    ApiError::internal("db.select_failed", "Failed to load item")
        .with_cause(&e)
        .with_context(serde_json::json!({
            "operation": "items.get",
            "item_id": id,
        }))
})?;
```

## Querying Error Logs

### List Errors with Filters

```rust
use underlay_http::{list_error_logs, ErrorLogFilters};

let filters = ErrorLogFilters {
    status_code: Some(500),           // Only 500 errors
    since: Some(Utc::now() - Duration::hours(24)), // Last 24 hours
    limit: 50,
    offset: 0,
    ..Default::default()
};

let errors = list_error_logs(&pool, filters).await?;
```

### Get a Specific Error

```rust
use underlay_http::get_error_log_by_id;

let error = get_error_log_by_id(&pool, error_id).await?;
```

### Count Errors for Pagination

```rust
use underlay_http::count_error_logs;

let total = count_error_logs(&pool, &filters).await?;
```

## Admin API Endpoints

For admin UIs, you'll typically want API endpoints to query error logs. Here's an example implementation:

```rust
// routes/admin/error_logs.rs
use axum::{extract::Query, response::IntoResponse, Json};
use underlay_http::{list_error_logs, get_error_log_by_id, count_error_logs, ErrorLogFilters};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListErrorLogsQuery {
    pub status_code: Option<i32>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn list_error_logs_handler(
    Query(query): Query<ListErrorLogsQuery>,
) -> impl IntoResponse {
    let filters = ErrorLogFilters {
        status_code: query.status_code,
        limit: query.limit.unwrap_or(50),
        offset: query.offset.unwrap_or(0),
        ..Default::default()
    };

    let logs = list_error_logs(&pool, filters).await?;
    let total = count_error_logs(&pool, &filters).await?;

    Json(PaginatedResponse { data: logs, total })
}
```

## Error Log Row Structure

Each error log entry contains:

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `occurred_at` | DateTime | When the error occurred |
| `endpoint` | String | Request path (e.g., `/v1/users`) |
| `method` | String | HTTP method (GET, POST, etc.) |
| `status_code` | i32 | HTTP status code (400, 500, etc.) |
| `error_code` | String | Application error code |
| `message` | String | Error message |
| `correlation_id` | String | Request ID for tracing |
| `context` | JSONB | Additional debugging context |

## Context Object Structure

The middleware automatically captures and stores context including:

```json
{
  "source": "my-api",
  "query": "?page=1&limit=10",
  "user_agent": "Mozilla/5.0...",
  "handler_context": {
    "db_error": "connection timeout",
    "record_id": "abc-123"
  }
}
```

## Best Practices

### 1. Always Add Context for Server Errors

For 5xx errors, always include relevant debugging information:

```rust
ApiError::internal("external.api_error", "External service unavailable")
    .with_context(serde_json::json!({
        "service": "payment-gateway",
        "response_code": response.status().as_u16(),
        "request_id": external_request_id,
    }))
    .with_cause(response_text)
```

### 2. Don't Log Sensitive Data

Never include these in `context`:
- Credentials (passwords, OTP codes, passkeys)
- Tokens or secrets (JWTs, refresh tokens, API keys, cookie values)
- Raw PII (full email bodies, addresses, phone numbers, full legal names)
- Raw request payload dumps from auth/account/profile endpoints

Safe context example:

```rust
ApiError::internal("db.update_failed", "Failed to update record")
    .with_context(serde_json::json!({
        "operation": "projects.update",
        "project_id": project_id,
        "request_id": request_id,
        "failure_class": "constraint_violation",
    }))
```

Unsafe anti-example:

```rust
ApiError::internal("auth.failed", "Authentication failed")
    .with_context(serde_json::json!({
        "password": payload.password,
        "token": auth_token,
        "raw_payload": payload,
    }))
```

### 3. Prefer IDs and Failure Class

For context fields, prefer:
- operation names (`projects.update`)
- stable IDs (`project_id`, `user_id`, `job_id`)
- failure class (`validation_failed`, `db_timeout`, `external_4xx`)

Avoid:
- full serialized payloads
- unbounded free-form blobs
- duplicate user-entered content already present in request body
### 4. Use Meaningful Error Codes

Structure error codes hierarchically:
- `auth.invalid_credentials`
- `validation.email_invalid`
- `db.connection_failed`
- `external.payment_declined`

### 5. Configure Log Retention

Error logs can grow quickly. Implement a maintenance job to purge old entries:

```rust
// Purge error logs older than 90 days
sqlx::query("DELETE FROM platform.error_log WHERE occurred_at < now() - interval '90 days'")
    .execute(&pool)
    .await?;
```

### 6. Monitor Error Trends

Use the error log data for monitoring:

```sql
-- Errors by status code in last 24 hours
SELECT status_code, COUNT(*) as count
FROM platform.error_log
WHERE occurred_at > now() - interval '24 hours'
GROUP BY status_code
ORDER BY count DESC;

-- Most common error codes
SELECT error_code, COUNT(*) as count
FROM platform.error_log
WHERE occurred_at > now() - interval '7 days'
GROUP BY error_code
ORDER BY count DESC
LIMIT 10;
```

## Integration with Observability

The error logging system integrates with Underlay's observability features:

- **Request IDs**: Errors are correlated via the `x-request-id` header
- **Tracing**: Error logs can be cross-referenced with distributed traces
- **Metrics**: Consider adding Prometheus counters for error rates

## TypeScript Client Types

For admin UIs, define corresponding TypeScript types:

```typescript
export interface ErrorLogSummary {
  id: string;
  occurredAt: string;
  endpoint: string;
  method: string;
  statusCode: number;
  errorCode: string;
  message: string;
  correlationId: string;
}

export interface ErrorLogDetail extends ErrorLogSummary {
  context: Record<string, unknown>;
}

export interface ErrorLogStats {
  totalLast24h: number;
  serverErrorsLast24h: number;
  clientErrorsLast24h: number;
}
```

## Troubleshooting

### Errors Not Being Logged

1. Check that the `error-logging` feature is enabled
2. Verify middleware ordering (must be before CORS layer)
3. Ensure the database pool is correctly passed to `ErrorLoggingConfig`
4. Check for database connection errors in application logs

### Context Not Appearing

1. Ensure handlers return `ApiError` (or legacy `error_response_with_context()`)
2. Verify the context is valid JSON
3. Check that context size doesn't exceed header limits (use smaller context if needed)

### Reference App Smoke Test

Use the Acme reference script to validate the capture path end-to-end:

```bash
cd /path/to/underlay-reference
./scripts/smoke-error-logging.sh
```

Expected result:
1. A forced 500 is returned from `POST /v1/dev/error-smoke` (debug build).
2. A new `platform.error_log` row exists for `error_code = smoke.forced_db_failure`.
3. The row includes non-null `message` and `context.handler_context.operation = smoke.error_logging_capture`.

To measure the current `handler_context` null-rate in the reference app:

```bash
cd /path/to/underlay-reference
./scripts/error-log-metrics.sh
```

To run the full reference validation sequence in one command:

```bash
cd /path/to/underlay-reference
./scripts/validate-error-reporting.sh
```

### High Database Load

1. Consider disabling `log_client_errors` for high-traffic APIs
2. Implement sampling for very high error volumes
3. Add database indexes on frequently-filtered columns
