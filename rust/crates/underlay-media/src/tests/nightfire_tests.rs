use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use serde_json::json;
use underlay_nightfire::{BlockData, NightfireValue};
use uuid::Uuid;

use crate::error::MediaResult;
use crate::nightfire::{
    resolve_nightfire_media_usage, NightfireBlockMediaHandler, NightfireBlockMediaHandlerMap,
    NightfireBlockMediaHandlerRegistry, NightfireBlockMediaReference,
    NightfireBlockMediaRegistration, NightfireBlockMediaUsageExtractor, NightfireFieldNameMatcher,
    NightfireMediaFieldRule, NightfireMediaUsageExtractor, NightfireMediaVisitContext,
    NightfireNestedValuePointer,
};
use crate::sync::{MediaUsageSyncRepository, StructuredContentMediaExtractor};
use crate::{
    MediaContentKind, MediaId, MediaLocatorKind, MediaUsageEdge, MediaUsageEdgeInput,
    MediaUsageProvenanceKind, MediaUsageRole,
};

fn block(id: Option<&str>, data: serde_json::Value) -> BlockData {
    BlockData {
        id: id.map(ToOwned::to_owned),
        r#type: "test".to_string(),
        version: "initial".to_string(),
        hash: "abc123".to_string(),
        data,
    }
}

fn matcher() -> NightfireFieldNameMatcher {
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

struct HeroBlockHandler;

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

struct PopupBlockHandler;

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

struct MediaBlockHandler;

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
struct TestUsageSyncRepository {
    existing: Arc<Mutex<Vec<MediaUsageEdge>>>,
    upserts: Arc<Mutex<Vec<MediaUsageEdgeInput>>>,
    removals: Arc<Mutex<Vec<String>>>,
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

    async fn remove_usage_edge(&self, key: &crate::domain::MediaUsageEdgeKey) -> MediaResult<bool> {
        self.removals.lock().unwrap().push(key.locator_key.clone());
        Ok(true)
    }
}

#[test]
fn extracts_block_id_locators_for_top_level_and_nested_block_ids() {
    let used_by_id = Uuid::now_v7();
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(used_by_id),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000010",
                "children": [
                    {
                        "id": "gallery_02",
                        "type": "gallery",
                        "version": "initial",
                        "hash": "def456",
                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000011" }]
                        }
                    }
                ]
            }),
        ),
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 2);
    assert!(usages
        .iter()
        .all(|usage| usage.locator_kind == MediaLocatorKind::BlockId));

    let mut locator_keys = usages
        .iter()
        .map(|usage| usage.locator_key.as_str())
        .collect::<Vec<_>>();
    locator_keys.sort_unstable();

    assert_eq!(
        locator_keys,
        vec!["gallery_02#/pages/0/imageId", "hero_01#/imageId"]
    );
}

#[test]
fn falls_back_to_ancestor_block_pointer_when_nested_block_has_no_id() {
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "children": [
                    {
                        "type": "gallery",
                        "version": "initial",
                        "hash": "def456",
                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000012" }]
                        }
                    }
                ]
            }),
        ),
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::BlockId);
    assert_eq!(
        usages[0].locator_key,
        "hero_01#/children/0/data/pages/0/imageId"
    );
}

#[test]
fn falls_back_to_rooted_path_when_top_level_block_has_no_id() {
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        matcher(),
    );

    let value = NightfireValue::multi(
        "test:schema@1",
        vec![block(
            None,
            json!({
                "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000013" }]
            }),
        )],
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::Path);
    assert_eq!(usages[0].locator_key, "/blocks/0/data/pages/0/imageId");
}

#[test]
fn common_field_matcher_covers_default_media_reference_names() {
    let matcher = NightfireFieldNameMatcher::with_common_media_fields();
    let rules = matcher
        .rules()
        .iter()
        .map(|rule| (rule.field_name.as_str(), rule.usage_role.as_str()))
        .collect::<Vec<_>>();

    assert!(rules.contains(&("imageId", "embedded")));
    assert!(rules.contains(&("mediaId", "embedded")));
    assert!(rules.contains(&("iconMediaId", "primary")));
    assert!(rules.contains(&("fileId", "attachment")));
    assert!(rules.contains(&("attachmentId", "attachment")));
}

