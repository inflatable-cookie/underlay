//! Audit log entry types.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Standard audit action types.
///
/// Applications can extend this with custom actions by using `AuditAction::Custom`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditAction {
    /// Resource was created
    Create,
    /// Resource was updated
    Update,
    /// Resource was deleted (soft or hard)
    Delete,
    /// Resource was published/made live
    Publish,
    /// Resource was unpublished/hidden
    Unpublish,
    /// Resource was archived
    Archive,
    /// Resource was restored from archive/soft-delete
    Restore,
    /// Bulk operation affecting multiple resources
    BulkUpdate,
    /// Permission or role was granted
    Grant,
    /// Permission or role was revoked
    Revoke,
    /// User logged in
    Login,
    /// User logged out
    Logout,
    /// Security setting was changed
    SecurityChange,
    /// Custom action (for app-specific events)
    Custom(String),
}

impl AuditAction {
    /// Convert to string representation for database storage.
    pub fn as_str(&self) -> &str {
        match self {
            AuditAction::Create => "create",
            AuditAction::Update => "update",
            AuditAction::Delete => "delete",
            AuditAction::Publish => "publish",
            AuditAction::Unpublish => "unpublish",
            AuditAction::Archive => "archive",
            AuditAction::Restore => "restore",
            AuditAction::BulkUpdate => "bulk_update",
            AuditAction::Grant => "grant",
            AuditAction::Revoke => "revoke",
            AuditAction::Login => "login",
            AuditAction::Logout => "logout",
            AuditAction::SecurityChange => "security_change",
            AuditAction::Custom(s) => s.as_str(),
        }
    }

    /// Parse from string representation.
    pub fn from_str(s: &str) -> Self {
        match s {
            "create" => AuditAction::Create,
            "update" => AuditAction::Update,
            "delete" => AuditAction::Delete,
            "publish" => AuditAction::Publish,
            "unpublish" => AuditAction::Unpublish,
            "archive" => AuditAction::Archive,
            "restore" => AuditAction::Restore,
            "bulk_update" => AuditAction::BulkUpdate,
            "grant" => AuditAction::Grant,
            "revoke" => AuditAction::Revoke,
            "login" => AuditAction::Login,
            "logout" => AuditAction::Logout,
            "security_change" => AuditAction::SecurityChange,
            other => AuditAction::Custom(other.to_string()),
        }
    }
}

impl std::fmt::Display for AuditAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// An audit log entry to be written.
///
/// This is the input type for creating new audit log entries.
#[derive(Debug, Clone)]
pub struct AuditEntry {
    /// The user who performed the action. None for system-initiated actions.
    pub user_id: Option<Uuid>,
    /// The type of action performed.
    pub action: AuditAction,
    /// The type of resource affected (e.g., "pathway", "module", "user").
    pub resource_type: String,
    /// The ID of the affected resource.
    pub resource_id: Uuid,
    /// Additional details about the action (JSON).
    pub details: serde_json::Value,
    /// Request correlation ID for tracing.
    pub correlation_id: Option<String>,
    /// Client IP address (for security auditing).
    pub ip_address: Option<String>,
}

impl AuditEntry {
    /// Create a new audit entry with minimal required fields.
    pub fn new(
        user_id: Option<Uuid>,
        action: AuditAction,
        resource_type: impl Into<String>,
        resource_id: Uuid,
    ) -> Self {
        Self {
            user_id,
            action,
            resource_type: resource_type.into(),
            resource_id,
            details: serde_json::json!({}),
            correlation_id: None,
            ip_address: None,
        }
    }

    /// Add details to the entry.
    pub fn with_details(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }

    /// Add correlation ID.
    pub fn with_correlation_id(mut self, id: impl Into<String>) -> Self {
        self.correlation_id = Some(id.into());
        self
    }

    /// Add IP address.
    pub fn with_ip_address(mut self, ip: impl Into<String>) -> Self {
        self.ip_address = Some(ip.into());
        self
    }
}

/// A stored audit log entry (read from database).
#[derive(Debug, Clone, FromRow, Serialize)]
pub struct AuditLogRow {
    pub id: Uuid,
    pub occurred_at: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub resource_type: String,
    pub resource_id: Uuid,
    pub details: serde_json::Value,
    pub correlation_id: Option<String>,
    pub ip_address: Option<String>,
}

impl AuditLogRow {
    /// Get the action as a typed enum.
    pub fn action_type(&self) -> AuditAction {
        AuditAction::from_str(&self.action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_action_round_trip() {
        let actions = vec![
            AuditAction::Create,
            AuditAction::Update,
            AuditAction::Delete,
            AuditAction::Publish,
            AuditAction::Custom("my_action".to_string()),
        ];

        for action in actions {
            let s = action.as_str();
            let parsed = AuditAction::from_str(s);
            assert_eq!(action, parsed);
        }
    }

    #[test]
    fn audit_entry_builder() {
        let entry = AuditEntry::new(
            Some(Uuid::nil()),
            AuditAction::Create,
            "pathway",
            Uuid::nil(),
        )
        .with_details(serde_json::json!({"title": "Test"}))
        .with_correlation_id("req-123")
        .with_ip_address("192.168.1.1");

        assert_eq!(entry.action, AuditAction::Create);
        assert_eq!(entry.resource_type, "pathway");
        assert_eq!(entry.correlation_id, Some("req-123".to_string()));
    }
}
