//! Local filesystem blob storage adapter for development.
//!
//! **WARNING**: This adapter is for development only and should NOT be used in production.
//! The associated file-serving endpoint must be completely removed from production builds.

use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use crate::adapter::BlobAdapter;
use crate::error::{BlobError, BlobResult};
use crate::types::{DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest};

/// Configuration for the local filesystem adapter.
#[derive(Debug, Clone)]
pub struct LocalConfig {
    /// Base directory where files are stored.
    pub base_path: PathBuf,

    /// Base URL for serving files (e.g., "http://localhost:8080/dev-uploads").
    /// This should point to a development-only endpoint that serves files from base_path.
    pub serve_url_base: String,

    /// Virtual bucket name (for compatibility with the adapter interface).
    pub bucket: String,

    /// Base URL for the upload endpoint (e.g., "http://localhost:8080/dev-uploads").
    /// If not set, uses serve_url_base.
    pub upload_url_base: Option<String>,
}

impl LocalConfig {
    /// Create a new local configuration.
    pub fn new(base_path: impl Into<PathBuf>, serve_url_base: impl Into<String>) -> Self {
        Self {
            base_path: base_path.into(),
            serve_url_base: serve_url_base.into(),
            bucket: "local".to_string(),
            upload_url_base: None,
        }
    }

    /// Set the virtual bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = bucket.into();
        self
    }

    /// Set a custom upload URL base.
    pub fn upload_url_base(mut self, url: impl Into<String>) -> Self {
        self.upload_url_base = Some(url.into());
        self
    }
}

/// Local filesystem blob storage adapter.
///
/// **WARNING**: This adapter is for development only. It stores files on the local
/// filesystem and serves them via a configurable URL. The serving endpoint must be
/// removed or disabled in production builds.
///
/// # File Storage
///
/// Files are stored at `{base_path}/{key}`. The directory structure is created
/// automatically based on the key path.
///
/// # Upload Flow
///
/// Unlike S3, which uses pre-signed URLs, the local adapter returns a URL that
/// the client can POST to directly. The actual file writing is handled by this
/// adapter's `write_file` method, which should be called from a development-only
/// HTTP endpoint.
pub struct LocalAdapter {
    config: LocalConfig,
}

impl LocalAdapter {
    /// Create a new local adapter with the given configuration.
    ///
    /// This will create the base directory if it doesn't exist.
    pub async fn new(config: LocalConfig) -> BlobResult<Self> {
        // Ensure the base directory exists
        fs::create_dir_all(&config.base_path)
            .await
            .map_err(|e| BlobError::ConfigError(format!("Failed to create base directory: {}", e)))?;

        Ok(Self { config })
    }

    /// Get the full filesystem path for a key.
    pub fn path_for_key(&self, key: &str) -> PathBuf {
        self.config.base_path.join(key)
    }

    /// Write a file to the local filesystem.
    ///
    /// This is called from a development-only HTTP endpoint after receiving
    /// the upload request. It should NOT be exposed in production.
    pub async fn write_file(&self, key: &str, data: &[u8], content_type: &str) -> BlobResult<StoredObject> {
        let path = self.path_for_key(key);

        // Create parent directories if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| BlobError::IoError(format!("Failed to create directories: {}", e)))?;
        }

        // Write the file
        let mut file = fs::File::create(&path)
            .await
            .map_err(|e| BlobError::IoError(format!("Failed to create file: {}", e)))?;

        file.write_all(data)
            .await
            .map_err(|e| BlobError::IoError(format!("Failed to write file: {}", e)))?;

        file.flush()
            .await
            .map_err(|e| BlobError::IoError(format!("Failed to flush file: {}", e)))?;

        Ok(StoredObject {
            provider: "local".to_string(),
            bucket: self.config.bucket.clone(),
            key: key.to_string(),
            size: data.len() as u64,
            content_type: content_type.to_string(),
            etag: None,
        })
    }

    /// Read a file from the local filesystem.
    ///
    /// This is used by a development-only HTTP endpoint to serve files.
    /// It should NOT be exposed in production.
    pub async fn read_file(&self, key: &str) -> BlobResult<Vec<u8>> {
        let path = self.path_for_key(key);

        fs::read(&path)
            .await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    BlobError::NotFound(key.to_string())
                } else {
                    BlobError::IoError(e.to_string())
                }
            })
    }
}