#[test]
fn registry_backed_extractor_walks_block_handlers_and_declared_nested_values() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        NightfireBlockMediaRegistration::new("popup", PopupBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    let extractor = NightfireBlockMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        registry,
    );

    let nested_content = NightfireValue::single(
        "test:nested@1",
        BlockData {
            id: Some("popup_media_01".to_string()),
            r#type: "media".to_string(),
            version: "initial".to_string(),
            hash: "media123".to_string(),
            data: json!({
                "mediaId": "019473c4-4a6d-7000-8000-000000000214"
            }),
        },
    );

    let value = NightfireValue::single(
        "test:schema@1",
        BlockData {
            id: Some("hero_01".to_string()),
            r#type: "hero".to_string(),
            version: "initial".to_string(),
            hash: "hero123".to_string(),
            data: json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000213",
                "children": [
                    {
                        "id": "popup_01",
                        "type": "popup",
                        "version": "initial",
                        "hash": "popup123",
                        "data": {
                            "content": serde_json::to_value(&nested_content).unwrap()
                        }
                    }
                ]
            }),
        },
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 2);

    let mut locator_keys = usages
        .iter()
        .map(|usage| usage.locator_key.as_str())
        .collect::<Vec<_>>();
    locator_keys.sort_unstable();

    assert_eq!(
        locator_keys,
        vec!["hero_01#/imageId", "popup_media_01#/mediaId"]
    );
}

#[test]
fn registry_backed_extractor_falls_back_to_outer_anchor_for_nested_child_without_ids() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("popup", PopupBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    let extractor = NightfireBlockMediaUsageExtractor::new(
        "lesson",
        Some(Uuid::now_v7()),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        registry,
    );

    let nested_content = NightfireValue::single(
        "test:nested@1",
        BlockData {
            id: None,
            r#type: "media".to_string(),
            version: "initial".to_string(),
            hash: "media123".to_string(),
            data: json!({
                "mediaId": "019473c4-4a6d-7000-8000-000000000215"
            }),
        },
    );

    let value = NightfireValue::single(
        "test:schema@1",
        BlockData {
            id: Some("popup_01".to_string()),
            r#type: "popup".to_string(),
            version: "initial".to_string(),
            hash: "popup123".to_string(),
            data: json!({
                "content": serde_json::to_value(&nested_content).unwrap()
            }),
        },
    );

    let usages = extractor
        .extract_media_usages("body_blocks", &value)
        .unwrap();

    assert_eq!(usages.len(), 1);
    assert_eq!(usages[0].locator_kind, MediaLocatorKind::BlockId);
    assert_eq!(
        usages[0].locator_key,
        "popup_01#/content/block/data/mediaId"
    );
}

#[test]
fn handler_map_accepts_block_module_registrations() {
    let registry = NightfireBlockMediaHandlerMap::from_registrations(vec![
        NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        NightfireBlockMediaRegistration::new("media", MediaBlockHandler),
    ]);

    assert!(registry.handler_for("hero").is_some());
    assert!(registry.handler_for("media").is_some());
    assert!(registry.handler_for("popup").is_none());
}

#[test]
fn handler_map_accepts_generic_block_registrations() {
    #[derive(Clone, Copy)]
    enum TestCategory {
        Content,
    }

    let registry = NightfireBlockMediaHandlerMap::from_block_registrations([
        underlay_nightfire::BlockRegistration::new(
            underlay_nightfire::BlockDescriptor {
                type_name: "hero",
                label: "Hero",
                category: TestCategory::Content,
            },
            NightfireBlockMediaRegistration::new("hero", HeroBlockHandler),
        ),
    ]);

    assert!(registry.handler_for("hero").is_some());
}

