use super::*;

#[test]
fn test_default_version_key() {
    let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
    let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

    let key = version_object_key(media_id, version_id, "photo.jpg").unwrap();
    assert_eq!(
            key.as_str(),
            "media/01234567-89ab-cdef-0123-456789abcdef/versions/fedcba98-7654-3210-fedc-ba9876543210/photo.jpg"
        );
}

#[test]
fn test_default_version_object_key_matches_string_key() {
    let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
    let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

    let object_key = version_object_key(media_id, version_id, "photo.jpg").unwrap();

    assert_eq!(
        object_key.as_str(),
        "media/01234567-89ab-cdef-0123-456789abcdef/versions/fedcba98-7654-3210-fedc-ba9876543210/photo.jpg"
    );
}

#[test]
fn test_default_rendition_key() {
    let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
    let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

    let key = rendition_object_key(media_id, version_id, "thumb").unwrap();
    assert_eq!(
            key.as_str(),
            "media/01234567-89ab-cdef-0123-456789abcdef/renditions/fedcba98-7654-3210-fedc-ba9876543210/thumb.jpg"
        );
}

#[test]
fn test_default_rendition_object_key_matches_string_key() {
    let media_id = Uuid::parse_str("01234567-89ab-cdef-0123-456789abcdef").unwrap();
    let version_id = Uuid::parse_str("fedcba98-7654-3210-fedc-ba9876543210").unwrap();

    let object_key = rendition_object_key(media_id, version_id, "thumb").unwrap();

    assert_eq!(
        object_key.as_str(),
        "media/01234567-89ab-cdef-0123-456789abcdef/renditions/fedcba98-7654-3210-fedc-ba9876543210/thumb.jpg"
    );
}

#[test]
fn test_object_key_helpers_reject_unsafe_components() {
    let media_id = Uuid::nil();
    let version_id = Uuid::nil();

    assert!(version_object_key(media_id, version_id, "../photo.jpg").is_err());
    assert!(rendition_object_key(media_id, version_id, "../thumb").is_err());
}

#[test]
fn test_custom_config() {
    let config = StorageKeyConfig::with_prefix("uploads")
        .unwrap()
        .versions_dir("original")
        .unwrap()
        .renditions_dir("thumbnails")
        .unwrap()
        .rendition_extension("webp")
        .unwrap();

    let generator = StorageKeyGenerator::new(config);
    let media_id = Uuid::nil();
    let version_id = Uuid::nil();

    let version_key = generator
        .version_object_key(media_id, version_id, "test.png")
        .unwrap();
    assert_eq!(
            version_key.as_str(),
            "uploads/00000000-0000-0000-0000-000000000000/original/00000000-0000-0000-0000-000000000000/test.png"
        );

    let rendition_key = generator
        .rendition_object_key(media_id, version_id, "thumb")
        .unwrap();
    assert_eq!(
            rendition_key.as_str(),
            "uploads/00000000-0000-0000-0000-000000000000/thumbnails/00000000-0000-0000-0000-000000000000/thumb.webp"
        );
}

#[test]
fn test_storage_key_config_rejects_unsafe_components() {
    assert!(StorageKeyConfig::with_prefix("../media").is_err());
    assert!(StorageKeyConfig::with_prefix("/media").is_ok());
    assert!(StorageKeyConfig::default()
        .versions_dir("../versions")
        .is_err());
    assert!(StorageKeyConfig::default()
        .versions_dir("nested/versions")
        .is_err());
    assert!(StorageKeyConfig::default()
        .rendition_extension("bad/ext")
        .is_err());
}

#[test]
fn test_rendition_key_for_type() {
    let generator = StorageKeyGenerator::with_defaults();
    let media_id = Uuid::nil();
    let version_id = Uuid::nil();

    assert!(generator
        .rendition_object_key_for_type(media_id, version_id, &RenditionType::Thumbnail)
        .unwrap()
        .as_str()
        .ends_with("/thumb.jpg"));
    assert!(generator
        .rendition_object_key_for_type(media_id, version_id, &RenditionType::Preview)
        .unwrap()
        .as_str()
        .ends_with("/preview.jpg"));
    assert!(generator
        .rendition_object_key_for_type(
            media_id,
            version_id,
            &RenditionType::Custom("hero".to_string())
        )
        .unwrap()
        .as_str()
        .ends_with("/hero.jpg"));
}

#[test]
fn test_prefix_generation() {
    let generator = StorageKeyGenerator::with_defaults();
    let media_id = Uuid::nil();
    let version_id = Uuid::nil();

    assert_eq!(
        generator.media_prefix(media_id),
        "media/00000000-0000-0000-0000-000000000000/"
    );
    assert_eq!(
        generator.versions_prefix(media_id),
        "media/00000000-0000-0000-0000-000000000000/versions/"
    );
    assert_eq!(
        generator.renditions_prefix(media_id),
        "media/00000000-0000-0000-0000-000000000000/renditions/"
    );
    assert_eq!(
            generator.version_renditions_prefix(media_id, version_id),
            "media/00000000-0000-0000-0000-000000000000/renditions/00000000-0000-0000-0000-000000000000/"
        );
}

#[test]
fn test_version_filename() {
    assert_eq!(
        version_filename("image/jpeg", Some("my-photo.jpg")),
        "my-photo.jpg"
    );
    assert_eq!(version_filename("image/jpeg", None), "file.jpg");
    assert_eq!(version_filename("image/png", None), "file.png");
    assert_eq!(version_filename("application/pdf", None), "file.pdf");
    assert_eq!(
        version_filename("application/octet-stream", None),
        "file.bin"
    );
}

#[test]
fn test_mime_to_extension() {
    assert_eq!(mime_to_extension("image/jpeg"), "jpg");
    assert_eq!(mime_to_extension("image/png"), "png");
    assert_eq!(mime_to_extension("image/gif"), "gif");
    assert_eq!(mime_to_extension("image/webp"), "webp");
    assert_eq!(mime_to_extension("application/pdf"), "pdf");
    assert_eq!(mime_to_extension("video/mp4"), "mp4");
    assert_eq!(mime_to_extension("unknown/type"), "bin");
}
