# 080 - Audit Logging

> **Underlay Crate**: `underlay-audit` provides types and database functions for recording audit trails of administrative actions.

This guide covers setting up audit logging for security, compliance, and debugging purposes.

## Overview

The `underlay-audit` crate provides:

- **Types**: `AuditEntry`, `AuditAction`, `AuditLogRow`, `AuditLogFilters`
- **Typed table config**: `AuditTable`
- **Functions**: `append_audit_log_to_table`, `list_audit_logs_from_table`, `get_audit_log_by_id_from_table`, `count_audit_logs_from_table`

Audit logs are essential for:
- Security forensics (who did what, when)
- Compliance requirements (SOC 2, GDPR audit trails)
- Debugging and troubleshooting
- Change tracking and accountability

## Database Setup

Create an audit log table in your migrations:

```sql
CREATE TABLE platform.audit_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    user_id UUID,  -- NULL for system actions
    action TEXT NOT NULL,
    resource_type TEXT NOT NULL,
    resource_id UUID NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    correlation_id TEXT,
    ip_address TEXT
);

CREATE INDEX idx_audit_log_occurred_at ON platform.audit_log (occurred_at DESC);
CREATE INDEX idx_audit_log_user_id ON platform.audit_log (user_id);
CREATE INDEX idx_audit_log_resource ON platform.audit_log (resource_type, resource_id);
CREATE INDEX idx_audit_log_action ON platform.audit_log (action);
```

## Recording Audit Events

### Basic Usage

```rust
use underlay_audit::{append_audit_log_to_table, AuditAction, AuditEntry, AuditTable};
use serde_json::json;

let audit_table = AuditTable::parse("platform.audit_log")?;

// Log an admin action
append_audit_log_to_table(
    &pool,
    &audit_table,
    AuditEntry {
        user_id: Some(admin_user.id),
        action: AuditAction::Create,
        resource_type: "project".to_string(),
        resource_id: project_id,
        details: json!({
            "name": "New Project",
            "description": "Project description"
        }),
        correlation_id: Some(request_id.to_string()),
        ip_address: Some(client_ip.to_string()),
    },
).await?;
```

### Available Actions

The `AuditAction` enum provides standard action types:

| Action | Description |
|--------|-------------|
| `Create` | Resource creation |
| `Update` | Resource modification |
| `Delete` | Resource deletion (soft or hard) |
| `View` | Sensitive resource access |
| `Login` | User authentication |
| `Logout` | Session termination |
| `Export` | Data export |
| `Import` | Data import |
| `Custom(String)` | App-specific actions |

### Async Logging

For non-blocking audit logging (e.g., in request handlers):

```rust
use underlay_audit::{append_audit_log_to_table_async, AuditTable};

let audit_table = AuditTable::parse("platform.audit_log")?;

// Fire-and-forget audit logging
append_audit_log_to_table_async(
    pool.clone(),
    audit_table,
    AuditEntry {
        user_id: Some(user.id),
        action: AuditAction::View,
        resource_type: "report".to_string(),
        resource_id: report_id,
        details: json!({}),
        correlation_id: None,
        ip_address: None,
    },
);
```

## Querying Audit Logs

### List with Filters

```rust
use underlay_audit::{list_audit_logs_from_table, AuditLogFilters, AuditTable};

let filters = AuditLogFilters {
    user_id: Some(admin_id),
    resource_type: Some("project".to_string()),
    action: Some("create".to_string()),
    since: Some(Utc::now() - Duration::days(7)),
    until: None,
    limit: 50,
    offset: 0,
};

let audit_table = AuditTable::parse("platform.audit_log")?;
let logs = list_audit_logs_from_table(&pool, &audit_table, filters).await?;
```

The older raw-string functions remain available as compatibility wrappers.
New code should construct `AuditTable` once from typed app config and pass it
through the typed helpers.

### Get Single Entry

```rust
use underlay_audit::get_audit_log_by_id;

let entry = get_audit_log_by_id(&pool, "platform.audit_log", log_id).await?;
```

### Count Entries

```rust
use underlay_audit::count_audit_logs;

let total = count_audit_logs(&pool, "platform.audit_log", &filters).await?;
```

## App-Specific Extensions

Applications often need to extend audit queries with app-specific joins. For example, joining with user tables to get actor names:

```rust
// App-specific query with actor info
pub async fn list_activity_with_actors(
    pool: &DbPool,
    limit: i64,
    offset: i64,
) -> Result<Vec<ActivityWithActorRow>, sqlx::Error> {
    sqlx::query_as::<_, ActivityWithActorRow>(
        r#"
        SELECT
            a.id,
            a.occurred_at,
            a.user_id,
            u.email AS actor_email,
            u.display_name AS actor_display_name,
            a.action,
            a.resource_type,
            a.resource_id,
            a.details
        FROM platform.audit_log a
        LEFT JOIN auth.users u ON a.user_id = u.id
        ORDER BY a.occurred_at DESC
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}
```

## Best Practices

### What to Audit

- **Always audit**: User authentication, role/permission changes, data deletion, sensitive data access
- **Consider auditing**: Resource creation/updates, configuration changes, exports
- **Don't audit**: Read operations on non-sensitive data (creates noise)

### Correlation IDs

Include request correlation IDs to trace audit entries back to specific API requests:

```rust
AuditEntry {
    correlation_id: Some(request_id.to_string()),
    // ...
}
```

### IP Addresses

Record client IP addresses for security investigations:

```rust
AuditEntry {
    ip_address: Some(client_ip.to_string()),
    // ...
}
```

### Details Field

Use the `details` JSONB field to store context-specific information:

```rust
AuditEntry {
    details: json!({
        "old_value": old_status,
        "new_value": new_status,
        "reason": "User requested status change"
    }),
    // ...
}
```

## Retention and Cleanup

Consider implementing a retention policy for audit logs. The `underlay-jobs` crate doesn't include a purge task for audit logs by default since retention requirements vary by compliance needs.

Example custom cleanup task:

```rust
// Custom task for apps that need to purge old audit logs
pub struct PurgeAuditLogsJob {
    pool: PgPool,
    retention_days: i32,
}

impl PurgeAuditLogsJob {
    pub fn new(pool: PgPool, retention_days: i32) -> Self {
        Self { pool, retention_days }
    }
}

#[async_trait]
impl JobHandler for PurgeAuditLogsJob {
    fn job_type(&self) -> &'static str { "purge_audit_logs" }

    async fn handle(&self, _job: Job) -> Result<(), JobHandlerError> {
        sqlx::query(
            "DELETE FROM platform.audit_log WHERE occurred_at < NOW() - ($1 || ' days')::interval"
        )
        .bind(self.retention_days)
        .execute(&self.pool)
        .await
        .map_err(|e| JobHandlerError::new(e.to_string()))?;
        Ok(())
    }
}
```

## Related Documentation

- [050 - Database & Migrations](./050-database.md) - Database setup
- [055 - Background Jobs](./055-background-jobs.md) - Scheduled cleanup tasks
