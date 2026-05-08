use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use chrono::Utc;
use uuid::Uuid;

use crate::domain::{
    MediaContentKind, MediaId, MediaLocatorKind, MediaUsageEdge, MediaUsageEdgeInput,
    MediaUsageProvenanceKind, MediaUsageRole,
};
use crate::error::MediaResult;
use crate::sync::{sync_media_usages_for_record, MediaUsageSyncRepository};

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

fn usage_input(locator_key: &str, media_id: MediaId) -> MediaUsageEdgeInput {
    MediaUsageEdgeInput {
        media_id,
        used_by_type: "blog_article".to_string(),
        used_by_id: Some(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000010").unwrap()),
        owner_field: Some("body_blocks".to_string()),
        content_kind: MediaContentKind::StructuredContent,
        locator_kind: MediaLocatorKind::BlockId,
        locator_key: locator_key.to_string(),
        usage_role: MediaUsageRole::Embedded,
        provenance_kind: MediaUsageProvenanceKind::ContentSync,
    }
}

fn usage_edge(id_suffix: u128, locator_key: &str, media_id: MediaId) -> MediaUsageEdge {
    MediaUsageEdge {
        id: Uuid::from_u128(id_suffix),
        media_id,
        used_by_type: "blog_article".to_string(),
        used_by_id: Some(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000010").unwrap()),
        owner_field: Some("body_blocks".to_string()),
        content_kind: MediaContentKind::StructuredContent,
        locator_kind: MediaLocatorKind::BlockId,
        locator_key: locator_key.to_string(),
        usage_role: MediaUsageRole::Embedded,
        provenance_kind: MediaUsageProvenanceKind::ContentSync,
        created_at: Utc::now(),
    }
}

#[tokio::test]
async fn sync_media_usages_inserts_and_removes_by_edge_key() {
    let existing_media =
        MediaId::from_uuid(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000001").unwrap());
    let desired_media =
        MediaId::from_uuid(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000002").unwrap());

    let repo = TestUsageSyncRepository {
        existing: Arc::new(Mutex::new(vec![usage_edge(
            1,
            "block-old:image",
            existing_media,
        )])),
        ..Default::default()
    };

    let desired = vec![usage_input("block-new:image", desired_media)];
    let owner_id = Uuid::parse_str("019473c4-4a6d-7000-8000-000000000010").unwrap();

    let report = sync_media_usages_for_record(
        &repo,
        "blog_article",
        owner_id,
        &desired,
        &MediaUsageProvenanceKind::ContentSync,
    )
    .await
    .unwrap();

    assert_eq!(report.inserted, 1);
    assert_eq!(report.retained, 0);
    assert_eq!(report.removed, 1);
    assert_eq!(repo.upserts.lock().unwrap().len(), 1);
    assert_eq!(
        repo.removals.lock().unwrap().as_slice(),
        ["block-old:image"]
    );
}

#[tokio::test]
async fn sync_media_usages_retains_matching_edge() {
    let media_id =
        MediaId::from_uuid(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000003").unwrap());
    let repo = TestUsageSyncRepository {
        existing: Arc::new(Mutex::new(vec![usage_edge(2, "block-a:image", media_id)])),
        ..Default::default()
    };
    let desired = vec![usage_input("block-a:image", media_id)];
    let owner_id = Uuid::parse_str("019473c4-4a6d-7000-8000-000000000010").unwrap();

    let report = sync_media_usages_for_record(
        &repo,
        "blog_article",
        owner_id,
        &desired,
        &MediaUsageProvenanceKind::ContentSync,
    )
    .await
    .unwrap();

    assert_eq!(report.inserted, 0);
    assert_eq!(report.retained, 1);
    assert_eq!(report.removed, 0);
    assert!(repo.upserts.lock().unwrap().is_empty());
    assert!(repo.removals.lock().unwrap().is_empty());
}

#[tokio::test]
async fn sync_media_usages_rejects_scope_mismatch() {
    let repo = TestUsageSyncRepository::default();
    let mut desired = vec![usage_input(
        "block-a:image",
        MediaId::from_uuid(Uuid::parse_str("019473c4-4a6d-7000-8000-000000000004").unwrap()),
    )];
    desired[0].used_by_type = "content_document".to_string();
    let owner_id = Uuid::parse_str("019473c4-4a6d-7000-8000-000000000010").unwrap();

    let error = sync_media_usages_for_record(
        &repo,
        "blog_article",
        owner_id,
        &desired,
        &MediaUsageProvenanceKind::ContentSync,
    )
    .await
    .unwrap_err();

    assert!(error.to_string().contains("scope mismatch"));
}
