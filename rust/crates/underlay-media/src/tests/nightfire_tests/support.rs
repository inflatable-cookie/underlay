use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use underlay_nightfire::BlockData;
use uuid::Uuid;

use crate::domain::MediaUsageEdgeKey;
use crate::error::MediaResult;
use crate::nightfire::{
    NightfireBlockMediaHandler, NightfireBlockMediaReference, NightfireFieldNameMatcher,
    NightfireMediaFieldRule, NightfireMediaVisitContext, NightfireNestedValuePointer,
};
use crate::sync::MediaUsageSyncRepository;
use crate::{
    MediaId, MediaUsageEdge, MediaUsageEdgeInput, MediaUsageProvenanceKind, MediaUsageRole,
};

pub(crate) fn block(id: Option<&str>, data: serde_json::Value) -> BlockData {
    BlockData {
        id: id.map(ToOwned::to_owned).unwrap_or_default(),
        r#type: "test".to_string(),
        version: "initial".to_string(),
        data,
    }
}

pub(crate) fn matcher() -> NightfireFieldNameMatcher {
    NightfireFieldNameMatcher::empty()
        .with_rule(NightfireMediaFieldRule::new(
            "imageId",
            MediaUsageRole::Embedded,
        ))
        .with_rule(NightfireMediaFieldRule::new(
            "iconMediaId",
            MediaUsageRole::Primary,
        ))
}

pub(crate) struct HeroBlockHandler;

impl NightfireBlockMediaHandler for HeroBlockHandler {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireBlockMediaReference>> {
        let Some(raw) = context.resolve_relative_pointer("/imageId") else {
            return Ok(Vec::new());
        };
        let Some(media_id) = raw
            .as_str()
            .and_then(|value| Uuid::parse_str(value).ok())
            .map(MediaId::from_uuid)
        else {
            return Ok(Vec::new());
        };

        Ok(vec![NightfireBlockMediaReference::new(
            media_id,
            MediaUsageRole::Embedded,
            "/imageId",
        )])
    }
}

pub(crate) struct PopupBlockHandler;

impl NightfireBlockMediaHandler for PopupBlockHandler {
    fn extract_media_references(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireBlockMediaReference>> {
        Ok(Vec::new())
    }

    fn nested_nightfire_values(
        &self,
        _context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireNestedValuePointer>> {
        Ok(vec![NightfireNestedValuePointer::new("/content")])
    }
}

pub(crate) struct MediaBlockHandler;

impl NightfireBlockMediaHandler for MediaBlockHandler {
    fn extract_media_references(
        &self,
        context: &NightfireMediaVisitContext<'_>,
    ) -> MediaResult<Vec<NightfireBlockMediaReference>> {
        let Some(raw) = context.resolve_relative_pointer("/mediaId") else {
            return Ok(Vec::new());
        };
        let Some(media_id) = raw
            .as_str()
            .and_then(|value| Uuid::parse_str(value).ok())
            .map(MediaId::from_uuid)
        else {
            return Ok(Vec::new());
        };

        Ok(vec![NightfireBlockMediaReference::new(
            media_id,
            MediaUsageRole::Embedded,
            "/mediaId",
        )])
    }
}

#[derive(Clone, Default)]
pub(crate) struct TestUsageSyncRepository {
    pub(crate) existing: Arc<Mutex<Vec<MediaUsageEdge>>>,
    pub(crate) upserts: Arc<Mutex<Vec<MediaUsageEdgeInput>>>,
    pub(crate) removals: Arc<Mutex<Vec<String>>>,
}

#[async_trait]
impl MediaUsageSyncRepository for TestUsageSyncRepository {
    async fn list_usage_edges_for_owner(
        &self,
        _used_by_type: &str,
        _used_by_id: Uuid,
        _provenance_kind: &MediaUsageProvenanceKind,
    ) -> MediaResult<Vec<MediaUsageEdge>> {
        Ok(self.existing.lock().unwrap().clone())
    }

    async fn upsert_usage_edge(&self, usage: &MediaUsageEdgeInput) -> MediaResult<()> {
        self.upserts.lock().unwrap().push(usage.clone());
        Ok(())
    }

    async fn remove_usage_edge(&self, key: &MediaUsageEdgeKey) -> MediaResult<bool> {
        self.removals.lock().unwrap().push(key.locator_key.clone());
        Ok(true)
    }
}
