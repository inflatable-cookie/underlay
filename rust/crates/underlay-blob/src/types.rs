//! Core types for blob storage operations.

use std::collections::HashMap;
use std::fmt;
use std::path::{Component, Path};
use std::str::FromStr;
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Error returned when an object key is not safe for Underlay blob storage.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlobObjectKeyError {
    Empty,
    Absolute,
    InvalidCharacter,
    TraversalComponent,
    InvalidComponent,
}

impl fmt::Display for BlobObjectKeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "object key must not be empty"),
            Self::Absolute => write!(f, "object key must be a relative path"),
            Self::InvalidCharacter => write!(f, "object key contains an invalid character"),
            Self::TraversalComponent => {
                write!(f, "object key must not contain traversal components")
            }
            Self::InvalidComponent => write!(f, "object key contains an invalid path component"),
        }
    }
}

impl std::error::Error for BlobObjectKeyError {}

/// Validated object key for Underlay blob storage.
///
/// This type rejects absolute paths, traversal components, backslashes, null
/// bytes, and control characters before an adapter sees the key.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct BlobObjectKey(String);

impl BlobObjectKey {
    pub fn parse(value: impl AsRef<str>) -> Result<Self, BlobObjectKeyError> {
        let value = value.as_ref();
        validate_blob_object_key(value)?;
        Ok(Self(value.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for BlobObjectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for BlobObjectKey {
    type Err = BlobObjectKeyError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

impl TryFrom<String> for BlobObjectKey {
    type Error = BlobObjectKeyError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

impl From<BlobObjectKey> for String {
    fn from(value: BlobObjectKey) -> Self {
        value.into_string()
    }
}

pub fn validate_blob_object_key(key: &str) -> Result<(), BlobObjectKeyError> {
    if key.trim().is_empty() {
        return Err(BlobObjectKeyError::Empty);
    }
    if key.starts_with('/') || key.starts_with('\\') {
        return Err(BlobObjectKeyError::Absolute);
    }
    if key
        .chars()
        .any(|ch| ch.is_control() || matches!(ch, '\\' | '\0'))
    {
        return Err(BlobObjectKeyError::InvalidCharacter);
    }

    for component in Path::new(key).components() {
        match component {
            Component::Normal(part) if !part.is_empty() => {}
            Component::CurDir
            | Component::ParentDir
            | Component::RootDir
            | Component::Prefix(_) => {
                return Err(BlobObjectKeyError::TraversalComponent);
            }
            Component::Normal(_) => {
                return Err(BlobObjectKeyError::InvalidComponent);
            }
        }
    }

    Ok(())
}

/// Request to initiate an upload.
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// The object key (path) to upload to.
    pub key: BlobObjectKey,

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
    pub fn new(key: BlobObjectKey, content_type: impl Into<String>, content_length: u64) -> Self {
        Self {
            key,
            content_type: content_type.into(),
            content_length,
            expires_in: Duration::from_secs(3600), // 1 hour default
            metadata: HashMap::new(),
        }
    }

    /// Parse an object key and create a new upload request.
    pub fn parse_key(
        key: impl AsRef<str>,
        content_type: impl Into<String>,
        content_length: u64,
    ) -> Result<Self, BlobObjectKeyError> {
        Ok(Self::new(
            BlobObjectKey::parse(key)?,
            content_type,
            content_length,
        ))
    }

    /// Create a new upload request from a pre-validated object key.
    pub fn from_object_key(
        key: BlobObjectKey,
        content_type: impl Into<String>,
        content_length: u64,
    ) -> Self {
        Self::new(key, content_type, content_length)
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
    pub key: BlobObjectKey,

    /// How long the download URL should be valid.
    pub expires_in: Duration,

    /// Optional filename to suggest for Content-Disposition.
    pub filename: Option<String>,
}

impl DownloadRequest {
    /// Create a new download request.
    pub fn new(key: BlobObjectKey) -> Self {
        Self {
            key,
            expires_in: Duration::from_secs(300), // 5 minutes default
            filename: None,
        }
    }

    /// Parse an object key and create a new download request.
    pub fn parse_key(key: impl AsRef<str>) -> Result<Self, BlobObjectKeyError> {
        Ok(Self::new(BlobObjectKey::parse(key)?))
    }

    /// Create a new download request from a pre-validated object key.
    pub fn from_object_key(key: BlobObjectKey) -> Self {
        Self::new(key)
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

/// Build an RFC 6266 `Content-Disposition: attachment` value for an
/// untrusted filename.
///
/// The quoted fallback strips quotes, backslashes, and control characters
/// (header-injection surface); the `filename*` parameter carries the full
/// UTF-8 name percent-encoded.
pub fn content_disposition_attachment(filename: &str) -> String {
    let fallback: String = filename
        .chars()
        .filter(|c| !c.is_control() && *c != '"' && *c != '\\')
        .map(|c| if c.is_ascii() { c } else { '_' })
        .collect();

    let encoded: String = filename
        .bytes()
        .flat_map(|b| {
            let needs_escape = !(b.is_ascii_alphanumeric()
                || matches!(
                    b,
                    b'-' | b'.' | b'_' | b'~' | b'!' | b'#' | b'$' | b'&' | b'+'
                ));
            if needs_escape {
                format!("%{:02X}", b).into_bytes()
            } else {
                vec![b]
            }
        })
        .map(char::from)
        .collect();

    format!(
        "attachment; filename=\"{}\"; filename*=UTF-8''{}",
        fallback, encoded
    )
}

/// A signed URL for downloading or accessing an object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedUrl {
    /// The signed URL.
    pub url: String,

    /// When the URL expires.
    pub expires_at: DateTime<Utc>,
}
