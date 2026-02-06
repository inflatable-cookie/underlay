# underlay-http - Error Logging

Error logging database support for Underlay HTTP services.

## Overview

The `error-logging` feature adds database persistence for HTTP errors, allowing you to track and analyze failures in your API.

## Features

- **Non-blocking async logging** - Uses `tokio::spawn` to avoid slowing down request handling
- **Configurable schema** - Uses `platform.error_log` by default
- **Rich filtering** - Query by time range, status code, error code, or endpoint
- **Indexed for performance** - Pre-configured indexes for common queries

## Setup

### 1. Enable the feature

Add to your `Cargo.toml`:

```toml
[dependencies]
underlay-http = { path = "../underlay-http", features = ["error-logging"] }
```

### 2. Run migrations

Sync the migration to your app:

```bash
cargo run --bin underlay-devtools -- sync-migrations --target ./migrations
```

Then run migrations:

```bash
cargo run --bin migrate_dev_db
```

This creates the `platform.error_log` table with indexes.

## Usage

### Canonical Handler Path (`ApiError`)

`ApiError` is the canonical way to return errors from handlers. It emits:

- standard error envelope
- `x-error-code`
- `x-error-message`
- `x-error-context`

These headers are consumed by error logging middleware and persisted to `platform.error_log`.

```rust
use underlay_http::{ApiError, ApiResult};
use axum::{extract::State, Json};

async fn list_users(State(state): State<AppState>) -> ApiResult<Json<Vec<UserDto>>> {
    let users = sqlx::query_as::<_, UserDto>("SELECT id, email FROM users")
        .fetch_all(state.pool())
        .await
        .map_err(|err| {
            ApiError::internal("db.query_failed", "Failed to list users")
                .with_context(serde_json::json!({
                    "operation": "list_users",
                }))
                .with_cause(&err)
        })?;

    Ok(Json(users))
}
```

### Basic Error Logging

```rust
use underlay_http::error_logging::append_error_log;

// Log an error (non-blocking via tokio::spawn recommended)
tokio::spawn(async move {
    let _ = append_error_log(
        &pool,
        "/v1/users",          // endpoint
        "GET",                // HTTP method
        500,                  // status code
        "database_error",     // error code
        "Connection timeout", // message
        "req-abc-123",        // correlation ID
        serde_json::json!({"details": "Pool exhausted"}),
    ).await;
});
```

### Using DbErrorLogSink

The `DbErrorLogSink` implements the `ErrorLogSink` trait for automatic error logging:

```rust
use underlay_http::error_logging::DbErrorLogSink;
use underlay_http::{ErrorLogContext, ErrorLogSink};

let sink = DbErrorLogSink::new(pool.clone());

// Use in error responses
sink.record(ErrorLogContext {
    request_id: Some(request_id),
    status: StatusCode::INTERNAL_SERVER_ERROR,
    code: "internal_error".to_string(),
    message: "Something went wrong".to_string(),
});
```

**Note**: `DbErrorLogSink.record()` doesn't capture endpoint/method. Prefer `ApiError` + middleware for normal HTTP handlers.

### Compatibility Path (`error_response_with_context`)

`error_response_with_context()` is kept for migration and compatibility, but it is not the preferred path for new handlers.

```rust
use underlay_http::error_response_with_context;
use underlay_core::AppError;
use axum::http::StatusCode;

let res = error_response_with_context(
    StatusCode::INTERNAL_SERVER_ERROR,
    AppError::new("db.error", "Database operation failed"),
    serde_json::json!({
        "operation": "legacy_handler",
        "resource_id": resource_id,
    }),
);
```

### Querying Error Logs

```rust
use underlay_http::error_logging::{list_error_logs, ErrorLogFilters};

// Get recent 500 errors
let filters = ErrorLogFilters {
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

### Filtering Options

```rust
use chrono::{Utc, Duration};
use underlay_http::error_logging::ErrorLogFilters;

let filters = ErrorLogFilters {
    // Time range
    since: Some(Utc::now() - Duration::hours(24)),
    until: Some(Utc::now()),
    
    // HTTP context
    status_code: Some(503),
    endpoint: Some("/v1/health".to_string()),
    error_code: Some("service_unavailable".to_string()),
    
    // Pagination
    limit: 100,
    offset: 0,
};
```

## Database Schema

```sql
CREATE TABLE platform.error_log (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at     TIMESTAMPTZ NOT NULL DEFAULT now(),
    endpoint        TEXT NOT NULL,
    method          TEXT NOT NULL,
    status_code     INTEGER NOT NULL,
    error_code      TEXT NOT NULL,
    message         TEXT NOT NULL,
    correlation_id  TEXT NOT NULL,
    context         JSONB NOT NULL DEFAULT '{}'::jsonb
);
```

## Best Practices

### 1. Log asynchronously

Always wrap error logging in `tokio::spawn` to avoid blocking request handling:

```rust
tokio::spawn(async move {
    let _ = append_error_log(&pool, ...).await;
});
```

### 2. Use correlation IDs

Use the request ID from `underlay-observability::RequestId` for tracing:

```rust
use underlay_observability::RequestId;

let request_id = RequestId::generate();

tokio::spawn(async move {
    let _ = append_error_log(
        &pool,
        endpoint,
        method,
        status,
        code,
        message,
        &request_id.to_string(), // correlation_id
        context,
    ).await;
});
```

### 3. Add context for debugging

Use the `context` JSONB field for additional debugging information:

```rust
let context = serde_json::json!({
    "user_id": user_id,
    "request_headers": headers,
    "stack_trace": stack_trace,
});

append_error_log(&pool, ..., context).await?;
```

### 4. Query efficiently

The table is indexed for common queries. Use filters to take advantage of indexes:

```rust
// GOOD: Uses occurred_at index
let filters = ErrorLogFilters {
    since: Some(Utc::now() - Duration::hours(1)),
    status_code: Some(500),
    ..Default::default()
};

// AVOID: Full table scan on context JSONB
// Instead, use specific error_code or add context to error_code
```

## Future Enhancements

The following are planned but not yet implemented:

- **Tower Middleware** - Automatic error logging for all responses
- **Retention Policies** - Automatic cleanup of old error logs
- **Aggregation Queries** - Helper functions for error rate metrics
- **Alert Triggers** - Database functions to trigger notifications

See `Phase 8.3` in the roadmap for details.

## Related

- `underlay-http::ErrorLogSink` - Trait for error logging
- `underlay-observability::RequestId` - Request correlation
- `underlay-db` - Database connection pool
