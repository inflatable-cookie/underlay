use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use super::identifiers::MediaId;

/// Broad source shape for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaContentKind {
    RecordField,
    StructuredContent,
    External,
    #[serde(untagged)]
    Custom(String),
}

impl MediaContentKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaContentKind::RecordField => "record_field",
            MediaContentKind::StructuredContent => "structured_content",
            MediaContentKind::External => "external",
            MediaContentKind::Custom(value) => value.as_str(),
        }
    }
}

/// Stable locator type for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaLocatorKind {
    Field,
    /// Canonical Nightfire form: `<block-id>#<json-pointer-relative-to-block.data>`.
    BlockId,
    /// Fallback form: JSON Pointer rooted at the stored field value.
    Path,
    ExternalRef,
    #[serde(untagged)]
    Custom(String),
}

impl MediaLocatorKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaLocatorKind::Field => "field",
            MediaLocatorKind::BlockId => "block_id",
            MediaLocatorKind::Path => "path",
            MediaLocatorKind::ExternalRef => "external_ref",
            MediaLocatorKind::Custom(value) => value.as_str(),
        }
    }
}

/// Semantic role for a media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUsageRole {
    Primary,
    Attachment,
    Embedded,
    External,
    Derived,
    #[serde(untagged)]
    Custom(String),
}

impl MediaUsageRole {
    pub fn as_str(&self) -> &str {
        match self {
            MediaUsageRole::Primary => "primary",
            MediaUsageRole::Attachment => "attachment",
            MediaUsageRole::Embedded => "embedded",
            MediaUsageRole::External => "external",
            MediaUsageRole::Derived => "derived",
            MediaUsageRole::Custom(value) => value.as_str(),
        }
    }
}

/// Source-of-truth lane responsible for a usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaUsageProvenanceKind {
    ContentSync,
    LegacyMigration,
    Manual,
    SystemGenerated,
    #[serde(untagged)]
    Custom(String),
}

impl MediaUsageProvenanceKind {
    pub fn as_str(&self) -> &str {
        match self {
            MediaUsageProvenanceKind::ContentSync => "content_sync",
            MediaUsageProvenanceKind::LegacyMigration => "legacy_migration",
            MediaUsageProvenanceKind::Manual => "manual",
            MediaUsageProvenanceKind::SystemGenerated => "system_generated",
            MediaUsageProvenanceKind::Custom(value) => value.as_str(),
        }
    }
}

/// Natural key for a media usage edge inside one managed scope.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MediaUsageEdgeKey {
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub provenance_kind: MediaUsageProvenanceKind,
}

/// Desired or persisted media usage edge without persistence metadata.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaUsageEdgeInput {
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub content_kind: MediaContentKind,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub usage_role: MediaUsageRole,
    pub provenance_kind: MediaUsageProvenanceKind,
}

impl MediaUsageEdgeInput {
    pub fn key(&self) -> MediaUsageEdgeKey {
        MediaUsageEdgeKey {
            media_id: self.media_id,
            used_by_type: self.used_by_type.clone(),
            used_by_id: self.used_by_id,
            owner_field: self.owner_field.clone(),
            locator_kind: self.locator_kind.clone(),
            locator_key: self.locator_key.clone(),
            provenance_kind: self.provenance_kind.clone(),
        }
    }
}

/// Persisted media usage edge.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaUsageEdge {
    pub id: Uuid,
    pub media_id: MediaId,
    pub used_by_type: String,
    pub used_by_id: Option<Uuid>,
    pub owner_field: Option<String>,
    pub content_kind: MediaContentKind,
    pub locator_kind: MediaLocatorKind,
    pub locator_key: String,
    pub usage_role: MediaUsageRole,
    pub provenance_kind: MediaUsageProvenanceKind,
    pub created_at: DateTime<Utc>,
}

impl MediaUsageEdge {
    pub fn key(&self) -> MediaUsageEdgeKey {
        MediaUsageEdgeKey {
            media_id: self.media_id,
            used_by_type: self.used_by_type.clone(),
            used_by_id: self.used_by_id,
            owner_field: self.owner_field.clone(),
            locator_kind: self.locator_kind.clone(),
            locator_key: self.locator_key.clone(),
            provenance_kind: self.provenance_kind.clone(),
        }
    }
}

/// A managed field payload that should be scanned for media references.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MediaUsageFieldPayload {
    pub owner_field: String,
    pub content_kind: MediaContentKind,
    pub value: Value,
}
