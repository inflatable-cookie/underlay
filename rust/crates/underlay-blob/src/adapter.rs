//! Blob storage adapter trait.

use async_trait::async_trait;

use crate::error::BlobResult;
use crate::types::{DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest};

/// Trait for blob storage backends.
///
/// Implementations provide different ways to store and retrieve blobs:
/// - S3-compatible storage (AWS S3, R2, MinIO, etc.)
/// - Local filesystem (for development)
/// - etc.
#[async_trait]
pub trait BlobAdapter: Send + Sync {
    /// Initiate an upload by generating a pre-signed URL or upload endpoint.
    ///
    /// Returns an `UploadPlan` containing the URL to upload to and any
    /// required headers or constraints.
    async fn initiate_upload(&self, request: UploadRequest) -> BlobResult<UploadPlan>;

    /// Finalise an upload by verifying the object exists and returning its metadata.
    ///
    /// This should be called after the client has uploaded the file to validate
    /// that the upload completed successfully.
    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject>;

    /// Get the public URL for an object.
    ///
    /// This returns a direct URL that doesn't require authentication.
    /// For private buckets, this may not be accessible.
    fn public_url(&self, key: &str) -> String;

    /// Generate a signed URL for downloading an object.
    ///
    /// The URL will be valid for the duration specified in the request.
    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl>;

    /// Delete an object.
    ///
    /// This operation is idempotent - deleting a non-existent object succeeds.
    async fn delete(&self, key: &str) -> BlobResult<()>;

    /// Get metadata about an object without downloading it.
    async fn head(&self, key: &str) -> BlobResult<ObjectInfo>;

    /// Check if an object exists.
    async fn exists(&self, key: &str) -> BlobResult<bool> {
        match self.head(key).await {
            Ok(_) => Ok(true),
            Err(crate::error::BlobError::NotFound(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// Get the name of this adapter (for logging/debugging).
    fn name(&self) -> &'static str;

    /// Get the bucket/container name this adapter is configured for.
    fn bucket(&self) -> &str;

    /// Check if the adapter is healthy/connected.
    ///
    /// Default implementation returns Ok. Adapters that maintain
    /// connections should override this to perform actual health checks.
    async fn health_check(&self) -> BlobResult<()> {
        Ok(())
    }
}

/// A no-op adapter that does nothing (useful for testing).
#[derive(Debug, Clone)]
pub struct NoopAdapter {
    bucket: String,
    base_url: String,
}

impl Default for NoopAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl NoopAdapter {
    /// Create a new no-op adapter.
    pub fn new() -> Self {
        Self {
            bucket: "noop-bucket".to_string(),
            base_url: "https://noop.example.com".to_string(),
        }
    }

    /// Create a no-op adapter with custom bucket and base URL.
    pub fn with_config(bucket: impl Into<String>, base_url: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            base_url: base_url.into(),
        }
    }
}

#[async_trait]
impl BlobAdapter for NoopAdapter {
    async fn initiate_upload(&self, request: UploadRequest) -> BlobResult<UploadPlan> {
        use chrono::Utc;

        Ok(UploadPlan {
            upload_url: format!("{}/{}", self.base_url, request.key),
            method: "PUT".to_string(),
            required_headers: std::collections::HashMap::new(),
            max_bytes: request.content_length,
            allowed_content_types: vec![request.content_type.clone()],
            expires_at: Utc::now() + chrono::Duration::seconds(request.expires_in.as_secs() as i64),
            object_key: request.key,
        })
    }

    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject> {
        Ok(StoredObject::new(
            "noop",
            &self.bucket,
            key,
            0,
            "application/octet-stream",
        ))
    }

    fn public_url(&self, key: &str) -> String {
        format!("{}/{}", self.base_url, key)
    }

    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl> {
        use chrono::Utc;

        Ok(SignedUrl {
            url: format!("{}/{}?signed=true", self.base_url, request.key),
            expires_at: Utc::now() + chrono::Duration::seconds(request.expires_in.as_secs() as i64),
        })
    }

    async fn delete(&self, _key: &str) -> BlobResult<()> {
        Ok(())
    }

    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        Ok(ObjectInfo {
            key: key.to_string(),
            size: 0,
            content_type: "application/octet-stream".to_string(),
            etag: None,
            last_modified: None,
            metadata: std::collections::HashMap::new(),
        })
    }

    fn name(&self) -> &'static str {
        "noop"
    }

    fn bucket(&self) -> &str {
        &self.bucket
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
