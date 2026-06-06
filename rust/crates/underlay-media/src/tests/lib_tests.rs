use super::*;
#[cfg(feature = "nightfire")]
use crate::nightfire::NightfireBlockMediaRegistration;
use crate::storage::{
    mime_to_extension, rendition_object_key, version_filename, version_object_key,
    StorageKeyConfig, StorageKeyGenerator,
};
use uuid::Uuid;

#[test]
fn test_root_contract_exports() {
    // Ensure core types are accessible
    let _id = MediaId::new();
    let _version_id = MediaVersionId::new();
    let _rendition_id = MediaRenditionId::new();
    let _kind = MediaContentKind::RecordField;
    let _role = MediaUsageRole::Embedded;
    #[cfg(feature = "nightfire")]
    let _registration: Option<NightfireBlockMediaRegistration> = None;
}

#[test]
fn test_media_kind_re_export() {
    assert_eq!(MediaKind::Image.as_str(), "image");
    assert_eq!(MediaKind::Pdf.as_str(), "pdf");
}

#[test]
fn test_detect_media_kind() {
    assert_eq!(
        detect_media_kind_from_mime_type("image/jpeg"),
        Some(MediaKind::Image)
    );
    assert_eq!(
        detect_media_kind_from_mime_type("application/pdf"),
        Some(MediaKind::Pdf)
    );
    assert_eq!(detect_media_kind_from_mime_type("text/plain"), None);
}

#[test]
fn test_storage_key_module_exports() {
    let media_id = Uuid::nil();
    let version_id = Uuid::nil();

    // Test validated convenience functions
    let key = version_object_key(media_id, version_id, "photo.jpg").unwrap();
    let key = key.as_str();
    assert!(key.contains("/versions/"));
    assert!(key.ends_with("/photo.jpg"));

    let rend_key = rendition_object_key(media_id, version_id, "thumb").unwrap();
    let rend_key = rend_key.as_str();
    assert!(rend_key.contains("/renditions/"));
    assert!(rend_key.ends_with("/thumb.jpg"));

    // Test filename generation
    assert_eq!(version_filename("image/jpeg", None), "file.jpg");
    assert_eq!(
        version_filename("image/png", Some("custom.png")),
        "custom.png"
    );

    // Test extension mapping
    assert_eq!(mime_to_extension("image/jpeg"), "jpg");
    assert_eq!(mime_to_extension("application/pdf"), "pdf");
}

#[test]
fn test_storage_key_generator_module_export() {
    let config = StorageKeyConfig::with_prefix("uploads").unwrap();
    let generator = StorageKeyGenerator::new(config);
    assert_eq!(generator.config().base_prefix(), "uploads");
}
