//! Core types for blob storage operations.

use std::collections::HashMap;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Request to initiate an upload.
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// The object key (path) to upload to.
    pub key: String,

    /// Expected content type (MIME type).
    pub content_type: String,

    /// Expected content length in bytes.
    pub content_length: u64,

    /// How long the upload URL should be valid.
    pub expires_in: Duration,

    /// Optional metadata to attach to the object.
    pub metadata: HashMap<String, String>,
}

impl UploadRequest {
    /// Create a new upload request.
    pub fn new(
        key: impl Into<String>,
        content_type: impl Into<String>,
        content_length: u64,
    ) -> Self {
        Self {
            key: key.into(),
            content_type: content_type.into(),
            content_length,
            expires_in: Duration::from_secs(3600), // 1 hour default
            metadata: HashMap::new(),
        }
    }

    /// Set the expiration duration for the upload URL.
    pub fn expires_in(mut self, duration: Duration) -> Self {
        self.expires_in = duration;
        self
    }

    /// Add metadata to the upload.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }
}

/// The result of initiating an upload, containing the pre-signed URL and constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UploadPlan {
    /// The URL to upload to (pre-signed PUT URL for S3, or direct endpoint for local).
    pub upload_url: String,

    /// HTTP method to use (PUT for S3, POST for some local setups).
    pub method: String,

    /// Required headers that must be included in the upload request.
    #[serde(rename = "headers")]
    pub required_headers: HashMap<String, String>,

    /// Maximum allowed file size in bytes.
    pub max_bytes: u64,

    /// Allowed content types.
    pub allowed_content_types: Vec<String>,

    /// When the upload URL expires.
    pub expires_at: DateTime<Utc>,

    /// The final object key (for reference).
    pub object_key: String,
}

/// Metadata about a stored object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    /// The object key (path).
    pub key: String,

    /// Size in bytes.
    pub size: u64,

    /// Content type (MIME type).
    pub content_type: String,

    /// ETag (typically MD5 hash for S3).
    pub etag: Option<String>,

    /// When the object was last modified.
    pub last_modified: Option<DateTime<Utc>>,

    /// Custom metadata attached to the object.
    pub metadata: HashMap<String, String>,
}

/// Reference to a stored object with its location.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredObject {
    /// The storage provider name (e.g., "s3", "local").
    pub provider: String,

    /// The bucket or container name.
    pub bucket: String,

    /// The object key (path).
    pub key: String,

    /// Size in bytes.
    pub size: u64,

    /// Content type (MIME type).
    pub content_type: String,

    /// ETag if available.
    pub etag: Option<String>,
}

impl StoredObject {
    /// Create a new stored object reference.
    pub fn new(
        provider: impl Into<String>,
        bucket: impl Into<String>,
        key: impl Into<String>,
        size: u64,
        content_type: impl Into<String>,
    ) -> Self {
        Self {
            provider: provider.into(),
            bucket: bucket.into(),
            key: key.into(),
            size,
            content_type: content_type.into(),
            etag: None,
        }
    }

    /// Set the ETag.
    pub fn with_etag(mut self, etag: impl Into<String>) -> Self {
        self.etag = Some(etag.into());
        self
    }
}

/// Request for a signed download URL.
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// The object key to download.
    pub key: String,

    /// How long the download URL should be valid.
    pub expires_in: Duration,

    /// Optional filename to suggest for Content-Disposition.
    pub filename: Option<String>,
}

impl DownloadRequest {
    /// Create a new download request.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            expires_in: Duration::from_secs(300), // 5 minutes default
            filename: None,
        }
    }

    /// Set the expiration duration.
    pub fn expires_in(mut self, duration: Duration) -> Self {
        self.expires_in = duration;
        self
    }

    /// Set the suggested filename for download.
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.filename = Some(name.into());
        self
    }
}

/// A signed URL for downloading or accessing an object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrl {
    /// The signed URL.
    pub url: String,

    /// When the URL expires.
    pub expires_at: DateTime<Utc>,
}
