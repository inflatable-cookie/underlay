use std::collections::HashMap;
use std::path::{Path, PathBuf};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::fs;
use tokio::io::AsyncWriteExt;

use super::config::LocalConfig;
use super::mime::guess_content_type;
#[cfg(test)]
use super::path::path_within_base;
use super::path::{cleanup_empty_parents, validate_local_object_key};
use crate::adapter::BlobAdapter;
use crate::error::{BlobError, BlobResult};
use crate::types::{
    DownloadRequest, ObjectInfo, SignedUrl, StoredObject, UploadPlan, UploadRequest,
};

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
        fs::create_dir_all(&config.base_path).await.map_err(|e| {
            BlobError::ConfigError(format!("Failed to create base directory: {}", e))
        })?;

        let canonical_base = config.base_path.canonicalize().map_err(|e| {
            BlobError::ConfigError(format!("Failed to canonicalize base path: {}", e))
        })?;

        Ok(Self {
            config,
            canonical_base,
        })
    }

    /// Get the full filesystem path for a key.
    pub fn path_for_key(&self, key: &str) -> BlobResult<PathBuf> {
        validate_local_object_key(key)?;
        Ok(self.config.base_path.join(key))
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
        let path = self.path_for_key(key)?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| BlobError::IoError(format!("Failed to create directories: {}", e)))?;
        }

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
        let path = self.path_for_key(key)?;

        fs::read(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })
    }

    async fn cleanup_empty_parents(&self, deleted_path: &Path) {
        cleanup_empty_parents(deleted_path, &self.canonical_base).await
    }
}

#[cfg(test)]
impl LocalAdapter {
    pub(super) fn is_path_within_base(&self, path: &Path) -> Option<PathBuf> {
        path_within_base(path, &self.canonical_base)
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
            method: "PUT".to_string(),
            required_headers: HashMap::new(),
            max_bytes: request.content_length,
            allowed_content_types: vec![request.content_type],
            expires_at,
            object_key: request.key,
        })
    }

    async fn finalise_upload(&self, key: &str) -> BlobResult<StoredObject> {
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
        let url = self.public_url(&request.key);

        let expires_at = Utc::now()
            + chrono::Duration::from_std(request.expires_in)
                .unwrap_or_else(|_| chrono::Duration::minutes(5));

        Ok(SignedUrl { url, expires_at })
    }

    async fn delete(&self, key: &str) -> BlobResult<()> {
        let path = self.path_for_key(key)?;

        match fs::remove_file(&path).await {
            Ok(()) => {
                self.cleanup_empty_parents(&path).await;
                Ok(())
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(BlobError::IoError(e.to_string())),
        }
    }

    async fn head(&self, key: &str) -> BlobResult<ObjectInfo> {
        let path = self.path_for_key(key)?;

        let metadata = fs::metadata(&path).await.map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                BlobError::NotFound(key.to_string())
            } else {
                BlobError::IoError(e.to_string())
            }
        })?;

        let last_modified = metadata.modified().ok().map(DateTime::<Utc>::from);
        let content_type = guess_content_type(key);

        Ok(ObjectInfo {
            key: key.to_string(),
            size: metadata.len(),
            content_type,
            etag: None,
            last_modified,
            metadata: HashMap::new(),
        })
    }

    async fn get_bytes(&self, key: &str) -> BlobResult<Vec<u8>> {
        let path = self.path_for_key(key)?;

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
