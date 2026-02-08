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
use crate::types::{
    DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

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
    /// Canonicalized base path for secure path comparisons.
    canonical_base: PathBuf,
}

impl LocalAdapter {
    /// Create a new local adapter with the given configuration.
    ///
    /// This will create the base directory if it doesn't exist.
    pub async fn new(config: LocalConfig) -> BlobResult<Self> {
        // Ensure the base directory exists
        fs::create_dir_all(&config.base_path).await.map_err(|e| {
            BlobError::ConfigError(format!("Failed to create base directory: {}", e))
        })?;

        // Canonicalize the base path for secure comparisons (resolves symlinks and ..)
        let canonical_base = config.base_path.canonicalize().map_err(|e| {
            BlobError::ConfigError(format!("Failed to canonicalize base path: {}", e))
        })?;

        Ok(Self {
            config,
            canonical_base,
        })
    }

    /// Get the full filesystem path for a key.
    pub fn path_for_key(&self, key: &str) -> PathBuf {
        self.config.base_path.join(key)
    }

    /// Write a file to the local filesystem.
    ///
    /// This is called from a development-only HTTP endpoint after receiving
    /// the upload request. It should NOT be exposed in production.
    pub async fn write_file(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
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

        fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })
    }

    /// Check if a path is safely within the base directory.
    ///
    /// Returns the canonicalized path if valid, None otherwise.
    /// This resolves symlinks and `..` to prevent path traversal attacks.
    fn is_path_within_base(&self, path: &std::path::Path) -> Option<PathBuf> {
        // Canonicalize the path to resolve symlinks and ..
        let canonical = match path.canonicalize() {
            Ok(p) => p,
            Err(_) => return None,
        };

        // Ensure the canonical path starts with our canonical base
        // and is not equal to it (we never want to delete the base itself)
        if canonical.starts_with(&self.canonical_base) && canonical != self.canonical_base {
            Some(canonical)
        } else {
            None
        }
    }

    /// Clean up empty parent directories after file deletion.
    ///
    /// Walks up from the deleted file's parent directory, removing empty
    /// directories until reaching the base path or a non-empty directory.
    ///
    /// # Safety
    ///
    /// This function uses canonicalized paths to prevent:
    /// - Path traversal attacks via `..`
    /// - Symlink attacks that could escape the base directory
    /// - Accidental deletion of the base directory itself
    async fn cleanup_empty_parents(&self, deleted_path: &std::path::Path) {
        let mut current = deleted_path.parent().map(|p| p.to_path_buf());

        while let Some(ref dir) = current {
            // Validate this directory is safely within our base path
            // This canonicalizes the path, resolving any symlinks or ..
            let canonical_dir = match self.is_path_within_base(dir) {
                Some(p) => p,
                None => {
                    // Path is outside base directory or doesn't exist - stop immediately
                    break;
                }
            };

            // Double-check: the canonical path must be strictly inside the canonical base
            // (not equal to it, and must be a proper child path)
            if canonical_dir == self.canonical_base {
                break;
            }

            // Try to remove the directory (only succeeds if empty)
            // Note: remove_dir does NOT follow symlinks - it removes the symlink itself
            // if the target is a symlink to a directory, which is the safe behavior
            match fs::remove_dir(&canonical_dir).await {
                Ok(()) => {
                    // Directory was empty and removed, continue to parent
                    current = dir.parent().map(|p| p.to_path_buf());
                }
                Err(_) => {
                    // Directory not empty, permission denied, or other error - stop
                    break;
                }
            }
        }
    }
}

#[async_trait]
impl BlobAdapter for LocalAdapter {
    async fn initiate_upload(&self, request: UploadRequest) -> BlobResult<UploadPlan> {
        let upload_base = self
            .config
            .upload_url_base
            .as_ref()
            .unwrap_or(&self.config.serve_url_base);

        let upload_url = format!("{}/{}", upload_base.trim_end_matches('/'), &request.key);

        let expires_at = Utc::now()
            + chrono::Duration::from_std(request.expires_in)
                .unwrap_or_else(|_| chrono::Duration::hours(1));

        Ok(UploadPlan {
            upload_url,
            method: "PUT".to_string(), // Local endpoint should accept PUT
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
        format!(
            "{}/{}",
            self.config.serve_url_base.trim_end_matches('/'),
            key
        )
    }

    async fn signed_download_url(&self, request: DownloadRequest) -> BlobResult<SignedUrl> {
        // Local adapter doesn't need signing - just return the public URL
        let url = self.public_url(&request.key);

        let expires_at = Utc::now()
            + chrono::Duration::from_std(request.expires_in)
                .unwrap_or_else(|_| chrono::Duration::minutes(5));

        Ok(SignedUrl { url, expires_at })
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        let path = self.path_for_key(key);

        match fs::remove_file(&path).await {
            Ok(()) => {
                // Clean up empty parent directories
                self.cleanup_empty_parents(&path).await;
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()), // Idempotent
            Err(e) => Err(BlobError::IoError(e.to_string())),
        }
    }

    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        let path = self.path_for_key(key);

        let metadata = fs::metadata(&path).await.map_err(|e| {
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

    async fn get_bytes(&self, key: &str) -> BlobResult<Vec<u8>> {
        let path = self.path_for_key(key);

        fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::DownloadFailed(format!("Failed to read file: {}", e))
            }
        })
    }

    async fn put_bytes(
        &self,
        key: &str,
        data: &[u8],
        content_type: &str,
    ) -> BlobResult<StoredObject> {
        self.write_file(key, data, content_type).await
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
    let extension = key.rsplit('.').next().map(|s| s.to_lowercase());

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
#[path = "local_tests.rs"]
mod tests;
