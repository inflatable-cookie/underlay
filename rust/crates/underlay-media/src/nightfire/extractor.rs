use underlay_nightfire::NightfireValue;
use uuid::Uuid;

use super::walk::BlockAnchor;
use super::{NightfireBlockMediaHandlerRegistry, NightfireMediaReferenceMatcher};
use crate::domain::{MediaUsageEdgeInput, MediaUsageProvenanceKind};
use crate::error::{MediaError, MediaResult};
use crate::sync::{
    sync_media_usages_for_record, MediaUsageSyncReport, MediaUsageSyncRepository,
    StructuredContentMediaExtractor, StructuredContentWalker,
};

pub struct NightfireMediaUsageExtractor<M> {
    pub(super) used_by_type: String,
    pub(super) used_by_id: Option<Uuid>,
    pub(super) owner_field: String,
    pub(super) provenance_kind: MediaUsageProvenanceKind,
    pub(super) matcher: M,
}

pub struct NightfireBlockMediaUsageExtractor<R> {
    pub(super) used_by_type: String,
    pub(super) used_by_id: Option<Uuid>,
    pub(super) owner_field: String,
    pub(super) provenance_kind: MediaUsageProvenanceKind,
    pub(super) registry: R,
}

impl<R> NightfireBlockMediaUsageExtractor<R> {
    pub fn new(
        used_by_type: impl Into<String>,
        used_by_id: Option<Uuid>,
        owner_field: impl Into<String>,
        provenance_kind: MediaUsageProvenanceKind,
        registry: R,
    ) -> Self {
        Self {
            used_by_type: used_by_type.into(),
            used_by_id,
            owner_field: owner_field.into(),
            provenance_kind,
            registry,
        }
    }

    pub async fn extract_and_sync<S>(
        &self,
        repo: &S,
        value: &NightfireValue,
    ) -> MediaResult<MediaUsageSyncReport>
    where
        S: MediaUsageSyncRepository,
        R: NightfireBlockMediaHandlerRegistry,
    {
        let Some(used_by_id) = self.used_by_id else {
            return Err(MediaError::validation(
                "Nightfire record sync requires a persisted used_by_id".to_string(),
            ));
        };

        let desired = self.extract_media_usages(&self.owner_field, value)?;

        sync_media_usages_for_record(
            repo,
            &self.used_by_type,
            used_by_id,
            &desired,
            &self.provenance_kind,
        )
        .await
    }
}

impl<M> NightfireMediaUsageExtractor<M> {
    pub fn new(
        used_by_type: impl Into<String>,
        used_by_id: Option<Uuid>,
        owner_field: impl Into<String>,
        provenance_kind: MediaUsageProvenanceKind,
        matcher: M,
    ) -> Self {
        Self {
            used_by_type: used_by_type.into(),
            used_by_id,
            owner_field: owner_field.into(),
            provenance_kind,
            matcher,
        }
    }

    /// Extract desired usage edges for one persisted Nightfire-bearing record
    /// and reconcile them through the shared usage-sync path.
    ///
    /// Recommended consumer flow:
    ///
    /// 1. persist a `NightfireValue` with stable block ids
    /// 2. call `extract_and_sync(...)`
    /// 3. later resolve stored locator rows with
    ///    `resolve_nightfire_media_usage(...)`
    pub async fn extract_and_sync<R>(
        &self,
        repo: &R,
        value: &NightfireValue,
    ) -> MediaResult<MediaUsageSyncReport>
    where
        R: MediaUsageSyncRepository,
        M: NightfireMediaReferenceMatcher,
    {
        let Some(used_by_id) = self.used_by_id else {
            return Err(MediaError::validation(
                "Nightfire record sync requires a persisted used_by_id".to_string(),
            ));
        };

        let desired = self.extract_media_usages(&self.owner_field, value)?;

        sync_media_usages_for_record(
            repo,
            &self.used_by_type,
            used_by_id,
            &desired,
            &self.provenance_kind,
        )
        .await
    }
}

impl<M> StructuredContentMediaExtractor<NightfireValue> for NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    fn extract_media_usages(
        &self,
        owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        if owner_field != self.owner_field {
            return Err(MediaError::validation(format!(
                "Nightfire extractor owner_field mismatch: expected {}, got {}",
                self.owner_field, owner_field
            )));
        }

        self.walk_media_usages(owner_field, value)
    }
}

impl<R> StructuredContentMediaExtractor<NightfireValue> for NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    fn extract_media_usages(
        &self,
        owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        if owner_field != self.owner_field {
            return Err(MediaError::validation(format!(
                "Nightfire extractor owner_field mismatch: expected {}, got {}",
                self.owner_field, owner_field
            )));
        }

        self.walk_media_usages(owner_field, value)
    }
}

impl<M> StructuredContentWalker<NightfireValue> for NightfireMediaUsageExtractor<M>
where
    M: NightfireMediaReferenceMatcher,
{
    fn walk_media_usages(
        &self,
        _owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        let mut edges = Vec::new();

        for (index, block) in value.blocks.iter().enumerate() {
            let rooted_pointer = format!("/blocks/{index}/data");
            let anchor = BlockAnchor::from_block(block, rooted_pointer.clone());
            self.walk_block(block, anchor, &rooted_pointer, &block.data, &mut edges)?;
        }

        Ok(edges)
    }
}

impl<R> StructuredContentWalker<NightfireValue> for NightfireBlockMediaUsageExtractor<R>
where
    R: NightfireBlockMediaHandlerRegistry,
{
    fn walk_media_usages(
        &self,
        _owner_field: &str,
        value: &NightfireValue,
    ) -> MediaResult<Vec<MediaUsageEdgeInput>> {
        let mut edges = Vec::new();
        self.walk_root_value_at(value, "", None, &mut edges)?;
        Ok(edges)
    }
}
