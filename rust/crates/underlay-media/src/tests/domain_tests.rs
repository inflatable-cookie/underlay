use super::*;

#[test]
fn test_media_id_creation() {
    let id1 = MediaId::new();
    let id2 = MediaId::new();
    assert_ne!(id1, id2);
}

#[test]
fn test_media_id_from_uuid() {
    let uuid = Uuid::now_v7();
    let id = MediaId::from_uuid(uuid);
    assert_eq!(id.into_inner(), uuid);
}

#[test]
fn test_media_id_serialization() {
    let uuid = Uuid::parse_str("019473c4-4a6d-7000-8000-000000000001").unwrap();
    let id = MediaId(uuid);
    let json = serde_json::to_string(&id).unwrap();
    assert_eq!(json, "\"019473c4-4a6d-7000-8000-000000000001\"");
}

#[test]
fn test_rendition_type_from_str() {
    assert_eq!(RenditionType::from("thumbnail"), RenditionType::Thumbnail);
    assert_eq!(RenditionType::from("preview"), RenditionType::Preview);
    assert_eq!(
        RenditionType::from("custom-size"),
        RenditionType::Custom("custom-size".to_string())
    );
}

#[test]
fn test_rendition_type_as_str() {
    assert_eq!(RenditionType::Thumbnail.as_str(), "thumbnail");
    assert_eq!(RenditionType::Preview.as_str(), "preview");
    assert_eq!(RenditionType::Custom("hero".to_string()).as_str(), "hero");
}

#[test]
fn test_media_is_deleted() {
    let media = Media {
        id: MediaId::new(),
        kind: MediaKind::Image,
        visibility: MediaVisibility::Public,
        title: "Test".to_string(),
        original_filename: None,
        alt_text: None,
        current_version_id: None,
        deleted_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        created_by: None,
    };
    assert!(!media.is_deleted());

    let deleted_media = Media {
        deleted_at: Some(Utc::now()),
        ..media
    };
    assert!(deleted_media.is_deleted());
}

#[test]
fn test_media_version_state_checks() {
    let version = MediaVersion {
        id: MediaVersionId::new(),
        media_id: MediaId::new(),
        state: MediaVersionState::Ready,
        object_key: Some(underlay_blob::BlobObjectKey::parse("test/key").unwrap()),
        mime_type: Some("image/jpeg".to_string()),
        byte_size: Some(1024),
        sha256_hash: None,
        width: None,
        height: None,
        storage_provider: Some("s3".to_string()),
        bucket: Some("test-bucket".to_string()),
        uploaded_by: None,
        created_at: Utc::now(),
    };
    assert!(version.is_ready());
    assert!(version.is_terminal());
    assert!(version.has_storage_info());
}

#[test]
fn test_media_usage_edge_key_round_trip() {
    let media_id = MediaId::new();
    let input = MediaUsageEdgeInput {
        media_id,
        used_by_type: "blog_article".to_string(),
        used_by_id: Some(Uuid::now_v7()),
        owner_field: Some("body_blocks".to_string()),
        content_kind: MediaContentKind::StructuredContent,
        locator_kind: MediaLocatorKind::BlockId,
        locator_key: "hero_01:image".to_string(),
        usage_role: MediaUsageRole::Embedded,
        provenance_kind: MediaUsageProvenanceKind::ContentSync,
    };

    let key = input.key();
    assert_eq!(key.media_id, media_id);
    assert_eq!(key.locator_key, "hero_01:image");
    assert_eq!(key.locator_kind, MediaLocatorKind::BlockId);
}

#[test]
fn test_custom_usage_enums_serialize_as_strings() {
    let role = MediaUsageRole::Custom("campaign_asset".to_string());
    let locator = MediaLocatorKind::Custom("widget_slot".to_string());

    assert_eq!(serde_json::to_string(&role).unwrap(), "\"campaign_asset\"");
    assert_eq!(serde_json::to_string(&locator).unwrap(), "\"widget_slot\"");
}
