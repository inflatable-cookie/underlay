use std::sync::{Arc, Mutex};

use chrono::Utc;
use serde_json::json;
use underlay_nightfire::NightfireValue;
use uuid::Uuid;

use super::support::{block, TestUsageSyncRepository};
use crate::nightfire::{NightfireFieldNameMatcher, NightfireMediaUsageExtractor};
use crate::{
    MediaContentKind, MediaId, MediaLocatorKind, MediaUsageEdge, MediaUsageProvenanceKind,
    MediaUsageRole,
};

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
            locator_key: "hero_01#/image_id".to_string(),
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
                "image_id": "019473c4-4a6d-7000-8000-000000000122"
            }),
        ),
    );

    let report = extractor.extract_and_sync(&repo, &value).await.unwrap();

    assert_eq!(report.inserted, 1);
    assert_eq!(report.retained, 0);
    assert_eq!(report.removed, 1);

    let upserts = repo.upserts.lock().unwrap();
    assert_eq!(upserts.len(), 1);
    assert_eq!(upserts[0].locator_key, "hero_01#/image_id");
    assert_eq!(upserts[0].usage_role, MediaUsageRole::Embedded);
    assert_eq!(
        repo.removals.lock().unwrap().as_slice(),
        ["hero_01#/image_id"]
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
                "image_id": "019473c4-4a6d-7000-8000-000000000123"
            }),
        ),
    );

    let error = extractor.extract_and_sync(&repo, &value).await.unwrap_err();
    assert!(error.to_string().contains("persisted used_by_id"));
}
