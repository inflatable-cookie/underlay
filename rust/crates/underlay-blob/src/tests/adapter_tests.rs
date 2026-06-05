use super::*;
use crate::BlobObjectKey;
use std::time::Duration;

#[tokio::test]
async fn test_noop_adapter_initiate_upload() {
    let adapter = NoopAdapter::new();
    let request = UploadRequest::new("test/file.txt", "text/plain", 1024);

    let plan = adapter.initiate_upload(request).await.unwrap();
    assert!(plan.upload_url.contains("test/file.txt"));
    assert_eq!(plan.method, "PUT");
}

#[tokio::test]
async fn test_noop_adapter_public_url() {
    let adapter = NoopAdapter::with_config("my-bucket", "https://cdn.example.com");
    let url = adapter.public_url("images/photo.jpg");
    assert_eq!(url, "https://cdn.example.com/images/photo.jpg");
}

#[tokio::test]
async fn test_noop_adapter_signed_url() {
    let adapter = NoopAdapter::new();
    let request = DownloadRequest::new("test/file.txt").expires_in(Duration::from_secs(600));

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
    assert_eq!(upload.key, "media/123/photo.jpg");
    assert_eq!(download.key, "media/123/photo.jpg");
}
