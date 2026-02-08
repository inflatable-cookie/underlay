//! Audit logging for admin actions and security-relevant events.
//!
//! This crate provides types and database functions for recording audit trails
//! of administrative actions. Audit logs are essential for:
//!
//! - Security forensics (who did what, when)
//! - Compliance requirements (SOC 2, GDPR audit trails)
//! - Debugging and troubleshooting
//! - Change tracking and accountability
//!
//! # Usage
//!
//! ```rust,ignore
//! use underlay_audit::{AuditAction, AuditEntry, append_audit_log};
//!
//! // Log an admin action
//! append_audit_log(
//!     &pool,
//!     "platform.audit_log",  // table name (app-configurable)
//!     AuditEntry {
//!         user_id: Some(user.id),
//!         action: AuditAction::Create,
//!         resource_type: "pathway".to_string(),
//!         resource_id: pathway_id,
//!         details: serde_json::json!({ "title": "New Pathway" }),
//!         correlation_id: Some(request_id.to_string()),
//!         ip_address: Some(client_ip),
//!     },
//! ).await?;
//! ```
//!
//! # Schema
//!
//! The consuming application must create an audit log table. Example migration:
//!
//! ```sql
//! CREATE TABLE platform.audit_log (
//!     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
//!     occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
//!     user_id UUID,  -- NULL for system actions
//!     action TEXT NOT NULL,
//!     resource_type TEXT NOT NULL,
//!     resource_id UUID NOT NULL,
//!     details JSONB NOT NULL DEFAULT '{}',
//!     correlation_id TEXT,
//!     ip_address TEXT
//! );
//!
//! CREATE INDEX idx_audit_log_occurred_at ON platform.audit_log (occurred_at DESC);
//! CREATE INDEX idx_audit_log_user_id ON platform.audit_log (user_id);
//! CREATE INDEX idx_audit_log_resource ON platform.audit_log (resource_type, resource_id);
//! CREATE INDEX idx_audit_log_action ON platform.audit_log (action);
//! ```

mod entry;
mod query;
mod writer;

pub use crate::entry::{AuditAction, AuditEntry, AuditLogRow};
pub use crate::query::{count_audit_logs, get_audit_log_by_id, list_audit_logs, AuditLogFilters};
pub use crate::writer::{append_audit_log, append_audit_log_async};

/// Convenience type alias for the database pool.
pub type DbPool = sqlx::PgPool;

/// Validate that a table name is safe for use in dynamic SQL.
///
/// Only allows alphanumeric characters, underscores, and dots (for schema.table).
/// Rejects any characters that could enable SQL injection.
pub(crate) fn validate_table_name(table: &str) -> Result<(), sqlx::Error> {
    if !table
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '.')
    {
        return Err(sqlx::Error::Protocol(
            "Invalid table name: must contain only alphanumeric, underscore, or dot".to_string(),
        ));
    }
    Ok(())
}