#[test]
fn resolve_nightfire_media_usage_reads_block_id_locator_values() {
    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000210",
                "children": [
                    {
                        "id": "gallery_02",
                        "type": "gallery",
                        "version": "initial",
                        "hash": "def456",
                        "data": {
                            "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000211" }]
                        }
                    }
                ]
            }),
        ),
    );

    let top_level =
        resolve_nightfire_media_usage(&value, &MediaLocatorKind::BlockId, "hero_01#/imageId");
    let nested = resolve_nightfire_media_usage(
        &value,
        &MediaLocatorKind::BlockId,
        "gallery_02#/pages/0/imageId",
    );

    assert_eq!(
        top_level,
        Some(json!("019473c4-4a6d-7000-8000-000000000210"))
    );
    assert_eq!(nested, Some(json!("019473c4-4a6d-7000-8000-000000000211")));
}

#[test]
fn resolve_nightfire_media_usage_reads_path_fallback_values() {
    let value = NightfireValue::multi(
        "test:schema@1",
        vec![block(
            None,
            json!({
                "pages": [{ "imageId": "019473c4-4a6d-7000-8000-000000000212" }]
            }),
        )],
    );

    let resolved = resolve_nightfire_media_usage(
        &value,
        &MediaLocatorKind::Path,
        "/blocks/0/data/pages/0/imageId",
    );

    assert_eq!(
        resolved,
        Some(json!("019473c4-4a6d-7000-8000-000000000212"))
    );
}

#[tokio::test]
async fn extract_and_sync_composes_shared_nightfire_extractor_with_usage_sync() {
    let used_by_id = Uuid::parse_str("019473c4-4a6d-7000-8000-000000000120").unwrap();
    let existing_media =
        MediaId::from_uuid(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000121").unwrap());
    let repo = TestUsageSyncRepository {
        existing: Arc::new(Mutex::new(vec![MediaUsageEdge {
            id: Uuid::from_u128(1),
            media_id: existing_media,
            used_by_type: "lesson".to_string(),
            used_by_id: Some(used_by_id),
            owner_field: Some("body_blocks".to_string()),
            content_kind: MediaContentKind::StructuredContent,
            locator_kind: MediaLocatorKind::BlockId,
            locator_key: "hero_01#/imageId".to_string(),
            usage_role: MediaUsageRole::Embedded,
            provenance_kind: MediaUsageProvenanceKind::ContentSync,
            created_at: Utc::now(),
        }])),
        ..Default::default()
    };

    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        Some(used_by_id),
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        NightfireFieldNameMatcher::with_common_media_fields(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000122"
            }),
        ),
    );

    let report = extractor.extract_and_sync(&repo, &value).await.unwrap();

    assert_eq!(report.inserted, 1);
    assert_eq!(report.retained, 0);
    assert_eq!(report.removed, 1);

    let upserts = repo.upserts.lock().unwrap();
    assert_eq!(upserts.len(), 1);
    assert_eq!(upserts[0].locator_key, "hero_01#/imageId");
    assert_eq!(upserts[0].usage_role, MediaUsageRole::Embedded);
    assert_eq!(
        repo.removals.lock().unwrap().as_slice(),
        ["hero_01#/imageId"]
    );
}

#[tokio::test]
async fn extract_and_sync_requires_persisted_owner_id() {
    let repo = TestUsageSyncRepository::default();
    let extractor = NightfireMediaUsageExtractor::new(
        "lesson",
        None,
        "body_blocks",
        MediaUsageProvenanceKind::ContentSync,
        NightfireFieldNameMatcher::with_common_media_fields(),
    );

    let value = NightfireValue::single(
        "test:schema@1",
        block(
            Some("hero_01"),
            json!({
                "imageId": "019473c4-4a6d-7000-8000-000000000123"
            }),
        ),
    );

    let error = extractor.extract_and_sync(&repo, &value).await.unwrap_err();
    assert!(error.to_string().contains("persisted used_by_id"));
}
