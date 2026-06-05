//! Error types for the media library.

use thiserror::Error;

/// Error type for media operations.
#[derive(Debug, Error)]
pub enum MediaError {
    /// Media item not found.
    #[error("Media not found: {0}")]
    NotFound(String),

    /// Media version not found.
    #[error("Media version not found: {0}")]
    VersionNotFound(String),

    /// Media rendition not found.
    #[error("Media rendition not found: {0}")]
    RenditionNotFound(String),

    /// Invalid state transition.
    #[error("Invalid state: {0}")]
    InvalidState(String),

    /// Database error.
    #[error("Database error: {0}")]
    Database(String),

    /// Blob storage error.
    #[error("Storage error: {0}")]
    Storage(String),

    /// Image processing error.
    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    /// Validation error.
    #[error("Validation error: {0}")]
    Validation(String),

    /// Media is still in use and cannot be deleted.
    #[error("Media is in use by {0} entities")]
    InUse(i64),

    /// Duplicate content detected.
    #[error("Duplicate content: media {0} has same hash")]
    Duplicate(String),
}

impl MediaError {
    /// Create a not found error.
    pub fn not_found(id: impl std::fmt::Display) -> Self {
        Self::NotFound(id.to_string())
    }

    /// Create a version not found error.
    pub fn version_not_found(id: impl std::fmt::Display) -> Self {
        Self::VersionNotFound(id.to_string())
    }

    /// Create a database error.
    pub fn database(msg: impl Into<String>) -> Self {
        Self::Database(msg.into())
    }

    /// Create a storage error.
    pub fn storage(msg: impl Into<String>) -> Self {
        Self::Storage(msg.into())
    }

    /// Create an image processing error.
    pub fn image_processing(msg: impl Into<String>) -> Self {
        Self::ImageProcessing(msg.into())
    }

    /// Create a validation error.
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}

/// Result type for media operations.
pub type MediaResult<T> = Result<T, MediaError>;

// Feature-gated conversions

impl From<underlay_blob::BlobError> for MediaError {
    fn from(err: underlay_blob::BlobError) -> Self {
        Self::Storage(err.to_string())
    }
}

impl From<crate::image::ImageError> for MediaError {
    fn from(err: crate::image::ImageError) -> Self {
        Self::ImageProcessing(err.to_string())
    }
}

#[cfg(test)]
#[path = "tests/error_tests.rs"]
mod tests;
