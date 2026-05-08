//! Shared media usage sync and extraction contracts.
//!
//! This module provides additive shared surfaces for generalized media usage
//! edges without forcing existing repositories to migrate in one step.

use std::collections::{HashMap, HashSet};

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{
    MediaUsageEdge, MediaUsageEdgeInput, MediaUsageEdgeKey, MediaUsageFieldPayload,
    MediaUsageProvenanceKind, MigratedAttachmentBinding, MigratedAttachmentBindingInput,
    MigratedAttachmentIdentity,
};
use crate::error::{MediaError, MediaResult};

/// Repository surface for media usage-edge sync.
#[async_trait]
pub trait MediaUsageSyncRepository: Send + Sync {
    /// List existing usage edges for one owner/provenance scope.
    async fn list_usage_edges_for_owner(
        &self,
        used_by_type: &str,
        used_by_id: Uuid,
        provenance_kind: &MediaUsageProvenanceKind,
    ) -> MediaResult<Vec<MediaUsageEdge>>;

    /// Persist a desired usage edge.
    async fn upsert_usage_edge(&self, usage: &MediaUsageEdgeInput) -> MediaResult<()>;

    /// Remove one usage edge by natural key.
    async fn remove_usage_edge(&self, key: &MediaUsageEdgeKey) -> MediaResult<bool>;
}

/// Repository surface for migration attachment binding reuse.
#[async_trait]
pub trait MigrationAttachmentBindingRepository: Send + Sync {
    async fn find_migrated_attachment_binding(
        &self,
        identity: &MigratedAttachmentIdentity,
        sha256: &str,
    ) -> MediaResult<Option<MigratedAttachmentBinding>>;

    async fn upsert_migrated_attachment_binding(
        &self,
        input: &MigratedAttachmentBindingInput,
    ) -> MediaResult<MigratedAttachmentBinding>;
}

/// Generic structured-content extractor.
pub trait StructuredContentMediaExtractor<T>: Send + Sync {
    fn extract_media_usages(
        &self,
        owner_field: &str,
        value: &T,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>>;
}

/// Generic structured-content walker.
pub trait StructuredContentWalker<T>: Send + Sync {
    fn walk_media_usages(
        &self,
        owner_field: &str,
        value: &T,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>>;
}

/// Audit source for managed media-bearing fields.
#[async_trait]
pub trait MediaUsageAuditSource: Send + Sync {
    async fn list_field_payloads_for_owner(
        &self,
        used_by_type: &str,
        used_by_id: Uuid,
    ) -> MediaResult<Vec<MediaUsageFieldPayload>>;
}

/// Summary of a usage sync operation.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MediaUsageSyncReport {
    pub inserted: usize,
    pub retained: usize,
    pub removed: usize,
}

/// Sync desired usage edges for one owner/provenance scope.
pub async fn sync_media_usages_for_record<R>(
    repo: &R,
    used_by_type: &str,
    used_by_id: Uuid,
    desired: &[MediaUsageEdgeInput],
    provenance_kind: &MediaUsageProvenanceKind,
) -> MediaResult<MediaUsageSyncReport>
where
    R: MediaUsageSyncRepository,
{
    validate_desired_scope(used_by_type, used_by_id, desired, provenance_kind)?;

    let existing = repo
        .list_usage_edges_for_owner(used_by_type, used_by_id, provenance_kind)
        .await?;

    let mut existing_by_key: HashMap<MediaUsageEdgeKey, MediaUsageEdge> = existing
        .into_iter()
        .map(|edge| (edge.key(), edge))
        .collect();
    let desired_keys: HashSet<MediaUsageEdgeKey> = desired.iter().map(|edge| edge.key()).collect();

    let mut report = MediaUsageSyncReport::default();

    for edge in desired {
        let key = edge.key();
        if existing_by_key.remove(&key).is_some() {
            report.retained += 1;
        } else {
            repo.upsert_usage_edge(edge).await?;
            report.inserted += 1;
        }
    }

    for key in existing_by_key.keys() {
        if !desired_keys.contains(key) {
            repo.remove_usage_edge(key).await?;
            report.removed += 1;
        }
    }

    Ok(report)
}

fn validate_desired_scope(
    used_by_type: &str,
    used_by_id: Uuid,
    desired: &[MediaUsageEdgeInput],
    provenance_kind: &MediaUsageProvenanceKind,
) -> MediaResult<()> {
    for edge in desired {
        if edge.used_by_type != used_by_type {
            return Err(MediaError::validation(format!(
                "usage edge scope mismatch: expected used_by_type={}, got {}",
                used_by_type, edge.used_by_type
            )));
        }

        if edge.used_by_id != Some(used_by_id) {
            return Err(MediaError::validation(format!(
                "usage edge scope mismatch: expected used_by_id={}, got {:?}",
                used_by_id, edge.used_by_id
            )));
        }

        if &edge.provenance_kind != provenance_kind {
            return Err(MediaError::validation(format!(
                "usage edge scope mismatch: expected provenance_kind={}, got {}",
                provenance_kind.as_str(),
                edge.provenance_kind.as_str()
            )));
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "tests/sync_tests.rs"]
mod tests;
