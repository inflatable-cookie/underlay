use super::*;
use crate::BlobObjectKey;
use std::time::Duration;

#[tokio::test]
async fn test_noop_adapter_initiate_upload() {
    let adapter = NoopAdapter::new();
    let request = UploadRequest::parse_key("test/file.txt", "text/plain", 1024).unwrap();

    let plan = adapter.initiate_upload(request).await.unwrap();
    assert!(plan.upload_url.contains("test/file.txt"));
    assert_eq!(plan.method, "PUT");
}

#[tokio::test]
async fn test_noop_adapter_public_url() {
    let adapter = NoopAdapter::with_config("my-bucket", "https://cdn.example.com");
    let url = adapter.public_url("images/photo.jpg");
    assert_eq!(url, "https://cdn.example.com/images/photo.jpg");

    let encoded = adapter.public_url("images/My photo #1.jpg");
    assert_eq!(
        encoded,
        "https://cdn.example.com/images/My%20photo%20%231.jpg"
    );
}

#[tokio::test]
async fn test_noop_adapter_signed_url() {
    let adapter = NoopAdapter::new();
    let request = DownloadRequest::parse_key("test/file.txt")
        .unwrap()
        .expires_in(Duration::from_secs(600));

    let signed = adapter.signed_download_url(request).await.unwrap();
    assert!(signed.url.contains("test/file.txt"));
    assert!(signed.url.contains("signed=true"));
}

#[tokio::test]
async fn test_noop_adapter_exists() {
    let adapter = NoopAdapter::new();
    // NoopAdapter always returns true for exists (via head)
    assert!(adapter.exists("any/key").await.unwrap());
}

#[tokio::test]
async fn typed_adapter_extension_methods_delegate_to_raw_methods() {
    let adapter = NoopAdapter::with_config("my-bucket", "https://cdn.example.com");
    let key = BlobObjectKey::parse("media/123/photo.jpg").unwrap();

    let stored = adapter.finalise_upload_object_key(&key).await.unwrap();
    assert_eq!(stored.key, key.as_str());

    assert_eq!(
        adapter.public_object_url(&key),
        "https://cdn.example.com/media/123/photo.jpg"
    );

    let info = adapter.head_object_key(&key).await.unwrap();
    assert_eq!(info.key, key.as_str());

    let bytes = adapter.get_object_bytes(&key).await.unwrap();
    assert!(bytes.is_empty());

    let stored = adapter
        .put_object_bytes(&key, b"hello", "text/plain")
        .await
        .unwrap();
    assert_eq!(stored.key, key.as_str());
    assert_eq!(stored.size, 5);

    assert!(adapter.exists_object_key(&key).await.unwrap());
    adapter.delete_object_key(&key).await.unwrap();
}

#[tokio::test]
async fn default_bounded_capture_and_create_only_refuse_as_unsupported() {
    use crate::error::BlobError;

    // NoopAdapter does not override the new methods; the trait's
    // fail-closed defaults must apply so it keeps compiling as a valid
    // `BlobAdapter` without silently gaining bounded/exclusive behavior.
    let adapter = NoopAdapter::new();

    let err = adapter
        .get_bytes_bounded("any/key", 1024)
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::Unsupported(_)));

    let err = adapter
        .put_bytes_create_only("any/key", b"data", "application/octet-stream")
        .await
        .unwrap_err();
    assert!(matches!(err, BlobError::Unsupported(_)));
}

#[test]
fn blob_object_key_rejects_unsafe_values() {
    assert!(BlobObjectKey::parse("").is_err());
    assert!(BlobObjectKey::parse("/absolute/path").is_err());
    assert!(BlobObjectKey::parse("../outside").is_err());
    assert!(BlobObjectKey::parse("nested/../outside").is_err());
    assert!(BlobObjectKey::parse("nested\\outside").is_err());
    assert!(BlobObjectKey::parse("bad\nkey").is_err());
}

#[test]
fn blob_object_key_can_build_requests() {
    let key = BlobObjectKey::parse("media/123/photo.jpg").unwrap();
    let upload = UploadRequest::from_object_key(key.clone(), "image/jpeg", 1024);
    let download = DownloadRequest::from_object_key(key.clone());

    assert_eq!(key.as_str(), "media/123/photo.jpg");
    assert_eq!(upload.key.as_str(), "media/123/photo.jpg");
    assert_eq!(download.key.as_str(), "media/123/photo.jpg");
}

#[test]
fn request_parsing_rejects_unsafe_keys() {
    assert!(UploadRequest::parse_key("../outside", "image/jpeg", 1024).is_err());
    assert!(DownloadRequest::parse_key("../outside").is_err());
}