#[async_trait]
impl BlobAdapter for LocalAdapter {
    async fn initiate_upload(&self, request: UploadRequest) -> BlobResult<UploadPlan> {
        let upload_base = self.config.upload_url_base
            .as_ref()
            .unwrap_or(&self.config.serve_url_base);

        let upload_url = format!("{}/{}", upload_base.trim_end_matches('/'), &request.key);

        let expires_at = Utc::now() + chrono::Duration::from_std(request.expires_in)
            .unwrap_or_else(|_| chrono::Duration::hours(1));

        Ok(UploadPlan {
            upload_url,
            method: "PUT".to_string(),  // Local endpoint should accept PUT
            required_headers: HashMap::new(),
            max_bytes: request.content_length,
            allowed_content_types: vec![request.content_type],
            expires_at,
            object_key: request.key,
        })
    }

    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject> {
        // Verify the file exists and get metadata
        let info = self.head(key).await?;

        Ok(StoredObject {
            provider: "local".to_string(),
            bucket: self.config.bucket.clone(),
            key: key.to_string(),
            size: info.size,
            content_type: info.content_type,
            etag: info.etag,
        })
    }

    fn public_url(&self, key: &str) -> String {
        format!("{}/{}", self.config.serve_url_base.trim_end_matches('/'), key)
    }

    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl> {
        // Local adapter doesn't need signing - just return the public URL
        let url = self.public_url(&request.key);

        let expires_at = Utc::now() + chrono::Duration::from_std(request.expires_in)
            .unwrap_or_else(|_| chrono::Duration::minutes(5));

        Ok(SignedUrl { url, expires_at })
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        let path = self.path_for_key(key);

        match fs::remove_file(&path).await {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()), // Idempotent
            Err(e) => Err(BlobError::IoError(e.to_string())),
        }
    }

    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        let path = self.path_for_key(key);

        let metadata = fs::metadata(&path)
            .await
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    BlobError::NotFound(key.to_string())
                } else {
                    BlobError::IoError(e.to_string())
                }
            })?;

        let last_modified = metadata
            .modified()
            .ok()
            .and_then(|t| DateTime::<Utc>::from(t).into());

        // Try to guess content type from extension
        let content_type = guess_content_type(key);

        Ok(ObjectInfo {
            key: key.to_string(),
            size: metadata.len(),
            content_type,
            etag: None, // Could compute hash if needed
            last_modified,
            metadata: HashMap::new(),
        })
    }

    fn name(&self) -> &'static str {
        "local"
    }

    fn bucket(&self) -> &str {
        &self.config.bucket
    }

    async fn health_check(&self) -> BlobResult<()> {
        // Just verify the base directory exists and is accessible
        fs::metadata(&self.config.base_path)
            .await
            .map_err(|e| BlobError::ConfigError(format!("Base path not accessible: {}", e)))?;

        Ok(())
    }
}

impl std::fmt::Debug for LocalAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalAdapter")
            .field("base_path", &self.config.base_path)
            .field("serve_url_base", &self.config.serve_url_base)
            .finish()
    }
}

/// Guess the content type from a file extension.
fn guess_content_type(key: &str) -> String {
    let extension = key
        .rsplit('.')
        .next()
        .map(|s| s.to_lowercase());

    match extension.as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        Some("pdf") => "application/pdf",
        Some("json") => "application/json",
        Some("txt") => "text/plain",
        Some("html") => "text/html",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        _ => "application/octet-stream",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_local_adapter_write_and_read() {
        let temp_dir = std::env::temp_dir().join("underlay-blob-test");
        let _ = fs::remove_dir_all(&temp_dir).await; // Clean up from previous runs

        let config = LocalConfig::new(&temp_dir, "http://localhost:8080/uploads");
        let adapter = LocalAdapter::new(config).await.unwrap();

        // Write a file
        let key = "test/hello.txt";
        let data = b"Hello, World!";
        let stored = adapter.write_file(key, data, "text/plain").await.unwrap();

        assert_eq!(stored.key, key);
        assert_eq!(stored.size, data.len() as u64);

        // Read it back
        let read_data = adapter.read_file(key).await.unwrap();
        assert_eq!(read_data, data);

        // Head
        let info = adapter.head(key).await.unwrap();
        assert_eq!(info.size, data.len() as u64);
        assert_eq!(info.content_type, "text/plain");

        // Delete
        adapter.delete(key).await.unwrap();
        assert!(!adapter.exists(key).await.unwrap());

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir).await;
    }

    #[tokio::test]
    async fn test_local_adapter_public_url() {
        let config = LocalConfig::new("/tmp/test", "http://localhost:8080/uploads");
        let adapter = LocalAdapter::new(config).await.unwrap();

        let url = adapter.public_url("media/123/photo.jpg");
        assert_eq!(url, "http://localhost:8080/uploads/media/123/photo.jpg");
    }

    #[test]
    fn test_guess_content_type() {
        assert_eq!(guess_content_type("photo.jpg"), "image/jpeg");
        assert_eq!(guess_content_type("photo.JPEG"), "image/jpeg");
        assert_eq!(guess_content_type("doc.pdf"), "application/pdf");
        assert_eq!(guess_content_type("unknown.xyz"), "application/octet-stream");
    }
}
