use super::*;

#[test]
fn test_defaults() {
    let config = BlobUploadConfig::default();
    assert_eq!(config.max_file_size_bytes_limit(), 50 * 1024 * 1024);
}

#[test]
fn test_builder_methods() {
    let config = BlobUploadConfig::default().max_file_size_mb(100);

    assert_eq!(config.max_file_size_bytes_limit(), 100 * 1024 * 1024);
}

#[test]
fn test_is_size_allowed() {
    let config = BlobUploadConfig::default().max_file_size_mb(10);

    assert!(config.is_size_allowed(5 * 1024 * 1024)); // 5 MB - ok
    assert!(config.is_size_allowed(10 * 1024 * 1024)); // 10 MB - ok (at limit)
    assert!(!config.is_size_allowed(11 * 1024 * 1024)); // 11 MB - too big
}

#[test]
fn test_display() {
    let config = BlobUploadConfig::default();
    assert_eq!(config.max_file_size_display(), "50 MB");

    let config = BlobUploadConfig::default().max_file_size_mb(100);
    assert_eq!(config.max_file_size_display(), "100 MB");
}

#[test]
fn test_default_allowlist_excludes_active_content() {
    let config = BlobUploadConfig::default();

    assert!(config.is_content_type_allowed("image/png"));
    assert!(config.is_content_type_allowed("Image/JPEG"));
    assert!(config.is_content_type_allowed("application/pdf; charset=binary"));

    assert!(!config.is_content_type_allowed("image/svg+xml"));
    assert!(!config.is_content_type_allowed("text/html"));
    assert!(!config.is_content_type_allowed("application/javascript"));
}

#[test]
fn test_validate_upload_request_enforces_size_and_type() {
    use crate::error::BlobError;
    use crate::types::UploadRequest;

    let config = BlobUploadConfig::default().max_file_size_mb(1);

    let ok = UploadRequest::parse_key("a/b.png", "image/png", 512 * 1024).unwrap();
    assert!(config.validate_upload_request(&ok).is_ok());

    let too_big = UploadRequest::parse_key("a/b.png", "image/png", 2 * 1024 * 1024).unwrap();
    assert!(matches!(
        config.validate_upload_request(&too_big),
        Err(BlobError::TooLarge(_, _))
    ));

    let bad_type = UploadRequest::parse_key("a/b.html", "text/html", 100).unwrap();
    assert!(matches!(
        config.validate_upload_request(&bad_type),
        Err(BlobError::InvalidContentType(_))
    ));
}

#[test]
fn test_allowlist_override() {
    let config = BlobUploadConfig::default().with_allowed_content_types(["image/svg+xml"]);

    assert!(config.is_content_type_allowed("image/svg+xml"));
    assert!(!config.is_content_type_allowed("image/png"));
}
