//! Blob storage adapter trait.

use async_trait::async_trait;

use crate::error::{BlobError, BlobResult};
use crate::types::{
    BlobObjectKey, DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

pub(crate) fn join_public_url(base: &str, key: &str) -> String {
    let fallback = || format!("{}/{}", base.trim_end_matches('/'), key);
    let Ok(mut url) = url::Url::parse(base) else {
        return fallback();
    };
    let Ok(mut segments) = url.path_segments_mut() else {
        return fallback();
    };
    segments.pop_if_empty();
    for segment in key.split('/') {
        segments.push(segment);
    }
    drop(segments);
    url.into()
}

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

    /// Download an object's contents as bytes.
    ///
    /// This fetches the entire object into memory. For large files,
    /// consider using `signed_download_url` and streaming instead.
    async fn get_bytes(&self, key: &str) -> BlobResult<Vec<u8>>;

    /// Upload bytes directly to storage.
    ///
    /// This is for server-side uploads (e.g., thumbnail generation, file processing).
    /// For client-side uploads, use `initiate_upload` to get a presigned URL.
    async fn put_bytes(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject>;

    /// Bounded capture: read at most `max_bytes + 1` bytes from `key` into an
    /// owned vector and return exactly that many bytes at most.
    ///
    /// Implementations must stop consuming the source once the cap is
    /// reached rather than buffering an oversized object in full; a source
    /// larger than `max_bytes` returns [`BlobError::TooLarge`]. This is the
    /// only capture path [`crate::promotion::BlobAdapterPromotionExt::promote_verified`]
    /// uses; it never calls the unbounded [`Self::get_bytes`].
    ///
    /// The default implementation refuses with [`BlobError::Unsupported`].
    /// Adapters that back verified promotion must override this with a real
    /// bounded read; a fail-closed default is required so third-party
    /// `BlobAdapter` implementations keep compiling without silently gaining
    /// unbounded-read behavior under this method.
    async fn get_bytes_bounded(&self, key: &str, max_bytes: u64) -> BlobResult<Vec<u8>> {
        let _ = (key, max_bytes);
        Err(BlobError::Unsupported(format!(
            "{} adapter does not implement bounded capture",
            self.name()
        )))
    }

    /// Create-only byte write: create `key` if and only if it does not
    /// already exist.
    ///
    /// Implementations must never overwrite, truncate, or follow an existing
    /// destination. A collision (the destination already exists, by any
    /// name — file, symlink, or directory) returns
    /// [`BlobError::DestinationExists`], distinguishable from transport or
    /// internal failure so callers never fall back to an unconditional
    /// write.
    ///
    /// The default implementation refuses with [`BlobError::Unsupported`].
    /// Adapters that back verified promotion must override this with a real
    /// exclusive create; a fail-closed default is required so third-party
    /// `BlobAdapter` implementations keep compiling without silently gaining
    /// overwrite behavior under this method.
    async fn put_bytes_create_only(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        let _ = (key, data, content_type);
        Err(BlobError::Unsupported(format!(
            "{} adapter does not implement exclusive create",
            self.name()
        )))
    }

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

/// Typed convenience methods for callers that already hold a validated object key.
///
/// These methods preserve the raw `BlobAdapter` trait as the compatibility
/// boundary for database-loaded and app-local string keys while giving generated
/// key paths a typed call surface.
#[async_trait]
pub trait BlobAdapterObjectKeyExt: BlobAdapter {
    /// Finalise an upload for a validated object key.
    async fn finalise_upload_object_key(&self, key: &BlobObjectKey) -> BlobResult<StoredObject> {
        self.finalise_upload(key.as_str()).await
    }

    /// Get the public URL for a validated object key.
    fn public_object_url(&self, key: &BlobObjectKey) -> String {
        self.public_url(key.as_str())
    }

    /// Delete a validated object key.
    async fn delete_object_key(&self, key: &BlobObjectKey) -> BlobResult<()> {
        self.delete(key.as_str()).await
    }

    /// Get metadata about a validated object key.
    async fn head_object_key(&self, key: &BlobObjectKey) -> BlobResult<ObjectInfo> {
        self.head(key.as_str()).await
    }

    /// Download bytes for a validated object key.
    async fn get_object_bytes(&self, key: &BlobObjectKey) -> BlobResult<Vec<u8>> {
        self.get_bytes(key.as_str()).await
    }

    /// Upload bytes directly to a validated object key.
    async fn put_object_bytes(
        &self,
        key: &BlobObjectKey,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        self.put_bytes(key.as_str(), data, content_type).await
    }

    /// Check if a validated object key exists.
    async fn exists_object_key(&self, key: &BlobObjectKey) -> BlobResult<bool> {
        self.exists(key.as_str()).await
    }
}

impl<T> BlobAdapterObjectKeyExt for T where T: BlobAdapter + ?Sized {}

/// Server-side upload enforcement over any adapter.
///
/// Declared content type and length are client-supplied; these methods are
/// the enforcement point that the raw `initiate_upload`/`finalise_upload`
/// calls do not provide.
#[async_trait]
pub trait BlobAdapterUploadExt: BlobAdapter {
    /// Validate an upload request against the config (size cap, MIME
    /// allowlist) before signing. The returned plan pins the declared
    /// content type and clamps `max_bytes` to the configured maximum.
    async fn initiate_upload_validated(
        &self,
        request: UploadRequest,
        config: &crate::config::BlobUploadConfig,
    ) -> BlobResult<UploadPlan> {
        config.validate_upload_request(&request)?;
        self.initiate_upload(request).await
    }

    /// Finalise an upload with verification: the stored object must be within
    /// the size cap, carry an allowed declared content type, and its leading
    /// bytes must be consistent with that type (magic-byte sniff).
    async fn finalise_upload_verified(
        &self,
        key: &str,
        declared_content_type: &str,
        config: &crate::config::BlobUploadConfig,
    ) -> BlobResult<StoredObject> {
        let stored = self.finalise_upload(key).await?;

        if !config.is_size_allowed(stored.size) {
            return Err(crate::error::BlobError::TooLarge(
                stored.size,
                config.max_file_size_bytes_limit(),
            ));
        }

        if !config.is_content_type_allowed(declared_content_type) {
            return Err(crate::error::BlobError::InvalidContentType(
                declared_content_type.to_string(),
            ));
        }

        let bytes = self.get_bytes(key).await?;
        if !crate::sniff::content_matches_declared(&bytes, declared_content_type) {
            return Err(crate::error::BlobError::InvalidContentType(format!(
                "stored bytes do not match declared content type {}",
                declared_content_type
            )));
        }

        // Pin the declared (validated + sniff-checked) type rather than
        // whatever the storage backend echoed back.
        Ok(StoredObject {
            content_type: declared_content_type.to_string(),
            ..stored
        })
    }
}

impl<T> BlobAdapterUploadExt for T where T: BlobAdapter + ?Sized {}

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
            upload_url: join_public_url(&self.base_url, request.key.as_str()),
            method: "PUT".to_string(),
            required_headers: std::collections::HashMap::new(),
            max_bytes: request.content_length,
            allowed_content_types: vec![request.content_type.clone()],
            expires_at: Utc::now() + chrono::Duration::seconds(request.expires_in.as_secs() as i64),
            object_key: request.key.into_string(),
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
        join_public_url(&self.base_url, key)
    }

    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl> {
        use chrono::Utc;

        Ok(SignedUrl {
            url: format!(
                "{}?signed=true",
                join_public_url(&self.base_url, request.key.as_str())
            ),
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

    async fn get_bytes(&self, _key: &str) -> BlobResult<Vec<u8>> {
        // Noop adapter returns empty bytes
        Ok(Vec::new())
    }

    async fn put_bytes(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        Ok(StoredObject::new(
            "noop",
            &self.bucket,
            key,
            data.len() as u64,
            content_type,
        ))
    }

    fn name(&self) -> &'static str {
        "noop"
    }

    fn bucket(&self) -> &str {
        &self.bucket
    }
}

#[cfg(test)]
#[path = "tests/adapter_tests.rs"]
mod tests;
