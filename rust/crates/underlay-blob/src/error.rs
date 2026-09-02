//! Error types for blob storage operations.

use thiserror::Error;

/// Errors that can occur during blob storage operations.
#[derive(Debug, Error)]
pub enum BlobError {
    /// The object was not found.
    #[error("object not found: {0}")]
    NotFound(String),

    /// Access denied to the object or bucket.
    #[error("access denied: {0}")]
    AccessDenied(String),

    /// The bucket does not exist.
    #[error("bucket not found: {0}")]
    BucketNotFound(String),

    /// Invalid object key.
    #[error("invalid key: {0}")]
    InvalidKey(String),

    /// The object exceeds the maximum allowed size.
    #[error("object too large: {0} bytes (max: {1})")]
    TooLarge(u64, u64),

    /// Invalid content type.
    #[error("invalid content type: {0}")]
    InvalidContentType(String),

    /// Configuration error.
    #[error("configuration error: {0}")]
    ConfigError(String),

    /// Authentication/credentials error.
    #[error("authentication error: {0}")]
    AuthError(String),

    /// Network or transport error.
    #[error("transport error: {0}")]
    TransportError(String),

    /// Pre-signed URL generation failed.
    #[error("failed to generate pre-signed URL: {0}")]
    PresignError(String),

    /// I/O error (for local adapter).
    #[error("I/O error: {0}")]
    IoError(String),

    /// Failed to download object.
    #[error("download failed: {0}")]
    DownloadFailed(String),

    /// Failed to upload object.
    #[error("upload failed: {0}")]
    UploadFailed(String),

    /// The adapter is not available or not configured.
    #[error("adapter not available: {0}")]
    AdapterUnavailable(String),

    /// A create-only write refused because the destination already exists.
    ///
    /// This is a collision, not a transport/internal failure: the caller
    /// must not retry as an unconditional overwrite. The message carries the
    /// object key only, never backend request/response detail.
    #[error("destination already exists: {0}")]
    DestinationExists(String),

    /// The adapter or the source does not support this operation.
    ///
    /// Covers the fail-closed default for adapters that have not implemented
    /// bounded capture or exclusive create, and local sources this crate
    /// refuses to read as bytes (symlinks, directories, and other
    /// non-regular files).
    #[error("unsupported: {0}")]
    Unsupported(String),

    /// Generic internal error.
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<std::io::Error> for BlobError {
    fn from(err: std::io::Error) -> Self {
        BlobError::IoError(err.to_string())
    }
}

/// Result type for blob storage operations.
pub type BlobResult<T> = Result<T, BlobError>;
