use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::identifiers::{MediaId, MediaVersionId};

/// Identity for a migrated source attachment.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MigratedAttachmentIdentity {
    pub source_system: String,
    pub source_attachment_type: String,
    pub source_attachment_id: String,
    pub source_owner_type: String,
    pub source_owner_id: String,
    pub field_or_purpose: String,
}

/// Persisted replay-safe source attachment binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratedAttachmentBinding {
    pub id: Uuid,
    pub identity: MigratedAttachmentIdentity,
    pub sha256: String,
    pub bundle_digest: String,
    pub media_id: MediaId,
    pub media_version_id: MediaVersionId,
    pub import_status: String,
    pub imported_at: DateTime<Utc>,
}

/// Input for storing or updating a migrated attachment binding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigratedAttachmentBindingInput {
    pub identity: MigratedAttachmentIdentity,
    pub sha256: String,
    pub bundle_digest: String,
    pub media_id: MediaId,
    pub media_version_id: MediaVersionId,
    pub import_status: String,
}
