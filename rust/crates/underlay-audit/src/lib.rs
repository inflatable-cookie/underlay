//! Audit logging for admin actions and security-relevant events.
//!
//! This crate provides the shared audit row, filter, and query contract for
//! recording administrative actions. The consuming application still owns the
//! concrete schema/table location. Audit logs are essential for:
//!
//! - Security forensics (who did what, when)
//! - Compliance requirements (SOC 2, GDPR audit trails)
//! - Debugging and troubleshooting
//! - Change tracking and accountability
//!
//! # Usage
//!
//! ```rust,ignore
//! use underlay_audit::{append_audit_log_to_table, AuditAction, AuditEntry, AuditTable};
//!
//! let audit_table = AuditTable::parse("platform.audit_log")?;
//!
//! // Log an admin action
//! append_audit_log_to_table(
//!     &pool,
//!     &audit_table,
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
//! The consuming application must create an audit log table. Underlay does not
//! own a fixed shared table location here; it owns the row/query semantics over
//! an app-supplied fully qualified table name. Example migration:
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
mod error;
mod query;
mod tables;
mod writer;

pub use crate::entry::{AuditAction, AuditEntry, AuditLogRow};
pub use crate::error::{AuditError, AuditResult};
pub use crate::query::{
    count_audit_logs_from_table, get_audit_log_by_id_from_table, list_audit_logs_from_table,
    AuditLogFilters,
};
pub use crate::tables::AuditTable;
pub use crate::writer::{append_audit_log_to_table, append_audit_log_to_table_async};

/// Convenience type alias for the database pool.
pub type DbPool = sqlx::PgPool;
